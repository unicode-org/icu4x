// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{yoke, zerofrom};
use zerovec::{VarZeroVec, ZeroVec};

use crate::error::Error;
use crate::internals::{ClosureSet, DotType, MappingKind};

// This represents case mapping exceptions that can't be represented as a delta applied to
// the original code point. Similar to ICU4C, data is stored as a u16 array. The codepoint
// trie in CaseMapping stores offsets into this array. The u16 at that index contains a
// set of flags describing the subsequent data.
//
//   [idx + 0]  Header word:
//       Bits:
//         0..7  Flag bits indicating which optional slots are present (if any):
//               0: Lowercase mapping (code point)
//               1: Case folding (code point)
//               2: Uppercase mapping (code point)
//               3: Titlecase mapping (code point)
//               4: Delta to simple case mapping (code point) (sign stored separately)
//               5: [RESERVED]
//               6: Closure mappings (string; see below)
//               7: Full mappings (strings; see below)
//            8  Double-width slots. If set, then each optional slot is stored as two
//               elements of the array (high and low halves of 32-bit values) instead of
//               a single element.
//            9  Has no simple case folding, even if there is a simple lowercase mapping
//           10  The value in the delta slot is negative
//           11  Is case-sensitive (not exposed)
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
// In ICU4C, the full mapping and closure strings are stored inline in the data, encoded
// as UTF-16, and the full mapping and closure slots contain information about the length
// of those strings. To avoid the need for allocations when converting from UTF-16 to
// UTF-8, we instead store strings encoded as UTF-8 in a side table. The full mapping and
// closure slots contain indices into that side table.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[derive(Debug, Eq, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
pub(crate) struct CaseMappingExceptions<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) slots: ZeroVec<'data, u16>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) strings: VarZeroVec<'data, str>,
}

#[derive(Copy, Clone)]
pub struct ExceptionHeader(pub u16);

impl ExceptionHeader {
    const SLOTS_MASK: u16 = 0xff;

    // Each slot is 2 u16 elements instead of 1
    const DOUBLE_SLOTS_FLAG: u16 = 0x100;

    const NO_SIMPLE_CASE_FOLDING_FLAG: u16 = 0x200;
    const DELTA_IS_NEGATIVE_FLAG: u16 = 0x400;
    const SENSITIVE_FLAG: u16 = 0x800;

    const DOT_SHIFT: u16 = 12;

    const CONDITIONAL_SPECIAL_FLAG: u16 = 0x4000;
    const CONDITIONAL_FOLD_FLAG: u16 = 0x8000;

    // The number of optional slots for this exception
    fn num_slots(&self) -> u16 {
        let slot_bits = self.0 & Self::SLOTS_MASK;
        slot_bits.count_ones() as u16
    }

    // Returns true if the given slot exists for this exception
    pub(crate) fn has_slot(&self, slot: ExceptionSlot) -> bool {
        let bit = 1u16 << (slot as u16);
        self.0 & bit != 0
    }

    // Returns the number of slots between this header and the given slot.
    fn slot_offset(&self, slot: ExceptionSlot) -> usize {
        debug_assert!(self.has_slot(slot));
        let slot_bit = 1u16 << (slot as u16);
        let previous_slot_mask = slot_bit - 1;
        let previous_slots = self.0 & previous_slot_mask;
        let slot_num = previous_slots.count_ones() as usize;

        if self.has_double_slots() {
            slot_num * 2
        } else {
            slot_num
        }
    }

    // Returns whether this exception has double-width slots
    pub fn has_double_slots(&self) -> bool {
        self.0 & Self::DOUBLE_SLOTS_FLAG != 0
    }

    // Returns true if there is no simple case folding for this exception
    fn no_simple_case_folding(&self) -> bool {
        self.0 & Self::NO_SIMPLE_CASE_FOLDING_FLAG != 0
    }

    // Returns true if the delta for this exception is negative.
    fn delta_is_negative(&self) -> bool {
        debug_assert!(self.has_slot(ExceptionSlot::Delta));
        self.0 & Self::DELTA_IS_NEGATIVE_FLAG != 0
    }

    // Returns whether this code point is case-sensitive.
    // (Note that this information is stored in the trie for code points without
    // exception data, but the exception index requires more bits than the delta.)
    fn is_sensitive(&self) -> bool {
        self.0 & Self::SENSITIVE_FLAG != 0
    }

    // Returns the dot type.
    // (Note that this information is stored in the trie for code points without
    // exception data, but the exception index requires more bits than the delta.)
    fn dot_type(&self) -> DotType {
        let masked_bits = (self.0 >> Self::DOT_SHIFT) & DotType::DOT_MASK;
        DotType::from_masked_bits(masked_bits)
    }

    // Returns whether there is a language-specific case mapping.
    fn has_conditional_special(&self) -> bool {
        self.0 & Self::CONDITIONAL_SPECIAL_FLAG != 0
    }

    // Returns whether there is a conditional case fold.
    // (This is used for Turkic mappings for dotted/dotless i.)
    fn has_conditional_fold(&self) -> bool {
        self.0 & Self::CONDITIONAL_FOLD_FLAG != 0
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ExceptionSlot {
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

impl<'data> CaseMappingExceptions<'data> {
    // Returns the array element at the given index
    #[inline]
    fn get(&self, idx: usize) -> u16 {
        self.slots.get(idx).expect("Checked in validate()")
    }

    // Returns the header for an entry.
    #[inline]
    fn header(&self, hdr_idx: u16) -> ExceptionHeader {
        ExceptionHeader(self.get(hdr_idx as usize))
    }

    // Returns the number of optional slots for an entry.
    fn num_slots(&self, hdr_idx: u16) -> u16 {
        self.header(hdr_idx).num_slots()
    }

    // Returns whether the given slot exists for an entry.
    #[inline]
    pub fn has_slot(&self, hdr_idx: u16, slot: ExceptionSlot) -> bool {
        self.header(hdr_idx).has_slot(slot)
    }

    // Returns the index of a given slot for an entry.
    #[inline]
    fn slot_index(&self, hdr_idx: u16, slot: ExceptionSlot) -> usize {
        hdr_idx as usize + 1 + self.header(hdr_idx).slot_offset(slot)
    }

    // Returns the value stored in a given slot for an entry.
    #[inline]
    fn slot_value(&self, hdr_idx: u16, slot: ExceptionSlot) -> u32 {
        debug_assert!(self.has_slot(hdr_idx, slot));
        let slot_idx = self.slot_index(hdr_idx, slot);
        if self.has_double_slots(hdr_idx) {
            let hi = self.get(slot_idx) as u32;
            let lo = self.get(slot_idx + 1) as u32;
            hi << 16 | lo
        } else {
            self.get(slot_idx) as u32
        }
    }

    // Returns the character stored in a given slot for an entry.
    #[inline]
    pub fn slot_char(&self, hdr_idx: u16, slot: ExceptionSlot) -> Option<char> {
        debug_assert!(slot.contains_char());
        if self.has_slot(hdr_idx, slot) {
            let raw = self.slot_value(hdr_idx, slot);
            Some(char::from_u32(raw).expect("Checked in validate() (step #1)"))
        } else {
            None
        }
    }

    // Given a mapping kind, returns the character for that kind, if it exists. Fold falls
    // back to Lower; Title falls back to Upper.
    #[inline]
    pub fn slot_char_for_kind(&self, hdr_idx: u16, kind: MappingKind) -> Option<char> {
        match kind {
            MappingKind::Lower | MappingKind::Upper => self.slot_char(hdr_idx, kind.into()),
            MappingKind::Fold => self
                .slot_char(hdr_idx, ExceptionSlot::Fold)
                .or_else(|| self.slot_char(hdr_idx, ExceptionSlot::Lower)),
            MappingKind::Title => self
                .slot_char(hdr_idx, ExceptionSlot::Title)
                .or_else(|| self.slot_char(hdr_idx, ExceptionSlot::Upper)),
        }
    }

    // Returns whether an entry has a delta slot.
    #[inline]
    pub fn has_delta(&self, hdr_idx: u16) -> bool {
        self.header(hdr_idx).has_slot(ExceptionSlot::Delta)
    }

    // Returns the delta value for an entry (with the correct sign).
    #[inline]
    pub fn delta(&self, hdr_idx: u16) -> i32 {
        debug_assert!(self.has_delta(hdr_idx));
        let raw: i32 = self.slot_value(hdr_idx, ExceptionSlot::Delta) as _;
        if self.header(hdr_idx).delta_is_negative() {
            -raw
        } else {
            raw
        }
    }

    // Returns whether an entry has double-width slots.
    #[inline]
    fn has_double_slots(&self, hdr_idx: u16) -> bool {
        self.header(hdr_idx).has_double_slots()
    }

    // Returns whether there is no simple case folding for an entry.
    #[inline]
    pub fn no_simple_case_folding(&self, hdr_idx: u16) -> bool {
        self.header(hdr_idx).no_simple_case_folding()
    }

    // Returns whether this code point is case-sensitive.
    // (Note that this information is stored in the trie for code points without
    // exception data, but the exception index requires more bits than the delta.)
    pub fn is_sensitive(&self, hdr_idx: u16) -> bool {
        self.header(hdr_idx).is_sensitive()
    }

    // Returns whether there is a conditional case fold for this entry.
    // (This is used to implement Turkic mappings for dotted/dotless i.)
    pub fn has_conditional_fold(&self, hdr_idx: u16) -> bool {
        self.header(hdr_idx).has_conditional_fold()
    }

    // Given a header index, returns whether there is a language-specific case mapping.
    pub fn has_conditional_special(&self, hdr_idx: u16) -> bool {
        self.header(hdr_idx).has_conditional_special()
    }

    // Given a header index, returns the dot type.
    // (Note that this information is stored in the trie for code points without
    // exception data, but the exception index requires more bits than the delta.)
    pub fn dot_type(&self, hdr_idx: u16) -> DotType {
        self.header(hdr_idx).dot_type()
    }

    // Given a header index and a mapping kind, returns the full mapping string.
    // Note that the string may be empty.
    pub fn full_mapping_string(&self, hdr_idx: u16, slot: MappingKind) -> &str {
        debug_assert!(self.has_slot(hdr_idx, ExceptionSlot::FullMappings));
        let mappings_idx = self.slot_value(hdr_idx, ExceptionSlot::FullMappings);
        let idx = mappings_idx as usize + slot as usize;
        &self.strings[idx]
    }

    // Given a header index, returns the closure string.
    fn closure_string(&self, hdr_idx: u16) -> &str {
        debug_assert!(self.has_slot(hdr_idx, ExceptionSlot::Closure));
        let closure_idx = self.slot_value(hdr_idx, ExceptionSlot::Closure) as usize;
        &self.strings[closure_idx]
    }

    pub fn add_full_and_closure_mappings<S: ClosureSet>(&self, hdr_idx: u16, set: &mut S) {
        if self.has_slot(hdr_idx, ExceptionSlot::FullMappings) {
            let mapping_string = self.full_mapping_string(hdr_idx, MappingKind::Fold);
            if !mapping_string.is_empty() {
                set.add_string(mapping_string);
            }
        };

        if self.has_slot(hdr_idx, ExceptionSlot::Closure) {
            for c in self.closure_string(hdr_idx).chars() {
                set.add_char(c);
            }
        };
    }

    pub fn validate(&self) -> Result<Vec<u16>, Error> {
        let mut valid_indices = vec![];
        let mut idx: u16 = 0;

        let data_len = self.slots.len() as u16;

        while idx < data_len {
            let mut slot_len = self.num_slots(idx);
            if self.has_double_slots(idx) {
                slot_len *= 2;
            }

            // Validate that we can read slot data without reading out of bounds.
            let next_idx = idx + 1 + slot_len;
            if next_idx > data_len {
                return Error::invalid("Exceptions: missing slot data");
            }

            // Validate slots that should contain chars.
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

            // Validate slots that should contain string indices.
            if self.has_slot(idx, ExceptionSlot::Closure) {
                let idx = self.slot_value(idx, ExceptionSlot::Closure);
                if idx as usize >= self.strings.len() {
                    return Error::invalid("Exceptions: closure index out of bounds");
                }
            }
            if self.has_slot(idx, ExceptionSlot::FullMappings) {
                let idx = self.slot_value(idx, ExceptionSlot::FullMappings);
                let max_idx = idx as usize + MappingKind::Title as usize;
                if max_idx >= self.strings.len() {
                    return Error::invalid("Exceptions: full mapping index out of bounds");
                }
            }

            valid_indices.push(idx);
            idx = next_idx;
        }
        Ok(valid_indices)
    }
}
