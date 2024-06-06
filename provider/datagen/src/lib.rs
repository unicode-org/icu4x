// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::needless_doctest_main)]
//! `icu_datagen` is a library to generate data files that can be used in ICU4X data providers.
//!
//! Data files can be generated either programmatically (i.e. in `build.rs`), or through a
//! command-line utility.
//!
//!
//! Also see our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md).
//!
//! # Examples
//!
//! ## Rust API
//!
//! ```no_run
//! use icu_datagen::blob_exporter::*;
//! use icu_datagen::prelude::*;
//! use std::fs::File;
//!
//! DatagenDriver::new()
//!     .with_markers([icu::list::provider::AndListV1Marker::INFO])
//!     .with_locales_and_fallback([LocaleFamily::FULL], Default::default())
//!     .export(
//!         &DatagenProvider::new_latest_tested(),
//!         BlobExporter::new_v2_with_sink(Box::new(
//!             File::create("data.postcard").unwrap(),
//!         )),
//!     )
//!     .unwrap();
//! ```
//!
//! ## Command line
//!
//! The command line interface can be installed through Cargo.
//!
//! ```bash
//! $ cargo install icu_datagen
//! ```
//!
//! Once the tool is installed, you can invoke it like this:
//!
//! ```bash
//! $ icu4x-datagen --markers all --locales de en-AU --format blob --out data.postcard
//! ```
//!
//! More details can be found by running `--help`.
//!
//! # Cargo features
//!
//! This crate has a lot of dependencies, some of which are not required for all operating modes. These default Cargo features
//! can be disabled to reduce dependencies:
//! * `baked_exporter`
//!   * enables the [`baked_exporter`] module
//!   * enables the `--format mod` CLI argument
//! * `blob_exporter`
//!   * enables the [`blob_exporter`] module, a reexport of [`icu_provider_blob::export`]
//!   * enables the `--format blob` CLI argument
//! * `fs_exporter`
//!   * enables the [`fs_exporter`] module, a reexport of [`icu_provider_fs::export`]
//!   * enables the `--format dir` CLI argument
//! * `networking`
//!   * enables methods on [`DatagenProvider`] that fetch source data from the network
//!   * enables the `--cldr-tag`, `--icu-export-tag`, and `--segmenter-lstm-tag` CLI arguments that download data
//! * `rayon`
//!   * enables parallelism during export
//! * `use_wasm` / `use_icu4c`
//!   * see the documentation on [`icu_codepointtrie_builder`](icu_codepointtrie_builder#build-configuration)
//! * `bin`
//!   * required by the CLI and enabled by default to make `cargo install` work
//! * `icu_experimental`
//!   * enables data generation for markers defined in the unstable `icu_experimental` crate
//!   * note that this features affects the behaviour of `all_markers`
//!
//! The meta-feature `experimental_components` is available to activate all experimental components.

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
#[cfg(feature = "provider")]
mod provider;
mod registry;

pub use driver::DatagenDriver;
pub use driver::DeduplicationStrategy;
pub use driver::FallbackOptions;
pub use driver::LocaleFamily;
pub use driver::NoFallbackOptions;
pub use driver::RuntimeFallbackLocation;

#[cfg(feature = "provider")]
pub use provider::CollationHanDatabase;
#[cfg(feature = "provider")]
pub use provider::CoverageLevel;
#[cfg(feature = "provider")]
pub use provider::DatagenProvider;

#[cfg(feature = "baked_exporter")]
pub mod baked_exporter;
#[cfg(feature = "blob_exporter")]
pub use icu_provider_blob::export as blob_exporter;
#[cfg(feature = "fs_exporter")]
pub use icu_provider_fs::export as fs_exporter;

/// A prelude for using the datagen API
pub mod prelude {
    #[doc(no_inline)]
    #[cfg(feature = "provider")]
    pub use crate::provider::{CollationHanDatabase, CoverageLevel, DatagenProvider};
    #[doc(no_inline)]
    pub use crate::{
        DatagenDriver, DeduplicationStrategy, FallbackOptions, LocaleFamily, NoFallbackOptions,
        RuntimeFallbackLocation,
    };
    #[doc(no_inline)]
    pub use icu_locale_core::{langid, LanguageIdentifier};
    #[doc(no_inline)]
    pub use icu_provider::{datagen::DataExporter, DataMarker, DataMarkerInfo};
}

use icu_provider::prelude::*;
use std::path::Path;

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

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        /// List of all markers that are available.
        ///
        /// Note that since v1.3, `all_markers` also contains experimental markers for which the
        /// corresponding Cargo features has been enabled.
        // Excludes the hello world marker, as that generally should not be generated.
        pub fn all_markers() -> Vec<DataMarkerInfo> {
            #[cfg(features = "experimental_components")]
            log::warn!("The icu_datagen crates has been built with the `experimental_components` feature, so `all_markers` returns experimental markers");
            vec![
                $(
                    <$marker>::INFO,
                )+
                $(
                    #[cfg(feature = "experimental_components")]
                    <$emarker>::INFO,
                )+
            ]
        }

        #[test]
        fn no_marker_collisions() {
            let mut map = std::collections::BTreeMap::new();
            let mut failed = false;
            for marker in all_markers() {
                if let Some(colliding_marker) = map.insert(marker.path.hashed(), marker) {
                    println!(
                        "{:?} and {:?} collide at {:?}",
                        marker.path,
                        colliding_marker.path,
                        marker.path.hashed(),
                    );
                    failed = true;
                }
            }
            if failed {
                panic!();
            }
        }

        /// Parses a human-readable marker identifier into a [`DataMarkerInfo`].
        //  Supports the hello world marker
        /// # Example
        /// ```
        /// # use icu_provider::DataMarker;
        /// assert_eq!(
        ///     icu_datagen::marker("list/and@1"),
        ///     Some(icu_list::provider::AndListV1Marker::INFO),
        /// );
        /// ```
        pub fn marker<S: AsRef<str>>(string: S) -> Option<DataMarkerInfo> {
            use std::sync::OnceLock;
            static LOOKUP: OnceLock<std::collections::HashMap<&'static str, Result<DataMarkerInfo, &'static str>>> = OnceLock::new();
            let lookup = LOOKUP.get_or_init(|| {
                [
                    ("core/helloworld@1", Ok(icu_provider::hello_world::HelloWorldV1Marker::INFO)),
                    $(
                        ($path, Ok(<$marker>::INFO)),
                    )+
                    $(
                        #[cfg(feature = "experimental_components")]
                        ($epath, Ok(<$emarker>::INFO)),
                        #[cfg(not(feature = "experimental_components"))]
                        ($epath, Err(stringify!(feature = "experimental_components"))),
                    )+

                ]
                .into_iter()
                .collect()
            });
            let path = string.as_ref();
            match lookup.get(path).copied() {
                None => {
                    log::warn!("Unknown marker {path:?}");
                    None
                },
                Some(Err(feature)) => {
                    log::warn!("Marker {path:?} requires {feature}");
                    None
                },
                Some(Ok(marker)) => Some(marker)
            }
        }

        #[test]
        fn test_paths_correct() {
            $(
                assert_eq!(<$marker>::INFO.path.get(), $path);
            )+
            $(
                assert_eq!(<$emarker>::INFO.path.get(), $epath);
            )+
        }

        #[macro_export]
        #[doc(hidden)] // macro
        macro_rules! make_exportable_provider {
            ($ty:ty) => {
                icu_provider::make_exportable_provider!(
                    $ty,
                    [
                        icu_provider::hello_world::HelloWorldV1Marker,
                        $(
                            $marker,
                        )+
                        $(
                            #[cfg(feature = "experimental_components")]
                            $emarker,
                        )+
                    ]
                );
            }
        }
    }
}
crate::registry!(cb);

/// Parses a list of human-readable marker identifiers and returns a
/// list of [`DataMarkerInfo`]s.
///
/// Unknown marker names are ignored.
//  Supports the hello world marker
/// # Example
/// ```
/// # use icu_provider::DataMarker;
/// assert_eq!(
///     icu_datagen::markers(&["list/and@1", "list/or@1"]),
///     vec![
///         icu::list::provider::AndListV1Marker::INFO,
///         icu::list::provider::OrListV1Marker::INFO,
///     ],
/// );
/// ```
pub fn markers<S: AsRef<str>>(strings: &[S]) -> Vec<DataMarkerInfo> {
    strings.iter().filter_map(crate::marker).collect()
}

/// Parses a compiled binary and returns a list of [`DataMarkerInfo`]s that it uses *at runtime*.
///
/// This function is intended to be used for binaries that use `AnyProvider` or `BufferProvider`,
/// for which the compiler cannot remove unused data. Using this function on a binary that only
/// uses compiled data (through the `compiled_data` feature or manual baked data) will not return
/// any markers, as the markers only exist in the type system.
///
/// # Example
///
/// #### build.rs
/// ```no_run
/// # use icu_provider::DataMarker;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// assert_eq!(
///     icu_datagen::markers_from_bin("target/release/my-app")?,
///     vec![
///         icu::list::provider::AndListV1Marker::INFO,
///         icu::list::provider::OrListV1Marker::INFO,
///     ],
/// );
/// # Ok(())
/// # }
/// ```
//  Supports the hello world marker
pub fn markers_from_bin<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<DataMarkerInfo>> {
    let file = std::fs::read(path.as_ref())?;
    let file = file.as_slice();

    Ok(markers_from_bin_inner(file))
}

fn markers_from_bin_inner(bytes: &[u8]) -> Vec<DataMarkerInfo> {
    use memchr::memmem::*;

    const LEADING_TAG: &[u8] = icu_provider::leading_tag!().as_bytes();
    const TRAILING_TAG: &[u8] = icu_provider::trailing_tag!().as_bytes();

    let trailing_tag = Finder::new(TRAILING_TAG);

    let mut result: Vec<DataMarkerInfo> = find_iter(bytes, LEADING_TAG)
        .map(|tag_position| tag_position + LEADING_TAG.len())
        .map(|marker_start| &bytes[marker_start..])
        .filter_map(move |marker_fragment| {
            trailing_tag
                .find(marker_fragment)
                .map(|end| &marker_fragment[..end])
        })
        .map(std::str::from_utf8)
        .filter_map(Result::ok)
        .filter_map(crate::marker)
        .collect();

    result.sort();
    result.dedup();

    result
}

#[test]
fn test_markers() {
    assert_eq!(
        markers(&[
            "list/and@1",
            "datetime/gregory/datelengths@1",
            "decimal/symbols@1",
            "trash",
        ]),
        vec![
            icu_list::provider::AndListV1Marker::INFO,
            icu_datetime::provider::calendar::GregorianDateLengthsV1Marker::INFO,
            icu_decimal::provider::DecimalSymbolsV1Marker::INFO,
        ]
    );
}

#[test]
fn test_markers_from_bin() {
    assert_eq!(
        markers_from_bin_inner(include_bytes!("../tests/data/tutorial_buffer.wasm")),
        vec![
            icu_datetime::provider::calendar::GregorianDateLengthsV1Marker::INFO,
            icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker::INFO,
            icu_datetime::provider::calendar::TimeLengthsV1Marker::INFO,
            icu_datetime::provider::calendar::TimeSymbolsV1Marker::INFO,
            icu_calendar::provider::WeekDataV1Marker::INFO,
            icu_decimal::provider::DecimalSymbolsV1Marker::INFO,
            icu_plurals::provider::OrdinalV1Marker::INFO,
        ]
    );
}
