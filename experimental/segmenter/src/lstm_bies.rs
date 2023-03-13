// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::grapheme::GraphemeClusterSegmenter;
use crate::lstm_error::Error;
use crate::math_helper::MatrixBorrowed;
use crate::math_helper::{self, to_mat_type, MatrixBorrowedMut, MatrixOwned, ZERO};
use crate::provider::{LstmDataV1Marker, RuleBreakDataV1};
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::str;
use icu_provider::DataPayload;
use zerovec::ule::AsULE;

pub struct Lstm<'l> {
    data: &'l DataPayload<LstmDataV1Marker>,
    mat1: MatrixOwned<2>,
    mat2: MatrixOwned<3>,
    mat3: MatrixOwned<3>,
    mat4: MatrixOwned<2>,
    mat5: MatrixOwned<3>,
    mat6: MatrixOwned<3>,
    mat7: MatrixOwned<2>,
    mat8: MatrixOwned<3>,
    mat9: MatrixOwned<1>,
    grapheme: Option<&'l RuleBreakDataV1<'l>>,
    hunits: usize,
    backward_hunits: usize,
}

impl<'l> Lstm<'l> {
    /// `try_new` is the initiator of struct `Lstm`
    pub fn try_new(
        data: &'l DataPayload<LstmDataV1Marker>,
        grapheme: Option<&'l RuleBreakDataV1<'l>>,
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

        // TODO: Perform this matrix reshaping in datagen.
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
        let hunits = *mat3.shape().first().ok_or(Error::DimensionMismatch)?;
        let backward_hunits = *mat6.shape().first().ok_or(Error::DimensionMismatch)?;
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
        let mut mat2 = mat2.into_shape((embedd_dim, 4, hunits))?;
        let mut mat3 = mat3.into_shape((hunits, 4, hunits))?;
        let mut mat4 = mat4.into_shape((4, hunits))?;
        let mut mat5 = mat5.into_shape((embedd_dim, 4, hunits))?;
        let mut mat6 = mat6.into_shape((hunits, 4, hunits))?;
        let mut mat7 = mat7.into_shape((4, hunits))?;
        let mut mat8 = mat8.into_shape((2, hunits, 4))?;
        mat2.swap_axes(0, 2);
        mat3.swap_axes(0, 2);
        mat4.swap_axes(0, 1);
        mat5.swap_axes(0, 2);
        mat6.swap_axes(0, 2);
        mat7.swap_axes(0, 1);
        mat8.swap_axes(1, 2);
        let mat2 = mat2.as_standard_layout().into_owned();
        let mat3 = mat3.as_standard_layout().into_owned();
        let mat4 = mat4.as_standard_layout().into_owned();
        let mat5 = mat5.as_standard_layout().into_owned();
        let mat6 = mat6.as_standard_layout().into_owned();
        let mat7 = mat7.as_standard_layout().into_owned();
        let mat8 = mat8.as_standard_layout().into_owned();
        Ok(Self {
            data,
            mat1: MatrixOwned::from_ndarray(mat1.mapv(to_mat_type)),
            mat2: MatrixOwned::from_ndarray(mat2.mapv(to_mat_type)),
            mat3: MatrixOwned::from_ndarray(mat3.mapv(to_mat_type)),
            mat4: MatrixOwned::from_ndarray(mat4.mapv(to_mat_type)),
            mat5: MatrixOwned::from_ndarray(mat5.mapv(to_mat_type)),
            mat6: MatrixOwned::from_ndarray(mat6.mapv(to_mat_type)),
            mat7: MatrixOwned::from_ndarray(mat7.mapv(to_mat_type)),
            mat8: MatrixOwned::from_ndarray(mat8.mapv(to_mat_type)),
            mat9: MatrixOwned::from_ndarray(mat9.mapv(to_mat_type)),
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
    fn compute_bies(&self, arr: MatrixBorrowed<1>) -> Result<char, Error> {
        let ind = arr.argmax();
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
    #[allow(clippy::too_many_arguments)]
    #[must_use] // return value is GIGO path
    fn compute_hc<'a>(
        &self,
        x_t: MatrixBorrowed<'a, 1>,
        mut h_tm1: MatrixBorrowedMut<'a, 1>,
        mut c_tm1: MatrixBorrowedMut<'a, 1>,
        warr: MatrixBorrowed<'a, 3>,
        uarr: MatrixBorrowed<'a, 3>,
        barr: MatrixBorrowed<'a, 2>,
        hunits: usize,
    ) -> Option<()> {
        let embedd_dim = x_t.dim();
        #[cfg(debug_assertions)]
        {
            h_tm1.as_borrowed().debug_assert_dims([hunits]);
            c_tm1.as_borrowed().debug_assert_dims([hunits]);
            warr.debug_assert_dims([hunits, 4, embedd_dim]);
            uarr.debug_assert_dims([hunits, 4, hunits]);
            barr.debug_assert_dims([hunits, 4]);
        }

        let mut s_t = barr.to_owned();

        s_t.as_mut().add_dot_3d(x_t, warr);
        s_t.as_mut().add_dot_3d(h_tm1.as_borrowed(), uarr);

        for i in 0..hunits {
            let tuple = s_t
                .as_borrowed()
                .submatrix::<1>(i)
                .and_then(|s| s.read_4())
                .unwrap_or((ZERO, ZERO, ZERO, ZERO));
            let p = math_helper::sigmoid(tuple.0);
            let f = math_helper::sigmoid(tuple.1);
            let c = math_helper::tanh(tuple.2);
            let o = math_helper::sigmoid(tuple.3);
            let c_old = c_tm1.as_borrowed().as_slice().get(i)?;
            let c_new = math_helper::muldiv(p, c) + math_helper::muldiv(f, *c_old);
            *c_tm1.as_mut_slice().get_mut(i)? = c_new;
            *h_tm1.as_mut_slice().get_mut(i)? = math_helper::muldiv(o, math_helper::tanh(c_new));
        }
        Some(())
    }

    /// `word_segmenter` is a function that gets a "clean" unsegmented string as its input and returns a BIES (B: Beginning, I: Inside, E: End,
    /// S: Single) sequence for grapheme clusters. The boundaries of words can be found easily using this BIES sequence.
    pub fn word_segmenter(&self, input: &str) -> String {
        match self.word_segmenter_inner(input) {
            Some(s) => s,
            None => {
                debug_assert!(false);
                String::new()
            }
        }
    }

    fn word_segmenter_inner(&self, input: &str) -> Option<String> {
        // input_seq is a sequence of id numbers that represents grapheme clusters or code points in the input line. These ids are used later
        // in the embedding layer of the model.
        // Already checked that the name of the model is either "codepoints" or "graphclsut"
        let input_seq: Vec<i16> = if let Some(grapheme) = self.grapheme {
            GraphemeClusterSegmenter::new_and_segment_str(input, grapheme)
                .collect::<Vec<usize>>()
                .windows(2)
                .map(|chunk| {
                    let range = if let [first, second, ..] = chunk {
                        *first..*second
                    } else {
                        unreachable!()
                    };
                    self.return_id(input.get(range).unwrap_or(input))
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
        let mut c_fw = MatrixOwned::<1>::new_zero([hunits]);
        let mut all_h_fw = MatrixOwned::<2>::new_zero([input_seq_len, hunits]);
        for (i, g_id) in input_seq.iter().enumerate() {
            let x_t = self.mat1.submatrix::<1>(*g_id as usize)?;
            if i > 0 {
                all_h_fw.as_mut().copy_submatrix::<1>(i - 1, i);
            }
            self.compute_hc(
                x_t,
                all_h_fw.submatrix_mut(i)?,
                c_fw.as_mut(),
                self.mat2.as_borrowed(),
                self.mat3.as_borrowed(),
                self.mat4.as_borrowed(),
                hunits,
            )?;
        }

        // Backward LSTM
        let mut c_bw = MatrixOwned::<1>::new_zero([hunits]);
        let mut all_h_bw = MatrixOwned::<2>::new_zero([input_seq_len, hunits]);
        for (i, g_id) in input_seq.iter().rev().enumerate() {
            let x_t = self.mat1.submatrix::<1>(*g_id as usize)?;
            if i > 0 {
                all_h_bw
                    .as_mut()
                    .copy_submatrix::<1>(input_seq_len - i, input_seq_len - i - 1);
            }
            self.compute_hc(
                x_t,
                all_h_bw.submatrix_mut(input_seq_len - i - 1)?,
                c_bw.as_mut(),
                self.mat5.as_borrowed(),
                self.mat6.as_borrowed(),
                self.mat7.as_borrowed(),
                self.backward_hunits,
            )?;
        }

        // Combining forward and backward LSTMs using the dense time-distributed layer
        let timeb = self.mat9.as_borrowed();
        let mut bies = String::new();
        for i in 0..input_seq_len {
            let curr_fw = all_h_fw.submatrix::<1>(i)?;
            let curr_bw = all_h_bw.submatrix::<1>(i)?;
            let timew_fw = self.mat8.submatrix(0)?;
            let timew_bw = self.mat8.submatrix(1)?;
            // TODO: Make curr_est be stack-allocated
            let mut curr_est = MatrixOwned::<1>::new_zero([4]);
            curr_est.as_mut().add_dot_2d(curr_fw, timew_fw);
            curr_est.as_mut().add_dot_2d(curr_bw, timew_bw);
            curr_est.as_mut().add(timeb)?;
            curr_est.as_mut().softmax_transform();
            // We use `unwrap_or` to fall back and prevent panics.
            bies.push(self.compute_bies(curr_est.as_borrowed()).unwrap_or('s'));
        }
        Some(bies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::GraphemeClusterBreakDataV1Marker;
    use icu_provider::prelude::*;
    use serde::Deserialize;
    use std::fs::File;
    use std::io::BufReader;

    /// `TestCase` is a struct used to store a single test case.
    /// Each test case has two attributs: `unseg` which denots the unsegmented line, and `true_bies` which indicates the Bies
    /// sequence representing the true segmentation.
    #[derive(PartialEq, Debug, Deserialize)]
    pub struct TestCase {
        pub unseg: String,
        pub expected_bies: String,
        pub true_bies: String,
    }

    /// `TestTextData` is a struct to store a vector of `TestCase` that represents a test text.
    #[derive(PartialEq, Debug, Deserialize)]
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
        let grapheme: DataPayload<GraphemeClusterBreakDataV1Marker> = icu_testdata::buffer()
            .as_deserializing()
            .load(Default::default())
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let lstm = Lstm::try_new(&lstm_data, Some(grapheme.get())).expect("Test data is invalid");
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
            println!("Estimated bies : {lstm_output}");
            println!("True bies      : {}", test_case.true_bies);
            println!("****************************************************");
            assert_eq!(test_case.expected_bies, lstm_output);
        }
    }
}
