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
use zerovec::ZeroMap;

impl DataProvider<UnitsConstantsV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Marker>, DataError> {
        self.check_req::<UnitsConstantsV1Marker>(_req)?;

        let _units_data: &cldr_serde::units::units_constants::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;
        let mut constants_map = BTreeMap::<&str, &str>::new();

        let constants = &_units_data.supplemental.unit_constants.constants;
        for (key, constant) in constants {
            constants_map.insert(key, &constant.value);
        }

        let result = UnitsConstantsV1 {
            constants_map: ZeroMap::from_iter(constants_map.into_iter()),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProvider<UnitsConstantsV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

#[test]
fn test_basic() {
    use icu_locid::locale;
    use icu_provider::prelude::*;
    use icu_unitsconversion::provider::*;

    let provider = crate::DatagenProvider::latest_tested_offline_subset();

    let und: DataPayload<UnitsConstantsV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("und").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let constants = &und.get().to_owned().constants_map;

    assert_eq!(constants.get("ft_to_m").unwrap(), "0.3048");
    assert_eq!(constants.get("ft2_to_m2").unwrap(), "ft_to_m*ft_to_m");
}
