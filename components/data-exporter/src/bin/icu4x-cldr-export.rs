use icu_data_exporter::json_exporter;
use icu_data_exporter::DataExporter;
use icu_data_exporter::JsonFileWriter;

use clap::{App, Arg, ArgGroup};
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::fs;

use icu_cldr_json_data_provider::CldrDataProvider;
use icu_cldr_json_data_provider::CldrPaths;
use icu_data_provider::icu_data_key;

use std::fmt;
use std::path::PathBuf;

// #[derive(Debug)]
enum Error {
    Unsupported(&'static str),
    ExportError(icu_data_exporter::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Unsupported(message) => write!(f, "Unsupported: {}", message),
            Error::ExportError(error) => write!(f, "{}", error),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

impl From<icu_data_exporter::Error> for Error {
    fn from(err: icu_data_exporter::Error) -> Error {
        Error::ExportError(err)
    }
}

// impl fmt::Debug for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // TODO
//     }
// }

fn main() -> Result<(), Error> {
    let matches = App::new("ICU4X Data Exporter")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Export CLDR JSON into the ICU4X data schema")
        .arg(
            Arg::with_name("VERBOSITY")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity."),
        )
        .arg(
            Arg::with_name("DRY_RUN")
                .short("n")
                .long("dry-run")
                .help("Do not touch the filesystem (consider using with -v)."),
        )
        .arg(
            Arg::with_name("OVERWRITE")
                .short("W")
                .long("overwrite")
                .help("Delete the output directory before writing data."),
        )
        .arg(
            Arg::with_name("CLDR_CORE")
                .long("cldr-core")
                .value_name("PATH")
                .help(
                    "Path to cldr-core JSON: \
                    https://github.com/unicode-cldr/cldr-core",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("KEY")
                .short("k")
                .long("keys")
                .multiple(true)
                .takes_value(true)
                .help("Include this data key in the output. Also see --key-file."),
        )
        .arg(
            Arg::with_name("KEY_FILE")
                .short("i")
                .long("key-file")
                .takes_value(true)
                .help(
                    "Path to text file with data keys to include, one per line. Empty lines and \
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
            ArgGroup::with_name("KEYS")
                .arg("KEY")
                .arg("KEY_FILE")
                .arg("ALL_KEYS")
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
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if !matches.is_present("ALL_KEYS") {
        return Err(Error::Unsupported("Lists of keys are not yet supported (see #192)"));
    }

    // TODO: Build up this list from --keys and --key-file
    let keys = [icu_data_key!(plurals: cardinal@1)];

    let output_path = PathBuf::from(
        matches
            .value_of_os("OUTPUT")
            .unwrap_or(OsStr::new("/tmp/icu4x_json")),
    );

    let mut cldr_paths = CldrPaths::default();

    if let Some(path) = matches.value_of("CLDR_CORE") {
        cldr_paths.cldr_core = Ok(path.into());
    }

    let provider = CldrDataProvider::new(&cldr_paths);

    let mut json_options = json_exporter::Options::default();
    json_options.root = output_path;
    json_options.aliasing = json_exporter::AliasOption::Symlink;
    json_options.overwrite = if matches.is_present("OVERWRITE") {
        json_exporter::OverwriteOption::RemoveAndReplace
    } else {
        json_exporter::OverwriteOption::CheckEmpty
    };
    let mut json_file_writer = JsonFileWriter::try_new(&json_options)?;

    let mut data_exporter = DataExporter {
        data_provider: &provider,
        file_writer: &mut json_file_writer,
    };

    data_exporter.write_data_key(
        &icu_data_key!(plurals: cardinal@1),
    )?;

    json_file_writer.flush()?;

    Ok(())
}
