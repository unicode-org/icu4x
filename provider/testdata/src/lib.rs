// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_testdata` is a unit testing crate for [`ICU4X`].
//!
//! The crate exposes a data provider with stable data useful for unit testing. The data is
//! based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.
//!
//! There are four modes of operation, enabled by features:
//! * `fs` (default) exposes [`get_json_provider`] with alias [`get_provider`]. In this mode you
//!   can optionally specify your own test data with the `ICU4X_TESTDATA_DIR` environment variable.
//! * `static` exposes [`get_postcard_provider`] with alias [`get_provider`] (unless `fs` is
//!   also enabled).
//! * `metadata` exposes the [`metadata`] module which contains information such as the CLDR Gitref
//!   and the list of included locales.
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
//! use std::borrow::Cow;
//! use icu_provider::prelude::*;
//! use icu_locid::locale;
//!
//! let data_provider = icu_testdata::get_provider();
//!
//! let data: DataPayload<icu_plurals::provider::CardinalV1Marker> = data_provider
//!     .load_resource(&DataRequest {
//!         options: locale!("ru").into(),
//!         metadata: Default::default(),
//!     })
//!     .unwrap()
//!     .take_payload()
//!     .unwrap();
//! let rule = "v = 0 and i % 10 = 2..4 and i % 100 != 12..14".parse()
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
        clippy::panic
    )
)]

extern crate alloc;

#[cfg(feature = "metadata")]
pub mod metadata;

#[cfg(feature = "std")]
pub mod paths;

/// Get a data, loading from the test data JSON directory.
///
/// # Panics
///
/// Panics if unable to load the data.
// The function is documented to allow panics.
#[allow(clippy::panic)]
#[cfg(feature = "fs")]
pub fn get_json_provider() -> icu_provider_fs::FsDataProvider {
    let path = match std::env::var_os("ICU4X_TESTDATA_DIR") {
        Some(val) => val.into(),
        None => paths::data_root().join("json"),
    };
    icu_provider_fs::FsDataProvider::try_new(&path).unwrap_or_else(|err| {
        panic!(
            "The test data directory was unable to be opened: {}: {:?}",
            err, path
        )
    })
}

/// Get a data provider, loading from the statically initialized postcard blob.
#[cfg(feature = "static")]
pub fn get_postcard_provider() -> icu_provider_blob::StaticDataProvider {
    // The statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    icu_provider_blob::StaticDataProvider::new_from_static_blob(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/testdata.postcard"
    )))
    .unwrap()
}

/// Get a small data provider that only contains the `decimal/symbols@1` key for `en` and `bn`.
#[cfg(feature = "static")]
pub fn get_smaller_postcard_provider() -> icu_provider_blob::StaticDataProvider {
    // THe statically compiled data file is valid.
    #[allow(clippy::unwrap_used)]
    icu_provider_blob::StaticDataProvider::new_from_static_blob(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/decimal-bn-en.postcard"
    )))
    .unwrap()
}

#[cfg(feature = "fs")]
pub use get_json_provider as get_provider;
#[cfg(all(feature = "static", not(feature = "fs")))]
pub use get_postcard_provider as get_provider;
#[cfg(feature = "static")]
pub use get_postcard_provider as get_static_provider;
#[cfg(feature = "static")]
pub use get_smaller_postcard_provider as get_smaller_static_provider;