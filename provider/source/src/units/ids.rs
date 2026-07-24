// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use icu::experimental::measure::parser::ids::unit_id;
use icu::experimental::measure::provider::UnitIdsV1;
use icu_provider::DataError;
use icu_provider::DataMarkerAttributes;
use icu_provider::DataProvider;
use icu_provider::DataRequest;
use icu_provider::DataResponse;
use icu_provider::prelude::*;

use crate::SourceDataProvider;
use crate::cldr_serde;

impl DataProvider<UnitIdsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitIdsV1>, DataError> {
        self.check_req::<UnitIdsV1>(req)?;

        let unit = unit_id(req.id.marker_attributes.as_str())
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.with_req(UnitIdsV1::INFO, req))?;

        Ok(DataResponse {
            payload: DataPayload::from_owned(unit),
            metadata: Default::default(),
        })
    }
}

impl crate::IterableDataProviderCached<UnitIdsV1> for SourceDataProvider {
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
            .filter(|unit_name| {
                if unit_id(unit_name).is_some() {
                    true
                } else {
                    log::error!("Unit '{unit_name}' does not have an ICU4X unit_id, skipping");
                    false
                }
            })
            .map(|unit_name| {
                DataIdentifierCow::from_marker_attributes_owned(
                    DataMarkerAttributes::try_from_string(unit_name.clone()).unwrap(),
                )
            })
            .collect();

        Ok(ids_set)
    }
}
