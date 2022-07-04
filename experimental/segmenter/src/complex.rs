// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use icu_provider::DataPayload;

use crate::language::*;
use crate::provider::*;
use crate::DictionarySegmenter;

// Use the LSTM when the feature is enabled.
#[cfg(feature = "lstm")]
use crate::lstm::{get_best_lstm_model, LstmSegmenter};

/// Return UTF-16 segment offset array using dictionary or lstm segmenter.
pub fn complex_language_segment_utf16(
    payload: &DataPayload<UCharDictionaryBreakDataV1Marker>,
    input: &[u16],
) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let lang_iter = LanguageIteratorUtf16::new(input);
    let mut offset = 0;
    for str_per_lang in lang_iter {
        #[cfg(feature = "lstm")]
        {
            if let Some(model) = get_best_lstm_model(str_per_lang[0] as u32) {
                if let Ok(segmenter) = LstmSegmenter::try_new(model) {
                    let breaks = segmenter.segment_utf16(&str_per_lang);
                    let mut r: Vec<usize> = breaks.map(|n| offset + n).collect();
                    result.append(&mut r);
                    offset += str_per_lang.len();
                    result.push(offset);
                    continue;
                }
            }
        }

        // TODO:
        // To support multiple dictionary, we have to pass multiple payloads per language.
        if let Ok(segmenter) = DictionarySegmenter::try_new(payload) {
            let breaks = segmenter.segment_utf16(&str_per_lang);
            let mut r: Vec<usize> = breaks.map(|n| offset + n).collect();
            result.append(&mut r);
            offset += str_per_lang.len();
            continue;
        }

        offset += str_per_lang.len();
        result.push(offset);
    }
    result
}

/// Return UTF-8 segment offset array using dictionary or lstm segmenter.
pub fn complex_language_segment_str(
    payload: &DataPayload<UCharDictionaryBreakDataV1Marker>,
    input: &str,
) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let lang_iter = LanguageIterator::new(input);
    let mut offset = 0;
    for str_per_lang in lang_iter {
        #[cfg(feature = "lstm")]
        {
            if let Some(model) = get_best_lstm_model(str_per_lang.chars().next().unwrap() as u32) {
                if let Ok(segmenter) = LstmSegmenter::try_new(model) {
                    let breaks = segmenter.segment_str(&str_per_lang);
                    let mut r: Vec<usize> = breaks.map(|n| offset + n).collect();
                    result.append(&mut r);
                    offset += str_per_lang.chars().fold(0, |n, c| n + c.len_utf8());
                    result.push(offset);
                    continue;
                }
            }
        }

        // TODO:
        // To support multiple dictionary, we have to pass multiple payloads per language.
        if let Ok(segmenter) = DictionarySegmenter::try_new(payload) {
            let breaks = segmenter.segment_str(&str_per_lang);
            let mut r: Vec<usize> = breaks.map(|n| offset + n).collect();
            result.append(&mut r);
            offset += str_per_lang.chars().fold(0, |n, c| n + c.len_utf8());
            continue;
        }

        offset += str_per_lang.chars().fold(0, |n, c| n + c.len_utf8());
        result.push(offset);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::Locale;
    use icu_provider::{DataRequest, ResourceOptions, ResourceProvider};

    #[test]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";
        let provider = icu_testdata::get_provider();
        let locale: Locale = ("th").parse().unwrap();
        let payload = provider
            .load_resource(&DataRequest {
                options: ResourceOptions::from(locale),
                metadata: Default::default(),
            })
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let breaks = complex_language_segment_str(&payload, TEST_STR);
        assert_eq!(breaks, [12, 21, 33, 42], "Thai test by UTF-8");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks = complex_language_segment_utf16(&payload, &utf16);
        assert_eq!(breaks, [4, 7, 11, 14], "Thai test by UTF-16");
    }
}
