// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{App, Arg, ArgGroup};
use eyre::WrapErr;

use icu_datagen::*;
use icu_provider::hello_world::HelloWorldV1Marker;
use simple_logger::SimpleLogger;
use std::path::PathBuf;

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
                .possible_value("dir")
                .possible_value("blob")
                .possible_value("mod")
                .help("Output to a directory on the filesystem, a single blob, or a Rust module.")
                .default_value("dir"),
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
                .help("File format syntax for data files."),
        )
        .arg(
            Arg::with_name("PRETTY")
                .short("p")
                .long("pretty")
                .help("Whether to pretty-print the output files. Only affects JSON and Rust modules."),
        )
        .arg(
            Arg::with_name("FINGERPRINT")
                .long("fingerprint")
                .help("Whether to add a fingerprints file to the output. Not compatible with --format=blob")
        )
        .arg(
            Arg::with_name("CLDR_TAG")
                .short("t")
                .long("cldr-tag")
                .value_name("TAG")
                .help(
                    "Download CLDR JSON data from this GitHub tag (https://github.com/unicode-org/cldr-json/tags)\n\
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
                    Ignored if '--cldr-tag' is present.\n\
                    Note that some keys do not support versions before 41.0.0.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ICUEXPORT_TAG")
                .long("icuexport-tag")
                .value_name("TAG")
                .help(
                    "Download Unicode Properties data from this GitHub tag (https://github.com/unicode-org/icu/tags)\n\
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
                    Ignored if '--icuexport-tag is present.\n\
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
                    Also see --key-file.",
                ),
        )
        .arg(
            Arg::with_name("KEY_FILE")
                .long("key-file")
                .takes_value(true)
                .help(
                    "Path to text file with resource keys to include, one per line. Empty lines \
                    and lines starting with '#' are ignored. Also see --key.",
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
        .arg(
            Arg::with_name("HELLO_WORLD")
                .long("hello-world-key")
                .help("Whether to include the 'hello world' key."),
        )
        .arg(
            Arg::with_name("ALL_KEYS")
                .long("all-keys")
                .help("Include all keys known to ICU4X."),
        )
        .group(
            ArgGroup::with_name("KEY_MODE")
                .arg("KEYS")
                .arg("KEY_FILE")
                .arg("HELLO_WORLD")
                .arg("ALL_KEYS")
                .arg("KEYS_FOR_BIN")
                .required(true),
        )
        .arg(
            Arg::with_name("LOCALES")
                .short("l")
                .long("locales")
                .multiple(true)
                .takes_value(true)
                .help(
                    "Include this locale in the output. Accepts multiple arguments. \
                    Omit this option to include all locales.",
                ),
        )
        .arg(
            Arg::with_name("ALL_LOCALES")
                .long("all-locales")
                .help("Include all locales present in the input CLDR JSON."),
        )
        .group(
            ArgGroup::with_name("LOCALE_MODE")
                .arg("LOCALES")
                .arg("ALL_LOCALES")
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to output directory or file. Must be empty or non-existent, unless \
                    --overwrite is present, in which case the directory is deleted first. \
                    For --format blob, omit this option to dump to stdout.",
                )
                .takes_value(true),
        )
        .arg(Arg::with_name("INSERT_FEATURE_GATES")
            .long("insert-feature-gates")
            .help("Module-mode only: Insert per-key feature gates for each key's crate.")
        )
        .arg(Arg::with_name("USE_SEPARATE_CRATES")
            .long("use-separate-crates")
            .help("Module-mode only: Use types from individual crates instead of the `icu` meta-crate.")
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

    let selected_locales = matches
        .values_of("LOCALES")
        .map(|ls| {
            ls.map(|s| s.parse::<LanguageIdentifier>().with_context(|| s.to_string()))
                .collect::<Result<Vec<LanguageIdentifier>, eyre::Error>>()
        })
        .transpose()?;

    let selected_keys = if matches.is_present("ALL_KEYS") {
        icu_datagen::all_keys()
    } else if matches.is_present("HELLO_WORLD") {
        vec![HelloWorldV1Marker::KEY]
    } else if let Some(paths) = matches.values_of("KEYS") {
        icu_datagen::keys(&paths.collect::<Vec<_>>())
    } else if let Some(key_file_path) = matches.value_of_os("KEY_FILE") {
        icu_datagen::keys_from_file(key_file_path)
            .with_context(|| key_file_path.to_string_lossy().into_owned())?
    } else if let Some(bin_path) = matches.value_of_os("KEYS_FOR_BIN") {
        icu_datagen::keys_from_bin(bin_path)
            .with_context(|| bin_path.to_string_lossy().into_owned())?
    } else {
        unreachable!();
    };

    if selected_keys.is_empty() {
        log::warn!("No keys selected");
    }

    let cldr_locales = match matches.value_of("CLDR_LOCALE_SUBSET") {
        Some("modern") => icu_datagen::CldrLocaleSubset::Modern,
        _ => icu_datagen::CldrLocaleSubset::Full,
    };

    let mut source_data = SourceData::default();
    if Some("latest") == matches.value_of("CLDR_TAG") {
        source_data = source_data.with_cldr_latest(cldr_locales)?;
    } else if let Some(tag) = matches.value_of("CLDR_TAG") {
        source_data = source_data.with_cldr_for_tag(tag, cldr_locales)?;
    } else if let Some(path) = matches.value_of("CLDR_ROOT") {
        source_data = source_data.with_cldr(PathBuf::from(path), cldr_locales)?;
    }

    if Some("latest") == matches.value_of("ICUEXPORT_TAG") {
        source_data = source_data.with_icuexport_latest()?;
    } else if let Some(tag) = matches.value_of("ICUEXPORT_TAG") {
        source_data = source_data.with_icuexport_for_tag(tag)?;
    } else if let Some(path) = matches.value_of("ICUEXPORT_ROOT") {
        source_data = source_data.with_icuexport(PathBuf::from(path))?;
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

    let out = match matches
        .value_of("FORMAT")
        .expect("Option has default value")
    {
        "dir" => icu_datagen::Out::Fs {
            output_path: matches
                .value_of_os("OUTPUT")
                .map(PathBuf::from)
                .ok_or_else(|| eyre::eyre!("--out must be specified for --format=dir"))?,
            serializer: match matches.value_of("SYNTAX") {
                Some("bincode") => Box::new(syntax::Bincode::default()),
                Some("postcard") => Box::new(syntax::Postcard::default()),
                _ if matches.is_present("PRETTY") => Box::new(syntax::Json::pretty()),
                _ => Box::new(syntax::Json::default()),
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
        "mod" => icu_datagen::Out::Module {
            mod_directory: matches
                .value_of_os("OUTPUT")
                .map(PathBuf::from)
                .ok_or_else(|| eyre::eyre!("--out must be specified for --format=mod"))?,
            pretty: matches.is_present("PRETTY"),
            insert_feature_gates: matches.is_present("INSERT_FEATURE_GATES"),
            use_separate_crates: matches.is_present("USE_SEPARATE_CRATES"),
        },
        _ => unreachable!(),
    };

    icu_datagen::datagen(
        selected_locales.as_deref(),
        &selected_keys,
        &source_data,
        vec![out],
    )
    .map_err(|e| -> eyre::ErrReport {
        if icu_datagen::is_missing_cldr_error(e) {
            eyre::eyre!(
                "Either --cldr-tag or --cldr-root or --input-from-testdata must be specified"
            )
        } else if icu_datagen::is_missing_icuexport_error(e) {
            eyre::eyre!(
                "Either --icuexport-tag or --icuexport-root or --input-from-testdata must be specified"
            )
        } else {
            e.into()
        }
    })
}
