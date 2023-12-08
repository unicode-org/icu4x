// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate icu_datagen;

use icu_datagen::baked_exporter::*;
use icu_datagen::prelude::*;
use std::path::Path;

const REPO_VERSION: &str = env!("CARGO_PKG_VERSION");

const COMPONENTS: &[(&str, &[DataKey], &str)] = &[
    ("calendar", icu::calendar::provider::KEYS, REPO_VERSION),
    ("casemap", icu::casemap::provider::KEYS, REPO_VERSION),
    ("collator", icu::collator::provider::KEYS, REPO_VERSION),
    (
        "compactdecimal",
        icu::compactdecimal::provider::KEYS,
        REPO_VERSION,
    ),
    ("datetime", icu::datetime::provider::KEYS, REPO_VERSION),
    ("decimal", icu::decimal::provider::KEYS, REPO_VERSION),
    (
        "displaynames",
        icu::displaynames::provider::KEYS,
        REPO_VERSION,
    ),
    ("list", icu::list::provider::KEYS, REPO_VERSION),
    (
        "locid_transform",
        icu::locid_transform::provider::KEYS,
        REPO_VERSION,
    ),
    ("normalizer", icu::normalizer::provider::KEYS, REPO_VERSION),
    ("plurals", icu::plurals::provider::KEYS, REPO_VERSION),
    ("properties", icu::properties::provider::KEYS, REPO_VERSION),
    (
        "relativetime",
        icu::relativetime::provider::KEYS,
        REPO_VERSION,
    ),
    ("segmenter", icu::segmenter::provider::KEYS, REPO_VERSION),
    ("timezone", icu::timezone::provider::KEYS, REPO_VERSION),
];

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let args = std::env::args();

    let components = if args.len() == 1 {
        COMPONENTS
            .iter()
            .map(|(krate, keys, version)| (krate.to_string(), *keys, *version))
            .collect::<Vec<_>>()
    } else {
        let map = std::collections::HashMap::<&str, (&'static [DataKey], &'static str)>::from_iter(
            COMPONENTS
                .iter()
                .map(|(krate, keys, version)| (*krate, (*keys, *version))),
        );
        args.skip(1)
            .filter_map(|krate| {
                map.get(krate.as_str())
                    .map(|(keys, version)| (krate, *keys, *version))
            })
            .collect()
    };

    let source = DatagenProvider::new_latest_tested();

    let driver = DatagenDriver::new()
        .with_locales(
            source
                .locales_for_coverage_levels([
                    CoverageLevel::Modern,
                    CoverageLevel::Moderate,
                    CoverageLevel::Basic,
                ])
                .unwrap(),
        )
        .with_recommended_segmenter_models()
        .with_fallback_mode(FallbackMode::Runtime);

    let mut options = Options::default();
    options.overwrite = true;
    options.pretty = true;

    let template = Path::new("provider/baked/_template_");

    for (component, keys, version) in &components {
        let path = Path::new("provider/baked").join(component);

        let _ = std::fs::remove_dir_all(&path);
        for dir in ["", "src", "data"] {
            std::fs::create_dir(&path.join(dir)).unwrap();
        }
        for file in [
            "build.rs",
            "Cargo.toml",
            "LICENSE",
            "README.md",
            "src/lib.rs",
        ] {
            std::fs::write(
                path.join(file),
                &std::fs::read_to_string(template.join(file))
                    .unwrap()
                    .replace("_component_", component)
                    .replace("_version_", version)
                    .replace("_cldr_tag_", DatagenProvider::LATEST_TESTED_CLDR_TAG)
                    .replace(
                        "_icuexport_tag_",
                        DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG,
                    )
                    .replace(
                        "_segmenter_lstm_tag_",
                        DatagenProvider::LATEST_TESTED_SEGMENTER_LSTM_TAG,
                    ),
            )
            .unwrap();
        }

        if component == "segmenter" {
            // segmenter uses hardcoded locales internally, so fallback is not necessary.
            driver.clone().with_fallback_mode(FallbackMode::Hybrid)
        } else {
            driver.clone()
        }
        .with_keys(keys.iter().copied())
        .export(
            &source,
            BakedExporter::new(path.join("data"), options).unwrap(),
        )
        .unwrap();

        for file in ["data/any.rs", "data/mod.rs"] {
            std::fs::remove_file(path.join(file)).unwrap();
        }
    }
}
