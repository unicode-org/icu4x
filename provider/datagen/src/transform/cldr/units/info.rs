// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{borrow::Cow, collections::BTreeMap};

use crate::transform::cldr::cldr_serde::{self};
use fraction::{GenericFraction, Zero};
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use icu_unitsconversion::provider::{
    ConversionInfo, DerivationSpecifier, Dimension, UnitsInfoIndex, UnitsInfoV1, UnitsInfoV1Marker,
};
use zerovec::{VarZeroVec, ZeroMap, ZeroVec};

use super::helpers::{extract_conversion_info, process_constants, process_factor};

impl DataProvider<UnitsInfoV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsInfoV1Marker>, DataError> {
        self.check_req::<UnitsInfoV1Marker>(_req)?;

        // Get all the constants in the form of a map from constant name to constant value as numerator and denominator.
        let units_data: &cldr_serde::units::units_constants::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;

        let constants = &units_data.supplemental.unit_constants.constants;

        let clean_constants_map = process_constants(constants)?;

        // Get all the units and their conversion information.
        let quantities = &units_data.supplemental.unit_quantities.quantities;
        let convert_units = &units_data.supplemental.convert_units.convert_units;

        let mut conversion_info_map = BTreeMap::<&str, UnitsInfoIndex>::new();
        let mut quantity_vec = Vec::<Dimension>::new();
        for (unit_name, quantity) in quantities {
            let quantity_simplicity = match quantity.status.as_deref() {
                Some("simple") => DerivationSpecifier::Base,
                _ => DerivationSpecifier::Derived,
            };

            let quantity_value = quantity.quantity.as_str();
            let quantity_index = quantity_vec.len();
            quantity_vec.push(Dimension {
                quantity: Cow::Borrowed(quantity_value),
                specifier: quantity_simplicity,
            });

            let units_info_index = UnitsInfoIndex {
                dimension: Some(quantity_index as u16),
                convert_info: None,
            };

            conversion_info_map.insert(unit_name.as_str(), units_info_index);
        }

        let mut convert_units_vec = Vec::<ConversionInfo>::new();
        for (unit_name, convert_unit) in convert_units {
            let base_unit = convert_unit.base_unit.as_str();
            let factor = match convert_unit.factor {
                Some(ref factor) => factor.as_str(),
                None => "1",
            };
            let offset = match convert_unit.offset {
                Some(ref offset) => offset.as_str(),
                None => "0",
            };

            let factor_scientific = process_factor(factor, &clean_constants_map)?;
            let offset_scientific = process_factor(offset, &clean_constants_map)?;

            let convert_unit_index = convert_units_vec.len();
            convert_units_vec.push(extract_conversion_info(
                base_unit,
                factor_scientific,
                offset_scientific,
            )?);

            if let Some(units_info_index) = conversion_info_map.get_mut(unit_name.as_str()) {
                units_info_index.convert_info = Some(convert_unit_index as u16);
            } else {
                let units_info_index = UnitsInfoIndex {
                    dimension: None,
                    convert_info: Some(convert_unit_index as u16),
                };
                conversion_info_map.insert(unit_name.as_str(), units_info_index);
            }
        }

        let result = UnitsInfoV1 {
            units_info: ZeroMap::from_iter(conversion_info_map),
            unit_dimensions: VarZeroVec::from(&quantity_vec),
            convert_infos: VarZeroVec::from(&convert_units_vec),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProvider<UnitsInfoV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

#[test]
fn test_basic() {
    use icu_locid::locale;
    use icu_provider::prelude::*;
    use icu_unitsconversion::provider::*;
    use num_bigint::BigUint;
    use zerofrom::ZeroFrom;
    use zerovec::maps::ZeroVecLike;

    let provider = crate::DatagenProvider::new_testing();

    let und: DataPayload<UnitsInfoV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("und").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let units_info = und.get().to_owned();
    let units_info_map = &units_info.units_info;
    let unit_quantity = &units_info.unit_dimensions;
    let convert_units = &units_info.convert_infos;

    let meter = units_info_map.get("meter").unwrap();
    let meter_quantity_index = meter.dimension.get().unwrap().as_unsigned_int() as usize;
    let meter_quantity = unit_quantity.zvl_get(meter_quantity_index).unwrap();
    assert_eq!(&meter_quantity.quantity, "length");
    // TODO: how to test this?
    // assert_eq!(meter_quantity.constant_exactness as u8, QuantitySimplicity::Simple as u8);

    let big_one = BigUint::from(1u32);

    let meter_convert_index = meter.convert_info.get().unwrap().as_unsigned_int() as usize;
    let meter_convert_ule = convert_units.zvl_get(meter_convert_index).unwrap();
    let meter_convert: ConversionInfo = ZeroFrom::zero_from(meter_convert_ule);

    assert_eq!(meter_convert.factor_sign, Sign::Positive);
    assert_eq!(
        meter_convert,
        ConversionInfo {
            base_unit: Cow::Borrowed("meter"),
            factor_sign: Sign::Positive,
            factor_num: ZeroVec::from(big_one.to_bytes_le()),
            factor_den: ZeroVec::from(big_one.to_bytes_le()),
            offset_sign: Sign::Positive,
            offset_num: ZeroVec::from(BigUint::zero().to_bytes_le()),
            offset_den: ZeroVec::from(big_one.to_bytes_le()),
            exactness: Exactness::Exact,
        }
    );

    let foot = units_info_map.get("foot").unwrap();
    let foot_convert_index = foot.convert_info.get().unwrap().as_unsigned_int() as usize;
    let foot_convert_ule = convert_units.zvl_get(foot_convert_index).unwrap();
    let foot_convert: ConversionInfo = ZeroFrom::zero_from(foot_convert_ule);
    let ft_to_m = GenericFraction::<BigUint>::new(BigUint::from(3048u32), BigUint::from(10000u32));

    assert_eq!(
        foot_convert,
        ConversionInfo {
            base_unit: Cow::Borrowed("meter"),
            factor_sign: Sign::Positive,
            factor_num: ZeroVec::from(ft_to_m.numer().unwrap().to_bytes_le()),
            factor_den: ZeroVec::from(ft_to_m.denom().unwrap().to_bytes_le()),
            offset_sign: Sign::Positive,
            offset_num: ZeroVec::from(BigUint::zero().to_bytes_le()),
            offset_den: ZeroVec::from(big_one.to_bytes_le()),
            exactness: Exactness::Exact,
        }
    );

    // TODO: add more tests
}
