// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::cldr_serde::units::data::Patterns;
use crate::cldr_serde::{self};
use crate::DatagenProvider;

use icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1Marker;

use icu::experimental::dimension::provider::pattern_key::{PatternKey, PowerValue};
use icu::experimental::dimension::provider::units_essentials::CompoundCount;
use icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;
use zerovec::ZeroMap;

impl DataProvider<UnitsEssentialsV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsEssentialsV1Marker>, DataError> {
        fn fill_si_binary(
            data: &BTreeMap<String, Patterns>,
            prefixes: &mut BTreeMap<PatternKey, String>,
        ) -> Result<(), DataError> {
            data.iter()
                .filter(|(key, _)| key.starts_with("1024p"))
                .map(|(key, patterns)| {
                    let key = key.trim_start_matches("1024p");
                    match key.parse::<u8>() {
                        Ok(power) => Ok((PatternKey::Binary(power), patterns)),
                        Err(_) => {
                            Err(DataError::custom("Failed to parse power").with_debug_context(key))
                        }
                    }
                })
                .try_for_each(|elem| match elem {
                    Ok((key, patterns)) => {
                        if let Some(pattern) = patterns.unit_prefix_pattern.as_ref() {
                            prefixes.insert(key, pattern.to_string());
                            Ok(())
                        } else {
                            Err(DataError::custom("Failed to get pattern").with_debug_context(&key))
                        }
                    }
                    Err(e) => Err(e),
                })?;

            Ok(())
        }

        fn fill_power(
            prefixes: &mut BTreeMap<PatternKey, String>,
            powers: Option<&Patterns>,
            power_value: PowerValue,
        ) {
            if let Some(powers) = powers {
                let counts = [
                    (
                        powers.zero_compound_unit_pattern1.as_ref(),
                        CompoundCount::Zero,
                    ),
                    (
                        powers.one_compound_unit_pattern1.as_ref(),
                        CompoundCount::One,
                    ),
                    (
                        powers.two_compound_unit_pattern1.as_ref(),
                        CompoundCount::Two,
                    ),
                    (
                        powers.few_compound_unit_pattern1.as_ref(),
                        CompoundCount::Few,
                    ),
                    (
                        powers.many_compound_unit_pattern1.as_ref(),
                        CompoundCount::Many,
                    ),
                    (
                        powers.other_compound_unit_pattern1.as_ref(),
                        CompoundCount::Other,
                    ),
                ];

                for (pattern, count) in counts.iter() {
                    if let Some(pattern) = pattern {
                        prefixes.insert(
                            PatternKey::Power {
                                power: power_value,
                                count: *count,
                            },
                            pattern.to_string(),
                        );
                    }
                }
            }
        }

        self.check_req::<UnitsEssentialsV1Marker>(req)?;

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource = self
            .cldr()?
            .units()
            .read_and_parse(&req.id.locale.get_langid(), "units.json")?;
        let units_format_data = &units_format_data.main.value.units;
        let length_data = match req.id.marker_attributes.as_str() {
            "long" => &units_format_data.long,
            "short" => &units_format_data.short,
            "narrow" => &units_format_data.narrow,
            _ => return Err(DataError::custom("Failed to get length data")),
        };
        let per = length_data
            .get("per")
            .and_then(|unit| unit.compound_unit_pattern.as_ref())
            .ok_or_else(|| DataError::custom("Failed to get per"))?
            .clone();

        let times = length_data
            .get("times")
            .and_then(|unit| unit.compound_unit_pattern.as_ref())
            .ok_or_else(|| DataError::custom("Failed to get times"))?
            .clone();

        // TODO: Fill prefixes (si prefixes (decimal), ... etc.) in the next PR.
        let mut prefixes = BTreeMap::<PatternKey, String>::new();
        let power2 = length_data.get("power2");
        let power3 = length_data.get("power3");
        fill_power(&mut prefixes, power2, PowerValue::Two);
        fill_power(&mut prefixes, power3, PowerValue::Three);

        fill_si_binary(length_data, &mut prefixes)?;

        let result = UnitsEssentialsV1 {
            per: per.into(),
            times: times.into(),
            prefixes: ZeroMap::from_iter(prefixes),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<UnitsEssentialsV1Marker> for DatagenProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let units = self.cldr()?.units();
        let langids = units.list_langs()?;
        Ok(langids
            .into_iter()
            .flat_map(|langid| {
                [
                    DataMarkerAttributes::from_str_or_panic("long"),
                    DataMarkerAttributes::from_str_or_panic("short"),
                    DataMarkerAttributes::from_str_or_panic("narrow"),
                ]
                .into_iter()
                .map(move |length| {
                    DataIdentifierCow::from_borrowed_and_owned(
                        length,
                        DataLocale::from(langid.clone()),
                    )
                })
            })
            .collect())
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;
    use icu_provider::prelude::*;

    let provider = DatagenProvider::new_testing();

    let us_long: DataPayload<UnitsEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long"),
                &langid!("en").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let per = us_long.get().per.to_string();
    assert_eq!(per, "{0} per {1}");

    let times = us_long.get().times.to_string();
    assert_eq!(times, "{0}-{1}");

    let fr_long: DataPayload<UnitsEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long"),
                &langid!("fr").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let per = fr_long.get().per.to_string();
    assert_eq!(per, "{0} par {1}");

    let times = fr_long.get().times.to_string();
    assert_eq!(times, "{0}-{1}");

    let ar_eg_short: DataPayload<UnitsEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short"),
                &langid!("ar").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let per = ar_eg_short.get().per.to_string();
    assert_eq!(per, "{0}/{1}");

    let times = ar_eg_short.get().times.to_string();
    assert_eq!(times, "{0}⋅{1}");
}
