// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod helpers;

use std::collections::BTreeMap;

use self::helpers::{
    contains_alphabetic_chars, convert_constant_to_num_denom_strings, convert_slices_to_fraction,
    transform_fraction_to_constant_value,
};
use crate::transform::cldr::{cldr_serde, units::helpers::is_scientific_number};
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use icu_unitsconversion::provider::{
    ConstantExactness, ConstantValue, UnitsConstantsV1, UnitsConstantsV1Marker,
};
use zerovec::{ZeroMap, ZeroVec};

impl DataProvider<UnitsConstantsV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Marker>, DataError> {
        self.check_req::<UnitsConstantsV1Marker>(_req)?;

        let units_data: &cldr_serde::units::units_constants::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;
        let constants = &units_data.supplemental.unit_constants.constants;

        let mut constants_map_in_str_form =
            BTreeMap::<&str, (Vec<String>, Vec<String>, ConstantExactness)>::new();
        for (cons_name, cons_value) in constants {
            let (num, den) = convert_constant_to_num_denom_strings(&cons_value.value)?;

            let constant_type = match cons_value.status.as_deref() {
                Some("approximate") => ConstantExactness::Approximate,
                _ => ConstantExactness::Actual,
            };

            constants_map_in_str_form.insert(cons_name, (num, den, constant_type));
        }

        // This loop iterates over the constants, replacing any string values with their corresponding constant values.
        let mut has_internal_constants;
        loop {
            has_internal_constants = false;
            let mut constants_with_constants_map_replaceable =
                BTreeMap::<&str, (Vec<String>, Vec<String>, ConstantExactness)>::new();
            for (cons_name, (num, den, constant_type)) in constants_map_in_str_form.iter() {
                let mut temp_num = num.clone();
                let mut temp_den = den.clone();
                let mut temp_constant_type = *constant_type;

                for i in 0..temp_num.len() {
                    if !contains_alphabetic_chars(temp_num[i].as_str())
                        || is_scientific_number(temp_num[i].as_str())
                    {
                        continue;
                    }

                    has_internal_constants = true;
                    if let Some((rnum, rden, rconstant_type)) =
                        constants_map_in_str_form.get(temp_num[i].as_str())
                    {
                        temp_num.remove(i);
                        // append the elements in rnum to num and rden to den
                        temp_num.extend(rnum.clone().into_iter());
                        temp_den.extend(rden.clone().into_iter());

                        if *rconstant_type == ConstantExactness::Approximate {
                            temp_constant_type = ConstantExactness::Approximate;
                        }
                    }
                }

                for i in 0..temp_den.len() {
                    if !contains_alphabetic_chars(temp_den[i].as_str())
                        || is_scientific_number(temp_den[i].as_str())
                    {
                        continue;
                    }

                    has_internal_constants = true;
                    if let Some((rnum, rden, rconstant_type)) =
                        constants_map_in_str_form.get(temp_den[i].as_str())
                    {
                        temp_den.remove(i);
                        // append the elements in rnum to den and rden to num
                        temp_num.extend(rden.clone().into_iter());
                        temp_den.extend(rnum.clone().into_iter());

                        if *rconstant_type == ConstantExactness::Approximate {
                            temp_constant_type = ConstantExactness::Approximate;
                        }
                    }
                }

                constants_with_constants_map_replaceable
                    .insert(cons_name, (temp_num, temp_den, temp_constant_type));
            }

            constants_map_in_str_form = constants_with_constants_map_replaceable;

            if !has_internal_constants {
                break;
            }
        }

        let mut constants_map = BTreeMap::<&str, ConstantValue>::new();

        for (cons_name, (num, den, constant_type)) in constants_map_in_str_form.iter() {
            let value = convert_slices_to_fraction(num, den)?;
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
    use fraction::GenericFraction;
    use icu_locid::locale;
    use icu_provider::prelude::*;
    use icu_unitsconversion::provider::*;
    use num_bigint::BigUint;
    use num_traits::ToBytes;

    let provider = crate::DatagenProvider::new_testing();

    let und: DataPayload<UnitsConstantsV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("und").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let constants = &und.get().to_owned().constants_map;
    let ft_to_m = constants.get("ft_to_m").unwrap();
    let expected_ft_to_m =
        GenericFraction::<BigUint>::new(BigUint::from(3048u32), BigUint::from(10000u32));
    assert_eq!(
        ft_to_m,
        zerovec::ule::encode_varule_to_box(&ConstantValue {
            numerator: expected_ft_to_m.numer().unwrap().to_le_bytes().into(),
            denominator: expected_ft_to_m.denom().unwrap().to_le_bytes().into(),
            sign: Sign::Positive,
            constant_type: ConstantExactness::Actual,
        })
        .as_ref()
    );

    let ft2_to_m2 = constants.get("ft2_to_m2").unwrap();
    let expected_ft2_to_m2 = GenericFraction::<BigUint>::new(
        BigUint::from(3048u32).pow(2),
        BigUint::from(10000u32).pow(2),
    );

    assert_eq!(
        ft2_to_m2,
        zerovec::ule::encode_varule_to_box(&ConstantValue {
            numerator: expected_ft2_to_m2.numer().unwrap().to_le_bytes().into(),
            denominator: expected_ft2_to_m2.denom().unwrap().to_le_bytes().into(),
            sign: Sign::Positive,
            constant_type: ConstantExactness::Actual,
        })
        .as_ref()
    );

    let ft3_to_m3 = constants.get("ft3_to_m3").unwrap();
    let expected_ft3_to_m3 = GenericFraction::<BigUint>::new(
        BigUint::from(3048u32).pow(3),
        BigUint::from(10000u32).pow(3),
    );

    assert_eq!(
        ft3_to_m3,
        zerovec::ule::encode_varule_to_box(&ConstantValue {
            numerator: expected_ft3_to_m3.numer().unwrap().to_le_bytes().into(),
            denominator: expected_ft3_to_m3.denom().unwrap().to_le_bytes().into(),
            sign: Sign::Positive,
            constant_type: ConstantExactness::Actual,
        })
        .as_ref()
    );

    // TODO: Implement tests for cases where the constant value includes another constant in the denominator.
    // Example: "12/ft2_to_m2"
    // Although this case is not currently present in CLDR data, it's important to test for it.
}
