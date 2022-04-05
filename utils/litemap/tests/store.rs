// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use litemap::LiteMap;
use litemap::testing::check_litemap;
use litemap::store::{Store, StoreIterable, StoreFromIterator};
use std::cmp::Ordering;

/// A Vec wrapper that leverages the default function impls from `Store`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct VecWithDefaults<T>(Vec<T>);

type MapF<'a, K, V> = fn(&'a (K, V)) -> (&'a K, &'a V);

#[inline]
fn map_f<'a, K, V>(input: &'a (K, V)) -> (&'a K, &'a V) {
    (&input.0, &input.1)
}

type MapFMut<'a, K, V> = fn(&'a mut (K, V)) -> (&'a K, &'a mut V);

#[inline]
fn map_f_mut<'a, K, V>(input: &'a mut (K, V)) -> (&'a K, &'a mut V) {
    (&input.0, &mut input.1)
}

impl<K, V> Store<K, V> for VecWithDefaults<(K, V)> {
    type KeyValueIntoIter = std::vec::IntoIter<(K, V)>;

    #[inline]
    fn lm_with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline]
    fn lm_reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    #[inline]
    fn lm_len(&self) -> usize {
        self.0.as_slice().len()
    }

    // leave lm_is_empty as default

    #[inline]
    fn lm_get(&self, index: usize) -> Option<(&K, &V)> {
        self.0.as_slice().get(index).map(map_f)
    }

    #[inline]
    fn lm_get_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.0.as_mut_slice().get_mut(index).map(map_f_mut)
    }

    // leave lm_last as default

    #[inline]
    fn lm_binary_search_by<F>(&self, mut cmp: F) -> Result<usize, usize>
    where
        F: FnMut(&K) -> Ordering,
    {
        self.0.as_slice().binary_search_by(|(k, _)| cmp(&k))
    }

    #[inline]
    fn lm_push(&mut self, key: K, value: V) {
        self.0.push((key, value))
    }

    #[inline]
    fn lm_insert(&mut self, index: usize, key: K, value: V) {
        self.0.insert(index, (key, value))
    }

    #[inline]
    fn lm_remove(&mut self, index: usize) -> (K, V) {
        self.0.remove(index)
    }

    // leave lm_extend_end as default

    // leave lm_extend_start as default

    #[inline]
    fn lm_clear(&mut self) {
        self.0.clear()
    }

    // leave lm_retain as default

    #[inline]
    fn lm_into_iter(self) -> Self::KeyValueIntoIter {
        IntoIterator::into_iter(self.0)
    }
}

impl<'a, K: 'a, V: 'a> StoreIterable<'a, K, V> for VecWithDefaults<(K, V)> {
    type KeyValueIter = core::iter::Map<core::slice::Iter<'a, (K, V)>, MapF<'a, K, V>>;
    type KeyValueIterMut = core::iter::Map<core::slice::IterMut<'a, (K, V)>, MapFMut<'a, K, V>>;

    #[inline]
    fn lm_iter(&'a self) -> Self::KeyValueIter {
        self.0.as_slice().iter().map(map_f)
    }

    #[inline]
    fn lm_iter_mut(&'a mut self) -> Self::KeyValueIterMut {
        self.0.as_mut_slice().iter_mut().map(map_f_mut)
    }
}

impl<A> std::iter::FromIterator<A> for VecWithDefaults<A> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<K, V> StoreFromIterator<K, V> for VecWithDefaults<(K, V)> {}

#[test]
fn test_default_impl() {
    let litemap_test = LiteMap::<u32, u64, VecWithDefaults<(u32, u64)>>::with_capacity(0);
    check_litemap(litemap_test);
}
