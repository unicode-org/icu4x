// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod vec_impl;

use core::cmp::Ordering;
use core::iter::DoubleEndedIterator;
use core::iter::FromIterator;
use core::iter::Iterator;

pub trait Store<K, V> {
    fn lm_with_capacity(capacity: usize) -> Self;
    fn lm_len(&self) -> usize;
    fn lm_is_empty(&self) -> bool;

    fn lm_get(&self, index: usize) -> Option<(&K, &V)>;
    fn lm_get_mut(&mut self, index: usize) -> Option<(&K, &mut V)>;
    fn lm_last(&self) -> Option<(&K, &V)>;

    fn lm_binary_search_by<F>(&self, cmp: F) -> Result<usize, usize>
    where
        F: FnMut(&K) -> Ordering;
    fn lm_push(&mut self, key: K, value: V);
    fn lm_insert(&mut self, index: usize, key: K, value: V);
    fn lm_remove(&mut self, index: usize) -> Option<(K, V)>;
    fn lm_extend_end(&mut self, other: Self);
    fn lm_extend_start(&mut self, other: Self);

    fn lm_clear(&mut self);
    fn lm_reserve(&mut self, additional: usize);
    fn lm_retain<F>(&mut self, predicate: F)
    where
        F: FnMut(&K, &V) -> bool;
}

pub trait StoreIterable<'a, K: 'a, V: 'a>: Store<K, V> {
    type KeyValueIter: Iterator<Item = (&'a K, &'a V)> + DoubleEndedIterator + 'a;
    type KeyValueIterMut: Iterator<Item = (&'a K, &'a mut V)> + DoubleEndedIterator + 'a;
    type KeyValueIntoIter: Iterator<Item = (K, V)>;

    fn lm_iter(&'a self) -> Self::KeyValueIter;
    fn lm_iter_mut(&'a mut self) -> Self::KeyValueIterMut;
    fn lm_into_iter(self) -> Self::KeyValueIntoIter;
}

pub trait StoreFromIterator<K, V>: FromIterator<(K, V)> {}
