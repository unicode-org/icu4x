// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields;
use displaydoc::Display;

/// These strings follow the recommendations for the serde::de::Unexpected::Other type.
/// <https://docs.serde.rs/serde/de/enum.Unexpected.html#variant.Other>
///
/// Serde will generate an error such as:
/// "invalid value: unclosed literal in pattern, expected a valid UTS 35 pattern string at line 1 column 12"
#[derive(Display, Debug)]
pub enum SkeletonError {
    #[displaydoc("field too long in skeleton")]
    InvalidFieldLength,
    #[displaydoc("duplicate field in skeleton")]
    DuplicateField,
    #[displaydoc("symbol unknown {0} in skeleton")]
    SymbolUnknown(char),
    #[displaydoc("symbol invalid {0} in skeleton")]
    SymbolInvalid(u8),
    #[displaydoc("symbol unimplemented {0} in skeleton")]
    SymbolUnimplemented(char),
    #[displaydoc("unimplemented field {0} in skeleton")]
    UnimplementedField(char),
    #[displaydoc("{0}")]
    Fields(fields::Error),
}

#[cfg(feature = "std")]
impl std::error::Error for SkeletonError {}

impl From<fields::Error> for SkeletonError {
    fn from(e: fields::Error) -> Self {
        SkeletonError::Fields(e)
    }
}

impl From<fields::LengthError> for SkeletonError {
    fn from(_: fields::LengthError) -> Self {
        Self::InvalidFieldLength
    }
}

impl From<fields::SymbolError> for SkeletonError {
    fn from(symbol_error: fields::SymbolError) -> Self {
        match symbol_error {
            fields::SymbolError::Invalid(ch) => Self::SymbolInvalid(ch),
            fields::SymbolError::InvalidIndex(_) => unimplemented!(),
            fields::SymbolError::Unknown(ch) => {
                // NOTE: If you remove a symbol due to it now being supported,
                //       make sure to regenerate the test data.
                //       https://github.com/unicode-org/icu4x/blob/main/provider/testdata/README.md
                match ch {
                    // TODO(#487) - Flexible day periods
                    'B'
                    // TODO(#486) - Era
                    | 'G'
                    // TODO(#502) - Week of month
                    | 'W'
                    // TODO(#501) - Quarters
                    | 'Q'
                    // TODO (#488) - Week of year
                    | 'w'
                    => Self::SymbolUnimplemented(ch),
                    _ => Self::SymbolUnknown(ch),
                }
            }
        }
    }
}
