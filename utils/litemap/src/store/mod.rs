// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod vec_impl;

use core::cmp::Ordering;
use core::iter::DoubleEndedIterator;
use core::iter::FromIterator;
use core::iter::Iterator;

pub trait Store<K, V> {
    fn with_capacity(len: usize) -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn get(&self, index: usize) -> Option<(&K, &V)>;
    fn get_mut(&mut self, index: usize) -> Option<(&K, &mut V)>;
    fn last(&self) -> Option<(&K, &V)>;

    fn binary_search_by<F>(&self, cmp: F) -> Result<usize, usize>
    where
        F: FnMut((&K, &V)) -> Ordering;
    fn push(&mut self, key: K, value: V);
    fn insert(&mut self, index: usize, key: K, value: V);
    fn remove(&mut self, index: usize) -> Option<(K, V)>;
    fn extend_end(&mut self, other: Self);
    fn extend_start(&mut self, other: Self);

    fn clear(&mut self);
    fn reserve(&mut self, additional: usize);
    fn retain<F>(&mut self, predicate: F)
    where
        F: FnMut((&K, &V)) -> bool;
}

pub trait StoreIterable<'a, K: 'a, V: 'a>: Store<K, V> {
    type KeyValueIter: Iterator<Item = (&'a K, &'a V)> + DoubleEndedIterator + 'a;
    type KeyValueIterMut: Iterator<Item = (&'a K, &'a mut V)> + DoubleEndedIterator + 'a;
    type KeyValueIntoIter: Iterator<Item = (K, V)>;

    fn iter(&'a self) -> Self::KeyValueIter;
    fn iter_mut(&'a mut self) -> Self::KeyValueIterMut;
    fn into_iter(self) -> Self::KeyValueIntoIter;
}

pub trait StoreFromIterator<K, V>: FromIterator<(K, V)> {}
