// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::manifest::LocalesOption;
use anyhow::Context;
use clap::{App, Arg, ArgGroup};
use icu_locid::LanguageIdentifier;
use icu_provider::export::DataExporter;
use icu_provider_cldr::download::CldrAllInOneDownloader;
use icu_provider_cldr::get_all_cldr_keys;
use icu_provider_cldr::CldrJsonDataProvider;
use icu_provider_cldr::CldrPaths;
use icu_provider_cldr::CldrPathsAllInOne;
use icu_provider_fs::export::fs_exporter;
use icu_provider_fs::export::serializers;
use icu_provider_fs::export::FilesystemExporter;
use icu_provider_fs::manifest;
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let matches = App::new("ICU4X Data Exporter")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Export CLDR JSON into the ICU4X data schema")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity (-v or -vv)"),
        )
        .arg(
            Arg::with_name("DRY_RUN")
                .short("n")
                .long("dry-run")
                .help("Do not touch the filesystem (consider using with -v)."),
        )
        .arg(
            Arg::with_name("ALIASING")
                .long("aliasing")
                .takes_value(true)
                .possible_value("none")
                .possible_value("symlink")
                .help("Sets the aliasing mode of the output on the filesystem."),
        )
        .arg(
            Arg::with_name("OVERWRITE")
                .short("W")
                .long("overwrite")
                .help("Delete the output directory before writing data."),
        )
        .arg(
            Arg::with_name("SYNTAX")
                .short("s")
                .long("syntax")
                .takes_value(true)
                .possible_value("json")
                .possible_value("bincode")
                .help("File format syntax for data files (defaults to JSON).")
                .default_value("json"),
        )
        .arg(
            Arg::with_name("PRETTY")
                .short("p")
                .long("pretty")
                .help("Whether to pretty-print the output JSON files. Ignored for Bincode."),
        )
        .arg(
            Arg::with_name("CLDR_TAG")
                .short("t")
                .long("cldr-tag")
                .value_name("TAG")
                .help(
                    "Download CLDR JSON data from this GitHub tag: \n\
                    https://github.com/unicode-org/cldr-json/tags",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CLDR_ROOT")
                .long("cldr-root")
                .value_name("PATH")
                .help(
                    "Path to CLDR JSON distribution. Ignored if '--cldr-tag' is present. \n\
                    https://github.com/unicode-org/cldr-json/tree/master/cldr-json",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CLDR_TESTDATA")
                .long("cldr-testdata")
                .help("Load CLDR JSON data from the icu_testdata project."),
        )
        .group(
            ArgGroup::with_name("CLDR_SOURCE")
                .arg("CLDR_TAG")
                .arg("CLDR_ROOT")
                .arg("CLDR_TESTDATA")
                .required(true),
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
            Arg::with_name("ALL_KEYS")
                .long("all-keys")
                .help("Include all keys known to ICU4X."),
        )
        .group(
            ArgGroup::with_name("KEY_MODE")
                .arg("KEYS")
                .arg("KEY_FILE")
                .arg("ALL_KEYS")
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
            Arg::with_name("TEST_LOCALES")
                .long("test-locales")
                .help("Include only test locales, as discussed in icu_testdata."),
        )
        .arg(
            Arg::with_name("ALL_LOCALES")
                .long("all-locales")
                .help("Include all locales present in the input CLDR JSON."),
        )
        .group(
            ArgGroup::with_name("LOCALE_MODE")
                .arg("LOCALES")
                .arg("TEST_LOCALES")
                .arg("ALL_LOCALES")
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to output data directory. Must be empty or non-existent, unless \
                    --overwrite is present, in which case the directory is deleted first.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT_TESTDATA")
                .long("out-testdata")
                .help("Write output to the ICU4X testdata tree."),
        )
        .group(
            ArgGroup::with_name("OUTPUT_MODE")
                .arg("OUTPUT")
                .arg("OUTPUT_TESTDATA")
                .required(true),
        )
        .get_matches();

    match matches.occurrences_of("VERBOSE") {
        0 => SimpleLogger::from_env().init().unwrap(),
        1 => SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap(),
        2 => SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap(),
        _ => anyhow::bail!("Only -v and -vv are supported"),
    }

    if !matches.is_present("ALL_KEYS") {
        anyhow::bail!("Lists of keys are not yet supported (see #192)",);
    }

    if matches.is_present("DRY_RUN") {
        anyhow::bail!("Dry-run is not yet supported");
    }

    // TODO: Build up this list from --keys and --key-file

    let syntax = matches
        .value_of("SYNTAX")
        .expect("Option has default value");

    let output_path = if matches.is_present("OUTPUT_TESTDATA") {
        icu_testdata::paths::data_root().join(syntax)
    } else {
        PathBuf::from(
            matches
                .value_of_os("OUTPUT")
                .expect("--out is a required option"),
        )
    };

    let locale_subset = matches.value_of("CLDR_LOCALE_SUBSET").unwrap_or("full");
    let cldr_paths: Box<dyn CldrPaths> = if let Some(tag) = matches.value_of("CLDR_TAG") {
        Box::new(CldrAllInOneDownloader::try_new_from_github(tag, locale_subset)?.download()?)
    } else if let Some(path) = matches.value_of("CLDR_ROOT") {
        Box::new(CldrPathsAllInOne {
            cldr_json_root: PathBuf::from(path),
            locale_subset: locale_subset.to_string(),
        })
    } else if matches.is_present("CLDR_TESTDATA") {
        Box::new(CldrPathsAllInOne {
            cldr_json_root: icu_testdata::paths::cldr_json_root(),
            locale_subset: "full".to_string(),
        })
    } else {
        anyhow::bail!("Either --cldr-tag or --cldr-root must be specified",)
    };

    let serializer: Box<dyn serializers::AbstractSerializer> = match matches.value_of("SYNTAX") {
        Some("json") | None => {
            let mut options = serializers::json::Options::default();
            if matches.is_present("PRETTY") {
                options.style = serializers::json::StyleOption::Pretty;
            }
            Box::new(serializers::json::Serializer::new(options))
        }
        #[cfg(feature = "bincode")]
        Some("bincode") => {
            let options = serializers::bincode::Options::default();
            Box::new(serializers::bincode::Serializer::new(options))
        }
        #[cfg(not(feature = "bincode"))]
        Some("bincode") => {
            use icu_provider_fs::manifest::SyntaxOption;
            use icu_provider_fs::FsDataError;
            Err(FsDataError::UnknownSyntax(SyntaxOption::Bincode))?;
            unreachable!()
        }
        _ => unreachable!(),
    };

    let mut options = fs_exporter::ExporterOptions::default();
    options.root = output_path;
    if let Some(value) = matches.value_of("ALIASING") {
        options.aliasing = match value {
            "none" => manifest::AliasOption::NoAliases,
            "symlink" => manifest::AliasOption::Symlink,
            _ => unreachable!(),
        };
    }
    if matches.is_present("OVERWRITE") {
        options.overwrite = fs_exporter::OverwriteOption::RemoveAndReplace
    }
    if let Some(locale_strs) = matches.values_of("LOCALES") {
        let locales_vec = locale_strs
            .map(|s| LanguageIdentifier::from_str(s).with_context(|| s.to_string()))
            .collect::<Result<Vec<LanguageIdentifier>, anyhow::Error>>()?;
        options.locales = LocalesOption::IncludeList(locales_vec.into_boxed_slice());
    } else if matches.is_present("TEST_LOCALES") {
        // TODO: Wait for thiserror to remove unwrap
        let locales_vec = icu_testdata::metadata::load()
            .unwrap()
            .package_metadata
            .locales;
        options.locales = LocalesOption::IncludeList(locales_vec.into_boxed_slice());
    }
    let mut exporter = FilesystemExporter::try_new(serializer, options)?;

    export_cldr(cldr_paths.as_ref(), &mut exporter)?;

    Ok(())
}

fn export_cldr(
    cldr_paths: &dyn CldrPaths,
    exporter: &mut FilesystemExporter,
) -> anyhow::Result<()> {
    let keys = get_all_cldr_keys();

    let provider = CldrJsonDataProvider::new(cldr_paths);
    for key in keys.iter() {
        log::info!("Writing key: {}", key);
        let result = exporter.put_key_from_provider(key, &provider);
        // Ensure flush() is called, even when the result is an error
        exporter.flush()?;
        result?;
    }

    Ok(())
}
