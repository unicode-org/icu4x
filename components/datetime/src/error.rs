use crate::pattern;
use icu_data_provider::prelude::DataError;

/// A list of possible error outcomes for the [`DateTimeFormat`] struct.
///
/// [`DateTimeFormat`]: ./struct.DateTimeFormat.html
#[derive(Debug)]
pub enum DateTimeFormatError {
    Pattern(pattern::Error),
    MissingData,
    /// An error originating inside of the DataProvider
    DataProvider(DataError),
}

impl From<DataError> for DateTimeFormatError {
    fn from(err: DataError) -> Self {
        Self::DataProvider(err)
    }
}

impl From<pattern::Error> for DateTimeFormatError {
    fn from(err: pattern::Error) -> Self {
        Self::Pattern(err)
    }
}
