// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::CharIndices;

use crate::break_iterator_impl;
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::lstm::get_line_break_utf16;
use crate::lstm::get_line_break_utf8;
use crate::rule_segmenter::*;

include!(concat!(env!("OUT_DIR"), "/generated_word_table.rs"));

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

// UTF-8 version of word break iterator using rule based segmenter.
break_iterator_impl!(WordBreakIterator, CharIndices<'a>, char);

impl<'a> WordBreakIterator<'a> {
    /// Create word break iterator
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

    fn handle_complex_language(&mut self, left_codepoint: char) -> Option<usize> {
        // word segmenter doesn't define break rules for some languages such as Thai.
        let start_iter = self.iter.clone();
        let start_point = self.current_pos_data;
        let mut s = String::new();
        s.push(left_codepoint);
        loop {
            s.push(self.current_pos_data.unwrap().1);
            self.current_pos_data = self.iter.next();
            if self.current_pos_data.is_none() {
                break;
            }
            if self.get_break_property() != self.complex_property {
                break;
            }
        }

        // Restore iterator to move to head of complex string
        self.iter = start_iter;
        self.current_pos_data = start_point;
        let breaks = get_complex_language_break_utf8(&s);
        self.result_cache = breaks;
        let mut i = self.current_pos_data.unwrap().1.len_utf8();
        loop {
            if i == *self.result_cache.first().unwrap() {
                // Re-calculate breaking offset
                self.result_cache = self.result_cache.iter().skip(1).map(|r| r - i).collect();
                return Some(self.current_pos_data.unwrap().0);
            }
            self.current_pos_data = self.iter.next();
            if self.current_pos_data.is_none() {
                self.result_cache.clear();
                return Some(self.len);
            }
            i += self.get_current_position_character_len();
        }
    }
}

// Latin-1 version of word break iterator using rule based segmenter.
break_iterator_impl!(WordBreakIteratorLatin1, Latin1Indices<'a>, u8);

impl<'a> WordBreakIteratorLatin1<'a> {
    /// Create word break iterator using Latin-1/8-bit string.
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

// UTF-16 version of word break iterator using rule based segmenter.
break_iterator_impl!(WordBreakIteratorUtf16, Utf16Indices<'a>, u32);

impl<'a> WordBreakIteratorUtf16<'a> {
    /// Create word break iterator using UTF-16 string.
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

    fn handle_complex_language(&mut self, left_codepoint: u32) -> Option<usize> {
        // word segmenter doesn't define break rules for some languages such as Thai.
        let start_iter = self.iter.clone();
        let start_point = self.current_pos_data;
        let mut s = vec![left_codepoint as u16];
        loop {
            s.push(self.current_pos_data.unwrap().1 as u16);
            self.current_pos_data = self.iter.next();
            if self.current_pos_data.is_none() {
                break;
            }
            if self.get_break_property() != self.complex_property {
                break;
            }
        }

        // Restore iterator to move to head of complex string
        self.iter = start_iter;
        self.current_pos_data = start_point;
        let breaks = get_complex_language_break(&s);
        let mut i = 1;
        self.result_cache = breaks;
        // result_cache vector is utf-16 index that is in BMP.
        loop {
            if i == *self.result_cache.first().unwrap() {
                // Re-calculate breaking offset
                self.result_cache = self.result_cache.iter().skip(1).map(|r| r - i).collect();
                return Some(self.current_pos_data.unwrap().0);
            }
            self.current_pos_data = self.iter.next();
            if self.current_pos_data.is_none() {
                self.result_cache.clear();
                return Some(self.len);
            }
            i += 1;
        }
    }
}
