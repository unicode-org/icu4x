// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{App, Arg};
use eyre::WrapErr;
use icu_datagen::SourceData;
use icu_locid::*;
use simple_logger::SimpleLogger;
use std::fs::{self, File};
use std::io::{self, Cursor};
use std::path::PathBuf;
use zip::ZipArchive;

include!("../../globs.rs.data");
include!("../../locales.rs.data");

fn main() -> eyre::Result<()> {
    let args = App::new("ICU4X Test Data Downloader")
        .version("0.0.1")
        .author("The ICU4X Project Developers")
        .about("Download data from CLDR and ICU for ICU4X testing")
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity (-v, -vv, or -vvv)"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("out")
                .help(
                    "Path to output data directory. The subdirectories 'cldr' and 'icuexport' will be overwritten. Omit this option to write data into the package tree.",
                )
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    match args.occurrences_of("VERBOSE") {
        0 => SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap(),
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
        _ => eyre::bail!("Only -v, -vv, and -vvv are supported"),
    }

    let output_path = PathBuf::from(args.value_of_os("OUTPUT").expect("arg is required"));

    let cached = cached_path::CacheBuilder::new()
        .freshness_lifetime(u64::MAX)
        .build()
        .unwrap();

    fn extract(zip: PathBuf, paths: Vec<String>, root: PathBuf) -> eyre::Result<()> {
        let mut zip = ZipArchive::new(Cursor::new(
            std::fs::read(&zip).expect("should just have been downloaded"),
        ))
        .with_context(|| format!("Failed to read zip file {:?}", &zip))?;

        let _ = fs::remove_dir_all(&root);

        for path in paths {
            if let Ok(mut file) = zip.by_name(&path) {
                let path = root.join(&path);
                fs::create_dir_all(path.parent().unwrap())?;
                io::copy(
                    &mut file,
                    &mut crlify::BufWriterWithLineEndingFix::new(
                        File::create(&path)
                            .with_context(|| format!("Failed to create file {:?}", &path))?,
                    ),
                )
                .with_context(|| format!("Failed to write file {:?}", &path))?;
            }
        }
        Ok(())
    }

    fn expand_paths(in_paths: &[&str], replace_hyphen_by_underscore: bool) -> Vec<String> {
        let mut paths = vec![];
        for pattern in in_paths {
            if pattern.contains("$LOCALES") {
                for locale in LOCALES.iter() {
                    let mut string = locale.to_string();
                    if replace_hyphen_by_underscore {
                        string = string.replace('-', "_");
                    }
                    paths.push(pattern.replace("$LOCALES", &string));
                }
                // Also add "root" for older CLDRs
                paths.push(pattern.replace("$LOCALES", "root"));
            } else {
                // No variable in pattern
                paths.push(pattern.to_string())
            }
        }
        paths
    }

    extract(
        cached
            .cached_path(&format!(
            "https://github.com/unicode-org/cldr-json/releases/download/{}/cldr-{}-json-full.zip",
            SourceData::LATEST_TESTED_CLDR_TAG,
            SourceData::LATEST_TESTED_CLDR_TAG
        ))
            .with_context(|| "Failed to download CLDR ZIP".to_owned())?,
        expand_paths(CLDR_JSON_GLOB, false),
        output_path.join("cldr"),
    )?;

    extract(
        cached
            .cached_path(&format!(
                "https://github.com/unicode-org/icu/releases/download/{}/icuexportdata_{}.zip",
                SourceData::LATEST_TESTED_ICUEXPORT_TAG,
                SourceData::LATEST_TESTED_ICUEXPORT_TAG.replace('/', "-")
            ))
            .with_context(|| "Failed to download ICU ZIP".to_owned())?,
        expand_paths(ICUEXPORTDATA_GLOB, true),
        output_path.join("icuexport"),
    )?;

    Ok(())
}
