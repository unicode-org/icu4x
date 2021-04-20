// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::mem;
use std::ops::{Index, IndexMut};
use std::{borrow::Borrow, iter::FromIterator};

/// A simple "flat" map based on a sorted vector
///
/// See the [module level documentation][super] for why one should use this.
///
/// The API is roughly similar to that of [`std::collections::HashMap`], though it
/// requires `Ord` instead of `Hash`.
#[derive(Clone, Debug, PartialEq)]
pub struct LiteMap<K, V> {
    values: Vec<(K, V)>,
}

impl<K, V> LiteMap<K, V> {
    /// Construct a new [`LiteMap`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a new [`LiteMap`] with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }

    /// The number of elements in the [`LiteMap`]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the [`LiteMap`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
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
        match self.values.binary_search_by(|k| k.0.borrow().cmp(key)) {
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
            .binary_search_by(|k| k.0.borrow().cmp(key))
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
        match self.values.binary_search_by(|k| k.0.borrow().cmp(key)) {
            Ok(found) => Some(&mut self.values[found].1),
            Err(_) => None,
        }
    }

    /// Appends `value` with `key` to the end of the underlying vector, returning
    /// `key` and `value` _if it failed_. Useful for extending with an existing
    /// sorted list.
    /// ```rust
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new();
    /// assert!(map.try_append(1, "uno").is_none());
    /// assert!(map.try_append(3, "tres").is_none());
    ///
    /// assert!(
    ///     matches!(map.try_append(3, "tres-updated"), Some((3, "tres-updated"))),
    ///     "append duplicate of last key",
    /// );
    ///
    /// assert!(
    ///     matches!(map.try_append(2, "dos"), Some((2, "dos"))),
    ///     "append out of order"
    /// );
    ///
    /// assert_eq!(map.get(&1), Some(&"uno"));
    ///
    /// // contains the original value for the key: 3
    /// assert_eq!(map.get(&3), Some(&"tres"));
    ///
    /// // not appended since it wasn't in order
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[must_use]
    pub fn try_append(&mut self, key: K, value: V) -> Option<(K, V)> {
        if let Some(last) = self.values.last() {
            if last.0 >= key {
                return Some((key, value));
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
        Self { values: vec![] }
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
impl<K: Ord, V> FromIterator<(K, V)> for LiteMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut map = match iter.size_hint() {
            (_, Some(upper)) => Self::with_capacity(upper),
            (lower, None) => Self::with_capacity(lower),
        };

        for (key, value) in iter {
            if let Some((key, value)) = map.try_append(key, value) {
                map.insert(key, value);
            }
        }

        map
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

#[cfg(feature = "serde")]
mod serde {
    use super::LiteMap;
    use serde::de::{MapAccess, Visitor};
    use serde::ser::SerializeMap;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;
    use std::marker::PhantomData;

    impl<K: Serialize, V: Serialize> Serialize for LiteMap<K, V> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.len()))?;
            for (k, v) in self.iter() {
                map.serialize_entry(k, v)?;
            }
            map.end()
        }
    }

    /// Modified example from https://serde.rs/deserialize-map.html
    struct LiteMapVisitor<K, V> {
        marker: PhantomData<fn() -> LiteMap<K, V>>,
    }

    impl<K, V> LiteMapVisitor<K, V> {
        fn new() -> Self {
            Self {
                marker: PhantomData,
            }
        }
    }

    impl<'de, K, V> Visitor<'de> for LiteMapVisitor<K, V>
    where
        K: Deserialize<'de> + Ord,
        V: Deserialize<'de>,
    {
        type Value = LiteMap<K, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map produced by LiteMap")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = LiteMap::with_capacity(access.size_hint().unwrap_or(0));

            // While there are entries remaining in the input, add them
            // into our map.
            while let Some((key, value)) = access.next_entry()? {
                // Try to append it at the end, hoping for a sorted map.
                // If not sorted, insert as usual.
                // This allows for arbitrary maps (e.g. from user JSON)
                // to be deserialized into LiteMap
                // without impacting performance in the case of deserializing
                // a serialized map that came from another LiteMap
                if let Some((key, value)) = map.try_append(key, value) {
                    // Note: this effectively selection sorts the map,
                    // which isn't efficient for large maps
                    map.insert(key, value);
                }
            }

            Ok(map)
        }
    }

    impl<'de, K: Ord + Deserialize<'de>, V: Deserialize<'de>> Deserialize<'de> for LiteMap<K, V> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(LiteMapVisitor::new())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_iterator() {
        let mut expected = LiteMap::with_capacity(4);
        expected.insert(1, "updated-one");
        expected.insert(2, "original-two");
        expected.insert(3, "original-three");
        expected.insert(4, "updated-four");

        let actual = vec![
            (1, "original-one"),
            (2, "original-two"),
            (4, "original-four"),
            (4, "updated-four"),
            (1, "updated-one"),
            (3, "original-three"),
        ]
        .into_iter()
        .collect::<LiteMap<_, _>>();

        assert_eq!(expected, actual);
    }
}
