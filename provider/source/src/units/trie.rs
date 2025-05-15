// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::cldr_serde;
use crate::cldr_serde::units::info::ConvertUnit;
use crate::SourceDataProvider;
use icu::experimental::measure::provider::ids::RESERVED_UNIT_IDS;
use icu::experimental::measure::provider::trie::{UnitsTrie, UnitsTrieV1};
use icu_provider::prelude::*;
use zerotrie::ZeroTrieSimpleAscii;

impl DataProvider<UnitsTrieV1> for SourceDataProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsTrieV1>, DataError> {
        self.check_req::<UnitsTrieV1>(_req)?;

        // Get all the constants in the form of a map from constant name to constant value as numerator and denominator.
        let units_data: &cldr_serde::units::info::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;

        // Get all the units and their conversion information.
        let convert_units = &units_data.supplemental.convert_units.convert_units;

        // Maps from unit id to its id which will be use to get the conversion information from the `UnitsInfo`.
        let mut trie_map = BTreeMap::<Vec<u8>, usize>::new();

        // Pre-process the conversion information to convert the factor and offset to scientific notation.
        // This used to get the id for each unit which is used to get the conversion information from the `UnitsInfo`.
        let mut convert_units_vec = Vec::<&ConvertUnit>::new();

        // First add the reserved unit ids to the trie with their corresponding index in the `convert_units_vec`.
        for cldr_id in RESERVED_UNIT_IDS {
            let convert_unit_index = convert_units_vec.len();
            let convert_unit = convert_units.get(*cldr_id).ok_or_else(|| {
                DataError::custom("Could not get convert unit from units.json data, the convert unit id in the reserved unit ids must be found in the convert units")
            })?;
            convert_units_vec.push(convert_unit);
            trie_map.insert(cldr_id.as_bytes().to_vec(), convert_unit_index);
        }

        // Then add the rest of the units to the trie with their corresponding index in the `convert_units_vec`.
        for (unit_name, convert_unit) in convert_units {
            if RESERVED_UNIT_IDS.contains(&unit_name.as_str()) {
                continue;
            }
            let convert_unit_index = convert_units_vec.len();
            convert_units_vec.push(convert_unit);
            trie_map.insert(unit_name.as_bytes().to_vec(), convert_unit_index);
        }

        let units_conversion_trie = ZeroTrieSimpleAscii::try_from(&trie_map).map_err(|e| {
            DataError::custom("Could not create ZeroTrie from units.json data")
                .with_display_context(&e)
        })?;

        let result = UnitsTrie {
            trie: units_conversion_trie.convert_store(),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<UnitsTrieV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[test]
fn test_basic() {
    use icu::experimental::units::provider::UnitsInfoV1;
    use icu::locale::langid;
    use icu_provider::prelude::*;

    let provider = SourceDataProvider::new_testing();

    let und_trie: DataResponse<UnitsTrieV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("und").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    let units_info = und_trie.payload.get().to_owned();
    let trie = &units_info.trie;

    let und_info: DataResponse<UnitsInfoV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("und").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    let units_info = und_info.payload.get().to_owned();
    let meter_id = trie.get("meter").unwrap();
    let foot_id = trie.get("foot").unwrap();
    let convert_infos = &units_info.conversion_info[meter_id];
    let basic_units = &convert_infos.basic_units();

    // Meter should have the same id as the one in the info because it is the `root` unit for length.
    assert_eq!(basic_units.first().unwrap().unit_id, meter_id as u16);

    let convert_infos = &units_info.conversion_info[foot_id];
    let basic_units = &convert_infos.basic_units();
    // Foot should have the same id as meter because meter is the root unit for length.
    assert_eq!(basic_units.first().unwrap().unit_id, meter_id as u16);
}
