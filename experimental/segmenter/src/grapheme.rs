// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use core::str::CharIndices;

use crate::indices::{Latin1Indices, Utf16Indices};
use crate::rule_segmenter::*;

include!(concat!(env!("OUT_DIR"), "/generated_grapheme_table.rs"));

pub struct GraphemeClusterBreakType;

// UTF-8 version of grapheme break iterator using rule based segmenter.
pub type GraphemeClusterBreakIterator<'a> = RuleBreakIterator<'a, GraphemeClusterBreakType>;

impl<'a> GraphemeClusterBreakIterator<'a> {
    /// Create grapheme break iterator
    pub fn new(input: &'a str) -> Self {
        Self {
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
}

impl<'a> RuleBreakType<'a> for GraphemeClusterBreakType {
    type IterAttr = CharIndices<'a>;
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

pub struct GraphemeClusterBreakTypeLatin1;

// Latin-1 version of grapheme break iterator using rule based segmenter.
pub type GraphemeClusterBreakIteratorLatin1<'a> =
    RuleBreakIterator<'a, GraphemeClusterBreakTypeLatin1>;

impl<'a> GraphemeClusterBreakIteratorLatin1<'a> {
    /// Create grapheme break iterator using Latin-1/8-bit string.
    pub fn new(input: &'a [u8]) -> Self {
        Self {
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
}

impl<'a> RuleBreakType<'a> for GraphemeClusterBreakTypeLatin1 {
    type IterAttr = Latin1Indices<'a>;
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

pub struct GraphemeClusterBreakTypeUtf16;

// UTF-16 version of grapheme break iterator using rule based segmenter.
pub type GraphemeClusterBreakIteratorUtf16<'a> =
    RuleBreakIterator<'a, GraphemeClusterBreakTypeUtf16>;

impl<'a> GraphemeClusterBreakIteratorUtf16<'a> {
    /// Create grapheme break iterator using UTF-16 string.
    pub fn new(input: &'a [u16]) -> Self {
        Self {
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

impl<'a> RuleBreakType<'a> for GraphemeClusterBreakTypeUtf16 {
    type IterAttr = Utf16Indices<'a>;
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
