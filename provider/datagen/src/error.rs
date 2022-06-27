// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{DataError, DataErrorKind};

/// Identifies errors that are due to missing CLDR data.
///
/// See [`datagen`](crate::datagen).
pub const MISSING_CLDR_ERROR: DataError = DataErrorKind::MissingSourceData.with_str_context("cldr");

/// Identifies errors that are due to missing ICU export data.
///
/// See [`datagen`](crate::datagen).
pub const MISSING_ICUEXPORT_ERROR: DataError =
    DataErrorKind::MissingSourceData.with_str_context("icuexport");

pub(crate) fn data_error_from_toml(other: toml::de::Error) -> DataError {
    DataError::custom("Toml deserialize").with_display_context(&other)
}
