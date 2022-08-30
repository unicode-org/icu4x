// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointtrie::toml::CodePointTrieToml;

/// Serde counterpart for `CollationDataV1`.
#[derive(serde::Deserialize)]
pub struct CollationData {
    pub trie: CodePointTrieToml,
    pub contexts: Vec<u16>,
    pub ce32s: Vec<u32>,
    // TOML integers are signed 64-bit, so the range of u64 isn't available
    pub ces: Vec<i64>,
}

/// Serde counterpart for `CollationDiacriticsV1`.
#[derive(serde::Deserialize)]
pub struct CollationDiacritics {
    pub secondaries: Vec<u16>,
}

/// Serde counterpart for `CollationJamoV1`.
#[derive(serde::Deserialize)]
pub struct CollationJamo {
    pub ce32s: Vec<u32>,
}

/// Serde counterpart for `CollationMetadataV1`.
#[derive(serde::Deserialize)]
pub struct CollationMetadata {
    pub bits: u32,
}

/// Serde counterpart for `CollationReorderingV1`.
#[derive(serde::Deserialize)]
pub struct CollationReordering {
    pub min_high_no_reorder: u32,
    pub reorder_table: Vec<u8>,
    pub reorder_ranges: Vec<u32>,
}

/// Serde counterpart for `CollationSpecialPrimariesV1`.
#[derive(serde::Deserialize)]
pub struct CollationSpecialPrimaries {
    pub last_primaries: Vec<u16>, // length always supposed to be 4
    pub numeric_primary: u8,
}
