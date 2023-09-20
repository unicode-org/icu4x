// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod helpers;

use std::collections::BTreeMap;

use crate::transform::cldr::{cldr_serde, units::helpers::is_scientific_number};

use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use icu_unitsconversion::provider::{
    ConstantType, ConstantValue, UnitsConstantsV1, UnitsConstantsV1Marker,
};

use zerovec::{ZeroMap, ZeroVec};

use self::helpers::{
    contains_alphabetic_chars, convert_array_of_strings_to_fraction, remove_whitespace,
    split_constant_string, transform_fraction_to_constant_value,
};

impl DataProvider<UnitsConstantsV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Marker>, DataError> {
        self.check_req::<UnitsConstantsV1Marker>(_req)?;

        let _units_data: &cldr_serde::units::units_constants::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;
        let _constants_map = BTreeMap::<&str, ConstantValue>::new();

        let constants = &_units_data.supplemental.unit_constants.constants;

        // Constants that has a constants in their value.
        //      For exmaple: "ft2_to_m2": "ft_to_m * ft_to_m",
        let mut constants_with_constants_map =
            BTreeMap::<&str, (Vec<String>, Vec<String>, ConstantType)>::new();

        for (cons_name, cons_value) in constants {
            let value = remove_whitespace(&cons_value.value);
            let (num, den) = match split_constant_string(&value) {
                Ok((num, den)) => (num, den),
                Err(e) => {
                    return Err(e);
                }
            };

            let constant_type = match cons_value.status.as_deref() {
                Some("approximate") => ConstantType::Approximate,
                _ => ConstantType::Actual,
            };

            constants_with_constants_map.insert(cons_name, (num, den, constant_type));
        }

        // This loop will replace all the constants in the value of a constant with their values.
        let mut cons_with_text;
        loop {
            cons_with_text = 0;
            let mut constants_with_constants_map_replaceable =
                BTreeMap::<&str, (Vec<String>, Vec<String>, ConstantType)>::new();
            for (cons_name, (num, den, constant_type)) in constants_with_constants_map.iter() {
                let mut temp_num = num.clone();
                let mut temp_den = den.clone();
                let mut temp_constant_type = *constant_type;

                for i in 0..temp_num.len() {
                    if !contains_alphabetic_chars(temp_num[i].as_str())
                        || is_scientific_number(temp_num[i].as_str())
                    {
                        continue;
                    }

                    cons_with_text += 1;
                    if let Some((rnum, rden, rconstant_type)) =
                        constants_with_constants_map.get(temp_num[i].as_str())
                    {
                        temp_num.remove(i);
                        // append the elements in rnum to num and rden to den
                        temp_num.append(&mut rnum.clone());
                        temp_den.append(&mut rden.clone());

                        if *rconstant_type == ConstantType::Approximate {
                            temp_constant_type = ConstantType::Approximate;
                        }
                    }
                }

                for i in 0..temp_den.len() {
                    if !contains_alphabetic_chars(temp_den[i].as_str())
                        || is_scientific_number(temp_den[i].as_str())
                    {
                        continue;
                    }

                    cons_with_text += 1;
                    if let Some((rnum, rden, rconstant_type)) =
                        constants_with_constants_map.get(temp_den[i].as_str())
                    {
                        temp_den.remove(i);
                        // append the elements in rnum to den and rden to num
                        temp_num.append(&mut rden.clone());
                        temp_den.append(&mut rnum.clone());

                        if *rconstant_type == ConstantType::Approximate {
                            temp_constant_type = ConstantType::Approximate;
                        }
                    }
                }

                constants_with_constants_map_replaceable
                    .insert(cons_name, (temp_num, temp_den, temp_constant_type));
            }

            constants_with_constants_map.clear();
            constants_with_constants_map = constants_with_constants_map_replaceable;

            if cons_with_text == 0 {
                break;
            }
        }

        let mut constants_map = BTreeMap::<&str, ConstantValue>::new();

        for (cons_name, (num, den, constant_type)) in constants_with_constants_map.iter() {
            let value = convert_array_of_strings_to_fraction(num, den)?;
            let (num, den, sign, cons_type) =
                transform_fraction_to_constant_value(value, *constant_type)?;
            constants_map.insert(
                cons_name,
                ConstantValue {
                    numerator: ZeroVec::from_iter(num),
                    denominator: ZeroVec::from_iter(den),
                    sign,
                    constant_type: cons_type,
                },
            );
        }
        let constants_map = ZeroMap::from_iter(
            constants_map
                .into_iter()
                .map(|(k, v)| (k, zerovec::ule::encode_varule_to_box(&v))),
        );
        let result = UnitsConstantsV1 { constants_map };

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

    let _und: DataPayload<UnitsConstantsV1Marker> = provider
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
