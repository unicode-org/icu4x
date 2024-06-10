// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::provider::transform::cldr::{
    cldr_serde::{self},
    units::helpers::ScientificNumber,
};
use crate::provider::DatagenProvider;
use icu_experimental::units::{
    measureunit::MeasureUnitParser,
    provider::{ConversionInfo, UnitsInfoV1, UnitsInfoV1Marker},
};
use icu_provider::{datagen::IterableDataProvider, prelude::*};
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::VarZeroVec;

use super::helpers::{extract_conversion_info, process_constants, process_factor};

impl DataProvider<UnitsInfoV1Marker> for DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsInfoV1Marker>, DataError> {
        self.check_req::<UnitsInfoV1Marker>(_req)?;

        // Get all the constants in the form of a map from constant name to constant value as numerator and denominator.
        let units_data: &cldr_serde::units::info::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;

        let constants = &units_data.supplemental.unit_constants.constants;

        let clean_constants_map = process_constants(constants)?;

        // Get all the units and their conversion information.
        let convert_units = &units_data.supplemental.convert_units.convert_units;
        let mut conversion_info_map = BTreeMap::<Vec<u8>, usize>::new();

        struct ConversionInfoPreProcessing<'a> {
            base_unit: &'a str,
            factor_scientific: ScientificNumber,
            offset_scientific: ScientificNumber,
        }

        let mut convert_units_vec = Vec::<ConversionInfoPreProcessing>::new();
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

            let convert_unit_index = convert_units_vec.len();
            convert_units_vec.push(ConversionInfoPreProcessing {
                base_unit,
                factor_scientific: process_factor(factor, &clean_constants_map)?,
                offset_scientific: process_factor(offset, &clean_constants_map)?,
            });

            conversion_info_map.insert(unit_name.as_bytes().to_vec(), convert_unit_index);
        }

        let units_conversion_trie =
            ZeroTrieSimpleAscii::try_from(&conversion_info_map).map_err(|e| {
                DataError::custom("Could not create ZeroTrie from units.json data")
                    .with_display_context(&e)
            })?;

        let parser = MeasureUnitParser::from_payload(units_conversion_trie.as_borrowed());

        let convert_infos = convert_units_vec
            .iter()
            .map(|convert_unit| {
                extract_conversion_info(
                    convert_unit.base_unit,
                    &convert_unit.factor_scientific,
                    &convert_unit.offset_scientific,
                    &parser,
                )
            })
            .collect::<Result<Vec<ConversionInfo>, DataError>>()?;

        let result = UnitsInfoV1 {
            units_conversion_trie: units_conversion_trie.convert_store(),
            convert_infos: VarZeroVec::from(&convert_infos),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProvider<UnitsInfoV1Marker> for DatagenProvider {
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[test]
fn test_basic() {
    use icu_experimental::units::provider::*;
    use icu_locale_core::langid;
    use icu_provider::prelude::*;
    use num_bigint::BigUint;
    use num_rational::Ratio;
    use zerofrom::ZeroFrom;
    use zerovec::maps::ZeroVecLike;
    use zerovec::ZeroVec;

    let provider = DatagenProvider::new_testing();

    let und: DataPayload<UnitsInfoV1Marker> = provider
        .load(DataRequest {
            locale: &langid!("und").into(),
            ..Default::default()
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let units_info = und.get().to_owned();
    let units_info_map = &units_info.units_conversion_trie;
    let convert_units = &units_info.convert_infos;

    let meter_index = units_info_map.get("meter").unwrap();

    let big_one = BigUint::from(1u32);

    let meter_convert_ule = convert_units.zvl_get(meter_index).unwrap();
    let meter_convert: ConversionInfo = ZeroFrom::zero_from(meter_convert_ule);

    assert_eq!(meter_convert.factor_sign, Sign::Positive);
    let meter_convert_base_unit = meter_convert.basic_units.to_owned();
    assert_eq!(
        meter_convert_base_unit.get(0).unwrap().unit_id,
        meter_index as u16
    );
    assert_eq!(
        meter_convert.factor_num,
        ZeroVec::from(big_one.to_bytes_le())
    );
    assert_eq!(
        meter_convert.factor_den,
        ZeroVec::from(big_one.to_bytes_le())
    );
    assert_eq!(
        meter_convert,
        ConversionInfo {
            basic_units: {
                let base_unit = vec![MeasureUnitItem {
                    power: 1,
                    si_prefix: SiPrefix {
                        power: 0,
                        base: Base::Decimal,
                    },
                    unit_id: meter_index as u16,
                }];
                ZeroVec::from_iter(base_unit.into_iter())
            },
            factor_sign: Sign::Positive,
            factor_num: ZeroVec::from(big_one.to_bytes_le()),
            factor_den: ZeroVec::from(big_one.to_bytes_le()),
            offset_sign: Sign::Positive,
            offset_num: ZeroVec::from(BigUint::from(0u32).to_bytes_le()),
            offset_den: ZeroVec::from(big_one.to_bytes_le()),
            exactness: Exactness::Exact,
        }
    );

    let foot_convert_index = units_info_map.get("foot").unwrap();
    let foot_convert_ule = convert_units.zvl_get(foot_convert_index).unwrap();
    let foot_convert: ConversionInfo = ZeroFrom::zero_from(foot_convert_ule);
    let ft_to_m = Ratio::new(BigUint::from(3048u32), BigUint::from(10000u32));

    assert_eq!(
        foot_convert,
        ConversionInfo {
            basic_units: {
                let base_unit = vec![MeasureUnitItem {
                    power: 1,
                    si_prefix: SiPrefix {
                        power: 0,
                        base: Base::Decimal,
                    },
                    unit_id: meter_index as u16,
                }];
                ZeroVec::from_iter(base_unit.into_iter())
            },
            factor_sign: Sign::Positive,
            factor_num: ZeroVec::from(ft_to_m.numer().to_bytes_le()),
            factor_den: ZeroVec::from(ft_to_m.denom().to_bytes_le()),
            offset_sign: Sign::Positive,
            offset_num: ZeroVec::from(BigUint::from(0u32).to_bytes_le()),
            offset_den: ZeroVec::from(big_one.to_bytes_le()),
            exactness: Exactness::Exact,
        }
    );

    // TODO: add more tests
}
