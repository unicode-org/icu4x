// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_uniset` is one of the [`ICU4X`] components.
//!
//! This API provides necessary functionality for highly efficient querying of sets of Unicode characters.
//!
//! It is an implementation of the existing [ICU4C UnicodeSet API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).
//!
//! # Architecture
//! ICU4X [`UnicodeSet`] is split up into independent levels, with [`UnicodeSet`] representing the membership/query API,
//! and [`UnicodeSetBuilder`] representing the builder API. A [Properties API](http://userguide.icu-project.org/strings/properties)
//! is in future works.
//!
//! # Examples:
//!
//! ## Creating a `UnicodeSet`
//!
//! UnicodeSets are created from either serialized [`UnicodeSets`](UnicodeSet),
//! represented by [inversion lists](http://userguide.icu-project.org/strings/properties),
//! the [`UnicodeSetBuilder`], or from the TBA Properties API.
//!
//! ```
//! use icu::uniset::{UnicodeSet, UnicodeSetBuilder};
//!
//! let mut builder = UnicodeSetBuilder::new();
//! builder.add_range(&('A'..'Z'));
//! let set: UnicodeSet = builder.build();
//!
//! assert!(set.contains('A'));
//! ```
//!
//! ## Querying a `UnicodeSet`
//!
//! Currently, you can check if a character/range of characters exists in the [`UnicodeSet`], or iterate through the characters.
//!
//! ```
//! use icu::uniset::{UnicodeSet, UnicodeSetBuilder};
//!
//! let mut builder = UnicodeSetBuilder::new();
//! builder.add_range(&('A'..'Z'));
//! let set: UnicodeSet = builder.build();
//!
//! assert!(set.contains('A'));
//! assert!(set.contains_range(&('A'..='C')));
//! assert_eq!(set.iter_chars().next(), Some('A'));
//! ```
//!
//! [`ICU4X`]: ../icu/index.html

#[macro_use]
mod builder;
mod conversions;
pub mod enum_props;
pub mod props;
pub mod provider;
mod uniset;
mod utils;

pub use builder::UnicodeSetBuilder;
pub use conversions::*;
use icu_provider::DataError;
use thiserror::Error;
pub use uniset::UnicodeSet;
pub use utils::*;

/// Custom Errors for [`UnicodeSet`].
#[derive(Error, Debug)]
pub enum UnicodeSetError {
    #[error("Invalid set: {0:?}")]
    InvalidSet(Vec<u32>),
    #[error("Invalid range: {0}..{1}")]
    InvalidRange(u32, u32),
    #[error(transparent)]
    PropDataLoad(#[from] DataError),
}

#[derive(PartialEq)]
pub enum UnicodeSetSpanCondition {
    Contained,
    NotContained,
}
