// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_codepointtrie::toml::CodePointTrieToml;

#[derive(serde::Deserialize)]
pub struct DecompositionData {
    pub trie: CodePointTrieToml,
    pub scalars16: Vec<u16>,
    pub scalars32: Vec<u32>,
    pub ranges: Vec<(u32, u32)>,
}
