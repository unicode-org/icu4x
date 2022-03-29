// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider_blob::StaticDataProvider;

const SMALLER_STATIC_STR_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/decimal-bn-en.postcard"
));

const STATIC_STR_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/testdata.postcard"
));

/// Get a `DataProvider`, loading from the statically initialized bincode blob.
/// Panics if unable to load the data.
pub fn get_static_provider() -> StaticDataProvider {
    // The statically compiled data file is valid.
    #[allow(clippy::expect_used)]
    StaticDataProvider::new_from_static_blob(STATIC_STR_DATA)
        .expect("Deserialization should succeed")
}

/// Get a `DataProvider`, loading from the statically initialized bincode blob
/// containing only bn/en decimal symbols
/// Panics if unable to load the data.
pub fn get_smaller_static_provider() -> StaticDataProvider {
    // The statically compiled data file is valid.
    #[allow(clippy::expect_used)]
    StaticDataProvider::new_from_static_blob(SMALLER_STATIC_STR_DATA)
        .expect("Deserialization should succeed")
}
