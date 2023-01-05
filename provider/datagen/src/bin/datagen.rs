// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{App, Arg, ArgGroup};
use eyre::WrapErr;

use icu_datagen::*;
use icu_locid::LanguageIdentifier;
use icu_provider_fs::export::serializers::{bincode, json, postcard};
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use std::str::FromStr;

fn main() -> eyre::Result<()> {
    let matches = App::new("ICU4X Data Exporter")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Export CLDR JSON into the ICU4X data schema")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .help("Requests verbose output"),
        )
        .arg(
            Arg::with_name("FORMAT")
                .long("format")
                .takes_value(true)
                .required(true)
                .possible_value("dir")
                .possible_value("blob")
                .possible_value("mod")
                .help("Select the output format: a directory tree of files, a single blob, or a Rust module."),
        )
        .arg(
            Arg::with_name("OVERWRITE")
                .short("W")
                .long("overwrite")
                .help("Delete the output before writing data."),
        )
        .arg(
            Arg::with_name("SYNTAX")
                .short("s")
                .long("syntax")
                .takes_value(true)
                .possible_value("json")
                .possible_value("bincode")
                .possible_value("postcard")
                .help("--format=dir only: serde serialization format."),
        )
        .arg(
            Arg::with_name("PRETTY")
                .short("p")
                .long("pretty")
                .help("--format=mod, --format=dir only: pretty-print the Rust or JSON output files."),
        )
        .arg(
            Arg::with_name("FINGERPRINT")
                .long("fingerprint")
                .help("--format=dir only: whether to add a fingerprints file to the output.")
        )
        .arg(
            Arg::with_name("CLDR_TAG")
                .short("t")
                .long("cldr-tag")
                .value_name("TAG")
                .default_value("latest")
                .help(
                    "Download CLDR JSON data from this GitHub tag (https://github.com/unicode-org/cldr-json/tags)\n\
                    Ignored if '--cldr-root' is present.\n\
                    Note that some keys do not support versions before 41.0.0.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CLDR_ROOT")
                .long("cldr-root")
                .value_name("PATH")
                .help(
                    "Path to a local cldr-{version}-json-full.zip directory (see https://github.com/unicode-org/cldr-json/releases).\n\
                    Note that some keys do not support versions before 41.0.0.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ICUEXPORT_TAG")
                .long("icuexport-tag")
                .value_name("TAG")
                .default_value("latest")
                .help(
                    "Download Unicode Properties data from this GitHub tag (https://github.com/unicode-org/icu/tags)\n\
                    Ignored if '--icuexport-root is present.\n\
                    Note that some keys do not support versions before release-71-1."
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ICUEXPORT_ROOT")
                .long("icuexport-root")
                .value_name("PATH")
                .help(
                    "Path to a local icuexportdata_uprops_full directory (see https://github.com/unicode-org/icu/releases).\n\
                    Note that some keys do not support versions before release-71-1.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("TRIE_TYPE")
                .long("trie-type")
                .takes_value(true)
                .possible_values(&["small", "fast"])
                .help(
                    "Whether to optimize CodePointTrie data structures for size (\"small\") or speed (\"fast\").\n\
                    Using \"fast\" mode increases performance of CJK text processing and segmentation. For more\n\
                    information, see the TrieType enum."
                )
                .default_value("small"),
        )
        .arg(
            Arg::with_name("COLLATION_HAN_DATABASE")
                .long("collation-han-database")
                .takes_value(true)
                .possible_values(&["unihan", "implicit"])
                .default_value("implicit")
                .help("Which collation han database to use.")
        )
        .arg(
            Arg::with_name("INCLUDE_COLLATIONS")
                .long("include-collations")
                .multiple(true)
                .takes_value(true)
                .possible_values(&["gb2312", "big5han", "search", "searchjl", "search*"])
                .help("Which less-common collation tables to include. 'search*' includes all search tables.")
        )
        .arg(
            Arg::with_name("CLDR_LOCALE_SUBSET")
                .long("cldr-locale-subset")
                .takes_value(true)
                .possible_value("full")
                .possible_value("modern")
                .help("CLDR JSON locale subset; defaults to 'full'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("KEYS")
                .short("k")
                .long("keys")
                .multiple(true)
                .takes_value(true)
                .help(
                    "Include this resource key in the output. Accepts multiple arguments. \
                    Set to 'all' for all keys, or 'none' for no keys.",
                ),
        )
        .arg(
            Arg::with_name("KEY_FILE")
                .long("key-file")
                .takes_value(true)
                .help(
                    "Path to text file with resource keys to include, one per line. Empty lines \
                    and lines starting with '#' are ignored.",
                ),
        )
        .arg(
            Arg::with_name("KEYS_FOR_BIN")
                .long("keys-for-bin")
                .takes_value(true)
                .help(
                    "Analyzes the binary and only includes keys that are used by the binary."
                ),
        )
        .group(
            ArgGroup::with_name("KEY_MODE")
                .arg("KEYS")
                .arg("KEY_FILE")
                .arg("KEYS_FOR_BIN")
                .required(true),
        )
        .arg(
            Arg::with_name("LOCALES")
                .short("l")
                .long("locales")
                .multiple(true)
                .takes_value(true)
                .required(true)
                .help(
                    "Include this locale in the output. Accepts multiple arguments. \
                    Set to 'all' for all locales, or 'none' for no locales.",
                ),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to output directory or file. Must be empty or non-existent, unless \
                    --overwrite is present, in which case the directory is deleted first. \
                    For --format=blob, omit this option to dump to stdout. \
                    For --format={dir,mod} defaults to 'icu4x_data'.",
                )
                .takes_value(true),
        )
        .arg(Arg::with_name("INSERT_FEATURE_GATES")
            .long("insert-feature-gates")
            .help("--format=mod only: insert feature gates for individual `icu_*` crates. Requires --use-separate-crates")
        )
        .arg(Arg::with_name("USE_SEPARATE_CRATES")
            .long("use-separate-crates")
            .help("--format=mod only: use types from individual `icu_*` crates instead of the `icu` meta-crate.")
        )
        .get_matches();

    if matches.is_present("VERBOSE") {
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

    let selected_locales = {
        let locales = matches.values_of("LOCALES").unwrap();
        if locales.len() == 1 {
            match matches.value_of("LOCALES") {
                Some("all") => None,
                Some("none") => Some(vec![]),
            }
        } else {
            Some(
                locales
                    .map(|s| LanguageIdentifier::from_str(s).with_context(|| s.to_string()))
                    .collect::<Result<Vec<LanguageIdentifier>, eyre::Error>>()?,
            )
        }
    };

    let selected_keys = if let Some(paths) = matches.values_of("KEYS") {
        match paths.collect::<Vec<_>>() {
            ["none"] => vec![],
            ["all"] => icu_datagen::all_keys(),
            keys => icu_datagen::keys(&keys),
        }
    } else if let Some(key_file_path) = matches.value_of_os("KEY_FILE") {
        icu_datagen::keys_from_file(key_file_path)
            .with_context(|| key_file_path.to_string_lossy().into_owned())?
    } else if let Some(bin_path) = matches.value_of_os("KEYS_FOR_BIN") {
        icu_datagen::keys_from_bin(bin_path)
            .with_context(|| bin_path.to_string_lossy().into_owned())?
    } else {
        unreachable!("required group")
    };

    if selected_keys.is_empty() {
        log::warn!("No keys selected");
    }

    let cldr_locales = match matches.value_of("CLDR_LOCALE_SUBSET") {
        Some("modern") => icu_datagen::CldrLocaleSubset::Modern,
        _ => icu_datagen::CldrLocaleSubset::Full,
    };

    let mut source_data = SourceData::default();
    if let Some(path) = matches.value_of("CLDR_ROOT") {
        source_data = source_data.with_cldr(PathBuf::from(path), cldr_locales)?;
    } else if Some("latest") == matches.value_of("CLDR_TAG") {
        source_data = source_data.with_cldr_latest(cldr_locales)?;
    } else if let Some(tag) = matches.value_of("CLDR_TAG") {
        source_data = source_data.with_cldr_for_tag(tag, cldr_locales)?;
    }

    if let Some(path) = matches.value_of("ICUEXPORT_ROOT") {
        source_data = source_data.with_icuexport(PathBuf::from(path))?;
    } else if Some("latest") == matches.value_of("ICUEXPORT_TAG") {
        source_data = source_data.with_icuexport_latest()?;
    } else if let Some(tag) = matches.value_of("ICUEXPORT_TAG") {
        source_data = source_data.with_icuexport_for_tag(tag)?;
    }

    if matches.value_of("TRIE_TYPE") == Some("fast") {
        source_data = source_data.with_fast_tries();
    }

    source_data =
        source_data.with_collation_han_database(match matches.value_of("COLLATION_HAN_DATABASE") {
            Some("unihan") => CollationHanDatabase::Unihan,
            _ => CollationHanDatabase::Implicit,
        });

    if let Some(collations) = matches.values_of("INCLUDE_COLLATIONS") {
        source_data =
            source_data.with_collations(collations.into_iter().map(String::from).collect());
    }

    let out = match matches.value_of("FORMAT").expect("required") {
        "dir" => icu_datagen::Out::Fs {
            output_path: matches
                .value_of_os("OUTPUT")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("icu4x_data")),
            serializer: match matches.value_of("SYNTAX") {
                Some("bincode") => Box::new(bincode::Serializer::default()),
                Some("postcard") => Box::new(postcard::Serializer::default()),
                _ if matches.is_present("PRETTY") => Box::new(json::Serializer::pretty()),
                _ => Box::new(json::Serializer::default()),
            },
            overwrite: matches.is_present("OVERWRITE"),
            fingerprint: matches.is_present("FINGERPRINT"),
        },
        "blob" => icu_datagen::Out::Blob(if let Some(path) = matches.value_of_os("OUTPUT") {
            let path_buf = PathBuf::from(path);
            if !matches.is_present("OVERWRITE") && path_buf.exists() {
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
                .value_of_os("OUTPUT")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("icu4x_data"));

            if mod_directory.exists() && overwrite {
                std::fs::remove_dir_all(&mod_directory)
                    .map_err(|e| DataError::from(e).with_path_context(&mod_directory))?;
            }

            icu_datagen::Out::Module {
                mod_directory,
                pretty: matches.is_present("PRETTY"),
                insert_feature_gates: matches.is_present("INSERT_FEATURE_GATES"),
                use_separate_crates: matches.is_present("USE_SEPARATE_CRATES"),
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
