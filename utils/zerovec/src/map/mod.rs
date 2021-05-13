// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! See [`ZeroMap`] for details.

use crate::ule::AsULE;
use crate::ZeroVec;
use std::cmp::Ordering;

mod kv;
#[cfg(feature = "serde")]
mod serde;
mod vecs;

pub use kv::ZeroMapKV;
pub use vecs::ZeroVecLike;

/// A zero-copy map datastructure, built on sorted binary-searchable [`ZeroVec`]
/// and [`VarZeroVec`](crate::VarZeroVec).
///
/// This type, like [`ZeroVec`] and [`VarZeroVec`](crate::VarZeroVec), is able to zero-copy
/// deserialize from appropriately formatted byte buffers. It is internally copy-on-write, so it can be mutated
/// afterwards as necessary.
pub struct ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
{
    keys: K::Container,
    values: V::Container,
}

impl<'a, K, V> Default for ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
{
    fn default() -> Self {
        Self {
            keys: K::Container::new(),
            values: V::Container::new(),
        }
    }
}

impl<'a, K, V> ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
{
    /// Construct a new [`ZeroMap`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a new [`ZeroMap`] with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keys: K::Container::with_capacity(capacity),
            values: V::Container::with_capacity(capacity),
        }
    }

    /// The number of elements in the [`ZeroMap`]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the [`ZeroMap`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.len() == 0
    }

    /// Remove all elements from the [`ZeroMap`]
    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }

    /// Reserve capacity for `additional` more elements to be inserted into
    /// the [`ZeroMap`] to avoid frequent reallocations.
    ///
    /// See [`Vec::reserve()`] for more information.
    pub fn reserve(&mut self, additional: usize) {
        self.keys.reserve(additional);
        self.values.reserve(additional);
    }

    /// Get the value associated with `key`, if it exists.
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(1, "one".to_owned());
    /// map.insert(2, "two".to_owned());
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
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(1, "one".to_owned());
    /// map.insert(2, "two".to_owned());
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&3), false);
    /// ```
    pub fn contains_key(&self, key: &K::NeedleType) -> bool {
        self.keys.binary_search(key).is_ok()
    }

    /// Insert `value` with `key`, returning the existing value if it exists.
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(1, "one".to_owned());
    /// map.insert(2, "two".to_owned());
    /// assert_eq!(map.get(&1), Some("one"));
    /// assert_eq!(map.get(&3), None);
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let key_needle = key.as_needle();
        match self.keys.binary_search(key_needle) {
            Ok(index) => Some(self.values.replace(index, value)),
            Err(index) => {
                self.keys.insert(index, key);
                self.values.insert(index, value);
                None
            }
        }
    }

    /// Remove the value at `key`, returning it if it exists.
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(1, "one".to_owned());
    /// map.insert(2, "two".to_owned());
    /// assert_eq!(map.remove(&1), Some("one".to_owned()));
    /// assert_eq!(map.get(&1), None);
    /// ```
    pub fn remove(&mut self, key: &K::NeedleType) -> Option<V> {
        let idx = self.keys.binary_search(key).ok()?;
        self.keys.remove(idx);
        Some(self.values.remove(idx))
    }

    /// Appends `value` with `key` to the end of the underlying vector, returning
    /// `key` and `value` _if it failed_. Useful for extending with an existing
    /// sorted list.
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// assert!(map.try_append(1, "uno".to_owned()).is_none());
    /// assert!(map.try_append(3, "tres".to_owned()).is_none());
    ///
    /// let unsuccessful = map.try_append(3, "tres-updated".to_owned());
    /// assert!(unsuccessful.is_some(), "append duplicate of last key");
    ///
    /// let unsuccessful = map.try_append(2, "dos".to_owned());
    /// assert!(unsuccessful.is_some(), "append out of order");
    ///
    /// assert_eq!(map.get(&1), Some("uno"));
    ///
    /// // contains the original value for the key: 3
    /// assert_eq!(map.get(&3), Some("tres"));
    ///
    /// // not appended since it wasn't in order
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[must_use]
    pub fn try_append(&mut self, key: K, value: V) -> Option<(K, V)> {
        if self.keys.len() != 0 {
            if let Some(last) = self.keys.get(self.keys.len() - 1) {
                if key.cmp_get(last) != Ordering::Greater {
                    return Some((key, value));
                }
            }
        }

        self.keys.push(key);
        self.values.push(value);
        None
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

impl<'a, K, V> ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>>,
    V: AsULE + Copy,
{
    /// For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`
    pub fn get_copied(&self, key: &K::NeedleType) -> Option<V> {
        let index = self.keys.binary_search(key).ok()?;
        ZeroVec::get(&self.values, index)
    }

    /// Similar to [`Self::iter()`] except it returns a direct copy of the values instead of references
    /// to `V::ULE`, in cases when `V` is fixed-size
    pub fn iter_copied_values<'b>(
        &'b self,
    ) -> impl Iterator<Item = (&'b <K as ZeroMapKV<'a>>::GetType, V)> {
        (0..self.keys.len()).map(move |idx| {
            (
                self.keys.get(idx).unwrap(),
                ZeroVec::get(&self.values, idx).unwrap(),
            )
        })
    }
}

impl<'a, K, V> ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a, Container = ZeroVec<'a, K>>,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>>,
    K: AsULE + Copy,
    V: AsULE + Copy,
{
    /// Similar to [`Self::iter()`] except it returns a direct copy of the keys values instead of references
    /// to `K::ULE` and `V::ULE`, in cases when `K` and `V` are fixed-size
    #[allow(clippy::needless_lifetimes)] // Lifetime is necessary in impl Trait
    pub fn iter_copied<'b>(&'b self) -> impl Iterator<Item = (K, V)> + 'b {
        let keys = &self.keys;
        let values = &self.values;
        (0..keys.len()).map(move |idx| {
            (
                ZeroVec::get(keys, idx).unwrap(),
                ZeroVec::get(values, idx).unwrap(),
            )
        })
    }
}
