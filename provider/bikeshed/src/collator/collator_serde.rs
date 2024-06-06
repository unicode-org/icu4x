// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::collections::codepointtrie::toml::CodePointTrieToml;

/// Serde counterpart for `CollationDataV1`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationData {
    pub(crate) trie: CodePointTrieToml,
    pub(crate) contexts: Vec<u16>,
    pub(crate) ce32s: Vec<u32>,
    // TOML integers are signed 64-bit, so the range of u64 isn't available
    pub(crate) ces: Vec<i64>,
}

/// Serde counterpart for `CollationDiacriticsV1`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationDiacritics {
    pub(crate) secondaries: Vec<u16>,
}

/// Serde counterpart for `CollationJamoV1`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationJamo {
    pub(crate) ce32s: Vec<u32>,
}

/// Serde counterpart for `CollationMetadataV1`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationMetadata {
    pub(crate) bits: u32,
}

/// Serde counterpart for `CollationReorderingV1`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationReordering {
    pub(crate) min_high_no_reorder: u32,
    pub(crate) reorder_table: Vec<u8>,
    pub(crate) reorder_ranges: Vec<u32>,
}

/// Serde counterpart for `CollationSpecialPrimariesV1`.
#[derive(serde::Deserialize)]
pub(crate) struct CollationSpecialPrimaries {
    pub(crate) last_primaries: Vec<u16>, // length always supposed to be 4
    pub(crate) numeric_primary: u8,
}
