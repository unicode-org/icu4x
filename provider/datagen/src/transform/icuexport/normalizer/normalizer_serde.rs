// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointtrie::toml::CodePointTrieToml;

#[derive(serde::Deserialize)]
pub struct DecompositionData {
    pub trie: CodePointTrieToml,
}

#[derive(serde::Deserialize)]
pub struct DecompositionSupplement {
    pub trie: CodePointTrieToml,
    pub flags: u8,
    pub cap: u16,
}

#[derive(serde::Deserialize)]
pub struct DecompositionTables {
    pub scalars16: Vec<u16>,
    pub scalars32: Vec<u32>,
}

#[derive(serde::Deserialize)]
pub struct CompositionPassthrough {
    pub trie: CodePointTrieToml,
    pub first: u32,
}

#[derive(serde::Deserialize)]
pub struct CanonicalCompositions {
    pub compositions: Vec<u16>,
}

#[derive(serde::Deserialize)]
pub struct NonRecursiveDecompositionSupplement {
    pub trie: CodePointTrieToml,
    pub scalars32: Vec<u32>,
}
