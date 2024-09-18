// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::collections::codepointtrie::toml::CodePointTrieToml;

#[derive(serde::Deserialize)]
pub(crate) struct Exceptions {
    pub(crate) exceptions: Vec<u16>,
}

#[derive(serde::Deserialize)]
pub(crate) struct Unfold {
    pub(crate) unfold: Vec<u16>,
}

#[derive(serde::Deserialize)]
pub(crate) struct Level1 {
    pub(crate) code_point_trie: CodePointTrieToml,
    pub(crate) exceptions: Exceptions,
    pub(crate) unfold: Unfold,
}

#[derive(serde::Deserialize)]
pub(crate) struct Main {
    pub(crate) ucase: Level1,
}
