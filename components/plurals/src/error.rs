// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::rules::parser::ParserError;
use icu_provider::prelude::DataError;
use thiserror::Error;

/// A list of possible error outcomes for the [`PluralRules`](crate::PluralRules) struct.
///
#[derive(Error, Debug)]
pub enum PluralRulesError {
    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),
    /// An error originating inside of the [`DataProvider`](icu_provider::DataProvider)
    #[error("Data provider error: {0}")]
    DataProvider(#[from] DataError),
}
