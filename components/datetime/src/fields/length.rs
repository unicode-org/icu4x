// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::cmp::{Ord, PartialOrd};
use core::fmt;
use core::str::FromStr;
use displaydoc::Display;
use zerovec::ule::{AsULE, ZeroVecError, ULE};

#[cfg(doc)]
use crate::fields::{Hour, Weekday};

/// An error relating to the length of a field within a date pattern.
#[derive(Display, Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum LengthError {
    /// The length of the field string within the pattern is invalid, according to
    /// the field type and its supported field patterns in LDML. See [`FieldLength`].
    #[displaydoc("Invalid length")]
    InvalidLength,
}

#[cfg(feature = "std")]
impl std::error::Error for LengthError {}

/// An enum representing the length of a field within a date or time formatting pattern string,
/// in which the pattern field is represented as a letter occurring 1 or more times in a row, ex:
/// `MMM`, `dd`, `y`.  See the
/// [LDML documentation in UTS 35](https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns)
/// for more details.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::fields),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_enums)] // part of data struct
pub enum FieldLength {
    /// Typical style is 1-2 digits. For numeric-only fields.
    One,
    /// Typical style is 2 digits. For numeric-only fields.
    TwoDigit,
    /// Abbreviated (spellout) format.
    Abbreviated,
    /// Wide / Long / Full  (spellout) format.
    Wide,
    /// Narrow / Long / Full  (spellout) format.
    Narrow,
    /// Meaning is field-dependent, for patterns that are 6 characters long. Ex: a [`Weekday`] pattern like
    /// `EEEEEE` means "Short", but `jjjjjj` or `CCCCCC` for [`Hour`] may mean
    /// "Numeric hour (2 digits, zero pad if needed), narrow dayPeriod if used". See the
    /// [LDML documentation in UTS 35](https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns)
    /// for more details.
    Six,
    /// A fixed size format for numeric-only fields that is at most 127 digits.
    Fixed(u8),
    /// FieldLength::One (numeric), but overridden with a different numbering system
    NumericOverride(FieldNumericOverrides),
}

impl FieldLength {
    #[inline]
    pub(crate) fn idx(&self) -> u8 {
        match self {
            FieldLength::One => 1,
            FieldLength::TwoDigit => 2,
            FieldLength::Abbreviated => 3,
            FieldLength::Wide => 4,
            FieldLength::Narrow => 5,
            FieldLength::Six => 6,
            FieldLength::NumericOverride(o) => 64 + (*o as u8).min(63),
            FieldLength::Fixed(p) => 128 + p.min(&127), /* truncate to at most 127 digits to avoid overflow */
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
                if idx < 64 {
                    return Err(LengthError::InvalidLength);
                }
                if idx < 128 {
                    Self::NumericOverride((idx - 64).try_into()?)
                } else {
                    Self::Fixed(idx - 128)
                }
            }
        })
    }

    #[inline]
    #[cfg(feature = "datagen")]
    pub(crate) fn to_len(self) -> usize {
        match self {
            FieldLength::One => 1,
            FieldLength::TwoDigit => 2,
            FieldLength::Abbreviated => 3,
            FieldLength::Wide => 4,
            FieldLength::Narrow => 5,
            FieldLength::Six => 6,
            FieldLength::NumericOverride(o) => 64 + o as usize,
            FieldLength::Fixed(p) => p as usize,
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FieldLengthULE(u8);

impl AsULE for FieldLength {
    type ULE = FieldLengthULE;
    fn to_unaligned(self) -> Self::ULE {
        FieldLengthULE(self.idx())
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        #[allow(clippy::unwrap_used)] // OK because the ULE is pre-validated
        Self::from_idx(unaligned.0).unwrap()
    }
}

impl FieldLengthULE {
    #[inline]
    pub(crate) fn validate_byte(byte: u8) -> Result<(), ZeroVecError> {
        FieldLength::from_idx(byte)
            .map(|_| ())
            .map_err(|_| ZeroVecError::parse::<FieldLength>())
    }
}

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_byte_slice() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_byte_slice() checks that the given byte slice has a valid length
//    (true since transparent over a type of size 1).
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
unsafe impl ULE for FieldLengthULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        for byte in bytes {
            Self::validate_byte(*byte)?;
        }
        Ok(())
    }
}

/// Various numeric overrides for datetime patterns
/// as found in CLDR
#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::fields),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum FieldNumericOverrides {
    /// `hanidec`
    Hanidec = 0,
    /// `hanidays`
    Hanidays = 1,
    /// `hebr`
    Hebr = 2,
    /// `romanlow`
    Romanlow = 3,
    /// `jpnyear`
    Jpnyear = 4,
}

impl TryFrom<u8> for FieldNumericOverrides {
    type Error = LengthError;
    fn try_from(other: u8) -> Result<Self, LengthError> {
        Ok(match other {
            0 => Self::Hanidec,
            1 => Self::Hanidays,
            2 => Self::Hebr,
            3 => Self::Romanlow,
            4 => Self::Jpnyear,
            _ => return Err(LengthError::InvalidLength),
        })
    }
}

impl FromStr for FieldNumericOverrides {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
            "hanidec" => Self::Hanidec,
            "d=hanidays" => Self::Hanidays,
            "hebr" => Self::Hebr,
            "M=romanlow" => Self::Romanlow,
            "y=jpnyear" => Self::Jpnyear,
            _ => return Err(()),
        })
    }
}

impl FieldNumericOverrides {
    /// Conver this to the corresponding string code
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hanidec => "hanidec",
            Self::Hanidays => "hanidays",
            Self::Hebr => "hebr",
            Self::Romanlow => "romanlow",
            Self::Jpnyear => "jpnyear",
        }
    }
}

impl fmt::Display for FieldNumericOverrides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
