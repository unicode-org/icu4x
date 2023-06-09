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
    pub(crate) fn from_integer(int: u8) -> Self {
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
