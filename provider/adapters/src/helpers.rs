// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;

pub(crate) fn result_is_err_missing_data_key<T>(result: &Result<T, DataError>) -> bool {
    matches!(
        result,
        Err(DataError {
            kind: DataErrorKind::MissingDataKey,
            ..
        })
    )
}

pub(crate) fn result_is_err_missing_data_options<T>(result: &Result<T, DataError>) -> bool {
    // TODO(#2117): Fold MissingLocale and MissingVariant into MissingDataLocale.
    matches!(
        result,
        Err(DataError {
            kind: DataErrorKind::MissingDataLocale,
            ..
        })
    ) || matches!(
        result,
        Err(DataError {
            kind: DataErrorKind::MissingLocale,
            ..
        })
    ) || matches!(
        result,
        Err(DataError {
            kind: DataErrorKind::MissingVariant,
            ..
        })
    )
}
