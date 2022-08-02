// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::grapheme::GraphemeClusterSegmenter;
use crate::lstm_error::Error;
use crate::math_helper;
use crate::provider::LstmDataV1Marker;
use alloc::string::String;
use alloc::vec::Vec;
use core::str;
use icu_provider::DataPayload;
use ndarray::{Array1, Array2, ArrayBase, Dim, ViewRepr};
use zerovec::ule::AsULE;

pub struct Lstm<'l> {
    data: &'l DataPayload<LstmDataV1Marker>,
    mat1: Array2<f32>,
    mat2: Array2<f32>,
    mat3: Array2<f32>,
    mat4: Array1<f32>,
    mat5: Array2<f32>,
    mat6: Array2<f32>,
    mat7: Array1<f32>,
    mat8: Array2<f32>,
    mat9: Array1<f32>,
    grapheme: Option<&'l GraphemeClusterSegmenter>,
    hunits: usize,
    backward_hunits: usize,
}

impl<'l> Lstm<'l> {
    /// `try_new` is the initiator of struct `Lstm`
    pub fn try_new(
        data: &'l DataPayload<LstmDataV1Marker>,
        grapheme: Option<&'l GraphemeClusterSegmenter>,
    ) -> Result<Self, Error> {
        if data.get().dic.len() > core::i16::MAX as usize {
            return Err(Error::Limit);
        }

        if !data.get().model.contains("_codepoints_") && !data.get().model.contains("_graphclust_")
        {
            return Err(Error::Syntax);
        }

        if data.get().model.contains("_graphclust_") && grapheme.is_none() {
            // grapheme cluster model requires grapheme cluster data.
            return Err(Error::Syntax);
        }

        let mat1 = data.get().mat1.as_ndarray2()?;
        let mat2 = data.get().mat2.as_ndarray2()?;
        let mat3 = data.get().mat3.as_ndarray2()?;
        let mat4 = data.get().mat4.as_ndarray1()?;
        let mat5 = data.get().mat5.as_ndarray2()?;
        let mat6 = data.get().mat6.as_ndarray2()?;
        let mat7 = data.get().mat7.as_ndarray1()?;
        let mat8 = data.get().mat8.as_ndarray2()?;
        let mat9 = data.get().mat9.as_ndarray1()?;
        let embedd_dim = *mat1.shape().get(1).ok_or(Error::DimensionMismatch)?;
        let hunits = *mat3.shape().get(0).ok_or(Error::DimensionMismatch)?;
        let backward_hunits = *mat6.shape().get(0).ok_or(Error::DimensionMismatch)?;
        if mat2.shape() != [embedd_dim, 4 * hunits]
            || mat3.shape() != [hunits, 4 * hunits]
            || mat4.shape() != [4 * hunits]
            || mat5.shape() != [embedd_dim, 4 * hunits]
            || mat6.shape() != [hunits, 4 * hunits]
            || mat7.shape() != [4 * hunits]
            || mat8.shape() != [2 * hunits, 4]
            || mat9.shape() != [4]
        {
            return Err(Error::DimensionMismatch);
        }
        Ok(Self {
            data,
            mat1,
            mat2,
            mat3,
            mat4,
            mat5,
            mat6,
            mat7,
            mat8,
            mat9,
            grapheme: if data.get().model.contains("_codepoints_") {
                None
            } else {
                grapheme
            },
            hunits,
            backward_hunits,
        })
    }

    /// `get_model_name` returns the name of the LSTM model.
    #[allow(dead_code)]
    pub fn get_model_name(&self) -> &str {
        &self.data.get().model
    }

    // TODO(#421): Use common BIES normalizer code
    /// `compute_bies` uses the computed probabilities of BIES and pick the letter with the largest probability
    fn compute_bies(&self, arr: Array1<f32>) -> Result<char, Error> {
        let ind = math_helper::max_arr1(arr.view());
        match ind {
            0 => Ok('b'),
            1 => Ok('i'),
            2 => Ok('e'),
            3 => Ok('s'),
            _ => Err(Error::Syntax),
        }
    }

    /// `_return_id` returns the id corresponding to a code point or a grapheme cluster based on the model dictionary.
    fn return_id(&self, g: &str) -> i16 {
        let id = self.data.get().dic.get(g);
        if let Some(id) = id {
            i16::from_unaligned(*id)
        } else {
            self.data.get().dic.len() as i16
        }
    }

    /// `compute_hc1` implemens the evaluation of one LSTM layer.
    fn compute_hc(
        &self,
        x_t: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>,
        h_tm1: &Array1<f32>,
        c_tm1: &Array1<f32>,
        warr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 2]>>,
        uarr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 2]>>,
        barr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>,
        hunits: usize,
    ) -> (Array1<f32>, Array1<f32>) {
        // i, f, and o respectively stand for input, forget, and output gates
        let s_t = x_t.dot(&warr) + h_tm1.dot(&uarr) + barr;
        let i = math_helper::sigmoid_arr1(s_t.slice(ndarray::s![..hunits]));
        let f = math_helper::sigmoid_arr1(s_t.slice(ndarray::s![hunits..2 * hunits]));
        let _c = math_helper::tanh_arr1(s_t.slice(ndarray::s![2 * hunits..3 * hunits]));
        let o = math_helper::sigmoid_arr1(s_t.slice(ndarray::s![3 * hunits..]));
        let c_t = i * _c + f * c_tm1;
        let h_t = o * math_helper::tanh_arr1(c_t.view());
        (h_t, c_t)
    }

    /// `word_segmenter` is a function that gets a "clean" unsegmented string as its input and returns a BIES (B: Beginning, I: Inside, E: End,
    /// S: Single) sequence for grapheme clusters. The boundaries of words can be found easily using this BIES sequence.
    pub fn word_segmenter(&self, input: &str) -> String {
        // input_seq is a sequence of id numbers that represents grapheme clusters or code points in the input line. These ids are used later
        // in the embedding layer of the model.
        // Already checked that the name of the model is either "codepoints" or "graphclsut"
        let input_seq: Vec<i16> = if let Some(grapheme) = self.grapheme {
            grapheme
                .segment_str(input)
                .collect::<Vec<usize>>()
                .windows(2)
                .map(|chunk| {
                    self.return_id(
                        input
                            .get(*chunk.get(0).unwrap_or(&0)..*chunk.get(1).unwrap_or(&input.len()))
                            .unwrap_or(input),
                    )
                })
                .collect()
        } else {
            input
                .chars()
                .map(|c| self.return_id(&c.to_string()))
                .collect()
        };

        // x_data is the data ready to be feed into the model
        let input_seq_len = input_seq.len();

        // hunits is the number of hidden unints in each LSTM cell
        let hunits = self.hunits;
        // Forward LSTM
        let mut c_fw = Array1::<f32>::zeros(hunits);
        let mut h_fw = Array1::<f32>::zeros(hunits);
        let mut all_h_fw = Array2::<f32>::zeros((input_seq_len, hunits));
        for (i, g_id) in input_seq.iter().enumerate() {
            let x_t = self.mat1.slice(ndarray::s![*g_id as isize, ..]);
            let (new_h, new_c) = self.compute_hc(
                x_t,
                &h_fw,
                &c_fw,
                self.mat2.view(),
                self.mat3.view(),
                self.mat4.view(),
                hunits,
            );
            h_fw = new_h;
            c_fw = new_c;
            all_h_fw = math_helper::change_row(all_h_fw, i, &h_fw);
        }

        // Backward LSTM
        let mut c_bw = Array1::<f32>::zeros(hunits);
        let mut h_bw = Array1::<f32>::zeros(hunits);
        let mut all_h_bw = Array2::<f32>::zeros((input_seq_len, hunits));
        for (i, g_id) in input_seq.iter().rev().enumerate() {
            let x_t = self.mat1.slice(ndarray::s![*g_id as isize, ..]);
            let (new_h, new_c) = self.compute_hc(
                x_t,
                &h_bw,
                &c_bw,
                self.mat5.view(),
                self.mat6.view(),
                self.mat7.view(),
                self.backward_hunits,
            );
            h_bw = new_h;
            c_bw = new_c;
            all_h_bw = math_helper::change_row(all_h_bw, input_seq_len - 1 - i, &h_bw);
        }

        // Combining forward and backward LSTMs using the dense time-distributed layer
        let timew = self.mat8.view();
        let timeb = self.mat9.view();
        let mut bies = String::new();
        for i in 0..input_seq_len {
            let curr_fw = all_h_fw.slice(ndarray::s![i, ..]);
            let curr_bw = all_h_bw.slice(ndarray::s![i, ..]);
            let concat_lstm = math_helper::concatenate_arr1(curr_fw, curr_bw);
            let curr_est = concat_lstm.dot(&timew) + timeb;
            let probs = math_helper::softmax(curr_est);
            // We use `unwrap_or` to fall back and prevent panics.
            bies.push(self.compute_bies(probs).unwrap_or('s'));
        }
        bies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_provider::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::BufReader;

    /// `TestCase` is a struct used to store a single test case.
    /// Each test case has two attributs: `unseg` which denots the unsegmented line, and `true_bies` which indicates the Bies
    /// sequence representing the true segmentation.
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub struct TestCase {
        pub unseg: String,
        pub expected_bies: String,
        pub true_bies: String,
    }

    /// `TestTextData` is a struct to store a vector of `TestCase` that represents a test text.
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub struct TestTextData {
        pub testcases: Vec<TestCase>,
    }

    #[derive(Debug)]
    pub struct TestText {
        pub data: TestTextData,
    }

    impl TestText {
        pub fn new(data: TestTextData) -> Self {
            Self { data }
        }
    }

    fn load_lstm_data(filename: &str) -> DataPayload<LstmDataV1Marker> {
        DataPayload::from_owned_buffer(
            std::fs::read(filename)
                .expect("File can read to end")
                .into_boxed_slice(),
        )
        .map_project(|bytes, _| serde_json::from_slice(bytes).expect("JSON syntax error"))
    }

    fn load_test_text(filename: &str) -> TestTextData {
        let file = File::open(filename).expect("File should be present");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("JSON syntax error")
    }

    #[test]
    fn test_model_loading() {
        let filename =
            "tests/testdata/Thai_graphclust_exclusive_model4_heavy/converted_weights.json";
        let lstm_data = load_lstm_data(filename);
        let grapheme =
            GraphemeClusterSegmenter::try_new_unstable(&icu_testdata::buffer().as_deserializing())
                .expect("Data exists");
        let lstm = Lstm::try_new(&lstm_data, Some(&grapheme)).expect("Test data is invalid");
        assert_eq!(
            lstm.get_model_name(),
            "Thai_graphclust_exclusive_model4_heavy"
        );
    }

    #[test]
    fn segment_file_by_lstm() {
        // Choosing the embedding system. It can be "graphclust" or "codepoints".
        let embedding: &str = "codepoints";
        let mut model_filename = "tests/testdata/Thai_".to_owned();
        model_filename.push_str(embedding);
        model_filename.push_str("_exclusive_model4_heavy/weights.json");
        let lstm_data = load_lstm_data(&model_filename);
        let lstm = Lstm::try_new(&lstm_data, None).expect("Test data is invalid");

        // Importing the test data
        let mut test_text_filename = "tests/testdata/test_text_".to_owned();
        test_text_filename.push_str(embedding);
        test_text_filename.push_str(".json");
        let test_text_data = load_test_text(&test_text_filename);
        let test_text = TestText::new(test_text_data);

        // Testing
        for test_case in test_text.data.testcases {
            let lstm_output = lstm.word_segmenter(&test_case.unseg);
            println!("Test case      : {}", test_case.unseg);
            println!("Expected bies  : {}", test_case.expected_bies);
            println!("Estimated bies : {}", lstm_output);
            println!("True bies      : {}", test_case.true_bies);
            println!("****************************************************");
            assert_eq!(test_case.expected_bies, lstm_output);
        }
    }
}
