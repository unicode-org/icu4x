// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::units::display_name::UnitsDisplayName;
use icu::experimental::dimension::provider::units::specific_display_name::UnitsNameMeterV1;
use icu::plurals::PluralElements;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;

impl DataProvider<UnitsNameMeterV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsNameMeterV1>, DataError> {
        self.check_req::<UnitsNameMeterV1>(req)?;

        let specific_unit = "meter";
        let length = req.id.marker_attributes.as_str();

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
        .categories
        .iter()
        .find_map(|(_, units_map)| units_map.get(specific_unit))
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
}

impl crate::IterableDataProviderCached<UnitsNameMeterV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut data_locales = HashSet::new();

        let specific_unit = "meter";

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

                // TODO: category is not used for now, but will be used when we have categorized data markers.
                for units_map in length_patterns.categories.values() {
                    for (unit, patterns) in units_map {
                        if patterns.other.is_none() || unit != specific_unit {
                            continue;
                        }
                        data_locales.insert(DataIdentifierCow::from_owned(
                            DataMarkerAttributes::try_from_string(format!("{length}")).map_err(
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
        }

        Ok(data_locales)
    }
}
