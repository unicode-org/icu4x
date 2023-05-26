// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub fn json() -> icu_provider_fs::FsDataProvider {
    #[allow(clippy::unwrap_used)] // Valid dir
    icu_provider_fs::FsDataProvider::try_new(paths::json()).unwrap()
}

pub mod paths {
    use std::path::PathBuf;
    fn data_root() -> PathBuf {
        PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data")
    }

    pub fn json() -> PathBuf {
        data_root().join("json")
    }

    pub fn cldr() -> PathBuf {
        data_root().join("cldr")
    }

    pub fn icuexport() -> PathBuf {
        data_root().join("icuexport")
    }

    pub fn lstm() -> PathBuf {
        data_root().join("lstm")
    }
}
