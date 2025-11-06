// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use crate::{cldr_serde, units::helpers::ScientificNumber};
use icu::experimental::measure::provider::single_unit::UnitID;
use icu::experimental::units::provider::{ConversionInfo, UnitsInfo, UnitsInfoV1};
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

use super::helpers::{extract_conversion_info, process_constants, process_factor};

impl DataProvider<UnitsInfoV1> for SourceDataProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsInfoV1>, DataError> {
        self.check_req::<UnitsInfoV1>(_req)?;

        // Load and parse the unit constants from the supplemental data file.
        let units_data: &cldr_serde::units::info::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;

        struct ConversionInfoPreProcessing<'a> {
            unit_id: UnitID,
            base_unit: &'a str,
            factor_scientific: ScientificNumber,
            offset_scientific: ScientificNumber,
        }

        // Initialize a vector to store pre-processed conversion information.
        let mut convert_units_vec =
            Vec::with_capacity(units_data.supplemental.convert_units.convert_units.len());

        // Process the unit constants to remove any constants that are in string format.
        let clean_constants_map =
            process_constants(&units_data.supplemental.unit_constants.constants)?;

        // Iterate over all the units and their conversion information.
        for (unit_name, convert_unit) in &units_data.supplemental.convert_units.convert_units {
            let base_unit = convert_unit.base_unit.as_str();
            let factor = convert_unit.factor.as_deref().unwrap_or("1");
            let offset = convert_unit.offset.as_deref().unwrap_or("0");

            convert_units_vec.push(ConversionInfoPreProcessing {
                unit_id: units_data.unit_id(unit_name)?,
                base_unit,
                factor_scientific: process_factor(factor, &clean_constants_map)?,
                offset_scientific: process_factor(offset, &clean_constants_map)?,
            });
        }

        // Ensure the conversion units are sorted by `unit_id` before the processing.
        convert_units_vec.sort_by_key(|convert_unit| convert_unit.unit_id);

        let conversion_info = convert_units_vec
            .iter()
            .map(|convert_unit| {
                extract_conversion_info(
                    convert_unit.unit_id,
                    convert_unit.base_unit,
                    &convert_unit.factor_scientific,
                    &convert_unit.offset_scientific,
                )
            })
            .collect::<Result<Vec<ConversionInfo>, DataError>>()?;

        let result = UnitsInfo {
            conversion_info: VarZeroVec::from(&conversion_info),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<UnitsInfoV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[test]
fn test_basic() {
    use icu::experimental::measure::parser::ids::CLDR_IDS_TRIE;
    use icu::experimental::measure::provider::si_prefix::{Base, SiPrefix};
    use icu::experimental::measure::provider::single_unit::SingleUnit;
    use icu::experimental::units::provider::*;
    use icu::locale::langid;
    use icu_provider::prelude::*;
    use num_bigint::BigUint;
    use num_rational::Ratio;
    use zerofrom::ZeroFrom;
    use zerovec::ZeroVec;

    let provider = SourceDataProvider::new_testing();

    let und: DataResponse<UnitsInfoV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("und").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    let units_info = und.payload.get().to_owned();

    let meter_index = CLDR_IDS_TRIE.get("meter").unwrap() as UnitID;

    let big_one = BigUint::from(1u32);

    let meter_convert_ule = units_info.conversion_info_by_unit_id(meter_index).unwrap();
    let meter_convert: ConversionInfo = ZeroFrom::zero_from(meter_convert_ule);

    assert_eq!(meter_convert.factor_sign, Sign::Positive);
    let meter_convert_base_unit = meter_convert.basic_units.to_owned();
    assert_eq!(meter_convert_base_unit.get(0).unwrap().unit_id, meter_index);
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
            unit_id: meter_index,
            basic_units: {
                let base_unit = vec![SingleUnit {
                    power: 1,
                    si_prefix: SiPrefix {
                        power: 0,
                        base: Base::Decimal,
                    },
                    unit_id: meter_index,
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

    let foot_index = CLDR_IDS_TRIE.get("foot").unwrap() as UnitID;
    let foot_convert_index = units_info
        .conversion_info_by_unit_id(foot_index)
        .unwrap()
        .unit_id;
    let foot_convert_ule = units_info
        .conversion_info_by_unit_id(foot_convert_index.as_unsigned_int())
        .unwrap();
    let foot_convert: ConversionInfo = ZeroFrom::zero_from(foot_convert_ule);
    let ft_to_m = Ratio::new(BigUint::from(3048u32), BigUint::from(10000u32));

    assert_eq!(
        foot_convert,
        ConversionInfo {
            unit_id: foot_convert_index.as_unsigned_int(),
            basic_units: {
                let base_unit = vec![SingleUnit {
                    power: 1,
                    si_prefix: SiPrefix {
                        power: 0,
                        base: Base::Decimal,
                    },
                    unit_id: meter_index,
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
}
