// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).


#[derive(serde::Deserialize)]
pub struct TestFile {
    ucharstrie: Char16Trie,
}

#[derive(serde::Deserialize)]
pub struct Char16Trie {
    data: Vec<u16>,
}
