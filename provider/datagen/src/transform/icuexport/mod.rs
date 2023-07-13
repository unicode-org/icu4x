// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

pub mod collator;
pub mod normalizer;
#[cfg(feature = "icu_casemap")]
pub mod ucase;
pub mod uprops;
