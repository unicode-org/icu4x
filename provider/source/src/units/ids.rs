// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;
use std::collections::HashSet;

use icu::experimental::measure::provider::ids::Index;
use icu::experimental::measure::provider::ids::UnitsIdsV1;
use icu_provider::prelude::*;
use icu_provider::DataError;
use icu_provider::DataMarkerAttributes;
use icu_provider::DataProvider;
use icu_provider::DataRequest;
use icu_provider::DataResponse;

use crate::cldr_serde;
use crate::SourceDataProvider;

impl DataProvider<UnitsIdsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsIdsV1>, DataError> {
        self.check_req::<UnitsIdsV1>(req)?;
        let units_data: &cldr_serde::units::info::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;

        let unit = req.id.marker_attributes.as_str();

        let indices_map: BTreeMap<String, usize> = units_data
            .supplemental
            .convert_units
            .convert_units
            .keys()
            .enumerate()
            .map(|(index, unit_name)| (unit_name.clone(), index))
            .collect();

        let index = indices_map
            .get(unit)
            .ok_or_else(|| DataError::custom("Failed to load units"))
            .and_then(|&idx| {
                u16::try_from(idx).map_err(|_| DataError::custom("Index out of range for u16"))
            })?;

        Ok(DataResponse {
            payload: DataPayload::from_owned(Index(index)),
            metadata: Default::default(),
        })
    }
}

impl crate::IterableDataProviderCached<UnitsIdsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let units_data: &cldr_serde::units::info::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;

        let ids_set = units_data
            .supplemental
            .convert_units
            .convert_units
            .keys()
            .map(|unit_name| {
                DataIdentifierCow::from_marker_attributes_owned(
                    DataMarkerAttributes::try_from_string(unit_name.clone()).unwrap(),
                )
            })
            .collect();

        Ok(ids_set)
    }
}
