// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_uniset` is a utility crate of the [`ICU4X`] project.
//!
//! This API provides necessary functionality for highly efficient querying of sets of Unicode characters.
//!
//! It is an implementation of the existing [ICU4C UnicodeSet API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).
//!
//! # Architecture
//! ICU4X [`CodePointInversionList`] is split up into independent levels, with [`CodePointInversionList`] representing the membership/query API,
//! and [`CodePointInversionListBuilder`] representing the builder API.
//!
//! # Examples:
//!
//! ## Creating a `CodePointInversionList`
//!
//! CodePointSets are created from either serialized [`CodePointSets`](CodePointInversionList),
//! represented by [inversion lists](http://userguide.icu-project.org/strings/properties),
//! the [`CodePointInversionListBuilder`], or from the Properties API.
//!
//! ```
//! use icu_uniset::{CodePointInversionList, CodePointInversionListBuilder};
//!
//! let mut builder = CodePointInversionListBuilder::new();
//! builder.add_range(&('A'..'Z'));
//! let set: CodePointInversionList = builder.build();
//!
//! assert!(set.contains('A'));
//! ```
//!
//! ## Querying a `CodePointInversionList`
//!
//! Currently, you can check if a character/range of characters exists in the [`CodePointInversionList`], or iterate through the characters.
//!
//! ```
//! use icu_uniset::{CodePointInversionList, CodePointInversionListBuilder};
//!
//! let mut builder = CodePointInversionListBuilder::new();
//! builder.add_range(&('A'..'Z'));
//! let set: CodePointInversionList = builder.build();
//!
//! assert!(set.contains('A'));
//! assert!(set.contains_range(&('A'..='C')));
//! assert_eq!(set.iter_chars().next(), Some('A'));
//! ```
//!
//! [`ICU4X`]: ../icu/index.html

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        // TODO(#1668): enable clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

#[macro_use]
mod builder;
mod conversions;
mod uniset;
mod utils;

use alloc::vec::Vec;

pub use builder::CodePointInversionListBuilder;
pub use conversions::*;
use displaydoc::Display;
pub use uniset::CodePointInversionList;
pub use utils::*;

/// Custom Errors for [`CodePointInversionList`].
#[derive(Display, Debug)]
pub enum CodePointSetError {
    /// A CodePointInversionList was constructed with an invalid inversion list
    #[displaydoc("Invalid set: {0:?}")]
    InvalidSet(Vec<u32>),
    /// A CodePointInversionList was constructed containing an invalid range
    #[displaydoc("Invalid range: {0}..{1}")]
    InvalidRange(u32, u32),
}

#[cfg(feature = "std")]
impl std::error::Error for CodePointSetError {}
