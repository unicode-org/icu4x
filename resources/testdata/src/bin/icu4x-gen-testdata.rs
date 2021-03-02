// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use clap::{App, Arg, ArgMatches};
use icu_provider::iter::DataExporter;
use icu_provider_cldr::download::CldrAllInOneDownloader;
use icu_provider_cldr::get_all_resc_keys;
use icu_provider_cldr::{CldrJsonDataProvider, CldrPathsAllInOne};
use icu_provider_fs::export::fs_exporter;
use icu_provider_fs::export::serializers;
use icu_provider_fs::export::FilesystemExporter;
use icu_provider_fs::manifest;
use icu_testdata::metadata::{self, PackageInfo};
use itertools::Itertools;
use simple_logger::SimpleLogger;
use std::path::{Path, PathBuf};
use std::{fmt, fs};
use writeable::Writeable;

enum Error {
    Unsupported(&'static str),
    Io(std::io::Error, Option<PathBuf>),
    Export(icu_provider_fs::FsDataError),
    DataProvider(icu_provider::DataError),
    Metadata(icu_testdata::metadata::Error),
    Download(icu_provider_cldr::download::Error),
    GlobWalk(globwalk::GlobError),
    Walkdir(walkdir::Error),
    StripPrefix(std::path::StripPrefixError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Unsupported(message) => write!(f, "Unsupported: {}", message),
            Error::Io(error, path) => write!(f, "I/O Error: {}, {:?}", error, path),
            Error::Export(error) => write!(f, "{}", error),
            Error::DataProvider(error) => write!(f, "{}", error),
            Error::Metadata(error) => write!(f, "Metadata Error: {}", error),
            Error::Download(error) => write!(f, "Download Error: {}", error),
            Error::GlobWalk(error) => write!(f, "Glob Error: {}", error),
            Error::Walkdir(error) => write!(f, "Walk Error: {}", error),
            Error::StripPrefix(error) => write!(f, "Strip Prefix Error: {}", error),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

/// To help with debugging, I/O errors should be paired with a file path.
/// If a path is unavailable, create the error directly: `Error::Io(err, None)`
impl<P: AsRef<Path>> From<(std::io::Error, P)> for Error {
    fn from(pieces: (std::io::Error, P)) -> Self {
        Self::Io(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
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

impl From<icu_testdata::metadata::Error> for Error {
    fn from(err: icu_testdata::metadata::Error) -> Error {
        Error::Metadata(err)
    }
}

impl From<icu_provider_cldr::download::Error> for Error {
    fn from(err: icu_provider_cldr::download::Error) -> Error {
        Error::Download(err)
    }
}

impl From<globwalk::GlobError> for Error {
    fn from(err: globwalk::GlobError) -> Error {
        Error::GlobWalk(err)
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Error {
        Error::Walkdir(err)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(err: std::path::StripPrefixError) -> Error {
        Error::StripPrefix(err)
    }
}

fn main() -> Result<(), Error> {
    let default_out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("json")
        .into_os_string();

    let args = App::new("ICU4X Test Data Generator")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Export CLDR JSON into the ICU4X data schema for test data")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity (-v, -vv, or -vvv)"),
        )
        .arg(
            Arg::with_name("MODE")
                .short("m")
                .long("mode")
                .help("Tasks to perform: 'download' to download the CLDR data into the repository; 'generate' to convert the CLDR into ICU4X JSON; or 'all' to perform both tasks.")
                .takes_value(true)
                .default_value("all"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to output data directory. The directory will be overwritten. Omit this option to write data into the package tree.",
                )
                .takes_value(true)
                .default_value_os(&default_out_dir),
        )
        .get_matches();

    match args.occurrences_of("VERBOSE") {
        0 => SimpleLogger::from_env().init().unwrap(),
        1 => SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap(),
        2 => SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap(),
        3 => SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap(),
        _ => return Err(Error::Unsupported("Only -v, -vv, and -vvv are supported")),
    }

    let metadata = metadata::load()?;

    log::info!("Package metadata: {:?}", metadata);

    if args.value_of("MODE") == Some("download") || args.value_of("MODE") == Some("all") {
        download(&args, &metadata)?;
    }
    if args.value_of("MODE") == Some("generate") || args.value_of("MODE") == Some("all") {
        generate(&args, &metadata)?;
    }
    Ok(())
}

/// Downloads the CLDR JSON and copies a subset of it into data/cldr
fn download(_args: &ArgMatches, metadata: &PackageInfo) -> Result<(), Error> {
    let downloader =
        CldrAllInOneDownloader::try_new_from_github_tag(&metadata.package_metadata.gitref)?;
    let downloaded_cldr = downloader.download()?;

    let cldr_json_root = icu_testdata::paths::cldr_json_root();
    fs::remove_dir_all(&cldr_json_root).map_err(|e| (e, &cldr_json_root))?;

    let locales_glob_substitute = format!(
        // Include "root" (locales array has "und" instead)
        "{{{},root}}",
        metadata
            .package_metadata
            .locales
            .iter()
            .map(|l| l.writeable_to_string())
            .join(",")
    );
    let glob_patterns: Vec<String> = metadata
        .package_metadata
        .cldr_json_glob
        .iter()
        .map(|pattern| pattern.replace("$LOCALES", &locales_glob_substitute))
        .collect();

    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        downloaded_cldr.cldr_json_root.clone(),
        &glob_patterns,
    )
    .case_insensitive(true)
    .build()?;

    for path in walker {
        let old_path = path?.into_path();
        let relative_path = old_path.strip_prefix(&downloaded_cldr.cldr_json_root)?;
        let new_path = cldr_json_root.join(relative_path);
        fs::create_dir_all(new_path.parent().unwrap()).map_err(|e| (e, &new_path))?;
        log::trace!("Copying {:?}", &relative_path);
        fs::copy(&old_path, &new_path).map_err(|e| (e, &new_path))?;
    }
    Ok(())
}

fn generate(args: &ArgMatches, metadata: &PackageInfo) -> Result<(), Error> {
    let keys = get_all_resc_keys();

    let output_path = PathBuf::from(
        args.value_of_os("OUTPUT")
            .expect("Option has a default value"),
    );

    log::info!("Writing testdata to: {:?}", output_path);

    let cldr_paths = CldrPathsAllInOne {
        cldr_json_root: icu_testdata::paths::cldr_json_root(),
        suffix: "full",
    };
    let provider = CldrJsonDataProvider::new(&cldr_paths);

    let mut options = serializers::json::Options::default();
    options.style = serializers::json::StyleOption::Pretty;
    let json_serializer = Box::new(serializers::json::Serializer::new(options));

    let mut options = fs_exporter::ExporterOptions::default();
    options.root = output_path;
    options.locales = manifest::LocalesOption::IncludeList(
        metadata.package_metadata.locales.clone().into_boxed_slice(),
    );
    options.overwrite = fs_exporter::OverwriteOption::RemoveAndReplace;
    let mut exporter = FilesystemExporter::try_new(json_serializer, options)?;

    for key in keys.iter() {
        log::info!("Processing key: {}", key);
        let result = exporter.put_key_from_provider(key, &provider);
        // Ensure flush() is called, even when the result is an error
        exporter.flush()?;
        result?;
    }

    Ok(())
}
