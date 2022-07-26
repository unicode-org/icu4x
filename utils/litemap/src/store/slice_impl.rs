// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;

impl<'a, K: 'a + ?Sized, V: 'a + ?Sized> Store<K, V> for &'a [(&'a K, &'a V)] {
    #[inline]
    fn lm_len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn lm_is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn lm_get(&self, index: usize) -> Option<(&K, &V)> {
        self.get(index).copied()
    }

    #[inline]
    fn lm_last(&self) -> Option<(&K, &V)> {
        self.last().copied()
    }

    #[inline]
    fn lm_binary_search_by<F>(&self, mut cmp: F) -> Result<usize, usize>
    where
        F: FnMut(&K) -> Ordering,
    {
        self.binary_search_by(|(k, _)| cmp(k))
    }
}

impl<'a, K: 'a, V: 'a> StoreIterable<'a, K, V> for &'a [(&'a K, &'a V)] {
    type KeyValueIter = core::iter::Copied<core::slice::Iter<'a, (&'a K, &'a V)>>;

    #[inline]
    fn lm_iter(&'a self) -> Self::KeyValueIter {
        self.iter().copied()
    }
}
