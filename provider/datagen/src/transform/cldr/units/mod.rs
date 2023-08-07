// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::f32::consts::E;

use icu_collections::codepointtrie::CodePointTrieError;
use icu_provider::{DataError, DataProvider, DataRequest, DataResponse};
use icu_unitsconversion::provider::UnitsConstantsV1Maker;
use crate::transform::cldr::cldr_serde;

use icu_locid::LanguageIdentifier;
use icu_properties::sets::load_for_general_category_group;
use icu_properties::GeneralCategoryGroup;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::maps::MutableZeroVecLike;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use crate::DatagenProvider;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::HashMap;
use tinystr::tinystr;


impl DataProvider<UnitsConstantsV1Maker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Maker>, DataError> {
        self.check_req::<UnitsConstantsV1Maker>(req)?;

        let units_data: &cldr_serde::units::units::Resource = self
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
