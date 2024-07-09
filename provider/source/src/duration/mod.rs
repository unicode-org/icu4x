// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::units::data::DurationUnits;
use crate::SourceDataProvider;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

use icu::experimental::duration::provider::digital::{
    DigitalDurationDataV1, DigitalDurationDataV1Marker,
};

impl DataProvider<DigitalDurationDataV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<DigitalDurationDataV1Marker>, DataError> {
        self.check_req::<DigitalDurationDataV1Marker>(req)?;
        let langid = req.id.locale.get_langid();

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource =
            self.cldr()?.units().read_and_parse(&langid, "units.json")?;
        let DurationUnits { hms, .. } = &units_format_data.main.value.units.duration;

        let result = DigitalDurationDataV1::from_str(&hms.pat).map_err(|err| {
            DataError::custom("Failed to convert hms pattern: ").with_debug_context(&err)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<DigitalDurationDataV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_langs()?
            .filter(|langid| {
                self.cldr()
                    .unwrap()
                    .units()
                    .read_and_parse::<cldr_serde::units::data::Resource>(langid, "units.json")
                    .is_ok()
            })
            .map(|langid| DataIdentifierCow::from_locale(DataLocale::from(&langid)))
            .collect())
    }
}
