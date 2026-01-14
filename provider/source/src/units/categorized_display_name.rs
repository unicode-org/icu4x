// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use cldr_serde::units::preferences::UnitType;
use icu::experimental::dimension::provider::units::categorized_display_names::{
    UnitsNamesAreaCoreV1, UnitsNamesAreaExtendedV1, UnitsNamesAreaOutlierV1,
    UnitsNamesDurationCoreV1, UnitsNamesDurationExtendedV1, UnitsNamesDurationOutlierV1,
    UnitsNamesLengthCoreV1, UnitsNamesLengthExtendedV1, UnitsNamesLengthOutlierV1,
    UnitsNamesMassCoreV1, UnitsNamesMassExtendedV1, UnitsNamesMassOutlierV1,
    UnitsNamesVolumeCoreV1, UnitsNamesVolumeExtendedV1, UnitsNamesVolumeOutlierV1,
};
use icu::experimental::dimension::provider::units::display_names::UnitsDisplayNames;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;
use std::collections::HashSet;

impl SourceDataProvider {
    fn get_display_name_payload<M>(&self, req: DataRequest) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker<DataStruct = UnitsDisplayNames<'static>>,
    {
        let (length, unit) = req
            .id
            .marker_attributes
            .split_once('-')
            .ok_or_else(|| DataErrorKind::InvalidRequest.with_req(M::INFO, req))?;

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
                    .with_req(M::INFO, req)
                    .with_debug_context(length))
            }
        }
        .categories
        .iter()
        .find_map(|(_, units_map)| units_map.get(unit))
        .ok_or_else(|| {
            DataErrorKind::IdentifierNotFound
                .with_req(M::INFO, req)
                .with_debug_context(length)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(UnitsDisplayNames {
                patterns: unit_patterns.try_into_plural_elements_packed_cow()?,
            }),
        })
    }

    fn get_display_name_iter_ids_cached(
        &self,
        unit_type: UnitType,
        category: &str,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut data_locales = HashSet::new();
        let numbers = self.cldr()?.numbers();
        let locales = numbers.list_locales()?;
        for locale in locales {
            // Load and parse the unit constants from the supplemental data file.
            let preferences: &cldr_serde::units::preferences::Resource = self
                .cldr()?
                .core()
                .read_and_parse("supplemental/unitPreferenceData.json")?;

            let categorized_units_list = preferences.supplemental.categorized_units_list();

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

                let units_map = length_patterns.categories.get(category).ok_or_else(|| {
                    DataErrorKind::InvalidRequest
                        .into_error()
                        .with_debug_context(category)
                })?;
                for (unit, patterns) in units_map {
                    if patterns.other.is_none() {
                        continue;
                    }

                    if cldr_serde::units::preferences::Supplemental::unit_type(
                        category,
                        unit,
                        &self.extract_or_infer_region(&locale)?.to_string(),
                        &categorized_units_list,
                    ) != unit_type
                    {
                        continue;
                    }

                    data_locales.insert(DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_string(format!("{length}-{unit}")).map_err(
                            |_| {
                                DataError::custom("Failed to parse the attribute")
                                    .with_debug_context(&unit)
                            },
                        )?,
                        locale,
                    ));
                }
            }
        }

        Ok(data_locales)
    }
}

macro_rules! impl_units_display_names_provider {
    ($marker:ty, $unit_type:expr, $category:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                self.get_display_name_payload::<$marker>(req)
            }
        }

        impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.get_display_name_iter_ids_cached($unit_type, $category)
            }
        }
    };
}

// Area
impl_units_display_names_provider!(UnitsNamesAreaCoreV1, UnitType::Core, "area");
impl_units_display_names_provider!(UnitsNamesAreaExtendedV1, UnitType::Extended, "area");
impl_units_display_names_provider!(UnitsNamesAreaOutlierV1, UnitType::Outlier, "area");

// Duration
impl_units_display_names_provider!(UnitsNamesDurationCoreV1, UnitType::Core, "duration");
impl_units_display_names_provider!(UnitsNamesDurationExtendedV1, UnitType::Extended, "duration");
impl_units_display_names_provider!(UnitsNamesDurationOutlierV1, UnitType::Outlier, "duration");

// Length
impl_units_display_names_provider!(UnitsNamesLengthCoreV1, UnitType::Core, "length");
impl_units_display_names_provider!(UnitsNamesLengthExtendedV1, UnitType::Extended, "length");
impl_units_display_names_provider!(UnitsNamesLengthOutlierV1, UnitType::Outlier, "length");

// Mass
impl_units_display_names_provider!(UnitsNamesMassCoreV1, UnitType::Core, "mass");
impl_units_display_names_provider!(UnitsNamesMassExtendedV1, UnitType::Extended, "mass");
impl_units_display_names_provider!(UnitsNamesMassOutlierV1, UnitType::Outlier, "mass");

// Volume
impl_units_display_names_provider!(UnitsNamesVolumeCoreV1, UnitType::Core, "volume");
impl_units_display_names_provider!(UnitsNamesVolumeExtendedV1, UnitType::Extended, "volume");
impl_units_display_names_provider!(UnitsNamesVolumeOutlierV1, UnitType::Outlier, "volume");

#[test]
fn test_basic() {
    use icu::locale::langid;
    use icu::plurals::PluralRules;
    use icu_provider::prelude::*;
    use writeable::assert_writeable_eq;

    let provider = SourceDataProvider::new_testing();

    let us_locale_long_meter: DataPayload<UnitsNamesLengthExtendedV1> = provider
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

    let en_rules = PluralRules::try_new_cardinal_unstable(&provider, langid!("en").into()).unwrap();
    let long = units_us.patterns.get(1.into(), &en_rules).interpolate([1]);
    assert_writeable_eq!(long, "1 meter");

    let us_locale_short_meter: DataPayload<UnitsNamesLengthExtendedV1> = provider
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

    let ar_eg_locale: DataPayload<UnitsNamesLengthCoreV1> = provider
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
    let ar_rules = PluralRules::try_new_cardinal_unstable(&provider, langid!("ar").into()).unwrap();
    let long = ar_eg_units
        .patterns
        .get(1.into(), &ar_rules)
        .interpolate([1]);
    assert_writeable_eq!(long, "متر");

    let fr_locale: DataPayload<UnitsNamesLengthCoreV1> = provider
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
    let fr_rules = PluralRules::try_new_cardinal_unstable(&provider, langid!("fr").into()).unwrap();
    let short = fr_units.patterns.get(5.into(), &fr_rules).interpolate([5]);
    assert_writeable_eq!(short, "5 m");
}
