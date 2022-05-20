// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{DataError, DataErrorKind};

/// Identifies errors that are due to missing CLDR data.
///
/// See [`datagen`](crate::datagen).
pub const MISSING_CLDR_ERROR: DataError = DataErrorKind::MissingSourceData
    .into_error()
    .with_str_context("CLDR");

/// Identifies errors that are due to missing Unicode properties data.
///
/// See [`datagen`](crate::datagen).
pub const MISSING_UPROPS_ERROR: DataError = DataErrorKind::MissingSourceData
    .into_error()
    .with_str_context("Uprops");

/// Identifies errors that are due to missing collation data.
/// 
/// See ['datagen`](crate::datagen).
pub const MISSING_COLLATION_ERROR: DataError = DataErrorKind::MissingSourceData
    .into_error()
    .with_str_context("Collation");

pub(crate) fn data_error_from_toml(other: toml::de::Error) -> DataError {
    DataError::custom("Toml deserialize").with_display_context(&other)
}
