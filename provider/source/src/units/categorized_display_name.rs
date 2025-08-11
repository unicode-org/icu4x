// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde::{self};
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::units::categorized_display_name::{
    LengthDisplayName, LengthDisplayNameV1,
};
use icu::plurals::PluralElements;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;

impl DataProvider<LengthDisplayNameV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LengthDisplayNameV1>, DataError> {
        self.check_req::<LengthDisplayNameV1>(req)?;

        // TODO: This category is based on the Marker, in the next PR, this will be variable in a Macro based on the category.
        let category = "length";
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
        .categories
        .get(category)
        .ok_or_else(|| {
            DataErrorKind::IdentifierNotFound
                .into_error()
                .with_debug_context(category)
        })?
        .get(unit)
        .ok_or_else(|| {
            DataErrorKind::IdentifierNotFound
                .into_error()
                .with_debug_context(length)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(LengthDisplayName {
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

impl crate::IterableDataProviderCached<LengthDisplayNameV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        // This category is based on the Marker, in the next PR, this will be variable in a Macro based on the category.
        let category = "length";

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
