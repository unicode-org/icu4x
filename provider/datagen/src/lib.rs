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
//!         &SourceData::default().with_uprops(PathBuf::from("/path/to/uprops/root")),
//!         Out::Blob(Box::new(File::create("data.postcard").unwrap())),
//!         false,
//!     ).unwrap();
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

#![warn(missing_docs)]

mod error;
mod registry;
mod source;
pub mod transform;

pub use error::{is_missing_cldr_error, is_missing_uprops_error};
pub use registry::get_all_keys;
pub use source::SourceData;

use icu_locid::LanguageIdentifier;
use icu_provider::datagen::IterableDynProvider;
use icu_provider::export::DataExporter;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
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
}

/// Runs ICU4X datagen.
///
/// The argument are used as follows:
/// * `locales`: If this is present, only locales that are either `und` or
///   contained (strictly, i.e. `en` != `en-US`) in the slice will be generated.
///   Otherwise, all locales supported by the source data will be generated.
/// * `keys`: The keys for which to generate data. See [`get_all_keys()`].
/// * `sources`: The underlying source data. CLDR and/or uprops data can be missing if no
///   requested key requires them. Otherwise a error will be returned that can be identified
///   with [`is_missing_cldr_error`] or [`is_missing_uprops_error`].
/// * `out`: The output format and location. See the documentation on [`Out`]
/// * `ignore_missing_resource_keys`: some keys are not supported by datagen yet. Using
///   all keys will not work unless this option is set.
pub fn datagen(
    locales: Option<&[LanguageIdentifier]>,
    keys: &[ResourceKey],
    sources: &SourceData,
    out: Out,
    ignore_missing_resource_keys: bool,
) -> Result<(), DataError> {
    let mut provider: Box<dyn IterableDynProvider<SerializeMarker> + Sync> =
        Box::new(create_datagen_provider!(*sources));

    if let Some(locales) = locales.as_ref() {
        provider = Box::new(
            provider
                .filterable("icu4x-datagen locales")
                .filter_by_langid(move |lid| lid.language.is_empty() || locales.contains(lid)),
        );
    }

    let mut exporter: Box<dyn DataExporter<_>> = match out {
        Out::Fs {
            output_path,
            serializer,
            overwrite,
        } => {
            let mut options = icu_provider_fs::export::fs_exporter::ExporterOptions::default();
            options.root = output_path;
            if overwrite {
                options.overwrite =
                    icu_provider_fs::export::fs_exporter::OverwriteOption::RemoveAndReplace
            }
            Box::new(icu_provider_fs::export::FilesystemExporter::try_new(
                serializer, options,
            )?)
        }
        Out::Blob(write) => Box::new(icu_provider_blob::export::BlobExporter::new_with_sink(
            write,
        )),
    };

    keys.into_par_iter().try_for_each(|&key| {
        let result = provider
            .supported_options_for_key(key)?
            .collect::<Vec<_>>()
            .into_par_iter()
            .try_for_each(|options| {
                let req = DataRequest {
                    options: options.clone(),
                    metadata: Default::default(),
                };
                let payload = provider
                    .load_payload(key, &req)
                    .and_then(DataResponse::take_payload)
                    .map_err(|e| e.with_req(key, &req))?;
                exporter.put_payload(key, options, payload)
            });

        exporter.flush(key)?;

        if ignore_missing_resource_keys
            && matches!(result, Err(e) if e.kind == DataErrorKind::MissingResourceKey)
        {
            log::trace!("Skipping key: {}", key);
            Ok(())
        } else {
            log::info!("Writing key: {}", key);
            result
        }
    })?;

    exporter.close()?;

    Ok(())
}
