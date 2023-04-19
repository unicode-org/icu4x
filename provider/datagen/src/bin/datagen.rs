// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{crate_version, ArgGroup, Parser};
use eyre::WrapErr;
use icu_datagen::prelude::*;
use simple_logger::SimpleLogger;
use std::path::PathBuf;

mod cli {
    use clap::ValueEnum;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
    pub(crate) enum Format {
        #[cfg_attr(feature = "fs", value(hide = true))]
        Dir,
        Blob,
        Mod,
        DeprecatedDefault,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
    pub(crate) enum Syntax {
        Json,
        Bincode,
        Postcard,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
    pub(crate) enum TrieType {
        Small,
        Fast,
    }
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
    pub(crate) enum CollationHanDatabase {
        Unihan,
        Implicit,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
    pub(crate) enum CollationTable {
        Gb2312,
        Big5han,
        Search,
        Searchji,
        #[value(alias = "search*")] // for backwards compatability
        SearchAll,
    }

    impl CollationTable {
        pub(crate) fn to_datagen_value(self) -> &'static str {
            match self {
                Self::Gb2312 => "gb2312",
                Self::Big5han => "big5han",
                Self::Search => "search",
                Self::Searchji => "searchji",
                Self::SearchAll => "search*",
            }
        }
    }
}
#[derive(Parser)]
#[command(name = "icu4x-datagen", author, version)]
#[command(about = concat!("Learn more at: https://docs.rs/icu_datagen/", crate_version!()), long_about = None)]
#[command(group(
            ArgGroup::new("key_mode")
                .required(true)
                .args(["keys", "key_file", "keys_for_bin", "all_keys"]),
        ))]
struct Cli {
    #[arg(short, long)]
    #[arg(help = "Requests verbose output")]
    verbose: bool,

    #[arg(long, value_enum, default_value_t = cli::Format::DeprecatedDefault, hide_default_value = true)]
    #[arg(
        help = "Select the output format: a directory tree of files, a single blob, or a Rust module."
    )]
    format: cli::Format,

    #[arg(short = 'W', long)]
    #[arg(help = "Delete the output before writing data.")]
    overwrite: bool,

    #[arg(short, long, value_enum, default_value_t = cli::Syntax::Json)]
    #[arg(help = "--format=dir only: serde serialization format.")]
    #[cfg_attr(feature = "fs", arg(hide = true))]
    syntax: cli::Syntax,

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
                    Ignored if '--cldr-root' is present. Requires binary to be built with `networking` feature (enabled by default).\n\
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
        help = "Download Unicode Properties data from this GitHub tag (https://github.com/unicode-org/icu/tags)\n\
                  Use 'latest' for the latest version verified to work with this version of the binary.\n\
                  Ignored if '--icuexport-root' is present. Requires binary to be built with `networking` feature (enabled by default).\n\
                  Note that some keys do not support versions before release-71-1."
    )]
    #[cfg_attr(not(feature = "networking"), arg(hide = true))]
    icuexport_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(
        help = "Path to a local icuexportdata_uprops_full directory (see https://github.com/unicode-org/icu/releases).\n\
                  Note that some keys do not support versions before release-71-1."
    )]
    icuexport_root: Option<PathBuf>,

    #[arg(long, value_enum, default_value_t = cli::TrieType::Small)]
    #[arg(
        help = "Whether to optimize CodePointTrie data structures for size (\"small\") or speed (\"fast\").\n\
                  Using \"fast\" mode increases performance of CJK text processing and segmentation. For more\n\
                  information, see the TrieType enum."
    )]
    trie_type: cli::TrieType,

    #[arg(long, value_enum, default_value_t = cli::CollationHanDatabase::Implicit)]
    #[arg(help = "Which collation han database to use.")]
    collation_han_database: cli::CollationHanDatabase,

    #[arg(long, value_enum, num_args = 1..)]
    #[arg(
        help = "Which less-common collation tables to include. 'search-all' includes all search tables."
    )]
    include_collations: Vec<cli::CollationTable>,

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

    #[arg(long, short, required_unless_present = "all_locales", num_args = 0..)]
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
}

fn main() -> eyre::Result<()> {
    let matches = Cli::parse();

    if matches.verbose {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap()
    } else {
        SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap()
    }

    let selected_keys = if matches.all_keys {
        icu_datagen::all_keys()
    } else if !matches.keys.is_empty() {
        match matches.keys.as_slice() {
            [x] if x == "none" => vec![],
            [x] if x == "all" => icu_datagen::all_keys(),
            [x] if x == "experimental-all" => icu_datagen::all_keys_with_experimental(),
            keys => icu_datagen::keys(keys),
        }
    } else if let Some(ref key_file_path) = matches.key_file {
        icu_datagen::keys_from_file(key_file_path)
            .with_context(|| key_file_path.to_string_lossy().into_owned())?
    } else if let Some(ref bin_path) = matches.keys_for_bin {
        icu_datagen::keys_from_bin(bin_path)
            .with_context(|| bin_path.to_string_lossy().into_owned())?
    } else {
        unreachable!("required group")
    };

    if selected_keys.is_empty() {
        log::warn!("No keys selected");
    }

    let mut source_data = SourceData::default();
    if let Some(path) = matches.cldr_root {
        source_data = source_data.with_cldr(path, Default::default())?;
    } else {
        #[cfg(feature = "networking")]
        {
            let tag = match &*matches.cldr_tag {
                "latest" => SourceData::LATEST_TESTED_CLDR_TAG,
                other => other,
            };
            source_data = source_data.with_cldr_for_tag(tag, Default::default())?
        }
        #[cfg(not(feature = "networking"))]
        {
            eyre::bail!("--cldr-root flag is mandatory unless datagen is built with the `\"networking\"` feature");
        }
    }

    if let Some(path) = matches.icuexport_root {
        source_data = source_data.with_icuexport(path)?;
    } else {
        #[cfg(feature = "networking")]
        {
            let tag = match &*matches.icuexport_tag {
                "latest" => SourceData::LATEST_TESTED_ICUEXPORT_TAG,
                other => other,
            };
            source_data = source_data.with_icuexport_for_tag(tag)?;
        }
        #[cfg(not(feature = "networking"))]
        {
            eyre::bail!("--icuexport-root flag is mandatory unless datagen is built with the `\"networking\"` feature");
        }
    }

    if matches.trie_type == cli::TrieType::Fast {
        source_data = source_data.with_fast_tries();
    }

    source_data = source_data.with_collation_han_database(match matches.collation_han_database {
        cli::CollationHanDatabase::Unihan => CollationHanDatabase::Unihan,
        cli::CollationHanDatabase::Implicit => CollationHanDatabase::Implicit,
    });

    if !matches.include_collations.is_empty() {
        source_data = source_data.with_collations(
            matches
                .include_collations
                .iter()
                .map(|c| c.to_datagen_value().to_owned())
                .collect(),
        );
    }

    let raw_locales = &matches.locales;

    let selected_locales = if raw_locales == &["none"] || selected_keys.is_empty() {
        Some(vec![])
    } else if raw_locales == &["full"] || matches.all_locales {
        None
    } else if let Some(locale_subsets) = raw_locales
        .iter()
        .map(|s| match &**s {
            "basic" => Some(CoverageLevel::Basic),
            "moderate" => Some(CoverageLevel::Moderate),
            "modern" => Some(CoverageLevel::Modern),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
    {
        Some(source_data.locales(&locale_subsets)?)
    } else {
        Some(
            raw_locales
                .iter()
                .map(|s| {
                    s.parse::<LanguageIdentifier>()
                        .with_context(|| s.to_string())
                })
                .collect::<Result<Vec<LanguageIdentifier>, eyre::Error>>()?,
        )
    };

    let out = match matches.format {
        v @ (cli::Format::Dir | cli::Format::DeprecatedDefault) => {
            if v == cli::Format::DeprecatedDefault {
                log::warn!("Defaulting to --format=dir. This will become a required parameter in the future.");
            }
            #[cfg(not(feature = "fs"))]
            eyre::bail!("--format=dir only available with the `fs` Cargo feature");
            #[cfg(feature = "fs")]
            icu_datagen::Out::Fs {
                output_path: matches
                    .output
                    .unwrap_or_else(|| PathBuf::from("icu4x_data")),
                serializer: match matches.syntax {
                    cli::Syntax::Bincode => Box::<syntax::Bincode>::default(),
                    cli::Syntax::Postcard => Box::<syntax::Postcard>::default(),
                    cli::Syntax::Json if matches.pretty => Box::new(syntax::Json::pretty()),
                    cli::Syntax::Json => Box::<syntax::Json>::default(),
                },
                overwrite: matches.overwrite,
                fingerprint: matches.fingerprint,
            }
        }
        cli::Format::Blob => icu_datagen::Out::Blob(if let Some(path) = matches.output {
            if !matches.overwrite && path.exists() {
                eyre::bail!("Output path is present: {:?}", path);
            }
            Box::new(
                std::fs::File::create(&path).with_context(|| path.to_string_lossy().to_string())?,
            )
        } else {
            Box::new(std::io::stdout())
        }),
        cli::Format::Mod => {
            let mod_directory = matches
                .output
                .unwrap_or_else(|| PathBuf::from("icu4x_data"));

            let mut options = BakedOptions::default();
            options.pretty = matches.pretty;
            options.insert_feature_gates = matches.insert_feature_gates;
            options.use_separate_crates = matches.use_separate_crates;
            options.overwrite = matches.overwrite;

            icu_datagen::Out::Baked {
                mod_directory,
                options,
            }
        }
    };

    icu_datagen::datagen(
        selected_locales.as_deref(),
        &selected_keys,
        &source_data,
        vec![out],
    )
    .map_err(eyre::ErrReport::from)
}
