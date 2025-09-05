// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, BTreeSet};

use icu::locale::{
    fallback::{LocaleFallbackConfig, LocaleFallbackPriority, LocaleFallbacker},
    locale, LocaleExpander,
};
use icu_experimental::displaynames::provider::RegionDisplayNamesV1;
use icu_provider::prelude::*;
use icu_provider_source::SourceDataProvider;
use litemap::LiteMap;
use ndarray::{Array2, Axis};
use tinystr::TinyAsciiStr;
use zerotrie::ZeroTrieSimpleAscii;

#[test]
fn dnametest() {
    let provider = SourceDataProvider::new_custom()
        .with_cldr(&std::path::PathBuf::from(
            "/home/sffc/lib/cldr-46.0.0-json-full",
        ))
        .unwrap();

    let locales: BTreeMap<DataIdentifierCow<'_>, usize> =
        IterableDataProvider::<RegionDisplayNamesV1>::iter_ids(&provider)
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();

    let payloads: BTreeMap<DataIdentifierCow, DataPayload<RegionDisplayNamesV1>> = locales
        .keys()
        .map(|locale| {
            let payload = provider
                .load(DataRequest {
                    id: locale.as_borrowed(),
                    ..Default::default()
                })
                .unwrap()
                .payload;
            (locale.clone(), payload)
        })
        .collect();

    let unique_names: Vec<&str> = payloads
        .values()
        .flat_map(|v| v.get().names.iter_values())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let unique_names_required_bits = (unique_names.len() as f64).log2().ceil() as usize;
    println!("unique_names: {} ({unique_names_required_bits})", unique_names.len());

    let regions: BTreeSet<TinyAsciiStr<3>> = payloads
        .get(&DataIdentifierCow::from_locale(locale!("en").into()))
        .unwrap()
        .get()
        .names
        .iter_keys()
        .map(|s| s.try_into_tinystr().unwrap())
        .collect();

    let expander = LocaleExpander::try_new_common_unstable(&provider).unwrap();
    let fallbacker = LocaleFallbacker::try_new_unstable(&provider).unwrap();
    let mut config = LocaleFallbackConfig::default();
    config.priority = LocaleFallbackPriority::Script;
    let fallbacker = fallbacker.for_config(config);

    let script_locales: BTreeMap<DataIdentifierCow, usize> = locales
        .keys()
        .filter_map(|locale| {
            let mut fallback_iterator = fallbacker.fallback_for(locale.locale);
            loop {
                let parent_locale = fallback_iterator.get();
                if parent_locale.is_unknown() {
                    println!("Didn't find script parent for: {:?}", locale.locale);
                    break None;
                }
                if parent_locale.language.is_unknown() && parent_locale.region.is_none() {
                    break Some(DataIdentifierCow::from_locale(*parent_locale));
                }
                fallback_iterator.step();
            }
        })
        .collect::<BTreeSet<_>>() // put in order
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    let mut dense_matrix =
        Array2::<Option<usize>>::default((locales.len() + script_locales.len(), regions.len()));

    for (i, (_locale, payload)) in payloads.iter().enumerate() {
        for (j, region) in regions.iter().enumerate() {
            if let Some(name) = payload.get().names.get(&region.to_unvalidated()) {
                let index = unique_names.binary_search(&name).unwrap();
                dense_matrix[(i, j)] = Some(index);
            }
        }
    }

    for (i, script_locale) in script_locales.keys().enumerate() {
        let i = i + locales.len();
        let mut locale = script_locale.locale.into_locale();
        expander.maximize(&mut locale.id);
        expander.minimize_favor_script(&mut locale.id);
        if let Some(k) = locales.get(&DataIdentifierCow::from_locale((&locale).into())) {
            println!("Copying: {locale:?} to {:?}", script_locale.locale);
            for (j, _region) in regions.iter().enumerate() {
                dense_matrix[(i, j)] = dense_matrix[(*k, j)];
            }
        }
    }

    for (i, (locale, _payload)) in payloads.iter().enumerate() {
        'j: for (j, _region) in regions.iter().enumerate() {
            let Some(value) = dense_matrix[(i, j)] else {
                continue;
            };
            let mut fallback_iterator = fallbacker.fallback_for(locale.locale);
            loop {
                fallback_iterator.step();
                let parent_locale = *fallback_iterator.get();
                if parent_locale.is_unknown() {
                    break;
                }
                if let Some(k) = locales
                    .get(&DataIdentifierCow::from_locale(parent_locale))
                    .copied()
                    .or_else(|| {
                        script_locales
                            .get(&DataIdentifierCow::from_locale(parent_locale))
                            .map(|x| x + locales.len())
                    })
                {
                    if let Some(parent_value) = dense_matrix[(k, j)] {
                        if parent_value == value {
                            dense_matrix[(i, j)] = None;
                        }
                        continue 'j;
                    }
                }
            }
        }
    }

    let large_small = dense_matrix.map_axis(Axis(1), |values| {
        values.iter().filter(|v| v.is_some()).count()
    });

    for (i, locale) in locales.keys().chain(script_locales.keys()).enumerate() {
        println!("{locale:<3}: {}", large_small[i]);
    }

    let locales_only_zerotrie: ZeroTrieSimpleAscii<Vec<u8>> = locales
        .keys()
        .chain(script_locales.keys())
        .enumerate()
        .map(|(i, locale)| (locale.to_string(), i))
        .collect();
    println!("locales_only_zerotrie: {}", locales_only_zerotrie.byte_len());

    let regions_only_zerotrie: ZeroTrieSimpleAscii<Vec<u8>> = regions.iter().enumerate()
        .map(|(i, locale)| (locale.to_string(), i))
        .collect();

    println!("regions_only_zerotrie: {}", regions_only_zerotrie.byte_len());

    let sparse_map: LiteMap<String, usize> = locales
        .keys()
        .chain(script_locales.keys())
        .enumerate()
        .flat_map(|(i, locale)| {
            let dense_matrix = &dense_matrix;
            regions.iter().enumerate().filter_map(move |(j, region)| {
                dense_matrix[(i, j)].map(|index| (format!("{locale}/{region}"), index))
            })
        })
        .collect();
    println!("sparse_map: {}", sparse_map.len());

    let sparse_zerotrie: ZeroTrieSimpleAscii<Vec<u8>> =
        sparse_map.iter().map(|(k, v)| (k, *v)).collect();
    println!("sparse_zerotrie: {}", sparse_zerotrie.byte_len());

    let dense_row_bit_size = regions.len() * unique_names_required_bits;

    let mut num_dense_locales = 0;
    let hybrid_sparse_map: LiteMap<String, usize> = locales
        .keys()
        .chain(script_locales.keys())
        .enumerate()
        .flat_map(|(i, locale)| {
            let dense_matrix = &dense_matrix;
            let row: Vec<(String, usize)> = regions.iter().enumerate().filter_map(move |(j, region)| {
                dense_matrix[(i, j)].map(|index| (format!("{locale}/{region}"), index))
            }).collect();
            let inner_zerotrie: ZeroTrieSimpleAscii<_> = row.iter().map(|(k, v)| (k, *v)).collect();
            if inner_zerotrie.byte_len() * 8 > dense_row_bit_size {
                num_dense_locales += 1;
                vec![(locale.to_string(), 0)].into_iter()
            } else {
                row.into_iter()
            }
        })
        .collect();
    println!("hybrid_sparse_map: {}", hybrid_sparse_map.len());
    println!("num_dense_locales: {} ({} B)", num_dense_locales, num_dense_locales * dense_row_bit_size / 8);

    let hybrid_sparse_zerotrie: ZeroTrieSimpleAscii<Vec<u8>> =
        hybrid_sparse_map.iter().map(|(k, v)| (k, *v)).collect();
    println!("hybrid_sparse_zerotrie: {}", hybrid_sparse_zerotrie.byte_len());
}
