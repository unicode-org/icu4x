// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Get paths to the test data directories. Some of these paths do not
//! exist anymore, and data should only be accessed through the functions
//! provided by this crate.

use std::path::PathBuf;

/// Returns the absolute path to the top-level data directory.
pub fn data_root() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data")
}

/// Returns the absolute path to the CLDR JSON root directory.
pub fn cldr_json_root() -> PathBuf {
    data_root().join("cldr")
}

/// Returns the absolute path to the icuexport TOML root directory.
pub fn icuexport_toml_root() -> PathBuf {
    data_root().join("icuexport")
}

/// Returns the absolute path to the collation tailoring TOML root directory.
pub fn coll_toml_root() -> PathBuf {
    data_root().join("coll")
}
