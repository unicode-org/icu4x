// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1Marker;

use icu::experimental::dimension::provider::pattern_key::{PatternKey, PowerValue};
use icu::experimental::dimension::provider::units_essentials::CompoundCount;
use icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;
use zerovec::ZeroMap;

impl DataProvider<UnitsEssentialsV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsEssentialsV1Marker>, DataError> {
        self.check_req::<UnitsEssentialsV1Marker>(req)?;

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
            .get("per")
            .and_then(|unit| unit.compound_unit_pattern.as_ref())
            .ok_or_else(|| DataError::custom("Failed to get per"))?
            .clone();

        let times = length_data
            .get("times")
            .and_then(|unit| unit.compound_unit_pattern.as_ref())
            .ok_or_else(|| DataError::custom("Failed to get times"))?
            .clone();

        let mut prefixes = BTreeMap::<PatternKey, String>::new();

        // Fill powers
        for (powers, power_value) in [
            (length_data.get("power2"), PowerValue::Two),
            (length_data.get("power3"), PowerValue::Three),
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

        /// Fills the prefixes map with the SI prefixes (binary and decimal)
        const BINARY_PREFIX: &str = "1024p";
        const DECIMAL_PREFIX: &str = "10p";

        for (key, patterns) in length_data {
            let pattern_key = if let Some(trimmed_key) = key.strip_prefix(BINARY_PREFIX) {
                trimmed_key.parse::<u8>().map(PatternKey::Binary)
            } else if let Some(trimmed_key) = key.strip_prefix(DECIMAL_PREFIX) {
                trimmed_key.parse::<i8>().map(PatternKey::Decimal)
            } else {
                // Skip keys that don't start with the binary or decimal prefixes
                // NOTE:
                //      In case there are other prefixes will be added in the future,
                //      we should update this code to handle them.
                continue;
            }
            .map_err(|_| {
                DataError::custom("Failed to parse pattern key").with_debug_context(&key)
            })?;

            if let Some(pattern) = patterns.unit_prefix_pattern.as_ref() {
                prefixes.insert(pattern_key, pattern.to_string());
            } else {
                return Err(DataError::custom("Failed to get pattern").with_debug_context(&key));
            }
        }

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

impl crate::IterableDataProviderCached<UnitsEssentialsV1Marker> for SourceDataProvider {
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
                .map(move |length| {
                    DataIdentifierCow::from_borrowed_and_owned(length, locale.clone())
                })
            })
            .collect())
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;
    use icu_provider::prelude::*;

    let provider = SourceDataProvider::new_testing();

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
