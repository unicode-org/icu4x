// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_testdata` is a unit testing crate for [`ICU4X`].
//!
//! The crate exposes data providers with stable data useful for unit testing. The data is
//! based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.
//!
//! The crate exposes three kinds of providers, corresponding to the three types of constructors
//! in ICU:
//! * [`unstable`], [`unstable_no_fallback`]
//! * [`any`], [`any_no_fallback`]
//! * [`buffer`], [`buffer_no_fallback`] (`buffer` Cargo feature)
//!
//! # Examples
//!
//! ```
//! use icu::locid::locale;
//! use icu_provider::hello_world::HelloWorldFormatter;
//!
//! // Unstable constructor
//! HelloWorldFormatter::try_new_unstable(
//!     &icu_testdata::unstable(),
//!     &locale!("en-CH").into(),
//! ).unwrap();
//!
//! // AnyProvider constructor
//! HelloWorldFormatter::try_new_with_any_provider(
//!     &icu_testdata::any(),
//!     &locale!("en-CH").into(),
//! ).unwrap();
//!
//! // BufferProvider constructor (`icu` with `serde` feature, `icu_testdata` with `buffer` feature)
//! HelloWorldFormatter::try_new_with_buffer_provider(
//!     &icu_testdata::buffer(),
//!     &locale!("en-CH").into(),
//! ).unwrap();
//!
//! // Without fallback the locale match needs to be exact
//! HelloWorldFormatter::try_new_unstable(
//!     &icu_testdata::unstable_no_fallback(),
//!     &locale!("en-CH").into(),
//! ).is_err();
//!
//! HelloWorldFormatter::try_new_unstable(
//!     &icu_testdata::unstable_no_fallback(),
//!     &locale!("en").into(),
//! ).unwrap();
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
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]
#![allow(unused_imports)] // too many feature combinations too keep track of
#![allow(deprecated)]

extern crate alloc;

#[path = "../data/metadata.rs.data"]
mod metadata;

pub mod versions {
    //! Functions to access version info of the ICU test data.

    /// Gets the CLDR tag used as the test data source (for formatters, likely subtags, ...)
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("43.1.0", icu_testdata::versions::cldr_tag());
    /// ```
    #[deprecated(since = "1.3.0", note = "use `compiled_data`")]
    pub fn cldr_tag() -> alloc::string::String {
        alloc::string::String::from(super::metadata::CLDR_TAG)
    }

    /// Gets the ICU tag used as the test data source (for properties, collator, ...)
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("icu4x/2023-05-02/73.x", icu_testdata::versions::icu_tag());
    /// ```
    #[deprecated(since = "1.3.0", note = "use `compiled_data`")]
    pub fn icu_tag() -> alloc::string::String {
        alloc::string::String::from(super::metadata::ICUEXPORT_TAG)
    }
}

/// Gets the locales supported by the test data.
///
/// # Examples
///
/// ```
/// # use icu_locid::langid;
/// assert!(icu_testdata::locales().contains(&langid!("es-AR")));
/// assert!(icu_testdata::locales().len() > 10);
/// ```
#[deprecated(since = "1.3.0", note = "use `compiled_data`")]
pub fn locales() -> alloc::vec::Vec<icu_locid::LanguageIdentifier> {
    alloc::vec::Vec::from(metadata::LOCALES)
}

#[cfg(feature = "std")]
#[deprecated]
/// Get paths to the test data directories. Some of these paths do not
/// exist anymore, and data should only be accessed through the functions
/// provided by this crate.
pub mod paths {
    use std::path::PathBuf;

    #[deprecated(since = "1.3.0", note = "use `compiled_data`")]
    /// Returns the absolute path to the top-level data directory.
    pub fn data_root() -> PathBuf {
        PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data")
    }

    #[deprecated(since = "1.3.0", note = "use `compiled_data`")]
    /// Returns the absolute path to the CLDR JSON root directory.
    pub fn cldr_json_root() -> PathBuf {
        data_root().join("cldr")
    }

    #[deprecated(since = "1.3.0", note = "use `compiled_data`")]
    /// Returns the absolute path to the icuexport TOML root directory.
    pub fn icuexport_toml_root() -> PathBuf {
        data_root().join("icuexport")
    }

    #[deprecated(since = "1.3.0", note = "use `compiled_data`")]
    /// Returns the absolute path to the collation tailoring TOML root directory.
    pub fn coll_toml_root() -> PathBuf {
        data_root().join("coll")
    }
}

use icu_provider::prelude::*;
use icu_provider_adapters::fallback::LocaleFallbackProvider;

/// A data provider that is compatible with all ICU `_unstable` constructors.
///
/// The return type of this method is not considered stable, mirroring the unstable trait
/// bounds of the constructors. For matching versions of `icu` and `icu_testdata`, however,
/// these are guaranteed to match.
#[cfg(feature = "icu_locid_transform")]
#[deprecated(since = "1.3.0", note = "use `compiled_data`")]
pub fn unstable() -> LocaleFallbackProvider<UnstableDataProvider> {
    // The statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    LocaleFallbackProvider::try_new_unstable(unstable_no_fallback()).unwrap()
}

/// A data provider that is compatible with all ICU `_unstable` constructors.
///
/// The return type of this method is not considered stable, mirroring the unstable trait
/// bounds of the constructors. For matching versions of `icu` and `icu_testdata`, however,
/// these are guaranteed to match.
#[deprecated(since = "1.3.0", note = "use `compiled_data`")]
pub fn unstable_no_fallback() -> UnstableDataProvider {
    UnstableDataProvider
}

/// An [`AnyProvider`] backed by baked data.
#[cfg(feature = "icu_locid_transform")]
#[deprecated(since = "1.3.0", note = "use `compiled_data`")]
pub fn any() -> impl AnyProvider {
    // The baked data is valid.
    #[allow(clippy::unwrap_used)]
    LocaleFallbackProvider::try_new_with_any_provider(any_no_fallback()).unwrap()
}

/// An [`AnyProvider`] backed by baked data.
#[deprecated(since = "1.3.0", note = "use `compiled_data`")]
pub fn any_no_fallback() -> impl AnyProvider {
    UnstableDataProvider
}

/// A [`BufferProvider`] backed by a Postcard blob.
///
/// This deserializes a large data blob from static memory, please cache the result if you
/// are calling this repeatedly and care about performance
#[cfg(feature = "buffer")]
#[deprecated(since = "1.3.0", note = "use `compiled_data`")]
pub fn buffer() -> impl BufferProvider {
    // The statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    LocaleFallbackProvider::try_new_with_buffer_provider(buffer_no_fallback()).unwrap()
}

/// A [`BufferProvider`] backed by a Postcard blob.
///
/// This deserializes a large data blob from static memory, please cache the result if you
/// are calling this repeatedly and care about performance
#[cfg(feature = "buffer")]
#[deprecated(since = "1.3.0", note = "use `compiled_data`")]
pub fn buffer_no_fallback() -> impl BufferProvider {
    #[allow(clippy::unwrap_used)] // The statically compiled data file is valid.
    icu_provider_blob::BlobDataProvider::try_new_from_static_blob(include_bytes!(
        "../data/testdata.postcard"
    ))
    .unwrap()
}

#[doc(hidden)]
#[non_exhaustive]
#[derive(Debug)]
#[deprecated(since = "1.3.0", note = "use `compiled_data`")]
pub struct UnstableDataProvider;

mod baked {
    include!("../data/baked/mod.rs");
    impl_data_provider!(super::UnstableDataProvider);
    impl_any_provider!(super::UnstableDataProvider);
}
