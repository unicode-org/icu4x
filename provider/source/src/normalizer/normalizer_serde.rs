// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::collections::codepointtrie::toml::CodePointTrieToml;

#[derive(serde::Deserialize)]
pub(crate) struct DecompositionData {
    pub(crate) trie: CodePointTrieToml,
    pub(crate) cap: u16,
}

#[derive(serde::Deserialize)]
pub(crate) struct DecompositionTables {
    pub(crate) scalars16: Vec<u16>,
    pub(crate) scalars32: Vec<u32>,
}

#[derive(serde::Deserialize)]
pub(crate) struct CompositionPassthrough {
    #[serde(rename = "trie")]
    pub(crate) _trie: CodePointTrieToml,
    #[serde(rename = "first")]
    pub(crate) _first: u32,
}

#[derive(serde::Deserialize)]
pub(crate) struct CanonicalCompositions {
    pub(crate) compositions: Vec<u16>,
}

#[derive(serde::Deserialize)]
pub(crate) struct NonRecursiveDecompositionSupplement {
    pub(crate) trie: CodePointTrieToml,
    pub(crate) scalars32: Vec<u32>,
}
