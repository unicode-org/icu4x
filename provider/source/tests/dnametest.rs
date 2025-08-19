// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, BTreeSet};

use icu_experimental::displaynames::provider::RegionDisplayNamesV1;
use icu_provider_source::SourceDataProvider;
use icu_provider::prelude::*;
use icu::locale::{fallback::{LocaleFallbackConfig, LocaleFallbackPriority, LocaleFallbacker}, locale};
use ndarray::{Array2, Axis};
use tinystr::TinyAsciiStr;

#[test]
fn dnametest() {
    let provider = SourceDataProvider::new();

    let locales: BTreeMap<DataIdentifierCow<'_>, usize> = IterableDataProvider::<RegionDisplayNamesV1>::iter_ids(&provider).unwrap().into_iter().enumerate().map(|(i, v)| (v, i)).collect();

    let payloads: BTreeMap<DataIdentifierCow, DataPayload<RegionDisplayNamesV1>> = locales.keys().map(|locale| {
        let payload = provider.load(DataRequest {
            id: locale.as_borrowed(),
            ..Default::default()
        })
        .unwrap()
        .payload;
    (locale.clone(), payload)
    }).collect();

    let en_names = payloads.get(&DataIdentifierCow::from_locale(locale!("en").into())).unwrap();

    let regions = en_names.get().names.iter_keys().map(|s| s.try_into_tinystr().unwrap()).collect::<BTreeSet<TinyAsciiStr<3>>>();

    let mut dense_matrix = Array2::<Option<&str>>::default((locales.len(), regions.len()));

    for (i, (_locale, payload)) in payloads.iter().enumerate() {
        for (j, region) in regions.iter().enumerate() {
            dense_matrix[(i, j)] = payload.get().names.get(&region.to_unvalidated());
        }
    }

    let fallbacker = LocaleFallbacker::try_new_unstable(&provider).unwrap();
    let mut config = LocaleFallbackConfig::default();
    config.priority = LocaleFallbackPriority::Script;
    let fallbacker = fallbacker.for_config(config);

    for (i, (locale, _payload)) in payloads.iter().enumerate() {
        'j: for (j, _region) in regions.iter().enumerate() {
            let Some(value) = dense_matrix[(i, j)] else {
                continue
            };
            let mut fallback_iterator = fallbacker.fallback_for(locale.locale);
            loop {
                fallback_iterator.step();
                let parent_locale = *fallback_iterator.get();
                if parent_locale.is_unknown() {
                    break;
                }
                if let Some(k) = locales.get(&DataIdentifierCow::from_locale(parent_locale)) {
                    if let Some(parent_value) = dense_matrix[(*k, j)] {
                        if parent_value == value {
                            dense_matrix[(i, j)] = None;
                        }
                        continue 'j;
                    }
                }
            }
        }
    }

    let large_small = dense_matrix.map_axis(Axis(1), |values| values.iter().filter(|v| v.is_some()).count());

    for (i, locale) in locales.keys().enumerate() {
        println!("{locale:<3}: {}", large_small[i]);
    }
}
