// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::grapheme::GraphemeClusterSegmenter;
use crate::lstm_error::Error;
use crate::math_helper::{self, MatrixBorrowed, MatrixBorrowedMut, MatrixOwned};
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

        // Note: We are currently copying the ZeroVecs into allocated matrices.
        // The ICU4X style guide discourages this. We do it here because:
        // 1. The data need to be aligned in order to be vectorized.
        // 2. The LSTM is highly performance-sensitive.
        let mat1 = data.get().mat1.alloc_matrix::<2>()?;
        let mat2 = data.get().mat2.alloc_matrix::<3>()?;
        let mat3 = data.get().mat3.alloc_matrix::<3>()?;
        let mat4 = data.get().mat4.alloc_matrix::<2>()?;
        let mat5 = data.get().mat5.alloc_matrix::<3>()?;
        let mat6 = data.get().mat6.alloc_matrix::<3>()?;
        let mat7 = data.get().mat7.alloc_matrix::<2>()?;
        let mat8 = data.get().mat8.alloc_matrix::<3>()?;
        let mat9 = data.get().mat9.alloc_matrix::<1>()?;
        let embedd_dim = mat1.as_borrowed().dim().1;
        let hunits = mat3.as_borrowed().dim().0;
        if mat2.as_borrowed().dim() != (hunits, 4, embedd_dim)
            || mat3.as_borrowed().dim() != (hunits, 4, hunits)
            || mat4.as_borrowed().dim() != (hunits, 4)
            || mat5.as_borrowed().dim() != (hunits, 4, embedd_dim)
            || mat6.as_borrowed().dim() != (hunits, 4, hunits)
            || mat7.as_borrowed().dim() != (hunits, 4)
            || mat8.as_borrowed().dim() != (2, 4, hunits)
            || mat9.as_borrowed().dim() != (4)
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
        })
    }

    /// `get_model_name` returns the name of the LSTM model.
    #[allow(dead_code)]
    pub fn get_model_name(&self) -> &str {
        &self.data.get().model
    }

    #[cfg(test)]
    fn embedding_type(&self) -> &'static str {
        if self.grapheme.is_some() {
            "grapheme"
        } else {
            "codepoints"
        }
    }

    // TODO(#421): Use common BIES normalizer code
    /// `compute_bies` uses the computed probabilities of BIES and pick the letter with the largest probability
    fn compute_bies(&self, arr: [f32; 4]) -> char {
        let [b, i, e, s] = arr;
        let mut result = 'b';
        let mut max = b;
        if i > max {
            result = 'i';
            max = i;
        }
        if e > max {
            result = 'e';
            max = e;
        }
        if s > max {
            result = 's';
            // max = s;
        }
        result
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
        #[cfg(debug_assertions)]
        {
            let embedd_dim = x_t.dim();
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
            let [s0, s1, s2, s3] = s_t
                .as_borrowed()
                .submatrix::<1>(i)
                .and_then(|s| s.read_4())?;
            let p = math_helper::sigmoid(s0);
            let f = math_helper::sigmoid(s1);
            let c = math_helper::tanh(s2);
            let o = math_helper::sigmoid(s3);
            let c_old = c_tm1.as_borrowed().as_slice().get(i)?;
            let c_new = p * c + f * c_old;
            *c_tm1.as_mut_slice().get_mut(i)? = c_new;
            *h_tm1.as_mut_slice().get_mut(i)? = o * math_helper::tanh(c_new);
        }
        Some(())
    }

    /// `word_segmenter` is a function that gets a "clean" unsegmented string as its input and returns a BIES (B: Beginning, I: Inside, E: End,
    /// S: Single) sequence for grapheme clusters. The boundaries of words can be found easily using this BIES sequence.
    pub fn word_segmenter(&self, input: &str) -> String {
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
        let input_seq_len = input_seq.len();
        let inner = self.word_segmenter_inner(input_seq);
        debug_assert!(inner.is_some(), "{:?}", input);
        // Fill in a GIGO result of all 's'
        let result = inner.unwrap_or_else(|| "s".repeat(input_seq_len));
        debug_assert_eq!(result.len(), input_seq_len, "{:?}", input);
        result
    }

    fn word_segmenter_inner(&self, input_seq: Vec<i16>) -> Option<String> {
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
                self.hunits,
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
            let weights = curr_est.as_borrowed().read_4()?;
            bies.push(self.compute_bies(weights));
        }
        Some(bies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::locale;
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

    fn load_test_text(filename: &str) -> TestTextData {
        let file = File::open(filename).expect("File should be present");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("JSON syntax error")
    }

    #[test]
    fn segment_file_by_lstm() {
        // Choosing the embedding system. It can be "graphclust" or "codepoints".
        let payload = icu_testdata::buffer()
            .as_deserializing()
            .load(DataRequest {
                locale: &locale!("th").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let lstm = Lstm::try_new(&payload, None).expect("Test data is invalid");

        // Importing the test data
        let mut test_text_filename = "tests/testdata/test_text_".to_owned();
        test_text_filename.push_str(lstm.embedding_type());
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
