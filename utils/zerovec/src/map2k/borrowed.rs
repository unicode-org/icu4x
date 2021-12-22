// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::AsULE;
use crate::{ZeroSlice, ZeroVec};

use core::fmt;
use core::ops::Range;

use crate::map::ZeroMapKV;
use crate::map::{BorrowedZeroVecLike, ZeroVecLike};

/// A borrowed-only version of [`ZeroMap2k`](super::ZeroMap2k)
///
/// This is useful for fully-zero-copy deserialization from non-human-readable
/// serialization formats. It also has the advantage that it can return references that live for
/// the lifetime of the backing buffer as opposed to that of the [`ZeroMap2kBorrowed`] instance.
///
/// # Examples
///
/// ```
/// use zerovec::map::ZeroMap2kBorrowed;
///
/// // Example byte buffer representing the map { 1: "one" }
/// let BINCODE_BYTES: &[u8; 31] = &[
///     4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0,
///     1, 0, 0, 0, 0, 0, 0, 0, 111, 110, 101
/// ];
///
/// // Deserializing to ZeroMap2k requires no heap allocations.
/// let zero_map: ZeroMap2kBorrowed<u32, str> = bincode::deserialize(BINCODE_BYTES)
///     .expect("Should deserialize successfully");
/// assert_eq!(zero_map.get(&1), Some("one"));
/// ```
///
/// This can be obtained from a [`ZeroMap2k`](super::ZeroMap2k) via [`ZeroMap2k::as_borrowed`](super::ZeroMap2k::as_borrowed)
pub struct ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    pub(crate) keys0: <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K0>>::BorrowedVariant,
    pub(crate) joiner: &'a ZeroSlice<u32>,
    pub(crate) keys1: <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K1>>::BorrowedVariant,
    pub(crate) values: <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, V>>::BorrowedVariant,
}

impl<'a, K0, K1, V> Copy for ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
}

impl<'a, K0, K1, V> Clone for ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    fn clone(&self) -> Self {
        ZeroMap2kBorrowed {
            keys0: self.keys0,
            joiner: self.joiner,
            keys1: self.keys1,
            values: self.values,
        }
    }
}

impl<'a, K0, K1, V> Default for ZeroMap2kBorrowed<'a, K0, K1, V>
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

impl<'a, K0, K1, V> ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// Creates a new, empty `ZeroMap2kBorrowed<K0, K1, V>`.
    ///
    /// Note: Since [`ZeroMap2kBorrowed`] is not mutable, the return value will be a stub unless
    /// converted into a [`ZeroMap2k`](super::ZeroMap2k).
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::map::ZeroMap2kBorrowed;
    ///
    /// let zm: ZeroMap2kBorrowed<u16, str> = ZeroMap2kBorrowed::new();
    /// assert!(zm.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            keys0:
                <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K0>>::BorrowedVariant::zvl_new(
                ),
            joiner: Default::default(),
            keys1:
                <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K1>>::BorrowedVariant::zvl_new(
                ),
            values:
                <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, V>>::BorrowedVariant::zvl_new(),
        }
    }

    /// The number of elements in the [`ZeroMap2kBorrowed`]
    pub fn len(&self) -> usize {
        self.values.zvl_len()
    }

    /// Whether the [`ZeroMap2kBorrowed`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.zvl_len() == 0
    }

    /// Get the value associated with `key`, if it exists.
    ///
    /// This is able to return values that live longer than the map itself
    /// since they borrow directly from the backing buffer. This is the
    /// primary advantage of using [`ZeroMap2kBorrowed`](super::ZeroMap2kBorrowed) over [`ZeroMap2k`](super::ZeroMap2k).
    ///
    /// ```rust
    /// use zerovec::ZeroMap2k;
    /// use zerovec::map2k::ZeroMap2kBorrowed;
    ///
    /// let mut map = ZeroMap2k::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "one", "bar");
    /// map.insert(&2, "two", "baz");
    ///
    /// let borrowed = map.as_borrowed();
    /// assert_eq!(borrowed.get(&1, "one"), Some("foo"));
    /// assert_eq!(borrowed.get(&1, "two"), None);
    /// assert_eq!(borrowed.get(&2, "one"), Some("bar"));
    /// assert_eq!(borrowed.get(&2, "two"), Some("baz"));
    /// assert_eq!(borrowed.get(&3, "three"), None);
    ///
    /// let borrow = borrowed.get(&1, "one");
    /// drop(borrowed);
    /// // still exists after the ZeroMap2kBorrowed has been dropped
    /// assert_eq!(borrow, Some("foo"));
    /// ```
    pub fn get(&self, key0: &K0, key1: &K1) -> Option<&'a V::GetType> {
        let (_, range) = self.get_range_for_key0(key0)?;
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        let index = self
            .keys1
            .zvl_binary_search_in_range(key1, range)
            .unwrap()
            .ok()?;
        self.values.zvl_get_borrowed(index)
    }

    /// Returns whether `key0` is contained in this map
    ///
    /// ```rust
    /// use zerovec::ZeroMap2k;
    /// use zerovec::map::ZeroMap2kBorrowed;
    ///
    /// let mut map = ZeroMap2k::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "two", "bar");
    /// let borrowed = map.as_borrowed();
    /// assert_eq!(borrowed.contains_key0(&1), true);
    /// assert_eq!(borrowed.contains_key0(&3), false);
    /// ```
    pub fn contains_key0(&self, key0: &K0) -> bool {
        self.keys0.zvl_binary_search(key0).is_ok()
    }

    /// Produce an ordered iterator over keys0
    pub fn iter_keys0<'b>(&'b self) -> impl Iterator<Item = &'b <K0 as ZeroMapKV<'a>>::GetType> {
        (0..self.keys0.zvl_len()).map(move |idx| self.keys0.zvl_get(idx).unwrap())
    }

    /// Produce an ordered iterator over keys1 for a particular key0, if key0 exists
    pub fn iter_keys1<'b>(
        &'b self,
        key0: &K0,
    ) -> Option<impl Iterator<Item = &'b <K1 as ZeroMapKV<'a>>::GetType>> {
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
}

impl<'a, K0, K1, V> ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
    V: AsULE + Ord + Copy + 'static,
{
    /// For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`
    pub fn get_copied(&self, key0: &K0, key1: &K1) -> Option<V> {
        let (_, range) = self.get_range_for_key0(key0)?;
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        let index = self
            .keys1
            .zvl_binary_search_in_range(key1, range)
            .unwrap()
            .ok()?;
        self.values.get(index)
    }
}

// We can't use the default PartialEq because ZeroMap2k is invariant
// so otherwise rustc will not automatically allow you to compare ZeroMaps
// with different lifetimes
impl<'a, 'b, K0, K1, V> PartialEq<ZeroMap2kBorrowed<'b, K0, K1, V>>
    for ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K0: for<'c> ZeroMapKV<'c> + ?Sized,
    K1: for<'c> ZeroMapKV<'c> + ?Sized,
    V: for<'c> ZeroMapKV<'c> + ?Sized,
    <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K0>>::BorrowedVariant:
        PartialEq<<<K0 as ZeroMapKV<'b>>::Container as ZeroVecLike<'b, K0>>::BorrowedVariant>,
    <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K1>>::BorrowedVariant:
        PartialEq<<<K1 as ZeroMapKV<'b>>::Container as ZeroVecLike<'b, K1>>::BorrowedVariant>,
    <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, V>>::BorrowedVariant:
        PartialEq<<<V as ZeroMapKV<'b>>::Container as ZeroVecLike<'b, V>>::BorrowedVariant>,
{
    fn eq(&self, other: &ZeroMap2kBorrowed<'b, K0, K1, V>) -> bool {
        self.keys0.eq(&other.keys0)
            && self.joiner.eq(other.joiner)
            && self.keys1.eq(&other.keys1)
            && self.values.eq(&other.values)
    }
}

impl<'a, K0, K1, V> fmt::Debug for ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K0>>::BorrowedVariant: fmt::Debug,
    <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K1>>::BorrowedVariant: fmt::Debug,
    <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, V>>::BorrowedVariant: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("ZeroMap2kBorrowed")
            .field("keys0", &self.keys0)
            .field("joiner", &self.joiner)
            .field("keys1", &self.keys1)
            .field("values", &self.values)
            .finish()
    }
}
