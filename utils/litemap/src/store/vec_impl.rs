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
}

impl<K: Ord, V> StoreFromIterable<K, V> for Vec<(K, V)> {
    fn lm_sort_from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut v = Self::new();
        v.lm_extend(iter);
        v
    }
}

impl<K: Ord, V> SortedStoreFromStore<K, V> for Vec<(K, V)> {
    fn lm_sorted_store_from_store(mut self) -> Self {
        sort_and_dedup_in_place(&mut self);
        self
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

    /// Extends this store with items from an iterator.
    #[inline]
    fn lm_extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
        K: Ord,
    {
        // From HashBrown's implementation:
        // Keys may be already present or show multiple times in the iterator.
        // Reserve the entire hint lower bound if the map is empty (e.g. from_iter).
        // Otherwise reserve half the hint (rounded up), so the map
        // will only resize twice in the worst case (assuming growth by doubling).
        let mut iter = iter.into_iter();
        let (size_hint_lower, _) = iter.size_hint();
        let reserve = if self.is_empty() {
            size_hint_lower
        } else {
            (size_hint_lower + 1) / 2
        };
        self.reserve_exact(reserve);

        // First extend self with the new elements.
        // Fast path: if all new elements are greater than the previous elements, including existing items.
        let mut fast_path = true;
        while let Some((key, value)) = iter.next() {
            fast_path = self.last().map_or(true, |l| key > l.0);
            self.push((key, value));
            if !fast_path {
                self.extend(iter);
                break;
            }
        }

        if !fast_path {
            sort_and_dedup_in_place(self);
        }
    }
}

fn sort_and_dedup_in_place<K: Ord, V>(v: &mut Vec<(K, V)>) {
    // Stable sort in order to preserve the order of elements.
    // This way, if there are elements with the same key we'll keep the last value.
    v.sort_by(|a, b| a.0.cmp(&b.0));
    // Then deduplicate elements with the same key by keeping the _last_ value.
    let mut i = 0;
    while i < v.len() {
        #[allow(clippy::indexing_slicing)] // while condition checks if i is in range
        let Some((head, tail)) = v[i..].split_first() else {
            break;
        };
        let equals = tail.iter().take_while(|t| head.0 == t.0).count();
        v.drain(i..i + equals);
        i += 1;
    }
}

impl<K, V> StoreFromIterator<K, V> for Vec<(K, V)> {}

#[test]
fn test_vec_impl() {
    crate::testing::check_store_full::<Vec<(u32, u64)>>();
}

#[test]
fn test_sort_and_dedup_in_place() {
    let mut vec = vec![
        (3, "c"),
        (1, "a"),
        (2, "b"),
        (1, "a2"),
        (3, "c2"),
        (3, "c3"),
    ];
    sort_and_dedup_in_place(&mut vec);
    assert_eq!(vec, vec![(1, "a2"), (2, "b"), (3, "c3")]);

    let mut vec: Vec<(u32, &str)> = Vec::new();
    sort_and_dedup_in_place(&mut vec);
    assert!(vec.is_empty());

    let mut vec = vec![(3, "c"), (1, "a"), (2, "b")];
    sort_and_dedup_in_place(&mut vec);
    assert_eq!(vec, vec![(1, "a"), (2, "b"), (3, "c")]);

    let mut vec = vec![(1, "a"), (1, "a2"), (1, "a3")];
    sort_and_dedup_in_place(&mut vec);
    assert_eq!(vec, vec![(1, "a3")]);
}
