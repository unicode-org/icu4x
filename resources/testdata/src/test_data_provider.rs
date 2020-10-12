// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_fs_data_provider::FsDataProvider;
use std::path::PathBuf;

/// Get a DataProvider loading from test data. Panics if unable to load the data.
pub fn get_provider() -> FsDataProvider {
    FsDataProvider::try_new(
        PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .join("data")
            .join("json"),
    )
    .expect("The test data directory was unable to be opened")
}
