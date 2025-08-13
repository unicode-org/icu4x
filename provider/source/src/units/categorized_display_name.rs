// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::units::categorized_display_name::{
    UnitsNameAreaV1, UnitsNameDurationV1, UnitsNameLengthV1, UnitsNameMassV1,
    UnitsNameVolumeV1,
};
use icu::experimental::dimension::provider::units::display_name::UnitsDisplayName;
use icu::plurals::PluralElements;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;

macro_rules! impl_categorized_display_name_data_provider {
    ($category:expr, $display_name:ident) => {
        impl DataProvider<$display_name> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$display_name>, DataError> {
                self.check_req::<$display_name>(req)?;

                // This category is based on the Marker
                let category = $category;
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
                            .with_req($display_name::INFO, req)
                            .with_debug_context(length))
                    }
                }
                .categories
                .get(category)
                .ok_or_else(|| {
                    DataErrorKind::IdentifierNotFound
                        .with_req($display_name::INFO, req)
                        .with_debug_context(category)
                })?
                .get(unit)
                .ok_or_else(|| {
                    DataErrorKind::IdentifierNotFound
                        .with_req($display_name::INFO, req)
                        .with_debug_context(length)
                })?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(UnitsDisplayName {
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
    };
}

impl_categorized_display_name_data_provider!("area", UnitsNameAreaV1);
impl_categorized_display_name_data_provider!("duration", UnitsNameDurationV1);
impl_categorized_display_name_data_provider!("length", UnitsNameLengthV1);
impl_categorized_display_name_data_provider!("mass", UnitsNameMassV1);
impl_categorized_display_name_data_provider!("volume", UnitsNameVolumeV1);

macro_rules! impl_categorized_display_name_iter_data_provider_cached {
    ($category:expr, $display_name:ident) => {
        impl crate::IterableDataProviderCached<$display_name> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                // This category is based on the Marker
                let category = $category;

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
                            _ => unreachable!(),
                        };

                        for (unit, patterns) in
                            length_patterns.categories.get(category).ok_or_else(|| {
                                DataErrorKind::IdentifierNotFound
                                    .into_error()
                                    .with_debug_context("length")
                            })?
                        {
                            if patterns.other.is_none() {
                                continue;
                            }
                            data_locales.insert(DataIdentifierCow::from_owned(
                                DataMarkerAttributes::try_from_string(format!("{length}-{unit}"))
                                    .map_err(|_| {
                                    DataError::custom("Failed to parse the attribute")
                                        .with_debug_context(&unit)
                                })?,
                                locale,
                            ));
                        }
                    }
                }

                Ok(data_locales)
            }
        }
    };
}

impl_categorized_display_name_iter_data_provider_cached!("area", UnitsNameAreaV1);
impl_categorized_display_name_iter_data_provider_cached!("duration", UnitsNameDurationV1);
impl_categorized_display_name_iter_data_provider_cached!("length", UnitsNameLengthV1);
impl_categorized_display_name_iter_data_provider_cached!("mass", UnitsNameMassV1);
impl_categorized_display_name_iter_data_provider_cached!("volume", UnitsNameVolumeV1);

#[test]
fn test_categorized_display_name_length() {
    use icu::locale::langid;
    use icu::plurals::PluralRules;
    use icu_provider::prelude::*;
    use writeable::assert_writeable_eq;

    let provider = SourceDataProvider::new_testing();

    let us_locale_long_meter: DataPayload<UnitsNameLengthV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long-meter"),
                &langid!("en").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let length_us = us_locale_long_meter.get().to_owned();

    let en_rules = PluralRules::try_new_cardinal_unstable(&provider, langid!("en").into()).unwrap();
    let long = length_us.patterns.get(1.into(), &en_rules).interpolate([1]);
    assert_writeable_eq!(long, "1 meter");

    let us_locale_short_meter: DataPayload<UnitsNameLengthV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short-meter"),
                &langid!("en").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let length_us_short = us_locale_short_meter.get().to_owned();
    let short = length_us_short
        .patterns
        .get(5.into(), &en_rules)
        .interpolate([5]);
    assert_writeable_eq!(short, "5 m");

    let ar_eg_locale: DataPayload<UnitsNameLengthV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long-meter"),
                &langid!("ar-EG").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let length_ar_eg = ar_eg_locale.get().to_owned();
    let ar_rules = PluralRules::try_new_cardinal_unstable(&provider, langid!("ar").into()).unwrap();
    let long = length_ar_eg
        .patterns
        .get(1.into(), &ar_rules)
        .interpolate([1]);
    assert_writeable_eq!(long, "متر");

    let fr_locale: DataPayload<UnitsNameLengthV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short-meter"),
                &langid!("fr").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let length_fr = fr_locale.get().to_owned();
    let fr_rules = PluralRules::try_new_cardinal_unstable(&provider, langid!("fr").into()).unwrap();
    let short = length_fr.patterns.get(5.into(), &fr_rules).interpolate([5]);
    assert_writeable_eq!(short, "5 m");
}
