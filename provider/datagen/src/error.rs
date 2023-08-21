// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{DataError, DataErrorKind};

pub(crate) const MISSING_CLDR_ERROR: DataError =
    DataErrorKind::MissingSourceData.with_str_context("cldr");

pub(crate) const MISSING_ICUEXPORT_ERROR: DataError =
    DataErrorKind::MissingSourceData.with_str_context("icuexport");

pub(crate) const MISSING_SEGMENTER_LSTM_ERROR: DataError =
    DataErrorKind::MissingSourceData.with_str_context("segmenter");

/// Identifies errors that are due to missing CLDR data.
pub fn is_missing_cldr_error(mut e: DataError) -> bool {
    e.key = None;
    e == MISSING_CLDR_ERROR
}

/// Identifies errors that are due to missing ICU export data.
pub fn is_missing_icuexport_error(mut e: DataError) -> bool {
    e.key = None;
    e == MISSING_ICUEXPORT_ERROR
}

/// Identifies errors that are due to missing segmenter LSTM data.
pub fn is_missing_segmenter_lstm_error(mut e: DataError) -> bool {
    e.key = None;
    e == MISSING_SEGMENTER_LSTM_ERROR
}
