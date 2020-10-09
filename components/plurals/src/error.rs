use crate::rules::parser::ParserError;
use icu_data_provider::prelude::DataError;

/// A list of possible error outcomes for the [`PluralRules`] struct.
///
/// [`PluralRules`]: ./struct.PluralRules.html
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
