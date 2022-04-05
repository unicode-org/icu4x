// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::store::{Store, StoreFromIterator, StoreIterable};
use crate::LiteMap;
use alloc::vec::Vec;
use core::fmt::Debug;

fn check_equivalence<K, V, S0, S1>(mut a: S0, mut b: S1)
where
    K: Ord + Debug + PartialEq,
    V: Debug + PartialEq,
    S0: Store<K, V>,
    S1: Store<K, V>,
{
    let len = a.lm_len();
    assert_eq!(len, b.lm_len());
    if len == 0 {
        assert!(a.lm_is_empty());
        assert!(b.lm_is_empty());
    }
    for i in 0..len {
        let a_kv = a.lm_get(i);
        let b_kv = b.lm_get(i);
        assert!(a_kv.is_some());
        assert_eq!(a_kv, b_kv);
        let a_kv_mut = a.lm_get_mut(i);
        let b_kv_mut = b.lm_get_mut(i);
        assert!(a_kv_mut.is_some());
        assert_eq!(a_kv_mut, b_kv_mut);
    }
    for j in 0..len {
        let needle = a.lm_get(j).unwrap().0;
        let a_binary = a.lm_binary_search_by(|k| k.cmp(needle));
        let b_binary = a.lm_binary_search_by(|k| k.cmp(needle));
        assert_eq!(Ok(j), a_binary);
        assert_eq!(Ok(j), b_binary);
    }
    assert!(a.lm_get(len).is_none());
    assert!(b.lm_get(len).is_none());
    assert_eq!(a.lm_last(), b.lm_last());
    let a_vec = a.lm_into_iter().collect::<Vec<_>>();
    let b_vec = b.lm_into_iter().collect::<Vec<_>>();
    assert_eq!(a_vec, b_vec);
}

const SORTED_DATA: &[(u32, u64)] = &[
    (106, 4816),
    (147, 9864),
    (188, 8588),
    (252, 6031),
    (434, 2518),
    (574, 8500),
    (607, 3756),
    (619, 4965),
    (663, 2669),
    (724, 9211),
];

const RANDOM_DATA: &[(u32, u64)] = &[
    (546, 7490),
    (273, 4999),
    (167, 8078),
    (176, 2101),
    (373, 1304),
    (339, 9613),
    (561, 3620),
    (301, 1214),
    (483, 4453),
    (704, 5359),
];

fn populate_litemap<S>(map: &mut LiteMap<u32, u64, S>)
where
    S: Store<u32, u64>,
{
    assert_eq!(0, map.len());
    assert!(map.is_empty());
    for (k, v) in SORTED_DATA.iter() {
        map.try_append(*k, *v).expect("appending sorted data");
    }
    assert_eq!(10, map.len());
    for (k, v) in RANDOM_DATA.iter() {
        map.try_append(*k, *v)
            .ok_or(())
            .expect_err("cannot append random data");
    }
    assert_eq!(10, map.len());
    for (k, v) in RANDOM_DATA.iter() {
        map.insert(*k, *v);
    }
    assert_eq!(20, map.len());
}

pub fn check_litemap<'a, S>(mut litemap_test: LiteMap<u32, u64, S>)
where
    S: Store<u32, u64>
        + StoreIterable<'a, u32, u64>
        + StoreFromIterator<u32, u64>
        + Clone
        + Debug
        + PartialEq,
{
    assert!(litemap_test.is_empty());
    let mut litemap_std = LiteMap::<u32, u64>::new();
    populate_litemap(&mut litemap_test);
    populate_litemap(&mut litemap_std);
    check_equivalence(
        litemap_test.clone().into_tuple_vec(),
        litemap_std.clone().into_tuple_vec(),
    );

    let extras = litemap_test.clone();
    let extras = litemap_test
        .extend_from_litemap(extras)
        .expect("duplicates");
    assert_eq!(extras, litemap_test);
}
