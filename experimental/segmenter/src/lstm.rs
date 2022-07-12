// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::language::*;
use crate::lstm_bies::Lstm;
use crate::lstm_structs::{LstmDataV1, LstmDataV1Marker};

use alloc::string::String;
use alloc::string::ToString;
use core::char::decode_utf16;
use icu_provider::DataError;
use icu_provider::DataPayload;

// TODO:
// json file is big, So I should use anoher binary format like npy.
// But provided npy uses tensorflow dtype.
const THAI_MODEL: &[u8; 373466] =
    include_bytes!("../tests/testdata/json/core/segmenter_lstm@1/th.json");
const BURMESE_MODEL: &[u8; 475209] =
    include_bytes!("../tests/testdata/json/core/segmenter_lstm@1/my.json");
const KHMER_MODEL: &[u8; 384592] =
    include_bytes!("../tests/testdata/json/core/segmenter_lstm@1/km.json");
const LAO_MODEL: &[u8; 372529] =
    include_bytes!("../tests/testdata/json/core/segmenter_lstm@1/lo.json");

lazy_static! {
    static ref THAI_LSTM: LstmDataV1<'static> =
        serde_json::from_slice(THAI_MODEL).expect("JSON syntax error");
    static ref BURMESE_LSTM: LstmDataV1<'static> =
        serde_json::from_slice(BURMESE_MODEL).expect("JSON syntax error");
    static ref KHMER_LSTM: LstmDataV1<'static> =
        serde_json::from_slice(KHMER_MODEL).expect("JSON syntax error");
    static ref LAO_LSTM: LstmDataV1<'static> =
        serde_json::from_slice(LAO_MODEL).expect("JSON syntax error");
}

// LSTM model depends on language, So we have to switch models per language.
pub fn get_best_lstm_model(codepoint: u32) -> Option<DataPayload<LstmDataV1Marker>> {
    let lang = get_language(codepoint);
    match lang {
        Language::Thai => Some(DataPayload::from_owned(THAI_LSTM.clone())),
        Language::Burmese => Some(DataPayload::from_owned(BURMESE_LSTM.clone())),
        Language::Khmer => Some(DataPayload::from_owned(KHMER_LSTM.clone())),
        Language::Lao => Some(DataPayload::from_owned(LAO_LSTM.clone())),
        _ => None,
    }
}

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
            self.pos_utf8 += self.input.chars().nth(self.pos).unwrap().len_utf8();
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
            input: input.to_string(),
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
            .map(|r| r.unwrap())
            .collect();
        let lstm_output = lstm.word_segmenter(&input);
        Self {
            bies_str: lstm_output,
            pos: 0,
        }
    }
}

pub struct LstmSegmenter {
    lstm: Lstm,
}

impl LstmSegmenter {
    pub fn try_new(payload: DataPayload<LstmDataV1Marker>) -> Result<Self, DataError> {
        let lstm = Lstm::try_new(payload).unwrap();

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
    use icu_locid::locale;
    use icu_provider::{DataRequest, DataOptions, DataProvider};

    #[test]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";

        let provider = icu_testdata::get_provider();
        let payload = provider
            .load_resource(&DataRequest {
                options: DataOptions::from(locale!("th")),
                metadata: Default::default(),
            })
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let segmenter = LstmSegmenter::try_new(payload).unwrap();
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

        let payload = get_best_lstm_model(TEST_STR.chars().next().unwrap() as u32).unwrap();
        let segmenter = LstmSegmenter::try_new(payload).unwrap();
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

        let payload = get_best_lstm_model(TEST_STR.chars().next().unwrap() as u32).unwrap();
        let segmenter = LstmSegmenter::try_new(payload).unwrap();
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

        let payload = get_best_lstm_model(TEST_STR.chars().next().unwrap() as u32).unwrap();
        let segmenter = LstmSegmenter::try_new(payload).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: LSTM finds a break at '12' that the dictionary does not find
        assert_eq!(breaks, [12, 21, 30, 39], "Lao test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4, 7, 10, 13], "Lao utf-16 test");
    }
}
