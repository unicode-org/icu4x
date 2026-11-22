// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::grapheme::GraphemeClusterSegmenterBorrowed;
use crate::indices::Utf16Indices;
use crate::provider::*;
use crate::scaffold::{PotentiallyIllFormedUtf8, RuleBreakType, Utf8, Utf16};
use alloc::vec::Vec;
use potential_utf::PotentialUtf8;
use utf8_iter::{Utf8CharIndices, Utf8Chars};
use zerovec::maps::ZeroMapBorrowed;

mod kernel;
mod matrix;
use alloc::vec;
use kernel::Direction;
use matrix::*;
use zerovec::ule::AsULE;

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
pub(super) struct LstmSegmenter<'data> {
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
    grapheme: Option<GraphemeClusterSegmenterBorrowed<'data>>,
}

impl<'data> LstmSegmenter<'data> {
    /// Returns `Err` if grapheme data is required but not present
    pub(super) fn new(
        lstm: &'data LstmData<'data>,
        grapheme: GraphemeClusterSegmenterBorrowed<'data>,
    ) -> Self {
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
                                char::decode_utf16(grapheme_cluster.iter().copied()).flat_map(
                                    |c| {
                                        let mut buf = [0; 4];
                                        let len = c
                                            .unwrap_or(char::REPLACEMENT_CHARACTER)
                                            .encode_utf8(&mut buf)
                                            .len();
                                        buf.into_iter().take(len)
                                    },
                                ),
                            )
                        })
                        .unwrap_or_else(|| self.dic.len() as u16)
                })
                .collect()
        } else {
            char::decode_utf16(input.iter().copied())
                .map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER))
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
    fw: Direction,
    timew_fw: MatrixZero<'data, 2>,
    timew_bw: MatrixZero<'data, 2>,
    time_b: MatrixZero<'data, 1>,
    /// For each position, the index of its id in the sorted distinct ids of the input.
    input_seq: core::iter::Enumerate<vec::IntoIter<u16>>,
    /// The backward hidden states, `hunits` values per position.
    h_bw: Vec<f32>,
    h_fw: Vec<f32>,
    c_fw: Vec<f32>,
    scratch: Vec<f32>,
}

impl<'data> BiesIterator<'data> {
    // input_seq is a sequence of id numbers that represents grapheme clusters or code points in the input line. These ids are used later
    // in the embedding layer of the model.
    fn new(
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
        }: LstmSegmenter<'data>,
        input_seq: Vec<u16>,
    ) -> Self {
        let hunits = fw_u.dim().1;

        let mut ids = input_seq.clone();
        ids.sort_unstable();
        ids.dedup();
        let slots: Vec<u16> = input_seq
            .iter()
            .map(|id| ids.binary_search(id).unwrap_or_default() as u16)
            .collect();

        // Backward LSTM. Its weights are dropped before the forward weights are
        // prepared, so only one direction is held in memory at a time.
        let bw = Direction::new(embedding, bw_w, bw_u, bw_b, &ids);
        let mut scratch = vec![0.0; bw.scratch_len()];
        let mut c_bw = vec![0.0; hunits];
        let mut h = vec![0.0; hunits];
        let mut h_bw = vec![0.0; slots.len() * hunits];
        for (&slot, h_out) in slots.iter().zip(h_bw.chunks_exact_mut(hunits.max(1))).rev() {
            bw.step(slot as usize, &mut h, &mut c_bw, &mut scratch);
            h_out.copy_from_slice(&h);
        }
        drop(bw);

        let fw = Direction::new(embedding, fw_w, fw_u, fw_b, &ids);

        Self {
            fw,
            timew_fw,
            timew_bw,
            time_b,
            input_seq: slots.into_iter().enumerate(),
            h_bw,
            h_fw: vec![0.0; hunits],
            c_fw: vec![0.0; hunits],
            scratch,
        }
    }
}

impl Iterator for BiesIterator<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, slot) = self.input_seq.next()?;
        let hunits = self.h_fw.len();

        self.fw.step(
            slot as usize,
            &mut self.h_fw,
            &mut self.c_fw,
            &mut self.scratch,
        );

        let curr_bw = self
            .h_bw
            .get(i * hunits..(i + 1) * hunits)
            .unwrap_or_default();
        let mut weights = [0.0; 4];
        for (((w, fw_row), bw_row), b) in weights
            .iter_mut()
            .zip(
                self.timew_fw
                    .as_slice()
                    .as_ule_slice()
                    .chunks_exact(hunits.max(1)),
            )
            .zip(
                self.timew_bw
                    .as_slice()
                    .as_ule_slice()
                    .chunks_exact(hunits.max(1)),
            )
            .zip(self.time_b.as_slice().iter())
        {
            let fw: f32 = self
                .h_fw
                .iter()
                .zip(fw_row)
                .map(|(h, w)| h * f32::from_unaligned(*w))
                .sum();
            let bw: f32 = curr_bw
                .iter()
                .zip(bw_row)
                .map(|(h, w)| h * f32::from_unaligned(*w))
                .sum();
            *w = fw + bw + b;
        }
        // For correct BIES weight calculation we'd now have to apply softmax, however
        // we're only doing a naive argmax, so a monotonic function doesn't make a difference.

        Some(weights[2] > weights[0] && weights[2] > weights[1] && weights[2] > weights[3])
    }
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
