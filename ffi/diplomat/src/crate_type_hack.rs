// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// See comment in icu_capi's Cargo.toml
//
// This is essentially a hack that allows icu_capi to be compiled
// to arbitrary `--crate-type`s without having to add a `--crate-type`
// to the list in Cargo.toml
extern crate icu_capi;
