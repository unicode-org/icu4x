// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{ArgAction, Parser};
use eyre::WrapErr;
use icu_locale_core::*;
use icu_provider::DataError;
use icu_provider_source::SourceDataProvider;
use simple_logger::SimpleLogger;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, BufRead, BufWriter, Cursor, Write};
use std::path::PathBuf;
use std::process::Command;
use zip::ZipArchive;

include!("../globs.rs.data");
include!("../../../../provider/source/tests/locales.rs.data");

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
        std::path::Path::new(std::env!("CARGO_MANIFEST_DIR")).join("../../../provider/source");

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
                    .into_body()
                    .into_reader(),
                &mut BufWriter::new(File::create(&root)?),
            )?;
        }

        log::info!("Reading: {root:?}");

        Ok(root)
    }

    fn extract_zip(
        zip: PathBuf,
        paths: Vec<String>,
        root: PathBuf,
        success: &mut BTreeSet<String>,
    ) -> eyre::Result<()> {
        let mut zip = ZipArchive::new(Cursor::new(
            std::fs::read(&zip).expect("should just have been downloaded"),
        ))
        .with_context(|| format!("Failed to read zip file {:?}", &zip))?;

        for spath in paths {
            if let Ok(mut file) = zip.by_name(&spath) {
                let path = root.join(&spath);
                fs::create_dir_all(path.parent().unwrap())?;
                io::copy(
                    &mut file,
                    &mut crlify::BufWriterWithLineEndingFix::new(
                        File::create(&path)
                            .with_context(|| format!("Failed to create file {:?}", &path))?,
                    ),
                )
                .with_context(|| format!("Failed to write file {:?}", &path))?;
                success.insert(spath);
            }
        }
        Ok(())
    }

    fn extract_tar(
        tar: PathBuf,
        paths: BTreeSet<String>,
        root: PathBuf,
        success: &mut BTreeSet<String>,
    ) -> eyre::Result<()> {
        let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(
            std::fs::File::open(&tar).expect("should just have been downloaded"),
        ));

        for e in tar.entries()? {
            let mut e = e?;
            let Some(spath) = e.path()?.to_str().map(ToString::to_string) else {
                continue;
            };
            if paths.contains(&spath) {
                let path = root.join(&spath);
                fs::create_dir_all(path.parent().unwrap())?;
                io::copy(
                    &mut e,
                    &mut crlify::BufWriterWithLineEndingFix::new(
                        File::create(&path)
                            .with_context(|| format!("Failed to create file {:?}", &path))?,
                    ),
                )
                .with_context(|| format!("Failed to write file {:?}", &spath))?;
                success.insert(spath);
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

    std::fs::remove_dir_all(out_root.join("tests/data/cldr"))?;
    let mut cldr_files = BTreeSet::new();
    extract_zip(
        cached(&format!(
            "https://github.com/unicode-org/cldr-json/releases/download/{}/cldr-{}-json-full.zip",
            SourceDataProvider::TESTED_CLDR_TAG,
            SourceDataProvider::TESTED_CLDR_TAG
        ))
        .with_context(|| "Failed to download CLDR ZIP".to_owned())?,
        expand_paths(CLDR_JSON_GLOB, false),
        out_root.join("tests/data/cldr"),
        &mut cldr_files,
    )?;

    std::fs::remove_dir_all(out_root.join("tests/data/icuexport"))?;
    let mut icuexport_files = BTreeSet::new();
    extract_zip(
        cached(&format!(
            "https://github.com/unicode-org/icu/releases/download/{}/icu4x-icuexportdata-{}.zip",
            SourceDataProvider::TESTED_ICUEXPORT_TAG,
            SourceDataProvider::TESTED_ICUEXPORT_TAG
                .replace("release-", "")
                .replace("icu4x-", "")
        ))
        .with_context(|| "Failed to download ICU ZIP".to_owned())?,
        expand_paths(ICUEXPORTDATA_GLOB, true),
        out_root.join("tests/data/icuexport"),
        &mut icuexport_files,
    )?;

    std::fs::remove_dir_all(out_root.join("tests/data/lstm"))?;
    let mut lstm_files = BTreeSet::new();
    extract_zip(
        cached(&format!(
            "https://github.com/unicode-org/lstm_word_segmentation/releases/download/{}/models.zip",
            SourceDataProvider::TESTED_SEGMENTER_LSTM_TAG,
        ))
        .with_context(|| "Failed to download LSTM ZIP".to_owned())?,
        LSTM_GLOB.iter().copied().map(String::from).collect(),
        out_root.join("tests/data/lstm"),
        &mut lstm_files,
    )?;

    std::fs::remove_dir_all(out_root.join("tests/data/unihan"))?;
    let mut unihan_files = BTreeSet::new();
    extract_zip(
        cached(&format!(
            "https://www.unicode.org/Public/{}/ucd/Unihan.zip",
            SourceDataProvider::TESTED_UCD_TAG,
        ))
        .with_context(|| "Failed to download Unihan ZIP".to_owned())?,
        UNIHAN_GLOB.iter().copied().map(String::from).collect(),
        out_root.join("tests/data/unihan"),
        &mut unihan_files,
    )?;
    let irg_path = out_root.join("tests/data/unihan/Unihan_IRGSources.txt");
    let file = File::open(&irg_path)?;
    let reader = io::BufReader::new(file);
    let filtered_content: String = reader
        .lines()
        .map_while(Result::ok)
        .filter(|l| l.contains("kRSUnicode") || l.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::remove_dir_all(out_root.join("tests/data/ucd"))?;
    let mut ucd_files = BTreeSet::new();
    for spath in UCD_GLOB {
        let path = out_root.join("tests/data/ucd").join(spath);
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::copy(
            cached(&format!(
                "https://www.unicode.org/Public/{}/security/IdentifierStatus.txt",
                SourceDataProvider::TESTED_UCD_TAG,
            ))
            .with_context(|| "Failed to download IdentifierStatus.txt".to_owned())?,
            &path,
        )?;
        ucd_files.insert(spath.to_string());
    }
    fs::write(&irg_path, filtered_content)?;
    let identifier_status_path = out_root.join("tests/data/ucd/security/IdentifierStatus.txt");
    let file = File::open(&identifier_status_path)?;
    let reader = io::BufReader::new(file);
    let filtered_content: String = reader
        .lines()
        .map_while(Result::ok)
        .filter(|l| l.contains("CJK") || l.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&identifier_status_path, filtered_content)?;

    std::fs::remove_dir_all(out_root.join("tests/data/tzdb"))?;
    let mut tzdb_files = BTreeSet::new();
    extract_tar(
        cached(&format!(
            "https://www.iana.org/time-zones/repository/releases/tzdata{}.tar.gz",
            SourceDataProvider::TESTED_TZDB_TAG,
        ))
        .with_context(|| "Failed to download TZDB ZIP".to_owned())?,
        TZDB_GLOB.iter().copied().map(String::from).collect(),
        out_root.join("tests/data/tzdb"),
        &mut tzdb_files,
    )?;

    let gen_files = ["rearguard.zi".into(), "vanguard.zi".into()];
    Command::new("make")
        .arg("-C")
        .arg(out_root.join("tests/data/tzdb"))
        .args(&gen_files)
        .status()
        .unwrap();
    tzdb_files.extend(gen_files);
    std::fs::remove_file(out_root.join("tests/data/tzdb/Makefile")).unwrap();
    std::fs::remove_file(out_root.join("tests/data/tzdb/ziguard.awk")).unwrap();
    tzdb_files.remove("Makefile");
    tzdb_files.remove("ziguard.awk");

    let [cldr_files, icuexport_files, lstm_files, unihan_files, ucd_files, tzdb_files] = [
        cldr_files,
        icuexport_files,
        lstm_files,
        unihan_files,
        ucd_files,
        tzdb_files,
    ]
    .map(|files| {
        files
            .iter()
            .map(|path| format!("{path:?}"))
            .collect::<Vec<_>>()
            .join(",\n        ")
    });

    write!(
        &mut crlify::BufWriterWithLineEndingFix::new(
            File::create(out_root.join("src/tests/data.rs")).unwrap()
        ),
        "\
// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Generated by `download-repo-sources.rs`

use crate::source::{{include_files, AbstractFs}};

#[rustfmt::skip]
pub fn cldr_data() -> AbstractFs {{
    include_files!(
        \"../../tests/data/cldr/\";
        {cldr_files}
    )
}}

#[rustfmt::skip]
pub fn icuexport_data() -> AbstractFs {{
    include_files!(
        \"../../tests/data/icuexport/\";
        {icuexport_files}
    )
}}

#[rustfmt::skip]
pub fn lstm_data() -> AbstractFs {{
    include_files!(
        \"../../tests/data/lstm/\";
        {lstm_files}
    )
}}

#[rustfmt::skip]
pub fn unihan_data() -> AbstractFs {{
    include_files!(
        \"../../tests/data/unihan/\";
        {unihan_files}
    )
}}

#[rustfmt::skip]
pub fn ucd_data() -> AbstractFs {{
    include_files!(
        \"../../tests/data/ucd/\";
        {ucd_files}
    )
}}

#[rustfmt::skip]
pub fn tzdb_data() -> AbstractFs {{
    include_files!(
        \"../../tests/data/tzdb/\";
        {tzdb_files}
    )
}}
"
    )
    .unwrap();

    Ok(())
}
