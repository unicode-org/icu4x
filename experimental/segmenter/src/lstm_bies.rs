// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::grapheme::GraphemeClusterSegmenter;
use crate::math_helper::{self, MatrixBorrowedMut, MatrixOwned, MatrixZero};
use crate::provider::{LstmDataV1, ModelType, RuleBreakDataV1};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::str;
use zerovec::maps::ZeroMapBorrowed;

// Polyfill float operations with libm in case we're no_std.
#[allow(unused_imports)]
use num_traits::Float;

pub struct Lstm<'l> {
    dic: ZeroMapBorrowed<'l, str, u16>,
    embedding: MatrixZero<'l, 2>,
    fw_w: MatrixZero<'l, 3>,
    fw_u: MatrixZero<'l, 3>,
    fw_b: MatrixZero<'l, 2>,
    bw_w: MatrixZero<'l, 3>,
    bw_u: MatrixZero<'l, 3>,
    bw_b: MatrixZero<'l, 2>,
    time_w: MatrixZero<'l, 3>,
    time_b: MatrixZero<'l, 1>,
    grapheme: Option<&'l RuleBreakDataV1<'l>>,
    hunits: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Bies {
    B,
    I,
    E,
    S,
}

#[cfg(test)]
impl Bies {
    fn as_char(&self) -> char {
        match self {
            Bies::B => 'b',
            Bies::I => 'i',
            Bies::E => 'e',
            Bies::S => 's',
        }
    }
}

impl<'l> Lstm<'l> {
    /// Returns `None` if grapheme data is required but not present
    pub fn try_new(
        data: &'l LstmDataV1<'l>,
        grapheme: Option<&'l RuleBreakDataV1<'l>>,
    ) -> Option<Self> {
        Some(Self {
            dic: data.dic.as_borrowed(),
            embedding: data.embedding.as_matrix_zero(),
            fw_w: data.fw_w.as_matrix_zero(),
            fw_u: data.fw_u.as_matrix_zero(),
            fw_b: data.fw_b.as_matrix_zero(),
            bw_w: data.bw_w.as_matrix_zero(),
            bw_u: data.bw_u.as_matrix_zero(),
            bw_b: data.bw_b.as_matrix_zero(),
            time_w: data.time_w.as_matrix_zero(),
            time_b: data.time_b.as_matrix_zero(),
            grapheme: if data.model == ModelType::GraphemeClusters {
                Some(grapheme?)
            } else {
                None
            },
            hunits: data.fw_u.as_matrix_zero().dim().0,
        })
    }

    // TODO(#421): Use common BIES normalizer code
    /// `compute_bies` uses the computed probabilities of BIES and pick the letter with the largest probability
    fn compute_bies(&self, arr: [f32; 4]) -> Bies {
        let [b, i, e, s] = arr;
        let mut result = Bies::B;
        let mut max = b;
        if i > max {
            result = Bies::I;
            max = i;
        }
        if e > max {
            result = Bies::E;
            max = e;
        }
        if s > max {
            result = Bies::S;
            // max = s;
        }
        result
    }

    /// `_return_id` returns the id corresponding to a code point or a grapheme cluster based on the model dictionary.
    fn return_id(&self, g: &str) -> u16 {
        self.dic
            .get_copied(g)
            .unwrap_or_else(|| self.dic.len() as u16)
    }

    /// `compute_hc1` implemens the evaluation of one LSTM layer.
    fn compute_hc<'a>(
        &self,
        x_t: MatrixZero<'a, 1>,
        mut h_tm1: MatrixBorrowedMut<'a, 1>,
        mut c_tm1: MatrixBorrowedMut<'a, 1>,
        w: MatrixZero<'a, 3>,
        u: MatrixZero<'a, 3>,
        b: MatrixZero<'a, 2>,
    ) {
        #[cfg(debug_assertions)]
        {
            let embedd_dim = x_t.dim();
            h_tm1.as_borrowed().debug_assert_dims([self.hunits]);
            c_tm1.as_borrowed().debug_assert_dims([self.hunits]);
            w.debug_assert_dims([self.hunits, 4, embedd_dim]);
            u.debug_assert_dims([self.hunits, 4, self.hunits]);
            b.debug_assert_dims([self.hunits, 4]);
        }

        let mut s_t = b.to_owned();

        s_t.as_mut().add_dot_3d_2(x_t, w);
        s_t.as_mut().add_dot_3d_1(h_tm1.as_borrowed(), u);

        #[allow(clippy::unwrap_used)]
        for i in 0..self.hunits {
            let [s0, s1, s2, s3] = s_t
                .as_borrowed()
                .submatrix::<1>(i)
                .unwrap()
                .read_4()
                .unwrap(); // shape (hunits, 4)
            let p = math_helper::sigmoid(s0);
            let f = math_helper::sigmoid(s1);
            let c = math_helper::tanh(s2);
            let o = math_helper::sigmoid(s3);
            let c_old = c_tm1.as_borrowed().as_slice().get(i).unwrap(); // shape (h_units)
            let c_new = p.mul_add(c, f * c_old);
            *c_tm1.as_mut_slice().get_mut(i).unwrap() = c_new; // shape (h_units)
            *h_tm1.as_mut_slice().get_mut(i).unwrap() = o * math_helper::tanh(c_new);
            // shape (hunits)
        }
    }

    /// `word_segmenter` is a function that gets a "clean" unsegmented string as its input and returns a BIES (B: Beginning, I: Inside, E: End,
    /// S: Single) sequence for grapheme clusters. The boundaries of words can be found easily using this BIES sequence.
    pub fn word_segmenter(&self, input: &str) -> Box<[Bies]> {
        // input_seq is a sequence of id numbers that represents grapheme clusters or code points in the input line. These ids are used later
        // in the embedding layer of the model.
        // Already checked that the name of the model is either "codepoints" or "graphclsut"
        let input_seq: Vec<u16> = if let Some(grapheme) = self.grapheme {
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

        // Forward LSTM
        let mut c_fw = MatrixOwned::<1>::new_zero([self.hunits]);
        let mut all_h_fw = MatrixOwned::<2>::new_zero([input_seq.len(), self.hunits]);
        for (i, &g_id) in input_seq.iter().enumerate() {
            #[allow(clippy::unwrap_used)]
            // embedding has shape (dict.len() + 1, hunit), g_id is at most dict.len()
            let x_t = self.embedding.submatrix::<1>(g_id as usize).unwrap();
            if i > 0 {
                all_h_fw.as_mut().copy_submatrix::<1>(i - 1, i);
            }
            #[allow(clippy::unwrap_used)]
            self.compute_hc(
                x_t,
                all_h_fw.submatrix_mut(i).unwrap(), // shape (input_seq.len(), hunits)
                c_fw.as_mut(),
                self.fw_w,
                self.fw_u,
                self.fw_b,
            );
        }

        // Backward LSTM
        let mut c_bw = MatrixOwned::<1>::new_zero([self.hunits]);
        let mut all_h_bw = MatrixOwned::<2>::new_zero([input_seq.len(), self.hunits]);
        for (i, &g_id) in input_seq.iter().enumerate().rev() {
            #[allow(clippy::unwrap_used)]
            // embedding has shape (dict.len() + 1, hunit), g_id is at most dict.len()
            let x_t = self.embedding.submatrix::<1>(g_id as usize).unwrap();
            if i + 1 < input_seq.len() {
                all_h_bw.as_mut().copy_submatrix::<1>(i + 1, i);
            }
            #[allow(clippy::unwrap_used)]
            self.compute_hc(
                x_t,
                all_h_bw.submatrix_mut(i).unwrap(), // shape (input_seq.len(), hunits)
                c_bw.as_mut(),
                self.bw_w,
                self.bw_u,
                self.bw_b,
            );
        }

        #[allow(clippy::unwrap_used)] // shape (2, 4, hunits)
        let timew_fw = self.time_w.submatrix(0).unwrap();
        #[allow(clippy::unwrap_used)] // shape (2, 4, hunits)
        let timew_bw = self.time_w.submatrix(1).unwrap();

        // Combining forward and backward LSTMs using the dense time-distributed layer
        (0..input_seq.len())
            .map(|i| {
                #[allow(clippy::unwrap_used)] // shape (input_seq.len(), hunits)
                let curr_fw = all_h_fw.submatrix::<1>(i).unwrap();
                #[allow(clippy::unwrap_used)] // shape (input_seq.len(), hunits)
                let curr_bw = all_h_bw.submatrix::<1>(i).unwrap();
                let mut weights = [0.0; 4];
                let mut curr_est = MatrixBorrowedMut {
                    data: &mut weights,
                    dims: [4],
                };
                curr_est.add_dot_2d(curr_fw, timew_fw);
                curr_est.add_dot_2d(curr_bw, timew_bw);
                #[allow(clippy::unwrap_used)] // both shape (4)
                curr_est.add(self.time_b).unwrap();
                curr_est.softmax_transform();
                self.compute_bies(weights)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::LstmDataV1Marker;
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
        let payload: DataPayload<LstmDataV1Marker> = icu_testdata::buffer()
            .as_deserializing()
            .load(DataRequest {
                locale: &locale!("th").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let lstm = Lstm::try_new(payload.get(), None).expect("Test data is invalid");

        // Importing the test data
        let mut test_text_filename = "tests/testdata/test_text_".to_owned();
        test_text_filename.push_str(if lstm.grapheme.is_some() {
            "grapheme"
        } else {
            "codepoints"
        });
        test_text_filename.push_str(".json");
        let test_text_data = load_test_text(&test_text_filename);
        let test_text = TestText::new(test_text_data);

        // Testing
        for test_case in test_text.data.testcases {
            let lstm_output = lstm.word_segmenter(&test_case.unseg);
            println!("Test case      : {}", test_case.unseg);
            println!("Expected bies  : {}", test_case.expected_bies);
            println!("Estimated bies : {lstm_output:?}");
            println!("True bies      : {}", test_case.true_bies);
            println!("****************************************************");
            assert_eq!(
                test_case.expected_bies,
                lstm_output.iter().map(Bies::as_char).collect::<String>()
            );
        }
    }
}
