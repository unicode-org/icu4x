// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Borrow;
use alloc::vec;
use alloc::vec::Vec;
use core::iter::FromIterator;
use core::mem;
use core::ops::{Index, IndexMut};

/// A simple "flat" map based on a sorted vector
///
/// See the [module level documentation][super] for why one should use this.
///
/// The API is roughly similar to that of [`std::collections::HashMap`], though it
/// requires `Ord` instead of `Hash`.
#[derive(Clone, Debug, PartialEq)]
pub struct LiteMap<K, V> {
    pub(crate) values: Vec<(K, V)>,
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

    /// Convert a `Vec<(K, V)>` into a [`LiteMap`].
    ///
    /// # Safety
    ///
    /// The vec must be sorted and have no duplicate keys.
    #[inline]
    pub unsafe fn from_tuple_vec_unchecked(values: Vec<(K, V)>) -> Self {
        Self { values }
    }

    /// Convert a [`LiteMap`] into a `Vec<(K, V)>`.
    #[inline]
    pub fn into_tuple_vec(self) -> Vec<(K, V)> {
        self.values
    }

    /// Get the key-value pair residing at a particular index
    ///
    /// In most cases, prefer [`LiteMap::get()`] over this method.
    #[inline]
    pub fn get_indexed(&self, index: usize) -> Option<(&K, &V)> {
        self.values.get(index).map(|(ref k, ref v)| (k, v))
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
        match self.find_index(key) {
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
        self.find_index(key).is_ok()
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
        match self.find_index(key) {
            Ok(found) => Some(&mut self.values[found].1),
            Err(_) => None,
        }
    }

    /// Get the lowest-rank key/value pair from the `LiteMap`, if it exists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new();
    /// assert!(map.try_append(1, "uno").is_none());
    /// assert!(map.try_append(3, "tres").is_none());
    ///
    /// assert_eq!(map.first(), Some((&1, &"uno")));
    /// ```
    #[inline]
    pub fn first(&self) -> Option<(&K, &V)> {
        self.values.get(0).map(|(k, v)| (k, v))
    }

    /// Get the highest-rank key/value pair from the `LiteMap`, if it exists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use litemap::LiteMap;
    ///
    /// let mut map = LiteMap::new();
    /// assert!(map.try_append(1, "uno").is_none());
    /// assert!(map.try_append(3, "tres").is_none());
    ///
    /// assert_eq!(map.last(), Some((&3, &"tres")));
    /// ```
    #[inline]
    pub fn last(&self) -> Option<(&K, &V)> {
        self.values.get(self.len() - 1).map(|(k, v)| (k, v))
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
        self.insert_save_key(key, value).map(|(_, v)| v)
    }

    /// Version of [`Self::insert()`] that returns both the key and the old value.
    fn insert_save_key(&mut self, key: K, value: V) -> Option<(K, V)> {
        match self.values.binary_search_by(|k| k.0.cmp(&key)) {
            Ok(found) => Some((key, mem::replace(&mut self.values[found].1, value))),
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

    /// Insert all elements from `other` into this `LiteMap`.
    ///
    /// If `other` contains keys that already exist in `self`, the values in `other` replace the
    /// corresponding ones in `self`, and the rejected items from `self` are returned as a new
    /// `LiteMap`. Otherwise, `None` is returned.
    ///
    /// The implementation of this function is optimized if `self` and `other` have no overlap.
    ///
    /// # Examples
    ///
    /// ```
    /// use litemap::LiteMap;
    ///
    /// let mut map1 = LiteMap::new();
    /// map1.insert(1, "one");
    /// map1.insert(2, "two");
    ///
    /// let mut map2 = LiteMap::new();
    /// map2.insert(2, "TWO");
    /// map2.insert(4, "FOUR");
    ///
    /// let leftovers = map1.extend_from_litemap(map2);
    ///
    /// assert_eq!(map1.len(), 3);
    /// assert_eq!(map1.get(&1), Some("one").as_ref());
    /// assert_eq!(map1.get(&2), Some("TWO").as_ref());
    /// assert_eq!(map1.get(&4), Some("FOUR").as_ref());
    ///
    /// let map3 = leftovers.expect("Duplicate keys");
    /// assert_eq!(map3.len(), 1);
    /// assert_eq!(map3.get(&2), Some("two").as_ref());
    /// ```
    pub fn extend_from_litemap(&mut self, other: LiteMap<K, V>) -> Option<LiteMap<K, V>> {
        if self.is_empty() {
            self.values = other.values;
            return None;
        }
        if other.is_empty() {
            return None;
        }
        if self.last().map(|(k, _)| k) < other.first().map(|(k, _)| k) {
            // append other to self
            self.values.extend(other.values);
            None
        } else if self.first().map(|(k, _)| k) > other.last().map(|(k, _)| k) {
            // prepend other to self
            self.values.splice(0..0, other.values);
            None
        } else {
            // insert every element
            let leftover_tuples = other
                .values
                .into_iter()
                .filter_map(|(k, v)| self.insert_save_key(k, v))
                .collect();
            let ret = LiteMap {
                values: leftover_tuples,
            };
            if ret.is_empty() {
                None
            } else {
                Some(ret)
            }
        }
    }

    /// Obtain the index for a given key, or if the key is not found, the index
    /// at which it would be inserted.
    ///
    /// (The return value works equivalently to [`slice::binary_search_by()`])
    ///
    /// The indices returned can be used with [`Self::get_indexed()`]. Prefer using
    /// [`Self::get()`] directly where possible.
    #[inline]
    pub fn find_index<Q: ?Sized>(&self, key: &Q) -> Result<usize, usize>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.values.binary_search_by(|k| k.0.borrow().cmp(key))
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
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> + DoubleEndedIterator {
        self.values.iter().map(|val| (&val.0, &val.1))
    }

    /// Produce an ordered iterator over keys
    pub fn iter_keys(&self) -> impl Iterator<Item = &K> + DoubleEndedIterator {
        self.values.iter().map(|val| &val.0)
    }

    /// Produce an iterator over values, ordered by their keys
    pub fn iter_values(&self) -> impl Iterator<Item = &V> + DoubleEndedIterator {
        self.values.iter().map(|val| &val.1)
    }

    /// Produce an ordered mutable iterator over key-value pairs
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> + DoubleEndedIterator {
        self.values.iter_mut().map(|val| (&val.0, &mut val.1))
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

    fn make_13() -> LiteMap<usize, &'static str> {
        let mut result = LiteMap::new();
        result.insert(1, "one");
        result.insert(3, "three");
        result
    }

    fn make_24() -> LiteMap<usize, &'static str> {
        let mut result = LiteMap::new();
        result.insert(2, "TWO");
        result.insert(4, "FOUR");
        result
    }

    fn make_46() -> LiteMap<usize, &'static str> {
        let mut result = LiteMap::new();
        result.insert(4, "four");
        result.insert(6, "six");
        result
    }

    #[test]
    fn extend_from_litemap_append() {
        let mut map = LiteMap::new();
        map.extend_from_litemap(make_13())
            .ok_or(())
            .expect_err("Append to empty map");
        map.extend_from_litemap(make_46())
            .ok_or(())
            .expect_err("Append to lesser map");
        assert_eq!(map.len(), 4);
    }

    #[test]
    fn extend_from_litemap_prepend() {
        let mut map = LiteMap::new();
        map.extend_from_litemap(make_46())
            .ok_or(())
            .expect_err("Prepend to empty map");
        map.extend_from_litemap(make_13())
            .ok_or(())
            .expect_err("Prepend to lesser map");
        assert_eq!(map.len(), 4);
    }

    #[test]
    fn extend_from_litemap_insert() {
        let mut map = LiteMap::new();
        map.extend_from_litemap(make_13())
            .ok_or(())
            .expect_err("Append to empty map");
        map.extend_from_litemap(make_24())
            .ok_or(())
            .expect_err("Insert with no conflict");
        map.extend_from_litemap(make_46())
            .ok_or(())
            .expect("Insert with conflict");
        assert_eq!(map.len(), 5);
    }
}
