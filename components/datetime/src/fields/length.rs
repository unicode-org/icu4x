// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::cmp::{Ord, PartialOrd};
use displaydoc::Display;

#[derive(Display, Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum LengthError {
    #[displaydoc("Invalid length")]
    InvalidLength,
}

#[cfg(feature = "std")]
impl std::error::Error for LengthError {}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FieldLength {
    One,
    TwoDigit,
    Abbreviated,
    Wide,
    Narrow,
    Six,
    Fixed(u8),
}

impl FieldLength {
    #[inline]
    pub(crate) fn idx_in_range(v: &u8) -> bool {
        (1..=6).contains(v) || *v >= 128
    }

    #[inline]
    pub(crate) fn idx(&self) -> u8 {
        match self {
            FieldLength::One => 1,
            FieldLength::TwoDigit => 2,
            FieldLength::Abbreviated => 3,
            FieldLength::Wide => 4,
            FieldLength::Narrow => 5,
            FieldLength::Six => 6,
            FieldLength::Fixed(p) => 128 + p.min(&127), // truncate to at most 127 digits to avoid overflow
        }
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
            idx => {
                if idx < 128 {
                    return Err(LengthError::InvalidLength);
                }
                Self::Fixed(idx - 128)
            }
        })
    }

    #[inline]
    pub(crate) fn to_len(self) -> usize {
        match self {
            FieldLength::One => 1,
            FieldLength::TwoDigit => 2,
            FieldLength::Abbreviated => 3,
            FieldLength::Wide => 4,
            FieldLength::Narrow => 5,
            FieldLength::Six => 6,
            FieldLength::Fixed(p) => p as usize,
        }
    }
}
