// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod helpers;

use std::collections::BTreeMap;

use self::helpers::{
    convert_constant_to_num_denom_strings, convert_slices_to_fraction,
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
        // Contains all the constants that are defined in terms of scientific numbers.
        let mut clean_constants_map =
            BTreeMap::<&str, (Vec<String>, Vec<String>, ConstantExactness)>::new();
        for (cons_name, cons_value) in constants {
            let (num, den) = convert_constant_to_num_denom_strings(&cons_value.value)?;

            let constant_exactness = match cons_value.status.as_deref() {
                Some("approximate") => ConstantExactness::Approximate,
                _ => ConstantExactness::Exact,
            };

            let mut clean_num = Vec::<String>::new();
            let mut clean_den = Vec::<String>::new();
            let mut replaceable_num = Vec::<String>::new();
            let mut replaceable_den = Vec::<String>::new();

            for num_elem in num.iter() {
                if is_scientific_number(num_elem) {
                    clean_num.push(num_elem.clone());
                    continue;
                }
                replaceable_num.push(num_elem.clone());
            }

            for den_elem in den.iter() {
                if is_scientific_number(den_elem) {
                    clean_den.push(den_elem.clone());
                    continue;
                }
                replaceable_den.push(den_elem.clone());
            }

            constants_map_in_str_form.insert(
                cons_name.as_str(),
                (replaceable_num, replaceable_den, constant_exactness),
            );

            clean_constants_map.insert(
                cons_name.as_str(),
                (clean_num, clean_den, constant_exactness),
            );
        }

        // TODO(#4100): Implement a more efficient algorithm for replacing constants with their values.
        // This loop iterates over the constants and replaces any string values with their corresponding constant values.
        let mut updated = false;
        loop {
            // TODO(#4100): remove this copy.
            let constants_map_in_str_form_copy = constants_map_in_str_form.clone();
            for (key, elem) in constants_map_in_str_form.iter_mut() {
                let (num_vec, den_vec, _) = elem;
                for i in (0..num_vec.len()).rev() {
                    if let Some((clean_num, clean_den, clean_constant_exactness)) =
                        clean_constants_map.get(num_vec[i].as_str()).cloned()
                    {
                        if clean_num.is_empty() || clean_den.is_empty() {
                            continue;
                        }
                        if let Some((clean_num_from_str, clean_den_from_str, _)) =
                            constants_map_in_str_form_copy.get(num_vec[i].as_str())
                        {
                            if !clean_num_from_str.is_empty() || !clean_den_from_str.is_empty() {
                                continue;
                            }
                        }
                        let (add_to_num, add_to_den, add_to_exactness) =
                            clean_constants_map.get_mut(key).unwrap();
                        num_vec.remove(i);
                        add_to_num.extend(clean_num);
                        add_to_den.extend(clean_den);
                        if clean_constant_exactness == ConstantExactness::Approximate {
                            *add_to_exactness = ConstantExactness::Approximate;
                        }
                        updated = true;
                    }
                }

                for i in (0..den_vec.len()).rev() {
                    if let Some((clean_num, clean_den, clean_constant_exactness)) =
                        clean_constants_map.get(den_vec[i].as_str()).cloned()
                    {
                        if clean_num.is_empty() || clean_den.is_empty() {
                            continue;
                        }
                        if let Some((clean_num_from_str, clean_den_from_str, _)) =
                            constants_map_in_str_form_copy.get(den_vec[i].as_str())
                        {
                            if !clean_num_from_str.is_empty() || !clean_den_from_str.is_empty() {
                                continue;
                            }
                        }
                        let (add_to_num, add_to_den, add_to_exactness) =
                            clean_constants_map.get_mut(key).unwrap();
                        den_vec.remove(i);
                        add_to_num.extend(clean_den);
                        add_to_den.extend(clean_num);
                        if clean_constant_exactness == ConstantExactness::Approximate {
                            *add_to_exactness = ConstantExactness::Approximate;
                        }

                        updated = true;
                    }
                }
            }

            if updated {
                updated = false;
                continue;
            }

            // Verify that all vectors in constants_map_in_str_form are empty.
            // If they are, we have successfully replaced all constants with their values.
            // If not, return an error due to an infinite loop.
            for (_, (num_vec, den_vec, _)) in constants_map_in_str_form.iter() {
                if !num_vec.is_empty() || !den_vec.is_empty() {
                    return Err(DataError::custom(
                        "Infinite loop detected while replacing constants with their values.",
                    ));
                }
            }

            break;
        }

        // Transforming the `constants_map_in_str_form` map into a ZeroMap of `ConstantValue`.
        // This is done by converting the numerator and denominator slices into a fraction,
        // and then transforming the fraction into a `ConstantValue`.
        let constants_map = ZeroMap::from_iter(
            clean_constants_map
                .into_iter()
                .map(|(cons_name, (num, den, constant_exactness))| {
                    let value = convert_slices_to_fraction(&num, &den)?;
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
    use fraction::GenericFraction;
    use icu_locid::locale;
    use icu_provider::prelude::*;
    use icu_unitsconversion::provider::*;
    use num_bigint::BigUint;

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
            numerator: expected_ft_to_m.numer().unwrap().to_bytes_le().into(),
            denominator: expected_ft_to_m.denom().unwrap().to_bytes_le().into(),
            sign: Sign::Positive,
            constant_exactness: ConstantExactness::Exact,
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
            numerator: expected_ft2_to_m2.numer().unwrap().to_bytes_le().into(),
            denominator: expected_ft2_to_m2.denom().unwrap().to_bytes_le().into(),
            sign: Sign::Positive,
            constant_exactness: ConstantExactness::Exact,
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
            numerator: expected_ft3_to_m3.numer().unwrap().to_bytes_le().into(),
            denominator: expected_ft3_to_m3.denom().unwrap().to_bytes_le().into(),
            sign: Sign::Positive,
            constant_exactness: ConstantExactness::Exact,
        })
        .as_ref()
    );

    // TODO: Implement tests for cases where the constant value includes another constant in the denominator.
    // Example: "12/ft2_to_m2"
    // Although this case is not currently present in CLDR data, it's important to test for it.
}
