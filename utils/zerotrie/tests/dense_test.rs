// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerotrie::ZeroTrieSimpleAscii;
use zerovec::VarZeroVec;

fn s2t<'a>(strings: impl Iterator<Item = &'a str>) -> impl Iterator<Item = (&'a str, usize)> {
    strings.enumerate().map(|(i, s)| (s, i))
}

#[test]
fn test_issue_sizes() {
    static KEYS: &[&str] = &[
        "ar/FR", "ar/IR", "ar/SA", "ar/UK", "ar/US", "en/AU", "en/FR", "en/UK", "en/US", "fr/FR",
        "fr/SA", "fr/UK", "fr/US", "it/IT",
    ];

    let sparse_zerotrie = ZeroTrieSimpleAscii::from_iter(s2t(KEYS.iter().copied()));
    assert_eq!(sparse_zerotrie.byte_len(), 71);

    let sparse_vzv = VarZeroVec::<str>::from(KEYS);
    assert_eq!(sparse_vzv.as_bytes().len(), 98);

    static DENSE_LANGS: &[&str] = &["ar", "en", "fr"];
    static DENSE_REGIONS: &[&str] = &["FR", "SA", "UK", "US"];

    let dense_lang_zerotrie = ZeroTrieSimpleAscii::from_iter(s2t(DENSE_LANGS.iter().copied()));
    assert_eq!(dense_lang_zerotrie.byte_len(), 12);

    let dense_region_zerotrie = ZeroTrieSimpleAscii::from_iter(s2t(DENSE_REGIONS.iter().copied()));
    assert_eq!(dense_region_zerotrie.byte_len(), 16);

    let hybrid_keys = KEYS
        .iter()
        .filter(|key| {
            for lang in DENSE_LANGS.iter() {
                for region in DENSE_REGIONS.iter() {
                    if writeable::cmp_str(&(lang, '/', region), key).is_eq() {
                        return false;
                    }
                }
            }
            true
        })
        .collect::<Vec<_>>();
    assert_eq!(hybrid_keys.len(), 3);

    let hybrid_zerotrie = ZeroTrieSimpleAscii::from_iter(s2t(hybrid_keys
        .iter()
        .copied()
        .chain(DENSE_LANGS.iter())
        .copied()));
    assert_eq!(hybrid_zerotrie.byte_len(), 27);
}
