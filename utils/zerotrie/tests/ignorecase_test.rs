// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerotrie::ZeroAsciiIgnoreCaseTrie;
use zerotrie::ByteStr;

mod testdata {
    use zerotrie::ByteStr;
    include!("data/data.rs");
}

use testdata::strings_to_litemap;

#[test]
fn test_ignore_case_coverage() {
    let litemap = strings_to_litemap(&["", "aBc", "aBcD", "aBce", "aBcF", "aBcghi"]);

    // Test both construction paths
    ZeroAsciiIgnoreCaseTrie::try_from(&litemap).unwrap();
    let trie = litemap
        .iter()
        .map(|(k, v)| (k.as_bytes(), *v))
        .collect::<ZeroAsciiIgnoreCaseTrie<Vec<u8>>>();

    // Test lookup
    for (k, v) in litemap.iter() {
        assert_eq!(trie.get(k.as_bytes()), Some(*v), "normal: {k:?}");
        let k_upper = k
                .as_bytes()
            .iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<u8>>();
        assert_eq!(trie.get(k_upper), Some(*v), "upper: {k:?}");
        let k_lower = k
            .as_bytes()
            .iter()
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<u8>>();
        assert_eq!(trie.get(k_lower), Some(*v), "lower: {k:?}");
    }

    // Test mixed-case strings
    let problematic_strs = &["A", "ab", "abc", "aBcd", "aBcgHi"];
    for problematic_str in problematic_strs {
        let mut litemap = litemap.clone();
        litemap.insert(ByteStr::from_str(problematic_str), 100);
        ZeroAsciiIgnoreCaseTrie::try_from(&litemap).expect_err(problematic_str);
    }
}
