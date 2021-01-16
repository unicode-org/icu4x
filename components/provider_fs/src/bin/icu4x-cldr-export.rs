// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::manifest::LocalesOption;
use clap::{App, Arg, ArgGroup};
use icu_locid::LanguageIdentifier;
use icu_provider::iter::DataExporter;
use icu_provider_cldr::download::CldrAllInOneDownloader;
use icu_provider_cldr::get_all_resc_keys;
use icu_provider_cldr::CldrJsonDataProvider;
use icu_provider_cldr::CldrPaths;
use icu_provider_cldr::CldrPathsLocal;
use icu_provider_fs::export::fs_exporter;
use icu_provider_fs::export::serializers;
use icu_provider_fs::export::FilesystemExporter;
use icu_provider_fs::manifest;
use simple_logger::SimpleLogger;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

enum Error {
    Unsupported(&'static str),
    Export(icu_provider_fs::FsDataError),
    DataProvider(icu_provider::DataError),
    LocaleParser(icu_locid::ParserError, String),
    Setup(Box<dyn std::error::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Unsupported(message) => write!(f, "Unsupported: {}", message),
            Error::Export(error) => write!(f, "{}", error),
            Error::DataProvider(error) => write!(f, "{}", error),
            Error::LocaleParser(error, s) => write!(f, "{}: {}", error, s),
            Error::Setup(error) => write!(f, "{}", error),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

impl From<icu_provider_fs::FsDataError> for Error {
    fn from(err: icu_provider_fs::FsDataError) -> Error {
        Error::Export(err)
    }
}

impl From<icu_provider::DataError> for Error {
    fn from(err: icu_provider::DataError) -> Error {
        Error::DataProvider(err)
    }
}

impl From<icu_provider_cldr::download::Error> for Error {
    fn from(err: icu_provider_cldr::download::Error) -> Error {
        Error::Setup(Box::from(err))
    }
}

fn main() -> Result<(), Error> {
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
                .help("File format syntax for data files (defaults to JSON)."),
        )
        .arg(
            Arg::with_name("PRETTY")
                .short("p")
                .long("pretty")
                .help("Whether to pretty-print the output JSON files. Ignored for Bincode."),
        )
        .arg(
            Arg::with_name("CLDR_TAG")
                .long("cldr-tag")
                .value_name("TAG")
                .help(
                    "Download CLDR 38+ JSON data from this GitHub tag: \n\
                    https://github.com/unicode-org/cldr-json/tags",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CLDR_CORE")
                .long("cldr-core")
                .value_name("PATH")
                .help(
                    "Path to cldr-core. Ignored if '--cldr-tag' is present. \n\
                    https://github.com/unicode-cldr/cldr-core",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CLDR_DATES")
                .long("cldr-dates")
                .value_name("PATH")
                .help(
                    "Path to cldr-dates. Ignored if '--cldr-tag' is present. \n\
                    https://github.com/unicode-cldr/cldr-dates-full",
                )
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
                .short("i")
                .long("key-file")
                .takes_value(true)
                .help(
                    "Path to text file with resource keys to include, one per line. Empty lines and \
                    lines starting with '#' are ignored. Also see --key.",
                ),
        )
        .arg(
            Arg::with_name("ALL_KEYS")
                .short("A")
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
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to output data directory. Must be empty or non-existent, unless \
                    --overwrite is present, in which case the directory is deleted first.",
                )
                .takes_value(true)
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
        _ => return Err(Error::Unsupported("Only -v and -vv are supported")),
    }

    if !matches.is_present("ALL_KEYS") {
        return Err(Error::Unsupported(
            "Lists of keys are not yet supported (see #192)",
        ));
    }

    if matches.is_present("DRY_RUN") {
        return Err(Error::Unsupported("Dry-run is not yet supported"));
    }

    // TODO: Build up this list from --keys and --key-file
    let keys = get_all_resc_keys();

    let output_path = PathBuf::from(
        matches
            .value_of_os("OUTPUT")
            .expect("--out is a required option"),
    );

    let cldr_paths: Box<dyn CldrPaths> = if let Some(tag) = matches.value_of("CLDR_TAG") {
        Box::new(CldrAllInOneDownloader::try_from_github_tag(tag)?)
    } else {
        let mut cldr_paths_local = CldrPathsLocal::default();
        if let Some(path) = matches.value_of("CLDR_CORE") {
            cldr_paths_local.cldr_core = Ok(path.into());
        }
        if let Some(path) = matches.value_of("CLDR_DATES") {
            cldr_paths_local.cldr_dates = Ok(path.into());
        }
        Box::new(cldr_paths_local)
    };

    let provider = CldrJsonDataProvider::new(cldr_paths.as_ref());

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
            return Err(Error::Export(FsDataError::UnknownSyntax(
                SyntaxOption::Bincode,
            )));
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
            .map(|s| {
                LanguageIdentifier::from_str(s).map_err(|e| Error::LocaleParser(e, s.to_string()))
            })
            .collect::<Result<Vec<LanguageIdentifier>, Error>>()?;
        options.locales = LocalesOption::IncludeList(locales_vec.into_boxed_slice());
    }
    let mut exporter = FilesystemExporter::try_new(serializer, options)?;

    for key in keys.iter() {
        log::info!("Writing key: {}", key);
        let result = exporter.put_key_from_provider(key, &provider);
        // Ensure flush() is called, even when the result is an error
        exporter.flush()?;
        result?;
    }

    Ok(())
}
