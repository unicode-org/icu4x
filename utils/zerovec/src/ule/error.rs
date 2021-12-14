// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::any;
use core::fmt;

/// A generic error type to be used for decoding slices of ULE types
#[derive(Copy, Clone, Debug)]
pub enum ULEError {
    InvalidLength { ty: &'static str, len: usize },
    ParseError { ty: &'static str },
    FormatError,
}

impl fmt::Display for ULEError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            ULEError::InvalidLength { ty, len } => {
                write!(f, "Invalid length {} for slice of type {}", len, ty)
            }
            ULEError::ParseError { ty } => {
                write!(f, "Could not parse bytes to slice of type {}", ty)
            }
            ULEError::FormatError => {
                write!(f, "Invalid format for VarZeroVec buffer")
            }
        }
    }
}

impl ULEError {
    pub fn parse<T: ?Sized + 'static>() -> ULEError {
        ULEError::ParseError {
            ty: any::type_name::<T>(),
        }
    }

    pub fn length<T: ?Sized + 'static>(len: usize) -> ULEError {
        ULEError::InvalidLength {
            ty: any::type_name::<T>(),
            len,
        }
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for ULEError {}
