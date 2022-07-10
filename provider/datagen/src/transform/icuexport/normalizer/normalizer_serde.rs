// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_codepointtrie::toml::CodePointTrieToml;

#[derive(serde::Deserialize)]
pub struct DecompositionData {
    pub trie: CodePointTrieToml,
    pub ranges: Vec<(u32, u32)>,
}

#[derive(serde::Deserialize)]
pub struct DecompositionSupplement {
    pub trie: CodePointTrieToml,
    pub flags: u8,
}

#[derive(serde::Deserialize)]
pub struct DecompositionTables {
    pub scalars16: Vec<u16>,
    pub scalars32: Vec<u32>,
}

#[derive(serde::Deserialize)]
pub struct CompositionPassthrough {
    pub ranges: Vec<(u32, u32)>,
}

#[derive(serde::Deserialize)]
pub struct CanonicalCompositions {
    pub compositions: Vec<u16>,
}
