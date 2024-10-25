// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::units::{UnitsDisplayNameV1, UnitsDisplayNameV1Marker};
use icu::plurals::PluralElements;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;

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
            DataErrorKind::IdentifierNotFound
                .into_error()
                .with_debug_context(length)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(UnitsDisplayNameV1 {
                patterns: PluralElements::new(
                    unit_patterns
                        .other
                        .as_deref()
                        .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?,
                )
                .with_zero_value(unit_patterns.zero.as_deref())
                .with_one_value(unit_patterns.one.as_deref())
                .with_two_value(unit_patterns.two.as_deref())
                .with_few_value(unit_patterns.few.as_deref())
                .with_many_value(unit_patterns.many.as_deref())
                .with_explicit_one_value(unit_patterns.explicit_one.as_deref())
                .with_explicit_zero_value(unit_patterns.explicit_zero.as_deref())
                .into(),
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

                for (unit, patterns) in &length_patterns.units {
                    if patterns.other.is_none() {
                        continue;
                    }
                    data_locales.insert(DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_string(format!("{length}-{unit}")).map_err(
                            |_| {
                                DataError::custom("Failed to parse the attribute")
                                    .with_debug_context(&unit)
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
    use icu::plurals::PluralRules;
    use icu_provider::prelude::*;
    use writeable::assert_writeable_eq;

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

    let en_rules =
        PluralRules::try_new_cardinal_unstable(&provider, &langid!("en").into()).unwrap();
    let long = units_us.patterns.get(1.into(), &en_rules).interpolate([1]);
    assert_writeable_eq!(long, "1 meter");

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
    let short = units_us_short
        .patterns
        .get(5.into(), &en_rules)
        .interpolate([5]);
    assert_writeable_eq!(short, "5 m");

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
    let ar_rules =
        PluralRules::try_new_cardinal_unstable(&provider, &langid!("ar").into()).unwrap();
    let long = ar_eg_units
        .patterns
        .get(1.into(), &ar_rules)
        .interpolate([1]);
    assert_writeable_eq!(long, "متر");

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
    let fr_rules =
        PluralRules::try_new_cardinal_unstable(&provider, &langid!("fr").into()).unwrap();
    let short = fr_units.patterns.get(5.into(), &fr_rules).interpolate([5]);
    assert_writeable_eq!(short, "5 m");
}
