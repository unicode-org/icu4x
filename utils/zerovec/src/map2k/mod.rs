// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! See [`ZeroMap2k`] for details.

use crate::ule::AsULE;
use crate::ZeroVec;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt;
use core::ops::Range;

mod borrowed;
#[cfg(feature = "serde")]
mod serde;

use crate::map::ZeroMapKV;
use crate::map::{MutableZeroVecLike, ZeroVecLike};
pub use borrowed::ZeroMap2kBorrowed;

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

    /// Obtain a borrowed version of this map
    pub fn as_borrowed(&'a self) -> ZeroMap2kBorrowed<'a, K0, K1, V> {
        ZeroMap2kBorrowed {
            keys0: self.keys0.zvl_as_borrowed(),
            joiner: &*self.joiner,
            keys1: self.keys1.zvl_as_borrowed(),
            values: self.values.zvl_as_borrowed(),
        }
    }

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
        let index = range.start
            + self
                .keys1
                .zvl_binary_search_in_range(key1, range)
                .unwrap()
                .ok()?;
        self.values.zvl_get(index)
    }

    /// Returns whether `key0` is contained in this map
    ///
    /// ```rust
    /// use zerovec::ZeroMap2k;
    ///
    /// let mut map = ZeroMap2k::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "two", "bar");
    /// assert_eq!(map.contains_key0(&1), true);
    /// assert_eq!(map.contains_key0(&3), false);
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
        let index = range.start
            + match self.keys1.zvl_binary_search_in_range(key1, range).unwrap() {
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
        let index = range.start
            + match self.keys1.zvl_binary_search_in_range(key1, range).unwrap() {
                Ok(index) => index,
                Err(_) => return None,
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
    pub fn try_append<'b>(
        &mut self,
        key0: &'b K0,
        key1: &'b K1,
        value: &'b V,
    ) -> Option<(&'b K0, &'b K1, &'b V)> {
        if self.is_empty() {
            self.keys0.zvl_push(key0);
            self.joiner.to_mut().push(1u32.as_unaligned());
            self.keys1.zvl_push(key1);
            self.values.zvl_push(value);
            return None;
        }

        // The unwraps are protected by the fact that we are not empty
        let last_key0 = self.keys0.zvl_get(self.keys0.zvl_len() - 1).unwrap();
        let key0_cmp = K0::Container::t_cmp_get(key0, last_key0);
        let last_key1 = self.keys1.zvl_get(self.keys1.zvl_len() - 1).unwrap();
        let key1_cmp = K1::Container::t_cmp_get(key1, last_key1);

        // Check for error case (out of order)
        match key0_cmp {
            Ordering::Less => {
                // Error case
                return Some((key0, key1, value));
            }
            Ordering::Equal => {
                match key1_cmp {
                    Ordering::Less | Ordering::Equal => {
                        // Error case
                        return Some((key0, key1, value));
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        let joiner_value = u32::try_from(self.keys1.zvl_len() + 1)
            .expect("Attempted to add more than 2^32 elements to a ZeroMap2k");

        // All OK to append
        if key0_cmp == Ordering::Greater {
            self.keys0.zvl_push(key0);
            self.joiner.to_mut().push(joiner_value.as_unaligned());
        } else {
            *self.joiner.to_mut().last_mut().unwrap() = joiner_value.as_unaligned();
        }
        self.keys1.zvl_push(key1);
        self.values.zvl_push(value);

        None
    }

    /// Produce an ordered iterator over keys0.
    pub fn iter_keys0<'b>(&'b self) -> impl Iterator<Item = &'b <K0 as ZeroMapKV<'a>>::GetType> {
        (0..self.keys0.zvl_len()).map(move |idx| self.keys0.zvl_get(idx).unwrap())
    }

    /// Produce an ordered iterator over keys1 for a particular key0, if key0 exists.
    pub fn iter_keys1<'b>(
        &'b self,
        key0: &K0,
    ) -> Option<impl Iterator<Item = &'b <K1 as ZeroMapKV<'a>>::GetType>> {
        let (_, range) = self.get_range_for_key0(key0)?;
        Some(range.map(move |idx| self.keys1.zvl_get(idx).unwrap()))
    }

    /// Produce an ordered iterator over keys1 for a particular key0_index, if key0_index exists.
    ///
    /// This method is designed to interoperate with the enumerated key of `iter_keys0`.
    ///
    /// # Panics
    ///
    /// Panics if `key0_index` is out of range.
    ///
    /// # Example
    ///
    /// Loop over all elements of a ZeroMap2k:
    ///
    /// ```
    /// use zerovec::ZeroMap2k;
    ///
    /// let mut map: ZeroMap2k<u16, u16, str> = ZeroMap2k::new();
    /// map.insert(&1, &1, "foo");
    /// map.insert(&2, &3, "bar");
    /// map.insert(&2, &4, "baz");
    ///
    /// let mut total_value = 0;
    ///
    /// let mut values_it = map.iter_values();
    /// for (key0_index, key0) in map.iter_keys0().enumerate() {
    ///     for key1 in map.iter_keys1_by_index(key0_index).unwrap() {
    ///         // This code runs for every (key0, key1) pair
    ///         total_value += key0.as_unsigned_int() as usize;
    ///         total_value += key1.as_unsigned_int() as usize;
    ///         total_value += values_it.next().unwrap().len();
    ///     }
    /// }
    ///
    /// assert_eq!(total_value, 22);
    /// ```
    pub fn iter_keys1_by_index<'b>(
        &'b self,
        key0_index: usize,
    ) -> Option<impl Iterator<Item = &'b <K1 as ZeroMapKV<'a>>::GetType>> {
        assert!(key0_index < self.keys0.zvl_len());
        let range = self.get_range_for_key0_index(key0_index);
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
            Ok(key0_index) => (key0_index, self.get_range_for_key0_index(key0_index)),
            Err(key0_index) => {
                // Add an entry to self.keys0 and self.joiner
                let joiner_value = if key0_index == 0 {
                    0
                } else {
                    debug_assert!(key0_index <= self.joiner.len());
                    self.joiner.get(key0_index - 1).unwrap()
                };
                self.keys0.zvl_insert(key0_index, key0);
                self.joiner
                    .to_mut()
                    .insert(key0_index, joiner_value.as_unaligned());
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
        self.joiner
            .to_mut()
            .iter_mut()
            .skip(key0_index)
            .for_each(|ref mut v| {
                // TODO(#1410): Make this fallible
                v.add_unsigned_int(1)
                    .expect("Attempted to add more than 2^32 elements to a ZeroMap2k")
            });
    }

    /// Shifts all joiner ranges from key0_index onward one index down
    fn joiner_shrink(&mut self, key0_index: usize) {
        self.joiner
            .to_mut()
            .iter_mut()
            .skip(key0_index)
            .for_each(|ref mut v| {
                v.add_unsigned_int(-1)
                    .expect("Shrink should always succeed")
            });
    }
}

impl<'a, K0, K1, V> ZeroMap2k<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
    V: AsULE + Copy,
{
    /// For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`
    ///
    /// # Examples
    ///
    /// ```
    /// # use zerovec::ZeroMap2k;
    /// let mut map: ZeroMap2k::<u16, u16, u16> = ZeroMap2k::new();
    /// map.insert(&1, &2, &3);
    /// map.insert(&1, &4, &5);
    /// map.insert(&6, &7, &8);
    ///
    /// assert_eq!(map.get_copied(&6, &7), 8);
    /// ```
    pub fn get_copied(&self, key0: &K0, key1: &K1) -> Option<V> {
        let (_, range) = self.get_range_for_key0(key0)?;
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        let index = range.start
            + self
                .keys1
                .zvl_binary_search_in_range(key1, range)
                .unwrap()
                .ok()?;
        self.values.get(index)
    }
}

impl<'a, K0, K1, V> From<ZeroMap2kBorrowed<'a, K0, K1, V>> for ZeroMap2k<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    fn from(other: ZeroMap2kBorrowed<'a, K0, K1, V>) -> Self {
        Self {
            keys0: K0::Container::zvl_from_borrowed(other.keys0),
            joiner: other.joiner.as_zerovec(),
            keys1: K1::Container::zvl_from_borrowed(other.keys1),
            values: V::Container::zvl_from_borrowed(other.values),
        }
    }
}

// We can't use the default PartialEq because ZeroMap2k is invariant
// so otherwise rustc will not automatically allow you to compare ZeroMaps
// with different lifetimes
impl<'a, 'b, K0, K1, V> PartialEq<ZeroMap2k<'b, K0, K1, V>> for ZeroMap2k<'a, K0, K1, V>
where
    K0: for<'c> ZeroMapKV<'c> + ?Sized,
    K1: for<'c> ZeroMapKV<'c> + ?Sized,
    V: for<'c> ZeroMapKV<'c> + ?Sized,
    <K0 as ZeroMapKV<'a>>::Container: PartialEq<<K0 as ZeroMapKV<'b>>::Container>,
    <K1 as ZeroMapKV<'a>>::Container: PartialEq<<K1 as ZeroMapKV<'b>>::Container>,
    <V as ZeroMapKV<'a>>::Container: PartialEq<<V as ZeroMapKV<'b>>::Container>,
{
    fn eq(&self, other: &ZeroMap2k<'b, K0, K1, V>) -> bool {
        self.keys0.eq(&other.keys0)
            && self.joiner.eq(&other.joiner)
            && self.keys1.eq(&other.keys1)
            && self.values.eq(&other.values)
    }
}

impl<'a, K0, K1, V> fmt::Debug for ZeroMap2k<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <K0 as ZeroMapKV<'a>>::Container: fmt::Debug,
    <K1 as ZeroMapKV<'a>>::Container: fmt::Debug,
    <V as ZeroMapKV<'a>>::Container: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("ZeroMap2k")
            .field("keys0", &self.keys0)
            .field("joiner", &self.joiner)
            .field("keys1", &self.keys1)
            .field("values", &self.values)
            .finish()
    }
}

impl<'a, K0, K1, V> Clone for ZeroMap2k<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <K0 as ZeroMapKV<'a>>::Container: Clone,
    <K1 as ZeroMapKV<'a>>::Container: Clone,
    <V as ZeroMapKV<'a>>::Container: Clone,
{
    fn clone(&self) -> Self {
        Self {
            keys0: self.keys0.clone(),
            joiner: self.joiner.clone(),
            keys1: self.keys1.clone(),
            values: self.values.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stress_test() {
        let mut zm2k = ZeroMap2k::<u16, str, str>::new();

        assert_eq!(format!("{:?}", zm2k), "ZeroMap2k { keys0: ZeroVec::Borrowed([]), joiner: ZeroVec::Borrowed([]), keys1: [], values: [] }");
        assert_eq!(zm2k.get(&0, ""), None);

        let result = zm2k.try_append(&3, "ccc", "CCC");
        assert!(matches!(result, None));

        assert_eq!(format!("{:?}", zm2k), "ZeroMap2k { keys0: ZeroVec::Owned([3]), joiner: ZeroVec::Owned([1]), keys1: [\"ccc\"], values: [\"CCC\"] }");
        assert_eq!(zm2k.get(&0, ""), None);
        assert_eq!(zm2k.get(&3, ""), None);
        assert_eq!(zm2k.get(&3, "ccc"), Some("CCC"));
        assert_eq!(zm2k.get(&99, ""), None);

        let result = zm2k.try_append(&3, "eee", "EEE");
        assert!(matches!(result, None));

        assert_eq!(format!("{:?}", zm2k), "ZeroMap2k { keys0: ZeroVec::Owned([3]), joiner: ZeroVec::Owned([2]), keys1: [\"ccc\", \"eee\"], values: [\"CCC\", \"EEE\"] }");
        assert_eq!(zm2k.get(&0, ""), None);
        assert_eq!(zm2k.get(&3, ""), None);
        assert_eq!(zm2k.get(&3, "ccc"), Some("CCC"));
        assert_eq!(zm2k.get(&3, "eee"), Some("EEE"));
        assert_eq!(zm2k.get(&3, "five"), None);
        assert_eq!(zm2k.get(&99, ""), None);

        // Out of order
        let result = zm2k.try_append(&3, "ddd", "DD0");
        assert!(matches!(result, Some(_)));

        // Append a few more elements
        let result = zm2k.try_append(&5, "ddd", "DD1");
        assert!(matches!(result, None));
        let result = zm2k.try_append(&7, "ddd", "DD2");
        assert!(matches!(result, None));
        let result = zm2k.try_append(&7, "eee", "EEE");
        assert!(matches!(result, None));
        let result = zm2k.try_append(&7, "www", "WWW");
        assert!(matches!(result, None));
        let result = zm2k.try_append(&9, "yyy", "YYY");
        assert!(matches!(result, None));

        assert_eq!(format!("{:?}", zm2k), "ZeroMap2k { keys0: ZeroVec::Owned([3, 5, 7, 9]), joiner: ZeroVec::Owned([2, 3, 6, 7]), keys1: [\"ccc\", \"eee\", \"ddd\", \"ddd\", \"eee\", \"www\", \"yyy\"], values: [\"CCC\", \"EEE\", \"DD1\", \"DD2\", \"EEE\", \"WWW\", \"YYY\"] }");
        assert_eq!(zm2k.get(&0, ""), None);
        assert_eq!(zm2k.get(&3, ""), None);
        assert_eq!(zm2k.get(&3, "ccc"), Some("CCC"));
        assert_eq!(zm2k.get(&3, "eee"), Some("EEE"));
        assert_eq!(zm2k.get(&3, "zzz"), None);
        assert_eq!(zm2k.get(&4, ""), None);
        assert_eq!(zm2k.get(&5, "aaa"), None);
        assert_eq!(zm2k.get(&5, "ddd"), Some("DD1"));
        assert_eq!(zm2k.get(&5, "zzz"), None);
        assert_eq!(zm2k.get(&6, ""), None);
        assert_eq!(zm2k.get(&7, "aaa"), None);
        assert_eq!(zm2k.get(&7, "ddd"), Some("DD2"));
        assert_eq!(zm2k.get(&7, "eee"), Some("EEE"));
        assert_eq!(zm2k.get(&7, "www"), Some("WWW"));
        assert_eq!(zm2k.get(&7, "yyy"), None);
        assert_eq!(zm2k.get(&7, "zzz"), None);
        assert_eq!(zm2k.get(&8, ""), None);
        assert_eq!(zm2k.get(&9, "aaa"), None);
        assert_eq!(zm2k.get(&9, "www"), None);
        assert_eq!(zm2k.get(&9, "yyy"), Some("YYY"));
        assert_eq!(zm2k.get(&9, "zzz"), None);
        assert_eq!(zm2k.get(&10, ""), None);
        assert_eq!(zm2k.get(&99, ""), None);

        // Insert some elements
        zm2k.insert(&3, "mmm", "MM0");
        zm2k.insert(&6, "ddd", "DD3");
        zm2k.insert(&6, "mmm", "MM1");
        zm2k.insert(&6, "nnn", "NNN");

        assert_eq!(format!("{:?}", zm2k), "ZeroMap2k { keys0: ZeroVec::Owned([3, 5, 6, 7, 9]), joiner: ZeroVec::Owned([3, 4, 7, 10, 11]), keys1: [\"ccc\", \"eee\", \"mmm\", \"ddd\", \"ddd\", \"mmm\", \"nnn\", \"ddd\", \"eee\", \"www\", \"yyy\"], values: [\"CCC\", \"EEE\", \"MM0\", \"DD1\", \"DD3\", \"MM1\", \"NNN\", \"DD2\", \"EEE\", \"WWW\", \"YYY\"] }");
        assert_eq!(zm2k.get(&0, ""), None);
        assert_eq!(zm2k.get(&3, ""), None);
        assert_eq!(zm2k.get(&3, "ccc"), Some("CCC"));
        assert_eq!(zm2k.get(&3, "eee"), Some("EEE"));
        assert_eq!(zm2k.get(&3, "zzz"), None);
        assert_eq!(zm2k.get(&4, ""), None);
        assert_eq!(zm2k.get(&5, "aaa"), None);
        assert_eq!(zm2k.get(&5, "ddd"), Some("DD1"));
        assert_eq!(zm2k.get(&5, "zzz"), None);
        assert_eq!(zm2k.get(&6, ""), None);
        assert_eq!(zm2k.get(&7, "aaa"), None);
        assert_eq!(zm2k.get(&7, "ddd"), Some("DD2"));
        assert_eq!(zm2k.get(&7, "eee"), Some("EEE"));
        assert_eq!(zm2k.get(&7, "www"), Some("WWW"));
        assert_eq!(zm2k.get(&7, "yyy"), None);
        assert_eq!(zm2k.get(&7, "zzz"), None);
        assert_eq!(zm2k.get(&8, ""), None);
        assert_eq!(zm2k.get(&9, "aaa"), None);
        assert_eq!(zm2k.get(&9, "www"), None);
        assert_eq!(zm2k.get(&9, "yyy"), Some("YYY"));
        assert_eq!(zm2k.get(&9, "zzz"), None);
        assert_eq!(zm2k.get(&10, ""), None);
        assert_eq!(zm2k.get(&99, ""), None);
    }
}
