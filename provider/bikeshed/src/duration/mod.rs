// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::units::data::DurationUnits;
use crate::DatagenProvider;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

use icu::experimental::duration::provider::digital::{
    DigitalDurationDataV1, DigitalDurationDataV1Marker, HmVariant, HmsVariant, MsVariant,
    UnknownPatternError,
};

impl DataProvider<DigitalDurationDataV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<DigitalDurationDataV1Marker>, DataError> {
        self.check_req::<DigitalDurationDataV1Marker>(req)?;
        let langid = req.id.locale.get_langid();

        let unit = match req.id.marker_attributes.parse::<String>() {
            Ok(aux_keys) => aux_keys,
            Err(_) => return Err(DataError::custom("Failed to get aux keys")),
        };

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource =
            self.cldr()?.units().read_and_parse(&langid, "units.json")?;
        let DurationUnits { hms, hm, ms } = &units_format_data.main.value.units.duration;

        let conversion_err =
            |msg, err: &UnknownPatternError| DataError::custom(msg).with_debug_context(err);

        let result = DigitalDurationDataV1 {
            hms: HmsVariant::from_str(&hms.pat)
                .map_err(|e| conversion_err("Invalid hms pattern", &e))?,
            hm: HmVariant::from_str(&hm.pat)
                .map_err(|e| conversion_err("Invalid hm pattern", &e))?,
            ms: MsVariant::from_str(&ms.pat)
                .map_err(|e| conversion_err("Invalid ms pattern", &e))?,
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<DigitalDurationDataV1Marker> for DatagenProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        todo!("IterableDataProviderCached for DigitalDurationDataV1Marker")
    }
}
