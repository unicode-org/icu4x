// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu` is the main meta-package of the `ICU4X` project.
//!
//! It provides a comprehensive selection of
//! [Unicode Internationalization Components](http://site.icu-project.org/)
//! in their canonical configurations intended to enable software
//! internationalization capabilities.
//!
//! The package is provided for convenience and users are encouraged
//! to fine-tune components with the features they need.
//!
//! The package does not bring any unique functionality. Users
//! can achieve the exact same by manually including the dependent
//! components with pre-selected features.
//!
//! # Data Management
//!
//! Most of Unicode functionality relies on data which has to be provided
//! to the APIs.
//!
//! `ICU4X` project uses a concept of [`DataProvider`] - a service used to
//! handle data management.
//!
//! There can be many different heuristics for handling data management and
//! this meta-package does not supply any default [`DataProvider`].
//!
//! When using `ICU4X` users are expected to decide which provider they want to use
//! and instrument it to point at the correct location where the data files are stored.
//!
//! In the following examples an [`icu_testdata`] package is used which wraps
//! an [`FsDataProvider`] with locally available subset of data.
//!
//! # Features
//!
//! ICU4X components share a set of common features that control whether core pieces of
//! functionality are compiled. These features are:
//!
//! - `serialize`: Whether to include Serde Deserialize implementations for
//!   ICU4X locale data structs, such as [`SymbolsV1`], and Serialize/Deserialize implementations
//!   for core libary types, such as [`Locale`] (On by default)
//! - `datagen`: Whether to include Serde Serialize and other data generation traits for ICU4X locale data structs.
//! - `bench`: Whether to enable exhaustive benchmarks. This can be enabled on individual crates
//!   when running `cargo bench`.
//! - `experimental`: Whether to enable experimental preview features. Modules enabled with
//!   this feature may not be production-ready and could change at any time.
//!
//! # Example
//!
//! ```
//! use icu::datetime::{mock::parse_gregorian_from_str, options::length, DateTimeFormatter};
//! use icu::locid::locale;
//!
//! let provider = icu_testdata::get_provider();
//!
//! let options =
//!     length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium).into();
//!
//! let dtf = DateTimeFormatter::try_new(locale!("en"), &provider, &options)
//!     .expect("Failed to create DateTimeFormatter instance.");
//!
//! let date = parse_gregorian_from_str("2020-09-12T12:35:00").expect("Failed to parse date.");
//!
//! let formatted_date = dtf.format(&date);
//! assert_eq!(
//!     formatted_date.to_string(),
//!     "September 12, 2020 at 12:35:00 PM"
//! );
//! ```
//!
//! [`DataProvider`]: ../icu_provider/prelude/trait.DataProvider.html
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`icu_testdata`]: ../icu_testdata/index.html
//! [`Locale`]: crate::locid::Locale
//! [`SymbolsV1`]: crate::decimal::provider::DecimalSymbolsV1

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

#[doc(inline)]
pub use icu_calendar as calendar;

#[doc(inline)]
pub use icu_datetime as datetime;

#[doc(inline)]
pub use icu_decimal as decimal;

#[doc(inline)]
pub use icu_locale_canonicalizer as locale_canonicalizer;

#[doc(inline)]
pub use icu_locid as locid;

#[doc(inline)]
pub use icu_plurals as plurals;

#[doc(inline)]
pub use icu_properties as properties;

#[doc(inline)]
pub use icu_list as list;
