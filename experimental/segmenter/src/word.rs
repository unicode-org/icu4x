// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::CharIndices;
use icu_locid::{locale, Locale};
use icu_provider::prelude::*;

use crate::complex::*;
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::provider::*;
use crate::rule_segmenter::*;

/// Word break iterator for an `str` (a UTF-8 string).
pub type WordBreakIteratorUtf8<'l, 's> = RuleBreakIterator<'l, 's, WordBreakTypeUtf8>;

/// Word break iterator for a Latin-1 (8-bit) string.
pub type WordBreakIteratorLatin1<'l, 's> = RuleBreakIterator<'l, 's, WordBreakTypeLatin1>;

/// Word break iterator for a UTF-16 string.
pub type WordBreakIteratorUtf16<'l, 's> = RuleBreakIterator<'l, 's, WordBreakTypeUtf16>;

/// Supports loading word break data, and creating word break iterators for different string
/// encodings. Please see the [module-level documentation](crate) for its usages.
pub struct WordBreakSegmenter {
    payload: DataPayload<WordBreakDataV1Marker>,
    dictionary: Dictionary,
    lstm: LstmPayloads,
}

impl WordBreakSegmenter {
    #[cfg(feature = "lstm")]
    pub fn try_new<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<WordBreakDataV1Marker>
            + DataProvider<UCharDictionaryBreakDataV1Marker>
            + DataProvider<LstmDataV1Marker>
            + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;

        let cj = Self::load_dictionary(provider, locale!("ja")).ok();

        let lstm = if cfg!(feature = "lstm") {
            let burmese = Self::load_lstm(provider, locale!("my")).ok();
            let khmer = Self::load_lstm(provider, locale!("lo")).ok();
            let lao = Self::load_lstm(provider, locale!("lo")).ok();
            let thai = Self::load_lstm(provider, locale!("th")).ok();
            LstmPayloads {
                burmese,
                khmer,
                lao,
                thai,
            }
        } else {
            LstmPayloads::default()
        };

        Ok(Self {
            payload,
            dictionary: Dictionary {
                burmese: None,
                khmer: None,
                lao: None,
                thai: None,
                cj,
            },
            lstm,
        })
    }

    #[cfg(not(feature = "lstm"))]
    pub fn try_new<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<WordBreakDataV1Marker>
            + DataProvider<UCharDictionaryBreakDataV1Marker>
            + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;

        let dictionary = if cfg!(feature = "lstm") {
            let cj = Self::load_dictionary(provider, locale!("ja")).ok();
            Dictionary {
                burmese: None,
                khmer: None,
                lao: None,
                thai: None,
                cj,
            }
        } else {
            let cj = Self::load_dictionary(provider, locale!("ja")).ok();
            let burmese = Self::load_dictionary(provider, locale!("my")).ok();
            let khmer = Self::load_dictionary(provider, locale!("km")).ok();
            let lao = Self::load_dictionary(provider, locale!("lo")).ok();
            let thai = Self::load_dictionary(provider, locale!("th")).ok();
            Dictionary {
                burmese,
                khmer,
                lao,
                thai,
                cj,
            }
        };

        Ok(Self {
            payload,
            dictionary,
            lstm: LstmPayloads::default(),
        })
    }

    fn load_dictionary<D: DataProvider<UCharDictionaryBreakDataV1Marker> + ?Sized>(
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

    #[cfg(feature = "lstm")]
    fn load_lstm<D: DataProvider<LstmDataV1Marker> + ?Sized>(
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

    /// Create a word break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> WordBreakIteratorUtf8<'l, 's> {
        WordBreakIteratorUtf8 {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
        }
    }

    /// Create a word break iterator for a Latin-1 (8-bit) string.
    pub fn segment_latin1<'l, 's>(&'l self, input: &'s [u8]) -> WordBreakIteratorLatin1<'l, 's> {
        WordBreakIteratorLatin1 {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
        }
    }

    /// Create a word break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(&'l self, input: &'s [u16]) -> WordBreakIteratorUtf16<'l, 's> {
        WordBreakIteratorUtf16 {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
        }
    }
}

pub struct WordBreakTypeUtf8;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakTypeUtf8 {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.current_pos_data.unwrap().1.len_utf8()
    }

    fn handle_complex_language(
        iter: &mut RuleBreakIterator<'l, 's, Self>,
        left_codepoint: Self::CharType,
    ) -> Option<usize> {
        // word segmenter doesn't define break rules for some languages such as Thai.
        let start_iter = iter.iter.clone();
        let start_point = iter.current_pos_data;
        let mut s = String::new();
        s.push(left_codepoint);
        loop {
            s.push(iter.current_pos_data.unwrap().1);
            iter.current_pos_data = iter.iter.next();
            if iter.current_pos_data.is_none() {
                break;
            }
            if iter.get_current_break_property() != iter.data.complex_property {
                break;
            }
        }

        // Restore iterator to move to head of complex string
        iter.iter = start_iter;
        iter.current_pos_data = start_point;
        let breaks = complex_language_segment_str(iter.dictionary, iter.lstm, &s);
        iter.result_cache = breaks;
        let mut i = iter.current_pos_data.unwrap().1.len_utf8();
        loop {
            if i == *iter.result_cache.first().unwrap() {
                // Re-calculate breaking offset
                iter.result_cache = iter.result_cache.iter().skip(1).map(|r| r - i).collect();
                return Some(iter.current_pos_data.unwrap().0);
            }
            iter.current_pos_data = iter.iter.next();
            if iter.current_pos_data.is_none() {
                iter.result_cache.clear();
                return Some(iter.len);
            }
            i += Self::get_current_position_character_len(iter);
        }
    }
}

pub struct WordBreakTypeLatin1;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakTypeLatin1 {
    type IterAttr = Latin1Indices<'s>;
    type CharType = u8;

    fn get_current_position_character_len(_: &RuleBreakIterator<Self>) -> usize {
        panic!("not reachable")
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<'l, 's, Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        panic!("not reachable")
    }
}

pub struct WordBreakTypeUtf16;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakTypeUtf16 {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        let ch = iter.current_pos_data.unwrap().1;
        if ch >= 0x10000 {
            2
        } else {
            1
        }
    }

    fn handle_complex_language(
        iter: &mut RuleBreakIterator<Self>,
        left_codepoint: Self::CharType,
    ) -> Option<usize> {
        // word segmenter doesn't define break rules for some languages such as Thai.
        let start_iter = iter.iter.clone();
        let start_point = iter.current_pos_data;
        let mut s = vec![left_codepoint as u16];
        loop {
            s.push(iter.current_pos_data.unwrap().1 as u16);
            iter.current_pos_data = iter.iter.next();
            if iter.current_pos_data.is_none() {
                break;
            }
            if iter.get_current_break_property() != iter.data.complex_property {
                break;
            }
        }

        // Restore iterator to move to head of complex string
        iter.iter = start_iter;
        iter.current_pos_data = start_point;
        let breaks = complex_language_segment_utf16(iter.dictionary, iter.lstm, &s);
        let mut i = 1;
        iter.result_cache = breaks;
        // result_cache vector is utf-16 index that is in BMP.
        loop {
            if i == *iter.result_cache.first().unwrap() {
                // Re-calculate breaking offset
                iter.result_cache = iter.result_cache.iter().skip(1).map(|r| r - i).collect();
                return Some(iter.current_pos_data.unwrap().0);
            }
            iter.current_pos_data = iter.iter.next();
            if iter.current_pos_data.is_none() {
                iter.result_cache.clear();
                return Some(iter.len);
            }
            i += 1;
        }
    }
}
