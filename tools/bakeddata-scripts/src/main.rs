// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate icu_datagen;

use icu_datagen::baked_exporter::*;
use icu_datagen::prelude::*;
use std::path::Path;

const COMPONENTS: &[(&str, &[DataKey])] = &[
    ("components/calendar", icu::calendar::provider::KEYS),
    ("components/casemap", icu::casemap::provider::KEYS),
    ("components/collator", icu::collator::provider::KEYS),
    ("components/datetime", icu::datetime::provider::KEYS),
    ("components/decimal", icu::decimal::provider::KEYS),
    ("components/list", icu::list::provider::KEYS),
    (
        "components/locid_transform",
        icu::locid_transform::provider::KEYS,
    ),
    ("components/normalizer", icu::normalizer::provider::KEYS),
    ("components/plurals", icu::plurals::provider::KEYS),
    ("components/properties", icu::properties::provider::KEYS),
    ("components/segmenter", icu::segmenter::provider::KEYS),
    ("components/timezone", icu::timezone::provider::KEYS),
    (
        "experimental/compactdecimal",
        icu_compactdecimal::provider::KEYS,
    ),
    (
        "experimental/displaynames",
        icu::displaynames::provider::KEYS,
    ),
    (
        "experimental/personnames",
        icu::personnames::provider::KEYS,
    ),
    (
        "experimental/relativetime",
        icu::relativetime::provider::KEYS,
    ),
    (
        "experimental/single_number_formatter",
        icu_singlenumberformatter::provider::KEYS,
    ),
    (
        "experimental/unitsconversion",
        icu_unitsconversion::provider::KEYS,
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

    for (path, keys) in components {
        if path == "components/segmenter" {
            // segmenter uses hardcoded locales internally, so fallback is not necessary.
            driver.clone().with_fallback_mode(FallbackMode::Hybrid)
        } else {
            driver.clone()
        }
        .with_keys(keys.iter().copied())
        .export(
            &source,
            BakedExporter::new(Path::new(&path).join("data/data"), options).unwrap(),
        )
        .unwrap();
    }
}
