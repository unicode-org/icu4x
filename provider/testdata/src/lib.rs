// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_testdata` is a unit testing crate for [`ICU4X`].
//!
//! The crate exposes a data provider with stable data useful for unit testing. The data is
//! based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.
//!
//! There are four modes of operation, enabled by features:
//! * `static` (default) exposes [`get_postcard_provider`].
//! * `fs` exposes [`get_json_provider`]
//! * `baked` exposes [`get_baked_provider`].
//! * `metadata` exposes the [`metadata`] module which contains information such as the CLDR Gitref
//!   and the list of included locales.
//!
//! However, clients should not generally choose a specific provider, but rather use [`get_provider`].
//! This is currently an alias for [`get_postcard_provider`], as it is fast and has few dependencies.
//!
//! # Re-generating the data
//!
//! ## Downloading fresh CLDR data
//!
//! ```bash
//! $ cargo run --bin --features=bin icu4x-testdata-download-sources
//! ```
//!
//! ## Regenerating JSON and postcard data
//!
//! ```bash
//! $ cargo run --bin --features=bin icu4x-testdata-datagen
//! ```
//!
//! # Examples
//!
//! ```
//! use icu_locid::locale;
//! use icu_provider::prelude::*;
//! use std::borrow::Cow;
//!
//! let data_provider = icu_testdata::get_provider();
//!
//! let data: DataPayload<icu_plurals::provider::CardinalV1Marker> = data_provider
//!     .load(DataRequest {
//!         locale: &locale!("ru").into(),
//!         metadata: Default::default(),
//!     })
//!     .unwrap()
//!     .take_payload()
//!     .unwrap();
//! let rule = "v = 0 and i % 10 = 2..4 and i % 100 != 12..14"
//!     .parse()
//!     .expect("Failed to parse plural rule");
//! assert_eq!(data.get().few, Some(rule));
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

#[cfg(feature = "static")]
use {
    icu_provider_adapters::fallback::LocaleFallbackProvider, icu_provider_blob::StaticDataProvider,
};

#[cfg(feature = "fs")]
use icu_provider_fs::FsDataProvider;

/// Get a data, loading from the test data JSON directory.
///
/// You can optionally specify your own test data with the
/// `ICU4X_TESTDATA_DIR` environment variable.
///
/// # Panics
///
/// Panics if unable to load the data.
// The function is documented to allow panics.
#[allow(clippy::panic)]
#[cfg(feature = "fs")]
pub fn get_json_provider() -> FsDataProvider {
    let path = match std::env::var_os("ICU4X_TESTDATA_DIR") {
        Some(val) => val.into(),
        None => paths::data_root().join("json"),
    };
    FsDataProvider::try_new(&path).unwrap_or_else(|err| {
        panic!(
            "The test data directory was unable to be opened: {}: {:?}",
            err, path
        )
    })
}

/// Get a data provider, loading from the statically initialized postcard blob.
#[cfg(feature = "static")]
pub fn get_postcard_provider() -> StaticDataProvider {
    // The statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    StaticDataProvider::new_from_static_blob(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/testdata.postcard"
    )))
    .unwrap()
}

/// Get a small data provider that only contains the `decimal/symbols@1` key for `en` and `bn`.
#[cfg(feature = "static")]
pub fn get_smaller_postcard_provider() -> StaticDataProvider {
    // The statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    StaticDataProvider::new_from_static_blob(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/decimal-bn-en.postcard"
    )))
    .unwrap()
}

#[cfg(feature = "baked")]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/baked/mod.rs"));

#[cfg(feature = "baked")]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/baked/any.rs"));

#[cfg(feature = "baked")]
/// Get a data provider that contains hardcoded data without any deserialization overhead.
pub fn get_baked_provider() -> BakedDataProvider {
    BakedDataProvider
}

/// Get a data provider loading from a statically initialized postcard blob
/// with locale fallbacking enabled.
#[cfg(feature = "static")]
pub fn get_provider() -> LocaleFallbackProvider<StaticDataProvider> {
    // The statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    LocaleFallbackProvider::try_new(get_postcard_provider()).unwrap()
}
