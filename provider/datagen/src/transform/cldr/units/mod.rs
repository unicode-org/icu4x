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

#[derive(Debug)]
struct CleanAndDirtyConstant {
    clean_num: Vec<String>,
    clean_den: Vec<String>,
    dirty_num: VecDeque<String>,
    dirty_den: VecDeque<String>,
    constant_exactness: ConstantExactness,
}

impl CleanAndDirtyConstant {
    fn new(num: &[String], den: &[String], exactness: ConstantExactness) -> Self {
        let mut constant = CleanAndDirtyConstant {
            clean_num: Vec::new(),
            clean_den: Vec::new(),
            dirty_num: VecDeque::new(),
            dirty_den: VecDeque::new(),
            constant_exactness: exactness,
        };

        for n in num {
            if is_scientific_number(n) {
                constant.clean_num.push(n.clone());
            } else {
                constant.dirty_num.push_back(n.clone());
            }
        }

        for d in den {
            if is_scientific_number(d) {
                constant.clean_den.push(d.clone());
            } else {
                constant.dirty_den.push_back(d.clone());
            }
        }
        constant
    }

    /// Determines if the constant is free of any dirty elements.
    fn is_clean(&self) -> bool {
        self.dirty_num.is_empty() && self.dirty_den.is_empty()
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

        let mut dirty_constants_queue = VecDeque::<(&str, CleanAndDirtyConstant)>::new();

        // Contains all the constants that do not have any dirty data. I.E. dirty_num and dirty_den are empty.
        let mut clean_constants_map = BTreeMap::<&str, CleanAndDirtyConstant>::new();
        for (cons_name, cons_value) in constants {
            let (num, den) = convert_constant_to_num_denom_strings(&cons_value.value)?;

            let constant_exactness = match cons_value.status.as_deref() {
                Some("approximate") => ConstantExactness::Approximate,
                _ => ConstantExactness::Exact,
            };

            let constant = CleanAndDirtyConstant::new(&num, &den, constant_exactness);

            if constant.is_clean() {
                clean_constants_map.insert(&cons_name, constant);
            } else {
                dirty_constants_queue.push_back((&cons_name, constant));
            }
        }

        // Replacing dirty constants with their corresponding clean value.
        let mut count = 0;
        while !dirty_constants_queue.is_empty() {
            let mut updated = false;
            let (constant_key, mut dirty_constant) = dirty_constants_queue
                .pop_front()
                .ok_or(DataError::custom("dirty queue defect"))?;

            for _ in 0..dirty_constant.dirty_num.len() {
                if let Some(num) = dirty_constant.dirty_num.pop_front() {
                    if let Some(clean_constant) = clean_constants_map.get(num.as_str()) {
                        dirty_constant
                            .clean_num
                            .extend(clean_constant.clean_num.clone());
                        dirty_constant
                            .clean_den
                            .extend(clean_constant.clean_den.clone());

                        updated = true;
                    } else {
                        dirty_constant.dirty_num.push_back(num);
                    }
                }
            }

            for _ in 0..dirty_constant.dirty_den.len() {
                if let Some(den) = dirty_constant.dirty_den.pop_front() {
                    if let Some(clean_constant) = clean_constants_map.get(den.as_str()) {
                        dirty_constant
                            .clean_num
                            .extend(clean_constant.clean_den.clone());
                        dirty_constant
                            .clean_den
                            .extend(clean_constant.clean_num.clone());

                        updated = true;
                    } else {
                        dirty_constant.dirty_den.push_back(den);
                    }
                }
            }

            if dirty_constant.is_clean() {
                clean_constants_map.insert(constant_key, dirty_constant);
            } else {
                dirty_constants_queue.push_back((constant_key, dirty_constant));
            }

            count = if !updated { count + 1 } else { 0 };
            if count > dirty_constants_queue.len() {
                return Err(DataError::custom(
                    "An Infinite loop was detected in the CLDR constants data!",
                ));
            }
        }

        // Transforming the `clean_constants_map` map into a ZeroMap of `ConstantValue`.
        // This is done by converting the numerator and denominator slices into a fraction,
        // and then transforming the fraction into a `ConstantValue`.
        let constants_map = ZeroMap::from_iter(
            clean_constants_map
                .into_iter()
                .map(|(cons_name, constant)| {
                    let value = convert_slices_to_fraction(
                        &constant
                            .clean_num
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
                        &constant
                            .clean_den
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
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
