// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::cldr_serde;
use crate::cldr_serde::units::info::ConvertUnit;
use crate::SourceDataProvider;
use icu_experimental::measure::provider::trie::{UnitsTrieV1, UnitsTrieV1Marker};
use icu_provider::prelude::*;
use zerotrie::ZeroTrieSimpleAscii;

impl DataProvider<UnitsTrieV1Marker> for SourceDataProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<UnitsTrieV1Marker>, DataError> {
        self.check_req::<UnitsTrieV1Marker>(_req)?;

        // Get all the constants in the form of a map from constant name to constant value as numerator and denominator.
        let units_data: &cldr_serde::units::info::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/units.json")?;

        // Get all the units and their conversion information.
        let convert_units = &units_data.supplemental.convert_units.convert_units;

        // Maps from unit id to its id which will be use to get the conversion information from the `UnitsInfoV1`.
        let mut trie_map = BTreeMap::<Vec<u8>, usize>::new();

        // Pre-process the conversion information to convert the factor and offset to scientific notation.
        // This used to get the id for each unit which is used to get the conversion information from the `UnitsInfoV1`.
        let mut convert_units_vec = Vec::<&ConvertUnit>::new();
        for (unit_name, convert_unit) in convert_units {
            let convert_unit_index = convert_units_vec.len();
            convert_units_vec.push(convert_unit);
            trie_map.insert(unit_name.as_bytes().to_vec(), convert_unit_index);
        }

        let units_conversion_trie = ZeroTrieSimpleAscii::try_from(&trie_map).map_err(|e| {
            DataError::custom("Could not create ZeroTrie from units.json data")
                .with_display_context(&e)
        })?;

        let result = UnitsTrieV1 {
            trie: units_conversion_trie.convert_store(),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<UnitsTrieV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;
    use icu_provider::prelude::*;

    let provider = SourceDataProvider::new_testing();

    let und: DataResponse<UnitsTrieV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("und").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    let units_info = und.payload.get().to_owned();
    let trie = &units_info.trie;

    assert_eq!(trie.get("meter").unwrap(), 84);

    // TODO: add more tests that test the accessing of UnitsInfoV1Marker and get the correct info for each couple of units
}
