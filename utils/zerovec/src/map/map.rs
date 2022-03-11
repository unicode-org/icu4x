// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::ule::AsULE;
use crate::{ZeroSlice, ZeroVec};
use alloc::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::iter::FromIterator;

/// A zero-copy map datastructure, built on sorted binary-searchable [`ZeroVec`]
/// and [`VarZeroVec`].
///
/// This type, like [`ZeroVec`] and [`VarZeroVec`], is able to zero-copy
/// deserialize from appropriately formatted byte buffers. It is internally copy-on-write, so it can be mutated
/// afterwards as necessary.
///
/// Internally, a `ZeroMap` is a zero-copy vector for keys paired with a zero-copy vector for
/// values, sorted by the keys. Therefore, all types used in `ZeroMap` need to work with either
/// [`ZeroVec`] or [`VarZeroVec`].
///
/// # Examples
///
/// ```
/// use zerovec::ZeroMap;
///
/// // Example byte buffer representing the map { 1: "one" }
/// let BINCODE_BYTES: &[u8; 31] = &[
///     4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0,
///     1, 0, 0, 0, 0, 0, 0, 0, 111, 110, 101
/// ];
///
/// // Deserializing to ZeroMap requires no heap allocations.
/// let zero_map: ZeroMap<u32, str> = bincode::deserialize(BINCODE_BYTES)
///     .expect("Should deserialize successfully");
/// assert_eq!(zero_map.get(&1), Some("one"));
/// ```
///
/// [`VarZeroVec`]: crate::VarZeroVec
pub struct ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    pub(crate) keys: K::Container,
    pub(crate) values: V::Container,
}

impl<'a, K, V> Default for ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K, V> ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    /// Creates a new, empty `ZeroMap<K, V>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::ZeroMap;
    ///
    /// let zm: ZeroMap<u16, str> = ZeroMap::new();
    /// assert!(zm.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            keys: K::Container::zvl_new(),
            values: V::Container::zvl_new(),
        }
    }

    /// Construct a new [`ZeroMap`] with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keys: K::Container::zvl_with_capacity(capacity),
            values: V::Container::zvl_with_capacity(capacity),
        }
    }

    /// Obtain a borrowed version of this map
    pub fn as_borrowed(&'a self) -> ZeroMapBorrowed<'a, K, V> {
        ZeroMapBorrowed {
            keys: self.keys.zvl_as_borrowed(),
            values: self.values.zvl_as_borrowed(),
        }
    }

    /// The number of elements in the [`ZeroMap`]
    pub fn len(&self) -> usize {
        self.values.zvl_len()
    }

    /// Whether the [`ZeroMap`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.zvl_len() == 0
    }

    /// Remove all elements from the [`ZeroMap`]
    pub fn clear(&mut self) {
        self.keys.zvl_clear();
        self.values.zvl_clear();
    }

    /// Reserve capacity for `additional` more elements to be inserted into
    /// the [`ZeroMap`] to avoid frequent reallocations.
    ///
    /// See [`Vec::reserve()`](alloc::vec::Vec::reserve) for more information.
    pub fn reserve(&mut self, additional: usize) {
        self.keys.zvl_reserve(additional);
        self.values.zvl_reserve(additional);
    }

    /// Get the value associated with `key`, if it exists.
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(&1, "one");
    /// map.insert(&2, "two");
    /// assert_eq!(map.get(&1), Some("one"));
    /// assert_eq!(map.get(&3), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V::GetType> {
        let index = self.keys.zvl_binary_search(key).ok()?;
        self.values.zvl_get(index)
    }

    /// Binary search the map with `predicate` to find a key, returning the value.
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(&1, "one");
    /// map.insert(&2, "two");
    /// assert_eq!(map.get_by(|probe| probe.cmp(&1)), Some("one"));
    /// assert_eq!(map.get_by(|probe| probe.cmp(&3)), None);
    /// ```
    pub fn get_by(&self, predicate: impl FnMut(&K) -> Ordering) -> Option<&V::GetType> {
        let index = self.keys.zvl_binary_search_by(predicate).ok()?;
        self.values.zvl_get(index)
    }

    /// Returns whether `key` is contained in this map
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(&1, "one");
    /// map.insert(&2, "two");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&3), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.keys.zvl_binary_search(key).is_ok()
    }

    /// Insert `value` with `key`, returning the existing value if it exists.
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(&1, "one");
    /// map.insert(&2, "two");
    /// assert_eq!(map.get(&1), Some("one"));
    /// assert_eq!(map.get(&3), None);
    /// ```
    pub fn insert(&mut self, key: &K, value: &V) -> Option<V::OwnedType> {
        match self.keys.zvl_binary_search(key) {
            Ok(index) => Some(self.values.zvl_replace(index, value)),
            Err(index) => {
                self.keys.zvl_insert(index, key);
                self.values.zvl_insert(index, value);
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
    /// map.insert(&1, "one");
    /// map.insert(&2, "two");
    /// assert_eq!(map.remove(&1), Some("one".to_owned().into_boxed_str()));
    /// assert_eq!(map.get(&1), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V::OwnedType> {
        let idx = self.keys.zvl_binary_search(key).ok()?;
        self.keys.zvl_remove(idx);
        Some(self.values.zvl_remove(idx))
    }

    /// Appends `value` with `key` to the end of the underlying vector, returning
    /// `key` and `value` _if it failed_. Useful for extending with an existing
    /// sorted list.
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap::new();
    /// assert!(map.try_append(&1, "uno").is_none());
    /// assert!(map.try_append(&3, "tres").is_none());
    ///
    /// let unsuccessful = map.try_append(&3, "tres-updated");
    /// assert!(unsuccessful.is_some(), "append duplicate of last key");
    ///
    /// let unsuccessful = map.try_append(&2, "dos");
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
    pub fn try_append<'b>(&mut self, key: &'b K, value: &'b V) -> Option<(&'b K, &'b V)> {
        if self.keys.zvl_len() != 0 {
            if let Some(last) = self.keys.zvl_get(self.keys.zvl_len() - 1) {
                if K::Container::t_cmp_get(key, last) != Ordering::Greater {
                    return Some((key, value));
                }
            }
        }

        self.keys.zvl_push(key);
        self.values.zvl_push(value);
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
        (0..self.keys.zvl_len()).map(move |idx| {
            (
                #[allow(clippy::unwrap_used)] // TODO(#1688) Clippy exceptions need docs or fixing.
                self.keys.zvl_get(idx).unwrap(),
                #[allow(clippy::unwrap_used)] // TODO(#1688) Clippy exceptions need docs or fixing.
                self.values.zvl_get(idx).unwrap(),
            )
        })
    }

    /// Produce an ordered iterator over keys
    pub fn iter_keys<'b>(&'b self) -> impl Iterator<Item = &'b <K as ZeroMapKV<'a>>::GetType> {
        #[allow(clippy::unwrap_used)] // TODO(#1688) Clippy exceptions need docs or fixing.
        (0..self.keys.zvl_len()).map(move |idx| self.keys.zvl_get(idx).unwrap())
    }

    /// Produce an iterator over values, ordered by keys
    pub fn iter_values<'b>(&'b self) -> impl Iterator<Item = &'b <V as ZeroMapKV<'a>>::GetType> {
        #[allow(clippy::unwrap_used)] // TODO(#1688) Clippy exceptions need docs or fixing.
        (0..self.values.zvl_len()).map(move |idx| self.values.zvl_get(idx).unwrap())
    }
}

impl<'a, K, V> ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
    V: AsULE + Copy,
{
    /// For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`
    pub fn get_copied(&self, key: &K) -> Option<V> {
        let index = self.keys.zvl_binary_search(key).ok()?;
        ZeroSlice::get(&*self.values, index)
    }

    /// Similar to [`Self::iter()`] except it returns a direct copy of the values instead of references
    /// to `V::ULE`, in cases when `V` is fixed-size
    pub fn iter_copied_values<'b>(
        &'b self,
    ) -> impl Iterator<Item = (&'b <K as ZeroMapKV<'a>>::GetType, V)> {
        (0..self.keys.zvl_len()).map(move |idx| {
            (
                #[allow(clippy::unwrap_used)] // TODO(#1688) Clippy exceptions need docs or fixing.
                self.keys.zvl_get(idx).unwrap(),
                #[allow(clippy::unwrap_used)] // TODO(#1688) Clippy exceptions need docs or fixing.
                ZeroSlice::get(&*self.values, idx).unwrap(),
            )
        })
    }
}

impl<'a, K, V> ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a, Container = ZeroVec<'a, K>> + ?Sized,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
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
                #[allow(clippy::unwrap_used)] // TODO(#1688) Clippy exceptions need docs or fixing.
                ZeroSlice::get(&**keys, idx).unwrap(),
                #[allow(clippy::unwrap_used)] // TODO(#1688) Clippy exceptions need docs or fixing.
                ZeroSlice::get(&**values, idx).unwrap(),
            )
        })
    }
}

impl<'a, K, V> From<ZeroMapBorrowed<'a, K, V>> for ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K: ?Sized,
    V: ?Sized,
{
    fn from(other: ZeroMapBorrowed<'a, K, V>) -> Self {
        Self {
            keys: K::Container::zvl_from_borrowed(other.keys),
            values: V::Container::zvl_from_borrowed(other.values),
        }
    }
}

// We can't use the default PartialEq because ZeroMap is invariant
// so otherwise rustc will not automatically allow you to compare ZeroMaps
// with different lifetimes
impl<'a, 'b, K, V> PartialEq<ZeroMap<'b, K, V>> for ZeroMap<'a, K, V>
where
    K: for<'c> ZeroMapKV<'c> + ?Sized,
    V: for<'c> ZeroMapKV<'c> + ?Sized,
    <K as ZeroMapKV<'a>>::Container: PartialEq<<K as ZeroMapKV<'b>>::Container>,
    <V as ZeroMapKV<'a>>::Container: PartialEq<<V as ZeroMapKV<'b>>::Container>,
{
    fn eq(&self, other: &ZeroMap<'b, K, V>) -> bool {
        self.keys.eq(&other.keys) && self.values.eq(&other.values)
    }
}

impl<'a, K, V> fmt::Debug for ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <K as ZeroMapKV<'a>>::Container: fmt::Debug,
    <V as ZeroMapKV<'a>>::Container: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("ZeroMap")
            .field("keys", &self.keys)
            .field("values", &self.values)
            .finish()
    }
}

impl<'a, K, V> Clone for ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <K as ZeroMapKV<'a>>::Container: Clone,
    <V as ZeroMapKV<'a>>::Container: Clone,
{
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
            values: self.values.clone(),
        }
    }
}

impl<'a, A, B, K, V> FromIterator<(A, B)> for ZeroMap<'a, K, V>
where
    A: Borrow<K>,
    B: Borrow<V>,
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (A, B)>,
    {
        let iter = iter.into_iter();
        let mut map = match iter.size_hint() {
            (_, Some(upper)) => Self::with_capacity(upper),
            (lower, None) => Self::with_capacity(lower),
        };

        for (key, value) in iter {
            if let Some((key, value)) = map.try_append(key.borrow(), value.borrow()) {
                map.insert(key, value);
            }
        }
        map
    }
}
