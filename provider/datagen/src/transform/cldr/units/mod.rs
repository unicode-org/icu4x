// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{DataError, DataPayload, DataProvider, DataRequest, DataResponse};
use icu_unitsconversion::provider::UnitsConstantsV1Maker;

use crate::transform::cldr::cldr_serde;
use crate::DatagenProvider;

use icu_provider::datagen::IterableDataProvider;

impl DataProvider<UnitsConstantsV1Maker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Maker>, DataError> {
        self.check_req::<UnitsConstantsV1Maker>(req)?;

        let units_data: &cldr_serde::units::units::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result?)),
        })
    }
}
