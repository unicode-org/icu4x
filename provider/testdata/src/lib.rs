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
//! * [`buffer`], [`buffer_no_fallback`] (`buffer` feature)
//!
//!
//! Additionally, the `metadata` feature exposes the [`metadata`] module which contains information
//! such as the CLDR Gitref  and the list of included locales.
//!
//! # `bin` feature
//!
//! ## Downloading fresh CLDR data
//!
//! ```bash
//! $ cargo run --bin --features=bin icu4x-testdata-download-sources
//! ```
//!
//! ## Regenerating data
//!
//! ```bash
//! $ cargo run --bin --features=bin icu4x-testdata-datagen
//! ```
//!
//! # Examples
//!
//! ```
//! use icu::locid::locale;
//! use icu_provider::hello_world::*;
//! use icu_provider::prelude::*;
//!
//! let req = DataRequest {
//!     locale: &locale!("en").into(),
//!     metadata: Default::default(),
//! };
//!
//! assert_eq!(
//!     DataProvider::<HelloWorldV1Marker>::load(
//!         &icu_testdata::unstable(),
//!         req
//!     )
//!     .and_then(DataResponse::take_payload)
//!     .unwrap()
//!     .get()
//!     .message,
//!     "Hello World"
//! );
//!
//! assert_eq!(
//!     BufferProvider::load_buffer(
//!         &icu_testdata::buffer(),
//!         HelloWorldV1Marker::KEY,
//!         req
//!     )
//!     .and_then(DataResponse::take_payload)
//!     .unwrap()
//!     .get(),
//!     &b"\x0bHello World"
//! );
//!
//! assert_eq!(
//!     AnyProvider::load_any(
//!         &icu_testdata::any(),
//!         HelloWorldV1Marker::KEY,
//!         req
//!     )
//!     .and_then(AnyResponse::downcast::<HelloWorldV1Marker>)
//!     .and_then(DataResponse::take_payload)
//!     .unwrap()
//!     .get()
//!     .message,
//!     "Hello World"
//! );
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
        clippy::exhaustive_enums
    )
)]
#![warn(missing_docs)]

extern crate alloc;

// If you want this to be made public stable, file an issue on the ICU4X repository.
#[cfg(feature = "metadata")]
#[doc(hidden)]
pub mod metadata;

#[cfg(feature = "metadata")]
pub mod versions {
    //! Functions to access version info of the ICU test data.

    /// Gets the CLDR tag used as the test data source (for formatters, likely subtags, ...)
    ///
    /// Enabled with the "metadata" feature.
    ///
    /// # Panics
    ///
    /// Panics if the package metadata cannot be loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("41.0.0", icu_testdata::versions::cldr_tag());
    /// ```
    #[allow(clippy::unwrap_used)] // documented
    pub fn cldr_tag() -> String {
        crate::metadata::load()
            .unwrap()
            .package_metadata
            .cldr_json_gitref
    }

    /// Gets the ICU tag used as the test data source (for properties, collator, ...)
    ///
    /// Enabled with the "metadata" feature.
    ///
    /// # Panics
    ///
    /// Panics if the package metadata cannot be loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("icu4x/2022-08-17/71.x", icu_testdata::versions::icu_tag());
    /// ```
    #[allow(clippy::unwrap_used)] // documented
    pub fn icu_tag() -> String {
        crate::metadata::load()
            .unwrap()
            .package_metadata
            .icuexportdata_gitref
    }
}

#[cfg(feature = "std")]
pub mod paths;

use icu_provider::prelude::*;
use icu_provider_adapters::fallback::LocaleFallbackProvider;

/// A data provider that is compatible with all ICU `_unstable` constructors.
///
/// The return type of this method is not considered stable, mirroring the unstable trait
/// bounds of the constructors. For matching versions of `icu` and `icu_testdata`, however,
/// these are guaranteed to match.
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
pub fn unstable_no_fallback() -> UnstableDataProvider {
    UnstableDataProvider
}

/// An [`AnyProvider`] backed by baked data.
pub fn any() -> impl AnyProvider {
    // The baked data is valid.
    #[allow(clippy::unwrap_used)]
    LocaleFallbackProvider::try_new_with_any_provider(any_no_fallback()).unwrap()
}

/// An [`AnyProvider`] backed by baked data.
pub fn any_no_fallback() -> impl AnyProvider {
    UnstableDataProvider
}

/// A [`BufferProvider`] backed by a Postcard blob.
#[cfg(feature = "buffer")]
pub fn buffer() -> impl BufferProvider {
    // The statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    LocaleFallbackProvider::try_new_with_buffer_provider(buffer_no_fallback()).unwrap()
}

/// A [`BufferProvider`] backed by a Postcard blob.
#[cfg(feature = "buffer")]
pub fn buffer_no_fallback() -> impl BufferProvider {
    lazy_static::lazy_static! {
        static ref POSTCARD: icu_provider_blob::BlobDataProvider = {
            // The statically compiled data file is valid.
            #[allow(clippy::unwrap_used)]
            icu_provider_blob::BlobDataProvider::try_new_from_static_blob(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/data/testdata.postcard"
            )))
            .unwrap()
        };
    }
    POSTCARD.clone()
}

#[doc(hidden)]
pub use baked::BakedDataProvider as UnstableDataProvider;

mod baked {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/baked/mod.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/baked/any.rs"));
}
