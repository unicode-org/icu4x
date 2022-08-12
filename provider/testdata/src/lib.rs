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
//!   * [`unstable_baked`], [`unstable_baked_no_fallback`] (`baked` feature)
//! * [`buffer`], [`buffer_no_fallback`], [`small_buffer`]
//!   * [`buffer_json`], [`buffer_json_no_fallback`] (`fs` feature)
//! * [`any`], [`any_no_fallback`] (`baked` feature)
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
//! use icu_locid::locale;
//! use icu_provider::hello_world::*;
//! use icu_provider::prelude::*;
//! use std::borrow::Cow;
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

extern crate alloc;

#[cfg(feature = "metadata")]
pub mod metadata;

#[cfg(feature = "std")]
pub mod paths;

use icu_provider::prelude::*;
use icu_provider::serde::*;
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::StaticDataProvider;

/// A data provider that is compatible with all ICU `_unstable` constructors.
///
/// The return type of this method is not considered stable, mirroring the unstable trait
/// bounds of the constructors. For matching versions of `icu` and `icu_testdata`, however,
/// these are guaranteed to match.
///
/// This uses serde internally, which adds a runtime overhead, but reduces build time
/// compared to [`unstable_baked`].
pub fn unstable() -> LocaleFallbackProvider<DeserializingBufferProvider<'static, StaticDataProvider>>
{
    LocaleFallbackProvider::try_new_unstable(POSTCARD.as_deserializing()).unwrap()
}

/// A data provider that is compatible with all ICU `_unstable` constructors.
///
/// The return type of this method is not considered stable, mirroring the unstable trait
/// bounds of the constructors. For matching versions of `icu` and `icu_testdata`, however,
/// these are guaranteed to match.
///
/// This uses serde internally, which adds a runtime overhead, but reduces build time
/// compared to [`unstable_baked_no_fallback`].
pub fn unstable_no_fallback() -> DeserializingBufferProvider<'static, StaticDataProvider> {
    POSTCARD.as_deserializing()
}

lazy_static::lazy_static! {
    static ref POSTCARD: StaticDataProvider = {
        // The statically compiled data file is valid.
        #[allow(clippy::unwrap_used)]
        StaticDataProvider::try_new_from_static_blob(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/data/testdata.postcard"
        )))
        .unwrap()
    };
}

/// A [`BufferProvider`] backed by a Postcard blob.
pub fn buffer() -> impl BufferProvider {
    // The statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    LocaleFallbackProvider::try_new_with_buffer_provider(*POSTCARD).unwrap()
}

/// A [`BufferProvider`] backed by a Postcard blob.
pub fn buffer_no_fallback() -> impl BufferProvider {
    *POSTCARD
}

/// A smaller [`BufferProvider`] backed by a Postcard blob.
///
/// This provider only contains the `decimal/symbols@1[u-nu]` key for `en` and `bn`.
pub fn small_buffer() -> impl BufferProvider {
    lazy_static::lazy_static! {
        static ref SMALLER_POSTCARD: StaticDataProvider = {
            // The statically compiled data file is valid.
            #[allow(clippy::unwrap_used)]
            StaticDataProvider::try_new_from_static_blob(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/data/decimal-bn-en.postcard"
            )))
            .unwrap()
        };
    }
    *SMALLER_POSTCARD
}

/// A [`BufferProvider`] backed by a JSON directory.
///
/// You can optionally specify your own test data with the
/// `ICU4X_TESTDATA_DIR` environment variable.
///
/// # Panics
///
/// Panics if unable to load the data.
#[cfg(feature = "fs")]
pub fn buffer_json() -> impl BufferProvider {
    // The statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    LocaleFallbackProvider::try_new_with_buffer_provider(buffer_json_no_fallback()).unwrap()
}

/// A [`BufferProvider`] backed by a JSON directory.
///
/// You can optionally specify your own test data with the
/// `ICU4X_TESTDATA_DIR` environment variable.
///
/// # Panics
///
/// Panics if unable to load the data.
#[allow(clippy::panic)]
#[cfg(feature = "fs")]
pub fn buffer_json_no_fallback() -> impl BufferProvider {
    lazy_static::lazy_static! {
        static ref JSON: icu_provider_fs::FsDataProvider = {
            let path = match std::env::var_os("ICU4X_TESTDATA_DIR") {
                Some(val) => val.into(),
                None => paths::data_root().join("json"),
            };
            // The statically compiled data file is valid.
            #[allow(clippy::unwrap_used)]
            icu_provider_fs::FsDataProvider::try_new(&path).unwrap_or_else(|err| {
                panic!(
                    "The test data directory was unable to be opened: {}: {:?}",
                    err, path
                )
            })
        };
    }
    (*JSON).clone()
}

#[cfg(feature = "baked")]
mod baked {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/baked/mod.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/baked/any.rs"));
}

/// An [`AnyProvider`] backed by baked data.
#[cfg(feature = "baked")]
pub fn any() -> impl AnyProvider {
    LocaleFallbackProvider::try_new_with_any_provider(baked::BakedDataProvider).unwrap()
}

/// An [`AnyProvider`] backed by baked data.
#[cfg(feature = "baked")]
pub fn any_no_fallback() -> impl AnyProvider {
    baked::BakedDataProvider
}

/// A data provider that is compatible with all ICU `_unstable` constructors.
///
/// The return type of this method is not considered stable, mirroring the unstable trait
/// bounds of `_unstable` constructors. For matching versions of `icu` and `icu_testdata`,
/// these are guaranteed to match, however.
///
/// This uses databake, which adds a build time overhead, but improves runtime performance
/// compared to [`unstable`].
#[cfg(feature = "baked")]
pub fn unstable_baked() -> LocaleFallbackProvider<baked::BakedDataProvider> {
    LocaleFallbackProvider::try_new_unstable(baked::BakedDataProvider).unwrap()
}

/// A data provider that is compatible with all ICU `_unstable` constructors.
///
/// The return type of this method is not considered stable, mirroring the unstable trait
/// bounds of `_unstable` constructors. For matching versions of `icu` and `icu_testdata`,
/// these are guaranteed to match, however.
///
/// This uses databake, which adds a build time overhead, but improves runtime performance
/// compared to [`unstable_no_fallback`].
#[cfg(feature = "baked")]
pub fn unstable_baked_no_fallback() -> baked::BakedDataProvider {
    baked::BakedDataProvider
}
