// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use cldr_serde::units::preferences::UnitType;
use icu::experimental::dimension::provider::units::categorized_display_name::{
    UnitsNameAreaCoreV1, UnitsNameAreaExtendedV1, UnitsNameAreaOutlierV1, UnitsNameDurationCoreV1,
    UnitsNameDurationExtendedV1, UnitsNameDurationOutlierV1, UnitsNameLengthCoreV1,
    UnitsNameLengthExtendedV1, UnitsNameLengthOutlierV1, UnitsNameMassCoreV1,
    UnitsNameMassExtendedV1, UnitsNameMassOutlierV1, UnitsNameVolumeCoreV1,
    UnitsNameVolumeExtendedV1, UnitsNameVolumeOutlierV1,
};
use icu::experimental::dimension::provider::units::display_name::UnitsDisplayName;
use icu::locale::LanguageIdentifier;
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
        .ok_or_else(|| DataErrorKind::InvalidRequest.with_req(M::INFO, req))?;

    let units_format_data: &cldr_serde::units::data::Resource = source_data_provider
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
        payload: DataPayload::from_owned(UnitsDisplayName {
            patterns: unit_patterns
                .try_into()
                .map_err(|e: DataError| e.with_req(M::INFO, req))?,
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
        let likely_subtags: &cldr_serde::likely_subtags::Resource = source_data_provider
            .cldr()?
            .core()
            .read_and_parse("supplemental/likelySubtags.json")?;

        // Try to find a likely subtag for the language, and extract its region.
        let region = locale
            .region
            .map(|r| r.to_string())
            .or_else(|| {
                let lang_id = LanguageIdentifier::from(locale.language);
                likely_subtags
                    .supplemental
                    .likely_subtags
                    .get(&lang_id)
                    .and_then(|id| id.region.map(|r| r.to_string()))
            })
            .ok_or_else(|| {
                let lang_id = LanguageIdentifier::from(locale.language);
                DataError::custom("No region found for language: {}").with_debug_context(&lang_id)
            })?;

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

// Area
impl_units_display_name_provider!(UnitsNameAreaCoreV1, UnitType::Core, "area");
impl_units_display_name_provider!(UnitsNameAreaExtendedV1, UnitType::Extended, "area");
impl_units_display_name_provider!(UnitsNameAreaOutlierV1, UnitType::Outlier, "area");

// Duration
impl_units_display_name_provider!(UnitsNameDurationCoreV1, UnitType::Core, "duration");
impl_units_display_name_provider!(UnitsNameDurationExtendedV1, UnitType::Extended, "duration");
impl_units_display_name_provider!(UnitsNameDurationOutlierV1, UnitType::Outlier, "duration");

// Length
impl_units_display_name_provider!(UnitsNameLengthCoreV1, UnitType::Core, "length");
impl_units_display_name_provider!(UnitsNameLengthExtendedV1, UnitType::Extended, "length");
impl_units_display_name_provider!(UnitsNameLengthOutlierV1, UnitType::Outlier, "length");

// Mass
impl_units_display_name_provider!(UnitsNameMassCoreV1, UnitType::Core, "mass");
impl_units_display_name_provider!(UnitsNameMassExtendedV1, UnitType::Extended, "mass");
impl_units_display_name_provider!(UnitsNameMassOutlierV1, UnitType::Outlier, "mass");

// Volume
impl_units_display_name_provider!(UnitsNameVolumeCoreV1, UnitType::Core, "volume");
impl_units_display_name_provider!(UnitsNameVolumeExtendedV1, UnitType::Extended, "volume");
impl_units_display_name_provider!(UnitsNameVolumeOutlierV1, UnitType::Outlier, "volume");
