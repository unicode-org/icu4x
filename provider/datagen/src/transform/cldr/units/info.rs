// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{borrow::Cow, collections::BTreeMap};

use crate::transform::cldr::cldr_serde::{self};
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use icu_unitsconversion::info_provider::{
    ConvertUnits, QuantitySimplicity, UnitQuantity, UnitsInfoIndex, UnitsInfoV1, UnitsInfoV1Marker,
};
use zerovec::{VarZeroVec, ZeroMap};

impl DataProvider<UnitsInfoV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsInfoV1Marker>, DataError> {
        self.check_req::<UnitsInfoV1Marker>(_req)?;

        let units_data: &cldr_serde::units::units_constants::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;
        let quantities = &units_data.supplemental.unit_quantities.quantities;
        let convert_units = &units_data.supplemental.convert_units.convert_units;

        let mut conversion_info_map = BTreeMap::<&str, UnitsInfoIndex>::new();
        let mut quantity_vec = Vec::<UnitQuantity>::new();
        for (unit_name, quantity) in quantities {
            let quantity_simplicity = match quantity.status.as_deref() {
                Some("simple") => QuantitySimplicity::Simple,
                _ => QuantitySimplicity::Complex,
            };

            let quantity_value = quantity.quantity.as_str();
            let quantity_index = quantity_vec.len();
            quantity_vec.push(UnitQuantity {
                quantity: Cow::Borrowed(quantity_value),
                constant_exactness: quantity_simplicity,
            });

            let units_info_index = UnitsInfoIndex {
                quantity: Some(quantity_index as u16),
                convert_unit: None,
            };

            conversion_info_map.insert(unit_name.as_str(), units_info_index);
        }

        let mut convert_units_vec = Vec::<ConvertUnits>::new();
        for (unit_name, convert_unit) in convert_units {
            let base_unit = convert_unit.base_unit.as_str();
            let factor = match convert_unit.factor {
                Some(ref factor) => factor.as_str(),
                None => "1",
            };

            let convert_unit_index = convert_units_vec.len();
            convert_units_vec.push(ConvertUnits {
                base_unit: Cow::Borrowed(base_unit),
                factor: Cow::Borrowed(factor),
            });

            if let Some(units_info_index) = conversion_info_map.get_mut(unit_name.as_str()) {
                units_info_index.convert_unit = Some(convert_unit_index as u16);
            } else {
                let units_info_index = UnitsInfoIndex {
                    quantity: None,
                    convert_unit: Some(convert_unit_index as u16),
                };
                conversion_info_map.insert(unit_name.as_str(), units_info_index);
            }
        }

        let result = UnitsInfoV1 {
            units_info: ZeroMap::from_iter(conversion_info_map),
            unit_quantity: VarZeroVec::from(&quantity_vec),
            convert_units: VarZeroVec::from(&convert_units_vec),
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
    let unit_quantity = &units_info.unit_quantity;
    let convert_units = &units_info.convert_units;

    let meter = units_info_map.get("meter").unwrap();
    let meter_quantity_index = meter.quantity.get().unwrap().as_unsigned_int() as usize;
    let meter_quantity = unit_quantity.zvl_get(meter_quantity_index).unwrap();
    assert_eq!(&meter_quantity.quantity, "length");
    // TODO: how to test this?
    // assert_eq!(meter_quantity.constant_exactness as u8, QuantitySimplicity::Simple as u8);

    let meter_convert_index = meter.convert_unit.get().unwrap().as_unsigned_int() as usize;
    let meter_convert = convert_units.zvl_get(meter_convert_index).unwrap();
    assert_eq!(meter_convert.base_unit(), "meter");
    assert_eq!(meter_convert.factor(), "1");

    let foot = units_info_map.get("foot").unwrap();
    let foot_convert_index = foot.convert_unit.get().unwrap().as_unsigned_int() as usize;
    let foot_convert = convert_units.zvl_get(foot_convert_index).unwrap();
    assert_eq!(foot_convert.base_unit(), "meter");
    assert_eq!(foot_convert.factor(), "ft_to_m");

    // TODO: add more tests
}
