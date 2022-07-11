// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

#[cfg(feature = "experimental")]
pub mod collator;
#[cfg(feature = "experimental")]
pub mod normalizer;
#[cfg(feature = "experimental")]
pub mod ucase;
pub mod uprops;
