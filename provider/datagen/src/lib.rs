// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::needless_doctest_main)]
//! `icu_datagen` is a library to generate data files that can be used in ICU4X data providers.
//!
//! Data files can be generated either programmatically (i.e. in `build.rs`), or through a
//! command-line utility.
//!
//! # Examples
//!
//! ## `build.rs`
//!
//! ```no_run
//! use icu_datagen::*;
//! use icu_locid::langid;
//! use std::fs::File;
//! use std::path::PathBuf;
//!
//! fn main() {
//!     icu_datagen::datagen(
//!         Some(&[langid!("de"), langid!("en-AU")]),
//!         &icu_datagen::keys(&["list/and@1"]),
//!         &SourceData::default(),
//!         vec![Out::Blob(Box::new(File::create("data.postcard").unwrap()))],
//!     )
//!     .unwrap();
//! }
//! ```
//!
//! ## Command line
//! The command line interface is available with the `bin` feature.
//! ```bash
//! cargo run --features bin -- \
//!     --uprops-root /path/to/uprops/root \
//!     --all-keys \
//!     --locales de,en-AU \
//!     --format blob \
//!     --out data.postcard
//! ```

//! More details can be found by running `--help`.

#![cfg_attr(
    not(test),
    deny(
        // This is a tool, and as such we don't care about panics too much
        // clippy::indexing_slicing,
        // clippy::unwrap_used,
        // clippy::expect_used,
        // clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums
    )
)]
#![warn(missing_docs)]

mod crabbake;
mod error;
mod registry;
mod source;
pub mod transform;

pub use error::{MISSING_CLDR_ERROR, MISSING_COLLATION_ERROR, MISSING_UPROPS_ERROR};
pub use registry::get_all_keys;
pub use source::{SourceData, TrieType};

use icu_locid::LanguageIdentifier;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use icu_provider_adapters::filter::Filterable;
use icu_provider_fs::export::serializers;
use rayon::prelude::*;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

/// Parses a list of human-readable key identifiers and returns a
/// list of [`ResourceKey`]s. Invalid key names are ignored.
///
/// # Example
/// ```
/// # use icu_provider::ResourceMarker;
/// assert_eq!(
///     icu_datagen::keys(&["list/and@1", "list/or@1"]),
///     vec![
///         icu_list::provider::AndListV1Marker::KEY,
///         icu_list::provider::OrListV1Marker::KEY,
///     ],
/// );
/// ```
pub fn keys<S: AsRef<str>>(strings: &[S]) -> Vec<ResourceKey> {
    let keys = strings.iter().map(AsRef::as_ref).collect::<HashSet<&str>>();
    get_all_keys()
        .into_iter()
        .filter(|k| keys.contains(k.get_path()))
        .collect()
}

/// Parses a file of human-readable key identifiers and returns a
/// list of [`ResourceKey`]s. Invalid key names are ignored.
///
/// # Example
///
/// #### keys.txt
/// ```text
/// list/and@1
/// list/or@1
/// ```
/// #### build.rs
/// ```no_run
/// # use icu_provider::ResourceMarker;
/// # use std::fs::File;
/// # fn main() -> std::io::Result<()> {
/// assert_eq!(
///     icu_datagen::keys_from_file(File::open("keys.txt")?)?,
///     vec![
///         icu_list::provider::AndListV1Marker::KEY,
///         icu_list::provider::OrListV1Marker::KEY,
///     ],
/// );
/// # Ok(())
/// # }
/// ```
pub fn keys_from_file<R: Read>(file: R) -> std::io::Result<Vec<ResourceKey>> {
    let keys = BufReader::new(file)
        .lines()
        .collect::<std::io::Result<HashSet<String>>>()?;
    Ok(get_all_keys()
        .into_iter()
        .filter(|k| keys.contains(k.get_path()))
        .collect())
}

/// The output format.
#[non_exhaustive]
pub enum Out {
    /// Output to a file system tree
    Fs {
        /// The root path.
        output_path: PathBuf,
        /// The serialization format. See [icu_provider_fs::export::serializers].
        serializer: Box<dyn serializers::AbstractSerializer + Sync>,
        /// Whether to overwrite existing data.
        overwrite: bool,
    },
    /// Output as a postcard blob to the given sink.
    Blob(Box<dyn std::io::Write + Sync>),
    /// Output a module at the given location.
    Module {
        /// The directory of the generated module.
        mod_directory: PathBuf,
        /// Whether to run `rustfmt` on the generated files.
        pretty: bool,
        /// Whether to gate each key on its crate name. This allows using the module
        /// even if some keys are not required and their dependencies are not included.
        insert_feature_gates: bool,
    },
}

/// Runs ICU4X datagen.
///
/// The argument are used as follows:
/// * `locales`: If this is present, only locales that are either `und` or
///   contained (strictly, i.e. `en` != `en-US`) in the slice will be generated.
///   Otherwise, all locales supported by the source data will be generated.
/// * `keys`: The keys for which to generate data. See [`get_all_keys()`].
/// * `sources`: The underlying source data. CLDR and/or uprops data can be missing if no
///   requested key requires them, otherwise [`MISSING_CLDR_ERROR`] or [`MISSING_UPROPS_ERROR`]
///   will be returned.
/// * `out`: The output format and location. See the documentation on [`Out`]
pub fn datagen(
    locales: Option<&[LanguageIdentifier]>,
    keys: &[ResourceKey],
    sources: &SourceData,
    outs: Vec<Out>,
) -> Result<(), DataError> {
    let exporters = outs
        .into_iter()
        .map(|out| -> Result<Box<dyn DataExporter>, DataError> {
            Ok(match out {
                Out::Fs {
                    output_path,
                    serializer,
                    overwrite,
                } => {
                    let mut options =
                        icu_provider_fs::export::fs_exporter::ExporterOptions::default();
                    options.root = output_path;
                    if overwrite {
                        options.overwrite =
                            icu_provider_fs::export::fs_exporter::OverwriteOption::RemoveAndReplace
                    }
                    Box::new(icu_provider_fs::export::FilesystemExporter::try_new(
                        serializer, options,
                    )?)
                }
                Out::Blob(write) => Box::new(
                    icu_provider_blob::export::BlobExporter::new_with_sink(write),
                ),
                Out::Module {
                    mod_directory,
                    pretty,
                    insert_feature_gates,
                } => Box::new(crabbake::ConstExporter::new(
                    mod_directory,
                    pretty,
                    insert_feature_gates,
                )),
            })
        })
        .collect::<Result<Vec<_>, DataError>>()?;

    let mut provider: Box<dyn ExportableProvider> = Box::new(create_datagen_provider!(*sources));

    if let Some(locales) = locales {
        let locales = locales.to_vec();
        provider = Box::new(
            provider
                .filterable("icu4x-datagen locales")
                .filter_by_langid(move |lid| lid.language.is_empty() || locales.contains(lid)),
        );
    }

    keys.into_par_iter().try_for_each(|&key| {
        let options = provider.supported_options_for_key(key)?;
        log::info!("Writing key: {}", key);
        let res = options.into_par_iter().try_for_each(|options| {
            let req = DataRequest {
                options: options.clone(),
                metadata: Default::default(),
            };
            let payload = provider
                .load_payload(key, &req)
                .and_then(DataResponse::take_payload)
                .map_err(|e| e.with_req(key, &req))?;
            exporters
                .par_iter()
                .try_for_each(|e| e.put_payload(key, &options, &payload))
        });

        for e in &exporters {
            e.flush(key)?;
        }

        res
    })?;

    for mut e in exporters {
        e.close()?;
    }

    Ok(())
}

#[test]
fn test_keys() {
    assert_eq!(
        keys(&["list/and@1", "datetime/lengths@1", "trash",]),
        vec![
            icu_datetime::provider::calendar::DatePatternsV1Marker::KEY,
            icu_list::provider::AndListV1Marker::KEY,
        ]
    );
}

#[test]
fn test_keys_from_file() {
    let file = std::fs::File::open(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/work_log+keys.txt"),
    )
    .unwrap();
    assert_eq!(
        keys_from_file(file).unwrap(),
        vec![
            icu_datetime::provider::calendar::DatePatternsV1Marker::KEY,
            icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY,
            icu_datetime::provider::calendar::DateSymbolsV1Marker::KEY,
            icu_datetime::provider::week_data::WeekDataV1Marker::KEY,
            icu_plurals::provider::OrdinalV1Marker::KEY,
        ]
    );
}
