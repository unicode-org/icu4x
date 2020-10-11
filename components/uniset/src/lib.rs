// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! # UnicodeSet
//! This crate is the [ICU4X](https://github.com/unicode-org/icu4x) implementation of the existing [ICU4C UnicodeSet API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).
//!
//! ## Purpose
//! UnicodeSet is designed to allow for highly efficient querying of sets of Unicode characters.
//!
//! # Architecture
//! ICU4X UnicodeSet is split up into independent levels, with [`UnicodeSet`](struct.UnicodeSet.html) representing the membership/query API,
//! and [`UnicodeSetBuilder`](struct.UnicodeSetBuilder.html) representing the builder API. A [Properties API](http://userguide.icu-project.org/strings/properties)
//! is in future works.
//!
//! # Examples:
//!
//! ## Creating a UnicodeSet
//!
//! UnicodeSets are created from either serialized UnicodeSets,
//! represented by [inversion lists](http://userguide.icu-project.org/strings/properties),
//! the [`UnicodeSetBuilder`](struct.UnicodeSetBuilder.html), or from the TBA Properties API.
//!
//! ```
//! use icu_uniset::UnicodeSet;
//! let inv_list = vec![65, 70]; // 'A'..'F'
//! let set = UnicodeSet::from_inversion_list(inv_list);
//! assert!(set.is_ok());
//!
//! use icu_uniset::UnicodeSetBuilder;
//! let mut builder = UnicodeSetBuilder::new();
//! builder.add_range(&('A'..'Z'));
//! let set: UnicodeSet = builder.build();
//! assert!(set.contains('A'));
//! ```
//!
//! ## Querying the UnicodeSet
//!
//! Currently, you can check if a character/range of characters exists in the UnicodeSet, or iterate through the characters.
//!
//! ```
//! use icu_uniset::UnicodeSet;
//! let inv_list = vec![65, 70];  // 'A'..'F'
//! let set = UnicodeSet::from_inversion_list(inv_list).unwrap();
//! assert!(set.contains('A'));
//! assert!(set.contains_range(&('A'..='C')));
//! assert_eq!(set.iter().next(), Some('A'));
//! ```

#[macro_use]
mod uniset;
mod builder;
mod conversions;
mod utils;

pub use builder::UnicodeSetBuilder;
pub use conversions::*;
pub use uniset::UnicodeSet;
pub use utils::*;

/// Custom Errors for UnicodeSet.
#[derive(Debug, PartialEq)]
pub enum UnicodeSetError {
    InvalidSet(Vec<u32>),
    InvalidRange(u32, u32),
}

#[derive(PartialEq)]
pub enum UnicodeSetSpanCondition {
    Contained,
    NotContained,
}
