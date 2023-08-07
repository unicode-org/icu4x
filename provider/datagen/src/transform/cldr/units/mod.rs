// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_provider::{DataError, DataPayload, DataProvider, DataRequest, DataResponse};
use icu_unitsconversion::provider::{ConstantValue, UnitsConstantsV1, UnitsConstantsV1Maker};
use zerovec::maps::MutableZeroVecLike;

impl DataProvider<UnitsConstantsV1Maker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Maker>, DataError> {
        // self.check_req::<UnitsConstantsV1Maker>(req)?;

        let _units_data: &cldr_serde::units::units_constants::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;
        let mut constants_values = zerovec::ZeroVec::<ConstantValue>::new();
        let mut constants_map = zerovec::ZeroMap::<str, u16>::new();

        let constants = &_units_data.supplemental.unit_constants.constants;
        for (key, _ /* value */) in constants {
            let constant: ConstantValue = Default::default();
            // TODO: Fill constant values.

            let index = constants_values.len() as u16;
            constants_values.zvl_push(&constant);
            constants_map.insert(key, &index);
        }

        let result = UnitsConstantsV1 {
            constants_map,
            constants_values,
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}
