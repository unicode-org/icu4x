// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! See [`ZeroMap2k`] for details.

use crate::ule::AsULE;
use crate::{ZeroSlice, ZeroVec};
use core::cmp::Ordering;
use core::fmt;
use core::ops::Range;
use core::convert::TryInto;

// mod borrowed;
// #[cfg(feature = "serde")]
// mod serde;

// use borrowed::ZeroMap2kBorrowed;
use crate::map::ZeroMapKV;
use crate::map::{MutableZeroVecLike, ZeroVecLike};

/// A zero-copy map datastructure that supports two layers of keys.
/// 
/// This is an extension of [`ZeroMap`] that supports two-dimensional keys. For example,
/// to map a pair of an integer and a string to a buffer, you can write:
/// 
/// ```no_run
/// let _: ZeroMap2k<u32, str, [u8]> = unimplemented!();
/// ```
/// 
/// Internally, `ZeroMap2k` stores four zero-copy vectors, one for each type argument plus
/// one more to match between the two vectors of keys.
///
/// # Examples
///
/// ```
/// use zerovec::ZeroMap2k;
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
/// [`ZeroMap`]: crate::ZeroMap
pub struct ZeroMap2k<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    pub(crate) keys0: K0::Container,
    pub(crate) joiner: ZeroVec<'a, u32>,
    pub(crate) keys1: K1::Container,
    pub(crate) values: V::Container,
}

impl<'a, K0, K1, V> Default for ZeroMap2k<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K0, K1, V> ZeroMap2k<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// Creates a new, empty `ZeroMap2k`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::ZeroMap2k;
    ///
    /// let zm: ZeroMap2k<u16, str, str> = ZeroMap2k::new();
    /// assert!(zm.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            keys0: K0::Container::zvl_new(),
            joiner: ZeroVec::new(),
            keys1: K1::Container::zvl_new(),
            values: V::Container::zvl_new(),
        }
    }

    /// Construct a new [`ZeroMap2k`] with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keys0: K0::Container::zvl_with_capacity(capacity),
            joiner: ZeroVec::with_capacity(capacity),
            keys1: K1::Container::zvl_with_capacity(capacity),
            values: V::Container::zvl_with_capacity(capacity),
        }
    }

    // /// Obtain a borrowed version of this map
    // pub fn as_borrowed(&'a self) -> ZeroMap2kBorrowed<'a, K0, K1, V> {
    //     ZeroMap2kBorrowed {
    //         keys0: self.keys0.zvl_as_borrowed(),
    //         joiner: self.joiner.as_borrowed(),
    //         keys1: self.keys1.zvl_as_borrowed(),
    //         values: self.values.zvl_as_borrowed(),
    //     }
    // }

    /// The number of values in the [`ZeroMap2k`]
    pub fn len(&self) -> usize {
        self.values.zvl_len()
    }

    /// Whether the [`ZeroMap2k`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.zvl_len() == 0
    }

    /// Remove all elements from the [`ZeroMap2k`]
    pub fn clear(&mut self) {
        self.keys0.zvl_clear();
        self.joiner.clear();
        self.keys1.zvl_clear();
        self.values.zvl_clear();
    }

    /// Reserve capacity for `additional` more elements to be inserted into
    /// the [`ZeroMap2k`] to avoid frequent reallocations.
    ///
    /// See [`Vec::reserve()`](alloc::vec::Vec::reserve) for more information.
    pub fn reserve(&mut self, additional: usize) {
        self.keys0.zvl_reserve(additional);
        self.joiner.zvl_reserve(additional);
        self.keys1.zvl_reserve(additional);
        self.values.zvl_reserve(additional);
    }

    /// Get the value associated with `key0` and `key1`, if it exists.
    ///
    /// ```rust
    /// use zerovec::ZeroMap2k;
    ///
    /// let mut map = ZeroMap2k::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "one", "bar");
    /// map.insert(&2, "two", "baz");
    /// assert_eq!(map.get(&1, "one"), Some("foo"));
    /// assert_eq!(map.get(&1, "two"), None);
    /// assert_eq!(map.get(&2, "one"), Some("bar"));
    /// assert_eq!(map.get(&2, "two"), Some("baz"));
    /// assert_eq!(map.get(&3, "three"), None);
    /// ```
    pub fn get(&self, key0: &K0, key1: &K1) -> Option<&V::GetType> {
        let (_, range) = self.get_range_for_key0(key0)?;
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        let index = self.keys1.zvl_binary_search_in_range(key1, range).unwrap().ok()?;
        self.values.zvl_get(index)
    }

    /// Returns whether `key0` is contained in this map
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    ///
    /// let mut map = ZeroMap2k::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "two", "bar");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&3), false);
    /// ```
    pub fn contains_key0(&self, key0: &K0) -> bool {
        self.keys0.zvl_binary_search(key0).is_ok()
    }

    /// Insert `value` with `key`, returning the existing value if it exists.
    ///
    /// See example in [`Self::get()`].
    pub fn insert(&mut self, key0: &K0, key1: &K1, value: &V) -> Option<V::OwnedType> {
        let (key0_index, range) = self.get_or_insert_range_for_key0(key0);
        debug_assert!(range.start <= range.end); // '<=' because we may have inserted a new key0
        debug_assert!(range.end <= self.keys1.zvl_len());
        let index = match self.keys1.zvl_binary_search_in_range(key1, range).unwrap() {
            Ok(index) => return Some(self.values.zvl_replace(index, value)),
            Err(index) => index,
        };
        self.keys1.zvl_insert(index, key1);
        self.values.zvl_insert(index, value);
        self.joiner_expand(key0_index);
        None
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
    pub fn remove(&mut self, key0: &K0, key1: &K1) -> Option<V::OwnedType> {
        let (key0_index, range) = self.get_range_for_key0(key0)?;
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        let is_singleton_range = range.start + 1 == range.end;
        let index = match self.keys1.zvl_binary_search_in_range(key1, range).unwrap() {
            Ok(index) => index,
            Err(index) => return None,
        };
        self.keys1.zvl_remove(index);
        self.values.zvl_remove(index);
        self.joiner_shrink(key0_index);
        if is_singleton_range {
            self.remove_key0_index(key0_index);
        }
        None
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
    pub fn try_append<'b>(&mut self, key0: &'b K0, key1: &'b K1, value: &'b V) -> Option<(&'b K0, &'b K1, &'b V)> {
        // Get or insert key0
        let last_key0 = if self.keys0.zvl_len() != 0 {
            self.keys0.zvl_get(self.keys0.zvl_len() - 1)
        } else {
            None
        };
        if let Some(last_key0) = last_key0 {
            match K0::Container::t_cmp_get(key0, last_key0) {
                Ordering::Less => {
                    // Error case
                    return Some((key0, key1, value));
                },
                Ordering::Equal => {}, // Already have a key0
                Ordering::Greater => {
                    // Append a new key0
                    self.keys0.zvl_push(key0);
                    // TODO: Make this fallible
                    debug_assert!(self.keys1.zvl_len() <= u32::MAX as usize);
                    let joiner_value: u32 = self.keys1.zvl_len().try_into().unwrap();
                    self.joiner.to_mut().push(joiner_value.as_unaligned());
                },
            }
        }

        // Insert key1
        let last_key1 = if self.keys1.zvl_len() != 0 {
            self.keys1.zvl_get(self.keys1.zvl_len() - 1)
        } else {
            None
        };
        if let Some(last_key1) = last_key1 {
            match K1::Container::t_cmp_get(key1, last_key1) {
                Ordering::Less | Ordering::Equal => {
                    // Error case
                    return Some((key0, key1, value));
                },
                Ordering::Greater => {
                    // Append a new key1
                    self.keys1.zvl_push(key1);
                    self.values.zvl_push(value);
                },
            };
        }

        // Increment the joiner range
        self.joiner.to_mut().as_mut_slice().last_mut().unwrap().add_unsigned(1);

        None
    }

    /// Produce an ordered iterator over keys0
    pub fn iter_keys0<'b>(&'b self) -> impl Iterator<Item = &'b <K0 as ZeroMapKV<'a>>::GetType> {
        (0..self.keys0.zvl_len()).map(move |idx| self.keys0.zvl_get(idx).unwrap())
    }

    /// Produce an ordered iterator over keys1 for a particular key0, if key0 exists
    pub fn iter_keys1<'b>(&'b self, key0: &K0) -> Option<impl Iterator<Item = &'b <K1 as ZeroMapKV<'a>>::GetType>> {
        let (_, range) = self.get_range_for_key0(key0)?;
        Some(range.map(move |idx| self.keys1.zvl_get(idx).unwrap()))
    }

    /// Produce an iterator over values, ordered by the pair (key0,key1)
    pub fn iter_values<'b>(&'b self) -> impl Iterator<Item = &'b <V as ZeroMapKV<'a>>::GetType> {
        (0..self.values.zvl_len()).map(move |idx| self.values.zvl_get(idx).unwrap())
    }

    // INTERNAL ROUTINES FOLLOW //

    /// Given a value that may exist in keys0, returns the corresponding range of keys1
    fn get_range_for_key0(&self, key0: &K0) -> Option<(usize, Range<usize>)> {
        let key0_index = self.keys0.zvl_binary_search(key0).ok()?;
        Some((key0_index, self.get_range_for_key0_index(key0_index)))
    }

    /// Given an index into the joiner array, returns the corresponding range of keys1
    fn get_range_for_key0_index(&self, key0_index: usize) -> Range<usize> {
        debug_assert!(key0_index < self.joiner.len());
        let start = if key0_index == 0 {
            0
        } else {
            // The unwrap is protected by the debug_assert above
            self.joiner.get(key0_index - 1).unwrap()
        };
        // The unwrap is protected by the debug_assert above
        let limit = self.joiner.get(key0_index).unwrap();
        (start as usize)..(limit as usize)
    }

    /// Same as `get_range_for_key0`, but creates key0 if it doesn't already exist
    fn get_or_insert_range_for_key0(&mut self, key0: &K0) -> (usize, Range<usize>) {
        match self.keys0.zvl_binary_search(key0) {
            Ok(key0_index) => {
                (key0_index, self.get_range_for_key0_index(key0_index))
            },
            Err(key0_index) => {
                // Add an entry to self.keys0 and self.joiner
                let joiner_value = if key0_index == 0 {
                    0
                } else {
                    debug_assert!(key0_index <= self.joiner.len());
                    self.joiner.get(key0_index - 1).unwrap()
                };
                self.keys0.zvl_insert(key0_index, key0);
                self.joiner.to_mut().insert(key0_index, joiner_value.as_unaligned());
                (key0_index, (joiner_value as usize)..(joiner_value as usize))
            }
        }
    }

    /// Removes key0_index from the keys0 array and the joiner array
    fn remove_key0_index(&mut self, key0_index: usize) {
        self.keys0.zvl_remove(key0_index);
        self.joiner.to_mut().remove(key0_index);
    }

    /// Shifts all joiner ranges from key0_index onward one index up
    fn joiner_expand(&mut self, key0_index: usize) {
        self.joiner.to_mut().iter_mut().skip(key0_index).for_each(|ref mut v| {
            // TODO: Make this fallible
            v.add_unsigned(1).expect("Attempted to add more than 2^32 elements to a ZeroMap2k")
        });
    }

    /// Shifts all joiner ranges from key0_index onward one index down
    fn joiner_shrink(&mut self, key0_index: usize) {
        self.joiner.to_mut().iter_mut().skip(key0_index).for_each(|ref mut v| {
            v.add_unsigned(-1).expect("Shrink should always succeed")
        });
    }
}
/*
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
                self.keys.zvl_get(idx).unwrap(),
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
                ZeroSlice::get(&**keys, idx).unwrap(),
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
*/
