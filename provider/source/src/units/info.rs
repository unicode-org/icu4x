// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::SourceDataProvider;
use crate::{cldr_serde, units::helpers::ScientificNumber};
use icu::experimental::measure::parser::MeasureUnitParser;
use icu::experimental::measure::provider::trie::UnitsTrie;
use icu::experimental::units::provider::{ConversionInfo, UnitsInfo, UnitsInfoV1};
use icu_provider::prelude::*;
use icu_provider_adapters::fixed::FixedProvider;
use zerotrie::ZeroTrieSimpleAscii;
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
            base_unit: &'a str,
            factor_scientific: ScientificNumber,
            offset_scientific: ScientificNumber,
        }

        // Initialize a vector to store pre-processed conversion information for `MeasureUnitParser`.
        let mut convert_units_vec =
            Vec::with_capacity(units_data.supplemental.convert_units.convert_units.len());

        // Initialize a map to associate unit names with their corresponding index in `convert_units_vec`.
        let mut conversion_info_map = BTreeMap::new();

        // Process the unit constants to remove any constants that are in string format.
        let clean_constants_map =
            process_constants(&units_data.supplemental.unit_constants.constants)?;

        // Iterate over all the units and their conversion information.
        for (unit_name, convert_unit) in &units_data.supplemental.convert_units.convert_units {
            let base_unit = convert_unit.base_unit.as_str();
            let factor = convert_unit.factor.as_deref().unwrap_or("1");
            let offset = convert_unit.offset.as_deref().unwrap_or("0");

            let convert_unit_index = convert_units_vec.len();
            convert_units_vec.push(ConversionInfoPreProcessing {
                base_unit,
                factor_scientific: process_factor(factor, &clean_constants_map)?,
                offset_scientific: process_factor(offset, &clean_constants_map)?,
            });

            conversion_info_map.insert(unit_name.as_bytes().to_vec(), convert_unit_index);
        }

        // TODO: remove this once we can use the `try_new_with_buffer_provider` constructor in `components/experimental/src/measure/parser.rs`.
        // OR just using `MeasureUnitParser::default()`
        let units_conversion_trie =
            ZeroTrieSimpleAscii::try_from(&conversion_info_map).map_err(|e| {
                DataError::custom("Could not create ZeroTrie from units.json data")
                    .with_display_context(&e)
            })?;

        // Convert the trie to use ZeroVec and wrap it in UnitsTrie
        let units_trie = UnitsTrie {
            trie: units_conversion_trie.convert_store(),
        };

        let parser = MeasureUnitParser::try_new_unstable(&FixedProvider::from_owned(units_trie))?;

        let conversion_info = convert_units_vec
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
    use icu::experimental::measure::provider::si_prefix::{Base, SiPrefix};
    use icu::experimental::measure::provider::single_unit::SingleUnit;
    use icu::experimental::measure::provider::trie::UnitsTrieV1;
    use icu::experimental::units::provider::*;
    use icu::locale::langid;
    use icu_provider::prelude::*;
    use num_bigint::BigUint;
    use num_rational::Ratio;
    use zerofrom::ZeroFrom;
    use zerovec::maps::ZeroVecLike;
    use zerovec::ZeroVec;

    let provider = SourceDataProvider::new_testing();

    let und: DataResponse<UnitsInfoV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("und").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    let und_trie: DataResponse<UnitsTrieV1> = provider.load(Default::default()).unwrap();

    let units_info = und.payload.get().to_owned();
    let units_info_map = &und_trie.payload.get().trie;
    let conversion_info = &units_info.conversion_info;

    let meter_index = units_info_map.get("meter").unwrap();

    let big_one = BigUint::from(1u32);

    let meter_convert_ule = conversion_info.zvl_get(meter_index).unwrap();
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
                let base_unit = vec![SingleUnit {
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
    let foot_convert_ule = conversion_info.zvl_get(foot_convert_index).unwrap();
    let foot_convert: ConversionInfo = ZeroFrom::zero_from(foot_convert_ule);
    let ft_to_m = Ratio::new(BigUint::from(3048u32), BigUint::from(10000u32));

    assert_eq!(
        foot_convert,
        ConversionInfo {
            basic_units: {
                let base_unit = vec![SingleUnit {
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
