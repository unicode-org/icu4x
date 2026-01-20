// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        clippy::trivially_copy_pass_by_ref,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

//! `icu` is the main meta-crate of the `ICU4X` project.
//!
//! It provides a comprehensive selection of functionality found in
//! [International Components for Unicode](https://icu.unicode.org/)
//! in their canonical configurations intended to enable software
//! internationalization capabilities.
//!
//! This crate exists to collect the most important functionality for users
//! together in one place.
//! It does not bring any unique functionality, but rather,
//! it re-exports the relevant crates as modules.
//! The exported crate corresponding to each module is also
//! available in a stand-alone manner, i.e. `icu::list` as `icu::list`.
//!
//! # Data Management
//!
//! Most internationalization algorithms are data-driven based on surveys of locale experts.
//! ICU4X offers multiple ways to manage locale data: many clients can start by using the
//! extensive data compiled into the library, while users with additional requirements can
//! provide data explicitly using [`DataProvider`]s.
//!
//! ## Compiled data
//!
//! Compiled data is exposed through idiomatic Rust constructors like `new` or `try_new`:
//!
//! ```
//! use icu::datetime::{fieldsets::YMD, DateTimeFormatter};
//! use icu::locale::locale;
//!
//! let dtf =
//!     DateTimeFormatter::try_new(locale!("es-US").into(), YMD::medium())
//!         .expect("compiled data should include 'es-US'");
//! ```
//!
//! Clients using compiled data benefit from simple code and optimal zero-cost data loading. Additionally,
//! ICU4X's APIs are designed such that dead-code elimination can optimize away unused compiled data.
//!
//! By default, most of the data available in [CLDR] is included. Users can customize data by using
//! the `icu4x-datagen` tool (with the `-⁠-format mod` flag) to, for example, select a smaller set of
//! locales, and then compiling with the `ICU4X_DATA_DIR` variable.
//!
//! ## Explicit data
//!
//! Powerful data management is possible with [`DataProvider`]s, which are passed to ICU4X APIs via
//! special constructors:
//!
//! ```no_run
//! use icu::datetime::{fieldsets::YMD, DateTimeFormatter};
//! use icu::locale::fallback::LocaleFallbacker;
//! use icu::locale::locale;
//! use icu_provider_adapters::fallback::LocaleFallbackProvider;
//! use icu_provider_blob::BlobDataProvider;
//!
//! let data: Box<[u8]> = todo!();
//!
//! let provider = BlobDataProvider::try_new_from_blob(data)
//!     .expect("data should be valid");
//!
//! let fallbacker = LocaleFallbacker::try_new_with_buffer_provider(&provider)
//!     .expect("provider should include fallback data");
//!
//! let provider = LocaleFallbackProvider::new(provider, fallbacker);
//!
//! let dtf = DateTimeFormatter::try_new_with_buffer_provider(
//!     &provider,
//!     locale!("es-US").into(),
//!     YMD::medium(),
//! )
//! .expect("data should include 'es-US', 'es', or 'und'");
//! ```
//!
//! Explicit data management can be used if the compiled-data constructors are too limiting. It allows:
//! * Accessing data without fallback
//! * Custom [`DataProvider`]s backed by sources like the operating system
//! * Lazily loading or updating data from I/O
//! * Composing data providers from different sources
//! * Manually including/excluding data
//! * ... and more. See our [data management tutorial]
//!
//! The following [`DataProvider`]s are available in separate crates:
//! * [`BlobDataProvider`]: deserializes data from an in-memory blob, which can be updated at runtime.
//! * `BakedDataProvider`: a code-generated provider that contains the data directly in Rust code. This is the
//!   same provider that is used internally by compiled data.
//! * [`FsDataProvider`]: uses a file system tree of Serde files. This is mostly useful for development and
//!   not recommended in production for performance reasons.
//! * [`icu_provider_adapters`]: this crate contains provider adapters to combine providers,
//!   provide additional functionality such as locale fallback, and more.
//!
//! The data that is required by these providers (in `BakedDataProvider`'s case, the provider itself) can be
//! generated and customized using the [`icu4x-datagen`] tool.
//!
//! # Features
//!
//! ICU4X components share a set of common Cargo features that control whether core pieces of
//! functionality are compiled. These features are:
//!
//! - `compiled_data` (default): Whether to include compiled data. Without this flag, only constructors with
//!   explicit `provider` arguments are available.
//! - `datagen`: Whether to implement functionality that is only required during data generation.
//! - `logging`: Enables logging through the `log` crate.
//! - `serde`: Activates `serde` implementations for core library types, such as [`Locale`], as well
//!   as `*_with_buffer_provider` constructors for runtime data management.
//! - `sync`: makes most ICU4X objects implement `Send + Sync`. Has a small performance impact when used with runtime data.
//!
//! # Experimental modules
//!
//! Experimental, unstable functionality can be found in the `icu::experimental` crate. The modules in that crate
//! are on track to be eventually stabilized into this crate.
//!
//!
//! [CLDR]: https://cldr.unicode.org/
//! [`DataProvider`]: icu_provider::DataProvider
//! [`FsDataProvider`]: https://docs.rs/icu_provider_fs/latest/icu_provider_fs/struct.FsDataProvider.html
//! [`BlobDataProvider`]: https://docs.rs/icu_provider_blob/latest/icu_provider_blob/struct.BlobDataProvider.html
//! [`icu_provider_adapters`]: https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/
//! [`icu4x-datagen`]: https://crates.io/crates/icu4x-datagen
//! [`Locale`]: crate::locale::Locale
//! [data management tutorial]: https://github.com/unicode-org/icu4x/blob/main/tutorials/data-provider-runtime.md#loading-additional-data-at-runtime

// Needed for intra-doc link to work, since icu_provider is otherwise never mentioned in this crate
use icu_provider as _;

#[doc(inline)]
pub use icu_calendar as calendar;

#[doc(inline)]
pub use icu_casemap as casemap;

#[doc(inline)]
pub use icu_collator as collator;

#[doc(inline)]
pub use icu_datetime as datetime;

#[doc(inline)]
pub use icu_decimal as decimal;

#[doc(inline)]
pub use icu_list as list;

#[doc(inline)]
pub use icu_locale as locale;

#[doc(inline)]
pub use icu_normalizer as normalizer;

#[doc(inline)]
pub use icu_plurals as plurals;

#[doc(inline)]
pub use icu_properties as properties;

#[doc(inline)]
pub use icu_collections as collections;

#[doc(inline)]
pub use icu_segmenter as segmenter;

#[doc(inline)]
pub use icu_time as time;

#[doc(inline)]
#[cfg(feature = "experimental")]
pub use icu_experimental as experimental;

#[doc(inline)]
#[cfg(feature = "experimental")]
pub use icu_pattern as pattern;

#[cfg(feature = "datagen")]
mod datagen;
#[cfg(feature = "datagen")]
pub use datagen::*;
