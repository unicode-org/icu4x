// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::rules::reference::parser::ParserError;
use displaydoc::Display;
use icu_provider::prelude::DataError;

/// A list of possible error outcomes for the [`PluralRules`](crate::PluralRules) struct.
///
#[derive(Display, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum PluralRulesError {
    /// A parsing error for the plural rules.
    #[displaydoc("Parser error: {0}")]
    Parser(ParserError),
    /// An error originating from [`icu_provider`].
    #[displaydoc("Data provider error: {0}")]
    DataProvider(DataError),
}

#[cfg(feature = "std")]
impl std::error::Error for PluralRulesError {}

impl From<ParserError> for PluralRulesError {
    fn from(e: ParserError) -> Self {
        PluralRulesError::Parser(e)
    }
}

impl From<DataError> for PluralRulesError {
    fn from(e: DataError) -> Self {
        PluralRulesError::DataProvider(e)
    }
}
