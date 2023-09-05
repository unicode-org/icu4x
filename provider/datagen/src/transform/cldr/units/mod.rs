// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod helpers;

use std::collections::BTreeMap;

use crate::transform::cldr::cldr_serde;
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use icu_unitsconversion::provider::{
    ConstantType, ConstantValue, UnitsConstantsV1, UnitsConstantsV1Marker,
};
use zerovec::{ZeroMap};

use self::helpers::{
    convert_any_constant_value_to_fractional, convert_constant_value_in_scientific_to_fractional,
};

impl DataProvider<UnitsConstantsV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Marker>, DataError> {
        self.check_req::<UnitsConstantsV1Marker>(_req)?;

        let _units_data: &cldr_serde::units::units_constants::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;
        let mut constants_map = BTreeMap::<&str, ConstantValue>::new();

        let constants = &_units_data.supplemental.unit_constants.constants;
        let mut constants_need_map = Vec::<(&str, &str, ConstantType)>::new();
        for (key, constant) in constants {
            let constant_type = match &constant.status {
                Some(status) => match status.as_str() {
                    "approximate" => ConstantType::Approximate,
                    _ => return Err(DataError::custom("Unknown constant type")),
                },
                None => ConstantType::Actual,
            };

            let constant_str = constant.value.as_str();
            let constant_value = match convert_constant_value_in_scientific_to_fractional(
                constant_str,
                constant_type,
            ) {
                Ok(value) => value,
                Err(_) => {
                    constants_need_map.push((key, constant_str, constant_type));
                    continue;
                }
            };

            constants_map.insert(key, constant_value);
        }

        for (key, constant_str, constant_type) in constants_need_map {
            let constant_value = convert_any_constant_value_to_fractional(
                constant_str,
                &constants_map,
                constant_type,
            );

            match constant_value {
                Ok(constant_value) => {
                    constants_map.insert(key, constant_value);
                }
                Err(_) => {
                    return Err(DataError::custom("Failed to convert constant_str")
                        .with_debug_context(constant_str))
                }
            }
        }

        let result = UnitsConstantsV1 {
            constants_map: ZeroMap::from_iter(constants_map),
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

    // let constants = &und.get().to_owned().constants_map;

    // let ft_to_m : ConstantValue = constants.get("ft_to_m").unwrap();

    // assert_eq!(ft_to_m , ConstantValue {
    //     numerator: 3048,
    //     denominator: 10000,
    //     constant_type: ConstantType::Actual,
    // });

    // assert!(constants.get("ft2_to_m2").eq( ConstantValue {
    //     numerator: 3048,
    //     denominator: 10000,
    //     constant_type: ConstantType::Actual,
    // }));
}
