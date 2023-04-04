// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::lstm_bies::{Bies, Lstm};
use crate::provider::{LstmDataV1Marker, RuleBreakDataV1};
use alloc::boxed::Box;
use alloc::string::String;
use core::char::{decode_utf16, REPLACEMENT_CHARACTER};
use icu_provider::DataPayload;

// A word break iterator using LSTM model. Input string have to be same language.

pub struct LstmSegmenterIterator<'s> {
    input: &'s str,
    bies_str: Box<[Bies]>,
    pos: usize,
    pos_utf8: usize,
}

impl Iterator for LstmSegmenterIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        #[allow(clippy::indexing_slicing)] // pos_utf8 in range
        loop {
            let bies = *self.bies_str.get(self.pos)?;
            self.pos_utf8 += self.input[self.pos_utf8..].chars().next()?.len_utf8();
            self.pos += 1;
            if bies == Bies::E && self.bies_str.len() > self.pos {
                return Some(self.pos_utf8);
            }
        }
    }
}

impl<'s> LstmSegmenterIterator<'s> {
    pub fn new(lstm: &Lstm, input: &'s str) -> Self {
        let lstm_output = lstm.word_segmenter(input);
        Self {
            input,
            bies_str: lstm_output,
            pos: 0,
            pos_utf8: 0,
        }
    }
}

pub struct LstmSegmenterIteratorUtf16 {
    bies_str: Box<[Bies]>,
    pos: usize,
}

impl Iterator for LstmSegmenterIteratorUtf16 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let bies = *self.bies_str.get(self.pos)?;
            self.pos += 1;
            if bies == Bies::E && self.bies_str.len() > self.pos {
                return Some(self.pos);
            }
        }
    }
}

impl LstmSegmenterIteratorUtf16 {
    pub fn new(lstm: &Lstm, input: &[u16]) -> Self {
        let input: String = decode_utf16(input.iter().cloned())
            .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
            .collect();
        let lstm_output = lstm.word_segmenter(&input);
        Self {
            bies_str: lstm_output,
            pos: 0,
        }
    }
}

pub struct LstmSegmenter<'l> {
    lstm: Lstm<'l>,
}

impl<'l> LstmSegmenter<'l> {
    pub fn try_new_unstable(
        payload: &'l DataPayload<LstmDataV1Marker>,
        grapheme: Option<&'l RuleBreakDataV1<'l>>,
    ) -> Option<Self> {
        let lstm = Lstm::try_new(payload.get(), grapheme)?;
        Some(Self { lstm })
    }

    /// Create an LSTM based break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'s>(&self, input: &'s str) -> LstmSegmenterIterator<'s> {
        LstmSegmenterIterator::new(&self.lstm, input)
    }

    /// Create an LSTM based break iterator for a UTF-16 string.
    pub fn segment_utf16(&self, input: &[u16]) -> LstmSegmenterIteratorUtf16 {
        LstmSegmenterIteratorUtf16::new(&self.lstm, input)
    }
}

#[cfg(test)]
mod tests {
    use crate::LineSegmenter;
    use icu_provider::prelude::*;
    use icu_provider_adapters::fork::ForkByKeyProvider;
    use icu_provider_fs::FsDataProvider;
    use std::path::PathBuf;

    fn get_segmenter_testdata_provider() -> impl BufferProvider {
        let segmenter_fs_provider = FsDataProvider::try_new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/provider"),
        )
        .unwrap();
        ForkByKeyProvider::new(segmenter_fs_provider, icu_testdata::buffer())
    }

    #[test]
    #[cfg(feature = "serde")]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";

        let provider = get_segmenter_testdata_provider();
        let segmenter = LineSegmenter::try_new_lstm_with_buffer_provider(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        assert_eq!(breaks, [12, 21, 33, TEST_STR.len()], "Thai test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4, 7, 11, utf16.len()], "Thai test");

        let utf16: [u16; 4] = [0x0e20, 0x0e32, 0x0e29, 0x0e32];
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4], "Thai test");
    }

    #[test]
    fn burmese_word_break() {
        // "Burmese Language" in Burmese
        const TEST_STR: &str = "မြန်မာဘာသာစကား";

        let provider = get_segmenter_testdata_provider();
        let segmenter = LineSegmenter::try_new_lstm_with_buffer_provider(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // LSTM model breaks more characters, but it is better to return [30].
        assert_eq!(breaks, [12, 18, 30, TEST_STR.len()], "Burmese test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        // LSTM model breaks more characters, but it is better to return [10].
        assert_eq!(breaks, [4, 6, 10, utf16.len()], "Burmese utf-16 test");
    }

    #[test]
    fn khmer_word_break() {
        const TEST_STR: &str = "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស";

        let provider = get_segmenter_testdata_provider();
        let segmenter = LineSegmenter::try_new_lstm_with_buffer_provider(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: This small sample matches the ICU dictionary segmenter
        assert_eq!(breaks, [39, 48, 54, 72, TEST_STR.len()], "Khmer test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [13, 16, 18, 24, utf16.len()], "Khmer utf-16 test");
    }

    #[test]
    fn lao_word_break() {
        const TEST_STR: &str = "ກ່ຽວກັບສິດຂອງມະນຸດ";

        let provider = get_segmenter_testdata_provider();
        let segmenter = LineSegmenter::try_new_lstm_with_buffer_provider(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: LSTM finds a break at '12' that the dictionary does not find
        assert_eq!(breaks, [12, 21, 30, 39, TEST_STR.len()], "Lao test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4, 7, 10, 13, utf16.len()], "Lao utf-16 test");
    }
}
