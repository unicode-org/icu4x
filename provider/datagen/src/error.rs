// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{DataError, DataErrorKind};

pub(crate) const MISSING_CLDR_ERROR: DataError =
    DataErrorKind::MissingSourceData.with_str_context("cldr");

pub(crate) const MISSING_ICUEXPORT_ERROR: DataError =
    DataErrorKind::MissingSourceData.with_str_context("icuexport");

/// Identifies errors that are due to missing CLDR data.
///
/// See [`datagen`](crate::datagen).
pub fn is_missing_cldr_error(mut e: DataError) -> bool {
    e.key = None;
    e == MISSING_CLDR_ERROR
}

/// Identifies errors that are due to missing ICU export data.
///
/// See [`datagen`](crate::datagen).
pub fn is_missing_icuexport_error(mut e: DataError) -> bool {
    e.key = None;
    e == MISSING_ICUEXPORT_ERROR
}

pub(crate) fn data_error_from_toml(other: toml::de::Error) -> DataError {
    DataError::custom("Toml deserialize").with_display_context(&other)
}

pub(crate) fn data_error_from_json(other: serde_json::Error) -> DataError {
    DataError::custom("JSON deserialize").with_display_context(&other)
}
