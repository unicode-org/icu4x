// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
// #![cfg_attr(not(any(test, doc)), no_std)]
// #![cfg_attr(
//     not(test),
//     deny(
//         clippy::indexing_slicing,
//         clippy::unwrap_used,
//         clippy::expect_used,
//         clippy::panic,
//     )
// )]
#![warn(missing_docs)]

//! The command line interface for ICU4X datagen.
//!
//! ```bash
//! $ cargo install icu4x-datagen
//! $ icu4x-datagen --markers all --locales de en-AU --format blob --out data.postcard
//! ```
//!
//! More details can be found by running `--help`, or by consulting the [`icu_provider_export`] documentation.

// If no exporter feature is enabled this all doesn't make sense
#![cfg_attr(
    not(any(
        feature = "baked_exporter",
        feature = "blob_exporter",
        feature = "fs_exporter",
    )),
    allow(unused_assignments, unreachable_code, unused_variables)
)]
// If no source feature is enabled this all doesn't make sense
#![cfg_attr(
    not(any(feature = "provider", feature = "blob_input",)),
    allow(unused_assignments, unreachable_code, unused_variables)
)]

use clap::{Parser, ValueEnum};
use displaydoc::Display;
use eyre::WrapErr;
use icu_provider::export::ExportableProvider;
use icu_provider::hello_world::HelloWorldV1;
use icu_provider::DataError;
use icu_provider_export::prelude::*;
use icu_provider_export::ExportMetadata;
#[cfg(feature = "provider")]
use icu_provider_source::SourceDataProvider;
use regex::Regex;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone)]
struct Filter {
    domain: String,
    regex: Regex,
    inverted: bool,
}

#[derive(Debug, Display)]
enum FilterError {
    #[displaydoc("no filter found. specify one after an =")]
    NoFilter,
    #[displaydoc("opening / delimiter for regex not found")]
    NoOpeningSlash,
    #[displaydoc("closing / delimiter for regex not found")]
    NoClosingSlash,
    #[displaydoc("{0}")]
    Regex(regex::Error),
}

impl From<regex::Error> for FilterError {
    fn from(value: regex::Error) -> Self {
        Self::Regex(value)
    }
}

impl std::error::Error for FilterError {}

impl FromStr for Filter {
    type Err = FilterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (domain, regex) = s.split_once('=').ok_or(FilterError::NoFilter)?;

        let (regex, inverted) = regex
            .strip_prefix('-')
            .map(|regex| (regex, true))
            .unwrap_or((regex, false));

        let regex = regex.strip_prefix('/').ok_or(FilterError::NoOpeningSlash)?;
        let regex = regex.strip_suffix('/').ok_or(FilterError::NoClosingSlash)?;

        // add an implicit `^(?:)$` around the regex
        let regex = format!("^(?:{})$", regex);
        let regex = Regex::new(&regex)?;

        Ok(Self {
            domain: domain.to_owned(),
            regex,
            inverted,
        })
    }
}

#[derive(Parser)]
#[command(name = "icu4x-datagen")]
#[command(author = "The ICU4X Project Developers", version = option_env!("CARGO_PKG_VERSION"))]
#[command(about = format!("Learn more at: https://docs.rs/icu_provider_export/{}", option_env!("CARGO_PKG_VERSION").unwrap_or("")), long_about = None)]
struct Cli {
    #[arg(short, long)]
    #[arg(help = "Requests verbose output")]
    verbose: bool,

    #[arg(long, value_enum)]
    #[arg(
        help = "Select the output format: a directory tree of files (fs), a single blob (blob), or a Rust module (baked)."
    )]
    format: Format,

    #[arg(short = 'W', long)]
    #[arg(help = "Delete the output before writing data.")]
    overwrite: bool,

    #[arg(short, long, value_enum, default_value_t = Syntax::Json)]
    #[arg(help = "--format=fs only: serde serialization format.")]
    syntax: Syntax,

    #[arg(short, long)]
    #[arg(help = "--format=baked, --format=fs only: pretty-print the Rust or JSON output files.")]
    pretty: bool,

    #[arg(short = 't', long, value_name = "TAG", default_value = "latest")]
    #[arg(
        help = "Download CLDR JSON data from this GitHub tag (https://github.com/unicode-org/cldr-json/tags)\n\
                    Use 'latest' for the latest version verified to work with this version of the binary.\n\
                    Ignored if '--cldr-root' is present. Requires binary to be built with `networking` Cargo feature (enabled by default).\n\
                    Note that some markers do not support versions before 41.0.0."
    )]
    #[cfg(feature = "provider")]
    #[cfg_attr(not(feature = "networking"), arg(hide = true))]
    cldr_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(
        help = "Path to a local cldr-{version}-json-full.zip directory (see https://github.com/unicode-org/cldr-json/releases).\n\
                  Note that some markers do not support versions before 41.0.0."
    )]
    #[cfg(feature = "provider")]
    cldr_root: Option<PathBuf>,

    #[arg(long, value_name = "TAG", default_value = "latest")]
    #[arg(
        help = "Download ICU data from this GitHub tag (https://github.com/unicode-org/icu/tags)\n\
                  Use 'latest' for the latest version verified to work with this version of the binary.\n\
                  Ignored if '--icuexport-root' is present. Requires binary to be built with `networking` Cargo feature (enabled by default).\n\
                  Note that some markers do not support versions before release-71-1."
    )]
    #[cfg_attr(not(feature = "networking"), arg(hide = true))]
    #[cfg(feature = "provider")]
    icuexport_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(
        help = "Path to a local icuexport directory (see https://github.com/unicode-org/icu/releases).\n\
                  Note that some markers do not support versions before release-71-1."
    )]
    #[cfg(feature = "provider")]
    icuexport_root: Option<PathBuf>,

    #[arg(long, value_name = "TAG", default_value = "latest")]
    #[arg(
        help = "Download segmentation LSTM models from this GitHub tag (https://github.com/unicode-org/lstm_word_segmentation/tags)\n\
                  Use 'latest' for the latest version verified to work with this version of the binary.\n\
                  Ignored if '--segmenter-lstm-root' is present. Requires binary to be built with `networking` Cargo feature (enabled by default)."
    )]
    #[cfg_attr(not(feature = "networking"), arg(hide = true))]
    #[cfg(feature = "provider")]
    segmenter_lstm_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(
        help = "Path to a local segmentation LSTM directory (see https://github.com/unicode-org/lstm_word_segmentation/releases)."
    )]
    #[cfg(feature = "provider")]
    segmenter_lstm_root: Option<PathBuf>,

    #[arg(long, value_name = "TAG", default_value = "latest")]
    #[arg(
        help = "Download tzdb from this IANA tag (https://data.iana.org/time-zones/releases/)\n\
                  Use 'latest' for the latest version verified to work with this version of the binary.\n\
                  Ignored if '--tzdb-root' is present. Requires binary to be built with `networking` Cargo feature (enabled by default)."
    )]
    #[cfg_attr(not(feature = "networking"), arg(hide = true))]
    #[cfg(feature = "provider")]
    tzdb_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(help = "Path to a local tzdb directory \
                (see any zip file from https://data.iana.org/time-zones/releases/, \
                directory structure matching https://data.iana.org/time-zones/tzdb-2025a/).")]
    #[cfg(feature = "provider")]
    tzdb_root: Option<PathBuf>,

    #[arg(long, value_enum, default_value_t = TrieType::Small)]
    #[arg(
        help = "Whether to optimize CodePointTrie data structures for size (\"small\") or speed (\"fast\").\n\
                  Using \"fast\" mode increases performance of CJK text processing and segmentation. For more\n\
                  information, see the TrieType enum. The tries for the core (UAX #15 but not UAX #46)\n\
                  normalization forms use the fast trie type regardless of this setting."
    )]
    #[cfg(feature = "provider")]
    trie_type: TrieType,

    #[arg(long, value_enum, default_value_t = CollationRootHan::Implicit)]
    #[arg(help = "Which collation han database to use.")]
    #[cfg(feature = "provider")]
    collation_root_han: CollationRootHan,

    #[arg(long, value_enum, num_args = 1..)]
    #[arg(
        help = "Which less-common collation tables to include. 'search-all' includes all search tables."
    )]
    include_collations: Vec<CollationTable>,

    #[arg(long, short, num_args = 1..)]
    #[arg(
        help = "Include these data markers in the output. Accepts multiple arguments.\n\
                Set to 'all' for all markers, or 'none' for no markers."
    )]
    markers: Vec<String>,

    #[arg(long, value_name = "BINARY")]
    #[arg(help = "Analyzes the binary and only includes markers that are used by the binary.")]
    markers_for_bin: Option<PathBuf>,

    #[arg(long, value_name = "FILTER")]
    #[arg(help = "Filter attributes on markers for a domain. Accepts form `domain=/regex/`.")]
    attribute_filter: Vec<Filter>,

    #[arg(long, short, num_args = 0..)]
    #[cfg_attr(feature = "provider", arg(default_value = "recommended"))]
    #[arg(
        help = "Include this locale in the output. Accepts multiple arguments. \
                  Set to 'full' or 'modern' for the respective CLDR locale sets, 'none' for no locales, \
                  or 'recommended' for the recommended set of locales."
    )]
    locales: Vec<String>,

    #[arg(long = "out", short, value_name = "PATH")]
    #[arg(
        help = "Path to output directory or file. Must be empty or non-existent, unless \
                  --overwrite is present, in which case the directory is deleted first. \
                  For --format=blob, omit this option to dump to stdout. \
                  For --format={dir,mod} defaults to 'icu4x_data'."
    )]
    output: Option<PathBuf>,

    #[arg(long)]
    #[arg(
        help = "--format=baked only: use types from individual `icu_*` crates instead of the `icu` meta-crate."
    )]
    use_separate_crates: bool,

    #[arg(long)]
    #[arg(help = "--format=baked only: don't include fallback code inside the baked provider")]
    no_internal_fallback: bool,

    #[arg(long, value_enum)]
    #[arg(
        help = "configures the deduplication of locales for exported data payloads. \
                If not set, determined by the export format: \
                if --format=baked, a more aggressive deduplication strategy is used."
    )]
    deduplication: Option<Deduplication>,

    #[arg(long, num_args = 0.., default_value = "recommended")]
    #[arg(
        help = "Include these segmenter models in the output. Accepts multiple arguments. \
                Defaults to 'recommended' for the recommended set of models. Use 'none' for no models"
    )]
    segmenter_models: Vec<String>,

    #[arg(long)]
    #[arg(help = "Use data from this blob file instead of generating it from sources")]
    #[cfg(feature = "blob_input")]
    input_blob: Option<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format {
    Fs,
    Blob,
    Baked,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Syntax {
    Json,
    Bincode,
    Postcard,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum TrieType {
    Small,
    Fast,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
// Mirrors icu_provider_export::CollationRootHan
enum CollationRootHan {
    Unihan,
    Implicit,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum CollationTable {
    Search,
    Searchji,
    #[value(alias = "search*")] // for backwards compatability
    SearchAll,
}

impl CollationTable {
    fn to_datagen_value(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::Searchji => "searchji",
            Self::SearchAll => "search*",
        }
    }
}

// Mirrors icu_provider_export::DeduplicationStrategy
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Deduplication {
    Maximal,
    RetainBaseLanguages,
    None,
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            // wasmer logging is very noisy
            .with_module_level("wasmer", log::LevelFilter::Warn)
            .init()
            .unwrap()
    } else {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .env()
            .init()
            .unwrap()
    }

    run(cli)
}

fn run(cli: Cli) -> eyre::Result<()> {
    let markers = if !cli.markers.is_empty() {
        match cli.markers.as_slice() {
            [x] if x == "none" => Default::default(),
            [x] if x == "all" => {
                #[cfg(feature = "experimental")]
                log::info!("The icu4x-datagen crate has been built with the `experimental` feature, so `--markers all` includes experimental markers");
                #[cfg(not(feature = "experimental"))]
                log::info!("The icu4x-datagen crate has been built without the `experimental` feature, so `--markers all` does not include experimental markers");
                all_markers()
            }
            markers => markers
                .iter()
                .map(|k| match marker_lookup().get(k.as_str()) {
                    Some(Some(marker)) => Ok(*marker),
                    Some(None) => {
                        eyre::bail!("Marker {k:?} requires `experimental` Cargo feature")
                    }
                    None => eyre::bail!("Unknown marker {k:?}"),
                })
                .collect::<Result<_, _>>()?,
        }
    } else if let Some(bin_path) = &cli.markers_for_bin {
        icu::markers_for_bin(&std::fs::read(bin_path)?)?
            .into_iter()
            .collect()
    } else {
        eyre::bail!("--markers or --markers-for-bin are required.")
    };

    enum PreprocessedLocales {
        Locales(Vec<DataLocale>),
        Full,
    }

    #[allow(unused_mut)]
    let mut preprocessed_locales = if cli.locales.as_slice() == ["none"] {
        Some(PreprocessedLocales::Locales(vec![]))
    } else if cli.locales.as_slice() == ["full"] {
        Some(PreprocessedLocales::Full)
    } else {
        if cli.locales.as_slice() == ["all"] {
            log::warn!(
                "`--locales all` selects the Allar language. Use `--locales full` for all locales"
            );
        }
        None
    };

    fn missing_data_message<T>(e: DataError) -> Result<T, eyre::Report> {
        #[cfg(feature = "provider")]
        if SourceDataProvider::is_missing_cldr_error(e) {
            eyre::bail!("CLDR data is required for this invocation, set --cldr-path or --cldr-tag");
        } else if SourceDataProvider::is_missing_icuexport_error(e) {
            eyre::bail!(
                "ICU data is required for this invocation, set --icuexport-path or --icuexport-tag"
            );
        } else if SourceDataProvider::is_missing_segmenter_lstm_error(e) {
            eyre::bail!("Segmentation LSTM data is required for this invocation, set --segementer-lstm-path or --segementer-lstm-tag");
        } else if SourceDataProvider::is_missing_tzdb_error(e) {
            eyre::bail!(
                "Timezone data is required for this invocation, set --tzdb-path or --tzdb-tag"
            );
        }

        Err(e.into())
    }

    let (provider, fallbacker): (Box<dyn ExportableProvider>, _) = match () {
        () if markers == [HelloWorldV1::INFO] => {
            // Just do naive fallback instead of pulling in compiled data or something. We only use this code path to debug
            // providers, so we don't need 100% correct fallback.
            (Box::new(icu_provider::hello_world::HelloWorldProvider), LocaleFallbacker::new_without_data())
        }
        () if markers.contains(&HelloWorldV1::INFO) => {
            eyre::bail!("HelloWorldV1 is only allowed as the only marker")
        }
        #[cfg(feature = "blob_input")]
        () if cli.input_blob.is_some() => {
            let provider = icu_provider_blob::BlobDataProvider::try_new_from_blob(
                std::fs::read(cli.input_blob.unwrap())?.into(),
            )?;
            let fallbacker = LocaleFallbacker::try_new_with_buffer_provider(&provider)?;
            (Box::new(ReexportableBlobDataProvider(provider)), fallbacker)
        },

        #[cfg(all(not(feature = "provider"), feature = "blob_input"))]
        () => eyre::bail!("--input-blob is required without the `provider` Cargo feature"),

        #[cfg(feature = "provider")]
        () => {
            let mut p = SourceDataProvider::new_custom();

            p = p.with_collation_root_han(match cli.collation_root_han {
                CollationRootHan::Unihan => icu_provider_source::CollationRootHan::Unihan,
                CollationRootHan::Implicit => {
                    icu_provider_source::CollationRootHan::Implicit
                }
            });

            if cli.trie_type == TrieType::Fast {
                p = p.with_fast_tries();
            }

            p = match (cli.cldr_root, cli.cldr_tag.as_str()) {
                (Some(path), _) => p.with_cldr(&path)?,
                #[cfg(feature = "networking")]
                (_, "latest") => p.with_cldr_for_tag(SourceDataProvider::TESTED_CLDR_TAG),
                #[cfg(feature = "networking")]
                (_, tag) => p.with_cldr_for_tag(tag),
                #[cfg(not(feature = "networking"))]
                (None, _) => p,
            };

            p = match (cli.icuexport_root, cli.icuexport_tag.as_str()) {
                (Some(path), _) => p.with_icuexport(&path)?,
                #[cfg(feature = "networking")]
                (_, "latest") => {
                    p.with_icuexport_for_tag(SourceDataProvider::TESTED_ICUEXPORT_TAG)
                }
                #[cfg(feature = "networking")]
                (_, tag) => p.with_icuexport_for_tag(tag),
                #[cfg(not(feature = "networking"))]
                (None, _) => p,
            };

            p = match (cli.segmenter_lstm_root, cli.segmenter_lstm_tag.as_str()) {
                (Some(path), _) => p.with_segmenter_lstm(&path)?,
                #[cfg(feature = "networking")]
                (_, "latest") => {
                    p.with_segmenter_lstm_for_tag(SourceDataProvider::TESTED_SEGMENTER_LSTM_TAG)
                }
                #[cfg(feature = "networking")]
                (_, tag) => p.with_segmenter_lstm_for_tag(tag),
                #[cfg(not(feature = "networking"))]
                (None, _) => p,
            };

            p = match (cli.tzdb_root, cli.tzdb_tag.as_str()) {
                (Some(path), _) => p.with_tzdb(&path)?,
                #[cfg(feature = "networking")]
                (_, "latest") => {
                    p.with_tzdb_for_tag(SourceDataProvider::TESTED_TZDB_TAG)
                }
                #[cfg(feature = "networking")]
                (_, tag) => p.with_tzdb_for_tag(tag),
                #[cfg(not(feature = "networking"))]
                (None, _) => p,
            };

            if cli.locales.as_slice() == ["recommended"] {
                preprocessed_locales = Some(PreprocessedLocales::Locales(
                    p.locales_for_coverage_levels([
                        icu_provider_source::CoverageLevel::Modern,
                        icu_provider_source::CoverageLevel::Moderate,
                        icu_provider_source::CoverageLevel::Basic,
                    ])?
                    .into_iter()
                    .collect(),
                ));
            } else if let Some(locale_subsets) = cli
                .locales
                .iter()
                .map(|s| match &**s {
                    "basic" => Some(icu_provider_source::CoverageLevel::Basic),
                    "moderate" => Some(icu_provider_source::CoverageLevel::Moderate),
                    "modern" => Some(icu_provider_source::CoverageLevel::Modern),
                    _ => None,
                })
                .collect::<Option<Vec<_>>>()
            {
                preprocessed_locales = Some(PreprocessedLocales::Locales(
                    p.locales_for_coverage_levels(locale_subsets.into_iter())?
                        .into_iter()
                        .collect(),
                ));
            }

            let fallbacker = LocaleFallbacker::try_new_unstable(&p).or_else(missing_data_message)?;
            (Box::new(p), fallbacker)
        }

        #[cfg(not(any(feature = "provider", feature = "blob_input")))]
        () => eyre::bail!("Only the `HelloWorld marker is supported without Cargo features `blob_input` or `provider`"),
    };

    let locale_families = match preprocessed_locales {
        Some(PreprocessedLocales::Full) => vec![DataLocaleFamily::FULL],
        Some(PreprocessedLocales::Locales(locales)) => locales
            .into_iter()
            .map(DataLocaleFamily::with_descendants)
            .collect(),
        None => cli
            .locales
            .into_iter()
            .map(|family_str| family_str.parse().wrap_err(family_str))
            .collect::<eyre::Result<Vec<_>>>()?,
    };

    let deduplication_strategy = match cli.deduplication {
        Some(Deduplication::Maximal) => DeduplicationStrategy::Maximal,
        Some(Deduplication::RetainBaseLanguages) => {
            DeduplicationStrategy::RetainBaseLanguages
        }
        Some(Deduplication::None) => DeduplicationStrategy::None,
        None => match cli.format {
            Format::Fs | Format::Blob => DeduplicationStrategy::None,
            Format::Baked if cli.no_internal_fallback && cli.deduplication.is_none() =>
                eyre::bail!("--no-internal-fallback requires an explicit --deduplication value. Baked exporter would default to maximal deduplication, which might not be intended"),
            Format::Baked => DeduplicationStrategy::Maximal,
        }
    };

    let mut driver = ExportDriver::new(locale_families, deduplication_strategy.into(), fallbacker);

    driver = driver.with_markers(markers);

    driver = driver.with_additional_collations(
        cli.include_collations
            .iter()
            .map(|c| c.to_datagen_value().to_owned()),
    );
    driver = if cli.segmenter_models.as_slice() == ["none"] {
        driver.with_segmenter_models([])
    } else if cli.segmenter_models.as_slice() == ["recommended"] {
        driver.with_segmenter_models([
            "Burmese_codepoints_exclusive_model4_heavy".into(),
            "burmesedict".into(),
            "cjdict".into(),
            "Khmer_codepoints_exclusive_model4_heavy".into(),
            "khmerdict".into(),
            "Lao_codepoints_exclusive_model4_heavy".into(),
            "laodict".into(),
            "Thai_codepoints_exclusive_model4_heavy".into(),
            "thaidict".into(),
        ])
    } else {
        driver.with_segmenter_models(cli.segmenter_models.clone())
    };

    let attribute_filters = cli.attribute_filter.into_iter().fold(
        HashMap::<_, Vec<(Regex, bool)>>::new(),
        |mut map, filter| {
            map.entry(filter.domain)
                .or_default()
                .push((filter.regex, filter.inverted));
            map
        },
    );
    for (domain, filters) in attribute_filters {
        driver = driver.with_marker_attributes_filter(&domain, move |attr| {
            filters
                .iter()
                .all(|(regex, inverted)| regex.is_match(attr) ^ inverted)
        })
    }

    let metadata: Result<ExportMetadata, DataError> = match cli.format {
        #[cfg(not(feature = "fs_exporter"))]
        Format::Fs => {
            eyre::bail!("Exporting to an FsProvider requires the `fs_exporter` Cargo feature")
        }
        #[cfg(feature = "fs_exporter")]
        Format::Fs => driver.export(&provider, {
            use icu_provider_export::fs_exporter::*;

            FilesystemExporter::try_new(
                match cli.syntax {
                    Syntax::Bincode => Box::<serializers::Bincode>::default(),
                    Syntax::Postcard => Box::<serializers::Postcard>::default(),
                    Syntax::Json if cli.pretty => Box::new(serializers::Json::pretty()),
                    Syntax::Json => Box::<serializers::Json>::default(),
                },
                {
                    let mut options = Options::default();
                    options.root = cli.output.unwrap_or_else(|| PathBuf::from("icu4x_data"));
                    if cli.overwrite {
                        options.overwrite = OverwriteOption::RemoveAndReplace
                    }
                    options
                },
            )?
        }),
        #[cfg(not(feature = "blob_exporter"))]
        Format::Blob => {
            eyre::bail!("Exporting to a BlobProvider requires the `blob_exporter` Cargo feature")
        }
        #[cfg(feature = "blob_exporter")]
        Format::Blob => driver.export(&provider, {
            use icu_provider_export::blob_exporter::*;

            let sink: Box<dyn std::io::Write + Sync> = if let Some(path) = cli.output {
                if !cli.overwrite && path.exists() {
                    eyre::bail!("Output path is present: {:?}", path);
                }
                Box::new(
                    std::fs::File::create(&path)
                        .with_context(|| path.to_string_lossy().to_string())?,
                )
            } else {
                Box::new(std::io::stdout())
            };
            BlobExporter::new_with_sink(sink)
        }),
        #[cfg(not(feature = "baked_exporter"))]
        Format::Baked => {
            eyre::bail!("Exporting to a baked provider requires the `baked_exporter` Cargo feature")
        }
        #[cfg(feature = "baked_exporter")]
        Format::Baked => driver.export(&provider, {
            icu_provider_export::baked_exporter::BakedExporter::new(
                cli.output.unwrap_or_else(|| PathBuf::from("icu4x_data")),
                {
                    let mut options = icu_provider_export::baked_exporter::Options::default();
                    options.pretty = cli.pretty;
                    options.use_internal_fallback = !cli.no_internal_fallback;
                    options.use_separate_crates = cli.use_separate_crates;
                    options.overwrite = cli.overwrite;
                    options
                },
            )?
        }),
    };

    let _metadata = metadata.or_else(missing_data_message)?;

    Ok(())
}

macro_rules! cb {
    ($($marker_ty:ty:$marker:ident,)+ #[experimental] $($emarker_ty:ty:$emarker:ident,)+) => {
        fn all_markers() -> Vec<DataMarkerInfo> {
            vec![
                $(
                    <$marker_ty>::INFO,
                )+
                $(
                    #[cfg(feature = "experimental")]
                    <$emarker_ty>::INFO,
                )+
            ]
        }

        fn marker_lookup() -> &'static HashMap<String, Option<DataMarkerInfo>> {
            use std::sync::OnceLock;
            static LOOKUP: OnceLock<HashMap<String, Option<DataMarkerInfo>>> = OnceLock::new();
            LOOKUP.get_or_init(|| {
                vec![
                    (stringify!(icu_provider::hello_world::HelloWorldV1).replace(' ', ""), Some(icu_provider::hello_world::HelloWorldV1::INFO)),
                    (stringify!(HelloWorldV1).into(), Some(icu_provider::hello_world::HelloWorldV1::INFO)),
                    $(
                        (stringify!($marker_ty).replace(' ', ""), Some(<$marker_ty>::INFO)),
                        (stringify!($marker).into(), Some(<$marker_ty>::INFO)),
                    )+
                    $(
                        #[cfg(feature = "experimental")]
                        (stringify!($emarker_ty).replace(' ', ""), Some(<$emarker_ty>::INFO)),
                        #[cfg(feature = "experimental")]
                        (stringify!($emarker).into(), Some(<$emarker_ty>::INFO)),
                        #[cfg(not(feature = "experimental"))]
                        (stringify!($emarker_ty).replace(' ', ""), None),
                        #[cfg(not(feature = "experimental"))]
                        (stringify!($emarker).into(), None),
                    )+

                ]
                .into_iter()
                .collect()
            })
        }

        #[test]
        fn test_lookup() {
            assert_eq!(marker_lookup().get("ListAndV1"), Some(&Some(icu::list::provider::ListAndV1::INFO)));
            assert_eq!(marker_lookup().get("icu::list::provider::ListAndV1"), Some(&Some(icu::list::provider::ListAndV1::INFO)));
            assert_eq!(marker_lookup().get("foo"), None);
        }


        #[cfg(feature = "blob_input")]
        icu_provider::export::make_exportable_provider!(
            ReexportableBlobDataProvider,
            [
                icu_provider::hello_world::HelloWorldV1,
                $(
                    $marker_ty,
                )+
                $(
                    #[cfg(feature = "experimental")]
                    $emarker_ty,
                )+
            ]
        );
    }
}

extern crate alloc;
icu_provider_registry::registry!(cb);

#[cfg(feature = "blob_input")]
use icu_provider::buf::DeserializingBufferProvider;
#[cfg(feature = "blob_input")]
use icu_provider::prelude::*;
#[cfg(feature = "blob_input")]
use icu_provider_blob::BlobDataProvider;

#[cfg(feature = "blob_input")]
struct ReexportableBlobDataProvider(icu_provider_blob::BlobDataProvider);

#[cfg(feature = "blob_input")]
impl<M: DataMarker> DataProvider<M> for ReexportableBlobDataProvider
where
    BlobDataProvider: AsDeserializingBufferProvider,
    for<'a> DeserializingBufferProvider<'a, BlobDataProvider>: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        self.0.as_deserializing().load(req)
    }
}

#[cfg(feature = "blob_input")]
impl<M: DataMarker> IterableDataProvider<M> for ReexportableBlobDataProvider
where
    BlobDataProvider: AsDeserializingBufferProvider,
    for<'a> DeserializingBufferProvider<'a, BlobDataProvider>: DataProvider<M>,
{
    fn iter_ids(&self) -> Result<std::collections::BTreeSet<DataIdentifierCow<'_>>, DataError> {
        self.0.iter_ids_for_marker(M::INFO)
    }
}

#[test]
fn test_attributes_regex() {
    let out = std::env::temp_dir().join("icu4x-datagen_test_attributes_regex_out");
    let _ = std::fs::remove_dir_all(&out);

    let mut args = Cli::parse_from([
        "bin",
        "--markers",
        "HelloWorldV1",
        "--locales",
        "full",
        "--format",
        "fs",
        "--attribute-filter",
        "hello=/r.*?|.*?case/",
        "--attribute-filter",
        "hello=-/lowercase/",
        "--attribute-filter",
        "hello=-/.*3/",
    ]);

    args.output = Some(out.clone());

    run(args).unwrap();

    assert!(std::fs::exists(out.join("hello/world/v1/reverse")).unwrap());

    assert!(std::fs::exists(out.join("hello/world/v1/rotate1")).unwrap());
    assert!(std::fs::exists(out.join("hello/world/v1/rotate2")).unwrap());
    assert!(!std::fs::exists(out.join("hello/world/v1/rotate3")).unwrap());

    assert!(std::fs::exists(out.join("hello/world/v1/uppercase")).unwrap());
    assert!(!std::fs::exists(out.join("hello/world/v1/lowercase")).unwrap());
}
