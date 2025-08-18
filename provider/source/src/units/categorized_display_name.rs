// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use cldr_serde::units::preferences::UnitType;
use icu::experimental::dimension::provider::units::categorized_display_name::{
    UnitsNameLengthCoreV1, UnitsNameMassCoreV1, UnitsNameLengthExtendedV1, UnitsNameMassExtendedV1,
    UnitsNameLengthOutlierV1, UnitsNameMassOutlierV1,
};
use icu::experimental::dimension::provider::units::display_name::UnitsDisplayName;
use icu::plurals::PluralElements;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;

fn get_display_name_payload<M>(
    source_data_provider: &SourceDataProvider,
    req: DataRequest,
) -> Result<DataResponse<M>, DataError>
where
    M: DataMarker<DataStruct = UnitsDisplayName<'static>>,
{
    let (length, unit) = req
        .id
        .marker_attributes
        .split_once('-')
        .ok_or_else(|| DataErrorKind::InvalidRequest.into_error())?;

    let units_format_data: &cldr_serde::units::data::Resource = source_data_provider
        .cldr()?
        .units()
        .read_and_parse(&req.id.locale, "units.json")?;

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
    .categories
    .iter()
    .find_map(|(_, units_map)| units_map.get(unit))
    .ok_or_else(|| {
        DataErrorKind::IdentifierNotFound
            .into_error()
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

fn get_display_name_iter_ids_cached(
    source_data_provider: &SourceDataProvider,
    unit_type: UnitType,
    category: &str,
) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
    let mut data_locales = HashSet::new();
    let numbers = source_data_provider.cldr()?.numbers();
    let locales = numbers.list_locales()?;
    for locale in locales {
        let region = match locale.region {
            Some(region) => region.to_string(),
            None => "US".to_string(),
        };

        // Load and parse the unit constants from the supplemental data file.
        let preferences: &cldr_serde::units::preferences::Resource = source_data_provider
            .cldr()?
            .core()
            .read_and_parse("supplemental/unitPreferenceData.json")?;

        let categorized_units_list = preferences.supplemental.categorized_units_list();

        let units_format_data: &cldr_serde::units::data::Resource = source_data_provider
            .cldr()?
            .units()
            .read_and_parse(&locale, "units.json")?;
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

            let units_map = length_patterns
                .categories
                .get(category)
                .ok_or(DataError::custom("Category not found"))?;
            for (unit, patterns) in units_map {
                if patterns.other.is_none() {
                    continue;
                }

                if cldr_serde::units::preferences::Supplemental::unit_type(
                    category,
                    unit,
                    &region,
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

macro_rules! impl_units_display_name_provider {
    ($marker:ty, $unit_type:expr, $category:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                get_display_name_payload::<$marker>(self, req)
            }
        }

        impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                get_display_name_iter_ids_cached(self, $unit_type, $category)
            }
        }
    };
}

impl_units_display_name_provider!(UnitsNameLengthCoreV1, UnitType::Core, "length");
impl_units_display_name_provider!(UnitsNameLengthExtendedV1, UnitType::Extended, "length");
impl_units_display_name_provider!(UnitsNameLengthOutlierV1, UnitType::Outlier, "length");
impl_units_display_name_provider!(UnitsNameMassCoreV1, UnitType::Core, "mass");
impl_units_display_name_provider!(UnitsNameMassExtendedV1, UnitType::Extended, "mass");
impl_units_display_name_provider!(UnitsNameMassOutlierV1, UnitType::Outlier, "mass");
