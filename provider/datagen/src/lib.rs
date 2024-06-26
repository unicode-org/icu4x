// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::needless_doctest_main)]
//! `icu_datagen` is a library to generate data files that can be used in ICU4X data providers.
//!
//! For command-line usage, see the [`icu4x-datagen` binary](https://crates.io/crate/icu4x-datagen).
//!
//! Also see our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md).
//!
//! # Examples
//!
//! ```no_run
//! use icu_datagen::blob_exporter::*;
//! use icu_datagen::prelude::*;
//! use icu_datagen_bikeshed::DatagenProvider;
//! use std::fs::File;
//!
//! let provider = DatagenProvider::new_latest_tested();
//!
//! DatagenDriver::new([LocaleFamily::FULL], FallbackOptions::no_deduplication(), LocaleFallbacker::try_new_unstable(&provider).unwrap())
//!     .with_markers([icu::list::provider::AndListV1Marker::INFO])
//!     .export(
//!         &provider,
//!         BlobExporter::new_v2_with_sink(Box::new(
//!             File::create("data.postcard").unwrap(),
//!         )),
//!     )
//!     .unwrap();
//! ```
//!
//! # Cargo features
//!
//! This crate has a lot of dependencies, some of which are not required for all operating modes. These default Cargo features
//! can be disabled to reduce dependencies:
//! * `baked_exporter`
//!   * enables the [`baked_exporter`] module, a reexport of [`icu_provider_baked::export`]
//!   * enables the `--format mod` CLI argument
//! * `blob_exporter`
//!   * enables the [`blob_exporter`] module, a reexport of [`icu_provider_blob::export`]
//!   * enables the `--format blob` CLI argument
//! * `fs_exporter`
//!   * enables the [`fs_exporter`] module, a reexport of [`icu_provider_fs::export`]
//!   * enables the `--format dir` CLI argument
//! * `rayon`
//!   * enables parallelism during export
//! * `experimental`
//!   * enables data generation for markers defined in the unstable `icu_experimental` crate
//!   * note that this features affects the behaviour of `all_markers`

#![cfg_attr(
    not(test),
    deny(
        // This is a tool, and as such we don't care about panics too much
        // clippy::indexing_slicing,
        // clippy::unwrap_used,
        // clippy::expect_used,
        // clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

mod driver;

pub use driver::DatagenDriver;
pub use driver::DeduplicationStrategy;
pub use driver::FallbackOptions;
pub use driver::LocaleFamily;
pub use driver::NoFallbackOptions;

#[cfg(feature = "baked_exporter")]
pub use icu_provider_baked::export as baked_exporter;
#[cfg(feature = "blob_exporter")]
pub use icu_provider_blob::export as blob_exporter;
#[cfg(feature = "fs_exporter")]
pub use icu_provider_fs::export as fs_exporter;

/// A prelude for using the datagen API
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        DatagenDriver, DeduplicationStrategy, FallbackOptions, LocaleFamily, NoFallbackOptions,
    };
    #[doc(no_inline)]
    pub use icu_locale::{langid, LanguageIdentifier, LocaleFallbacker};
    #[doc(no_inline)]
    pub use icu_provider::{export::DataExporter, DataMarker, DataMarkerInfo};
}

#[cfg(feature = "rayon")]
pub(crate) use rayon::prelude as rayon_prelude;

#[cfg(not(feature = "rayon"))]
pub(crate) mod rayon_prelude {
    pub trait IntoParallelIterator: IntoIterator + Sized {
        fn into_par_iter(self) -> <Self as IntoIterator>::IntoIter {
            self.into_iter()
        }
    }
    impl<T: IntoIterator> IntoParallelIterator for T {}
}
