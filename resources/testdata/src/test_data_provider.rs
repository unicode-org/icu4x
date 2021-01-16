// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_provider_fs::FsDataProvider;
use std::path::PathBuf;

/// Get a `DataProvider` loading from test data. Panics if unable to load the data.
pub fn get_provider() -> FsDataProvider {
    let path: PathBuf = match std::env::var_os("ICU4X_TESTDATA_DIR") {
        Some(val) => val.into(),
        None => PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .join("data")
            .join("json")
            .into(),
    };
    FsDataProvider::try_new(&path).expect(&format!(
        "The test data directory was unable to be opened: {:?}",
        path
    ))
}
