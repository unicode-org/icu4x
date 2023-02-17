
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
}
