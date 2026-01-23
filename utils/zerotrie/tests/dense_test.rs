// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use itertools::Either;
use itertools::Itertools;
use rand::Rng;
use rand::SeedableRng;
use std::collections::BTreeMap;
use zerotrie::dense::ZeroAsciiDenseSparse2dTrieBorrowed;
use zerotrie::dense::ZeroAsciiDenseSparse2dTrieOwned;
use zerotrie::dense::ZeroAsciiDenseSparse2dTrieVarULE;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::VarZeroCow;

mod testdata {
    include!("data/data.rs");
}

const BASIC_DATA: &[&str] = &[
    "ar/FR", "ar/IR", "ar/SA", "ar/UK", "ar/US", "en/AU", "en/FR", "en/UK", "en/US", "fr/FR",
    "fr/SA", "fr/UK", "fr/US", "it/IT",
];

const NON_EXISTENT_BASIC_DATA: &[&str] = &["ar/IT", "en/ZA", "it/FR", "zh/UK"];

fn strings_to_2d_btreemap<'a>(
    strings: &[&'a str],
    delimiter: &str,
) -> BTreeMap<&'a str, BTreeMap<&'a str, usize>> {
    let mut map = BTreeMap::<&str, BTreeMap<&str, usize>>::default();
    for (value, (prefix, suffix)) in strings
        .iter()
        .flat_map(|s| s.split_once(delimiter))
        .enumerate()
    {
        map.entry(prefix).or_default().insert(suffix, value);
    }
    map
}

fn check_data(
    data: &BTreeMap<&str, BTreeMap<&str, usize>>,
    trie: ZeroAsciiDenseSparse2dTrieBorrowed,
    polarity: bool,
) {
    for (prefix, values) in data.iter() {
        for (suffix, value) in values.iter() {
            if polarity {
                assert_eq!(trie.get(prefix, suffix), Some(*value), "{prefix}/{suffix}");
            } else {
                assert_eq!(trie.get(prefix, suffix), None, "{prefix}/{suffix}");
            }
        }
    }
}

#[must_use]
fn check_encoding(trie: ZeroAsciiDenseSparse2dTrieBorrowed) -> usize {
    let ule =
        VarZeroCow::<ZeroAsciiDenseSparse2dTrieVarULE>::from_encodeable(&trie.as_encodeable());
    let decoded = ZeroAsciiDenseSparse2dTrieBorrowed::from(&*ule);
    assert_eq!(decoded, trie);
    ule.as_bytes().len()
}

#[must_use]
fn make_simple_ascii_trie(
    data: &BTreeMap<&str, BTreeMap<&str, usize>>,
) -> ZeroTrieSimpleAscii<Vec<u8>> {
    let mut data_for_simple_ascii_trie = BTreeMap::new();
    for (prefix, values) in data.iter() {
        for (suffix, value) in values.iter() {
            data_for_simple_ascii_trie.insert(format!("{prefix}/{suffix}"), *value);
        }
    }
    ZeroTrieSimpleAscii::try_from_btree_map_str(&data_for_simple_ascii_trie).unwrap()
}

#[test]
fn test_basic() {
    let data_as_map = strings_to_2d_btreemap(BASIC_DATA, "/");
    let dense =
        ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data_as_map, b'/').unwrap();
    check_data(&data_as_map, dense.as_borrowed(), true);
    let non_existent_data_as_map = strings_to_2d_btreemap(NON_EXISTENT_BASIC_DATA, "/");
    check_data(&non_existent_data_as_map, dense.as_borrowed(), false);
    let byte_len = check_encoding(dense.as_borrowed());
    assert_eq!(byte_len, 106);
    let simple_trie = make_simple_ascii_trie(&data_as_map);
    assert_eq!(simple_trie.byte_len(), 71);
}

#[test]
fn test_locales_with_aux() {
    let data_as_map = strings_to_2d_btreemap(testdata::locales_with_aux::STRINGS, "-x-");
    let dense =
        ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data_as_map, b'/').unwrap();
    check_data(&data_as_map, dense.as_borrowed(), true);
    let byte_len = check_encoding(dense.as_borrowed());
    assert_eq!(byte_len, 4045);
    let simple_trie = make_simple_ascii_trie(&data_as_map);
    assert_eq!(simple_trie.byte_len(), 4614);
}

#[test]
fn test_short_subtags() {
    let (prefixes, suffixes): (Vec<&str>, Vec<&str>) = testdata::short_subtags::STRINGS
        .iter()
        .copied()
        .partition_map(|s| match s.strip_prefix("und-") {
            Some(suffix) => Either::Right(suffix),
            None => Either::Left(s),
        });
    assert_eq!(prefixes.len(), 1427);
    assert_eq!(suffixes.len(), 450);
    let mut counter = 0;
    let mut rng = rand_pcg::Lcg64Xsh32::seed_from_u64(2025);
    let mut data: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    for prefix in prefixes.iter() {
        // Two types of keys: big and small. Pick at random with a certain chance of big.
        let is_big = rng.random::<f64>() < 0.3;
        for suffix in suffixes.iter() {
            // Include the suffix with a probability dependent on is_big.
            let include_value = rng.random::<f64>() < if is_big { 0.8 } else { 0.05 };
            if include_value {
                data.entry(prefix).or_default().insert(suffix, counter);
                counter += 1;
            }
        }
    }
    assert_eq!(data.values().map(|m| m.len()).sum::<usize>(), 175306);
    let dense = ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data, b'/').unwrap();
    check_data(&data, dense.as_borrowed(), true);
    let byte_len = check_encoding(dense.as_borrowed());
    assert_eq!(byte_len, 555240);
    let simple_trie = make_simple_ascii_trie(&data);
    assert_eq!(simple_trie.byte_len(), 1099634);
}

#[test]
fn test_dense_sparse_window_selection() {
    //Make initial Btree (not using enumerate)
    let row_width = usize::from(u16::MAX); // Densetype max
    let far_low = 0;
    let cluster_start = 50;
    let far_high = cluster_start + row_width + 100;

    let mut inner = BTreeMap::new();
    inner.insert("low", far_low);
    inner.insert("a", 50); // cluster_start
    inner.insert("c", 53); // cluster_start + 3
    inner.insert("d", cluster_start + row_width - 3);
    inner.insert("e", cluster_start + row_width - 2);
    inner.insert("f", cluster_start + row_width - 1);
    inner.insert("c2", 53); // cluster_start + 3
    inner.insert("g", cluster_start + row_width);
    inner.insert("h", cluster_start + row_width + 1);
    inner.insert("b", 52); // cluster_start + 2
    inner.insert("high", far_high);
    inner.insert("c3", 53); // cluster_start + 3
    inner.insert("low2", far_low);

    let mut data = BTreeMap::new();
    data.insert("p", inner);

    // Build the 2d trie.
    let dense = ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data, b'/').unwrap();
    let trie = dense.as_borrowed();

    check_data(&data, trie, true);

    let byte_len = check_encoding(dense.as_borrowed());
    assert_eq!(byte_len, 102);
    let simple_trie = make_simple_ascii_trie(&data);
    assert_eq!(simple_trie.byte_len(), 60);
}
