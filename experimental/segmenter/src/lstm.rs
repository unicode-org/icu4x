// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::lstm_bies::{Bies, Lstm};
use crate::provider::{LstmDataV1Marker, RuleBreakDataV1};
use crate::SegmenterError;
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
    ) -> Result<Self, SegmenterError> {
        let lstm = Lstm::try_new(payload, grapheme)?;
        Ok(Self { lstm })
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
    use crate::lstm::*;
    use icu_locid::locale;
    use icu_provider::prelude::*;

    #[test]
    #[cfg(feature = "serde")]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";

        let payload = icu_testdata::buffer()
            .as_deserializing()
            .load(DataRequest {
                locale: &DataLocale::from(locale!("th")),
                metadata: Default::default(),
            })
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let segmenter = LstmSegmenter::try_new_unstable(&payload, None).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        assert_eq!(breaks, [12, 21, 33], "Thai test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4, 7, 11], "Thai test");

        //let utf16: [u16; 4] = [0x0e20, 0x0e32, 0x0e29, 0x0e32];
        //let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        //assert_eq!(breaks, [4], "Thai test");
    }
}
