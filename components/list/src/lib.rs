// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![warn(missing_docs)]

//! [`icu_list`](crate) provides the [`ListFormatter`] which renders sequences of [`Writeable`](
//! writeable::Writeable)s as lists in a locale-sensitive way.
//!
//! # Examples
//!
//! ## Format a list of strings in Spanish
//!
//! ```
//! use icu_list::{ListFormatter, ListStyle};
//! use icu_locid::locale;
//! use writeable::Writeable;
//!
//! let list_formatter = ListFormatter::try_new_and(
//!     locale!("es"),
//!     &icu_testdata::get_static_provider(),
//!     ListStyle::Wide,
//! )
//! .expect("Data should load successfully");
//!
//! assert_eq!(
//!     list_formatter.format(["España", "Suiza"].iter())
//!         .write_to_string(),
//!     "España y Suiza"
//! );
//!
//! // The Spanish 'y' sometimes becomes an 'e':
//! assert_eq!(
//!     list_formatter.format(["España", "Suiza", "Italia"].iter())
//!         .write_to_string(),
//!     "España, Suiza e Italia"
//! );
//!
//! // We can use any Writeables as inputs:
//! assert_eq!(
//!     list_formatter.format(1..=10).write_to_string(),
//!     "1, 2, 3, 4, 5, 6, 7, 8, 9 y 10"
//! );
//!```
//!
//! [`ListFormatter`]: ListFormatter

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    not(test),
    deny(
        // TODO(#1668) Clippy exceptions need docs or fixing
        // clippy::indexing_slicing,
        // clippy::unwrap_used,
        // clippy::expect_used,
        // clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums
    )
)]

extern crate alloc;

mod list_formatter;
mod string_matcher;

pub mod provider;

pub use list_formatter::*;

/// Represents the style of a list. See the
/// [CLDR spec](https://unicode.org/reports/tr35/tr35-general.html#ListPatterns)
/// for an explanation of the different styles.
#[derive(Copy, Clone, PartialEq, Debug)]
#[non_exhaustive]
pub enum ListStyle {
    /// A typical list
    Wide,
    /// A shorter list
    Short,
    /// The shortest type of list
    Narrow,
}
