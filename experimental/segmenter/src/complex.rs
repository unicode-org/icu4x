// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dictionary::DictionarySegmenter;
use crate::language::*;
use crate::provider::*;
use alloc::vec::Vec;
use icu_locid::{locale, Locale};
use icu_provider::prelude::*;

// Use the LSTM when the feature is enabled.
#[cfg(feature = "lstm")]
use crate::lstm::LstmSegmenter;

#[derive(Default)]
pub struct LstmPayloads {
    pub burmese: Option<DataPayload<LstmDataV1Marker>>,
    pub khmer: Option<DataPayload<LstmDataV1Marker>>,
    pub lao: Option<DataPayload<LstmDataV1Marker>>,
    pub thai: Option<DataPayload<LstmDataV1Marker>>,
}

impl LstmPayloads {
    #[cfg(feature = "lstm")]
    pub fn best(&self, codepoint: u32) -> Option<&DataPayload<LstmDataV1Marker>> {
        let lang = get_language(codepoint);
        match lang {
            Language::Burmese => self.burmese.as_ref(),
            Language::Khmer => self.khmer.as_ref(),
            Language::Lao => self.lao.as_ref(),
            Language::Thai => self.thai.as_ref(),
            _ => None,
        }
    }

    /// Construct a [`LstmPayloads`] for all supported languages.
    pub(crate) fn new<D: DataProvider<LstmDataV1Marker> + ?Sized>(provider: &D) -> Self {
        let burmese = Self::load(provider, locale!("my")).ok();
        let khmer = Self::load(provider, locale!("lo")).ok();
        let lao = Self::load(provider, locale!("lo")).ok();
        let thai = Self::load(provider, locale!("th")).ok();
        LstmPayloads {
            burmese,
            khmer,
            lao,
            thai,
        }
    }

    pub(crate) fn load<D: DataProvider<LstmDataV1Marker> + ?Sized>(
        provider: &D,
        locale: Locale,
    ) -> Result<DataPayload<LstmDataV1Marker>, DataError> {
        provider
            .load(DataRequest {
                locale: &DataLocale::from(locale),
                metadata: Default::default(),
            })?
            .take_payload()
    }
}

#[derive(Default)]
pub struct Dictionary {
    pub burmese: Option<DataPayload<UCharDictionaryBreakDataV1Marker>>,
    pub khmer: Option<DataPayload<UCharDictionaryBreakDataV1Marker>>,
    pub lao: Option<DataPayload<UCharDictionaryBreakDataV1Marker>>,
    pub thai: Option<DataPayload<UCharDictionaryBreakDataV1Marker>>,
    pub cj: Option<DataPayload<UCharDictionaryBreakDataV1Marker>>,
}

impl Dictionary {
    fn best(&self, input: u32) -> Option<&DataPayload<UCharDictionaryBreakDataV1Marker>> {
        match get_language(input) {
            Language::Burmese => self.burmese.as_ref(),
            Language::Khmer => self.khmer.as_ref(),
            Language::Lao => self.lao.as_ref(),
            Language::Thai => self.thai.as_ref(),
            Language::ChineseOrJapanese => self.cj.as_ref(),
            _ => None,
        }
    }

    /// Construct a [`Dictionary`] for all supported languages.
    pub(crate) fn new<D: DataProvider<UCharDictionaryBreakDataV1Marker> + ?Sized>(
        provider: &D,
    ) -> Self {
        let burmese = Self::load(provider, locale!("my")).ok();
        let khmer = Self::load(provider, locale!("km")).ok();
        let lao = Self::load(provider, locale!("lo")).ok();
        let thai = Self::load(provider, locale!("th")).ok();
        let cj = Self::load(provider, locale!("ja")).ok();
        Dictionary {
            burmese,
            khmer,
            lao,
            thai,
            cj,
        }
    }

    /// Construct a [`Dictionary`] for Chinese and Japanese.
    pub(crate) fn new_chinese_japanese<
        D: DataProvider<UCharDictionaryBreakDataV1Marker> + ?Sized,
    >(
        provider: &D,
    ) -> Self {
        let cj = Self::load(provider, locale!("ja")).ok();
        Dictionary {
            cj,
            ..Default::default()
        }
    }

    /// Construct a [`Dictionary`] for Southeast Asian languages (Burmese, Khmer, Lao, and Thai).
    pub(crate) fn new_southeast_asian<
        D: DataProvider<UCharDictionaryBreakDataV1Marker> + ?Sized,
    >(
        provider: &D,
    ) -> Self {
        let burmese = Self::load(provider, locale!("my")).ok();
        let khmer = Self::load(provider, locale!("km")).ok();
        let lao = Self::load(provider, locale!("lo")).ok();
        let thai = Self::load(provider, locale!("th")).ok();
        Dictionary {
            burmese,
            khmer,
            lao,
            thai,
            ..Default::default()
        }
    }

    pub(crate) fn load<D: DataProvider<UCharDictionaryBreakDataV1Marker> + ?Sized>(
        provider: &D,
        locale: Locale,
    ) -> Result<DataPayload<UCharDictionaryBreakDataV1Marker>, DataError> {
        provider
            .load(DataRequest {
                locale: &DataLocale::from(locale),
                metadata: Default::default(),
            })?
            .take_payload()
    }
}

/// Return UTF-16 segment offset array using dictionary or lstm segmenter.
#[allow(unused_variables)]
pub fn complex_language_segment_utf16(
    dictionary: Option<&Dictionary>,
    lstm: Option<&LstmPayloads>,
    grapheme: Option<&RuleBreakDataV1>,
    input: &[u16],
) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let lang_iter = LanguageIteratorUtf16::new(input);
    let mut offset = 0;
    for str_per_lang in lang_iter {
        if let Some(first_ch) = str_per_lang.get(0) {
            #[cfg(feature = "lstm")]
            {
                if let Some(lstm) = lstm {
                    if let Some(model) = lstm.best(*first_ch as u32) {
                        if let Ok(segmenter) = LstmSegmenter::try_new_unstable(model, grapheme) {
                            let breaks = segmenter.segment_utf16(&str_per_lang);
                            result.extend(breaks.map(|n| offset + n));
                            offset += str_per_lang.len();
                            result.push(offset);
                            continue;
                        }
                    }
                }
            }

            if let Some(dictionary) = dictionary {
                if let Some(grapheme) = grapheme {
                    if let Some(payload) = dictionary.best(*first_ch as u32) {
                        if let Ok(segmenter) =
                            DictionarySegmenter::try_new_unstable(payload, grapheme)
                        {
                            let breaks = segmenter.segment_utf16(&str_per_lang);
                            result.extend(breaks.map(|n| offset + n));
                            offset += str_per_lang.len();
                            continue;
                        }
                    }
                }
            }

            offset += str_per_lang.len();
            result.push(offset);
        }
    }
    result
}

/// Return UTF-8 segment offset array using dictionary or lstm segmenter.
#[allow(unused_variables)]
pub fn complex_language_segment_str(
    dictionary: Option<&Dictionary>,
    lstm: Option<&LstmPayloads>,
    grapheme: Option<&RuleBreakDataV1>,
    input: &str,
) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let lang_iter = LanguageIterator::new(input);
    let mut offset = 0;
    for str_per_lang in lang_iter {
        if let Some(first_ch) = str_per_lang.chars().next() {
            #[cfg(feature = "lstm")]
            {
                if let Some(lstm) = lstm {
                    if let Some(model) = lstm.best(first_ch as u32) {
                        if let Ok(segmenter) = LstmSegmenter::try_new_unstable(model, grapheme) {
                            let breaks = segmenter.segment_str(&str_per_lang);
                            result.extend(breaks.map(|n| offset + n));
                            offset += str_per_lang.chars().map(|c| c.len_utf8()).sum::<usize>();
                            result.push(offset);
                            continue;
                        }
                    }
                }
            }

            if let Some(dictionary) = dictionary {
                if let Some(grapheme) = grapheme {
                    if let Some(payload) = dictionary.best(first_ch as u32) {
                        if let Ok(segmenter) =
                            DictionarySegmenter::try_new_unstable(payload, grapheme)
                        {
                            let breaks = segmenter.segment_str(&str_per_lang);
                            result.extend(breaks.map(|n| offset + n));
                            offset += str_per_lang.chars().map(|c| c.len_utf8()).sum::<usize>();
                            continue;
                        }
                    }
                }
            }
            offset += str_per_lang.chars().fold(0, |n, c| n + c.len_utf8());
            result.push(offset);
        }
    }
    result
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;
    use icu_locid::locale;

    #[test]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";
        let data_locale = locale!("th").into();
        let payload = icu_testdata::buffer()
            .as_deserializing()
            .load(DataRequest {
                locale: &data_locale,
                metadata: Default::default(),
            })
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let dictionary = Dictionary {
            burmese: None,
            khmer: None,
            lao: None,
            thai: Some(payload),
            cj: None,
        };
        let payload = icu_testdata::buffer()
            .as_deserializing()
            .load(DataRequest {
                locale: &data_locale,
                metadata: Default::default(),
            })
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let lstm = LstmPayloads {
            burmese: None,
            khmer: None,
            lao: None,
            thai: Some(payload),
        };
        let grapheme: DataPayload<GraphemeClusterBreakDataV1Marker> = icu_testdata::buffer()
            .as_deserializing()
            .load(Default::default())
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let breaks = complex_language_segment_str(
            Some(&dictionary),
            Some(&lstm),
            Some(grapheme.get()),
            TEST_STR,
        );
        assert_eq!(breaks, [12, 21, 33, 42], "Thai test by UTF-8");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks = complex_language_segment_utf16(
            Some(&dictionary),
            Some(&lstm),
            Some(grapheme.get()),
            &utf16,
        );
        assert_eq!(breaks, [4, 7, 11, 14], "Thai test by UTF-16");
    }
}
