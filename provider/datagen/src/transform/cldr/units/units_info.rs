// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{
    borrow::Cow,
    collections::{BTreeMap, VecDeque},
};

use crate::transform::cldr::{
    cldr_serde::{self, units::units_constants::ConvertUnit},
    units::helpers::is_scientific_number,
};
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use icu_unitsconversion::{
    provider::{ConstantExactness, ConstantValue, UnitsConstantsV1, UnitsConstantsV1Marker},
    units_provider::{
        ConvertUnits, QuantitySimplicity, UnitQuantity, UnitsInfoIndex, UnitsInfoV1Marker, UnitsInfoV1, UnitQuantityULE, ConvertUnitsULE,
    },
};
use zerovec::{ZeroMap, VarZeroVec};

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

        let result = 
            UnitsInfoV1 {
                units_info: ZeroMap::from_iter(conversion_info_map.into_iter()),
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
