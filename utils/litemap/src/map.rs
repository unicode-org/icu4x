// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use std::borrow::Borrow;
use std::mem;
use std::ops::{Index, IndexMut};

/// A simple "flat" map based on a sorted vector
///
/// See the [module level documentation][super] for why one should use this.
///
/// The API is roughly similar to that of [`std::collections::HashMap`], though it
/// requires `Ord` instead of `Hash`.
#[derive(Clone, Debug)]
pub struct LiteMap<K, V> {
    values: Vec<(K, V)>,
}

impl<K, V> LiteMap<K, V> {
    /// Construct a new [`LiteMap`]
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of elements in the [`LiteMap`]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Remove all elements from the [`LiteMap`]
    pub fn clear(&mut self) {
        self.values.clear()
    }

    /// Reserve capacity for `additional` more elements to be inserted into
    /// the [`LiteMap`] to avoid frequent reallocations.
    ///
    /// See [`Vec::reserve()`] for more information.
    pub fn reserve(&mut self, additional: usize) {
        self.values.reserve(additional)
    }
}

impl<K: Ord, V> LiteMap<K, V> {
    /// Get the value associated with `key`, if it exists.
    ///
    /// ```rust
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new();
    /// map.insert(1, "one");
    /// map.insert(2, "two");
    /// assert_eq!(map.get(&1), Some(&"one"));
    /// assert_eq!(map.get(&3), None);
    /// ```
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match self.values.binary_search_by(|k| k.0.borrow().cmp(&key)) {
            Ok(found) => Some(&self.values[found].1),
            Err(_) => None,
        }
    }

    /// Returns whether `key` is contained in this map
    ///
    /// ```rust
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new();
    /// map.insert(1, "one");
    /// map.insert(2, "two");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&3), false);
    /// ```
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.values
            .binary_search_by(|k| k.0.borrow().cmp(&key))
            .is_ok()
    }

    /// Get the value associated with `key`, if it exists, as a mutable reference.
    ///
    /// ```rust
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new();
    /// map.insert(1, "one");
    /// map.insert(2, "two");
    /// if let Some(mut v) = map.get_mut(&1) {
    ///     *v = "uno";   
    /// }
    /// assert_eq!(map.get(&1), Some(&"uno"));
    /// ```
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match self.values.binary_search_by(|k| k.0.borrow().cmp(&key)) {
            Ok(found) => Some(&mut self.values[found].1),
            Err(_) => None,
        }
    }

    /// Appends `value` with `key` to the end of the underlying vector, returning
    /// `value` _if it failed_. Useful for extending with an existing sorted list.
    ///
    /// ```rust
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new();
    /// assert!(map.try_append(1, "uno").is_none());
    /// assert!(map.try_append(3, "tres").is_none());
    /// // out of order append:
    /// assert!(map.try_append(2, "dos").is_some());
    ///
    /// assert_eq!(map.get(&1), Some(&"uno"));
    /// // not appended since it wasn't in order
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn try_append(&mut self, key: K, value: V) -> Option<V> {
        if let Some(ref last) = self.values.last() {
            if last.0 > key {
                return Some(value);
            }
        }

        self.values.push((key, value));
        None
    }

    /// Insert `value` with `key`, returning the existing value if it exists.
    ///
    /// ```rust
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new();
    /// map.insert(1, "one");
    /// map.insert(2, "two");
    /// assert_eq!(map.get(&1), Some(&"one"));
    /// assert_eq!(map.get(&3), None);
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.values.binary_search_by(|k| k.0.cmp(&key)) {
            Ok(found) => Some(mem::replace(&mut self.values[found].1, value)),
            Err(ins) => {
                self.values.insert(ins, (key, value));
                None
            }
        }
    }

    /// Remove the value at `key`, returning it if it exists.
    ///
    /// ```rust
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new();
    /// map.insert(1, "one");
    /// map.insert(2, "two");
    /// assert_eq!(map.remove(&1), Some("one"));
    /// assert_eq!(map.get(&1), None);
    /// ```
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match self.values.binary_search_by(|k| k.0.borrow().cmp(key)) {
            Ok(found) => Some(self.values.remove(found).1),
            Err(_) => None,
        }
    }
}

impl<K, V> Default for LiteMap<K, V> {
    fn default() -> Self {
        LiteMap { values: vec![] }
    }
}
impl<K: Ord, V> Index<&'_ K> for LiteMap<K, V> {
    type Output = V;
    fn index(&self, key: &K) -> &V {
        self.get(key).expect("LiteMap could not find key")
    }
}
impl<K: Ord, V> IndexMut<&'_ K> for LiteMap<K, V> {
    fn index_mut(&mut self, key: &K) -> &mut V {
        self.get_mut(key).expect("LiteMap could not find key")
    }
}

impl<K, V> LiteMap<K, V> {
    /// Produce an ordered iterator over key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.values.iter().map(|val| (&val.0, &val.1))
    }

    /// Produce an ordered iterator over keys
    pub fn iter_keys(&self) -> impl Iterator<Item = &K> {
        self.values.iter().map(|val| &val.0)
    }

    /// Produce an iterator over values, ordered by their keys
    pub fn iter_values(&self) -> impl Iterator<Item = &V> {
        self.values.iter().map(|val| &val.1)
    }

    /// Produce an ordered mutable iterator over key-value pairs
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.values.iter_mut().map(|val| (&val.0, &mut val.1))
    }
}
