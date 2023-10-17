use super::helpers::{convert_slices_to_fraction, transform_fraction_to_constant_value};
use crate::transform::cldr::cldr_serde::{self};
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use icu_unitsconversion::cons_provider::{ConstantValue, UnitsConstantsV1, UnitsConstantsV1Marker};
use zerovec::{ZeroMap, ZeroVec};

use super::helpers::process_constants;

impl DataProvider<UnitsConstantsV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsConstantsV1Marker>, DataError> {
        self.check_req::<UnitsConstantsV1Marker>(_req)?;

        let units_data: &cldr_serde::units::units_constants::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;
        let constants = &units_data.supplemental.unit_constants.constants;

        let clean_constants_map = process_constants(constants)?;
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
    use icu_unitsconversion::cons_provider::*;
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
