// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)]

use core::char::DecodeUtf16Error;
use core::convert::TryFrom;
use core::num::TryFromIntError;
use core::ops::Range;
use icu_codepointtrie::{CodePointTrie, CodePointTrieHeader, TrieValue};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use yoke::{Yokeable, ZeroCopyFrom};
use zerovec::ule::{AsULE, PlainOldULE};
use zerovec::{ZeroVec, ZeroMap};

use crate::error::Error;

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
    // After masking them to get a value between 0 and 3, this
    // function converts to CaseType.
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

/// The dot type of a Unicode character. This indicates how dotted
/// letters (like `i` and `j`) combine with accents placed above the
/// letter.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DotType {
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

    // The dot type is stored in either the codepoint trie or the
    // exception table as two bits.  After shifting and masking them
    // to get a value between 0 and 3, this function converts to
    // DotType.
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

// The datatype stored in the codepoint trie for casemapping.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct CaseMappingData(u16);

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

    // In simple cases, the CaseMappingData stores the delta between
    // a code point and its upper/lowercase equivalent. Exceptions
    // to this pattern are stored in a secondary data structure.
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

    // The delta between this code point and its upper/lowercase equivalent.
    // This should only be called for codepoints without exception data.
    #[inline]
    fn delta(&self) -> i16 {
        debug_assert!(!self.has_exception());
        (self.0 as i16) >> Self::DELTA_SHIFT
    }

    // The index of the exception data for this codepoint in the exception
    // table. This should only be called for codepoints with exception data.
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

// Case mapping exceptions that can't be represented as a delta applied to the original
// code point. Following ICU4C, data is stored as a u16 array. The codepoint trie in
// CaseMapping stores offsets into this array. The u16 at that index contains a set of
// flags describing the subsequent data.
//
//   [idx + 0]  Header word:
//       Bits:
//         0..7  Flag bits indicating which optional slots are present (if any):
//               0: Lowercase mapping (code point)
//               4: Case folding (code point)
//               2: Uppercase mapping (code point)
//               3: Titlecase mapping (code point)
//               4: Delta to simple case mapping (code point) (sign stored separately)
//               5: [RESERVED]
//               6: Closure mappings (see below)
//               7: Full mappings (see below)
//            8  Double-width slots. If set, then each optional slot is stored as two
//               elements of the array (high and low halves of 32-bit values) instead of
//               a single element.
//            9  Has no simple case folding, even if there is a simple lowercase mapping
//           10  The value in the delta slot is negative
//           11  Is case-sensitive
//       12..13  Dot type
//           14  Has conditional special casing
//           15  Has conditional case folding
//
//   If double-width slots is false:
//
//   [idx + 1] First optional slot
//   [idx + 2] Second optional slot
//   [idx + 3] Third optional slot
//   ...
//
//   If double-width slots is true:
//
//   [idx + 1] First optional slot
//   [idx + 3] Second optional slot
//   [idx + 5] Third optional slot
//   ...
//
//   Full mappings: If there is at least one full (string) case mapping, then the lengths
//   of the mappings are encoded as nibbles in the full mappings slot:
//       Bits:
//          0..4   Length of lowercase string
//          5..7   Length of case mapping string
//          8..11  Length of uppercase string
//          12..15 Length of titlecase string
//   Mappings that do not exist have length 0. The strings themselves are stored in the
//   above order immediately following the last optional slot, encoded as UTF16.
//
//   Case closure mappings: If the case closure for a code point includes code points
//   that are not included in the simple or full mappings, then bits 0..3 of the closure
//   mappings slot will contain the number of codepoints in the closure string. (Other
//   bits are reserved.) The closure string itself is encoded as UTF16 and stored
//   following the full mappings data (if it exists) or the final optional slot.
//
//   This representation is generated by casepropsbuilder.cpp in ICU.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
struct CaseMappingExceptions<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    raw: ZeroVec<'data, u16>,
}

#[derive(Copy, Clone, Debug)]
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

impl CaseMappingExceptionSlot {
    fn contains_char(&self) -> bool {
	match self {
	    Self::Lower | Self::Fold | Self::Upper | Self::Title => true,
	    _ => false
	}
    }
}

#[derive(Copy, Clone, Debug)]
enum FullMapping {
    Lower = 0,
    Fold = 1,
    Upper = 2,
    Title = 3,
}

impl<'data> CaseMappingExceptions<'data> {
    const SLOTS_MASK: u16 = 0xff;

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

    fn try_from_icu(raw: &[u16]) -> Result<(Self, Vec<u16>), Error> {
	let raw = ZeroVec::clone_from_slice(raw);
	let exceptions = Self { raw };
	let valid_indices = exceptions.validate()?;
	Ok((exceptions, valid_indices))
    }

    // Returns the array element at the given index
    #[inline(always)]
    fn get(&self, idx: usize) -> u16 {
        <u16 as AsULE>::from_unaligned(self.raw.as_slice()[idx as usize])
    }

    // Given a base index, returns the number of optional slots for the entry at that index
    fn num_slots(&self, base_idx: u16) -> u16 {
	let slot_bits = self.get(base_idx as usize) & Self::SLOTS_MASK;
	slot_bits.count_ones() as u16
    }

    // Given a base index, returns true if the given slot exists
    #[inline]
    fn has_slot(&self, base_idx: u16, slot: CaseMappingExceptionSlot) -> bool {
        let bit = 1u16 << (slot as u16);
        self.get(base_idx as usize) & bit != 0
    }

    // Given a base index, returns the index of a given slot
    fn slot_index(&self, base_idx: u16, slot: CaseMappingExceptionSlot) -> usize {
        debug_assert!(self.has_slot(base_idx, slot));
        let flags = self.get(base_idx as usize);

        let slot_bit = 1u16 << (slot as u16);
        let previous_slot_mask = slot_bit - 1;
        let previous_slots = flags & previous_slot_mask;
        let slot_num = previous_slots.count_ones() as usize;

        let offset = if self.has_double_slots(base_idx) {
            slot_num * 2
        } else {
            slot_num
        };

        base_idx as usize + 1 + offset
    }

    // Given a base index, returns the value stored in a given slot
    fn slot_value(&self, base_idx: u16, slot: CaseMappingExceptionSlot) -> u32 {
        debug_assert!(self.has_slot(base_idx, slot));
        let slot_idx = self.slot_index(base_idx, slot);
        if self.has_double_slots(base_idx) {
            let hi = self.get(slot_idx) as u32;
            let lo = self.get(slot_idx + 1) as u32;
            hi << 16 | lo
        } else {
            self.get(slot_idx) as u32
        }
    }

    // Given a base index, returns the character stored in a given slot
    fn slot_char(&self, base_idx: u16, slot: CaseMappingExceptionSlot) -> char {
        let raw = self.slot_value(base_idx, slot);

	// Safety: the exception data is validated on construction.
	debug_assert!(slot.contains_char());
	debug_assert!(char::from_u32(raw).is_some());
        unsafe { char::from_u32_unchecked(raw) }
    }

    // Given a base index, returns the delta (with the correct sign)
    fn delta(&self, base_idx: u16) -> i32 {
        debug_assert!(self.has_slot(base_idx, CaseMappingExceptionSlot::Delta));
        let raw: i32 = self.slot_value(base_idx, CaseMappingExceptionSlot::Delta) as _;
        if self.get(base_idx as usize) & Self::DELTA_IS_NEGATIVE_FLAG != 0 {
            -raw
        } else {
            raw
        }
    }

    // Given a base index, returns whether the entry beginning at that index has double-width slots.
    fn has_double_slots(&self, base_idx: u16) -> bool {
        self.get(base_idx as usize) & Self::DOUBLE_SLOTS_FLAG != 0
    }

    // Given a base index, returns whether there is a simple case folding
    fn no_simple_case_folding(&self, base_idx: u16) -> bool {
        self.get(base_idx as usize) & Self::NO_SIMPLE_CASE_FOLDING_FLAG != 0
    }

    // Given a base index, returns whether this code point is case-sensitive.
    // (Note that this information is stored in the trie for code points without
    // exception data, but the exception index requires more bits than the delta.)
    fn is_sensitive(&self, base_idx: u16) -> bool {
        self.get(base_idx as usize) & Self::SENSITIVE_FLAG != 0
    }

    // Given a base index, returns whether there is a conditional case fold.
    // (This is used for Turkic mappings for dotted/dotless i.)
    fn has_conditional_fold(&self, base_idx: u16) -> bool {
        self.get(base_idx as usize) & Self::CONDITIONAL_FOLD_FLAG != 0
    }

    // Given a base index, returns whether there is a language-specific case mapping.
    fn has_conditional_special(&self, base_idx: u16) -> bool {
        self.get(base_idx as usize) & Self::CONDITIONAL_SPECIAL_FLAG != 0
    }

    // Given a base index, returns the dot type.
    // (Note that this information is stored in the trie for code points without
    // exception data, but the exception index requires more bits than the delta.)
    fn dot_type(&self, base_idx: u16) -> DotType {
        let masked_bits = (self.get(base_idx as usize) >> Self::DOT_SHIFT) & DotType::DOT_MASK;
        DotType::from_masked_bits(masked_bits)
    }

    // Given a base index, returns the bounds of the given full mapping string.
    // Used to read full mapping strings, and also to find the start of the closure string.
    #[inline(always)]
    fn full_mapping_string_range(&self, base_idx: u16, slot: FullMapping) -> Range<usize> {
	debug_assert!(self.has_slot(base_idx, CaseMappingExceptionSlot::FullMappings));
	let mut lengths = self.slot_value(base_idx, CaseMappingExceptionSlot::FullMappings);
        let mut data_idx = self.slot_index(base_idx, CaseMappingExceptionSlot::FullMappings) + 1;

	// Skip past previous slots
	for _ in 0 .. (slot as usize) {
            let str_len = lengths & Self::FULL_MAPPINGS_LENGTH_MASK;
            data_idx += str_len as usize;
            lengths >>= Self::FULL_MAPPINGS_LENGTH_SHIFT;
	}

	// Return range of this string
        let str_len = (lengths & Self::FULL_MAPPINGS_LENGTH_MASK) as usize;
	data_idx .. (data_idx + str_len)
    }

    // Full mapping strings are stored as UTF16 immediately following the full mappings slot.
    // The length of each full mapping string (in code units) is stored in a nibble of the slot.
    fn full_mapping_string(&self, base_idx: u16, slot: FullMapping) -> Result<String, DecodeUtf16Error> {
	debug_assert!(self.has_slot(base_idx, CaseMappingExceptionSlot::FullMappings));
	let range = self.full_mapping_string_range(base_idx, slot);
        let iter = self.raw.as_slice()[range]
            .iter()
            .map(|&ule| <u16 as AsULE>::from_unaligned(ule));
        char::decode_utf16(iter).collect()
    }

    // Given a base index, returns the length of the closure string
    fn closure_len(&self, base_idx: u16) -> usize {
	debug_assert!(self.has_slot(base_idx, CaseMappingExceptionSlot::Closure));
        let value = self.slot_value(base_idx, CaseMappingExceptionSlot::Closure);
        (value & Self::CLOSURE_MAX_LENGTH) as usize
    }

    // The closure string is stored as UTF16 following the full mappings (if they exist) or the
    // closure slot (if no full mappings are present). The closure slot stores the length of the
    // closure string (in code points).
    // This function returns an iterator over the characters of the closure string.
    fn closure_string(&self, base_idx: u16) -> Result<String, DecodeUtf16Error> {
	let start_idx = if self.has_slot(base_idx, CaseMappingExceptionSlot::FullMappings) {
	    self.full_mapping_string_range(base_idx, FullMapping::Title).end
	} else {
	    self.slot_index(base_idx, CaseMappingExceptionSlot::Closure) + 1
	};
	let closure_len = self.closure_len(base_idx);
	let u16_iter = self.raw.as_slice()[start_idx..]
	    .iter()
	    .map(|&ule| <u16 as AsULE>::from_unaligned(ule));
	char::decode_utf16(u16_iter).take(closure_len).collect()
    }

    fn add_full_and_closure_mappings<S: SetAdder>(&self, base_idx: u16, set: &mut S) {
        if self.has_slot(base_idx, CaseMappingExceptionSlot::FullMappings) {
	    let mapping_string = self.full_mapping_string(base_idx, FullMapping::Fold)
		.expect("No unpaired surrogates");
	    if !mapping_string.is_empty() {
		set.add_string(&mapping_string);
	    }
        };

        if self.has_slot(base_idx, CaseMappingExceptionSlot::Closure) {
	    for c in self.closure_string(base_idx).expect("No unpaired surrogates").chars() {
                set.add_char(c);
	    }
        };
    }

    fn validate(&self) -> Result<Vec<u16>, Error> {
	let mut valid_indices = vec![];
	let mut idx: u16 = 0;

	let data_len = self.raw.len() as u16;

	while idx < data_len {
	    let mut slot_len = self.num_slots(idx);
	    if self.has_double_slots(idx) {
		slot_len *= 2;
	    }

	    // Validate that we can read slot data without reading out of bounds.
	    let mut next_idx = idx + 1 + slot_len;
	    if next_idx > data_len {
		return Error::invalid("Exceptions: missing slot data");
	    }

	    // Validate slots that should contain chars
	    for slot in [CaseMappingExceptionSlot::Lower, CaseMappingExceptionSlot::Fold,
			 CaseMappingExceptionSlot::Upper, CaseMappingExceptionSlot::Title] {
		if self.has_slot(idx, slot) {
		    let val = self.slot_value(idx, slot);
		    if !char::from_u32(val).is_some() {
			return Error::invalid("Exceptions: invalid char slot");
		    }
		}
	    }

	    // Validate full mappings.
	    if self.has_slot(idx, CaseMappingExceptionSlot::FullMappings) {
		for full_mapping in [FullMapping::Lower, FullMapping::Fold,
				     FullMapping::Upper, FullMapping::Title] {
		    let range = self.full_mapping_string_range(idx, full_mapping);
		    next_idx += range.len() as u16;
		    if next_idx > data_len {
			return Error::invalid("Exceptions: missing full mapping data");
		    }

		    self.full_mapping_string(idx, full_mapping)?;
		}
	    }

	    // Validate closure string.
	    if self.has_slot(idx, CaseMappingExceptionSlot::Closure) {
		let closure_len = self.closure_len(idx);
		if next_idx + closure_len as u16 > data_len {
		    return Error::invalid("Exceptions: missing closure data");
		}

		for c in self.closure_string(idx)?.chars() {
		    next_idx += c.len_utf16() as u16;
		}
	    }

	    valid_indices.push(idx);
	    idx = next_idx;
	}
	Ok(valid_indices)
    }
}

/// Reverse case folding data. Maps from multi-character strings back to code-points that fold to those
/// strings.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[yoke(prove_covariance_manually)]
struct CaseMappingUnfoldData<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    map: ZeroMap<'data, str, str>,
}

impl<'data> CaseMappingUnfoldData<'data> {
    // Unfold data is exported by ICU as an array of 16-bit values, representing a short
    // header followed by a two-column key/value table. The header indicates:
    // - The number of rows.
    // - The number of UTF16 code units per row.
    // - The number of UTF16 code units in the first (key) column.
    //   (The number of code units in the value column can be derived from the above.)
    //
    // The key in the first column is the case folding of each of the code points in
    // the second column. Keys/values that are shorter than the column width are
    // null-terminated. The table is sorted by key. Binary search is used to find the value.
    //
    // Rust strings are UTF8 by default. To avoid the cost of converting from UTF16 on access,
    // we convert the ICU data into a more convenient format during construction.
    fn try_from_icu(raw: &[u16]) -> Result<Self, Error> {
	const ROWS_INDEX: usize = 0;
	const ROW_WIDTH_INDEX: usize = 1;
	const STRING_WIDTH_INDEX: usize = 2;

	if raw.len() <= STRING_WIDTH_INDEX {
	    return Error::invalid("Unfold: header missing");
	}

	let num_rows = raw[ROWS_INDEX] as usize;
	let row_width = raw[ROW_WIDTH_INDEX] as usize;
	let string_width = raw[STRING_WIDTH_INDEX] as usize;

	if row_width == 0 {
	    return Error::invalid("Unfold: invalid row width");
	}

	// Header takes up one row.
	let row_data = &raw[row_width..];

	let mut map = ZeroMap::new();

	debug_assert!(num_rows == row_data.chunks_exact(row_width).count());
	for row in row_data.chunks_exact(row_width) {
	    let key = Self::decode_string(&row[..string_width])
		.ok_or(Error::Validation("Unfold: unpaired surrogate in key"))?;
	    let val = Self::decode_string(&row[string_width..])
		.ok_or(Error::Validation("Unfold: unpaired surrogate in value"))?;
	    if let Some(_) = map.try_append(key.as_ref(), val.as_ref()) {
		return Error::invalid("Unfold: keys not sorted/unique");
	    }
	}
	Ok(Self { map })
    }

    // Decode a zero-terminated UTF16 string from a slice of u16.
    fn decode_string(slice: &[u16]) -> Option<String> {
	let iter = slice.iter().map(|&c| c).take_while(|&c| c != 0);
	char::decode_utf16(iter).collect::<Result<String, _>>().ok()
    }

    // Given a string, returns another string representing the set of characters
    // that case fold to that string.
    fn get(&self, key: &str) -> Option<&str> {
	self.map.get(key)
    }
}

// Used to control the behavior of CaseMapping::fold.
// Currently only used to decide whether to use Turkic (T) mappings for dotted/dotless i.
struct FoldOptions {
    exclude_special_i: bool,
}

impl FoldOptions {
    fn with_turkic_mappings() -> Self {
        Self {
	    exclude_special_i: true,
	}
    }
}

impl Default for FoldOptions {
    fn default() -> Self {
        Self {
	    exclude_special_i: false,
	}
    }
}

/// CaseMapping provides low-level access to the data necessary to
/// convert characters and strings to upper, lower, or title case.
///
/// TODO: more documentation once we decide on the API surface.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[yoke(prove_covariance_manually)]
pub struct CaseMapping<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    trie: CodePointTrie<'data, CaseMappingData>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    exceptions: CaseMappingExceptions<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    unfold: CaseMappingUnfoldData<'data>
}

impl<'data> CaseMapping<'data> {
    pub fn try_from_icu(trie_header: CodePointTrieHeader,
			trie_index: &[u16],
			trie_data: &[u16],
			exceptions: &[u16],
			unfold: &[u16]) -> Result<Self, Error> {
	let (exceptions, valid_exception_indices) = CaseMappingExceptions::try_from_icu(exceptions)?;
	let unfold = CaseMappingUnfoldData::try_from_icu(unfold)?;

	let trie_index = ZeroVec::clone_from_slice(trie_index);
	let trie_data = trie_data.iter().map(|&i| CaseMappingData(i)).collect::<ZeroVec<_>>();

	for data in trie_data.iter() {
	    if data.has_exception() && !valid_exception_indices.contains(&data.exception_index()) {
		return Error::invalid("Invalid exception index in trie data");
	    }
	}

	let trie = CodePointTrie::try_new(trie_header, trie_index, trie_data)?;

	Ok(Self { trie, exceptions, unfold })
    }

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
    fn fold(&self, c: char, options: FoldOptions) -> char {
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
        let is_turkic = options.exclude_special_i;
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

    fn dot_type(&self, c: char) -> DotType {
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

//
pub trait SetAdder {
    fn add_char(&mut self, c: char);
    fn add_string(&mut self, string: &str);
}
