// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{ArgAction, Parser};
use eyre::WrapErr;
use icu_datagen::DatagenProvider;
use icu_locid::*;
use icu_provider::DataError;
use simple_logger::SimpleLogger;
use std::fs::{self, File};
use std::io::{self, BufWriter, Cursor};
use std::path::PathBuf;
use zip::ZipArchive;

include!("../../globs.rs.data");
include!("../../../../provider/datagen/tests/locales.rs.data");

#[derive(Parser)]
#[command(
    author = "The ICU4X Project Developers",
    about = "Download data from CLDR and ICU for ICU4X testing"
)]
struct Args {
    #[arg(short, long, help = "Sets the level of verbosity (-v, -vv, or -vvv)", action = ArgAction::Count)]
    verbose: u8,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    match args.verbose {
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

    let out_root =
        std::path::Path::new(std::env!("CARGO_MANIFEST_DIR")).join("../../provider/datagen");

    fn cached(resource: &str) -> Result<PathBuf, DataError> {
        let root = std::env::var_os("ICU4X_SOURCE_CACHE")
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::temp_dir().join("icu4x-source-cache/"))
            .join(resource.rsplit("//").next().unwrap());

        if !root.exists() {
            log::info!("Downloading {resource}");
            std::fs::create_dir_all(root.parent().unwrap())?;
            std::io::copy(
                &mut ureq::get(resource)
                    .call()
                    .map_err(|e| DataError::custom("Download").with_display_context(&e))?
                    .into_reader(),
                &mut BufWriter::new(File::create(&root)?),
            )?;
        }

        Ok(root)
    }

    fn extract(zip: PathBuf, paths: Vec<String>, root: PathBuf) -> eyre::Result<()> {
        let mut zip = ZipArchive::new(Cursor::new(
            std::fs::read(&zip).expect("should just have been downloaded"),
        ))
        .with_context(|| format!("Failed to read zip file {:?}", &zip))?;

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

    // TODO(#3736): Remove this workaround when cldr-json has transform rules
    std::fs::rename(
        out_root.join("tests/data/cldr/cldr-transforms-full"),
        out_root.join("tests/data/cldr-transforms-full"),
    )?;

    std::fs::remove_dir_all(out_root.join("tests/data/cldr"))?;
    extract(
        cached(&format!(
            "https://github.com/unicode-org/cldr-json/releases/download/{}/cldr-{}-json-full.zip",
            DatagenProvider::LATEST_TESTED_CLDR_TAG,
            DatagenProvider::LATEST_TESTED_CLDR_TAG
        ))
        .with_context(|| "Failed to download CLDR ZIP".to_owned())?,
        expand_paths(CLDR_JSON_GLOB, false),
        out_root.join("tests/data/cldr"),
    )?;

    // TODO(#3736): Remove this workaround when cldr-json has transform rules
    std::fs::rename(
        out_root.join("tests/data/cldr-transforms-full"),
        out_root.join("tests/data/cldr/cldr-transforms-full"),
    )?;

    std::fs::remove_dir_all(out_root.join("tests/data/icuexport"))?;
    extract(
        cached(&format!(
            "https://github.com/unicode-org/icu/releases/download/{}/icuexportdata_{}.zip",
            DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG,
            DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG.replace('/', "-")
        ))
        .with_context(|| "Failed to download ICU ZIP".to_owned())?,
        expand_paths(ICUEXPORTDATA_GLOB, true),
        out_root.join("tests/data/icuexport"),
    )?;

    std::fs::remove_dir_all(out_root.join("tests/data/lstm"))?;
    extract(
        cached(&format!(
            "https://github.com/unicode-org/lstm_word_segmentation/releases/download/{}/models.zip",
            DatagenProvider::LATEST_TESTED_SEGMENTER_LSTM_TAG,
        ))
        .with_context(|| "Failed to download LSTM ZIP".to_owned())?,
        LSTM_GLOB.iter().copied().map(String::from).collect(),
        out_root.join("tests/data/lstm"),
    )?;

    Ok(())
}
