// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Error types for decimal formatting.

use displaydoc::Display;

/// A list of error outcomes for various operations in this module.
///
// TODO: should this be [`DimensionError`]
/// Re-exported as [`Error`](crate::dimension::error::DimensionError).
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum DimensionError {
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("error loading data: {0}")]
    Data(icu_provider::DataError),

    /// An error originating inside of the fixed decimal formatter.
    #[displaydoc("error loading fixed decimal formatter: {0}")]
    Decimal(icu_decimal::DecimalError),
}

#[cfg(feature = "std")]
impl std::error::Error for DimensionError {}

impl From<icu_provider::DataError> for DimensionError {
    fn from(e: icu_provider::DataError) -> Self {
        DimensionError::Data(e)
    }
}

impl From<icu_decimal::DecimalError> for DimensionError {
    fn from(e: icu_decimal::DecimalError) -> Self {
        DimensionError::Decimal(e)
    }
}
