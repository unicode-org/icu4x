// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::grapheme::GraphemeClusterSegmenter;
use crate::lstm_bies::Lstm;
use crate::provider::LstmDataV1Marker;
use alloc::borrow::ToOwned;
use alloc::string::String;
use core::char::{decode_utf16, REPLACEMENT_CHARACTER};
use icu_provider::{DataError, DataErrorKind, DataPayload};

// A word break iterator using LSTM model. Input string have to be same language.

pub struct LstmSegmenterIterator {
    input: String,
    bies_str: String,
    pos: usize,
    pos_utf8: usize,
}

impl Iterator for LstmSegmenterIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            self.pos_utf8 += self.input.chars().nth(self.pos)?.len_utf8();
            self.pos += 1;
            if ch == 'e' && self.bies_str.len() > self.pos {
                return Some(self.pos_utf8);
            }
        }
    }
}

impl LstmSegmenterIterator {
    pub fn new(lstm: &Lstm, input: &str) -> Self {
        let lstm_output = lstm.word_segmenter(input);
        Self {
            input: input.to_owned(),
            bies_str: lstm_output,
            pos: 0,
            pos_utf8: 0,
        }
    }
}

pub struct LstmSegmenterIteratorUtf16 {
    bies_str: String,
    pos: usize,
}

impl Iterator for LstmSegmenterIteratorUtf16 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            // This ch is always in bitmap.
            self.pos += 1;
            if ch == 'e' && self.bies_str.len() > self.pos {
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
        grapheme: Option<&'l GraphemeClusterSegmenter>,
    ) -> Result<Self, DataError> {
        let lstm = Lstm::try_new(payload, grapheme)
            .map_err(|_| DataErrorKind::MissingPayload.with_type_context::<LstmDataV1Marker>())?;
        Ok(Self { lstm })
    }

    /// Create a dictionary based break iterator for an `str` (a UTF-8 string).
    pub fn segment_str(&self, input: &str) -> LstmSegmenterIterator {
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
    use crate::provider::LstmDataV1;
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

    #[test]
    fn burmese_word_break() {
        // "Burmese Language" in Burmese
        const TEST_STR: &str = "မြန်မာဘာသာစကား";

        const BURMESE_MODEL: &[u8; 475209] =
            include_bytes!("../tests/testdata/json/core/segmenter_lstm@1/my.json");
        let data: LstmDataV1 = serde_json::from_slice(BURMESE_MODEL).expect("JSON syntax error");
        let payload = DataPayload::<LstmDataV1Marker>::from_owned(data);
        let segmenter = LstmSegmenter::try_new_unstable(&payload, None).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // LSTM model breaks more characters, but it is better to return [30].
        assert_eq!(breaks, [12, 18, 30], "Burmese test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        // LSTM model breaks more characters, but it is better to return [10].
        assert_eq!(breaks, [4, 6, 10], "Burmese utf-16 test");
    }

    #[test]
    fn khmer_word_break() {
        const TEST_STR: &str = "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស";
        const KHMER_MODEL: &[u8; 384592] =
            include_bytes!("../tests/testdata/json/core/segmenter_lstm@1/km.json");
        let data: LstmDataV1 = serde_json::from_slice(KHMER_MODEL).expect("JSON syntax error");
        let payload = DataPayload::<LstmDataV1Marker>::from_owned(data);
        let segmenter = LstmSegmenter::try_new_unstable(&payload, None).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: This small sample matches the ICU dictionary segmenter
        assert_eq!(breaks, [39, 48, 54, 72], "Khmer test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [13, 16, 18, 24], "Khmer utf-16 test");
    }

    #[test]
    fn lao_word_break() {
        const TEST_STR: &str = "ກ່ຽວກັບສິດຂອງມະນຸດ";
        const LAO_MODEL: &[u8; 372529] =
            include_bytes!("../tests/testdata/json/core/segmenter_lstm@1/lo.json");
        let data: LstmDataV1 = serde_json::from_slice(LAO_MODEL).expect("JSON syntax error");
        let payload = DataPayload::<LstmDataV1Marker>::from_owned(data);
        let segmenter = LstmSegmenter::try_new_unstable(&payload, None).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: LSTM finds a break at '12' that the dictionary does not find
        assert_eq!(breaks, [12, 21, 30, 39], "Lao test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4, 7, 10, 13], "Lao utf-16 test");
    }

    #[test]
    fn thai_word_break_with_grapheme_model() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";
        // The keys of Lstm JSON data has to be sorted. So this JSON is generated by converter.py in data directory.
        const MODEL: &[u8; 280433] = include_bytes!(
            "../tests/testdata/Thai_graphclust_exclusive_model4_heavy/converted_weights.json"
        );
        let data: LstmDataV1 = serde_json::from_slice(MODEL).expect("JSON syntax error");
        let payload = DataPayload::<LstmDataV1Marker>::from_owned(data);
        let grapheme =
            GraphemeClusterSegmenter::try_new_unstable(&icu_testdata::buffer().as_deserializing())
                .expect("Data exists");
        let segmenter = LstmSegmenter::try_new_unstable(&payload, Some(&grapheme)).expect("");
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        assert_eq!(breaks, [6, 12, 21, 27, 33], "Thai test with grapheme model");
    }
}
