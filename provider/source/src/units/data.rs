// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::units::{
    Count, UnitsDisplayNameV1, UnitsDisplayNameV1Marker,
};

use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;
use zerovec::ZeroMap;

impl DataProvider<UnitsDisplayNameV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsDisplayNameV1Marker>, DataError> {
        self.check_req::<UnitsDisplayNameV1Marker>(req)?;

        let (length, unit) = req
            .id
            .marker_attributes
            .split_once('-')
            .ok_or_else(|| DataErrorKind::InvalidRequest.into_error())?;

        let units_format_data: &cldr_serde::units::data::Resource = self
            .cldr()?
            .units()
            .read_and_parse(req.id.locale, "units.json")?;
        let units_format_data = &units_format_data.main.value.units;

        let unit_patterns = match length {
            "long" => &units_format_data.long,
            "short" => &units_format_data.short,
            "narrow" => &units_format_data.narrow,
            _ => {
                return Err(DataErrorKind::InvalidRequest
                    .into_error()
                    .with_debug_context(length))
            }
        }
        .units
        .get(unit)
        .ok_or_else(|| {
            DataErrorKind::InvalidRequest
                .into_error()
                .with_debug_context(length)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(UnitsDisplayNameV1 {
                patterns: ZeroMap::from_iter(
                    [
                        (Count::One, unit_patterns.one.as_deref()),
                        (Count::Two, unit_patterns.two.as_deref()),
                        (Count::Few, unit_patterns.few.as_deref()),
                        (Count::Many, unit_patterns.many.as_deref()),
                        (Count::Other, unit_patterns.other.as_deref()),
                    ]
                    .into_iter()
                    .filter_map(|(count, pattern)| match (count, pattern) {
                        (Count::Other, Some(p)) => Some((count, p)),
                        // As per Unicode TR 35:
                        //      https://www.unicode.org/reports/tr35/tr35-55/tr35.html#Multiple_Inheritance
                        // If the pattern is not found for the associated `Count`, fall back to the `Count::Other` pattern.
                        // Therefore, we filter out any patterns that are the same as the `Count::Other` pattern.
                        (_, p) if p == unit_patterns.other.as_deref() => None,
                        _ => Some((count, pattern?)),
                    }),
                ),
            }),
        })
    }
}

impl crate::IterableDataProviderCached<UnitsDisplayNameV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut data_locales = HashSet::new();

        let numbers = self.cldr()?.numbers();
        let locales = numbers.list_locales()?;
        for locale in locales {
            let units_format_data: &cldr_serde::units::data::Resource =
                self.cldr()?.units().read_and_parse(&locale, "units.json")?;
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

                for unit in length_patterns.units.keys() {
                    data_locales.insert(DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_string(format!("{length}-{unit}")).map_err(
                            |_| {
                                DataError::custom("Failed to parse the attribute")
                                    .with_debug_context(unit)
                            },
                        )?,
                        locale.clone(),
                    ));
                }
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
    let short = units_us_short.patterns.get(&Count::Other).unwrap();
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
    let short = fr_units.patterns.get(&Count::Other).unwrap();
    assert_eq!(short, "{0} m");
}
