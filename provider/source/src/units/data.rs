// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::cldr_serde::units::data::Patterns;
use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::units::{
    Count, UnitsDisplayNameV1, UnitsDisplayNameV1Marker,
};

use icu::locale::LanguageIdentifier;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;
use zerovec::ZeroMap;

impl DataProvider<UnitsDisplayNameV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsDisplayNameV1Marker>, DataError> {
        self.check_req::<UnitsDisplayNameV1Marker>(req)?;

        let langid = req.id.locale.get_langid();
        let (length, unit) = req
            .id
            .marker_attributes
            .split_once('-')
            .ok_or_else(|| DataErrorKind::InvalidRequest.into_error())?;

        let units_format_data: &cldr_serde::units::data::Resource =
            self.cldr()?.units().read_and_parse(&langid, "units.json")?;
        let units_format_data = &units_format_data.main.value.units;

        fn insert_unit_with_name(
            map: &mut BTreeMap<Count, String>,
            count: Count,
            unit: Option<&str>,
        ) {
            if let Some(unit) = unit {
                map.insert(count, unit.to_string());
            }
        }

        fn populate_units_map(
            unit_length_map: &BTreeMap<String, Patterns>,
            unit: &str,
        ) -> Result<BTreeMap<Count, String>, DataError> {
            let mut result = BTreeMap::new();
            // TODO(younies): this should be coming from the aux key or from the main key.
            let legth_key = "length-".to_string() + unit;
            let duration_key = "duration-".to_string() + unit;

            let unit_length_map = match (
                unit_length_map.get(&legth_key),
                unit_length_map.get(&duration_key),
            ) {
                (Some(length), None) => length,
                (None, Some(length)) => length,
                _ => {
                    return Err(DataErrorKind::IdentifierNotFound
                        .into_error()
                        .with_debug_context(unit))
                }
            };

            insert_unit_with_name(&mut result, Count::One, unit_length_map.one.as_deref());
            insert_unit_with_name(&mut result, Count::Two, unit_length_map.two.as_deref());
            insert_unit_with_name(&mut result, Count::Few, unit_length_map.few.as_deref());
            insert_unit_with_name(&mut result, Count::Many, unit_length_map.many.as_deref());
            insert_unit_with_name(&mut result, Count::Other, unit_length_map.other.as_deref());

            Ok(result)
        }

        let result = UnitsDisplayNameV1 {
            patterns: ZeroMap::from_iter(
                populate_units_map(
                    match length {
                        "long" => &units_format_data.long,
                        "short" => &units_format_data.short,
                        "narrow" => &units_format_data.narrow,
                        _ => {
                            return Err(DataErrorKind::IdentifierNotFound
                                .into_error()
                                .with_debug_context(length))
                        }
                    },
                    unit,
                )?
                .iter()
                .map(|(k, v)| (k, v.as_str())),
            ),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<UnitsDisplayNameV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        fn make_request_element(
            langid: &LanguageIdentifier,
            unit: &str,
            length: &str,
        ) -> Result<DataIdentifierCow<'static>, DataError> {
            let data_locale = DataLocale::from(langid);
            let key = length.to_string() + "-" + unit;
            let attribute = DataMarkerAttributes::try_from_str(key.as_str()).map_err(|_| {
                DataError::custom("Failed to parse the attribute").with_debug_context(unit)
            })?;
            Ok(DataIdentifierCow::from_owned(
                attribute.to_owned(),
                data_locale,
            ))
        }

        fn fill_data_ids(
            data_locales: &mut HashSet<DataIdentifierCow<'_>>,
            langid: &LanguageIdentifier,
            length: &str,
            length_patterns: &BTreeMap<String, Patterns>,
        ) -> Result<(), DataError> {
            let quantities = length_patterns
                .keys()
                // TODO: remove this filter once we are supporting all the units categories.
                // NOTE:
                //  if this filter is removed, we have to add a filter to remove all the prefixes.
                .filter_map(|key| {
                    if let Some(rest) = key.strip_prefix("length-") {
                        Some(rest)
                    } else if let Some(rest) = key.strip_prefix("duration-") {
                        Some(rest)
                    } else {
                        None
                    }
                });

            for truncated_quantity in quantities {
                data_locales.insert(make_request_element(langid, truncated_quantity, length)?);
            }

            Ok(())
        }

        let mut data_locales = HashSet::new();

        let numbers = self.cldr()?.numbers();
        let langids = numbers.list_langs()?;
        for langid in langids {
            let units_format_data: &cldr_serde::units::data::Resource =
                self.cldr()?.units().read_and_parse(&langid, "units.json")?;
            let units_format_data = &units_format_data.main.value.units;

            for length in &["long", "short", "narrow"] {
                let length_patterns = match *length {
                    "long" => &units_format_data.long,
                    "short" => &units_format_data.short,
                    "narrow" => &units_format_data.narrow,
                    _ => {
                        return Err(DataErrorKind::IdentifierNotFound
                            .into_error()
                            .with_debug_context(length))
                    }
                };

                fill_data_ids(&mut data_locales, &langid, length, length_patterns)?;
            }
        }

        Ok(data_locales)
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;
    use icu_provider::prelude::*;

    let provider = SourceDataProvider::new_testing();

    let us_locale_long_meter: DataPayload<UnitsDisplayNameV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long-meter"),
                &langid!("en").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let units_us = us_locale_long_meter.get().to_owned();
    let long = units_us.patterns.get(&Count::One).unwrap();
    assert_eq!(long, "{0} meter");

    let us_locale_short_meter: DataPayload<UnitsDisplayNameV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short-meter"),
                &langid!("en").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let units_us_short = us_locale_short_meter.get().to_owned();
    let short = units_us_short.patterns.get(&Count::One).unwrap();
    assert_eq!(short, "{0} m");

    let ar_eg_locale: DataPayload<UnitsDisplayNameV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long-meter"),
                &langid!("ar-EG").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let ar_eg_units = ar_eg_locale.get().to_owned();
    let long = ar_eg_units.patterns.get(&Count::One).unwrap();
    assert_eq!(long, "متر");

    let fr_locale: DataPayload<UnitsDisplayNameV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short-meter"),
                &langid!("fr").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let fr_units = fr_locale.get().to_owned();
    let short = fr_units.patterns.get(&Count::One).unwrap();
    assert_eq!(short, "{0} m");
}
