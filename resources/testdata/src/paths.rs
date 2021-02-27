use std::path::PathBuf;

/// Returns the absolute path to the CLDR JSON root directory.
pub fn cldr_json_root() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("cldr")
}

/// Returns the absolute path to the ICU4X JSON root directory.
pub fn icu4x_json_root() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("json")
}
