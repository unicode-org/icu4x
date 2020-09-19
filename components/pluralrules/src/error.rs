use {crate::rules::parser::ParserError, std::fmt, icu_data_provider::prelude::DataError};

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

impl fmt::Display for PluralRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PluralRulesError::Parser(error) => write!(f, "Parser error: {}", error),
            PluralRulesError::DataProvider(error) => write!(f, "Data provider error: {}", error),
        }
    }
}

impl std::error::Error for PluralRulesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PluralRulesError::Parser(error) => Some(error),
            PluralRulesError::DataProvider(error) => Some(error),
        }
    }
}
