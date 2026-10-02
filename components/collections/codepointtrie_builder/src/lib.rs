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
//! Under the hood, this crate uses a pure-Rust port of the [`CodePointTrie`] builder code
//! from ICU4C, [`UMutableCPTrie`].
//! For more context, see <https://github.com/unicode-org/icu4x/issues/1837>.
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
//! let mut builder =
//!     CodePointTrieBuilder::new(default_value, error_value, TrieType::Small);
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

use core::ops::RangeInclusive;

use icu_collections::codepointtrie::CodePointTrie;
use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;

mod rust;

#[cfg(test)]
mod wasm;

use rust::Builder;

/// Builder for a [`CodePointTrie`].
#[allow(clippy::exhaustive_structs)]
#[derive(Debug)]
pub struct CodePointTrieBuilder<T: TrieValue> {
    inner: Builder<T>,
    trie_type: TrieType,
    default_value: T,
}

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
    pub fn set_value(&mut self, cp: u32, value: T) {
        if value == self.default_value || cp > char::MAX as u32 {
            return;
        }
        self.inner.set_value(cp, value);
    }

    /// Adds a set of codepoints with the same value.
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
fn test_cpt_builder() {
    let mut builder = CodePointTrieBuilder::new(100, 0xFFF, TrieType::Fast);

    // Buckets of ten characters for 0 to 0x100, and then some default values, and then heterogenous "last hex digit" for 0x100 to 0x200
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

#[test]
fn test_rust_vs_wasm_identical() {
    for trie_type in [TrieType::Fast, TrieType::Small] {
        // Test 1: 8-bit values across BMP and supplementary planes, including ranges and mixed blocks
        let mut rust_b = Builder::<u8>::create(0, 0xFF);
        let mut wasm_b = wasm::Builder::<u8>::create(0, 0xFF);

        for cp in (0..0x10FFFF).step_by(113) {
            let v = ((cp * 31 + 7) & 0xFF) as u8;
            rust_b.set_value(cp, v);
            wasm_b.set_value(cp, v);
        }
        for chunk in (0x1000..0x50000).step_by(0x700) {
            let v = ((chunk >> 8) & 0xFF) as u8;
            rust_b.set_range_value(chunk..=(chunk + 0x155), v);
            wasm_b.set_range_value(chunk..=(chunk + 0x155), v);
        }
        assert_eq!(rust_b.build(trie_type, 2), wasm_b.build(trie_type, 2));

        // Test 2: 16-bit values with >32 distinct ALL_SAME blocks (triggers AllSameBlocks overflow)
        let mut rust_b16 = Builder::<u16>::create(1, 0xFFFF);
        let mut wasm_b16 = wasm::Builder::<u16>::create(1, 0xFFFF);
        for i in 0..50u32 {
            let start = i * 64;
            let v = (i as u16) + 10;
            rust_b16.set_range_value(start..=(start + 63), v);
            wasm_b16.set_range_value(start..=(start + 63), v);
        }
        for cp in 0x10000..0x12000 {
            let v = ((cp * 17) & 0xFFFF) as u16;
            rust_b16.set_value(cp, v);
            wasm_b16.set_value(cp, v);
        }
        assert_eq!(rust_b16.build(trie_type, 0), wasm_b16.build(trie_type, 0));

        // Test 3: 32-bit values with large data table (> 64k entries, triggers 18-bit index-3 blocks)
        let mut rust_b32 = Builder::<u32>::create(0, 0xDEAD_BEEF);
        let mut wasm_b32 = wasm::Builder::<u32>::create(0, 0xDEAD_BEEF);
        for cp in 0..0x20000u32 {
            let v = cp.wrapping_mul(2654435761);
            rust_b32.set_value(cp, v);
            wasm_b32.set_value(cp, v);
        }
        assert_eq!(rust_b32.build(trie_type, 1), wasm_b32.build(trie_type, 1));
    }
}
