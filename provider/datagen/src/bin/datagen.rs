// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{App, Arg, ArgGroup, ArgMatches};
use eyre::WrapErr;

use icu_datagen::cldr::CldrPathsAllInOne;
use icu_datagen::{get_all_keys, DatagenOptions};

use icu_locid::LanguageIdentifier;
use icu_provider::datagen::IterableDynProvider;
use icu_provider::export::DataExporter;
use icu_provider::hello_world::{HelloWorldProvider, HelloWorldV1Marker};
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use icu_provider_adapters::filter::Filterable;

use icu_provider_blob::export::BlobExporter;
use icu_provider_fs::export::fs_exporter;
use icu_provider_fs::export::serializers;
use icu_provider_fs::export::FilesystemExporter;
use rayon::prelude::*;
use simple_logger::SimpleLogger;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
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
            Arg::with_name("DRY_RUN")
                .short("n")
                .long("dry-run")
                .help("Do not touch the filesystem (consider using with -v)."),
        )
        .arg(
            Arg::with_name("FORMAT")
                .long("format")
                .takes_value(true)
                .possible_value("dir")
                .possible_value("blob")
                .help("Output to a directory on the filesystem or a single blob.")
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
                .help("Whether to pretty-print the output JSON files. Ignored for Bincode."),
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
            Arg::with_name("UPROPS_TAG")
                .long("uprops-tag")
                .value_name("TAG")
                .help(
                    "Download Unicode Properties data from this GitHub tag (https://github.com/unicode-org/icu/tags)\n\
                    Note that some keys do not support versions before release-71-1."
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("UPROPS_ROOT")
                .long("uprops-root")
                .value_name("PATH")
                .help(
                    "Path to a local icuexportdata_uprops_full directory (see https://github.com/unicode-org/icu/releases).\n\
                    Ignored if '--uprops-tag is present.\n\
                    Note that some keys do not support versions before release-71-1.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("UPROPS_MODE")
                .takes_value(true)
                .possible_values(&["small", "fast"])
                .help("Whether to optimize Unicode property data structures for size (\"small\") or speed (\"fast\")")
                .default_value("small"),
        )
        .arg(
            Arg::with_name("INPUT_FROM_TESTDATA")
                .long("input-from-testdata")
                .help("Load input data from the icu_testdata project."),
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
            Arg::with_name("HELLO_WORLD")
                .long("hello-world-key")
                .help("Whether to include the 'hello world' key."),
        )
        .arg(
            Arg::with_name("ALL_KEYS")
                .long("all-keys")
                .help("Include all keys known to ICU4X."),
        )
        .arg(
            Arg::with_name("TEST_KEYS")
                .long("test-keys")
                .help("Include all keys supported by testdata."),
        )
        .group(
            ArgGroup::with_name("KEY_MODE")
                .arg("KEYS")
                .arg("KEY_FILE")
                .arg("HELLO_WORLD")
                .arg("ALL_KEYS")
                .arg("TEST_KEYS")
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
                    "Path to output directory or file. Must be empty or non-existent, unless \
                    --overwrite is present, in which case the directory is deleted first. \
                    For --format blob, omit this option to dump to stdout.",
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
                .arg("OUTPUT_TESTDATA"),
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

    if matches.is_present("DRY_RUN") {
        eyre::bail!("Dry-run is not yet supported");
    }

    let selected_locales = if let Some(locale_strs) = matches.values_of("LOCALES") {
        Some(
            locale_strs
                .map(|s| LanguageIdentifier::from_str(s).with_context(|| s.to_string()))
                .collect::<Result<Vec<LanguageIdentifier>, eyre::Error>>()?,
        )
    } else if matches.is_present("TEST_LOCALES") {
        Some(icu_testdata::metadata::load()?.package_metadata.locales)
    } else {
        None
    };

    let all_keys = get_all_keys();

    #[allow(clippy::if_same_then_else)]
    let selected_keys = if matches.is_present("ALL_KEYS") {
        all_keys
    } else if matches.is_present("TEST_KEYS") {
        // We go with all keys now and later ignore key errors.
        all_keys
    } else if matches.is_present("HELLO_WORLD") {
        vec![HelloWorldV1Marker::KEY]
    } else {
        let mut keys = HashSet::new();

        if let Some(paths) = matches.values_of("KEYS") {
            keys.extend(paths.map(Cow::Borrowed));
        } else if let Some(key_file_path) = matches.value_of_os("KEY_FILE") {
            let file = File::open(key_file_path)
                .with_context(|| key_file_path.to_string_lossy().into_owned())?;
            for line in io::BufReader::new(file).lines() {
                let path = line.with_context(|| key_file_path.to_string_lossy().into_owned())?;
                keys.insert(Cow::Owned(path));
            }
        }

        let filtered: Vec<_> = all_keys
            .into_iter()
            .filter(|k| keys.contains(k.get_path()))
            .collect();

        if filtered.is_empty() {
            eyre::bail!("No keys selected");
        }

        filtered
    };

    let mut provider: Box<dyn IterableDynProvider<SerializeMarker> + Sync> = if matches
        .is_present("HELLO_WORLD")
    {
        Box::new(HelloWorldProvider::new_with_placeholder_data())
    } else {
        let cldr_paths = CldrPathsAllInOne {
            cldr_json_root: if let Some(_tag) = matches.value_of("CLDR_TAG") {
                #[cfg(not(feature = "download"))]
                eyre::bail!("--cldr-tag requires the download feature");
                #[cfg(feature = "download")]
                cached_path::CacheBuilder::new().freshness_lifetime(u64::MAX).build()?
                    .cached_path_with_options(
                        &format!(
                            "https://github.com/unicode-org/cldr-json/releases/download/{}/cldr-{}-json-{}.zip",
                            _tag, _tag, matches.value_of("CLDR_LOCALE_SUBSET").unwrap_or("full")),
                        &cached_path::Options::default().extract(),
                    )?
            } else if let Some(path) = matches.value_of("CLDR_ROOT") {
                PathBuf::from(path)
            } else if matches.is_present("INPUT_FROM_TESTDATA") {
                icu_testdata::paths::cldr_json_root()
            } else {
                eyre::bail!(
                    "Either --cldr-tag or --cldr-root or --input-from-testdata must be specified",
                )
            },
            locale_subset: matches
                .value_of("CLDR_LOCALE_SUBSET")
                .unwrap_or("full")
                .to_string(),
        };

        let uprops_root = if let Some(_tag) = matches.value_of("UPROPS_TAG") {
            #[cfg(not(feature = "download"))]
            eyre::bail!("--uprops-tag requires the download feature");
            #[cfg(feature = "download")]
            cached_path::CacheBuilder::new().freshness_lifetime(u64::MAX).build()?
                .cached_path_with_options(
                    &format!("https://github.com/unicode-org/icu/releases/download/{}/icuexportdata_uprops_full.zip", _tag),
                    &cached_path::Options::default().extract()
                )?
                .join("icuexportdata_uprops_full")
                .join(matches.value_of("UPROPS_MODE").unwrap())
        } else if let Some(path) = matches.value_of("UPROPS_ROOT") {
            PathBuf::from(path)
        } else if matches.is_present("INPUT_FROM_TESTDATA") {
            icu_testdata::paths::uprops_toml_root()
        } else {
            eyre::bail!(
                "Either --uprops-tag or --uprops-root or --input-from-testdata must be specified",
            )
        };

        let segmenter_data_root = icu_datagen::segmenter::segmenter_data_root();

        let p = icu_datagen::create_datagen_provider!(DatagenOptions {
            cldr_paths: Some(&cldr_paths),
            uprops_root: Some(&uprops_root),
            segmenter_data_root: Some(&segmenter_data_root),
        });

        Box::new(p)
    };

    if let Some(locales) = selected_locales.as_ref() {
        provider = Box::new(
            provider
                .filterable("icu4x-datagen locales")
                .filter_by_langid_allowlist_strict(locales),
        );
    }

    let mut exporter: Box<dyn DataExporter<_>> = match matches
        .value_of("FORMAT")
        .expect("Option has default value")
    {
        "dir" => Box::new(get_fs_exporter(&matches)?),
        "blob" => Box::new(get_blob_exporter(&matches)?),
        _ => unreachable!(),
    };

    selected_keys.into_par_iter().try_for_each(|key| {
        let result = provider
            .supported_options_for_key(key)?
            .collect::<Vec<_>>()
            .into_par_iter()
            .try_for_each(|options| {
                let payload = provider
                    .load_payload(
                        key,
                        &DataRequest {
                            options: options.clone(),
                            metadata: Default::default(),
                        },
                    )?
                    .take_payload()?;
                exporter.put_payload(key, options, payload)
            });

        exporter.flush(key)?;

        if matches.is_present("TEST_KEYS")
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

fn get_fs_exporter(matches: &ArgMatches) -> eyre::Result<FilesystemExporter> {
    let syntax = matches.value_of("SYNTAX").unwrap_or("json");

    let output_path: PathBuf = if matches.is_present("OUTPUT_TESTDATA") {
        icu_testdata::paths::data_root().join(syntax)
    } else if let Some(v) = matches.value_of_os("OUTPUT") {
        PathBuf::from(v)
    } else {
        eyre::bail!("--out must be specified for --format=dir");
    };

    log::info!("Writing to filesystem tree at: {}", output_path.display());

    let serializer: Box<dyn serializers::AbstractSerializer + Sync> =
        match matches.value_of("SYNTAX") {
            Some("json") | None => {
                let mut options = serializers::json::Options::default();
                if matches.is_present("PRETTY") {
                    options.style = serializers::json::StyleOption::Pretty;
                }
                Box::new(serializers::json::Serializer::new(options))
            }
            Some("bincode") => {
                let options = serializers::bincode::Options::default();
                Box::new(serializers::bincode::Serializer::new(options))
            }
            Some("postcard") => {
                let options = serializers::postcard::Options::default();
                Box::new(serializers::postcard::Serializer::new(options))
            }
            _ => unreachable!(),
        };

    let mut options = fs_exporter::ExporterOptions::default();
    options.root = output_path;
    if matches.is_present("OVERWRITE") {
        options.overwrite = fs_exporter::OverwriteOption::RemoveAndReplace
    }

    let exporter = FilesystemExporter::try_new(serializer, options)?;
    Ok(exporter)
}

fn get_blob_exporter(matches: &ArgMatches) -> eyre::Result<BlobExporter<'static>> {
    if matches.value_of("SYNTAX") == Some("json") {
        eyre::bail!("Cannot use --format=blob with --syntax=json");
    }

    let output_path: Option<PathBuf> = if matches.is_present("OUTPUT_TESTDATA") {
        Some(icu_testdata::paths::data_root().join("testdata.postcard"))
    } else {
        matches.value_of_os("OUTPUT").map(PathBuf::from)
    };

    match output_path {
        Some(ref p) => log::info!("Writing blob to filesystem at: {}", p.display()),
        None => log::info!("Writing blob to standard out"),
    };

    let sink: Box<dyn std::io::Write + Sync> = if let Some(path_buf) = output_path {
        if !matches.is_present("OVERWRITE") && path_buf.exists() {
            eyre::bail!("Output path is present: {:?}", path_buf);
        }
        let context = path_buf.to_string_lossy().to_string();
        let temp = std::fs::File::create(path_buf).with_context(|| context)?;
        Box::new(temp)
    } else {
        let temp = std::io::stdout();
        Box::new(temp)
    };

    Ok(BlobExporter::new_with_sink(sink))
}
