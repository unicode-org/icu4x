// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{crate_authors, crate_version, Arg, ArgGroup, Command, Parser, ValueEnum};
use eyre::WrapErr;
use icu_datagen::prelude::*;
use simple_logger::SimpleLogger;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Dir,
    Blob,
    Mod,
    DeprecatedDefault,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Syntax {
    Json,
    Bincode,
    Postcard,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TrieType {
    Small,
    Fast,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CliCollationHanDatabase {
    Unihan,
    Implicit,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CollationTable {
    Gb2312,
    Big5han,
    Search,
    Searchji,
    #[value(alias = "search*")] // for backwards compatability
    SearchAll,
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

    #[arg(long, value_enum, default_value_t = Format::DeprecatedDefault, hide_default_value = true)]
    #[arg(
        help = "Select the output format: a directory tree of files, a single blob, or a Rust module."
    )]
    format: Format,

    #[arg(short = 'W', long)]
    #[arg(help = "Delete the output before writing data.")]
    overwrite: bool,

    #[arg(short, long, value_enum)]
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
                    Ignored if '--cldr-root' is present.\n\
                    Note that some keys do not support versions before 41.0.0."
    )]
    cldr_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(
        help = "Path to a local cldr-{version}-json-full.zip directory (see https://github.com/unicode-org/cldr-json/releases).\n\
                  Note that some keys do not support versions before 41.0.0."
    )]
    cldr_root: PathBuf,

    #[arg(long, value_name = "TAG", default_value = "latest")]
    #[arg(
        help = "Download Unicode Properties data from this GitHub tag (https://github.com/unicode-org/icu/tags)\n\
                  Use 'latest' for the latest version verified to work with this version of the binary.\n\
                  Ignored if '--icuexport-root' is present.\n\
                  Note that some keys do not support versions before release-71-1."
    )]
    icuexport_tag: String,

    #[arg(long, value_name = "PATH")]
    #[arg(
        help = "Path to a local icuexportdata_uprops_full directory (see https://github.com/unicode-org/icu/releases).\n\
                  Note that some keys do not support versions before release-71-1."
    )]
    icuexport_root: PathBuf,

    #[arg(long, value_enum, default_value_t = TrieType::Small)]
    #[arg(
        help = "Whether to optimize CodePointTrie data structures for size (\"small\") or speed (\"fast\").\n\
                  Using \"fast\" mode increases performance of CJK text processing and segmentation. For more\n\
                  information, see the TrieType enum."
    )]
    trie_type: TrieType,

    #[arg(long, value_enum, default_value_t = CliCollationHanDatabase::Implicit)]
    #[arg(help = "Which collation han database to use.")]
    collation_han_database: CliCollationHanDatabase,

    #[arg(long, value_enum)]
    #[arg(
        help = "Which less-common collation tables to include. 'search-all' includes all search tables."
    )]
    include_collations: Vec<CollationTable>,

    #[arg(long, hide = true)]
    #[arg(help = "Deprecated, use --locales full or --locales modern")]
    cldr_locale_subset: bool,

    #[arg(long, short)]
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

    #[arg(long, short, required_unless_present = "all_locales")]
    #[arg(
        help = "Include this locale in the output. Accepts multiple arguments. \
                  Set to 'full' or 'modern' for the respective CLDR locale sets, or 'none' for no locales."
    )]
    locales: Vec<String>,

    #[arg(long, hide = true)]
    #[arg(help = "Deprecated: alias for --locales full")]
    all_locales: bool,

    #[arg(long, short, value_name = "PATH")]
    #[arg(
        help = "Path to output directory or file. Must be empty or non-existent, unless \
                  --overwrite is present, in which case the directory is deleted first. \
                  For --format=blob, omit this option to dump to stdout. \
                  For --format={dir,mod} defaults to 'icu4x_data'."
    )]
    output: PathBuf,

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
    unimplemented!();
    let matches = Command::new("icu4x-datagen")
        .version(crate_version!())
        .author(crate_authors!())
        .about(concat!("Learn more at: https://docs.rs/icu_datagen/", crate_version!()))
        .arg(
            Arg::new("VERBOSE")
                .short('v')
                .long("verbose")
                .help("Requests verbose output"),
        )
        .arg(
            Arg::new("FORMAT")
                .long("format")
                .num_args(1)
                .value_parser(["dir", "blob", "mod", "deprecated-default"])
                .default_value("deprecated-default")
                .hide_default_value(true)
                .help("Select the output format: a directory tree of files, a single blob, or a Rust module."),
        )
        .arg(
            Arg::new("OVERWRITE")
                .short('W')
                .long("overwrite")
                .help("Delete the output before writing data."),
        )
        .arg(
            Arg::new("SYNTAX")
                .short('s')
                .long("syntax")
                .num_args(1)
                .value_parser(["json", "bincode", "postcard"])
                .help("--format=dir only: serde serialization format."),
        )
        .arg(
            Arg::new("PRETTY")
                .short('p')
                .long("pretty")
                .help("--format=mod, --format=dir only: pretty-print the Rust or JSON output files."),
        )
        .arg(
            Arg::new("FINGERPRINT")
                .long("fingerprint")
                .help("--format=dir only: whether to add a fingerprints file to the output.")
        )
        .arg(
            Arg::new("CLDR_TAG")
                .short('t')
                .long("cldr-tag")
                .value_name("TAG")
                .default_value("latest")
                .help(
                    "Download CLDR JSON data from this GitHub tag (https://github.com/unicode-org/cldr-json/tags)\n\
                    Use 'latest' for the latest version verified to work with this version of the binary.\n\
                    Ignored if '--cldr-root' is present.\n\
                    Note that some keys do not support versions before 41.0.0.",
                )
                .num_args(1),
        )
        .arg(
            Arg::new("CLDR_ROOT")
                .long("cldr-root")
                .value_name("PATH")
                .help(
                    "Path to a local cldr-{version}-json-full.zip directory (see https://github.com/unicode-org/cldr-json/releases).\n\
                    Note that some keys do not support versions before 41.0.0.",
                )
                .num_args(1),
        )
        .arg(
            Arg::new("ICUEXPORT_TAG")
                .long("icuexport-tag")
                .value_name("TAG")
                .default_value("latest")
                .help(
                    "Download Unicode Properties data from this GitHub tag (https://github.com/unicode-org/icu/tags)\n\
                    Use 'latest' for the latest version verified to work with this version of the binary.\n\
                    Ignored if '--icuexport-root' is present.\n\
                    Note that some keys do not support versions before release-71-1."
                )
                .num_args(1),
        )
        .arg(
            Arg::new("ICUEXPORT_ROOT")
                .long("icuexport-root")
                .value_name("PATH")
                .help(
                    "Path to a local icuexportdata_uprops_full directory (see https://github.com/unicode-org/icu/releases).\n\
                    Note that some keys do not support versions before release-71-1.",
                )
                .num_args(1),
        )
        .arg(
            Arg::new("TRIE_TYPE")
                .long("trie-type")
                .num_args(1)
                .value_parser(["small", "fast"])
                .help(
                    "Whether to optimize CodePointTrie data structures for size (\"small\") or speed (\"fast\").\n\
                    Using \"fast\" mode increases performance of CJK text processing and segmentation. For more\n\
                    information, see the TrieType enum."
                )
                .default_value("small"),
        )
        .arg(
            Arg::new("COLLATION_HAN_DATABASE")
                .long("collation-han-database")
                .num_args(1)
                .value_parser(["unihan", "implicit"])
                .default_value("implicit")
                .help("Which collation han database to use.")
        )
        .arg(
            Arg::new("INCLUDE_COLLATIONS")
                .long("include-collations")
                .action(clap::ArgAction::Append)
                .num_args(1)
                .value_parser(["gb2312", "big5han", "search", "searchjl", "search*"])
                .help("Which less-common collation tables to include. 'search-all' includes all search tables.")
        )
        .arg(
            Arg::new("CLDR_LOCALE_SUBSET")
                .long("cldr-locale-subset")
                .hide(true)
                .help("Deprecated, use --locales full or --locales modern")
                .num_args(1),
        )
        .arg(
            Arg::new("KEYS")
                .short('k')
                .long("keys")
                .action(clap::ArgAction::Append)
                .num_args(1)
                .help(
                    "Include these resource keys in the output. Accepts multiple arguments.\n\
                    Set to 'all' for all keys, 'experimental-all' to include experimental keys,\n\
                    or 'none' for no keys.",
                ),
        )
        .arg(
            Arg::new("KEY_FILE")
                .long("key-file")
                .num_args(1)
                .help(
                    "Path to text file with resource keys to include, one per line. Empty lines \
                    and lines starting with '#' are ignored.",
                ),
        )
        .arg(
            Arg::new("KEYS_FOR_BIN")
                .long("keys-for-bin")
                .num_args(1)
                .help(
                    "Analyzes the binary and only includes keys that are used by the binary."
                ),
        )
        .arg(
            Arg::new("ALL_KEYS")
                .long("all-keys")
                .hide(true)
                .help("Deprecated: alias for --keys all"),
        )
        .group(
            ArgGroup::new("KEY_MODE")
                .arg("KEYS")
                .arg("KEY_FILE")
                .arg("KEYS_FOR_BIN")
                .arg("ALL_KEYS")
                .required(true),
        )
        .arg(
            Arg::new("LOCALES")
                .short('l')
                .long("locales")
                .action(clap::ArgAction::Append)
                .num_args(1)
                .required_unless_present("ALL_LOCALES")
                .help(
                    "Include this locale in the output. Accepts multiple arguments. \
                    Set to 'full' or 'modern' for the respective CLDR locale sets, or 'none' for no locales.",
                ),
        )
        .arg(
            Arg::new("ALL_LOCALES")
                .long("all-locales")
                .hide(true)
                .help("Deprecated: alias for --locales full"),
        )
        .arg(
            Arg::new("OUTPUT")
                .short('o')
                .long("out")
                .help(
                    "Path to output directory or file. Must be empty or non-existent, unless \
                    --overwrite is present, in which case the directory is deleted first. \
                    For --format=blob, omit this option to dump to stdout. \
                    For --format={dir,mod} defaults to 'icu4x_data'.",
                )
                .num_args(1),
        )
        .arg(Arg::new("INSERT_FEATURE_GATES")
            .long("insert-feature-gates")
            .help("--format=mod only: insert feature gates for individual `icu_*` crates. Requires --use-separate-crates")
        )
        .arg(Arg::new("USE_SEPARATE_CRATES")
            .long("use-separate-crates")
            .help("--format=mod only: use types from individual `icu_*` crates instead of the `icu` meta-crate.")
        )
        .get_matches();

    if matches.contains_id("VERBOSE") {
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

    let selected_keys = if matches.contains_id("ALL_KEYS") {
        icu_datagen::all_keys()
    } else if let Some(paths) = matches.get_many("KEYS") {
        match paths.copied().collect::<Vec<_>>().as_slice() {
            ["none"] => vec![],
            ["all"] => icu_datagen::all_keys(),
            ["experimental-all"] => icu_datagen::all_keys_with_experimental(),
            keys => icu_datagen::keys(keys),
        }
    } else if let Some(key_file_path) = matches.get_one::<&Path>("KEY_FILE") {
        icu_datagen::keys_from_file(*key_file_path)
            .with_context(|| key_file_path.to_string_lossy().into_owned())?
    } else if let Some(bin_path) = matches.get_one::<&Path>("KEYS_FOR_BIN") {
        icu_datagen::keys_from_bin(*bin_path)
            .with_context(|| bin_path.to_string_lossy().into_owned())?
    } else {
        unreachable!("required group")
    };

    if selected_keys.is_empty() {
        log::warn!("No keys selected");
    }

    let mut source_data = SourceData::default();
    if let Some(path) = matches.get_one::<&Path>("CLDR_ROOT") {
        source_data = source_data.with_cldr(PathBuf::from(path), Default::default())?;
    } else {
        source_data = source_data.with_cldr_for_tag(
            if Some(&"latest") == matches.get_one("CLDR_TAG") {
                SourceData::LATEST_TESTED_CLDR_TAG
            } else {
                *matches.get_one("CLDR_TAG").unwrap()
            },
            Default::default(),
        )?
    }

    if let Some(path) = matches.get_one::<&Path>("ICUEXPORT_ROOT") {
        source_data = source_data.with_icuexport(PathBuf::from(*path))?;
    } else {
        source_data = source_data.with_icuexport_for_tag(
            if Some(&"latest") == matches.get_one("ICUEXPORT_TAG") {
                SourceData::LATEST_TESTED_ICUEXPORT_TAG
            } else {
                *matches.get_one("ICUEXPORT_TAG").unwrap()
            },
        )?;
    }

    if matches.get_one("TRIE_TYPE") == Some(&"fast") {
        source_data = source_data.with_fast_tries();
    }

    source_data =
        source_data.with_collation_han_database(match matches.get_one("COLLATION_HAN_DATABASE") {
            Some(&"unihan") => CollationHanDatabase::Unihan,
            _ => CollationHanDatabase::Implicit,
        });

    if let Some(collations) = matches.get_many::<String>("INCLUDE_COLLATIONS") {
        source_data =
            source_data.with_collations(collations.into_iter().map(String::from).collect());
    }

    let raw_locales = matches
        .get_many("LOCALES")
        .unwrap()
        .copied()
        .collect::<Vec<_>>();

    let selected_locales = if raw_locales == ["none"] || selected_keys.is_empty() {
        Some(vec![])
    } else if raw_locales == ["full"] || matches.contains_id("ALL_LOCALES") {
        None
    } else if let Some(locale_subsets) = raw_locales
        .iter()
        .map(|&s| match s {
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
                .into_iter()
                .map(|s| {
                    s.parse::<LanguageIdentifier>()
                        .with_context(|| s.to_string())
                })
                .collect::<Result<Vec<LanguageIdentifier>, eyre::Error>>()?,
        )
    };

    let out = match *matches.get_one("FORMAT").expect("required") {
        v @ ("dir" | "deprecated-default") => {
            if v == "deprecated-default" {
                log::warn!("Defaulting to --format=dir. This will become a required parameter in the future.");
            }
            icu_datagen::Out::Fs {
                output_path: matches
                    .get_one::<&Path>("OUTPUT")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("icu4x_data")),
                serializer: match matches.get_one("SYNTAX") {
                    Some(&"bincode") => Box::<syntax::Bincode>::default(),
                    Some(&"postcard") => Box::<syntax::Postcard>::default(),
                    _ if matches.contains_id("PRETTY") => Box::new(syntax::Json::pretty()),
                    _ => Box::<syntax::Json>::default(),
                },
                overwrite: matches.contains_id("OVERWRITE"),
                fingerprint: matches.contains_id("FINGERPRINT"),
            }
        }
        "blob" => icu_datagen::Out::Blob(if let Some(path) = matches.get_one::<&Path>("OUTPUT") {
            let path_buf = PathBuf::from(path);
            if !matches.contains_id("OVERWRITE") && path_buf.exists() {
                eyre::bail!("Output path is present: {:?}", path_buf);
            }
            Box::new(
                std::fs::File::create(&path_buf)
                    .with_context(|| path_buf.to_string_lossy().to_string())?,
            )
        } else {
            Box::new(std::io::stdout())
        }),
        "mod" => {
            let mod_directory = matches
                .get_one::<&Path>("OUTPUT")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("icu4x_data"));

            let mut options = BakedOptions::default();
            options.pretty = matches.contains_id("PRETTY");
            options.insert_feature_gates = matches.contains_id("INSERT_FEATURE_GATES");
            options.use_separate_crates = matches.contains_id("USE_SEPARATE_CRATES");
            options.overwrite = matches.contains_id("OVERWRITE");

            icu_datagen::Out::Baked {
                mod_directory,
                options,
            }
        }
        _ => unreachable!(),
    };

    icu_datagen::datagen(
        selected_locales.as_deref(),
        &selected_keys,
        &source_data,
        vec![out],
    )
    .map_err(eyre::ErrReport::from)
}
