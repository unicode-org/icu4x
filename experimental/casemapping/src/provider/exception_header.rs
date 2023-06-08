// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains various types for the header part of casemapping exception data
//!
//! This is both used in datagen to decode ICU4C's data, and natively in ICU4X's
//! own data model.
//!
//! [`ExceptionHeader`] is the core type common to both backends: it represents
//! a metadata header with a bunch of bits ([`ExceptionBits`]) as well as information
//! on slots used ([`SlotPresence`]).
//!
//! The `exceptions_builder` module of this crate handles decoding ICU4C data using the exception
//! header.

use crate::provider::data::{DotType, MappingKind};
use zerovec::ule::{AsULE, ULE};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
pub struct ExceptionBits {
    pub double_width_slots: bool,
    pub no_simple_case_folding: bool,
    pub negative_delta: bool,
    pub is_sensitive: bool,
    pub dot_type: DotType,
    pub has_conditional_special: bool,
    pub has_conditional_fold: bool,
}

impl ExceptionBits {
    /// Extract from the upper half of an ICU4C-format u16
    fn from_integer(int: u8) -> Self {
        let ule = ExceptionBitsULE(int);
        let double_width_slots = ule.double_width_slots();
        let no_simple_case_folding = ule.no_simple_case_folding();
        let negative_delta = ule.negative_delta();
        let is_sensitive = ule.is_sensitive();
        let has_conditional_special = ule.has_conditional_special();
        let has_conditional_fold = ule.has_conditional_fold();
        let dot_type = ule.dot_type();

        Self {
            double_width_slots,
            no_simple_case_folding,
            negative_delta,
            is_sensitive,
            dot_type,
            has_conditional_special,
            has_conditional_fold,
        }
    }

    /// Convert to an ICU4C-format upper half of u16
    pub(crate) fn to_integer(self) -> u8 {
        let mut int = 0;
        let dot_data = (self.dot_type as u8) << ExceptionBitsULE::DOT_SHIFT;
        int |= dot_data;

        if self.double_width_slots {
            int |= ExceptionBitsULE::DOUBLE_SLOTS_FLAG
        }
        if self.no_simple_case_folding {
            int |= ExceptionBitsULE::NO_SIMPLE_CASE_FOLDING_FLAG
        }
        if self.negative_delta {
            int |= ExceptionBitsULE::NEGATIVE_DELTA_FLAG
        }
        if self.is_sensitive {
            int |= ExceptionBitsULE::SENSITIVE_FLAG
        }
        if self.has_conditional_special {
            int |= ExceptionBitsULE::CONDITIONAL_SPECIAL_FLAG
        }
        if self.has_conditional_fold {
            int |= ExceptionBitsULE::CONDITIONAL_FOLD_FLAG
        }
        int
    }
}

/// Packed slot presence marker
///
/// All bits are valid, though bit 4 is unused and reserved
///
/// Bits:
///
/// ```text
///               0: Lowercase mapping (code point)
///               1: Case folding (code point)
///               2: Uppercase mapping (code point)
///               3: Titlecase mapping (code point)
///               4: Delta to simple case mapping (code point) (sign stored separately)
///               5: RESERVED
///               6: Closure mappings (string; see below)
///               7: Full mappings (strings; see below)
/// ```
#[derive(Copy, Clone, PartialEq, Eq, ULE, Debug, Default)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
pub struct SlotPresence(pub u8);

impl SlotPresence {
    pub(crate) fn add_slot(&mut self, slot: ExceptionSlot) {
        self.0 |= 1 << slot as u8;
    }
    pub(crate) fn has_slot(self, slot: ExceptionSlot) -> bool {
        let bit = 1 << (slot as u8);
        self.0 & bit != 0
    }
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ExceptionHeader {
    /// The various slots that are present, masked by ExceptionSlot
    ///
    /// We still store this as a bitmask since it's more convenient to access as one
    pub slot_presence: SlotPresence,
    pub bits: ExceptionBits,
}

impl ExceptionHeader {
    /// Construct from an ICU4C-format u16.
    pub(crate) fn from_integer(int: u16) -> Self {
        let slot_presence =
            SlotPresence(u8::try_from(int & ExceptionHeaderULE::SLOTS_MASK).unwrap_or(0));
        let bits = ExceptionBits::from_integer(
            u8::try_from(int >> ExceptionHeaderULE::BITS_SHIFT).unwrap_or(0),
        );
        Self {
            slot_presence,
            bits,
        }
    }

    /// Convert to an ICU4C-format u16
    #[cfg(feature = "datagen")]
    pub(crate) fn to_integer(self) -> u16 {
        let mut sixteen: u16 = self.slot_presence.0.into();
        sixteen |= (u16::from(self.bits.to_integer())) << ExceptionHeaderULE::BITS_SHIFT;
        sixteen
    }

    // The number of optional slots for this exception
    pub(crate) fn num_slots(&self) -> u16 {
        self.slot_presence.0.count_ones() as u16
    }

    // Returns true if the given slot exists for this exception
    pub(crate) fn has_slot(&self, slot: ExceptionSlot) -> bool {
        self.slot_presence.has_slot(slot)
    }

    // Returns the number of slots between this header and the given slot.
    pub(crate) fn slot_offset(&self, slot: ExceptionSlot) -> usize {
        debug_assert!(self.has_slot(slot));
        let slot_bit = 1 << (slot as u8);
        let previous_slot_mask = slot_bit - 1;
        let previous_slots = self.slot_presence.0 & previous_slot_mask;
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
#[repr(packed)]
pub struct ExceptionHeaderULE {
    slot_presence: SlotPresence,
    bits: ExceptionBitsULE,
}

/// The bitflags on an exception header (bits 8-15, see docs on [`ExceptionHeaderULE`])
///
/// All bits are valid, though in ICU4X data bits 0 and 2 are not used
#[derive(Copy, Clone, PartialEq, Eq, ULE)]
#[repr(transparent)]
pub struct ExceptionBitsULE(pub u8);

impl ExceptionBitsULE {
    const DOUBLE_SLOTS_FLAG: u8 = 0x1;

    const NO_SIMPLE_CASE_FOLDING_FLAG: u8 = 0x2;
    const NEGATIVE_DELTA_FLAG: u8 = 0x4;
    const SENSITIVE_FLAG: u8 = 0x8;

    const DOT_SHIFT: u8 = 4;

    const CONDITIONAL_SPECIAL_FLAG: u8 = 0x40;
    const CONDITIONAL_FOLD_FLAG: u8 = 0x80;
}

impl ExceptionBitsULE {
    pub fn double_width_slots(self) -> bool {
        self.0 & Self::DOUBLE_SLOTS_FLAG != 0
    }
    pub fn no_simple_case_folding(self) -> bool {
        self.0 & Self::NO_SIMPLE_CASE_FOLDING_FLAG != 0
    }
    pub fn negative_delta(self) -> bool {
        self.0 & Self::NEGATIVE_DELTA_FLAG != 0
    }
    pub fn is_sensitive(self) -> bool {
        self.0 & Self::SENSITIVE_FLAG != 0
    }
    pub fn has_conditional_special(self) -> bool {
        self.0 & Self::CONDITIONAL_SPECIAL_FLAG != 0
    }
    pub fn has_conditional_fold(self) -> bool {
        self.0 & Self::CONDITIONAL_FOLD_FLAG != 0
    }
    pub fn dot_type(self) -> DotType {
        DotType::from_masked_bits((u16::from(self.0 >> Self::DOT_SHIFT)) & DotType::DOT_MASK)
    }
}
impl ExceptionHeaderULE {
    const SLOTS_MASK: u16 = 0xff;
    const BITS_SHIFT: u16 = 8;
}

impl AsULE for ExceptionHeader {
    type ULE = ExceptionHeaderULE;
    fn from_unaligned(u: ExceptionHeaderULE) -> Self {
        Self {
            slot_presence: u.slot_presence,
            bits: ExceptionBits::from_integer(u.bits.0),
        }
    }

    fn to_unaligned(self) -> ExceptionHeaderULE {
        ExceptionHeaderULE {
            slot_presence: self.slot_presence,
            bits: ExceptionBitsULE(self.bits.to_integer()),
        }
    }
}

impl AsULE for ExceptionBits {
    type ULE = ExceptionBitsULE;
    fn from_unaligned(u: ExceptionBitsULE) -> Self {
        ExceptionBits::from_integer(u.0)
    }

    fn to_unaligned(self) -> ExceptionBitsULE {
        ExceptionBitsULE(self.to_integer())
    }
}

impl AsULE for SlotPresence {
    type ULE = SlotPresence;
    fn from_unaligned(u: Self) -> Self {
        u
    }

    fn to_unaligned(self) -> Self {
        self
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) enum ExceptionSlot {
    Lower = 0,
    Fold = 1,
    Upper = 2,
    Title = 3,
    /// Also the Simple slot (todo: rename this)
    Delta = 4,
    // Slot 5 is reserved
    Closure = 6,
    FullMappings = 7,
}

impl ExceptionSlot {
    /// Where the string slots begin
    pub(crate) const STRING_SLOTS_START: Self = Self::Closure;
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
