// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use crate::cldr_serde;
use cldr_serde::units::preferences::UnitType;
use icu::experimental::dimension::provider::units::categorized_display_names::{
    UnitsNamesAreaHeavyV1, UnitsNamesAreaLightV1, UnitsNamesAreaTinyV1, UnitsNamesDurationHeavyV1,
    UnitsNamesDurationLightV1, UnitsNamesDurationTinyV1, UnitsNamesLengthHeavyV1,
    UnitsNamesLengthLightV1, UnitsNamesLengthTinyV1, UnitsNamesMassHeavyV1, UnitsNamesMassLightV1,
    UnitsNamesMassTinyV1, UnitsNamesVolumeHeavyV1, UnitsNamesVolumeLightV1, UnitsNamesVolumeTinyV1,
};
use icu::experimental::dimension::provider::units::display_names::UnitsDisplayNames;
use icu_provider::DataMarkerAttributes;
use icu_provider::prelude::*;
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
                    .with_debug_context(length));
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
            payload: DataPayload::from_owned(unit_patterns.try_into_plural_elements_packed_cow()?),
        })
    }

    fn get_display_name_iter_ids_cached(
        &self,
        unit_type: UnitType,
        category: &str,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut data_locales = HashSet::new();
        let units = self.cldr()?.units();
        let locales = units.list_locales()?;
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
                            .with_debug_context(length));
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
                        &self.cldr()?.extract_or_infer_region(&locale)?.to_string(),
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
impl_units_display_names_provider!(UnitsNamesAreaTinyV1, UnitType::Tiny, "area");
impl_units_display_names_provider!(UnitsNamesAreaLightV1, UnitType::Light, "area");
impl_units_display_names_provider!(UnitsNamesAreaHeavyV1, UnitType::Heavy, "area");

// Duration
impl_units_display_names_provider!(UnitsNamesDurationTinyV1, UnitType::Tiny, "duration");
impl_units_display_names_provider!(UnitsNamesDurationLightV1, UnitType::Light, "duration");
impl_units_display_names_provider!(UnitsNamesDurationHeavyV1, UnitType::Heavy, "duration");

// Length
impl_units_display_names_provider!(UnitsNamesLengthTinyV1, UnitType::Tiny, "length");
impl_units_display_names_provider!(UnitsNamesLengthLightV1, UnitType::Light, "length");
impl_units_display_names_provider!(UnitsNamesLengthHeavyV1, UnitType::Heavy, "length");

// Mass
impl_units_display_names_provider!(UnitsNamesMassTinyV1, UnitType::Tiny, "mass");
impl_units_display_names_provider!(UnitsNamesMassLightV1, UnitType::Light, "mass");
impl_units_display_names_provider!(UnitsNamesMassHeavyV1, UnitType::Heavy, "mass");

// Volume
impl_units_display_names_provider!(UnitsNamesVolumeTinyV1, UnitType::Tiny, "volume");
impl_units_display_names_provider!(UnitsNamesVolumeLightV1, UnitType::Light, "volume");
impl_units_display_names_provider!(UnitsNamesVolumeHeavyV1, UnitType::Heavy, "volume");

#[test]
fn test_basic() {
    use icu::locale::{data_locale, locale};
    use icu::plurals::PluralRules;
    use icu_provider::prelude::*;
    use writeable::assert_writeable_eq;

    let provider = SourceDataProvider::new_testing();

    let us_locale_long_meter: DataPayload<UnitsNamesLengthLightV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long-meter"),
                &data_locale!("en"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let en_rules = PluralRules::try_new_cardinal_unstable(&provider, locale!("en").into()).unwrap();
    let long = us_locale_long_meter
        .get()
        .get(1.into(), &en_rules)
        .interpolate([1]);
    assert_writeable_eq!(long, "1 meter");

    let us_locale_short_meter: DataPayload<UnitsNamesLengthLightV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short-meter"),
                &data_locale!("en"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let short = us_locale_short_meter
        .get()
        .get(5.into(), &en_rules)
        .interpolate([5]);
    assert_writeable_eq!(short, "5 m");

    let ar_eg_locale: DataPayload<UnitsNamesLengthTinyV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long-meter"),
                &data_locale!("ar-EG"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let ar_rules = PluralRules::try_new_cardinal_unstable(&provider, locale!("ar").into()).unwrap();
    let long = ar_eg_locale.get().get(1.into(), &ar_rules).interpolate([1]);
    assert_writeable_eq!(long, "متر");

    let fr_locale: DataPayload<UnitsNamesLengthTinyV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short-meter"),
                &data_locale!("fr"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let fr_rules = PluralRules::try_new_cardinal_unstable(&provider, locale!("fr").into()).unwrap();
    let short = fr_locale.get().get(5.into(), &fr_rules).interpolate([5]);
    assert_writeable_eq!(short, "5 m");
}
