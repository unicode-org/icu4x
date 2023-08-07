// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).




use icu_provider::{DataError, DataProvider, DataRequest, DataResponse};
use icu_unitsconversion::provider::UnitsConstantsV1Maker;
use crate::transform::cldr::cldr_serde;
















impl DataProvider<UnitsConstantsV1Maker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Maker>, DataError> {
        // self.check_req::<UnitsConstantsV1Maker>(req)?;

        let _units_data: &cldr_serde::units::units_constants::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;



        // TODO
        // Ok(DataResponse {
        //     metadata: Default::default(),
        //     payload: Some(DataPayload::from_owned(result?)),
        // })

        let data_error = DataError::custom("This is an example error");

        Err(data_error)
    }
}
