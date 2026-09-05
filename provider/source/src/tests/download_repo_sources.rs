// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use crate::source::{AbstractFs, RscdCache};
use icu_provider::DataError;
use std::path::Path;

impl RscdCache {
    fn init(&self) -> Result<(), DataError> {
        if let Some(unihan_zip) = self.unihan_zip.as_ref() {
            unihan_zip.init()?;
        }
        if let Some(ucd_zip) = self.ucd_zip.as_ref() {
            ucd_zip.init()?;
        }
        if let Some(uts_39_zip) = self.uts_39_zip.as_ref() {
            uts_39_zip.init()?;
        }

        // List all required files that are not accessed through one of the ZIP files
        self.root.file_exists("emoji/emoji-sequences.txt")?;

        Ok(())
    }
}

#[test]
#[ignore]
fn download_repo_sources() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    println!(
        "Caching sources in {}",
        AbstractFs::data_cache_dir().display()
    );

    let provider = SourceDataProvider::new();

    provider.cldr().unwrap().serde_cache.root.init().unwrap();

    provider.icuexport().unwrap().root.init().unwrap();

    provider.segmenter_lstm().unwrap().root.init().unwrap();

    provider.tzdb().unwrap().root.init().unwrap();

    provider.rscd().unwrap().init().unwrap();

    let repo_root = Path::new(concat!(std::env!("CARGO_MANIFEST_DIR"), "/../.."));

    // Download RSCD test files
    for (rscd_path, repo_path) in [
        (
            "ucd/NormalizationTest.txt",
            "components/normalizer/tests/data/NormalizationTest.txt",
        ),
        (
            "ucd/PropertyValueAliases.txt",
            "components/properties/tests/data/PropertyValueAliases.txt",
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
            repo_root.join(repo_path),
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
            repo_root.join(repo_path),
            AbstractFs::new_from_url(format!(
                "https://raw.githubusercontent.com/unicode-org/cldr/refs/tags/release-{}/",
                SourceDataProvider::TESTED_CLDR_TAG
                    .replace(".0", "")
                    .replace(".", "-")
                    .to_ascii_lowercase()
            ))
            .read_to_string(cldr_path)
            .expect(cldr_path),
        )
        .unwrap();
    }

    for (icu_path, repo_path) in [
        (
            "icu4c/source/test/testdata/riwords.txt",
            "components/collator/tests/data/riwords.txt",
        ),
        (
            "icu4c/source/data/unidata/prop_numbers.txt",
            "components/properties/tests/data/prop_numbers.txt",
        ),
        (
            "icu4c/source/test/testdata/CollationTest_SHIFTED_SHORT.txt",
            "components/collator/tests/data/CollationTest_CLDR_SHIFTED_SHORT.txt",
        ),
        (
            "icu4c/source/test/testdata/CollationTest_NON_IGNORABLE_SHORT.txt",
            "components/collator/tests/data/CollationTest_CLDR_NON_IGNORABLE_SHORT.txt",
        ),
    ] {
        std::fs::write(
            repo_root.join(repo_path),
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
