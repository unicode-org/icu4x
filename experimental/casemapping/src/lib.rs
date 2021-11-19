// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO: module documentation

use core::convert::TryFrom;
use core::num::TryFromIntError;
use icu_codepointtrie::codepointtrie::{CodePointTrie, TrieValue};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use yoke::{Yokeable, ZeroCopyFrom};
use zerovec::ule::{AsULE, PlainOldULE};
use zerovec::{ZeroVec, ZeroMap};

pub mod provider;

/// The case of a Unicode character
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CaseType {
    /// Not a cased letter
    None = 0,
    /// Lowercase letter
    Lower = 1,
    /// Uppercase letter
    Upper = 2,
    /// Titlecase letter
    Title = 3,
}

impl CaseType {
    const CASE_MASK: u16 = 0x3;

    // The casetype is stored in the codepoint trie as two bits.
    // After masking them to get a value between 0 and 3, this function converts
    // to CaseType.
    #[inline]
    fn from_masked_bits(b: u16) -> Self {
        debug_assert!(b & Self::CASE_MASK == b);
        match b {
            0 => CaseType::None,
            1 => CaseType::Lower,
            2 => CaseType::Upper,
            _ => CaseType::Title,
        }
    }
}

/// The dot type of a Unicode character.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DotType {
    /// Normal characters with combining class 0
    NoDot = 0,
    /// Soft-dotted characters with combining class 0
    SoftDotted = 1,
    /// "Above" accents with combining class 230
    Above = 2,
    /// Other accent characters
    OtherAccent = 3,
}

impl DotType {
    const DOT_MASK: u16 = 0x3;

    // The dot type is stored in either the codepoint trie or the exception table as two bits.
    // After shifting and masking them to get a value between 0 and 3, this function converts
    // to DotType.
    #[inline]
    fn from_masked_bits(b: u16) -> Self {
        debug_assert!(b & Self::DOT_MASK == b);
        match b {
            0 => DotType::NoDot,
            1 => DotType::SoftDotted,
            2 => DotType::Above,
            _ => DotType::OtherAccent,
        }
    }
}

/// The datatype stored in the codepoint trie for casemapping.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct CaseMappingData(u16);

impl CaseMappingData {
    const IGNORABLE_FLAG: u16 = 0x4;
    const EXCEPTION_FLAG: u16 = 0x8;
    const SENSITIVE_FLAG: u16 = 0x10;

    const DOT_SHIFT: u16 = 5;

    const DELTA_SHIFT: usize = 7;
    const EXCEPTION_SHIFT: usize = 4;

    #[inline]
    fn case_type(&self) -> CaseType {
        let masked_bits = self.0 & CaseType::CASE_MASK;
        CaseType::from_masked_bits(masked_bits)
    }

    #[inline]
    fn is_upper_or_title(&self) -> bool {
        match self.case_type() {
            CaseType::None | CaseType::Lower => false,
            CaseType::Upper | CaseType::Title => true,
        }
    }

    #[inline]
    fn is_ignorable(&self) -> bool {
        self.0 & Self::IGNORABLE_FLAG != 0
    }

    #[inline]
    fn has_exception(&self) -> bool {
        self.0 & Self::EXCEPTION_FLAG != 0
    }

    #[inline]
    fn is_sensitive(&self) -> bool {
        self.0 & Self::SENSITIVE_FLAG != 0
    }

    #[inline]
    fn dot_type(&self) -> DotType {
        let masked_bits = (self.0 >> Self::DOT_SHIFT) & DotType::DOT_MASK;
        DotType::from_masked_bits(masked_bits)
    }

    #[inline]
    fn delta(&self) -> i16 {
        debug_assert!(!self.has_exception());
        (self.0 as i16) >> Self::DELTA_SHIFT
    }

    #[inline]
    fn exception_index(&self) -> u16 {
        debug_assert!(self.has_exception());
        self.0 >> Self::EXCEPTION_SHIFT
    }
}

impl AsULE for CaseMappingData {
    type ULE = PlainOldULE<2>;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        PlainOldULE(self.0.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        CaseMappingData(u16::from_le_bytes(unaligned.0))
    }
}

impl TrieValue for CaseMappingData {
    const DATA_GET_ERROR_VALUE: CaseMappingData = CaseMappingData(0);
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u16::try_from(i).map(CaseMappingData)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
struct CaseMappingExceptions<'data> {
    raw: ZeroVec<'data, u16>,
}

#[derive(Copy, Clone)]
enum CaseMappingExceptionSlot {
    Lower = 0,
    Fold = 1,
    Upper = 2,
    Title = 3,
    Delta = 4,
    // Slot 5 is reserved
    Closure = 6,
    FullMappings = 7,
}

/// Case mapping exceptions that can't be represented as a simple offset added to the original character.
/// Following ICU4C, data is stored as a u16 array. The codepoint trie in CaseMapping stores offsets into
/// this array. The u16 at that index contains a set of flags describing the subsequent data.
/// TODO: diagram of layout
impl<'data> CaseMappingExceptions<'data> {
    // Each slot is 2 u16 elements instead of 1
    const DOUBLE_SLOTS_FLAG: u16 = 0x100;

    const NO_SIMPLE_CASE_FOLDING_FLAG: u16 = 0x200;
    const DELTA_IS_NEGATIVE_FLAG: u16 = 0x400;
    const SENSITIVE_FLAG: u16 = 0x800;

    const DOT_SHIFT: u16 = 12;

    const CONDITIONAL_SPECIAL_FLAG: u16 = 0x4000;
    const CONDITIONAL_FOLD_FLAG: u16 = 0x8000;

    const FULL_MAPPINGS_ALL_LENGTHS_MASK: u32 = 0xffff;
    const FULL_MAPPINGS_LENGTH_MASK: u32 = 0xf;
    const FULL_MAPPINGS_LENGTH_SHIFT: u32 = 4;

    const CLOSURE_MAX_LENGTH: u32 = 0xf;

    #[inline(always)]
    fn get(&self, idx: usize) -> u16 {
        <u16 as AsULE>::from_unaligned(self.raw.as_slice()[idx as usize])
    }

    // Returns true if the given slot exists for the exception data starting at the given index
    #[inline]
    fn has_slot(&self, idx: u16, slot: CaseMappingExceptionSlot) -> bool {
        let bit = 1u16 << (slot as u16);
        self.get(idx as usize) & bit != 0
    }

    // Returns the index of a given slot for the exception data starting at a given index.
    fn slot_index(&self, idx: u16, slot: CaseMappingExceptionSlot) -> usize {
        debug_assert!(self.has_slot(idx, slot));
        let flags = self.get(idx as usize);

        let slot_bit = 1u16 << (slot as u16);
        let previous_slot_mask = slot_bit - 1;
        let previous_slots = flags & previous_slot_mask;
        let slot_num = previous_slots.count_ones() as usize;

        let offset = if flags & Self::DOUBLE_SLOTS_FLAG != 0 {
            slot_num * 2
        } else {
            slot_num
        };

        idx as usize + 1 + offset
    }

    fn slot_value(&self, idx: u16, slot: CaseMappingExceptionSlot) -> u32 {
        debug_assert!(self.has_slot(idx, slot));
        let slot_idx = self.slot_index(idx, slot);
        if self.get(idx as usize) & Self::DOUBLE_SLOTS_FLAG != 0 {
            let hi = self.get(slot_idx) as u32;
            let lo = self.get(slot_idx + 1) as u32;
            hi << 16 | lo
        } else {
            self.get(slot_idx) as u32
        }
    }

    fn slot_char(&self, idx: u16, slot: CaseMappingExceptionSlot) -> char {
        let raw = self.slot_value(idx, slot);
        char::from_u32(raw).expect("Character should be valid")
    }

    fn delta(&self, idx: u16) -> i32 {
        debug_assert!(self.has_slot(idx, CaseMappingExceptionSlot::Delta));
        let raw: i32 = self.slot_value(idx, CaseMappingExceptionSlot::Delta) as _;
        if self.get(idx as usize) & Self::DELTA_IS_NEGATIVE_FLAG != 0 {
            -raw
        } else {
            raw
        }
    }

    fn no_simple_case_folding(&self, idx: u16) -> bool {
        self.get(idx as usize) & Self::NO_SIMPLE_CASE_FOLDING_FLAG != 0
    }

    fn is_sensitive(&self, idx: u16) -> bool {
        self.get(idx as usize) & Self::SENSITIVE_FLAG != 0
    }

    fn has_conditional_fold(&self, idx: u16) -> bool {
        self.get(idx as usize) & Self::CONDITIONAL_FOLD_FLAG != 0
    }

    fn has_conditional_special(&self, idx: u16) -> bool {
        self.get(idx as usize) & Self::CONDITIONAL_SPECIAL_FLAG != 0
    }

    fn dot_type(&self, idx: u16) -> DotType {
        let masked_bits = (self.get(idx as usize) >> Self::DOT_SHIFT) & DotType::DOT_MASK;
        DotType::from_masked_bits(masked_bits)
    }

    fn decode_string(&self, idx: usize, len: usize) -> String {
        let iter = self.raw.as_slice()[idx..idx + len]
            .iter()
            .map(|&ule| <u16 as AsULE>::from_unaligned(ule));
        char::decode_utf16(iter)
            .map(|c| c.expect("No unpaired surrogates"))
            .collect::<String>()
    }

    fn add_full_and_closure_mappings<S: SetAdder>(&self, idx: u16, set: &mut S) {
        // The full mapping strings and the closure string are stored following
        // the final slot.
        let mut data_idx: usize = if self.has_slot(idx, CaseMappingExceptionSlot::FullMappings) {
            self.slot_index(idx, CaseMappingExceptionSlot::FullMappings) + 1
        } else if self.has_slot(idx, CaseMappingExceptionSlot::Closure) {
            self.slot_index(idx, CaseMappingExceptionSlot::Closure) + 1
        } else {
            // No closure or full mappings.
            return;
        };

        if self.has_slot(idx, CaseMappingExceptionSlot::FullMappings) {
            // Full mapping strings are stored as UTF16.
            // The length of each full mapping string is stored in a nibble of the slot.
            let value = self.slot_value(idx, CaseMappingExceptionSlot::FullMappings);
            let mut all_lengths: u32 = value & Self::FULL_MAPPINGS_ALL_LENGTHS_MASK;

            // Skip lowercase result string.
            let lowercase_len = all_lengths & Self::FULL_MAPPINGS_LENGTH_MASK;
            data_idx += lowercase_len as usize;
            all_lengths >>= Self::FULL_MAPPINGS_LENGTH_SHIFT;

            // Add the full case mapping string.
            let mapping_len = all_lengths & Self::FULL_MAPPINGS_LENGTH_MASK;
            if mapping_len != 0 {
                let mapping_string = self.decode_string(data_idx as usize, mapping_len as usize);
                set.add_string(&mapping_string);
                data_idx += mapping_len as usize;
            }

            // Skip uppercase and titlecase result strings.
            let uppercase_len = all_lengths & Self::FULL_MAPPINGS_LENGTH_MASK;
            data_idx += uppercase_len as usize;
            all_lengths >>= Self::FULL_MAPPINGS_LENGTH_SHIFT;
            let titlecase_len = all_lengths & Self::FULL_MAPPINGS_LENGTH_MASK;
            data_idx += titlecase_len as usize;
        };

        if self.has_slot(idx, CaseMappingExceptionSlot::Closure) {
            // The length of the closure is stored in the lower bits of the slot. Upper bits are reserved.
            let value = self.slot_value(idx, CaseMappingExceptionSlot::Closure);
            let closure_len = (value & Self::CLOSURE_MAX_LENGTH) as usize;

            let closure_iter = self.raw.as_slice()[data_idx..]
                .iter()
                .map(|&ule| <u16 as AsULE>::from_unaligned(ule));
            for c in char::decode_utf16(closure_iter).take(closure_len) {
                let c = c.expect("No unpaired surrogates");
                set.add_char(c);
            }
        };
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
struct CaseMappingUnfoldData<'data> {
    map: ZeroMap<'data, str, str>,
}

impl<'data> CaseMappingUnfoldData<'data> {
    const ROWS_INDEX: usize = 0;
    const ROW_WIDTH_INDEX: usize = 1;
    const STRING_WIDTH_INDEX: usize = 2;

    // TODO: pub(crate)
    pub fn try_from_raw(raw: &[u16]) -> Option<Self> {
	let num_rows = *raw.get(Self::ROWS_INDEX)? as usize;
	let row_width = *raw.get(Self::ROW_WIDTH_INDEX)? as usize;
	let string_width = *raw.get(Self::STRING_WIDTH_INDEX)? as usize;

	if row_width == 0 {
	    return None;
	}
	let row_data = &raw[row_width..];

	let mut map = ZeroMap::new();

	debug_assert!(num_rows == row_data.chunks_exact(row_width).count());

	for row in row_data.chunks_exact(row_width) {
	    let key = Self::decode_string(&row[..string_width]);
	    let val = Self::decode_string(&row[string_width..]);
	    if let Some(_) = map.try_append(key.as_ref(), val.as_ref()) {
		return None;
	    }
	}
	Some(Self { map })
    }
    fn decode_string(slice: &[u16]) -> String {
	let iter = slice.iter().map(|&c| c).take_while(|&c| c != 0);
	char::decode_utf16(iter)
	    .map(|c| c.expect("No unpaired surrogates"))
	    .collect::<String>()
    }
    pub fn get(&self, key: &str) -> Option<&str> {
	self.map.get(key)
    }
}

pub struct FoldOptions(u32);

impl FoldOptions {
    const EXCLUDE_SPECIAL_I_FLAG: u32 = 0x1;

    pub fn with_turkic_mappings() -> Self {
        Self(Self::EXCLUDE_SPECIAL_I_FLAG)
    }

    pub fn exclude_special_i(&self) -> bool {
        self.0 & Self::EXCLUDE_SPECIAL_I_FLAG != 0
    }
}

impl Default for FoldOptions {
    fn default() -> Self {
        Self(0)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
pub struct CaseMapping<'data> {
    trie: CodePointTrie<'data, CaseMappingData>,
    exceptions: CaseMappingExceptions<'data>,
    unfold: CaseMappingUnfoldData<'data>
}

impl<'data> CaseMapping<'data> {
    fn lookup_data(&self, c: char) -> CaseMappingData {
        self.trie.get(c as u32)
    }

    pub fn to_lower(&self, c: char) -> char {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.is_upper_or_title() {
                let lower = c as i32 + data.delta() as i32;
                char::from_u32(lower as u32).expect("Character should be valid")
            } else {
                c
            }
        } else {
            let idx = data.exception_index();
            if data.is_upper_or_title()
                && self
                    .exceptions
                    .has_slot(idx, CaseMappingExceptionSlot::Delta)
            {
                let lower = c as i32 + self.exceptions.delta(idx);
                char::from_u32(lower as u32).expect("Character should be valid")
            } else if self
                .exceptions
                .has_slot(idx, CaseMappingExceptionSlot::Lower)
            {
                self.exceptions
                    .slot_char(idx, CaseMappingExceptionSlot::Lower)
            } else {
                c
            }
        }
    }

    pub fn to_upper(&self, c: char) -> char {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.case_type() == CaseType::Lower {
                let upper = c as i32 + data.delta() as i32;
                char::from_u32(upper as u32).expect("Character should be valid")
            } else {
                c
            }
        } else {
            let idx = data.exception_index();
            if data.case_type() == CaseType::Lower
                && self
                    .exceptions
                    .has_slot(idx, CaseMappingExceptionSlot::Delta)
            {
                let upper = c as i32 + self.exceptions.delta(idx);
                char::from_u32(upper as u32).expect("Character should be valid")
            } else if self
                .exceptions
                .has_slot(idx, CaseMappingExceptionSlot::Upper)
            {
                self.exceptions
                    .slot_char(idx, CaseMappingExceptionSlot::Upper)
            } else {
                c
            }
        }
    }

    pub fn to_title(&self, c: char) -> char {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            // In general, titlecase is the same as uppercase.
            if data.case_type() == CaseType::Lower {
                let upper = c as i32 + data.delta() as i32;
                char::from_u32(upper as u32).expect("Character should be valid")
            } else {
                c
            }
        } else {
            let idx = data.exception_index();
            if data.case_type() == CaseType::Lower
                && self
                    .exceptions
                    .has_slot(idx, CaseMappingExceptionSlot::Delta)
            {
                let upper = c as i32 + self.exceptions.delta(idx);
                char::from_u32(upper as u32).expect("Character should be valid")
            } else if self
                .exceptions
                .has_slot(idx, CaseMappingExceptionSlot::Title)
            {
                self.exceptions
                    .slot_char(idx, CaseMappingExceptionSlot::Title)
            } else if self
                .exceptions
                .has_slot(idx, CaseMappingExceptionSlot::Upper)
            {
                self.exceptions
                    .slot_char(idx, CaseMappingExceptionSlot::Upper)
            } else {
                c
            }
        }
    }

    /// Return the simple case folding mapping for `c`
    pub fn fold(&self, c: char, options: FoldOptions) -> char {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.is_upper_or_title() {
                let folded = c as i32 + data.delta() as i32;
                char::from_u32(folded as u32).expect("Character should be valid")
            } else {
                c
            }
        } else {
            let idx = data.exception_index();
            if self.exceptions.has_conditional_fold(idx) {
                self.conditional_fold(c, options)
            } else if self.exceptions.no_simple_case_folding(idx) {
                c
            } else if data.is_upper_or_title()
                && self
                    .exceptions
                    .has_slot(idx, CaseMappingExceptionSlot::Delta)
            {
                let folded = c as i32 + self.exceptions.delta(idx);
                char::from_u32(folded as u32).expect("Character should be valid")
            } else if self
                .exceptions
                .has_slot(idx, CaseMappingExceptionSlot::Fold)
            {
                self.exceptions
                    .slot_char(idx, CaseMappingExceptionSlot::Fold)
            } else if self
                .exceptions
                .has_slot(idx, CaseMappingExceptionSlot::Lower)
            {
                self.exceptions
                    .slot_char(idx, CaseMappingExceptionSlot::Lower)
            } else {
                c
            }
        }
    }

    // Special case folding mappings, hardcoded.
    // This handles the special Turkic mappings for uppercase I and dotted uppercase I
    // For non-Turkic languages, this mapping is normally not used.
    // For Turkic languages (tr, az), this mapping can be used instead of the normal mapping for these characters.
    fn conditional_fold(&self, c: char, options: FoldOptions) -> char {
        debug_assert!(c == '\u{49}' || c == '\u{130}');
        let is_turkic = options.exclude_special_i();
        match (c, is_turkic) {
            // Turkic mappings
            ('\u{49}', true) => '\u{131}', // 0049; T; 0131; # LATIN CAPITAL LETTER I
            ('\u{130}', true) => '\u{69}', // 0130; T; 0069; # LATIN CAPITAL LETTER I WITH DOT ABOVE

            // Default mappings
            ('\u{49}', false) => '\u{69}', // 0049; C; 0069; # LATIN CAPITAL LETTER I

            // There is no simple case folding for U+130.
            (c, _) => c,
        }
    }

    pub fn case_type(&self, c: char) -> CaseType {
        self.lookup_data(c).case_type()
    }

    pub fn is_ignorable(&self, c: char) -> bool {
        self.lookup_data(c).is_ignorable()
    }

    pub fn dot_type(&self, c: char) -> DotType {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            data.dot_type()
        } else {
            let idx = data.exception_index();
            self.exceptions.dot_type(idx)
        }
    }

    pub fn is_soft_dotted(&self, c: char) -> bool {
        self.lookup_data(c).dot_type() == DotType::SoftDotted
    }

    pub fn is_case_sensitive(&self, c: char) -> bool {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            data.is_sensitive()
        } else {
            let idx = data.exception_index();
            self.exceptions.is_sensitive(idx)
        }
    }
    pub fn add_case_closure<S: SetAdder>(&self, c: char, set: &mut S) {
        // Hardcode the case closure of i and its relatives and ignore the
        // data file data for these characters.
        // The Turkic dotless i and dotted I with their case mapping conditions
        // and case folding option make the related characters behave specially.
        // This code matches their closure behavior to their case folding behavior.
        match c {
            // Regular i and I are in one equivalence class.
            '\u{49}' => {
                set.add_char('\u{69}');
                return;
            }
            '\u{69}' => {
                set.add_char('\u{49}');
                return;
            }

            // Dotted I is in a class with <0069 0307> (for canonical equivalence with <0049 0307>)
            '\u{130}' => {
                set.add_string("\u{69}\u{307}");
                return;
            }

            // Dotless i is in a class by itself
            '\u{131}' => {
                return;
            }

            _ => {}
        }

        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.case_type() != CaseType::None {
                let delta = data.delta() as i32;
                if delta != 0 {
                    // Add the one simple case mapping, no matter what type it is.
                    let codepoint = c as i32 + delta;
                    let mapped =
                        char::from_u32(codepoint as u32).expect("Character should be valid");
                    set.add_char(mapped);
                }
            }
            return;
        }

        // c has exceptions, so there may be multiple simple and/or full case mappings.
        let idx = data.exception_index();

        // Add all simple case mappings.
        for slot in [
            CaseMappingExceptionSlot::Lower,
            CaseMappingExceptionSlot::Fold,
            CaseMappingExceptionSlot::Upper,
            CaseMappingExceptionSlot::Title,
        ] {
            if self.exceptions.has_slot(idx, slot) {
                let simple = self.exceptions.slot_char(idx, slot);
                set.add_char(simple);
            }
        }
        if self
            .exceptions
            .has_slot(idx, CaseMappingExceptionSlot::Delta)
        {
            let codepoint = c as i32 + self.exceptions.delta(idx);
            let mapped = char::from_u32(codepoint as u32).expect("Character should be valid");
            set.add_char(mapped);
        }

        self.exceptions.add_full_and_closure_mappings(idx, set);
    }

    pub fn add_string_case_closure<S: SetAdder>(&self, s: &str, set: &mut S) -> bool {
	if s.chars().count() <= 1 {
	    // The string is too short to find any match.
	    return false;
	}
	match self.unfold.get(s) {
	    Some(closure_string) => {
		for c in closure_string.chars() {
		    set.add_char(c);
		    self.add_case_closure(c, set);
		}
		true
	    },
	    None => false
	}
    }
}

pub trait SetAdder {
    fn add_char(&mut self, c: char);
    fn add_range(&mut self, start: char, end: char);
    fn add_string(&mut self, string: &str);
    // Note: SetAdder in ICU4C also has remove and remove_range, but these are only used for ISO/IEC 2022
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashSet;

//     pub(crate) fn get_case_mapping() -> CaseMapping<'static> {
// 	let trie = super::test_data::get_case_trie();
// 	let exceptions = CaseMappingExceptions {
// 	    raw: ZeroVec::from_slice(&test_data::UCASE_PROPS_EXCEPTIONS),
// 	};
// 	let unfold = CaseMappingUnfoldData::try_from_raw(&test_data::UCASE_PROPS_UNFOLD).unwrap();
// 	CaseMapping { trie, exceptions, unfold }
//     }

//     #[test]
//     fn test_upper() {
// 	let case_mapping = get_case_mapping();
// 	assert_eq!(case_mapping.to_upper('a'), 'A');
// 	assert_eq!(case_mapping.to_upper('\u{1c4}'), '\u{1c4}');
// 	assert_eq!(case_mapping.to_title('\u{1c4}'), '\u{1c5}');
// 	assert_eq!(case_mapping.to_lower('\u{1c4}'), '\u{1c6}');
// 	assert_eq!(case_mapping.to_upper('\u{1c5}'), '\u{1c4}');
// 	assert_eq!(case_mapping.to_title('\u{1c5}'), '\u{1c5}');
// 	assert_eq!(case_mapping.to_lower('\u{1c5}'), '\u{1c6}');
// 	assert_eq!(case_mapping.to_upper('\u{1c6}'), '\u{1c4}');
// 	assert_eq!(case_mapping.to_title('\u{1c6}'), '\u{1c5}');
// 	assert_eq!(case_mapping.to_lower('\u{1c6}'), '\u{1c6}');
//     }

//     #[test]
//     fn test_softdotted() {
// 	let case_mapping = get_case_mapping();
// 	assert_eq!(case_mapping.is_soft_dotted('a'), false);
// 	assert_eq!(case_mapping.dot_type('i'), DotType::SoftDotted);
// 	assert_eq!(case_mapping.dot_type('j'), DotType::SoftDotted);
//     }

//     #[derive(Eq,PartialEq,Default)]
//     struct SimpleSet {
// 	chars: HashSet<char>,
// 	strings: HashSet<String>,
//     }

//     impl SimpleSet {
// 	pub fn chars(&self) -> Vec<char> {
// 	    let mut result: Vec<char> = self.chars.iter().map(|&c| c).collect();
// 	    result.sort();
// 	    result
// 	}
// 	pub fn strings(&self) -> Vec<String> {
// 	    let mut result: Vec<String> = self.strings.iter().map(|c| c.clone()).collect();
// 	    result.sort();
// 	    result
// 	}
// 	pub fn clear(&mut self) {
// 	    self.chars.clear();
// 	    self.strings.clear();
// 	}
//     }

//     impl SetAdder for SimpleSet {
// 	fn add_char(&mut self, c: char) {
// 	    self.chars.insert(c);
// 	}
// 	fn add_string(&mut self, s: &str) {
// 	    self.strings.insert(String::from(s));
// 	}
// 	fn add_range(&mut self, start: char, end: char) {
// 	    for c in start..=end {
// 		self.chars.insert(c);
// 	    }
// 	}
//     }

//     #[test]
//     fn test_closure() {
// 	let case_mapping = get_case_mapping();
// 	let mut closure = SimpleSet::default();

// 	case_mapping.add_case_closure('i', &mut closure);
// 	assert_eq!(closure.chars(), vec!['I']);
// 	assert!(closure.strings().is_empty());
// 	closure.clear();

// 	case_mapping.add_case_closure('k', &mut closure);
// 	assert_eq!(closure.chars(), vec!['K', '\u{212a}']); // Kelvin sign
// 	assert!(closure.strings().is_empty());
// 	closure.clear();

// 	case_mapping.add_case_closure('s', &mut closure);
// 	assert_eq!(closure.chars(), vec!['S', '\u{17f}']); // long S
// 	assert!(closure.strings().is_empty());
// 	closure.clear();

// 	case_mapping.add_case_closure('\u{df}', &mut closure); // lowercase sharp s
// 	assert_eq!(closure.chars(), vec!['\u{1e9e}']); // uppercase sharp s
// 	assert_eq!(closure.strings(), vec![String::from("ss")]);
// 	closure.clear();
//     }
// }
