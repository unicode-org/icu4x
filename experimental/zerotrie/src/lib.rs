// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A data structure offering zero-copy storage and retrieval of byte strings, with a focus
//! on the efficient storage of ASCII strings. Strings are mapped to `usize` values.
//!
//! [`ZeroTrie`] does not support mutation because doing so would require recomputing the entire
//! data structure. Instead, it supports conversion to and from [`LiteMap`] and [`BTreeMap`].
//!
//! There are multiple variants of [`ZeroTrie`] optimized for different use cases.
//!
//! # Examples
//!
//! ```
//! use zerotrie::ZeroTrie;
//!
//! let data: &[(&str, usize)] = &[
//!     ("abc", 11),
//!     ("xyz", 22),
//!     ("axyb", 33),
//! ];
//!
//! let trie: ZeroTrie<Vec<u8>> = data.iter().copied().collect();
//!
//! assert_eq!(trie.get("axyb"), Some(33));
//! assert_eq!(trie.byte_len(), 18);
//! ```
//!
//! # Internal Structure
//!
//! To read about the internal structure of [`ZeroTrie`], build the docs with private modules:
//!
//! ```bash
//! cargo doc --document-private-items --all-features --no-deps --open
//! ```
//!
//! [`LiteMap`]: litemap::LiteMap
//! [`BTreeMap`]: alloc::collections::BTreeMap

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod builder;
mod byte_phf;
mod error;
mod helpers;
mod reader;
#[cfg(feature = "serde")]
mod serde;
mod varint;
mod zerotrie;

pub use crate::zerotrie::ZeroTrie;
pub use crate::zerotrie::ZeroTrieExtendedCapacity;
pub use crate::zerotrie::ZeroTriePerfectHash;
pub use crate::zerotrie::ZeroTrieSimpleAscii;
pub use error::Error as ZeroTrieError;

#[doc(hidden)]
pub mod _internal {
    pub use crate::byte_phf::f1;
    pub use crate::byte_phf::f2;
    pub use crate::byte_phf::PerfectByteHashMap;
}
