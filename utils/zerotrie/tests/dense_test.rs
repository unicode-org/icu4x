// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;
use zerotrie::dense::DenseSparse2dAsciiWithFixedDelimiterBorrowed;
use zerotrie::dense::DenseSparse2dAsciiWithFixedDelimiterOwned;
use zerotrie::dense::DenseSparse2dAsciiWithFixedDelimiterVarULE;
use zerovec::VarZeroCow;

const SIMPLE_DATA: &[&str] = &[
    "ar/FR", "ar/IR", "ar/SA", "ar/UK", "ar/US", "en/AU", "en/FR", "en/UK", "en/US", "fr/FR",
    "fr/SA", "fr/UK", "fr/US", "it/IT",
];

const NON_EXISTENT_SIMPLE_DATA: &[&str] = &[
    "ar/IT", "en/ZA", "it/FR", "zh/UK"
];

fn strings_to_2d_btreemap<'a>(strings: &[&'a str]) -> BTreeMap<&'a str, BTreeMap<&'a str, usize>> {
    let mut map = BTreeMap::<&str, BTreeMap<&str, usize>>::default();
    for (value, (prefix, suffix)) in strings.iter().flat_map(|s| s.split_once('/')).enumerate() {
        map.entry(prefix).or_default().insert(suffix, value);
    }
    map
}

#[test]
fn test_simple() {
    let data_as_map = strings_to_2d_btreemap(SIMPLE_DATA);
    let dense =
        DenseSparse2dAsciiWithFixedDelimiterOwned::try_from_btree_map_str(&data_as_map, b'/')
            .unwrap();
    for (prefix, values) in data_as_map.iter() {
        for (suffix, value) in values.iter() {
            assert_eq!(
                dense.as_borrowed().get(prefix, suffix),
                Some(*value),
                "{prefix}/{suffix}"
            );
        }
    }
    let non_existent_data_as_map = strings_to_2d_btreemap(&NON_EXISTENT_SIMPLE_DATA);
    for (prefix, values) in non_existent_data_as_map.iter() {
        for (suffix, _) in values.iter() {
            assert_eq!(
                dense.as_borrowed().get(prefix, suffix),
                None,
                "{prefix}/{suffix}"
            );
        }
    }
    let ule = VarZeroCow::<DenseSparse2dAsciiWithFixedDelimiterVarULE>::from_encodeable(
        &dense.as_encodeable(),
    );
    let decoded = DenseSparse2dAsciiWithFixedDelimiterBorrowed::from(&*ule);
    assert_eq!(decoded, dense.as_borrowed());
    assert_eq!(ule.as_bytes().len(), 102);
}
