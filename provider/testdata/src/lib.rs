// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_testdata` is a unit testing package for [`ICU4X`].
//!
//! The package exposes a `DataProvider` with stable data useful for unit testing. The data is
//! based on a CLDR tag and a short list of locales that, together, cover a range of scenarios.
//!
//! The output data can be found in the [data](./data/) subdirectory. There, you will find:
//!
//! - `json` for the ICU4X JSON test data
//! - `cldr` for the source CLDR JSON
//! - `uprops` for the source Unicode properties data
//!
//! ## Pointing to custom test data
//!
//! If you wish to run ICU4X tests with custom test data, you may do so by setting the "ICU4X_TESTDATA_DIR" environment variable:
//!
//! ```bash
//! $ ICU4X_TESTDATA_DIR=/path/to/custom/testdata cargo test
//! ```
//!
//! Note: this does not work with [`get_static_provider`](crate::get_static_provider).
//!
//! ## Re-generating the data
//!
//! From the top level directory of the `icu4x` metapackage, run:
//!
//! ```bash
//! $ cargo make testdata
//! ```
//!
//! The following commands are also available:
//!
//! - `cargo make testdata-download-sources` downloads fresh CLDR JSON
//! - `cargo make testdata-build-json` re-generates the ICU4X JSON
//! - `cargo make testdata-build-blob` re-generates the ICU4X blob file
//! - `cargo make testdata-build-bincode` re-generates Bincode filesystem testdata
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

use icu_locid::{langid, LanguageIdentifier};

/// Git tag or sha1 for the CLDR data used to generate the testdata.
pub const CLDR_GITREF: &str = "40.0.0";

/// Locales included in testdata. We use 10 base languages with a small number
/// of variants to allow for 10 languages to be used in microbenchmarks.
// Keep this list somewhat short, but cover all features.
pub const LOCALES: &[LanguageIdentifier] = &[
    // Arabic:
    // - Good example for RTL
    // - Non-latin numerals in Egypt
    langid!("ar"),
    langid!("ar-EG"),
    // Bangla:
    // - Uses non-Latin numerals
    langid!("bn"),
    // Chakma:
    // - High-coverage language that uses non-BMP code points
    langid!("ccp"),
    // English:
    // - Widely understood language in software engineering
    // - Includes regional variants to test similar-data fallbacks
    langid!("en"),
    langid!("en-001"),
    langid!("en-ZA"),
    // Spanish:
    //  - Most popular Romance language
    //  - South American dialect
    //  - Has context dependent list fragments
    langid!("es"),
    langid!("es-AR"),
    // French:
    // - Often the first non-English locale to receive new data in CLDR
    langid!("fr"),
    // Filipino:
    // - Week of month/year have plural variants.
    langid!("fil"),
    // Japanese:
    // - Four scripts
    // - Complex date patterns
    langid!("ja"),
    // Russian:
    // - Cyrillic script
    // - Interesting plural rules
    // - Hightly inflected, many gramatical cases
    langid!("ru"),
    // Serbian:
    // - Multiple scripts
    // - Southern Europe
    // - Hightly inflected, many gramatical cases
    langid!("sr"),
    langid!("sr-Cyrl"),
    langid!("sr-Latn"),
    // Thai:
    // - Complex word breaking
    langid!("th"),
    // Turkish:
    // - Interesting case-mappings
    langid!("tr"),
    // Root data
    LanguageIdentifier::UND,
];

#[cfg(any(feature = "bin", feature = "fs"))]
pub mod paths;

#[cfg(feature = "static")]
mod blob;
#[cfg(feature = "fs")]
mod fs;

#[cfg(feature = "static")]
pub use blob::{get_smaller_static_provider, get_static_provider};
#[cfg(feature = "fs")]
pub use fs::get_provider;
