// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

pub(in crate::provider) mod collator;
pub(in crate::provider) mod normalizer;
pub(in crate::provider) mod ucase;
pub(in crate::provider) mod uprops;
