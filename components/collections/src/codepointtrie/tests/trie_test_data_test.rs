// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod test_util;

#[test]
fn code_point_trie_test_data_check_test() {
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/free-blocks.16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/free-blocks.32.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/free-blocks.8.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/free-blocks.small16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/grow-data.16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/grow-data.32.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/grow-data.8.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/grow-data.small16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set1.16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set1.32.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set1.8.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set1.small16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set2-overlap.16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set2-overlap.32.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set2-overlap.small16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set3-initial-9.16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set3-initial-9.32.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set3-initial-9.8.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set3-initial-9.small16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set-empty.16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set-empty.32.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set-empty.8.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set-empty.small16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set-single-value.16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set-single-value.32.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set-single-value.8.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/set-single-value.small16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/short-all-same.16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/short-all-same.8.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/short-all-same.small16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/small0-in-fast.16.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/small0-in-fast.32.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/small0-in-fast.8.toml"));
    test_util::run_deserialize_test_from_test_data(include_str!("testdata/small0-in-fast.small16.toml"));
}
