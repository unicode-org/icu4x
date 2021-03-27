// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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

/// Returns the absolute path to ppucd.txt.
pub fn ppucd_path() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("ppucd")
        .join("ppucd.txt")
}
