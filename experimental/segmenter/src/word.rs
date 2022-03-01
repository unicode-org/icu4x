// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::CharIndices;
use icu_provider::prelude::*;

use crate::indices::{Latin1Indices, Utf16Indices};
use crate::lstm::get_line_break_utf16;
use crate::lstm::get_line_break_utf8;
use crate::provider::*;
use crate::rule_segmenter::*;

fn get_complex_language_break(input: &[u16]) -> Vec<usize> {
    if let Some(mut ret) = get_line_break_utf16(input) {
        ret.push(input.len());
        return ret;
    }
    [input.len()].to_vec()
}

fn get_complex_language_break_utf8(input: &str) -> Vec<usize> {
    if let Some(mut ret) = get_line_break_utf8(input) {
        ret.push(input.len());
        return ret;
    }
    [input.len()].to_vec()
}

/// Word break iterator for an `str` (a UTF-8 string).
pub type WordBreakIterator<'l, 's> = RuleBreakIterator<'l, 's, WordBreakType>;

/// Word break iterator for a Latin-1 (8-bit) string.
pub type WordBreakIteratorLatin1<'l, 's> = RuleBreakIterator<'l, 's, WordBreakTypeLatin1>;

/// Word break iterator for a UTF-16 string.
pub type WordBreakIteratorUtf16<'l, 's> = RuleBreakIterator<'l, 's, WordBreakTypeUtf16>;

/// Supports loading word break data, and creating word break iterators for different string
/// encodings. Please see the [module-level documentation](crate) for its usages.
pub struct WordBreakSegmenter {
    payload: DataPayload<WordBreakDataV1Marker>,
}

impl WordBreakSegmenter {
    pub fn try_new<D>(provider: &D) -> Result<Self, DataError>
    where
        D: ResourceProvider<WordBreakDataV1Marker> + ?Sized,
    {
        let payload = provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;
        Ok(Self { payload })
    }

    /// Create a word break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> WordBreakIterator<'l, 's> {
        WordBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
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
        }
    }
}

pub struct WordBreakType;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakType {
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
        let breaks = get_complex_language_break_utf8(&s);
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
    type CharType = u8; // TODO: Latin1Char

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
        let breaks = get_complex_language_break(&s);
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
