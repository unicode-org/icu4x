// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;

/// Starts at 1 for niche optimization of `Result<(), VarZeroVecFormatError>`
#[derive(Debug)]
#[repr(u8)]
pub enum VarZeroVecFormatError {
    /// The byte buffer was not in the appropriate format for [`VarZeroVec`](crate::VarZeroVec).
    Metadata = 1,
    /// One of the values could not be decoded.
    Values(crate::ule::UleError),
}

// Ensure niche optimization is working
const _: () =
    assert!(size_of::<Result<(), VarZeroVecFormatError>>() == size_of::<VarZeroVecFormatError>());

impl core::error::Error for VarZeroVecFormatError {}

impl Display for VarZeroVecFormatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Metadata => write!(f, "VarZeroVecFormatError: metadata"),
            Self::Values(e) => write!(f, "VarZeroVecFormatError: {e}"),
        }
    }
}
