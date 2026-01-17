// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(all(feature = "alloc", feature = "dense"))]

use std::collections::BTreeMap;
use zerotrie::dense::ZeroAsciiDenseSparse2dTrieOwned;

#[test]
fn test_dense_suffix_count_with_low_frequency_suffixes() {
    // Create a dataset where many suffixes appear in only one prefix.
    // This should result in fewer dense suffixes than if we included all.

    let mut data: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();

    // Common suffixes that appear in multiple prefixes
    data.entry("en").or_default().insert("US", 1);
    data.entry("en").or_default().insert("GB", 2);
    data.entry("en").or_default().insert("CA", 3);

    data.entry("fr").or_default().insert("FR", 4);
    data.entry("fr").or_default().insert("CA", 5);
    data.entry("fr").or_default().insert("BE", 6);

    data.entry("de").or_default().insert("DE", 7);
    data.entry("de").or_default().insert("AT", 8);
    data.entry("de").or_default().insert("CH", 9);

    // Low-frequency suffixes (appear in only one prefix each)
    data.entry("en").or_default().insert("rare1", 100);
    data.entry("fr").or_default().insert("rare2", 101);
    data.entry("de").or_default().insert("rare3", 102);
    data.entry("es").or_default().insert("rare4", 103);
    data.entry("es").or_default().insert("ES", 104);

    let trie = ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data, b'/').unwrap();
    let trie = trie.as_borrowed();

    // With filtering, only CA should be in dense (appears in 2/4 = 50% of prefixes)
    // The rare suffixes should be in sparse only

    // Verify CA is accessible (high frequency)
    assert_eq!(trie.get("en", "CA"), Some(3));
    assert_eq!(trie.get("fr", "CA"), Some(5));

    // Verify rare suffixes are still accessible (via sparse)
    assert_eq!(trie.get("en", "rare1"), Some(100));
    assert_eq!(trie.get("fr", "rare2"), Some(101));
    assert_eq!(trie.get("de", "rare3"), Some(102));
    assert_eq!(trie.get("es", "rare4"), Some(103));

    // Verify other combinations work
    assert_eq!(trie.get("en", "US"), Some(1));
    assert_eq!(trie.get("de", "CH"), Some(9));
    assert_eq!(trie.get("es", "ES"), Some(104));

    // Non-existent lookups
    assert_eq!(trie.get("en", "FR"), None);
    assert_eq!(trie.get("nonexistent", "US"), None);
}

#[test]
fn test_empty_dense_when_no_suffix_meets_threshold() {
    // Create a dataset where NO suffix appears in enough prefixes to meet threshold.
    // Dense matrix should be empty, all lookups use sparse path.

    let mut data: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();

    // Each suffix appears in only one prefix (0% overlap)
    data.entry("a").or_default().insert("s1", 1);
    data.entry("b").or_default().insert("s2", 2);
    data.entry("c").or_default().insert("s3", 3);

    let trie = ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data, b'/').unwrap();
    let trie = trie.as_borrowed();

    // With empty dense matrix, all lookups should still work via sparse path
    assert_eq!(trie.get("a", "s1"), Some(1));
    assert_eq!(trie.get("b", "s2"), Some(2));
    assert_eq!(trie.get("c", "s3"), Some(3));

    // Non-existent
    assert_eq!(trie.get("a", "s2"), None);
    assert_eq!(trie.get("b", "s3"), None);
}

#[test]
fn test_empty_input() {
    let data: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();

    let trie = ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data, b'/').unwrap();
    let trie = trie.as_borrowed();

    // Should handle empty input gracefully
    assert_eq!(trie.get("any", "thing"), None);
}

#[test]
fn test_lookup_semantics_remain_correct() {
    // Verify that both dense and sparse suffixes return correct values

    let mut data: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();

    // High-frequency suffix (will be dense)
    for i in 0..10 {
        let prefix = format!("p{}", i);
        data.entry(prefix.leak() as &str)
            .or_default()
            .insert("common", 100 + i);
    }

    // Low-frequency suffix (will be sparse)
    data.entry("p0").or_default().insert("rare", 999);

    let trie = ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data, b'/').unwrap();
    let trie = trie.as_borrowed();

    // Dense suffix lookups
    for i in 0..10 {
        let prefix = format!("p{}", i);
        assert_eq!(trie.get(&prefix, "common"), Some(100 + i));
    }

    // Sparse suffix lookup
    assert_eq!(trie.get("p0", "rare"), Some(999));

    // Non-existent combinations
    assert_eq!(trie.get("p1", "rare"), None);
    assert_eq!(trie.get("p99", "common"), None);
}

#[test]
fn test_delimiter_validation() {
    let mut data: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();

    // Try to use delimiter in suffix
    data.entry("test").or_default().insert("bad/suffix", 1);

    let result = ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data, b'/');
    assert!(result.is_err());

    // Try to use delimiter in prefix
    let mut data2: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    data2.entry("bad/prefix").or_default().insert("ok", 1);

    let result2 = ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data2, b'/');
    assert!(result2.is_err());
}
