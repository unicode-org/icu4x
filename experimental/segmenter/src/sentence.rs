// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use core::str::CharIndices;
use icu_provider::DataError;

use crate::indices::{Latin1Indices, Utf16Indices};
use crate::rule_segmenter::*;

include!(concat!(env!("OUT_DIR"), "/generated_sentence_table.rs"));

/// Sentence break iterator for an `str` (a UTF-8 string).
pub type SentenceBreakIterator<'l, 's> = RuleBreakIterator<'l, 's, SentenceBreakType>;

/// Sentence break iterator for a Latin-1 (8-bit) string.
pub type SentenceBreakIteratorLatin1<'l, 's> = RuleBreakIterator<'l, 's, SentenceBreakTypeLatin1>;

/// Sentence break iterator for a UTF-16 string.
pub type SentenceBreakIteratorUtf16<'l, 's> = RuleBreakIterator<'l, 's, SentenceBreakTypeUtf16>;

/// Supports loading sentence break data, and creating sentence break iterators for different string
/// encodings. Please see the [module-level documentation] for its usages.
///
/// [module-level documentation]: index.html
pub struct SentenceBreakSegmenter;

impl SentenceBreakSegmenter {
    pub fn try_new() -> Result<Self, DataError> {
        // Note: This will be able to return an Error once DataProvider is added
        Ok(Self)
    }

    /// Create a sentence break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> SentenceBreakIterator<'l, 's> {
        SentenceBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            break_state_table: &BREAK_STATE_MACHINE_TABLE,
            property_table: &PROPERTY_TABLE,
            rule_property_count: PROPERTY_COUNT,
            last_codepoint_property: LAST_CODEPOINT_PROPERTY,
            sot_property: PROP_SOT as u8,
            eot_property: PROP_EOT as u8,
            complex_property: PROP_COMPLEX as u8,
        }
    }

    /// Create a sentence break iterator for a Latin-1 (8-bit) string.
    pub fn segment_latin1<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> SentenceBreakIteratorLatin1<'l, 's> {
        SentenceBreakIteratorLatin1 {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            break_state_table: &BREAK_STATE_MACHINE_TABLE,
            property_table: &PROPERTY_TABLE,
            rule_property_count: PROPERTY_COUNT,
            last_codepoint_property: LAST_CODEPOINT_PROPERTY,
            sot_property: PROP_SOT as u8,
            eot_property: PROP_EOT as u8,
            complex_property: PROP_COMPLEX as u8,
        }
    }

    /// Create a sentence break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(&'l self, input: &'s [u16]) -> SentenceBreakIteratorUtf16<'l, 's> {
        SentenceBreakIteratorUtf16 {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            break_state_table: &BREAK_STATE_MACHINE_TABLE,
            property_table: &PROPERTY_TABLE,
            rule_property_count: PROPERTY_COUNT,
            last_codepoint_property: LAST_CODEPOINT_PROPERTY,
            sot_property: PROP_SOT as u8,
            eot_property: PROP_EOT as u8,
            complex_property: PROP_COMPLEX as u8,
        }
    }
}

pub struct SentenceBreakType;

impl<'l, 's> RuleBreakType<'l, 's> for SentenceBreakType {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.current_pos_data.unwrap().1.len_utf8()
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        panic!("not reachable")
    }
}

pub struct SentenceBreakTypeLatin1;

impl<'l, 's> RuleBreakType<'l, 's> for SentenceBreakTypeLatin1 {
    type IterAttr = Latin1Indices<'s>;
    type CharType = u8; // TODO: Latin1Char

    fn get_current_position_character_len(_: &RuleBreakIterator<Self>) -> usize {
        panic!("not reachable")
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        panic!("not reachable")
    }
}

pub struct SentenceBreakTypeUtf16;

impl<'l, 's> RuleBreakType<'l, 's> for SentenceBreakTypeUtf16 {
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
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        panic!("not reachable")
    }
}
