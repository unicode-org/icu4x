// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use super::*;

impl<K, V> Store<K, V> for Vec<(K, V)> {
    fn with_capacity(len: usize) -> Self {
        todo!()
    }
    fn len(&self) -> usize {
        todo!()
    }
    fn is_empty(&self) -> bool {
        todo!()
    }

    fn get(&self, index: usize) -> Option<(&K, &V)> {
        todo!()
    }
    fn get_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        todo!()
    }
    fn last(&self) -> Option<(&K, &V)> {
        todo!()
    }

    fn binary_search_by<F>(&self, cmp: F) -> Result<usize, usize>
    where
        F: FnMut((&K, &V)) -> Ordering {
            todo!()
        }
    fn push(&mut self, key: K, value: V) {
        todo!()
    }
    fn insert(&mut self, index: usize, key: K, value: V) {
        todo!()
    }
    fn remove(&mut self, index: usize) -> Option<(K, V)> {
        todo!()
    }
    fn extend_end(&mut self, other: Self) {
        todo!()
    }
    fn extend_start(&mut self, other: Self) {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }
    fn reserve(&mut self, additional: usize) {
        todo!()
    }
    fn retain<F>(&mut self, predicate: F)
    where
        F: FnMut((&K, &V)) -> bool {
            todo!()
        }
}

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

impl<'a, K: 'a, V: 'a> StoreIterable<'a, K, V> for Vec<(K, V)> {
    type KeyValueIter = core::iter::Map<core::slice::Iter<'a, (K, V)>, MapF<'a, K, V>>;
    type KeyValueIterMut = core::iter::Map<core::slice::IterMut<'a, (K, V)>, MapFMut<'a, K, V>>;
    type KeyValueIntoIter = alloc::vec::IntoIter<(K, V)>;

    #[inline]
    fn iter(&'a self) -> Self::KeyValueIter {
        self.as_slice().iter().map(map_f)
    }
    #[inline]
    fn iter_mut(&'a mut self) -> Self::KeyValueIterMut {
        self.as_mut_slice().iter_mut().map(map_f_mut)
    }
    #[inline]
    fn into_iter(self) -> Self::KeyValueIntoIter {
        IntoIterator::into_iter(self)
    }
}

impl<K, V> StoreFromIterator<K, V> for Vec<(K, V)> {}
