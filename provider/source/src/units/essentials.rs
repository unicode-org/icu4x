// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::units::essentials::CompoundCount;
use icu::experimental::dimension::provider::units::essentials::UnitsEssentials;
use icu::experimental::dimension::provider::units::essentials::UnitsEssentialsV1;
use icu::experimental::dimension::provider::units::pattern_key::{PatternKey, PowerValue};
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;
use zerovec::ZeroMap;

impl DataProvider<UnitsEssentialsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsEssentialsV1>, DataError> {
        self.check_req::<UnitsEssentialsV1>(req)?;

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource = self
            .cldr()?
            .units()
            .read_and_parse(req.id.locale, "units.json")?;
        let units_format_data = &units_format_data.main.value.units;
        let length_data = match req.id.marker_attributes.as_str() {
            "long" => &units_format_data.long,
            "short" => &units_format_data.short,
            "narrow" => &units_format_data.narrow,
            _ => return Err(DataError::custom("Failed to get length data")),
        };
        let per = length_data
            .per
            .compound_unit_pattern
            .as_ref()
            .ok_or_else(|| DataError::custom("Failed to get per"))?
            .clone();

        let times = length_data
            .times
            .compound_unit_pattern
            .as_ref()
            .ok_or_else(|| DataError::custom("Failed to get times"))?
            .clone();

        let mut prefixes = BTreeMap::<PatternKey, String>::new();

        // Fill powers
        for (powers, power_value) in [
            (length_data.powers.get(&2), PowerValue::Two),
            (length_data.powers.get(&3), PowerValue::Three),
        ] {
            let powers = powers
                .as_ref()
                .ok_or_else(|| DataError::custom("Failed to get powers"))?;
            [
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
            ]
            .iter()
            .filter(|(pattern, _)| pattern.is_some())
            .map(|(pattern, count)| (pattern.unwrap(), count))
            .for_each(|(pattern, count)| {
                prefixes.insert(
                    PatternKey::Power {
                        power: power_value,
                        count: *count,
                    },
                    pattern.to_string(),
                );
            });
        }

        for (power, patterns) in &length_data.binary {
            prefixes.insert(
                PatternKey::Binary(*power),
                patterns
                    .unit_prefix_pattern
                    .as_ref()
                    .ok_or_else(|| DataError::custom("Failed to get binary pattern"))?
                    .clone(),
            );
        }
        for (power, patterns) in &length_data.decimal {
            prefixes.insert(
                PatternKey::Decimal(*power),
                patterns
                    .unit_prefix_pattern
                    .as_ref()
                    .ok_or_else(|| DataError::custom("Failed to get decimal pattern"))?
                    .clone(),
            );
        }

        let result = UnitsEssentials {
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

impl crate::IterableDataProviderCached<UnitsEssentialsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let units = self.cldr()?.units();
        let locales = units.list_locales()?;
        Ok(locales
            .into_iter()
            .flat_map(|locale| {
                [
                    DataMarkerAttributes::from_str_or_panic("long"),
                    DataMarkerAttributes::from_str_or_panic("short"),
                    DataMarkerAttributes::from_str_or_panic("narrow"),
                ]
                .into_iter()
                .map(move |length| DataIdentifierCow::from_borrowed_and_owned(length, locale))
            })
            .collect())
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;
    use icu_provider::prelude::*;

    let provider = SourceDataProvider::new_testing();

    let us_long: DataPayload<UnitsEssentialsV1> = provider
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

    let fr_long: DataPayload<UnitsEssentialsV1> = provider
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

    let ar_eg_short: DataPayload<UnitsEssentialsV1> = provider
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
