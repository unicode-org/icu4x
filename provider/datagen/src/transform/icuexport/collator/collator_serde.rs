// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::collections::codepointtrie::toml::CodePointTrieToml;

/// Serde counterpart for `CollationDataV1`.
#[derive(serde::Deserialize)]
pub(in crate::provider) struct CollationData {
    pub(in crate::provider) trie: CodePointTrieToml,
    pub(in crate::provider) contexts: Vec<u16>,
    pub(in crate::provider) ce32s: Vec<u32>,
    // TOML integers are signed 64-bit, so the range of u64 isn't available
    pub(in crate::provider) ces: Vec<i64>,
}

/// Serde counterpart for `CollationDiacriticsV1`.
#[derive(serde::Deserialize)]
pub(in crate::provider) struct CollationDiacritics {
    pub(in crate::provider) secondaries: Vec<u16>,
}

/// Serde counterpart for `CollationJamoV1`.
#[derive(serde::Deserialize)]
pub(in crate::provider) struct CollationJamo {
    pub(in crate::provider) ce32s: Vec<u32>,
}

/// Serde counterpart for `CollationMetadataV1`.
#[derive(serde::Deserialize)]
pub(in crate::provider) struct CollationMetadata {
    pub(in crate::provider) bits: u32,
}

/// Serde counterpart for `CollationReorderingV1`.
#[derive(serde::Deserialize)]
pub(in crate::provider) struct CollationReordering {
    pub(in crate::provider) min_high_no_reorder: u32,
    pub(in crate::provider) reorder_table: Vec<u8>,
    pub(in crate::provider) reorder_ranges: Vec<u32>,
}

/// Serde counterpart for `CollationSpecialPrimariesV1`.
#[derive(serde::Deserialize)]
pub(in crate::provider) struct CollationSpecialPrimaries {
    pub(in crate::provider) last_primaries: Vec<u16>, // length always supposed to be 4
    pub(in crate::provider) numeric_primary: u8,
}
