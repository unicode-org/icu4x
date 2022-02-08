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
//! use icu_locid::Locale;
//! use icu_locid_macros::langid;
//! use writeable::Writeable;
//!
//! let locale: Locale = langid!("es").into();
//! let provider = icu_testdata::get_provider();
//! let list_formatter = ListFormatter::try_new_and(locale, &provider, ListStyle::Wide)
//!     .expect("Data should load successfully");
//!
//! assert_eq!(
//!     list_formatter.format(["España", "Suiza"].iter())
//!         .writeable_to_string(),
//!     "España y Suiza"
//! );
//!
//! // The Spanish 'y' sometimes becomes an 'e':
//! assert_eq!(
//!     list_formatter.format(["España", "Suiza", "Italia"].iter())
//!         .writeable_to_string(),
//!     "España, Suiza e Italia"
//! );
//!
//! // We can use any Writeables as inputs:
//! assert_eq!(
//!     list_formatter.format(1..=10).writeable_to_string(),
//!     "1, 2, 3, 4, 5, 6, 7, 8, 9 y 10"
//! );
//!```
//!
//! [`ListFormatter`]: ListFormatter

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod list_formatter;
#[cfg(feature = "provider_transform_internals")]
pub mod provider;
#[cfg(not(feature = "provider_transform_internals"))]
mod provider;
mod string_matcher;

pub use list_formatter::*;

/// Represents the style of a list. See the
/// [CLDR spec](https://unicode.org/reports/tr35/tr35-general.html#ListPatterns)
/// for an explanation of the different styles.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ListStyle {
    /// A typical list
    Wide,
    /// A shorter list
    Short,
    /// The shortest type of list
    Narrow,
}
