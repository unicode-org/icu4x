// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractGraphemeClusterSegmenterBorrowed;
use crate::indices::Utf16Indices;
use crate::provider::*;
#[cfg(feature = "unstable")]
use crate::scaffold::PotentiallyIllFormedUtf8;
use crate::scaffold::{RuleBreakType, Utf8, Utf16};
use alloc::vec::Vec;
use core::char::{REPLACEMENT_CHARACTER, decode_utf16};
use potential_utf::PotentialUtf8;
#[cfg(feature = "unstable")]
use utf8_iter::{Utf8CharIndices, Utf8Chars};
use zerovec::maps::ZeroMapBorrowed;

mod matrix;
use matrix::*;

// A word break iterator using LSTM model. Input string have to be same complex script.
#[derive(Debug)]
pub(super) struct LstmSegmenterIterator<'data, 's, R: RuleBreakType> {
    chars: R::IterAttr<'s>,
    bies: BiesIterator<'data>,
}

impl<R: RuleBreakType> Iterator for LstmSegmenterIterator<'_, '_, R> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let is_e = self.bies.next()?;
            let (idx, ch) = self.chars.next()?;
            if is_e || self.bies.input_seq.len() == 0 {
                return Some(idx + R::char_len(ch));
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct LstmSegmenter<'data, G: AbstractGraphemeClusterSegmenterBorrowed<'data>> {
    dic: ZeroMapBorrowed<'data, PotentialUtf8, u16>,
    embedding: MatrixZero<'data, 2>,
    fw_w: MatrixZero<'data, 3>,
    fw_u: MatrixZero<'data, 3>,
    fw_b: MatrixZero<'data, 2>,
    bw_w: MatrixZero<'data, 3>,
    bw_u: MatrixZero<'data, 3>,
    bw_b: MatrixZero<'data, 2>,
    timew_fw: MatrixZero<'data, 2>,
    timew_bw: MatrixZero<'data, 2>,
    time_b: MatrixZero<'data, 1>,
    grapheme: Option<G>,
}

impl<'data, G: AbstractGraphemeClusterSegmenterBorrowed<'data>> LstmSegmenter<'data, G> {
    /// Returns `Err` if grapheme data is required but not present
    pub(super) fn new(lstm: &'data LstmData<'data>, grapheme: G) -> Self {
        let LstmData::Float32(lstm) = lstm;
        let time_w = MatrixZero::from(&lstm.time_w);
        #[expect(clippy::unwrap_used)] // shape (2, 4, hunits)
        let timew_fw = time_w.submatrix(0).unwrap();
        #[expect(clippy::unwrap_used)] // shape (2, 4, hunits)
        let timew_bw = time_w.submatrix(1).unwrap();
        Self {
            dic: lstm.dic.as_borrowed(),
            embedding: MatrixZero::from(&lstm.embedding),
            fw_w: MatrixZero::from(&lstm.fw_w),
            fw_u: MatrixZero::from(&lstm.fw_u),
            fw_b: MatrixZero::from(&lstm.fw_b),
            bw_w: MatrixZero::from(&lstm.bw_w),
            bw_u: MatrixZero::from(&lstm.bw_u),
            bw_b: MatrixZero::from(&lstm.bw_b),
            timew_fw,
            timew_bw,
            time_b: MatrixZero::from(&lstm.time_b),
            grapheme: (lstm.model == ModelType::GraphemeClusters).then_some(grapheme),
        }
    }

    /// Create an LSTM based break iterator for an `str` (a UTF-8 string).
    pub(super) fn segment_str<'s>(self, input: &'s str) -> LstmSegmenterIterator<'data, 's, Utf8> {
        let input_seq = if let Some(grapheme) = self.grapheme {
            grapheme
                .segment_str(input)
                .collect::<Vec<usize>>()
                .windows(2)
                .map(|chunk| {
                    let range = if let [first, second, ..] = chunk {
                        *first..*second
                    } else {
                        unreachable!()
                    };
                    let grapheme_cluster = if let Some(grapheme_cluster) = input.get(range) {
                        grapheme_cluster
                    } else {
                        return self.dic.len() as u16;
                    };

                    self.dic
                        .get_copied(PotentialUtf8::from_str(grapheme_cluster))
                        .unwrap_or_else(|| self.dic.len() as u16)
                })
                .collect()
        } else {
            input
                .chars()
                .map(|c| {
                    self.dic
                        .get_copied(PotentialUtf8::from_str(c.encode_utf8(&mut [0; 4])))
                        .unwrap_or_else(|| self.dic.len() as u16)
                })
                .collect()
        };
        LstmSegmenterIterator {
            chars: input.char_indices(),
            bies: BiesIterator::new(self, input_seq),
        }
    }

    /// Create an LSTM based break iterator for a UTF-8 string.
    #[cfg(feature = "unstable")]
    pub(super) fn segment_utf8<'s>(
        self,
        input: &'s [u8],
    ) -> LstmSegmenterIterator<'data, 's, PotentiallyIllFormedUtf8> {
        let input_seq = if let Some(grapheme) = self.grapheme {
            grapheme
                .segment_utf8(input)
                .collect::<Vec<usize>>()
                .windows(2)
                .map(|chunk| {
                    let range = if let [first, second, ..] = chunk {
                        *first..*second
                    } else {
                        unreachable!()
                    };
                    let grapheme_cluster = if let Some(grapheme_cluster) = input.get(range) {
                        grapheme_cluster
                    } else {
                        return self.dic.len() as u16;
                    };

                    self.dic
                        .get_copied(PotentialUtf8::from_bytes(grapheme_cluster))
                        .unwrap_or_else(|| self.dic.len() as u16)
                })
                .collect()
        } else {
            Utf8Chars::new(input)
                .map(|c| {
                    self.dic
                        .get_copied(PotentialUtf8::from_str(c.encode_utf8(&mut [0; 4])))
                        .unwrap_or_else(|| self.dic.len() as u16)
                })
                .collect()
        };
        LstmSegmenterIterator {
            chars: Utf8CharIndices::new(input),
            bies: BiesIterator::new(self, input_seq),
        }
    }

    /// Create an LSTM based break iterator for a UTF-16 string.
    pub(super) fn segment_utf16<'s>(
        self,
        input: &'s [u16],
    ) -> LstmSegmenterIterator<'data, 's, Utf16> {
        let input_seq = if let Some(grapheme) = self.grapheme {
            grapheme
                .segment_utf16(input)
                .collect::<Vec<usize>>()
                .windows(2)
                .map(|chunk| {
                    let range = if let [first, second, ..] = chunk {
                        *first..*second
                    } else {
                        unreachable!()
                    };
                    let grapheme_cluster = if let Some(grapheme_cluster) = input.get(range) {
                        grapheme_cluster
                    } else {
                        return self.dic.len() as u16;
                    };

                    self.dic
                        .get_copied_by(|key| {
                            key.as_bytes().iter().copied().cmp(
                                decode_utf16(grapheme_cluster.iter().copied()).flat_map(|c| {
                                    let mut buf = [0; 4];
                                    let len = c
                                        .unwrap_or(REPLACEMENT_CHARACTER)
                                        .encode_utf8(&mut buf)
                                        .len();
                                    buf.into_iter().take(len)
                                }),
                            )
                        })
                        .unwrap_or_else(|| self.dic.len() as u16)
                })
                .collect()
        } else {
            decode_utf16(input.iter().copied())
                .map(|c| c.unwrap_or(REPLACEMENT_CHARACTER))
                .map(|c| {
                    self.dic
                        .get_copied(PotentialUtf8::from_str(c.encode_utf8(&mut [0; 4])))
                        .unwrap_or_else(|| self.dic.len() as u16)
                })
                .collect()
        };
        LstmSegmenterIterator {
            chars: Utf16Indices::new(input),
            bies: BiesIterator::new(self, input_seq),
        }
    }
}

#[derive(Debug)]
struct BiesIterator<'data> {
    embedding: MatrixZero<'data, 2>,
    fw_w: MatrixZero<'data, 3>,
    fw_u: MatrixZero<'data, 3>,
    fw_b: MatrixZero<'data, 2>,
    timew_fw: MatrixZero<'data, 2>,
    timew_bw: MatrixZero<'data, 2>,
    time_b: MatrixZero<'data, 1>,
    input_seq: core::iter::Enumerate<alloc::vec::IntoIter<u16>>,
    h_bw: MatrixOwned<2>,
    curr_fw: MatrixOwned<1>,
    c_fw: MatrixOwned<1>,
}

impl<'data> BiesIterator<'data> {
    // input_seq is a sequence of id numbers that represents grapheme clusters or code points in the input line. These ids are used later
    // in the embedding layer of the model.
    fn new<G: AbstractGraphemeClusterSegmenterBorrowed<'data>>(
        LstmSegmenter {
            embedding,
            fw_w,
            fw_u,
            fw_b,
            bw_w,
            bw_u,
            bw_b,
            timew_fw,
            timew_bw,
            time_b,
            ..
        }: LstmSegmenter<'data, G>,
        input_seq: Vec<u16>,
    ) -> Self {
        let hunits = fw_u.dim().1;

        // Backward LSTM
        let mut c_bw = MatrixOwned::<1>::new_zero([hunits]);
        let mut h_bw = MatrixOwned::<2>::new_zero([input_seq.len(), hunits]);
        for (i, &g_id) in input_seq.iter().enumerate().rev() {
            if i + 1 < input_seq.len() {
                h_bw.as_mut().copy_submatrix::<1>(i + 1, i);
            }
            #[expect(clippy::unwrap_used)]
            compute_hc(
                embedding.submatrix::<1>(g_id as usize).unwrap(), /* shape (dict.len() + 1, hunit), g_id is at most dict.len() */
                h_bw.submatrix_mut(i).unwrap(),                   // shape (input_seq.len(), hunits)
                c_bw.as_mut(),
                bw_w,
                bw_u,
                bw_b,
            );
        }

        Self {
            input_seq: input_seq.into_iter().enumerate(),
            h_bw,
            c_fw: MatrixOwned::<1>::new_zero([hunits]),
            curr_fw: MatrixOwned::<1>::new_zero([hunits]),
            embedding,
            fw_w,
            fw_u,
            fw_b,
            timew_fw,
            timew_bw,
            time_b,
        }
    }
}

impl Iterator for BiesIterator<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, g_id) = self.input_seq.next()?;

        #[expect(clippy::unwrap_used)]
        compute_hc(
            self.embedding.submatrix::<1>(g_id as usize).unwrap(), // shape (dict.len() + 1, hunit), g_id is at most dict.len()
            self.curr_fw.as_mut(),
            self.c_fw.as_mut(),
            self.fw_w,
            self.fw_u,
            self.fw_b,
        );

        #[expect(clippy::unwrap_used)] // shape (input_seq.len(), hunits)
        let curr_bw = self.h_bw.submatrix::<1>(i).unwrap();
        let mut weights = [0.0; 4];
        let mut curr_est = MatrixBorrowedMut {
            data: &mut weights,
            dims: [4],
        };
        curr_est.add_dot_2d(self.curr_fw.as_borrowed(), self.timew_fw);
        curr_est.add_dot_2d(curr_bw, self.timew_bw);
        #[expect(clippy::unwrap_used)] // both shape (4)
        curr_est.add(self.time_b).unwrap();
        // For correct BIES weight calculation we'd now have to apply softmax, however
        // we're only doing a naive argmax, so a monotonic function doesn't make a difference.

        Some(weights[2] > weights[0] && weights[2] > weights[1] && weights[2] > weights[3])
    }
}

/// `compute_hc1` implemens the evaluation of one LSTM layer.
fn compute_hc<'a>(
    x_t: MatrixZero<'a, 1>,
    mut h_tm1: MatrixBorrowedMut<'a, 1>,
    mut c_tm1: MatrixBorrowedMut<'a, 1>,
    w: MatrixZero<'a, 3>,
    u: MatrixZero<'a, 3>,
    b: MatrixZero<'a, 2>,
) {
    #[cfg(debug_assertions)]
    {
        let hunits = h_tm1.dim();
        let embedd_dim = x_t.dim();
        c_tm1.as_borrowed().debug_assert_dims([hunits]);
        w.debug_assert_dims([4, hunits, embedd_dim]);
        u.debug_assert_dims([4, hunits, hunits]);
        b.debug_assert_dims([4, hunits]);
    }

    let mut s_t = b.to_owned();

    s_t.as_mut().add_dot_3d_2(x_t, w);
    s_t.as_mut().add_dot_3d_1(h_tm1.as_borrowed(), u);

    #[expect(clippy::unwrap_used)] // first dimension is 4
    s_t.submatrix_mut::<1>(0).unwrap().sigmoid_transform();
    #[expect(clippy::unwrap_used)] // first dimension is 4
    s_t.submatrix_mut::<1>(1).unwrap().sigmoid_transform();
    #[expect(clippy::unwrap_used)] // first dimension is 4
    s_t.submatrix_mut::<1>(2).unwrap().tanh_transform();
    #[expect(clippy::unwrap_used)] // first dimension is 4
    s_t.submatrix_mut::<1>(3).unwrap().sigmoid_transform();

    #[expect(clippy::unwrap_used)] // first dimension is 4
    c_tm1.convolve(
        s_t.as_borrowed().submatrix(0).unwrap(),
        s_t.as_borrowed().submatrix(2).unwrap(),
        s_t.as_borrowed().submatrix(1).unwrap(),
    );

    #[expect(clippy::unwrap_used)] // first dimension is 4
    h_tm1.mul_tanh(s_t.as_borrowed().submatrix(3).unwrap(), c_tm1.as_borrowed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GraphemeClusterSegmenter;
    use icu_provider::prelude::*;
    use serde::Deserialize;

    /// `TestCase` is a struct used to store a single test case.
    /// Each test case has two attributes: `unseg` which denotes the unsegmented line, and `true_bies` which indicates the Bies
    /// sequence representing the true segmentation.
    #[derive(PartialEq, Debug, Deserialize)]
    struct TestCase {
        unseg: String,
        expected_bies: String,
        true_bies: String,
    }

    /// `TestTextData` is a struct to store a vector of `TestCase` that represents a test text.
    #[derive(PartialEq, Debug, Deserialize)]
    struct TestTextData {
        testcases: Vec<TestCase>,
    }

    #[derive(Debug)]
    struct TestText {
        data: TestTextData,
    }

    #[test]
    fn segment_file_by_lstm() {
        let lstm: DataResponse<SegmenterLstmAutoV1> = Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(
                    DataMarkerAttributes::from_str_or_panic(
                        "Thai_codepoints_exclusive_model4_heavy",
                    ),
                ),
                ..Default::default()
            })
            .unwrap();
        let lstm = LstmSegmenter::new(lstm.payload.get(), GraphemeClusterSegmenter::new());

        // Importing the test data
        let test_text_data = serde_json::from_str(if lstm.grapheme.is_some() {
            include_str!("../../../tests/testdata/test_text_graphclust.json")
        } else {
            include_str!("../../../tests/testdata/test_text_codepoints.json")
        })
        .expect("JSON syntax error");
        let test_text = TestText {
            data: test_text_data,
        };

        // Testing
        for test_case in &test_text.data.testcases {
            let lstm_output = lstm
                .segment_str(&test_case.unseg)
                .bies
                .map(|is_e| if is_e { 'e' } else { '?' })
                .collect::<String>();
            println!("Test case      : {}", test_case.unseg);
            println!("Expected bies  : {}", test_case.expected_bies);
            println!("Estimated bies : {lstm_output}");
            println!("True bies      : {}", test_case.true_bies);
            println!("****************************************************");
            assert_eq!(
                test_case.expected_bies.replace(['b', 'i', 's'], "?"),
                lstm_output
            );
        }
    }
}
