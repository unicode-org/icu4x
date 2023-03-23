// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::lstm_bies::Lstm;
use crate::provider::{LstmDataV1Marker, RuleBreakDataV1};
use crate::SegmenterError;
use alloc::string::String;
use core::char::{decode_utf16, REPLACEMENT_CHARACTER};
use icu_provider::DataPayload;

// A word break iterator using LSTM model. Input string have to be same language.

pub struct LstmSegmenterIterator<'a> {
    chars: core::str::CharIndices<'a>,
    bies: alloc::vec::IntoIter<u8>,
}

impl Iterator for LstmSegmenterIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (i, _) = self.chars.next()?;
            let bies = self.bies.next()?;

            if matches!(bies, b'b' | b's') {
                return Some(i);
            }
        }
    }
}

impl<'a> LstmSegmenterIterator<'a> {
    pub fn new(lstm: &Lstm, input: &'a str) -> Self {
        let bies = lstm.word_segmenter(input);
        debug_assert_eq!(bies.len(), input.chars().count());
        let mut chars = input.char_indices();
        let mut bies = bies.into_bytes().into_iter();
        // Skip first char as we don't want to output 0 as the first element
        // TODO: why not actually?
        chars.next();
        bies.next();
        Self { chars, bies }
    }
}

pub struct LstmSegmenterIteratorUtf16 {
    bies: core::iter::Enumerate<alloc::vec::IntoIter<u8>>,
}

impl Iterator for LstmSegmenterIteratorUtf16 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (i, bies) = self.bies.next()?;
            if matches!(bies, b'b' | b's') {
                return Some(i);
            }
        }
    }
}

impl LstmSegmenterIteratorUtf16 {
    pub fn new(lstm: &Lstm, input: &[u16]) -> Self {
        let input: String = decode_utf16(input.iter().copied())
            .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
            .collect();
        let bies = lstm.word_segmenter(&input);
        let mut bies = bies.into_bytes().into_iter().enumerate();
        // Skip first char as we don't want to output 0 as the first element
        // TODO: why not actually?
        bies.next();
        Self { bies }
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

    /// Create a dictionary based break iterator for an `str` (a UTF-8 string).
    pub fn segment_str(&self, input: &'l str) -> LstmSegmenterIterator {
        LstmSegmenterIterator::new(&self.lstm, input)
    }

    /// Create a dictionary based break iterator for a UTF-16 string.
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
