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
    ("collator", icu::collator::provider::KEYS, "1.3.3"),
    (
        "compactdecimal",
        icu_compactdecimal::provider::KEYS,
        "1.3.4",
    ),
    ("datetime", icu::datetime::provider::KEYS, "1.3.4"),
    ("decimal", icu::decimal::provider::KEYS, "1.3.4"),
    ("displaynames", icu::displaynames::provider::KEYS, "1.3.4"),
    ("list", icu::list::provider::KEYS, REPO_VERSION),
    (
        "locid_transform",
        icu::locid_transform::provider::KEYS,
        REPO_VERSION,
    ),
    ("normalizer", icu::normalizer::provider::KEYS, REPO_VERSION),
    ("plurals", icu::plurals::provider::KEYS, REPO_VERSION),
    ("properties", icu::properties::provider::KEYS, "1.3.4"),
    (
        "relativetime",
        icu::relativetime::provider::KEYS,
        REPO_VERSION,
    ),
    ("segmenter", icu::segmenter::provider::KEYS, REPO_VERSION),
    (
        "singlenumberformatter",
        icu_singlenumberformatter::provider::KEYS,
        REPO_VERSION,
    ),
    ("timezone", icu::timezone::provider::KEYS, REPO_VERSION),
    (
        "unitsconversion",
        icu_unitsconversion::provider::KEYS,
        REPO_VERSION,
    ),
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
                    .replace("_version_", version),
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
