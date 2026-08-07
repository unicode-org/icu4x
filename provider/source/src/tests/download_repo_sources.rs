// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use crate::source::{AbstractFs, RscdCache};
use icu::locale::{LanguageIdentifier, langid};
use icu_provider::DataError;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

include!("../../tests/globs.rs.data");
include!("../../tests/locales.rs.data");

impl AbstractFs {
    fn dump(
        &self,
        target: &Path,
        mut files: BTreeSet<String>,
    ) -> Result<BTreeSet<String>, DataError> {
        std::fs::remove_dir_all(target)?;

        for file in files.clone() {
            if !self.file_exists(&file).unwrap() {
                files.remove(&file);
                continue;
            }

            std::fs::create_dir_all(target.join(&file).parent().unwrap())?;
            crlify::BufWriterWithLineEndingFix::new(File::create(target.join(&file))?)
                .write_all(&self.read_to_buf(&file)?)?;
        }

        Ok(files)
    }
}

impl RscdCache {
    pub fn dump(
        &self,
        target: &Path,
        mut files: BTreeSet<String>,
    ) -> Result<BTreeSet<String>, DataError> {
        std::fs::remove_dir_all(target)?;

        for file in files.clone() {
            if !self.file_exists(&file).unwrap() {
                files.remove(&file);
                continue;
            }

            std::fs::create_dir_all(target.join(&file).parent().unwrap())?;
            crlify::BufWriterWithLineEndingFix::new(File::create(target.join(&file))?)
                .write_all(self.read_to_string(&file)?.as_bytes())?;
        }

        Ok(files)
    }
}

#[test]
#[ignore]
fn download_repo_sources() {
    let crate_root = Path::new(std::env!("CARGO_MANIFEST_DIR"));
    let out_root = crate_root.join("tests/data");

    fn expand_paths(in_paths: &[&str], replace_hyphen_by_underscore: bool) -> BTreeSet<String> {
        let mut paths = BTreeSet::new();
        for pattern in in_paths {
            if pattern.contains("$LOCALES") {
                for locale in LOCALES.iter() {
                    let mut string = locale.to_string();
                    if replace_hyphen_by_underscore {
                        string = string.replace('-', "_");
                    }
                    paths.insert(pattern.replace("$LOCALES", &string));
                }
                // Also add "root" for older CLDRs
                paths.insert(pattern.replace("$LOCALES", "root"));
            } else {
                // No variable in pattern
                paths.insert(pattern.to_string());
            }
        }
        paths
    }

    let provider = SourceDataProvider::new();

    let cldr_files = provider
        .cldr()
        .unwrap()
        .serde_cache
        .root
        .dump(&out_root.join("cldr"), expand_paths(CLDR_JSON_GLOB, false))
        .unwrap();

    let icuexport_files = provider
        .icuexport()
        .unwrap()
        .root
        .dump(
            &out_root.join("icuexport"),
            expand_paths(ICUEXPORTDATA_GLOB, true),
        )
        .unwrap();

    let lstm_files = provider
        .segmenter_lstm()
        .unwrap()
        .root
        .dump(
            &out_root.join("lstm"),
            LSTM_GLOB.iter().copied().map(String::from).collect(),
        )
        .unwrap();

    let rscd_files = provider
        .rscd()
        .unwrap()
        .dump(
            &out_root.join("rscd"),
            RSCD_GLOB.iter().copied().map(String::from).collect(),
        )
        .unwrap();
    let irg_path = out_root.join("rscd/ucd/Unihan/Unihan_IRGSources.txt");
    std::io::copy(
        &mut BufReader::new(File::open(&irg_path).unwrap())
            .lines()
            .map_while(Result::ok)
            .filter(|l| l.contains("kRSUnicode") || l.starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n")
            .as_bytes(),
        &mut crlify::BufWriterWithLineEndingFix::new(File::create(&irg_path).unwrap()),
    )
    .unwrap();

    let mut tzdb_files = provider
        .tzdb()
        .unwrap()
        .root
        .dump(
            &out_root.join("tzdb"),
            TZDB_GLOB.iter().copied().map(String::from).collect(),
        )
        .unwrap();
    let gen_files = ["rearguard.zi".into(), "vanguard.zi".into()];
    Command::new("make")
        .arg("-C")
        .arg(out_root.join("tzdb"))
        .args(&gen_files)
        .status()
        .unwrap();
    tzdb_files.extend(gen_files);
    std::io::copy(
        &mut std::fs::read_to_string(out_root.join("tzdb/rearguard.zi"))
            .unwrap()
            .as_bytes(),
        &mut crlify::BufWriterWithLineEndingFix::new(
            File::create(out_root.join("tzdb/rearguard.zi")).unwrap(),
        ),
    )
    .unwrap();
    std::io::copy(
        &mut std::fs::read_to_string(out_root.join("tzdb/vanguard.zi"))
            .unwrap()
            .as_bytes(),
        &mut crlify::BufWriterWithLineEndingFix::new(
            File::create(out_root.join("tzdb/vanguard.zi")).unwrap(),
        ),
    )
    .unwrap();
    std::fs::remove_file(out_root.join("tzdb/Makefile")).unwrap();
    std::fs::remove_file(out_root.join("tzdb/ziguard.awk")).unwrap();
    tzdb_files.remove("Makefile");
    tzdb_files.remove("ziguard.awk");

    let [
        cldr_files,
        icuexport_files,
        lstm_files,
        rscd_files,
        tzdb_files,
    ] = [
        cldr_files,
        icuexport_files,
        lstm_files,
        rscd_files,
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
            File::create(crate_root.join("src/tests/data.rs")).unwrap()
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
pub fn rscd_data() -> AbstractFs {{
    include_files!(
        \"../../tests/data/rscd/\";
        {rscd_files}
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

    // Download RSCD test files
    for (rscd_path, repo_path) in [
        (
            "ucd/NormalizationTest.txt",
            "components/normalizer/tests/data/NormalizationTest.txt",
        ),
        (
            "ucd/auxiliary/GraphemeBreakTest.txt",
            "components/segmenter/tests/testdata/GraphemeBreakTest.txt",
        ),
        (
            "ucd/auxiliary/LineBreakTest.txt",
            "components/segmenter/tests/testdata/LineBreakTest.txt",
        ),
        (
            "ucd/auxiliary/SentenceBreakTest.txt",
            "components/segmenter/tests/testdata/SentenceBreakTest.txt",
        ),
        (
            "ucd/auxiliary/WordBreakTest.txt",
            "components/segmenter/tests/testdata/WordBreakTest.txt",
        ),
    ] {
        std::fs::write(
            crate_root.join("../..").join(repo_path),
            provider
                .rscd()
                .unwrap()
                .read_to_string(rscd_path)
                .expect(rscd_path),
        )
        .unwrap();
    }

    for (cldr_path, repo_path) in [
        (
            "common/testData/units/unitsTest.txt",
            "components/experimental/tests/units/data/unitsTest.txt",
        ),
        (
            "common/uca/CollationTest_CLDR_SHIFTED_SHORT.txt",
            "components/collator/tests/data/CollationTest_CLDR_SHIFTED_SHORT.txt",
        ),
        (
            "common/uca/CollationTest_CLDR_NON_IGNORABLE_SHORT.txt",
            "components/collator/tests/data/CollationTest_CLDR_NON_IGNORABLE_SHORT.txt",
        ),
        (
            "common/testData/transforms/el-Latn-t-el-m0-bgn.txt",
            "components/experimental/tests/transliterate/data/fixtures/el-Latn-t-el-m0-bgn.txt",
        ),
        (
            "common/testData/transforms/und-Arab-t-und-beng.txt",
            "components/experimental/tests/transliterate/data/fixtures/und-Arab-t-und-beng.txt",
        ),
        (
            "common/testData/transforms/und-t-d0-publish.txt",
            "components/experimental/tests/transliterate/data/fixtures/und-t-d0-publish.txt",
        ),
        (
            "common/testData/transforms/und-t-s0-publish.txt",
            "components/experimental/tests/transliterate/data/fixtures/und-t-s0-publish.txt",
        ),
        (
            "common/testData/transforms/und-t-und-latn-d0-ascii.txt",
            "components/experimental/tests/transliterate/data/fixtures/und-t-und-latn-d0-ascii.txt",
        ),
    ] {
        std::fs::write(
            crate_root.join("../..").join(repo_path),
            AbstractFs::new_from_url(format!(
                "https://raw.githubusercontent.com/unicode-org/cldr/refs/tags/release-{}/",
                SourceDataProvider::TESTED_CLDR_TAG
                    .replace(".", "-")
                    .rsplit_once("-")
                    .unwrap()
                    .0
            ))
            .read_to_string(cldr_path)
            .expect(cldr_path),
        )
        .unwrap();
    }

    #[allow(clippy::single_element_loop)]
    for (icu_path, repo_path) in [(
        "icu4c/source/test/testdata/riwords.txt",
        "components/collator/tests/data/riwords.txt",
    )] {
        std::fs::write(
            crate_root.join("../..").join(repo_path),
            AbstractFs::new_from_url(format!(
                "https://raw.githubusercontent.com/unicode-org/icu/refs/tags/{}/",
                SourceDataProvider::TESTED_ICUEXPORT_TAG
            ))
            .read_to_string(icu_path)
            .expect(icu_path),
        )
        .unwrap();
    }
}
