// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use clap::{App, Arg};
use icu_cldr_json_data_provider::download::CldrPathsDownload;
use icu_cldr_json_data_provider::CldrJsonDataProvider;
use icu_data_provider::icu_data_key;
use icu_data_provider::iter::IterableDataProvider;
use icu_fs_data_provider::export::fs_exporter;
use icu_fs_data_provider::export::serializers;
use icu_fs_data_provider::export::FilesystemExporter;
use icu_fs_data_provider::manifest;
use simple_logger::SimpleLogger;
use std::fmt;
use std::path::PathBuf;

enum Error {
    Unsupported(&'static str),
    Export(icu_fs_data_provider::FsDataError),
    DataProvider(icu_data_provider::DataError),
    Metadata(icu_testdata::metadata::Error),
    Download(icu_cldr_json_data_provider::download::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Unsupported(message) => write!(f, "Unsupported: {}", message),
            Error::Export(error) => write!(f, "{}", error),
            Error::DataProvider(error) => write!(f, "{}", error),
            Error::Metadata(error) => write!(f, "Metadata Error: {}", error),
            Error::Download(error) => write!(f, "Download Error: {}", error),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

impl From<icu_fs_data_provider::FsDataError> for Error {
    fn from(err: icu_fs_data_provider::FsDataError) -> Error {
        Error::Export(err)
    }
}

impl From<icu_data_provider::DataError> for Error {
    fn from(err: icu_data_provider::DataError) -> Error {
        Error::DataProvider(err)
    }
}

impl From<icu_testdata::metadata::Error> for Error {
    fn from(err: icu_testdata::metadata::Error) -> Error {
        Error::Metadata(err)
    }
}

impl From<icu_cldr_json_data_provider::download::Error> for Error {
    fn from(err: icu_cldr_json_data_provider::download::Error) -> Error {
        Error::Download(err)
    }
}

fn main() -> Result<(), Error> {
    let default_out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("json")
        .into_os_string();

    let matches = App::new("ICU4X Test Data Generator")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Export CLDR JSON into the ICU4X data schema for test data")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity (-v or -vv)"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to output data directory. The directory will be overwritten. \
                    Omit this option to write data into the package tree.",
                )
                .takes_value(true)
                .default_value_os(&default_out_dir),
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

    // TODO: Save this list of keys in a better place
    let keys = [
        icu_data_key!(plurals: cardinal@1),
        icu_data_key!(plurals: ordinal@1),
        icu_data_key!(dates: gregory@1),
    ];

    let metadata = icu_testdata::metadata::load()?;

    log::info!("Package metadata: {:?}", metadata);

    let output_path = PathBuf::from(
        matches
            .value_of_os("OUTPUT")
            .expect("Option has a default value"),
    );

    log::info!("Writing testdata to: {:?}", output_path);

    let cldr_paths = CldrPathsDownload::try_from_github_tag(&metadata.package_metadata.gitref)?;
    let provider = CldrJsonDataProvider::new(&cldr_paths);

    let mut options = serializers::JsonSerializerOptions::default();
    options.style = serializers::StyleOption::Pretty;
    let json_serializer = Box::new(serializers::JsonSerializer::new(&options));

    let mut options = fs_exporter::ExporterOptions::default();
    options.root = output_path;
    options.locales =
        manifest::LocalesOption::IncludeList(metadata.package_metadata.locales.into_boxed_slice());
    options.overwrite = fs_exporter::OverwriteOption::RemoveAndReplace;
    let mut exporter = FilesystemExporter::try_new(json_serializer, options)?;

    for key in keys.iter() {
        log::info!("Processing key: {}", key);
        let result = provider.export_key(key, &mut exporter);
        // Ensure flush() is called, even when the result is an error
        exporter.flush()?;
        result?;
    }

    Ok(())
}
