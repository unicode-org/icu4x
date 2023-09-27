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

            let constant_exactness = match cons_value.status.as_deref() {
                Some("approximate") => ConstantExactness::Approximate,
                _ => ConstantExactness::Exact,
            };

            constants_map_in_str_form.insert(cons_name, (num, den, constant_exactness));
        }

        // This loop iterates over the constants, replacing any string values with their corresponding constant values.
        // For example, if the constant "ft_to_m" has the value "0.3048", and the constant "ft2_to_m2" has the value "ft_to_m * ft_to_m",
        // the maximum depth represents the maximum number of nested constants that can be replaced.
        // If CLDR added more constants that are defined in terms of other constants, the maximum depth should be increased.
        let maximum_depth = 10;
        let mut has_internal_constants;
        let mut max_depth_reached = 0;
        while max_depth_reached < maximum_depth {
            has_internal_constants = false;
            max_depth_reached += 1;
            let mut constants_with_constants_map_replaceable =
                BTreeMap::<&str, (Vec<String>, Vec<String>, ConstantExactness)>::new();
            for (cons_name, (num, den, constant_exactness)) in constants_map_in_str_form.iter() {
                let mut temp_num = Vec::<String>::new();
                let mut temp_den = Vec::<String>::new();
                let mut temp_constant_exactness = constant_exactness.clone();

                for num_str in num.iter().cloned() {
                    if !contains_alphabetic_chars(&num_str) || is_scientific_number(&num_str) {
                        temp_num.push(num_str.clone());
                        continue;
                    }
                    has_internal_constants = true;

                    if let Some((rnum, rden, rconstant_exactness)) =
                        constants_map_in_str_form.get(num_str.as_str())
                    {
                        temp_num.extend(rnum.clone());
                        temp_den.extend(rden.clone());
                        if rconstant_exactness == &ConstantExactness::Approximate {
                            temp_constant_exactness = ConstantExactness::Approximate;
                        }
                    } else {
                        temp_num.push(num_str.clone());
                    }
                }

                for den_str in den.iter().cloned() {
                    if !contains_alphabetic_chars(&den_str) || is_scientific_number(&den_str) {
                        temp_den.push(den_str.clone());
                        continue;
                    }
                    has_internal_constants = true;

                    if let Some((rnum, rden, rconstant_exactness)) =
                        constants_map_in_str_form.get(den_str.as_str())
                    {
                        temp_num.extend(rden.clone());
                        temp_den.extend(rnum.clone());

                        if rconstant_exactness == &ConstantExactness::Approximate {
                            temp_constant_exactness = rconstant_exactness.clone();
                        }
                    } else {
                        temp_den.push(den_str.clone());
                    }
                }

                constants_with_constants_map_replaceable
                    .insert(cons_name, (temp_num, temp_den, temp_constant_exactness));
            }

            constants_map_in_str_form = constants_with_constants_map_replaceable;

            if !has_internal_constants {
                break;
            }
        }

        if max_depth_reached >= maximum_depth {
            return Err(DataError::custom(
                "Maximum depth reached while parsing constants. \
                This is likely due to a circular dependency in the constants. \
                Note: If the depth was increased, you may need to increase the maximum depth in the code.",
            ));
        }

        // Transforming the `constants_map_in_str_form` map into a ZeroMap of `ConstantValue`.
        // This is done by converting the numerator and denominator slices into a fraction,
        // and then transforming the fraction into a `ConstantValue`.
        let constants_map = ZeroMap::from_iter(
            constants_map_in_str_form
                .into_iter()
                .map(|(cons_name, (num, den, constant_exactness))| {
                    // Converting slices to fraction
                    let value = convert_slices_to_fraction(&num, &den)?;

                    // Transforming the fraction to a constant value
                    let (num, den, sign, cons_type) =
                        transform_fraction_to_constant_value(value, constant_exactness)?;
                    Ok((
                        cons_name,
                        zerovec::ule::encode_varule_to_box(&ConstantValue {
                            numerator: ZeroVec::from_iter(num),
                            denominator: ZeroVec::from_iter(den),
                            sign,
                            constant_exactness: cons_type,
                        }),
                    ))
                })
                .collect::<Result<Vec<_>, DataError>>()?,
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
    use num_bigint::BigInt;
    use num_rational::BigRational;
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
    let expected_ft_to_m = BigRational::new(BigInt::from(3048u32), BigInt::from(10000u32));

    assert_eq!(
        ft_to_m,
        zerovec::ule::encode_varule_to_box(&ConstantValue {
            numerator: expected_ft_to_m.numer().to_le_bytes().into(),
            denominator: expected_ft_to_m.denom().to_le_bytes().into(),
            sign: Sign::Positive,
            constant_exactness: ConstantExactness::Exact,
        })
        .as_ref()
    );

    let ft2_to_m2 = constants.get("ft2_to_m2").unwrap();
    let expected_ft2_to_m2 =
        BigRational::new(BigInt::from(3048u32).pow(2), BigInt::from(10000u32).pow(2));

    assert_eq!(
        ft2_to_m2,
        zerovec::ule::encode_varule_to_box(&ConstantValue {
            numerator: expected_ft2_to_m2.numer().to_le_bytes().into(),
            denominator: expected_ft2_to_m2.denom().to_le_bytes().into(),
            sign: Sign::Positive,
            constant_exactness: ConstantExactness::Exact,
        })
        .as_ref()
    );

    let ft3_to_m3 = constants.get("ft3_to_m3").unwrap();
    let expected_ft3_to_m3 =
        BigRational::new(BigInt::from(3048u32).pow(3), BigInt::from(10000u32).pow(3));

    assert_eq!(
        ft3_to_m3,
        zerovec::ule::encode_varule_to_box(&ConstantValue {
            numerator: expected_ft3_to_m3.numer().to_le_bytes().into(),
            denominator: expected_ft3_to_m3.denom().to_le_bytes().into(),
            sign: Sign::Positive,
            constant_exactness: ConstantExactness::Exact,
        })
        .as_ref()
    );

    // TODO: Implement tests for cases where the constant value includes another constant in the denominator.
    // Example: "12/ft2_to_m2"
    // Although this case is not currently present in CLDR data, it's important to test for it.
}
