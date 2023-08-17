// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use crate::transform::cldr::cldr_serde;
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use icu_unitsconversion::provider::{UnitsConstantsV1, UnitsConstantsV1Marker};
use zerovec::{maps::MutableZeroVecLike, VarZeroVec, ZeroMap};

impl DataProvider<UnitsConstantsV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Marker>, DataError> {
        self.check_req::<UnitsConstantsV1Marker>(_req)?;

        let _units_data: &cldr_serde::units::units_constants::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;
        let mut constants_values = Vec::<&str>::new();
        let mut constants_map = BTreeMap::<&str, u16>::new();

        let constants = &_units_data.supplemental.unit_constants.constants;
        for (key, value) in constants {
            let index = constants_values.len() as u16;
            constants_values.push(key);
            constants_map.insert(key, index);
        }

        let result = UnitsConstantsV1 {
            constants_map: ZeroMap::from_iter(constants_map.into_iter()),
            constants_values: VarZeroVec::from(&constants_values),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProvider<UnitsConstantsV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .numbers()
            .list_langs()?
            .map(DataLocale::from)
            .collect())
    }
}
