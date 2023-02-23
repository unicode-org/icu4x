// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_codepointtrie_builder` is a utility crate of the [`ICU4X`] project.
//!
//! This crate exposes functionality to build a [`CodePointTrie`] from values provided at runtime.
//! Because it is normally expected for CodePointTrie data to be pre-compiled, this crate is not
//! optimized for speed; it should be used during a build phase.
//!
//! Under the hood, this crate uses the CodePointTrie builder code from ICU4C, [`UMutableCPTrie`].
//! For more context, see <https://github.com/unicode-org/icu4x/issues/1837>.
//!
//! # Build configuration
//!
//! This crate has two primary modes it can be used in: `"wasm"` and `"icu4c"`, exposed as
//! Cargo features. If both are enabled, the code will internally use the wasm codepath.
//!
//! The `"wasm"` mode uses an included wasm module that was built by linking
//! it to ICU4C, run on a wasm runtime. It pulls in a lot of dependencies to do this, but
//! it should just work with no further effort.
//!
//! The `"icu4c"` mode requires some extra effort: it links to a local copy of ICU4C.
//! If using Cargo, you can use the `ICU4C_LIB_PATH` environment variable to point this to
//! a directory full of ICU4X static or shared libraries, and `ICU4C_LINK_STATICALLY` to use
//! static linking (if using dynamic linking you will have to set `[DY]LD_LIBRARY_PATH` at runtime
//! as well). If building directly, make sure this path is provided via `-L`, and that the
//! CLI requests to link against `icuuc`, `icui18n` and `icudata` via `-l` flags.
//!
//! ICU4C can  *renamed* symbols, where each function is suffixed with a version number.
//! This crate by default will link to unrenamed symbols. If you have built it with renaming
//! enabled, you can set the `ICU4C_RENAME_VERSION=<version>` env var. When building without Cargo
//! this must be paired with `--cfg icu4c_enable_renaming`.
//!
//! # Examples
//!
//! ```
//! use icu::collections::codepointtrie::CodePointTrie;
//! use icu::collections::codepointtrie::TrieType;
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
//! assert_eq!(cpt.get32(0), 3);
//! assert_eq!(cpt.get32(1), 4);
//! assert_eq!(cpt.get32(2), 5);
//! assert_eq!(cpt.get32(3), 6);
//! assert_eq!(cpt.get32(4), 1); // default value
//! assert_eq!(cpt.get32(u32::MAX), 2); // error value
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
        clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]
// This is a build tool with many invariants being enforced
#![allow(clippy::panic)]
#![allow(clippy::expect_used)]

use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "icu4c")]
mod native;

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
    /// Under the hood, this function runs ICU4C code compiled into WASM,
    /// or links natively to ICU4C as specified by the `ICU4C_LIB_PATH` env var
    ///
    /// This function needs either the `"wasm"` or `"icu4c"` feature
    #[cfg(any(feature = "wasm", feature = "icu4c"))]
    pub fn build(self) -> icu_collections::codepointtrie::CodePointTrie<'static, T> {
        #[cfg(feature = "wasm")]
        {
            use icu_collections::codepointtrie::toml::CodePointTrieToml;

            let toml_str = wasm::run_wasm(&self);
            let toml_obj: CodePointTrieToml =
                toml::from_str(&toml_str).expect("the tool should produce valid TOML");
            (&toml_obj)
                .try_into()
                .expect("the toml should be a valid CPT")
        }

        #[cfg(all(feature = "icu4c", not(feature = "wasm")))]
        {
            native::run_native(&self)
        }
    }
}

#[test]
fn test_cpt_builder() {
    // Buckets of ten characters for 0 to 100, and then some default values, and then heterogenous "last hex digit" for 0x100 to 0x200
    let values: Vec<u32> = (0..100)
        .map(|x| x / 10)
        .chain((100..0x100).map(|_| 100))
        .chain((0x100..0x200).map(|x| x % 16))
        .collect();

    let builder = CodePointTrieBuilder {
        data: CodePointTrieBuilderData::ValuesByCodePoint(&values),
        default_value: 100,
        error_value: 0xFFFF,
        trie_type: TrieType::Fast,
    };

    let cpt = builder.build();

    assert_eq!(cpt.get32(0), 0);
    assert_eq!(cpt.get32(10), 1);
    assert_eq!(cpt.get32(20), 2);
    assert_eq!(cpt.get32(21), 2);
    assert_eq!(cpt.get32(99), 9);
    assert_eq!(cpt.get32(0x101), 0x1);
    assert_eq!(cpt.get32(0x102), 0x2);
    assert_eq!(cpt.get32(0x105), 0x5);
    assert_eq!(cpt.get32(0x125), 0x5);
    assert_eq!(cpt.get32(0x135), 0x5);
    assert_eq!(cpt.get32(0x13F), 0xF);
    // default value
    assert_eq!(cpt.get32(0x300), 100);
}
