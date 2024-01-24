// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Module for tests that need to access `#[cfg(test)]` code and dependencies.
//!
//! Most tests should either be in-module unit tests or integration tests.

#[cfg(all(feature = "fs_exporter", feature = "use_wasm"))]
mod make_testdata;

#[cfg(icu4x_test_datagen)]
mod test_datagen;

mod data;
