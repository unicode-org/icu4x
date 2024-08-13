// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
        feature = "blob_exporter",
        feature = "fs_exporter",
        feature = "baked_exporter"
    )),
    allow(unused_assignments, unreachable_code, unused_variables)
)]
// If no source feature is enabled this all doesn't make sense
#![cfg_attr(
    not(any(feature = "provider", feature = "blob_input",)),
    allow(unused_assignments, unreachable_code, unused_variables)
)]

use clap::{Parser, ValueEnum};
use eyre::WrapErr;
use icu_provider::export::ExportableProvider;
use icu_provider::hello_world::HelloWorldV1Marker;
use icu_provider_export::prelude::*;
#[cfg(feature = "provider")]
use icu_provider_source::SourceDataProvider;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::path::PathBuf;

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
        help = "Select the output format: a directory tree of files, a single blob, or a Rust module."
    )]
    format: Format,

    #[arg(short = 'W', long)]
    #[arg(help = "Delete the output before writing data.")]
    overwrite: bool,

    #[arg(short, long, value_enum, default_value_t = Syntax::Json)]
    #[arg(help = "--format=dir only: serde serialization format.")]
    syntax: Syntax,

    #[arg(short, long)]
    #[arg(help = "--format=mod, --format=dir only: pretty-print the Rust or JSON output files.")]
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

    #[arg(long, value_enum, default_value_t = TrieType::Small)]
    #[arg(
        help = "Whether to optimize CodePointTrie data structures for size (\"small\") or speed (\"fast\").\n\
                  Using \"fast\" mode increases performance of CJK text processing and segmentation. For more\n\
                  information, see the TrieType enum."
    )]
    #[cfg(feature = "provider")]
    trie_type: TrieType,

    #[arg(long, value_enum, default_value_t = CollationHanDatabase::Implicit)]
    #[arg(help = "Which collation han database to use.")]
    #[cfg(feature = "provider")]
    collation_han_database: CollationHanDatabase,

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
                  For --format={blob,blob2}, omit this option to dump to stdout. \
                  For --format={dir,mod} defaults to 'icu4x_data'."
    )]
    output: Option<PathBuf>,

    #[arg(long)]
    #[arg(
        help = "--format=mod only: use types from individual `icu_*` crates instead of the `icu` meta-crate."
    )]
    use_separate_crates: bool,

    #[arg(long)]
    #[arg(help = "--format=mod only: don't include fallback code inside the baked provider")]
    no_internal_fallback: bool,

    #[arg(long, value_enum)]
    #[arg(
        help = "configures the deduplication of locales for exported data payloads. \
                If not set, determined by the export format: \
                if --format=mod, a more aggressive deduplication strategy is used."
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
    Dir,
    Blob,
    Blob2,
    Mod,
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
// Mirrors crate::CollationHanDatabase
enum CollationHanDatabase {
    Unihan,
    Implicit,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum CollationTable {
    Gb2312,
    Big5han,
    Search,
    Searchji,
    #[value(alias = "search*")] // for backwards compatability
    SearchAll,
}

impl CollationTable {
    fn to_datagen_value(self) -> &'static str {
        match self {
            Self::Gb2312 => "gb2312",
            Self::Big5han => "big5han",
            Self::Search => "search",
            Self::Searchji => "searchji",
            Self::SearchAll => "search*",
        }
    }
}

// Mirrors crate::FallbackMode
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Fallback {
    Auto,
    Hybrid,
    Runtime,
    RuntimeManual,
    Preresolved,
}

// Mirrors crate::DeduplicationStrategy
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
        icu::markers_for_bin(bin_path)?.into_iter().collect()
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

    let (provider, fallbacker): (Box<dyn ExportableProvider>, _) = match () {
        () if markers == [HelloWorldV1Marker::INFO] => {
            // Just do naive fallback instead of pulling in compiled data or something. We only use this code path to debug
            // providers, so we don't need 100% correct fallback.
            (Box::new(icu_provider::hello_world::HelloWorldProvider), LocaleFallbacker::new_without_data())
        }
        () if markers.contains(&HelloWorldV1Marker::INFO) => {
            eyre::bail!("HelloWorldV1Marker is only allowed as the only marker")
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

            p = p.with_collation_han_database(match cli.collation_han_database {
                CollationHanDatabase::Unihan => icu_provider_source::CollationHanDatabase::Unihan,
                CollationHanDatabase::Implicit => {
                    icu_provider_source::CollationHanDatabase::Implicit
                }
            });

            if cli.trie_type == TrieType::Fast {
                p = p.with_fast_tries();
            }

            p = match (cli.cldr_root, cli.cldr_tag.as_str()) {
                (Some(path), _) => p.with_cldr(&path)?,
                #[cfg(feature = "networking")]
                (_, "latest") => p.with_cldr_for_tag(SourceDataProvider::LATEST_TESTED_CLDR_TAG),
                #[cfg(feature = "networking")]
                (_, tag) => p.with_cldr_for_tag(tag),
                #[cfg(not(feature = "networking"))]
                (None, _) => {
                    eyre::bail!(
                        "Downloading data from tags requires the `networking` Cargo feature"
                    )
                }
            };

            p = match (cli.icuexport_root, cli.icuexport_tag.as_str()) {
                (Some(path), _) => p.with_icuexport(&path)?,
                #[cfg(feature = "networking")]
                (_, "latest") => {
                    p.with_icuexport_for_tag(SourceDataProvider::LATEST_TESTED_ICUEXPORT_TAG)
                }
                #[cfg(feature = "networking")]
                (_, tag) => p.with_icuexport_for_tag(tag),
                #[cfg(not(feature = "networking"))]
                (None, _) => {
                    eyre::bail!(
                        "Downloading data from tags requires the `networking` Cargo feature"
                    )
                }
            };

            p = match (cli.segmenter_lstm_root, cli.segmenter_lstm_tag.as_str()) {
                (Some(path), _) => p.with_segmenter_lstm(&path)?,
                #[cfg(feature = "networking")]
                (_, "latest") => {
                    p.with_segmenter_lstm_for_tag(SourceDataProvider::LATEST_TESTED_SEGMENTER_LSTM_TAG)
                }
                #[cfg(feature = "networking")]
                (_, tag) => p.with_segmenter_lstm_for_tag(tag),
                #[cfg(not(feature = "networking"))]
                (None, _) => {
                    eyre::bail!(
                        "Downloading data from tags requires the `networking` Cargo feature"
                    )
                }
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

            let fallbacker = LocaleFallbacker::try_new_unstable(&p)?;
            (Box::new(p), fallbacker)
        }

        #[cfg(not(any(feature = "provider", feature = "blob_input")))]
        () => eyre::bail!("Only the `HelloWorldV1 marker is supported without Cargo features `blob_input` or `provider`"),
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
        Some(Deduplication::Maximal) => icu_provider_export::DeduplicationStrategy::Maximal,
        Some(Deduplication::RetainBaseLanguages) => {
            icu_provider_export::DeduplicationStrategy::RetainBaseLanguages
        }
        Some(Deduplication::None) => icu_provider_export::DeduplicationStrategy::None,
        None => match cli.format {
            Format::Dir | Format::Blob | Format::Blob2 => DeduplicationStrategy::None,
            Format::Mod if cli.no_internal_fallback && cli.deduplication.is_none() =>
                eyre::bail!("--no-internal-fallback requires an explicit --deduplication value. Baked exporter would default to maximal deduplication, which might not be intended"),
            // TODO(2.0): Default to RetainBaseLanguages here
            Format::Mod => DeduplicationStrategy::Maximal,
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

    match cli.format {
        #[cfg(not(feature = "fs_exporter"))]
        Format::Dir => {
            eyre::bail!("Exporting to an FsProvider requires the `fs_exporter` Cargo feature")
        }
        #[cfg(feature = "fs_exporter")]
        Format::Dir => driver.export(&provider, {
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
        })?,
        #[cfg(not(feature = "blob_exporter"))]
        Format::Blob | Format::Blob2 => {
            eyre::bail!("Exporting to a BlobProvider requires the `blob_exporter` Cargo feature")
        }
        #[cfg(feature = "blob_exporter")]
        Format::Blob | Format::Blob2 => driver.export(&provider, {
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
            if cli.format == Format::Blob {
                BlobExporter::new_with_sink(sink)
            } else {
                BlobExporter::new_v2_with_sink(sink)
            }
        })?,
        #[cfg(not(feature = "baked_exporter"))]
        Format::Mod => {
            eyre::bail!("Exporting to a baked provider requires the `baked_exporter` Cargo feature")
        }
        #[cfg(feature = "baked_exporter")]
        Format::Mod => driver.export(&provider, {
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
        })?,
    }

    Ok(())
}

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        fn all_markers() -> Vec<DataMarkerInfo> {
            vec![
                $(
                    <$marker>::INFO,
                )+
                $(
                    #[cfg(feature = "experimental")]
                    <$emarker>::INFO,
                )+
            ]
        }

        fn marker_lookup() -> &'static HashMap<&'static str, Option<DataMarkerInfo>> {
            use std::sync::OnceLock;
            static LOOKUP: OnceLock<HashMap<&'static str, Option<DataMarkerInfo>>> = OnceLock::new();
            LOOKUP.get_or_init(|| {
                [
                    ("core/helloworld@1", Some(icu_provider::hello_world::HelloWorldV1Marker::INFO)),
                    (stringify!(icu_provider::hello_world::HelloWorldV1Marker).split("::").last().unwrap().trim(), Some(icu_provider::hello_world::HelloWorldV1Marker::INFO)),
                    $(
                        ($path, Some(<$marker>::INFO)),
                        (stringify!($marker).split("::").last().unwrap().trim(), Some(<$marker>::INFO)),
                    )+
                    $(
                        #[cfg(feature = "experimental")]
                        ($epath, Some(<$emarker>::INFO)),
                        #[cfg(feature = "experimental")]
                        (stringify!($emarker).split("::").last().unwrap().trim(), Some(<$emarker>::INFO)),
                        #[cfg(not(feature = "experimental"))]
                        ($epath, None),
                        #[cfg(not(feature = "experimental"))]
                        (stringify!($emarker).split("::").last().unwrap().trim(), None),
                    )+

                ]
                .into_iter()
                .collect()
            })
        }

        #[test]
        fn test_lookup() {
            assert_eq!(marker_lookup().get("AndListV2Marker"), Some(&Some(icu::list::provider::AndListV2Marker::INFO)));
            assert_eq!(marker_lookup().get("list/and@2"), Some(&Some(icu::list::provider::AndListV2Marker::INFO)));
            assert_eq!(marker_lookup().get("foo"), None);
        }


        #[cfg(feature = "blob_input")]
        icu_provider::export::make_exportable_provider!(
            ReexportableBlobDataProvider,
            [
                icu_provider::hello_world::HelloWorldV1Marker,
                $(
                    $marker,
                )+
                $(
                    #[cfg(feature = "experimental")]
                    $emarker,
                )+
            ]
        );
    }
}
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
    fn iter_ids(&self) -> Result<std::collections::BTreeSet<DataIdentifierCow>, DataError> {
        self.0.iter_ids_for_marker(M::INFO)
    }
}
