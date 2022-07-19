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
//! ICU4X [`CodePointSet`] is split up into independent levels, with [`CodePointSet`] representing the membership/query API,
//! and [`CodePointSetBuilder`] representing the builder API. A [Properties API](http://userguide.icu-project.org/strings/properties)
//! is in future works.
//!
//! # Examples:
//!
//! ## Creating a `CodePointSet`
//!
//! CodePointSets are created from either serialized [`CodePointSets`](CodePointSet),
//! represented by [inversion lists](http://userguide.icu-project.org/strings/properties),
//! the [`CodePointSetBuilder`], or from the TBA Properties API.
//!
//! ```
//! use icu_uniset::{CodePointSet, CodePointSetBuilder};
//!
//! let mut builder = CodePointSetBuilder::new();
//! builder.add_range(&('A'..'Z'));
//! let set: CodePointSet = builder.build();
//!
//! assert!(set.contains('A'));
//! ```
//!
//! ## Querying a `CodePointSet`
//!
//! Currently, you can check if a character/range of characters exists in the [`CodePointSet`], or iterate through the characters.
//!
//! ```
//! use icu_uniset::{CodePointSet, CodePointSetBuilder};
//!
//! let mut builder = CodePointSetBuilder::new();
//! builder.add_range(&('A'..'Z'));
//! let set: CodePointSet = builder.build();
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
        clippy::panic
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

pub use builder::CodePointSetBuilder;
pub use conversions::*;
use displaydoc::Display;
pub use uniset::CodePointSet;
pub use utils::*;

/// Custom Errors for [`CodePointSet`].
#[derive(Display, Debug)]
pub enum CodePointSetError {
    /// A CodePointSet was constructed with an invalid inversion list
    #[displaydoc("Invalid set: {0:?}")]
    InvalidSet(Vec<u32>),
    /// A CodePointSet was constructed containing an invalid range
    #[displaydoc("Invalid range: {0}..{1}")]
    InvalidRange(u32, u32),
}

#[cfg(feature = "std")]
impl std::error::Error for CodePointSetError {}
