// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu` is the main meta-crate of the `ICU4X` project.
//!
//! It provides a comprehensive selection of
//! [Unicode Internationalization Components](http://site.icu-project.org/)
//! in their canonical configurations intended to enable software
//! internationalization capabilities.
//!
//! This crate does not bring any unique functionality. Each module is also
//! available as a stand-alone crate, i.e. `icu::list` as `icu_list`.
//!
//! # Data Management
//!
//! Most functionality relies on data which clients have to provide to the APIs.
//!
//! `ICU4X` uses the concept of a [`DataProvider`] to separate data from logic.
//! Data providers come in many different forms; the following providers are provided
//! by `ICU4X` in separate crates:
//! * [`BlobDataProvider`]: uses an in-memory serde-serialized blob. This is the most flexible provider, and
//!   data can be updated at runtime.
//! * `BakedDataProvider`: a code-generated provider that contains the data directly in Rust code. This is
//!   the most efficient provider as it's serialization-free, and allows for compile-time optimizations.
//! * [`FsDataProvider`]: uses a file system tree of Serde files. This is mostly useful for development and
//!   not recommended in production for performance reasons.
//! * [`icu_provider_adapters`]: this crate contains APIs to combine providers or
//!   provide additional functionality such as locale fallback.
//!
//! The data that is required by these providers (in `BakedDataProvider`'s case, the provider itself) can be
//! generated and customized using the [`icu_datagen`] crate.
//!
//! The following example uses the [`icu_testdata`] crate, which contains prepackaged data providers
//! for a small set of locales.
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
//! let dtf = DateTimeFormatter::try_new(&locale!("en").into(), &provider, &options)
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
//! # Features
//!
//! ICU4X components share a set of common features that control whether core pieces of
//! functionality are compiled. These features are:
//!
//! - `std`: Whether to include `std` support. Without this feature, `icu` is `#[no_std]`-compatible
//! - `serde`: Whether to include `serde::Deserialize` implementations for data structs, such as [`SymbolsV1`],
//!   and `serde::{Serialize, Deserialize}` implementations for core libary types, such as [`Locale`]. These are
//!   required with `serde`-backed providers like [`BlobDataProvider`][^1].
//! - `experimental`: Whether to enable experimental preview features. Modules enabled with
//!   this feature may not be production-ready and could change at any time.
//!
//! The following features are only available on the individual crates, but not on this meta-crate:
//! - `datagen`: Whether to implement `serde::Serialize` and functionality that is only required during data generation.
//! - `bench`: Whether to enable exhaustive benchmarks. This can be enabled on individual crates
//!   when running `cargo bench`.
//!
//! [^1]: [`FsDataProvider`] also requires the `serde_human` feature if JSON is used, as that data is less
//!       preprocessed.
//!
//!
//! [`DataProvider`]: ../icu_provider/prelude/trait.DataProvider.html
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`BlobDataProvider`]: ../icu_provider_blob/struct.BlobDataProvider.html
//! [`icu_testdata`]: ../icu_testdata/index.html
//! [`icu_provider_adapters`]: ../icu_provider_adapters/index.html
//! [`icu_datagen`]: ../icu_datagen/index.html
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
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

#[doc(inline)]
pub use icu_calendar as calendar;

#[doc(inline)]
pub use icu_collator as collator;

#[doc(inline)]
pub use icu_datetime as datetime;

#[doc(inline)]
pub use icu_decimal as decimal;

#[doc(inline)]
pub use icu_list as list;

#[doc(inline)]
pub use icu_locale_canonicalizer as locale_canonicalizer;

#[doc(inline)]
pub use icu_locid as locid;

#[doc(inline)]
pub use icu_normalizer as normalizer;

#[doc(inline)]
pub use icu_plurals as plurals;

#[doc(inline)]
pub use icu_properties as properties;

#[cfg(feature = "experimental")]
#[doc(inline)]
pub use icu_segmenter as segmenter;
