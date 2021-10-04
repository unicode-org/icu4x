// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

/// A generic error type to be used for decoding slices of ULE types
#[derive(Copy, Clone, Debug)]
pub enum ULEError<E> {
    InvalidLength { ty: &'static str, len: usize },
    ParseError(E),
}

impl<E: fmt::Display> fmt::Display for ULEError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            ULEError::InvalidLength { ty, len } => {
                write!(f, "Invalid length {} for slice of type {}", len, ty)
            }
            ULEError::ParseError(ref e) => e.fmt(f),
        }
    }
}

impl<E> From<E> for ULEError<E> {
    fn from(e: E) -> Self {
        ULEError::ParseError(e)
    }
}

#[cfg(feature = "std")]
impl<E: fmt::Display + fmt::Debug> ::std::error::Error for ULEError<E> {}
