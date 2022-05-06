// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_codepointtrie_builder` is a utility crate of the [`ICU4X`] project.
//!
//! This crate exposes functionality to build a [`CodePointTrie`] from values provided at runtime.
//! Because it is normally expected for CodePointTrie data to be pre-compiled, this crate is not
//! optimized for speed; it should be used during a build phase.
//!
//! Under the hood, this crate uses the CodePointTrie builder code from ICU4C, [`UMutableCPTrie`],
//! shipped as a WebAssembly module and then JIT-compiled at runtime.
//!
//! # Examples
//!
//! ```
//! use icu_codepointtrie::CodePointTrie;
//! use icu_codepointtrie::TrieType;
//! use icu_codepointtrie_builder::CodePointTrieBuilder;
//! use icu_codepointtrie_builder::CodePointTrieBuilderData;
//!
//! let default_value = 1;
//! let error_value = 2;
//! let values_by_code_point = &[3, 4, 5, 6];
//!
//! let cpt: CodePointTrie<u8> = CodePointTrieBuilder {
//!     data: CodePointTrieBuilderData::ValuesByCodePoint(values_by_code_point),
//!     default_value,
//!     error_value,
//!     trie_type: TrieType::Small
//! }.build();
//!
//! assert_eq!(cpt.get(0), 3);
//! assert_eq!(cpt.get(1), 4);
//! assert_eq!(cpt.get(2), 5);
//! assert_eq!(cpt.get(3), 6);
//! assert_eq!(cpt.get(4), 1); // default value
//! assert_eq!(cpt.get(u32::MAX), 2); // error value
//! ```
//!
//! [`ICU4X`]: ../icu/index.html
//! [`UMutableCPTrie`]: (https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/umutablecptrie_8h.html#ad8945cf34ca9d40596a66a1395baa19b)

use icu_codepointtrie::toml::CodePointTrieToml;
use icu_codepointtrie::CodePointTrie;
use icu_codepointtrie::TrieType;
use icu_codepointtrie::TrieValue;
use std::convert::TryInto;

mod wasm;

#[non_exhaustive]
pub enum CodePointTrieBuilderData<'a, T> {
    ValuesByCodePoint(&'a [T]),
}

#[allow(clippy::exhaustive_structs)]
pub struct CodePointTrieBuilder<'a, T> {
    pub data: CodePointTrieBuilderData<'a, T>,
    pub default_value: T,
    pub error_value: T,
    pub trie_type: TrieType,
}

impl<T> CodePointTrieBuilder<'_, T>
where
    T: TrieValue + Into<u32>,
{
    pub fn build(self) -> CodePointTrie<'static, T> {
        let toml_str = wasm::run_wasm(&self);
        let toml_obj: CodePointTrieToml = toml::from_str(&toml_str).unwrap();
        (&toml_obj).try_into().unwrap()
    }
}
