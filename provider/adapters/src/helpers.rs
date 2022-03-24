use icu_provider::prelude::*;

pub(crate) fn result_is_err_missing_resource_key<T>(result: &Result<T, DataError>) -> bool {
    matches!(
        result,
        Err(DataError {
            kind: DataErrorKind::MissingResourceKey,
            ..
        })
    )
}
