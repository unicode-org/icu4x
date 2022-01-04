// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)]

use core::char::DecodeUtf16Error;
use core::convert::TryFrom;
use core::num::TryFromIntError;
use core::ops::Range;
#[cfg(feature = "provider_transform_internals")]
use icu_codepointtrie::CodePointTrieHeader;
use icu_codepointtrie::{CodePointTrie, TrieValue};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use yoke::{Yokeable, ZeroCopyFrom};
use zerovec::ule::{AsULE, RawBytesULE};
use zerovec::{ZeroMap, ZeroVec};

use crate::error::Error;

/// The case of a Unicode character
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CaseType {
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
    fn is_relevant_to(&self, kind: MappingKind) -> bool {
        match kind {
            MappingKind::Lower | MappingKind::Fold => self.is_upper_or_title(),
            MappingKind::Upper | MappingKind::Title => self.case_type() == CaseType::Lower,
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
    type ULE = RawBytesULE<2>;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        RawBytesULE(self.0.to_le_bytes())
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
enum ExceptionSlot {
    Lower = 0,
    Fold = 1,
    Upper = 2,
    Title = 3,
    Delta = 4,
    // Slot 5 is reserved
    Closure = 6,
    FullMappings = 7,
}

impl ExceptionSlot {
    fn contains_char(&self) -> bool {
        matches!(self, Self::Lower | Self::Fold | Self::Upper | Self::Title)
    }
}

impl From<MappingKind> for ExceptionSlot {
    fn from(full: MappingKind) -> Self {
        match full {
            MappingKind::Lower => Self::Lower,
            MappingKind::Fold => Self::Fold,
            MappingKind::Upper => Self::Upper,
            MappingKind::Title => Self::Title,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MappingKind {
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

    const MAPPINGS_ALL_LENGTHS_MASK: u32 = 0xffff;
    const FULL_MAPPINGS_LENGTH_MASK: u32 = 0xf;
    const FULL_MAPPINGS_LENGTH_SHIFT: u32 = 4;

    const CLOSURE_MAX_LENGTH: u32 = 0xf;

    // Returns the array element at the given index
    #[inline(always)]
    fn get(&self, idx: usize) -> u16 {
        self.raw.get(idx).expect("Checked in validate()")
    }

    // Given a base index, returns the number of optional slots for the entry at that index
    fn num_slots(&self, base_idx: u16) -> u16 {
        let slot_bits = self.get(base_idx as usize) & Self::SLOTS_MASK;
        slot_bits.count_ones() as u16
    }

    // Given a base index, returns true if the given slot exists
    #[inline]
    fn has_slot(&self, base_idx: u16, slot: ExceptionSlot) -> bool {
        let bit = 1u16 << (slot as u16);
        self.get(base_idx as usize) & bit != 0
    }

    // Given a base index, returns the index of a given slot
    fn slot_index(&self, base_idx: u16, slot: ExceptionSlot) -> usize {
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
    fn slot_value(&self, base_idx: u16, slot: ExceptionSlot) -> u32 {
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
    fn slot_char(&self, base_idx: u16, slot: ExceptionSlot) -> Option<char> {
        debug_assert!(slot.contains_char());
        if self.has_slot(base_idx, slot) {
            let raw = self.slot_value(base_idx, slot);
            Some(char::from_u32(raw).expect("Checked in validate() (step #1)"))
        } else {
            None
        }
    }

    fn slot_char_for_kind(&self, base_idx: u16, kind: MappingKind) -> Option<char> {
        match kind {
            MappingKind::Lower | MappingKind::Upper => self.slot_char(base_idx, kind.into()),
            MappingKind::Fold => self
                .slot_char(base_idx, ExceptionSlot::Fold)
                .or_else(|| self.slot_char(base_idx, ExceptionSlot::Lower)),
            MappingKind::Title => self
                .slot_char(base_idx, ExceptionSlot::Title)
                .or_else(|| self.slot_char(base_idx, ExceptionSlot::Upper)),
        }
    }

    fn has_delta(&self, base_idx: u16) -> bool {
        self.has_slot(base_idx, ExceptionSlot::Delta)
    }

    // Given a base index, returns the delta (with the correct sign)
    fn delta(&self, base_idx: u16) -> i32 {
        debug_assert!(self.has_slot(base_idx, ExceptionSlot::Delta));
        let raw: i32 = self.slot_value(base_idx, ExceptionSlot::Delta) as _;
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
    fn full_mapping_string_range(&self, base_idx: u16, slot: MappingKind) -> Range<usize> {
        debug_assert!(self.has_slot(base_idx, ExceptionSlot::FullMappings));
        let mut lengths = self.slot_value(base_idx, ExceptionSlot::FullMappings);
        let mut data_idx = self.slot_index(base_idx, ExceptionSlot::FullMappings) + 1;

        // Skip past previous slots
        for _ in 0..(slot as usize) {
            let str_len = lengths & Self::FULL_MAPPINGS_LENGTH_MASK;
            data_idx += str_len as usize;
            lengths >>= Self::FULL_MAPPINGS_LENGTH_SHIFT;
        }

        // Return range of this string
        let str_len = (lengths & Self::FULL_MAPPINGS_LENGTH_MASK) as usize;
        data_idx..(data_idx + str_len)
    }

    // Full mapping strings are stored as UTF16 immediately following the full mappings slot.
    // The length of each full mapping string (in code units) is stored in a nibble of the slot.
    fn full_mapping_string(
        &self,
        base_idx: u16,
        slot: MappingKind,
    ) -> Result<String, DecodeUtf16Error> {
        debug_assert!(self.has_slot(base_idx, ExceptionSlot::FullMappings));
        let range = self.full_mapping_string_range(base_idx, slot);
        let iter = self.raw.as_ule_slice()[range]
            .iter()
            .map(|&ule| <u16 as AsULE>::from_unaligned(ule));
        char::decode_utf16(iter).collect()
    }

    // Given a base index, returns the length of the closure string
    fn closure_len(&self, base_idx: u16) -> usize {
        debug_assert!(self.has_slot(base_idx, ExceptionSlot::Closure));
        let value = self.slot_value(base_idx, ExceptionSlot::Closure);
        (value & Self::CLOSURE_MAX_LENGTH) as usize
    }

    // The closure string is stored as UTF16 following the full mappings (if they exist) or the
    // closure slot (if no full mappings are present). The closure slot stores the length of the
    // closure string (in code points).
    // This function returns an iterator over the characters of the closure string.
    fn closure_string(&self, base_idx: u16) -> Result<String, DecodeUtf16Error> {
        let start_idx = if self.has_slot(base_idx, ExceptionSlot::FullMappings) {
            self.full_mapping_string_range(base_idx, MappingKind::Title)
                .end
        } else {
            self.slot_index(base_idx, ExceptionSlot::Closure) + 1
        };
        let closure_len = self.closure_len(base_idx);
        let u16_iter = self.raw.as_ule_slice()[start_idx..]
            .iter()
            .map(|&ule| <u16 as AsULE>::from_unaligned(ule));
        char::decode_utf16(u16_iter).take(closure_len).collect()
    }

    fn add_full_and_closure_mappings<S: SetAdder>(&self, base_idx: u16, set: &mut S) {
        if self.has_slot(base_idx, ExceptionSlot::FullMappings) {
            let mapping_string = self
                .full_mapping_string(base_idx, MappingKind::Fold)
                .expect("Checked in validate() (step #2)");
            if !mapping_string.is_empty() {
                set.add_string(&mapping_string);
            }
        };

        if self.has_slot(base_idx, ExceptionSlot::Closure) {
            for c in self
                .closure_string(base_idx)
                .expect("Checked in validate() (step #3)")
                .chars()
            {
                set.add_char(c);
            }
        };
    }

    #[cfg(any(feature = "provider_transform_internals", test))]
    fn from_raw(raw: &[u16]) -> Self {
        let raw = ZeroVec::alloc_from_slice(raw);
        Self { raw }
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

            // #1: Validate slots that should contain chars.
            for slot in [
                ExceptionSlot::Lower,
                ExceptionSlot::Fold,
                ExceptionSlot::Upper,
                ExceptionSlot::Title,
            ] {
                if self.has_slot(idx, slot) {
                    let val = self.slot_value(idx, slot);
                    if char::from_u32(val).is_none() {
                        return Error::invalid("Exceptions: invalid char slot");
                    }
                }
            }

            // #2: Validate full mappings.
            if self.has_slot(idx, ExceptionSlot::FullMappings) {
                for full_mapping in [
                    MappingKind::Lower,
                    MappingKind::Fold,
                    MappingKind::Upper,
                    MappingKind::Title,
                ] {
                    let range = self.full_mapping_string_range(idx, full_mapping);
                    next_idx += range.len() as u16;
                    if next_idx > data_len {
                        return Error::invalid("Exceptions: missing full mapping data");
                    }

                    self.full_mapping_string(idx, full_mapping)?;
                }
            }

            // #3: Validate closure string.
            if self.has_slot(idx, ExceptionSlot::Closure) {
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

#[test]
fn test_exception_validation() {
    let missing_slot_data: &[u16] = &[1 << (ExceptionSlot::Lower as u16)];
    let result = CaseMappingExceptions::from_raw(missing_slot_data).validate();
    assert_eq!(
        result,
        Err(Error::Validation("Exceptions: missing slot data"))
    );

    let invalid_char_slot: &[u16] = &[1 << (ExceptionSlot::Lower as u16), 0xdf00];
    let result = CaseMappingExceptions::from_raw(invalid_char_slot).validate();
    assert_eq!(
        result,
        Err(Error::Validation("Exceptions: invalid char slot"))
    );

    let missing_full_mappings: &[u16] = &[1 << (ExceptionSlot::FullMappings as u16), 0x1111];
    let result = CaseMappingExceptions::from_raw(missing_full_mappings).validate();
    assert_eq!(
        result,
        Err(Error::Validation("Exceptions: missing full mapping data"))
    );

    let full_mapping_unpaired_surrogate: &[u16] =
        &[1 << (ExceptionSlot::FullMappings as u16), 0x1111, 0xdf00];
    let result = CaseMappingExceptions::from_raw(full_mapping_unpaired_surrogate).validate();
    assert!(matches!(result, Err(Error::DecodeUtf16(_))));

    let missing_closure_data: &[u16] = &[1 << (ExceptionSlot::Closure as u16), 0x1];
    let result = CaseMappingExceptions::from_raw(missing_closure_data).validate();
    assert_eq!(
        result,
        Err(Error::Validation("Exceptions: missing closure data"))
    );

    let closure_unpaired_surrogate: &[u16] = &[1 << (ExceptionSlot::Closure as u16), 0x1, 0xdf00];
    let result = CaseMappingExceptions::from_raw(closure_unpaired_surrogate).validate();
    assert!(matches!(result, Err(Error::DecodeUtf16(_))));
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
    #[cfg(feature = "provider_transform_internals")]
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
            if map.try_append(key.as_ref(), val.as_ref()).is_some() {
                return Error::invalid("Unfold: keys not sorted/unique");
            }
        }
        Ok(Self { map })
    }

    // Decode a zero-terminated UTF16 string from a slice of u16.
    #[cfg(feature = "provider_transform_internals")]
    fn decode_string(slice: &[u16]) -> Option<String> {
        let iter = slice.iter().copied().take_while(|&c| c != 0);
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
    unfold: CaseMappingUnfoldData<'data>,
}

impl<'data> CaseMapping<'data> {
    /// Creates a new CaseMapping using data exported by the `icuexportdata` tool
    /// in ICU4C. Validates that the data is consistent.
    #[cfg(feature = "provider_transform_internals")]
    pub fn try_from_icu(
        trie_header: CodePointTrieHeader,
        trie_index: &[u16],
        trie_data: &[u16],
        exceptions: &[u16],
        unfold: &[u16],
    ) -> Result<Self, Error> {
        let exceptions = CaseMappingExceptions::from_raw(exceptions);
        let unfold = CaseMappingUnfoldData::try_from_icu(unfold)?;

        let trie_index = ZeroVec::alloc_from_slice(trie_index);
        let trie_data = trie_data
            .iter()
            .map(|&i| CaseMappingData(i))
            .collect::<ZeroVec<_>>();
        let trie = CodePointTrie::try_new(trie_header, trie_index, trie_data)?;

        let result = Self {
            trie,
            exceptions,
            unfold,
        };
        result.validate()?;
        Ok(result)
    }

    /// Given an existing CaseMapping, validates that the data is
    /// consistent.  A CaseMapping created by the ICU transformer has
    /// already been validated. Calling this function is only
    /// necessary if you are concerned about data corruption after
    /// deserializing.
    pub fn validate(&self) -> Result<(), Error> {
        // First, validate that exception data is well-formed.
        let valid_exception_indices = self.exceptions.validate()?;

        let validate_delta = |c: char, delta: i32| -> Result<(), Error> {
            let new_c = u32::try_from(c as i32 + delta)
                .map_err(|_| Error::Validation("Delta larger than character"))?;
            char::from_u32(new_c).ok_or(Error::Validation("Invalid delta"))?;
            Ok(())
        };

        for i in 0..char::MAX as u32 {
            if let Some(c) = char::from_u32(i) {
                let data = self.lookup_data(c);

                if data.has_exception() {
                    let idx = data.exception_index();
                    // Verify that the exception index points to a valid exception header.
                    if !valid_exception_indices.contains(&idx) {
                        return Error::invalid("Invalid exception index in trie data");
                    }
                    if self.exceptions.has_slot(idx, ExceptionSlot::Delta) {
                        validate_delta(c, self.exceptions.delta(idx))?;
                    }
                } else {
                    validate_delta(c, data.delta() as i32)?;
                }
            }
        }

        // The unfold data is structurally guaranteed to be valid,
        // so there is nothing left to check.
        Ok(())
    }

    fn lookup_data(&self, c: char) -> CaseMappingData {
        self.trie.get(c as u32)
    }

    fn simple_helper(&self, c: char, kind: MappingKind) -> char {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.is_relevant_to(kind) {
                let folded = c as i32 + data.delta() as i32;
                char::from_u32(folded as u32).expect("Checked in validate()")
            } else {
                c
            }
        } else {
            let idx = data.exception_index();
            if data.is_relevant_to(kind) && self.exceptions.has_delta(idx) {
                let folded = c as i32 + self.exceptions.delta(idx);
                return char::from_u32(folded as u32).expect("Checked in validate()");
            }
            self.exceptions.slot_char_for_kind(idx, kind).unwrap_or(c)
        }
    }

    /// Returns the lowercase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    pub fn to_lower(&self, c: char) -> char {
        self.simple_helper(c, MappingKind::Lower)
    }

    /// Returns the uppercase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    pub fn to_upper(&self, c: char) -> char {
        self.simple_helper(c, MappingKind::Upper)
    }

    /// Returns the titlecase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    pub fn to_title(&self, c: char) -> char {
        self.simple_helper(c, MappingKind::Title)
    }

    /// Return the simple case folding mapping of the given char.
    fn fold(&self, c: char, options: FoldOptions) -> char {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.is_upper_or_title() {
                let folded = c as i32 + data.delta() as i32;
                char::from_u32(folded as u32).expect("Checked in validate()")
            } else {
                c
            }
        } else {
            // TODO: if we can move conditional fold and no_simple_case_folding into
	    // simple_helper, this function can just call simple_helper.
            let idx = data.exception_index();
            if self.exceptions.has_conditional_fold(idx) {
                self.conditional_fold(c, options)
            } else if self.exceptions.no_simple_case_folding(idx) {
                c
            } else if data.is_upper_or_title()
                && self.exceptions.has_slot(idx, ExceptionSlot::Delta)
            {
                let folded = c as i32 + self.exceptions.delta(idx);
                char::from_u32(folded as u32).expect("Checked in validate()")
            } else if let Some(slot_char) =
                self.exceptions.slot_char_for_kind(idx, MappingKind::Fold)
            {
                slot_char
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

    fn dot_type(&self, c: char) -> DotType {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            data.dot_type()
        } else {
            let idx = data.exception_index();
            self.exceptions.dot_type(idx)
        }
    }

    /// Returns true if the character has a dot above it that should be replaced with an accent if one is added.
    /// TODO: implement functions using this, and make this function private.
    pub fn is_soft_dotted(&self, c: char) -> bool {
        self.dot_type(c) == DotType::SoftDotted
    }

    fn is_case_sensitive(&self, c: char) -> bool {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            data.is_sensitive()
        } else {
            let idx = data.exception_index();
            self.exceptions.is_sensitive(idx)
        }
    }

    #[inline(always)]
    fn full_helper(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
        kind: MappingKind,
    ) -> FullMappingResult {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.is_relevant_to(kind) {
                let mapped = c as i32 + data.delta() as i32;
                let mapped = char::from_u32(mapped as u32).expect("Checked in validate()");
                FullMappingResult::CodePoint(mapped)
            } else {
                FullMappingResult::CodePoint(c)
            }
        } else {
            let idx = data.exception_index();
            if self.exceptions.has_conditional_special(idx) {
                if let Some(special) = match kind {
                    MappingKind::Lower => self.to_full_lower_special_case(c, context, locale),
                    MappingKind::Fold => self.to_full_fold_special_case(c, context, locale),
                    MappingKind::Upper | MappingKind::Title => self
                        .to_full_upper_or_title_special_case(
                            c,
                            context,
                            locale,
                            kind == MappingKind::Title,
                        ),
                } {
                    return special;
                }
            }
            if self.exceptions.has_slot(idx, ExceptionSlot::FullMappings) {
                let mapped_string = self
                    .exceptions
                    .full_mapping_string(idx, kind)
                    .expect("Checked in validation");
                if !mapped_string.is_empty() {
                    return FullMappingResult::String(mapped_string);
                }
            }
            if data.is_relevant_to(kind) && self.exceptions.has_slot(idx, ExceptionSlot::Delta) {
                let mapped = c as i32 + self.exceptions.delta(idx);
                let mapped = char::from_u32(mapped as u32).expect("Checked in validate()");
                return FullMappingResult::CodePoint(mapped);
            }
            if kind == MappingKind::Fold && self.exceptions.no_simple_case_folding(idx) {
                return FullMappingResult::CodePoint(c);
            }

            if let Some(slot_char) = self.exceptions.slot_char_for_kind(idx, kind) {
                FullMappingResult::CodePoint(slot_char)
            } else {
                FullMappingResult::CodePoint(c)
            }
        }
    }

    pub(crate) fn to_full_lower(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> FullMappingResult {
        self.full_helper(c, context, locale, MappingKind::Lower)
    }

    pub(crate) fn to_full_upper(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> FullMappingResult {
        self.full_helper(c, context, locale, MappingKind::Upper)
    }

    pub(crate) fn to_full_title(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> FullMappingResult {
        self.full_helper(c, context, locale, MappingKind::Title)
    }

    pub(crate) fn to_full_folding(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> FullMappingResult {
        self.full_helper(c, context, locale, MappingKind::Fold)
    }

    // These constants are used for hardcoded locale-specific foldings.
    const I_DOT: &'static str = "\u{69}\u{307}";
    const J_DOT: &'static str = "\u{6a}\u{307}";
    const I_OGONEK_DOT: &'static str = "\u{12f}\u{307}";
    const I_DOT_GRAVE: &'static str = "\u{69}\u{307}\u{300}";
    const I_DOT_ACUTE: &'static str = "\u{69}\u{307}\u{301}";
    const I_DOT_TILDE: &'static str = "\u{69}\u{307}\u{303}";

    fn to_full_lower_special_case(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> Option<FullMappingResult> {
        if locale == CaseMapLocale::Lithuanian {
            // Lithuanian retains the dot in a lowercase i when followed by accents.
            // Introduce an explicit dot above when lowercasing capital I's and J's
            // whenever there are more accents above (of the accents used in
            // Lithuanian: grave, acute, and tilde above).

            // Check for accents above I, J, and I-with-ogonek.
            if c == 'I' && context.followed_by_more_above(&self) {
                return Some(FullMappingResult::string(Self::I_DOT));
            } else if c == 'J' && context.followed_by_more_above(&self) {
                return Some(FullMappingResult::string(Self::J_DOT));
            } else if c == '\u{12e}' && context.followed_by_more_above(&self) {
                return Some(FullMappingResult::string(Self::I_OGONEK_DOT));
            }

            // These characters are precomposed with accents above, so we don't
            // have to look at the context.
            if c == '\u{cc}' {
                return Some(FullMappingResult::string(Self::I_DOT_GRAVE));
            } else if c == '\u{cd}' {
                return Some(FullMappingResult::string(Self::I_DOT_ACUTE));
            } else if c == '\u{128}' {
                return Some(FullMappingResult::string(Self::I_DOT_TILDE));
            }
        }

        if locale == CaseMapLocale::Turkish {
            if c == '\u{130}' {
                // I and i-dotless; I-dot and i are case pairs in Turkish and Azeri
                return Some(FullMappingResult::CodePoint('i'));
            } else if c == '\u{307}' && context.preceded_by_capital_i(&self) {
                // When lowercasing, remove dot_above in the sequence I + dot_above,
                // which will turn into i. This matches the behaviour of the
                // canonically equivalent I-dot_above.
                return Some(FullMappingResult::Remove);
            } else if c == 'I' && !context.followed_by_dot_above(&self) {
                // When lowercasing, unless an I is before a dot_above, it turns
                // into a dotless i.
                return Some(FullMappingResult::CodePoint('\u{131}'));
            }
        }

        if c == '\u{130}' {
            // Preserve canonical equivalence for I with dot. Turkic is handled above.
            return Some(FullMappingResult::string(Self::I_DOT));
        }

        if c == '\u{3a3}'
            && context.preceded_by_cased_letter(&self)
            && !context.followed_by_cased_letter(&self)
        {
            // Greek capital sigman maps depending on surrounding cased letters.
            return Some(FullMappingResult::CodePoint('\u{3c2}'));
        }

        // No relevant special case mapping. Use a normal mapping.
        None
    }

    fn to_full_upper_or_title_special_case(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
        _is_title: bool,
    ) -> Option<FullMappingResult> {
        if locale == CaseMapLocale::Turkish && c == 'i' {
            // In Turkic languages, i turns into a dotted capital I.
            return Some(FullMappingResult::CodePoint('\u{130}'));
        }
        if locale == CaseMapLocale::Lithuanian
            && c == '\u{307}'
            && context.preceded_by_soft_dotted(&self)
        {
            // Lithuanian retains the dot in a lowercase i when followed by accents.
            // Remove dot_above after i with upper or titlecase.
            return Some(FullMappingResult::Remove);
        }
        // TODO: ICU4C has a non-standard extension for Armenian ligature ech-yiwn.
        // Should we also implement it?
        // if c == '\u{587}' {
        //     return match (locale, is_title) {
        // 	(CaseMapLocale::Armenian, false) => Some(FullMappingResult::string("ԵՎ")),
        // 	(CaseMapLocale::Armenian, true) => Some(FullMappingResult::string("Եվ")),
        // 	(_, false) => Some(FullMappingResult::string("ԵՒ")),
        // 	(_, true) => Some(FullMappingResult::string("Եւ")),
        //     }
        // }
        None
    }

    fn to_full_fold_special_case(
        &self,
        c: char,
        _context: ContextIterator,
        locale: CaseMapLocale,
    ) -> Option<FullMappingResult> {
        // TODO: In ICU4C, full upper/lower/title mappings decide whether to use
        // Turkic or non-Turkic mappings for dotless/dotted I based on locale,
        // but ucase_toFullFolding / ucase_fold use an options argument (with a single
        // defined bit). Can we somehow unify those two?
        let is_turkic = locale == CaseMapLocale::Turkish;
        match (c, is_turkic) {
            // Turkic mappings
            ('\u{49}', true) => Some(FullMappingResult::CodePoint('\u{131}')),
            ('\u{130}', true) => Some(FullMappingResult::CodePoint('\u{69}')),

            // Default mappings
            ('\u{49}', false) => Some(FullMappingResult::CodePoint('\u{69}')),
            ('\u{130}', false) => Some(FullMappingResult::string(Self::I_DOT)),
            (_, _) => None,
        }
    }

    /// Adds all simple case mappings and the full case folding for `c` to `set`.
    /// Also adds special case closure mappings.
    /// The character itself is not added.
    /// For example, the mappings
    /// - for s include long s
    /// - for sharp s include ss
    /// - for k include the Kelvin sign
    /// TODO: implement functions using this, and make this function private.
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
                set.add_string(Self::I_DOT);
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
                    let mapped = char::from_u32(codepoint as u32).expect("Checked in validate()");
                    set.add_char(mapped);
                }
            }
            return;
        }

        // c has exceptions, so there may be multiple simple and/or full case mappings.
        let idx = data.exception_index();

        // Add all simple case mappings.
        for slot in [
            ExceptionSlot::Lower,
            ExceptionSlot::Fold,
            ExceptionSlot::Upper,
            ExceptionSlot::Title,
        ] {
            if let Some(simple) = self.exceptions.slot_char(idx, slot) {
                set.add_char(simple);
            }
        }
        if self.exceptions.has_slot(idx, ExceptionSlot::Delta) {
            let codepoint = c as i32 + self.exceptions.delta(idx);
            let mapped = char::from_u32(codepoint as u32).expect("Checked in validate()");
            set.add_char(mapped);
        }

        self.exceptions.add_full_and_closure_mappings(idx, set);
    }

    /// Maps the string to single code points and adds the associated case closure
    /// mappings.
    /// The string is mapped to code points if it is their full case folding string.
    /// In other words, this performs a reverse full case folding and then
    /// adds the case closure items of the resulting code points.
    /// If the string is found and its closure applied, then
    /// the string itself is added as well as part of its code points' closure.
    ///
    /// Returns true if the string was found
    /// TODO: implement functions using this, and make this function private.
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
            }
            None => false,
        }
    }
}

/// TODO: hook this up to locid?
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum CaseMapLocale {
    Root,
    Turkish,
    Lithuanian,
    Greek,
    Dutch,
    Armenian,
}

pub(crate) enum FullMappingResult {
    Remove,
    CodePoint(char),
    String(String), // TODO: rewrite the exception data to have a sidetable of &str, so that this can be &str
}

// Temporary helper until FullMappingResult can use &str
impl FullMappingResult {
    fn string(s: &str) -> Self {
        Self::String(String::from(s))
    }
}

/// Interface for adding items to a set of chars + strings.
/// Eventually this may become UnicodeSet, but that can't currently hold strings.
pub trait SetAdder {
    /// Add a character to the set
    fn add_char(&mut self, c: char);
    /// Add a string to the set
    fn add_string(&mut self, string: &str);
}

pub(crate) struct ContextIterator<'a> {
    before: &'a str,
    after: &'a str,
}

impl<'a> ContextIterator<'a> {
    // Returns a context iterator with the characters before
    // and after the character at a given index.
    pub fn new(s: &'a str, idx: usize) -> Self {
        let (before, char_and_after) = s.split_at(idx);
        let after = &char_and_after[1..];
        Self { before, after }
    }

    fn preceded_by_soft_dotted(&self, mapping: &CaseMapping) -> bool {
        for c in self.before.chars().rev() {
            match mapping.dot_type(c) {
                DotType::SoftDotted => return true,
                DotType::OtherAccent => continue,
                _ => return false,
            }
        }
        false
    }
    fn preceded_by_capital_i(&self, mapping: &CaseMapping) -> bool {
        for c in self.before.chars().rev() {
            if c == 'I' {
                return true;
            }
            if mapping.dot_type(c) != DotType::OtherAccent {
                break;
            }
        }
        false
    }
    fn preceded_by_cased_letter(&self, mapping: &CaseMapping) -> bool {
        for c in self.before.chars().rev() {
            let data = mapping.lookup_data(c);
            if !data.is_ignorable() {
                return data.case_type() != CaseType::None;
            }
        }
        return false;
    }
    fn followed_by_cased_letter(&self, mapping: &CaseMapping) -> bool {
        for c in self.after.chars() {
            let data = mapping.lookup_data(c);
            if !data.is_ignorable() {
                return data.case_type() != CaseType::None;
            }
        }
        return false;
    }
    fn followed_by_more_above(&self, mapping: &CaseMapping) -> bool {
        for c in self.after.chars() {
            match mapping.dot_type(c) {
                DotType::Above => return true,
                DotType::OtherAccent => continue,
                _ => return false,
            }
        }
        false
    }
    fn followed_by_dot_above(&self, mapping: &CaseMapping) -> bool {
        for c in self.after.chars() {
            if c == '\u{307}' {
                return true;
            }
            if mapping.dot_type(c) != DotType::OtherAccent {
                return false;
            }
        }
        false
    }
}
