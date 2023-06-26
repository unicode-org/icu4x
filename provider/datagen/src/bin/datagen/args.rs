// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::config;
use clap::ValueEnum;
use clap::{ArgGroup, Parser};
use eyre::WrapErr;
use icu_datagen::prelude::*;
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format {
    Dir,
    Blob,
    Mod,
    DeprecatedDefault,
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

// Mirrors crate::options::FallbackMode
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Fallback {
    Legacy,
    Runtime,
    Expand,
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

#[derive(Parser)]
#[command(name = "icu4x-datagen")]
#[command(author = "The ICU4X Project Developers", version = option_env!("CARGO_PKG_VERSION"))]
#[command(about = format!("Learn more at: https://docs.rs/icu_datagen/{}", option_env!("CARGO_PKG_VERSION").unwrap_or("")), long_about = None)]
#[command(group(
            ArgGroup::new("key_mode")
                // .required(true)
                .args(["keys", "key_file", "keys_for_bin", "all_keys"]),
        ))]
pub struct Cli {
    #[arg(short, long)]
    #[arg(help = "Requests verbose output")]
    pub verbose: bool,

    #[arg(long, value_enum, default_value_t = Format::DeprecatedDefault, hide_default_value = true)]
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

    #[arg(long)]
    #[arg(help = "--format=dir only: whether to add a fingerprints file to the output.")]
    fingerprint: bool,

    #[arg(short = 't', long, value_name = "TAG", default_value = "latest")]
    #[arg(
        help = "Download CLDR JSON data from this GitHub tag (https://github.com/unicode-org/cldr-json/tags)\n\
                    Use 'latest' for the latest version verified to work with this version of the binary.\n\
                    Ignored if '--cldr-root' is present. Requires binary to be built with `networking` Cargo feature (enabled by default).\n\
                    Note that some keys do not support versions before 41.0.0."
    )]
    #[cfg_attr(not(feature = "networking"), arg(hide = true))]
    cldr_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(
        help = "Path to a local cldr-{version}-json-full.zip directory (see https://github.com/unicode-org/cldr-json/releases).\n\
                  Note that some keys do not support versions before 41.0.0."
    )]
    cldr_root: Option<PathBuf>,

    #[arg(long, value_name = "TAG", default_value = "latest")]
    #[arg(
        help = "Download ICU data from this GitHub tag (https://github.com/unicode-org/icu/tags)\n\
                  Use 'latest' for the latest version verified to work with this version of the binary.\n\
                  Ignored if '--icuexport-root' is present. Requires binary to be built with `networking` Cargo feature (enabled by default).\n\
                  Note that some keys do not support versions before release-71-1."
    )]
    #[cfg_attr(not(feature = "networking"), arg(hide = true))]
    icuexport_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(
        help = "Path to a local icuexport directory (see https://github.com/unicode-org/icu/releases).\n\
                  Note that some keys do not support versions before release-71-1."
    )]
    icuexport_root: Option<PathBuf>,

    #[arg(long, value_name = "TAG", default_value = "latest")]
    #[arg(
        help = "Download segmentation LSTM models from this GitHub tag (https://github.com/unicode-org/lstm_word_segmentation/tags)\n\
                  Use 'latest' for the latest version verified to work with this version of the binary.\n\
                  Ignored if '--segmenter-lstm-root' is present. Requires binary to be built with `networking` Cargo feature (enabled by default)."
    )]
    #[cfg_attr(not(feature = "networking"), arg(hide = true))]
    segmenter_lstm_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(
        help = "Path to a local segmentation LSTM directory (see https://github.com/unicode-org/lstm_word_segmentation/releases)."
    )]
    segmenter_lstm_root: Option<PathBuf>,

    #[arg(long, value_enum, default_value_t = TrieType::Small)]
    #[arg(
        help = "Whether to optimize CodePointTrie data structures for size (\"small\") or speed (\"fast\").\n\
                  Using \"fast\" mode increases performance of CJK text processing and segmentation. For more\n\
                  information, see the TrieType enum."
    )]
    trie_type: TrieType,

    #[arg(long, value_enum, default_value_t = CollationHanDatabase::Implicit)]
    #[arg(help = "Which collation han database to use.")]
    collation_han_database: CollationHanDatabase,

    #[arg(long, value_enum, num_args = 1..)]
    #[arg(
        help = "Which less-common collation tables to include. 'search-all' includes all search tables."
    )]
    include_collations: Vec<CollationTable>,

    #[arg(long, hide = true)]
    #[arg(help = "Deprecated, use --locales full or --locales modern")]
    cldr_locale_subset: bool,

    #[arg(long, short, num_args = 1..)]
    #[arg(
        help = "Include these resource keys in the output. Accepts multiple arguments.\n\
                  Set to 'all' for all keys, 'experimental-all' to include experimental keys,\n\
                  or 'none' for no keys."
    )]
    keys: Vec<String>,

    #[arg(long, value_name = "KEY_FILE")]
    #[arg(
        help = "Path to text file with resource keys to include, one per line. Empty lines \
                  and lines starting with '#' are ignored."
    )]
    key_file: Option<PathBuf>,

    #[arg(long, value_name = "BINARY")]
    #[arg(help = "Analyzes the binary and only includes keys that are used by the binary.")]
    keys_for_bin: Option<PathBuf>,

    #[arg(long, hide = true)]
    #[arg(help = "Deprecated: alias for --keys all")]
    all_keys: bool,

    #[arg(long, short, num_args = 0..)]
    #[arg(
        help = "Include this locale in the output. Accepts multiple arguments. \
                  Set to 'full' or 'modern' for the respective CLDR locale sets, or 'none' for no locales."
    )]
    locales: Vec<String>,

    #[arg(long, hide = true)]
    #[arg(help = "Deprecated: alias for --locales full")]
    all_locales: bool,

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
        help = "--format=mod only: insert feature gates for individual `icu_*` crates. Requires --use-separate-crates"
    )]
    insert_feature_gates: bool,

    #[arg(long)]
    #[arg(
        help = "--format=mod only: use types from individual `icu_*` crates instead of the `icu` meta-crate."
    )]
    use_separate_crates: bool,

    #[arg(long)]
    #[arg(help = "Load a TOML config")]
    config: Option<PathBuf>,

    #[arg(short, long, value_enum, default_value_t = Fallback::Legacy)]
    fallback: Fallback,
}

impl Cli {
    pub fn as_config(&self) -> eyre::Result<config::Config> {
        Ok(if let Some(ref path) = self.config {
            serde_json::from_str(&std::fs::read_to_string(path)?)?
        } else {
            config::Config {
                keys: self.make_keys()?,
                locales: self.make_locales()?,
                cldr: self.make_path(&self.cldr_root, &self.cldr_tag, "cldr-root")?,
                icu_export: self.make_path(
                    &self.icuexport_root,
                    &self.icuexport_tag,
                    "icuexport-root",
                )?,
                segmenter_lstm: self.make_path(
                    &self.segmenter_lstm_root,
                    &self.segmenter_lstm_tag,
                    "segmenter-lstm",
                )?,
                trie_type: match self.trie_type {
                    TrieType::Fast => config::TrieType::Fast,
                    TrieType::Small => config::TrieType::Small,
                },
                collation_han_database: match self.collation_han_database {
                    CollationHanDatabase::Unihan => config::CollationHanDatabase::Unihan,
                    CollationHanDatabase::Implicit => config::CollationHanDatabase::Implicit,
                },
                collations: self
                    .include_collations
                    .iter()
                    .map(|c| c.to_datagen_value().to_owned())
                    .collect(),
                export: self.make_exporter()?,
                fallback: match self.fallback {
                    Fallback::Legacy => config::FallbackMode::Legacy,
                    Fallback::Runtime => config::FallbackMode::Runtime,
                    Fallback::Expand => config::FallbackMode::Expand,
                },
                overwrite: self.overwrite,
            }
        })
    }

    fn make_keys(&self) -> eyre::Result<config::KeyInclude> {
        Ok(if self.all_keys {
            config::KeyInclude::All
        } else if !self.keys.is_empty() {
            match self.keys.as_slice() {
                [x] if x == "none" => config::KeyInclude::None,
                [x] if x == "all" => config::KeyInclude::All,
                [x] if x == "experimental-all" => config::KeyInclude::AllWithExperimental,
                keys => config::KeyInclude::Explicit(
                    keys.iter()
                        .map(|k| icu_datagen::key(k).ok_or(eyre::eyre!(k.to_string())))
                        .collect::<Result<_, _>>()?,
                ),
            }
        } else if let Some(key_file_path) = &self.key_file {
            log::warn!("The --key-file argument is deprecated. Use --options with a JSON file.");
            #[allow(deprecated)]
            config::KeyInclude::Explicit(
                icu_datagen::keys_from_file(key_file_path)
                    .with_context(|| key_file_path.to_string_lossy().into_owned())?
                    .into_iter()
                    .collect(),
            )
        } else if let Some(bin_path) = &self.keys_for_bin {
            config::KeyInclude::ForBinary(bin_path.clone())
        } else {
            unreachable!("Argument group");
        })
    }

    fn make_locales(&self) -> eyre::Result<config::LocaleInclude> {
        Ok(if self.locales.as_slice() == ["none"] {
            config::LocaleInclude::None
        } else if self.locales.as_slice() == ["full"] || self.all_locales {
            config::LocaleInclude::All
        } else if let Some(locale_subsets) = self
            .locales
            .iter()
            .map(|s| match &**s {
                "basic" => Some(config::CoverageLevel::Basic),
                "moderate" => Some(config::CoverageLevel::Moderate),
                "modern" => Some(config::CoverageLevel::Modern),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()
        {
            config::LocaleInclude::CldrSet(locale_subsets.into_iter().collect())
        } else {
            config::LocaleInclude::Explicit(
                self.locales
                    .iter()
                    .map(|s| {
                        s.parse::<LanguageIdentifier>()
                            .with_context(|| s.to_string())
                    })
                    .collect::<Result<_, eyre::Error>>()?,
            )
        })
    }

    fn make_path(
        &self,
        root: &Option<PathBuf>,
        tag: &str,
        root_arg: &'static str,
    ) -> eyre::Result<config::PathOrTag> {
        Ok(match (root, tag) {
            (Some(path), _) => config::PathOrTag::Path(path.clone()),
            #[cfg(feature = "networking")]
            (_, "latest") => config::PathOrTag::Latest,
            #[cfg(feature = "networking")]
            (_, tag) => config::PathOrTag::Tag(String::from(tag)),
            #[cfg(not(feature = "networking"))]
            _ => config::PathOrTag::None,
        })
    }

    fn make_exporter(&self) -> eyre::Result<config::Export> {
        match self.format {
            v @ (Format::Dir | Format::DeprecatedDefault) => {
                if v == Format::DeprecatedDefault {
                    log::warn!("Defaulting to --format=dir. This will become a required parameter in the future.");
                }
                #[cfg(not(feature = "provider_fs"))]
                eyre::bail!("FsDataProvider export requires the provider_fs Cargo feature.");
                #[cfg(feature = "provider_fs")]
                Ok(config::Export::Fs {
                    output_path: if let Some(root) = self.output.as_ref() {
                        root.clone()
                    } else {
                        PathBuf::from("icu4x_data")
                    },
                    syntax: match self.syntax {
                        Syntax::Bincode => config::FsSyntax::Bincode,
                        Syntax::Postcard => config::FsSyntax::Postcard,
                        Syntax::Json if self.pretty => config::FsSyntax::JsonPretty,
                        Syntax::Json => config::FsSyntax::Json,
                    },
                    fingerprint: self.fingerprint,
                })
            }
            Format::Blob => {
                #[cfg(not(feature = "provider_blob"))]
                eyre::bail!("BlobDataProvider export requires the provider_blob Cargo feature.");
                #[cfg(feature = "provider_blob")]
                Ok(config::Export::Blob(if let Some(path) = &self.output {
                    path.clone()
                } else {
                    PathBuf::from("/stdout")
                }))
            }
            Format::Mod => {
                #[cfg(not(feature = "provider_baked"))]
                eyre::bail!("Baked data export requires the provider_baked Cargo feature.");
                #[cfg(feature = "provider_baked")]
                Ok(config::Export::Baked {
                    output_path: if let Some(mod_directory) = self.output.as_ref() {
                        mod_directory.clone()
                    } else {
                        PathBuf::from("icu4x_data")
                    },
                    pretty: self.pretty,
                    insert_feature_gates: self.insert_feature_gates,
                    use_separate_crates: self.use_separate_crates,
                })
            }
        }
    }
}
