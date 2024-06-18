// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::collections::codepointtrie::toml::CodePointTrieToml;

#[derive(serde::Deserialize)]
pub(in crate::provider) struct Exceptions {
    pub(in crate::provider) exceptions: Vec<u16>,
}

#[derive(serde::Deserialize)]
pub(in crate::provider) struct Unfold {
    pub(in crate::provider) unfold: Vec<u16>,
}

#[derive(serde::Deserialize)]
pub(in crate::provider) struct Level1 {
    pub(in crate::provider) code_point_trie: CodePointTrieToml,
    pub(in crate::provider) exceptions: Exceptions,
    pub(in crate::provider) unfold: Unfold,
}

#[derive(serde::Deserialize)]
pub(in crate::provider) struct Main {
    pub(in crate::provider) ucase: Level1,
}
