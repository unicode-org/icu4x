// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! [`icu_list`](crate) provides the [`ListFormatter`] which renders sequences of [`Writeable`](
//! writeable::Writeable)s as lists in a locale-sensitive way.
//!
//! # Examples
//!
//! ## Formatting *and* lists in Spanish
//!
//! ```
//! # use icu_list::{ListFormatter, ListStyle};
//! # use icu_locid::locale;
//! # use writeable::*;
//! #
//! let list_formatter = ListFormatter::try_new_and(
//!     locale!("es"),
//!     &icu_testdata::get_provider(),
//!     ListStyle::Wide,
//! )
//! .expect("Data should load successfully");
//!
//! assert_writeable_eq!(
//!     list_formatter.format(["España", "Suiza"].iter()),
//!     "España y Suiza",
//! );
//!
//! // The Spanish 'y' sometimes becomes an 'e':
//! assert_writeable_eq!(
//!     list_formatter.format(["España", "Suiza", "Italia"].iter()),
//!     "España, Suiza e Italia",
//! );
//! ```
//!
//! ## Formatting *or* lists in Thai
//!
//! ```
//! # use icu_list::{ListFormatter, ListStyle};
//! # use icu_locid::locale;
//! # use writeable::*;
//! #
//! let list_formatter = ListFormatter::try_new_or(
//!     locale!("th"),
//!     &icu_testdata::get_provider(),
//!     ListStyle::Short,
//! )
//! .expect("Data should load successfully");
//!
//! // We can use any Writeables as inputs
//! assert_writeable_eq!(
//!     list_formatter.format(1..=3),
//!     "1, 2 หรือ 3",
//! );
//! ```
//!
//! ## Formatting unit lists in English
//!
//! ```
//! # use icu_list::{ListFormatter, ListStyle};
//! # use icu_locid::locale;
//! # use writeable::*;
//! #
//! let list_formatter = ListFormatter::try_new_unit(
//!     locale!("en"),
//!     &icu_testdata::get_provider(),
//!     ListStyle::Wide,
//! )
//! .expect("Data should load successfully");
//!
//! assert_writeable_eq!(
//!     list_formatter.format(["1ft", "2in"].iter()),
//!     "1ft, 2in",
//! );
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums
    )
)]
#![warn(missing_docs)]

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
    // *Important*: When adding a variant here, make sure the code in
    // ListFormatterPatterns::{start, middle, end, pair} stays panic-free!
}
