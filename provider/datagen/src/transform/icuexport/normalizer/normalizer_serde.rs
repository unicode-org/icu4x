// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::collections::codepointtrie::toml::CodePointTrieToml;

#[derive(serde::Deserialize)]
pub(in crate::provider) struct DecompositionData {
    pub(in crate::provider) trie: CodePointTrieToml,
}

#[derive(serde::Deserialize)]
pub(in crate::provider) struct DecompositionSupplement {
    pub(in crate::provider) trie: CodePointTrieToml,
    pub(in crate::provider) flags: u8,
    pub(in crate::provider) cap: u16,
}

#[derive(serde::Deserialize)]
pub(in crate::provider) struct DecompositionTables {
    pub(in crate::provider) scalars16: Vec<u16>,
    pub(in crate::provider) scalars32: Vec<u32>,
}

#[derive(serde::Deserialize)]
pub(in crate::provider) struct CompositionPassthrough {
    #[serde(rename = "trie")]
    pub(in crate::provider) _trie: CodePointTrieToml,
    #[serde(rename = "first")]
    pub(in crate::provider) _first: u32,
}

#[derive(serde::Deserialize)]
pub(in crate::provider) struct CanonicalCompositions {
    pub(in crate::provider) compositions: Vec<u16>,
}

#[derive(serde::Deserialize)]
pub(in crate::provider) struct NonRecursiveDecompositionSupplement {
    pub(in crate::provider) trie: CodePointTrieToml,
    pub(in crate::provider) scalars32: Vec<u32>,
}
