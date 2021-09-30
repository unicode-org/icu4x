// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::cmp::{Ord, PartialOrd};
use displaydoc::Display;

#[derive(Display, Debug, PartialEq)]
pub enum LengthError {
    #[displaydoc("Invalid length")]
    InvalidLength,
}

#[cfg(feature = "std")]
impl std::error::Error for LengthError {}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum FieldLength {
    One = 1,
    TwoDigit = 2,
    Abbreviated = 3,
    Wide = 4,
    Narrow = 5,
    Six = 6,
}

impl FieldLength {
    #[inline]
    pub(crate) fn idx_in_range(v: &u8) -> bool {
        (1..=6).contains(v)
    }

    #[inline]
    pub(crate) fn idx(&self) -> u8 {
        *self as usize as u8
    }

    #[inline]
    pub(crate) fn from_idx(idx: u8) -> Result<Self, LengthError> {
        Ok(match idx {
            1 => Self::One,
            2 => Self::TwoDigit,
            3 => Self::Abbreviated,
            4 => Self::Wide,
            5 => Self::Narrow,
            6 => Self::Six,
            _ => return Err(LengthError::InvalidLength),
        })
    }
}
