// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;

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

impl<K, V, T> StoreBase<K, V> for T
where
    T: StoreSlice<K, V>,
{
    #[inline]
    fn lm_len(&self) -> usize {
        self.lm_as_slice().len()
    }

    #[inline]
    fn lm_is_empty(&self) -> bool {
        self.lm_as_slice().is_empty()
    }

    #[inline]
    fn lm_get(&self, index: usize) -> Option<(&K, &V)> {
        self.lm_as_slice().get(index).map(map_f)
    }

    #[inline]
    fn lm_get_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.lm_as_mut_slice().get_mut(index).map(map_f_mut)
    }

    #[inline]
    fn lm_last(&self) -> Option<(&K, &V)> {
        self.lm_as_slice().last().map(map_f)
    }

    #[inline]
    fn lm_binary_search_by<F>(&self, mut cmp: F) -> Result<usize, usize>
    where
        F: FnMut(&K) -> Ordering,
    {
        self.lm_as_slice().binary_search_by(|(k, _)| cmp(&k))
    }
}

impl<'a, K: 'a, V: 'a, T> StoreIterable<'a, K, V> for T
where
    T: StoreSlice<K, V> + Store<K, V>,
{
    type KeyValueIter = core::iter::Map<core::slice::Iter<'a, (K, V)>, MapF<'a, K, V>>;
    type KeyValueIterMut = core::iter::Map<core::slice::IterMut<'a, (K, V)>, MapFMut<'a, K, V>>;

    #[inline]
    fn lm_iter(&'a self) -> Self::KeyValueIter {
        self.lm_as_slice().iter().map(map_f)
    }

    #[inline]
    fn lm_iter_mut(&'a mut self) -> Self::KeyValueIterMut {
        self.lm_as_mut_slice().iter_mut().map(map_f_mut)
    }
}
