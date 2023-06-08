// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::data::{DotType, MappingKind};
use zerovec::ule::{AsULE, RawBytesULE, ULE};

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct ExceptionBits {
    pub double_width_slots: bool,
    pub no_simple_case_folding: bool,
    pub negative_delta: bool,
    pub is_sensitive: bool,
    pub dot_type: DotType,
    pub has_conditional_special: bool,
    pub has_conditional_fold: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct ExceptionHeader {
    /// The various slots that are present, masked by ExceptionSlot
    ///
    /// We still store this as a bitmask since it's more convenient to access as one
    pub slot_presence: u8,
    pub bits: ExceptionBits,
}

impl ExceptionHeader {
    /// Construct from an ICU4C-format u16.
    pub(crate) fn from_integer(int: u16) -> Self {
        let slot_presence = u8::try_from(int & ExceptionHeaderULE::SLOTS_MASK).unwrap_or(0);
        let double_width_slots = int & ExceptionHeaderULE::DOUBLE_SLOTS_FLAG != 0;
        let no_simple_case_folding = int & ExceptionHeaderULE::NO_SIMPLE_CASE_FOLDING_FLAG != 0;
        let negative_delta = int & ExceptionHeaderULE::NEGATIVE_DELTA_FLAG != 0;
        let is_sensitive = int & ExceptionHeaderULE::SENSITIVE_FLAG != 0;
        let has_conditional_special = int & ExceptionHeaderULE::CONDITIONAL_SPECIAL_FLAG != 0;
        let has_conditional_fold = int & ExceptionHeaderULE::CONDITIONAL_FOLD_FLAG != 0;

        let dot_type =
            DotType::from_masked_bits((int >> ExceptionHeaderULE::DOT_SHIFT) & DotType::DOT_MASK);
        Self {
            slot_presence,
            bits: ExceptionBits {
                double_width_slots,
                no_simple_case_folding,
                negative_delta,
                is_sensitive,
                dot_type,
                has_conditional_special,
                has_conditional_fold,
            },
        }
    }

    /// Convert to an ICU4C-format u16
    pub(crate) fn to_integer(self) -> u16 {
        let mut sixteen: u16 = self.slot_presence.into();
        let dot_data = (self.bits.dot_type as u16) << ExceptionHeaderULE::DOT_SHIFT;
        sixteen |= dot_data;

        if self.bits.double_width_slots {
            sixteen |= ExceptionHeaderULE::DOUBLE_SLOTS_FLAG
        }
        if self.bits.no_simple_case_folding {
            sixteen |= ExceptionHeaderULE::NO_SIMPLE_CASE_FOLDING_FLAG
        }
        if self.bits.negative_delta {
            sixteen |= ExceptionHeaderULE::NEGATIVE_DELTA_FLAG
        }
        if self.bits.is_sensitive {
            sixteen |= ExceptionHeaderULE::SENSITIVE_FLAG
        }
        if self.bits.has_conditional_special {
            sixteen |= ExceptionHeaderULE::CONDITIONAL_SPECIAL_FLAG
        }
        if self.bits.has_conditional_fold {
            sixteen |= ExceptionHeaderULE::CONDITIONAL_FOLD_FLAG
        }
        sixteen
    }

    // The number of optional slots for this exception
    pub(crate) fn num_slots(&self) -> u16 {
        self.slot_presence.count_ones() as u16
    }

    // Returns true if the given slot exists for this exception
    pub(crate) fn has_slot(&self, slot: ExceptionSlot) -> bool {
        let bit = 1 << (slot as u8);
        self.slot_presence & bit != 0
    }

    // Returns the number of slots between this header and the given slot.
    pub(crate) fn slot_offset(&self, slot: ExceptionSlot) -> usize {
        debug_assert!(self.has_slot(slot));
        let slot_bit = 1 << (slot as u8);
        let previous_slot_mask = slot_bit - 1;
        let previous_slots = self.slot_presence & previous_slot_mask;
        let slot_num = previous_slots.count_ones() as usize;

        if self.bits.double_width_slots {
            slot_num * 2
        } else {
            slot_num
        }
    }
}

/// Packed exception header (format from icu4c, documented in casepropsbuilder.cpp)
///
/// ```text
///       Bits:
///         0..7  Flag bits indicating which optional slots are present (if any):
///               0: Lowercase mapping (code point)
///               1: Case folding (code point)
///               2: Uppercase mapping (code point)
///               3: Titlecase mapping (code point)
///               4: Delta to simple case mapping (code point) (sign stored separately)
///               5: RESERVED
///               6: Closure mappings (string; see below)
///               7: Full mappings (strings; see below)
///            8  Double-width slots. If set, then each optional slot is stored as two
///               elements of the array (high and low halves of 32-bit values) instead of
///               a single element.
///            9  Has no simple case folding, even if there is a simple lowercase mapping
///           10  The value in the delta slot is negative
///           11  Is case-sensitive (not exposed)
///       12..13  Dot type
///           14  Has conditional special casing
///           15  Has conditional case folding
/// ```
///
/// In this struct the RESERVED bit is still allowed to be set, and it will produce a different
/// exception header, but it will not have any other effects.
#[derive(Copy, Clone, PartialEq, Eq, ULE)]
#[repr(transparent)]
pub(crate) struct ExceptionHeaderULE(pub(crate) RawBytesULE<2>);

impl ExceptionHeaderULE {
    const SLOTS_MASK: u16 = 0xff;

    // Each slot is 2 u16 elements instead of 1
    const DOUBLE_SLOTS_FLAG: u16 = 0x100;

    const NO_SIMPLE_CASE_FOLDING_FLAG: u16 = 0x200;
    const NEGATIVE_DELTA_FLAG: u16 = 0x400;
    const SENSITIVE_FLAG: u16 = 0x800;

    const DOT_SHIFT: u16 = 12;

    const CONDITIONAL_SPECIAL_FLAG: u16 = 0x4000;
    const CONDITIONAL_FOLD_FLAG: u16 = 0x8000;
}

impl AsULE for ExceptionHeader {
    type ULE = ExceptionHeaderULE;
    fn from_unaligned(u: ExceptionHeaderULE) -> Self {
        Self::from_integer(u.0.as_unsigned_int())
    }

    fn to_unaligned(self) -> ExceptionHeaderULE {
        ExceptionHeaderULE(self.to_integer().to_unaligned())
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
    pub(crate) fn contains_char(&self) -> bool {
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
