// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate icu_datagen;

use icu_datagen::baked_exporter::*;
use icu_datagen::prelude::*;
use std::path::Path;

const COMPONENTS: &[(&str, &[DataKey])] = &[
    ("calendar", icu::calendar::provider::KEYS),
    ("casemap", icu::casemap::provider::KEYS),
    ("collator", icu::collator::provider::KEYS),
    ("compactdecimal", icu_compactdecimal::provider::KEYS),
    ("datetime", icu::datetime::provider::KEYS),
    ("decimal", icu::decimal::provider::KEYS),
    ("displaynames", icu::displaynames::provider::KEYS),
    ("list", icu::list::provider::KEYS),
    ("locid_transform", icu::locid_transform::provider::KEYS),
    ("normalizer", icu::normalizer::provider::KEYS),
    ("plurals", icu::plurals::provider::KEYS),
    ("properties", icu::properties::provider::KEYS),
    ("relativetime", icu::relativetime::provider::KEYS),
    ("segmenter", icu::segmenter::provider::KEYS),
    (
        "singlenumberformatter",
        icu_singlenumberformatter::provider::KEYS,
    ),
    ("timezone", icu::timezone::provider::KEYS),
    ("unitsconversion", icu_unitsconversion::provider::KEYS),
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
            .map(|(k, v)| (k.to_string(), *v))
            .collect::<Vec<_>>()
    } else {
        let map = std::collections::HashMap::<&str, &'static [DataKey]>::from_iter(
            COMPONENTS.iter().copied(),
        );
        args.skip(1)
            .filter_map(|arg| {
                map.get(arg.strip_suffix('/').unwrap_or(arg.as_str()))
                    .map(|k| (arg, *k))
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

    for (component, keys) in &components {
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
                    .replace("_component_", component),
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
