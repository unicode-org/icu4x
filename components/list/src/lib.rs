// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Formatting lists in a locale-sensitive way.
//!
//! This module is published as its own crate ([`icu_list`](https://docs.rs/icu_list/latest/icu_list/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
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
//!     &locale!("es").into(),
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
//!     &locale!("th").into(),
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
//!     &locale!("en").into(),
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
//! Note: this last example is not fully internationalized. See [icu4x/2192](https://github.com/unicode-org/icu4x/issues/2192)
//! for full unit handling.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
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
