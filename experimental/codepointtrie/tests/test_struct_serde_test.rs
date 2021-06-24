// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use toml::Value;

mod test_util;

#[test]
fn code_point_trie_test_data_check_test() {
    test_util::run_deserialize_test_from_test_data("tests/testdata/free-blocks.8.toml");
    test_util::run_deserialize_test_from_test_data("tests/testdata/free-blocks.16.toml");
    test_util::run_deserialize_test_from_test_data("tests/testdata/free-blocks.32.toml");
    test_util::run_deserialize_test_from_test_data("tests/testdata/free-blocks.small16.toml");
    test_util::run_deserialize_test_from_test_data("tests/testdata/grow-data.16.toml");
}
