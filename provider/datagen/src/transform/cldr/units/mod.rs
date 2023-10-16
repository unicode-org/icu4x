// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod helpers;

use std::collections::{BTreeMap, VecDeque};

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

/// Represents a general constant which contains scientific and non scientific numbers.
#[derive(Debug)]
struct GeneralConstant {
    /// Contains numerator terms that are represented as scientific numbers
    clean_num: Vec<String>,
    /// Contains denominator terms that are represented as scientific numbers
    clean_den: Vec<String>,
    /// Contains numerator terms that are not represented as scientific numbers
    non_scientific_num: VecDeque<String>,
    /// Contains denominator terms that are not represented as scientific numbers
    non_scientific_den: VecDeque<String>,
    constant_exactness: ConstantExactness,
}

impl GeneralConstant {
    fn new(num: &[String], den: &[String], exactness: ConstantExactness) -> Self {
        let mut constant = GeneralConstant {
            clean_num: Vec::new(),
            clean_den: Vec::new(),
            non_scientific_num: VecDeque::new(),
            non_scientific_den: VecDeque::new(),
            constant_exactness: exactness,
        };

        for n in num {
            if is_scientific_number(n) {
                constant.clean_num.push(n.clone());
            } else {
                constant.non_scientific_num.push_back(n.clone());
            }
        }

        for d in den {
            if is_scientific_number(d) {
                constant.clean_den.push(d.clone());
            } else {
                constant.non_scientific_den.push_back(d.clone());
            }
        }
        constant
    }

    /// Determines if the constant is free of any non_scientific elements.
    fn is_free_of_non_scientific(&self) -> bool {
        self.non_scientific_num.is_empty() && self.non_scientific_den.is_empty()
    }
}

impl DataProvider<UnitsConstantsV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Marker>, DataError> {
        self.check_req::<UnitsConstantsV1Marker>(_req)?;

        let units_data: &cldr_serde::units::units_constants::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;
        let constants = &units_data.supplemental.unit_constants.constants;

        let mut constants_with_non_scientific = VecDeque::<(&str, GeneralConstant)>::new();

        // Contains all the constants that do not have any non-scientific numbers. I.E., non_scientific_num and non_scientific_den are empty.
        let mut clean_constants_map = BTreeMap::<&str, GeneralConstant>::new();
        for (cons_name, cons_value) in constants {
            let (num, den) = convert_constant_to_num_denom_strings(&cons_value.value)?;

            let constant_exactness = match cons_value.status.as_deref() {
                Some("approximate") => ConstantExactness::Approximate,
                _ => ConstantExactness::Exact,
            };

            let constant = GeneralConstant::new(&num, &den, constant_exactness);

            if constant.is_free_of_non_scientific() {
                clean_constants_map.insert(cons_name, constant);
            } else {
                constants_with_non_scientific.push_back((&cons_name, constant));
            }
        }

        // Replacing non scientific constant terms with their corresponding clean value.
        let mut no_update_count = 0;
        while !constants_with_non_scientific.is_empty() {
            let mut updated = false;
            let (constant_key, mut non_scientific_constant) = constants_with_non_scientific
                .pop_front()
                .ok_or(DataError::custom(
                    "non scientific queue error: an element must exist",
                ))?;

            for _ in 0..non_scientific_constant.non_scientific_num.len() {
                if let Some(num) = non_scientific_constant.non_scientific_num.pop_front() {
                    if let Some(clean_constant) = clean_constants_map.get(num.as_str()) {
                        non_scientific_constant
                            .clean_num
                            .extend(clean_constant.clean_num.clone());
                        non_scientific_constant
                            .clean_den
                            .extend(clean_constant.clean_den.clone());

                        updated = true;
                    } else {
                        non_scientific_constant.non_scientific_num.push_back(num);
                    }
                }
            }

            for _ in 0..non_scientific_constant.non_scientific_den.len() {
                if let Some(den) = non_scientific_constant.non_scientific_den.pop_front() {
                    if let Some(clean_constant) = clean_constants_map.get(den.as_str()) {
                        non_scientific_constant
                            .clean_num
                            .extend(clean_constant.clean_den.clone());
                        non_scientific_constant
                            .clean_den
                            .extend(clean_constant.clean_num.clone());

                        updated = true;
                    } else {
                        non_scientific_constant.non_scientific_den.push_back(den);
                    }
                }
            }

            if non_scientific_constant.is_free_of_non_scientific() {
                clean_constants_map.insert(constant_key, non_scientific_constant);
            } else {
                constants_with_non_scientific.push_back((constant_key, non_scientific_constant));
            }

            no_update_count = if !updated { no_update_count + 1 } else { 0 };
            if no_update_count > constants_with_non_scientific.len() {
                return Err(DataError::custom(
                    "An Infinite loop was detected in the CLDR constants data!",
                ));
            }
        }

        // Convert `clean_constants_map` into a ZeroMap of `ConstantValue`.
        // This involves transforming the numerator and denominator slices into a fraction,
        // and subsequently converting the fraction into a `ConstantValue`.
        let constants_map = ZeroMap::from_iter(
            clean_constants_map
                .into_iter()
                .map(|(cons_name, constant)| {
                    let value = convert_slices_to_fraction(
                        &constant
                            .clean_num
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&str>>(),
                        &constant
                            .clean_den
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&str>>(),
                    )?;
                    let (num, den, sign, cons_type) =
                        transform_fraction_to_constant_value(value, constant.constant_exactness)?;

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
    let expected_ft2_to_m2 = &expected_ft_to_m * &expected_ft_to_m;

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
    let expected_ft3_to_m3 = &expected_ft2_to_m2 * &expected_ft_to_m;

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
