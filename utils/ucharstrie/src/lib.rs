// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # icu_ucharstrie [![crates.io](http://meritbadge.herokuapp.com/icu_ucharstrie)](https://crates.io/crates/icu_ucharstrie)
//!
//! `icu_ucharstrie` is a utility crate of the [`ICU4X`] project.
//!
//! This component provides a data structure for an time-efficient lookup of values
//! associated to code points.
//!
//! It is an implementation of the existing [ICU4C UCharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UCharsTrie.html)
//! / [ICU4J UCharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/util/CharsTrie.html) API.
//!
//! ## Architecture
//!
//! ICU4X [`UCharsTrie`](crate::ucharstrie::UCharsTrie) is designed to provide a read-only view of UCharTrie data that is exported from ICU4C.
//!
//! ## Examples
//!
//! ### Querying a `CodePointTrie`
//!
//! ```rust
//! // TODO!
//! ```

//#![no_std]

pub mod trie;
pub mod ucharstrie;
