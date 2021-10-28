// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::AsULE;
use crate::ZeroVec;
use core::cmp::Ordering;

pub use super::kv::ZeroMapKV;
pub use super::vecs::{BorrowedZeroVecLike, ZeroVecLike};

/// A borrowed-only version of [`ZeroMapBorrowed`](super::ZeroMapBorrowed)
pub struct ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K: ?Sized,
    V: ?Sized,
{
    pub(crate) keys: <<K as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K>>::BorrowedVersion,
    pub(crate) values: <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, V>>::BorrowedVersion,
}

impl<'a, K, V> ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K: ?Sized,
    V: ?Sized,
{
    /// The number of elements in the [`ZeroMapBorrowed`]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the [`ZeroMapBorrowed`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.len() == 0
    }

    /// Get the value associated with `key`, if it exists.
    ///
    /// ```rust
    /// use zerovec::ZeroMapBorrowed;
    ///
    /// let mut map = ZeroMapBorrowed::new();
    /// map.insert(&1, "one");
    /// map.insert(&2, "two");
    /// assert_eq!(map.get(&1), Some("one"));
    /// assert_eq!(map.get(&3), None);
    /// ```
    pub fn get(&self, key: &K::NeedleType) -> Option<&V::GetType> {
        let index = self.keys.binary_search(key).ok()?;
        self.values.get(index)
    }

    /// Returns whether `key` is contained in this map
    ///
    /// ```rust
    /// use zerovec::ZeroMapBorrowed;
    ///
    /// let mut map = ZeroMapBorrowed::new();
    /// map.insert(&1, "one");
    /// map.insert(&2, "two");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&3), false);
    /// ```
    pub fn contains_key(&self, key: &K::NeedleType) -> bool {
        self.keys.binary_search(key).is_ok()
    }

    /// Produce an ordered iterator over key-value pairs
    pub fn iter<'b>(
        &'b self,
    ) -> impl Iterator<
        Item = (
            &'b <K as ZeroMapKV<'a>>::GetType,
            &'b <V as ZeroMapKV<'a>>::GetType,
        ),
    > {
        (0..self.keys.len())
            .map(move |idx| (self.keys.get(idx).unwrap(), self.values.get(idx).unwrap()))
    }

    /// Produce an ordered iterator over keys
    pub fn iter_keys<'b>(&'b self) -> impl Iterator<Item = &'b <K as ZeroMapKV<'a>>::GetType> {
        (0..self.keys.len()).map(move |idx| self.keys.get(idx).unwrap())
    }

    /// Produce an iterator over values, ordered by keys
    pub fn iter_values<'b>(&'b self) -> impl Iterator<Item = &'b <V as ZeroMapKV<'a>>::GetType> {
        (0..self.values.len()).map(move |idx| self.values.get(idx).unwrap())
    }
}

impl<'a, K, V> ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>>,
    V: AsULE + Ord + Copy,
{
    /// For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`
    pub fn get_copied(&self, key: &K::NeedleType) -> Option<V> {
        let index = self.keys.binary_search(key).ok()?;
        <[V::ULE]>::get(&self.values, index)
            .copied()
            .map(V::from_unaligned)
    }

    /// Similar to [`Self::iter()`] except it returns a direct copy of the values instead of references
    /// to `V::ULE`, in cases when `V` is fixed-size
    pub fn iter_copied_values<'b>(
        &'b self,
    ) -> impl Iterator<Item = (&'b <K as ZeroMapKV<'a>>::GetType, V)> {
        (0..self.keys.len()).map(move |idx| {
            (
                self.keys.get(idx).unwrap(),
                <[V::ULE]>::get(&self.values, idx)
                    .copied()
                    .map(V::from_unaligned)
                    .unwrap(),
            )
        })
    }
}

impl<'a, K, V> ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a, Container = ZeroVec<'a, K>>,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>>,
    K: AsULE + Copy + Ord,
    V: AsULE + Copy + Ord,
{
    /// Similar to [`Self::iter()`] except it returns a direct copy of the keys values instead of references
    /// to `K::ULE` and `V::ULE`, in cases when `K` and `V` are fixed-size
    #[allow(clippy::needless_lifetimes)] // Lifetime is necessary in impl Trait
    pub fn iter_copied<'b>(&'b self) -> impl Iterator<Item = (K, V)> + 'b {
        let keys = &self.keys;
        let values = &self.values;
        let len = <[K::ULE]>::len(keys);
        (0..len).map(move |idx| {
            (
                <[K::ULE]>::get(keys, idx)
                    .copied()
                    .map(K::from_unaligned)
                    .unwrap(),
                <[V::ULE]>::get(values, idx)
                    .copied()
                    .map(V::from_unaligned)
                    .unwrap(),
            )
        })
    }
}
