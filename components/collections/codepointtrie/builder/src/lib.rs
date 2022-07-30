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
//! shipped as a WebAssembly module and then JIT-compiled at runtime. For more context, see
//! <https://github.com/unicode-org/icu4x/issues/1837>.
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
//!     trie_type: TrieType::Small,
//! }
//! .build();
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

#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums
    )
)]
// This is a build tool with many invariants being enforced
#![allow(clippy::panic)]
#![allow(clippy::expect_used)]

use icu_codepointtrie::toml::CodePointTrieToml;
use icu_codepointtrie::CodePointTrie;
use icu_codepointtrie::TrieType;
use icu_codepointtrie::TrieValue;
use std::convert::TryInto;

mod wasm;

/// Wrapper over the data to be encoded into a CodePointTrie.
///
/// There is currently only one variant, but more may be added in the future.
#[non_exhaustive]
pub enum CodePointTrieBuilderData<'a, T> {
    /// A list of values for each code point, starting from code point 0.
    ///
    /// For example, the value for U+0020 (space) should be at index 32 in the slice.
    /// Index 0 sets the value for the U+0000 (NUL).
    ValuesByCodePoint(&'a [T]),
}

/// Settings for building a CodePointTrie.
#[allow(clippy::exhaustive_structs)]
pub struct CodePointTrieBuilder<'a, T> {
    /// The data to be encoded.
    pub data: CodePointTrieBuilderData<'a, T>,
    /// The default value for code points not specified in the data.
    pub default_value: T,
    /// The error value for invalid code points.
    pub error_value: T,
    /// The [`TrieType`]: fast or small.
    pub trie_type: TrieType,
}

impl<T> CodePointTrieBuilder<'_, T>
where
    T: TrieValue + Into<u32>,
{
    /// Build the [`CodePointTrie`].
    ///
    /// Under the hood, this function runs ICU4C code compiled into WASM.
    pub fn build(self) -> CodePointTrie<'static, T> {
        let toml_str = wasm::run_wasm(&self);
        let toml_obj: CodePointTrieToml =
            toml::from_str(&toml_str).expect("the tool should produce valid TOML");
        (&toml_obj)
            .try_into()
            .expect("the toml should be a valid CPT")
    }
}
