// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::rules::parser::ParserError;
use icu_provider::prelude::DataError;
use std::fmt;

/// A list of possible error outcomes for the [`PluralRules`](crate::PluralRules) struct.
///
#[derive(Debug)]
pub enum PluralRulesError {
    Parser(ParserError),
    /// An error originating inside of the DataProvider
    DataProvider(DataError),
}

impl From<DataError> for PluralRulesError {
    fn from(err: DataError) -> Self {
        Self::DataProvider(err)
    }
}

impl From<ParserError> for PluralRulesError {
    fn from(err: ParserError) -> Self {
        Self::Parser(err)
    }
}

impl fmt::Display for PluralRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parser(error) => write!(f, "Parser error: {}", error),
            Self::DataProvider(error) => write!(f, "Data provider error: {}", error),
        }
    }
}

impl std::error::Error for PluralRulesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parser(error) => Some(error),
            Self::DataProvider(error) => Some(error),
        }
    }
}
