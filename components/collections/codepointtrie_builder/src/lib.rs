// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
// #![cfg_attr(not(any(test, doc)), no_std)]
// #![cfg_attr(
//     not(test),
//     deny(
//         clippy::indexing_slicing,
//         clippy::unwrap_used,
//         clippy::expect_used,
//         clippy::panic,
//     )
// )]
#![warn(missing_docs)]

//! `icu_codepointtrie_builder` is a utility crate of the [`ICU4X`] project.
//!
//! This crate exposes functionality to build a [`CodePointTrie`] from values provided at runtime.
//! Because it is normally expected for [`CodePointTrie`] data to be pre-compiled, this crate is not
//! optimized for speed; it should be used during a build phase.
//!
//! Under the hood, this crate uses the [`CodePointTrie`] builder code from ICU4C, [`UMutableCPTrie`].
//! For more context, see <https://github.com/unicode-org/icu4x/issues/1837>.
//!
//! Unlike most of ICU4X, due in large part to the native dependency, this crate is not guaranteed
//! to be panic-free.
//!
//! # Build configuration
//!
//! This crate has two primary modes it can be used in: `"wasm"` and `"icu4c"`, exposed as
//! Cargo features. If both are enabled, the code will internally use the wasm codepath.
//!
//! The `"wasm"` mode uses a Wasm module packaged into this Rust crate that contains
//! pre-compiled ICU4C [`CodePointTrie`] builder code. It evaluates the Wasm module using
//! the Wasmer runtime, which "just works", but it requires a large number of
//! Rust/Cargo dependencies.
//!
//! The `"icu4c"` mode reduces the number of Rust dependencies, but it requires having a local copy
//! of ICU4C available. To configure `"icu4c"` mode in Cargo, set the following environment variables:
//!
//! - Set `ICU4C_LIB_PATH` to a directory full of ICU4C static or shared libraries.
//! - Set `ICU4C_LINK_STATICALLY` to any value to use the static libraries.
//! - Set `ICU4C_RENAME_VERSION` to the integer ICU4C version if ICU4C has renaming
//!   enabled. By default, we attempt to link non-renamed symbols.
//!
//! If using dynamic linking, at runtime, you may need to set `[DY]LD_LIBRARY_PATH`
//! to the `ICU4C_LIB_PATH`.
//!
//! If _not_ using Cargo, make sure to pass `ICU4C_LIB_PATH` to the linker via `-L`, link against
//! `icuuc`, `icui18n` and `icudata` via `-l` flags, and set `--cfg icu4c_enable_renaming` if you need
//! renamed ICU4C symbols.
//!
//! # Examples
//!
//! ```
//! use icu::collections::codepointtrie::TrieType;
//! use icu_codepointtrie_builder::CodePointTrieBuilder;
//!
//! let default_value = 1u8;
//! let error_value = 2;
//!
//! let mut builder = CodePointTrieBuilder::new(default_value, error_value, TrieType::Small);
//! builder.set_value(0, 3);
//! builder.set_value(1, 4);
//! builder.set_value(2, 5);
//! builder.set_value(3, 6);
//! let cpt = builder.build();
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
//! [`CodePointTrie`]: icu_collections::codepointtrie::CodePointTrie
//! [`UMutableCPTrie`]: (https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/umutablecptrie_8h.html#ad8945cf34ca9d40596a66a1395baa19b)

#[cfg(any(feature = "wasm", feature = "icu4c"))]
use core::ops::RangeInclusive;

#[cfg(any(feature = "wasm", feature = "icu4c"))]
use icu_collections::codepointtrie::CodePointTrie;
#[cfg(any(feature = "wasm", feature = "icu4c"))]
use icu_collections::codepointtrie::TrieType;
#[cfg(any(feature = "wasm", feature = "icu4c"))]
use icu_collections::codepointtrie::TrieValue;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "icu4c")]
mod native;

#[cfg(feature = "wasm")]
use wasm::Builder;

#[cfg(all(feature = "icu4c", not(feature = "wasm")))]
use native::Builder;

/// Builder for a [`CodePointTrie`].
///
/// Under the hood, this runs ICU4C code compiled into WASM,
/// or links natively to ICU4C as specified by the `ICU4C_LIB_PATH` env var
///
/// ✨ *Enabled with either the `wasm` or the `icu4c` Cargo feature.*
#[allow(clippy::exhaustive_structs)]
#[derive(Debug)]
#[cfg(any(feature = "wasm", feature = "icu4c"))]
pub struct CodePointTrieBuilder<T: TrieValue> {
    inner: Builder<T>,
    trie_type: TrieType,
    default_value: T,
}

#[cfg(any(feature = "wasm", feature = "icu4c"))]
impl<T: TrieValue> CodePointTrieBuilder<T> {
    /// Creates a new [`CodePointTrieBuilder`] with the given defaults.
    pub fn new(default_value: T, error_value: T, trie_type: TrieType) -> Self {
        Self {
            inner: Builder::create(default_value, error_value),
            trie_type,
            default_value,
        }
    }

    /// Sets a value for a codepoint.
    #[cfg(any(feature = "wasm", feature = "icu4c"))]
    pub fn set_value(&mut self, cp: u32, value: T) {
        if value == self.default_value || cp > char::MAX as u32 {
            return;
        }
        self.inner.set_value(cp, value);
    }

    /// Adds a set of codepoints with the same value.
    #[cfg(any(feature = "wasm", feature = "icu4c"))]
    pub fn set_range_value(&mut self, cps: RangeInclusive<u32>, value: T) {
        if value == self.default_value {
            return;
        }
        self.inner.set_range_value(
            (*cps.start()).min(char::MAX as u32)..=(*cps.end()).min(char::MAX as u32),
            value,
        );
    }

    /// Build the [`CodePointTrie`].
    #[cfg(any(feature = "wasm", feature = "icu4c"))]
    pub fn build(self) -> CodePointTrie<'static, T> {
        let width = match size_of::<T::ULE>() {
            1 => 2,     // UCPTRIE_VALUE_BITS_8
            2 => 0,     // UCPTRIE_VALUE_BITS_16
            3 | 4 => 1, // UCPTRIE_VALUE_BITS_32
            other => panic!("Don't know how to make trie with width {other}"),
        };

        self.inner.build(self.trie_type, width)
    }
}

#[test]
#[cfg(any(feature = "wasm", feature = "icu4c"))]
fn test_cpt_builder() {
    let mut builder = CodePointTrieBuilder::new(100, 0xFFF, TrieType::Fast);

    // Buckets of ten characters for 0 to 100, and then some default values, and then heterogenous "last hex digit" for 0x100 to 0x200
    for (cp, value) in (0..100)
        .map(|x| x / 10)
        .chain((100..0x100).map(|_| 100))
        .chain((0x100..0x200).map(|x| x % 16))
        .enumerate()
    {
        builder.set_value(cp as u32, value);
    }

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
