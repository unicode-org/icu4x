// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_decimal::DecimalError;
use icu_plurals::PluralsError;
use icu_provider::DataError;

/// A list of error outcomes for various operations in this module.
///
/// Re-exported as [`Error`](crate::Error).
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum RelativeTimeError {
    /// An error originating from [`PluralRules`](icu_plurals::PluralRules).
    #[displaydoc("Error loading plural rules: {0}")]
    PluralRules(PluralsError),
    /// An error originating from [`DataProvider`](icu_provider::DataProvider).
    #[displaydoc("Error loading data: {0}")]
    Data(DataError),
    /// An error originating from [`FixedDecimalFormatter`](icu_decimal::FixedDecimalFormatter).
    #[displaydoc("Error loading FixedDecimalFormatter: {0}")]
    Decimal(DecimalError),
}

impl From<PluralsError> for RelativeTimeError {
    fn from(e: PluralsError) -> Self {
        RelativeTimeError::PluralRules(e)
    }
}

impl From<DataError> for RelativeTimeError {
    fn from(e: DataError) -> Self {
        RelativeTimeError::Data(e)
    }
}

impl From<DecimalError> for RelativeTimeError {
    fn from(e: DecimalError) -> Self {
        RelativeTimeError::Decimal(e)
    }
}
