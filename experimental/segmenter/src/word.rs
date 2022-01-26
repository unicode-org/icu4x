// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::CharIndices;

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

pub struct WordBreakType;

// UTF-8 version of word break iterator using rule based segmenter.
pub type WordBreakIterator<'a> = RuleBreakIterator<'a, WordBreakType>;

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
}

impl<'a> RuleBreakType<'a> for WordBreakType {
    type IterAttr = CharIndices<'a>;
    type CharType = char;

    fn get_break_property(iter: &RuleBreakIterator<Self>) -> u8 {
        get_break_property_utf8(iter.current_pos_data.unwrap().1, iter.property_table)
    }

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.current_pos_data.unwrap().1.len_utf8()
    }

    fn handle_complex_language(iter: &mut RuleBreakIterator<'a, Self>, left_codepoint: Self::CharType) -> Option<usize> {
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
            if Self::get_break_property(iter) != iter.complex_property {
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

// Latin-1 version of word break iterator using rule based segmenter.
pub type WordBreakIteratorLatin1<'a> = RuleBreakIterator<'a, WordBreakTypeLatin1>;

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
}

impl<'a> RuleBreakType<'a> for WordBreakTypeLatin1 {
    type IterAttr = Latin1Indices<'a>;
    type CharType = u8; // TODO: Latin1Char

    #[inline]
    fn get_break_property(iter: &RuleBreakIterator<Self>) -> u8 {
        get_break_property_latin1(iter.current_pos_data.unwrap().1, iter.property_table)
    }

    fn get_current_position_character_len(_: &RuleBreakIterator<Self>) -> usize {
        panic!("not reachable")
    }

    fn handle_complex_language(_: &mut RuleBreakIterator<'a, Self>, _: Self::CharType) -> Option<usize> {
        panic!("not reachable")
    }
}

pub struct WordBreakTypeUtf16;

// UTF-16 version of word break iterator using rule based segmenter.
pub type WordBreakIteratorUtf16<'a> = RuleBreakIterator<'a, WordBreakTypeUtf16>;

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
}

impl<'a> RuleBreakType<'a> for WordBreakTypeUtf16 {
    type IterAttr = Utf16Indices<'a>;
    type CharType = u32;

    #[inline]
    fn get_break_property(iter: &RuleBreakIterator<Self>) -> u8 {
        get_break_property_utf32(iter.current_pos_data.unwrap().1, iter.property_table)
    }

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        let ch = iter.current_pos_data.unwrap().1;
        if ch >= 0x10000 {
            2
        } else {
            1
        }
    }

    fn handle_complex_language(iter: &mut RuleBreakIterator<Self>, left_codepoint: Self::CharType) -> Option<usize> {
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
            if Self::get_break_property(iter) != iter.complex_property {
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
