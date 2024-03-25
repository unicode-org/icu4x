// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointtrie::toml::CodePointTrieToml;

#[derive(serde::Deserialize)]
pub struct Exceptions {
    pub exceptions: Vec<u16>,
}

#[derive(serde::Deserialize)]
pub struct Unfold {
    pub unfold: Vec<u16>,
}

#[derive(serde::Deserialize)]
pub struct Level1 {
    pub code_point_trie: CodePointTrieToml,
    pub exceptions: Exceptions,
    pub unfold: Unfold,
}

#[derive(serde::Deserialize)]
pub struct Main {
    pub ucase: Level1,
}
