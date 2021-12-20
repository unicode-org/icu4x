// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use core::str::CharIndices;

use crate::break_iterator_impl;
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::rule_segmenter::*;

include!(concat!(env!("OUT_DIR"), "/generated_sentence_table.rs"));

// UTF-8 version of sentence break iterator using rule based segmenter.
break_iterator_impl!(SentenceBreakIterator, CharIndices<'a>, char);

impl<'a> SentenceBreakIterator<'a> {
    /// Create sentence break iterator
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

    fn get_break_property(&mut self) -> u8 {
        get_break_property_utf8(self.current_pos_data.unwrap().1, self.property_table)
    }

    fn get_current_position_character_len(&self) -> usize {
        self.current_pos_data.unwrap().1.len_utf8()
    }

    fn handle_complex_language(&mut self, _: char) -> Option<usize> {
        panic!("not reachable")
    }
}

// Latin-1 version of sentence break iterator using rule based segmenter.
break_iterator_impl!(SentenceBreakIteratorLatin1, Latin1Indices<'a>, u8);

impl<'a> SentenceBreakIteratorLatin1<'a> {
    /// Create sentence break iterator using Latin-1/8-bit string.
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

    #[inline]
    fn get_break_property(&mut self) -> u8 {
        get_break_property_latin1(self.current_pos_data.unwrap().1, self.property_table)
    }

    fn get_current_position_character_len(&self) -> usize {
        panic!("not reachable")
    }

    fn handle_complex_language(&mut self, _: u8) -> Option<usize> {
        panic!("not reachable")
    }
}

// UTF-16 version of sentence break iterator using rule based segmenter.
break_iterator_impl!(SentenceBreakIteratorUtf16, Utf16Indices<'a>, u32);

impl<'a> SentenceBreakIteratorUtf16<'a> {
    /// Create sentence break iterator using UTF-16 string.
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

    #[inline]
    fn get_break_property(&mut self) -> u8 {
        get_break_property_utf32(self.current_pos_data.unwrap().1, self.property_table)
    }

    fn get_current_position_character_len(&self) -> usize {
        let ch = self.current_pos_data.unwrap().1;
        if ch >= 0x10000 {
            2
        } else {
            1
        }
    }

    fn handle_complex_language(&mut self, _: u32) -> Option<usize> {
        panic!("not reachable")
    }
}
