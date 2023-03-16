// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Debug;
use displaydoc::Display;
use icu_provider::prelude::DataError;

#[cfg(feature = "std")]
impl std::error::Error for SegmenterError {}

/// A list of error outcomes for various operations in the `icu_timezone` crate.
///
/// Re-exported as [`Error`](crate::Error).
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum SegmenterError {
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("{0}")]
    Data(DataError),
    /// An error occurred while preparing the LSTM data.
    #[displaydoc("LSTM data error")]
    #[cfg(feature = "lstm")]
    LstmDataError,
}

impl From<DataError> for SegmenterError {
    fn from(e: DataError) -> Self {
        Self::Data(e)
    }
}

#[cfg(feature = "lstm")]
impl From<crate::lstm_error::Error> for SegmenterError {
    fn from(_: crate::lstm_error::Error) -> Self {
        Self::LstmDataError
    }
}
