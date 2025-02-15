// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::vec::Vec;

type MapF<K, V> = fn(&(K, V)) -> (&K, &V);

#[inline]
fn map_f<K, V>(input: &(K, V)) -> (&K, &V) {
    (&input.0, &input.1)
}

type MapFMut<K, V> = fn(&mut (K, V)) -> (&K, &mut V);

#[inline]
fn map_f_mut<K, V>(input: &mut (K, V)) -> (&K, &mut V) {
    (&input.0, &mut input.1)
}

impl<K, V> StoreConstEmpty<K, V> for Vec<(K, V)> {
    const EMPTY: Vec<(K, V)> = Vec::new();
}

impl<K, V> Store<K, V> for Vec<(K, V)> {
    #[inline]
    fn lm_len(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    fn lm_is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    #[inline]
    fn lm_get(&self, index: usize) -> Option<(&K, &V)> {
        self.as_slice().get(index).map(map_f)
    }

    #[inline]
    fn lm_last(&self) -> Option<(&K, &V)> {
        self.as_slice().last().map(map_f)
    }

    #[inline]
    fn lm_binary_search_by<F>(&self, mut cmp: F) -> Result<usize, usize>
    where
        F: FnMut(&K) -> Ordering,
    {
        self.as_slice().binary_search_by(|(k, _)| cmp(k))
    }
}

impl<K, V> StoreSlice<K, V> for Vec<(K, V)> {
    type Slice = [(K, V)];

    fn lm_get_range(&self, range: Range<usize>) -> Option<&Self::Slice> {
        self.get(range)
    }
}

impl<K, V> StoreMut<K, V> for Vec<(K, V)> {
    #[inline]
    fn lm_with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    #[inline]
    fn lm_reserve(&mut self, additional: usize) {
        self.reserve(additional)
    }

    #[inline]
    fn lm_get_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.as_mut_slice().get_mut(index).map(map_f_mut)
    }

    #[inline]
    fn lm_push(&mut self, key: K, value: V) {
        self.push((key, value))
    }

    #[inline]
    fn lm_insert(&mut self, index: usize, key: K, value: V) {
        self.insert(index, (key, value))
    }

    #[inline]
    fn lm_remove(&mut self, index: usize) -> (K, V) {
        self.remove(index)
    }

    #[inline]
    fn lm_clear(&mut self) {
        self.clear()
    }

    #[inline]
    fn lm_retain<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        self.retain(|(k, v)| predicate(k, v))
    }

    /// Extends this store with items from an iterator.
    ///
    /// It uses a three-pass approach to avoid any potential quadratic costs.
    /// The asymptotic worst case complexity of this method is O((n + m) log(n + m)),
    /// where `n` is the number of elements already in `self` and `m` is the number
    /// of elements in the iterator.
    ///
    /// ```text
    /// Example:
    ///   self:         [1,3,5]
    ///   incoming:     [2,2,4,1,6]
    ///
    /// After First Pass
    ///   self:         [1,3,5,6] <- 6 was directly appended
    ///   out_of_order: [2,2,1,4] <- elements that couldn't be directly appended
    ///
    /// Second Pass
    ///   1) sort out_of_order → [1,2,2,4]
    ///   2) merge into self   → [1,3,5,6,2,4]
    ///                           |       |--> second sorted run starts here but it's deduplicated
    ///                           |--> 1 was already in self and was updated in place
    ///
    /// Third Pass
    ///   sort self → [1,2,3,4,5,6]
    /// ```
    ///
    /// In practice the function can exit early, skipping the second and/or third passes.
    #[inline]
    fn lm_extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
        K: Ord,
    {
        // Pre-allocate space to minimize reallocations
        // From HashBrown's implementation:
        // Keys may be already present or show multiple times in the iterator.
        // Reserve the entire hint lower bound if the map is empty (e.g. from_iter).
        // Otherwise reserve half the hint (rounded up), so the map
        // will only resize twice in the worst case (assuming growth by doubling).
        let iter = iter.into_iter();
        let (size_hint_lower, _) = iter.size_hint();
        let reserve = if self.is_empty() {
            size_hint_lower
        } else {
            (size_hint_lower + 1) / 2
        };
        self.reserve_exact(reserve);

        // Scratch space for elements that can't be directly appended.
        let mut out_of_order: Vec<(K, V)> = Vec::new();

        // First pass: fast-path append or collect out-of-order elements
        for (key, value) in iter {
            // The fast path checks if the incoming key is >= the last key in self.
            match self.last_mut().map(|l| (key.cmp(&l.0), l)) {
                // Empty vector or new key is greater than last: append
                None | Some((Ordering::Greater, _)) => self.push((key, value)),
                // Key equals last: update in place
                Some((Ordering::Equal, l)) => *l = (key, value),
                // Key is less than last: collect for second pass
                // Note: Attempting to update existing keys here hurts performance.
                // Likely because of cache misses during search. We'll perform
                // updates later, after sorting the scratch space.
                _ => out_of_order.push((key, value)),
            }
        }

        // If everything was in order, we're done
        if out_of_order.is_empty() {
            return;
        }

        // Second pass: handle out-of-order elements
        // Elements already in self are already sorted.
        let sorted_len = self.len();
        // Stable sort out_of_order to preserve the order of any repeated keys.
        out_of_order.sort_by(|a, b| a.0.cmp(&b.0));
        // Extend the sorted self with the out_of_order (now sorted).
        // The scratch may have elements that already exist in self that need to be updated.
        // We'll perform the updates in place to avoid a costly deduplication operation in the end.
        // This process may create a second sorted run in self that we'll sort later.
        for pair in out_of_order {
            // We need to look for the key in the sorted section of self but also the last position
            // of self to prevent adding duplicated elements from the scratch space.
            #[allow(clippy::indexing_slicing)]
            if let Some((Ordering::Equal, last)) = self.last_mut().map(|l| (l.0.cmp(&pair.0), l)) {
                *last = pair;
            } else if let Ok(idx) = self[..sorted_len].binary_search_by(|(k, _)| k.cmp(&pair.0)) {
                // `sorted_len` remains valid as we do not remove items from self and only update existing keys in place.
                // Note: Attempting to reduce the range of the search slice hurts performance.
                // Likely due to less predictable memory access.
                self[idx] = pair
            } else {
                self.push(pair);
            }
        }
        // If we pushed new items above we ended up with two sorted runs in self.
        if self.len() != sorted_len {
            // While this could be sort_unstable the stable sort is faster when merging two sorted runs.
            self.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }
}

impl<K: Ord, V> StoreFromIterable<K, V> for Vec<(K, V)> {
    fn lm_sort_from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut v = Self::new();
        v.lm_extend(iter);
        v
    }
}

impl<'a, K: 'a, V: 'a> StoreIterable<'a, K, V> for Vec<(K, V)> {
    type KeyValueIter = core::iter::Map<core::slice::Iter<'a, (K, V)>, MapF<K, V>>;

    #[inline]
    fn lm_iter(&'a self) -> Self::KeyValueIter {
        self.as_slice().iter().map(map_f)
    }
}

impl<'a, K: 'a, V: 'a> StoreIterableMut<'a, K, V> for Vec<(K, V)> {
    type KeyValueIterMut = core::iter::Map<core::slice::IterMut<'a, (K, V)>, MapFMut<K, V>>;

    #[inline]
    fn lm_iter_mut(&'a mut self) -> Self::KeyValueIterMut {
        self.as_mut_slice().iter_mut().map(map_f_mut)
    }
}

impl<K, V> StoreIntoIterator<K, V> for Vec<(K, V)> {
    type KeyValueIntoIter = alloc::vec::IntoIter<(K, V)>;

    #[inline]
    fn lm_into_iter(self) -> Self::KeyValueIntoIter {
        IntoIterator::into_iter(self)
    }

    #[inline]
    fn lm_extend_end(&mut self, other: Self) {
        self.extend(other)
    }

    #[inline]
    fn lm_extend_start(&mut self, other: Self) {
        self.splice(0..0, other);
    }
}

impl<K, V> StoreFromIterator<K, V> for Vec<(K, V)> {}

#[test]
fn test_vec_impl() {
    crate::testing::check_store_full::<Vec<(u32, u64)>>();
}
