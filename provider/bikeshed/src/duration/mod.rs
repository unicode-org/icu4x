// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::provider::transform::cldr::cldr_serde;
use crate::provider::DatagenProvider;
use crate::provider::IterableDataProviderCached;

use std::borrow::Cow;

use icu_provider::datagen::IterableDataProvider;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use tinystr::tinystr;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use icu_provider::DataProvider;

use icu_experimental::duration::provider::*;
use icu_provider::prelude::*;

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
        let duration_data = &units_format_data.main.value.units.duration;

        let mut result = DigitalDurationDataV1 {
            ..todo!()
        };

        Ok(DataResponse {
            metadata: Default::default("DigitalDurationDataV1Marker"),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProvider<DigitalDurationDataV1Marker> for DatagenProvider {
    fn iter_ids(&self) -> Result<std::collections::HashSet<DataIdentifierCow>, DataError> {
        Ok(todo!("Implement iter_ids for DigitalDurationDataV1Marker"))
    }
}
