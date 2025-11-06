// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::collections::codepointtrie::toml::CodePointTrieToml;

/// Serde counterpart for `CollationData`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationData {
    pub(crate) trie: CodePointTrieToml,
    pub(crate) contexts: Vec<u16>,
    pub(crate) ce32s: Vec<u32>,
    // TOML integers are signed 64-bit, so the range of u64 isn't available
    pub(crate) ces: Vec<i64>,
}

/// Serde counterpart for `CollationDiacritics`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationDiacritics {
    pub(crate) secondaries: Vec<u16>,
}

/// Serde counterpart for `CollationJamo`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationJamo {
    pub(crate) ce32s: Vec<u32>,
}

/// Serde counterpart for `CollationMetadata`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationMetadata {
    pub(crate) bits: u32,
}

/// Serde counterpart for `CollationReordering`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationReordering {
    pub(crate) min_high_no_reorder: u32,
    pub(crate) reorder_table: Vec<u8>,
    pub(crate) reorder_ranges: Vec<u32>,
}

/// Serde counterpart for `CollationSpecialPrimaries`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationSpecialPrimaries {
    /// Length always supposed to be 4
    pub(crate) last_primaries: Vec<u16>, //
    /// `Option` to support datagen with ICU4X 2.0.0-associated
    /// TOML from ICU4C tag icu4x/2025-05-01/77.x, which doesn't
    /// have this data.
    ///
    /// For correct results, when CLDR is updated on the ICU4C
    /// side, icuexportdata must start including this data in
    /// the TOML.
    pub(crate) compressible_bytes: Option<Vec<bool>>,
    pub(crate) numeric_primary: u8,
}
