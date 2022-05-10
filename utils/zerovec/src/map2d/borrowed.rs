// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::AsULE;
use crate::{ZeroMap2d, ZeroSlice, ZeroVec};

use core::fmt;
use core::ops::Range;

use crate::map::ZeroMapKV;
use crate::map::{BorrowedZeroVecLike, ZeroVecLike};
use crate::map2d::KeyError;

/// A borrowed-only version of [`ZeroMap2d`](super::ZeroMap2d)
///
/// This is useful for fully-zero-copy deserialization from non-human-readable
/// serialization formats. It also has the advantage that it can return references that live for
/// the lifetime of the backing buffer as opposed to that of the [`ZeroMap2dBorrowed`] instance.
///
/// # Examples
///
/// ```
/// use zerovec::maps::ZeroMap2dBorrowed;
///
/// // Example byte buffer representing the map { 1: {2: "three" } }
/// let BINCODE_BYTES: &[u8; 53] = &[
///     2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
///     2, 0, 13, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 116, 104, 114, 101, 101,
/// ];
///
/// // Deserializing to ZeroMap2d requires no heap allocations.
/// let zero_map: ZeroMap2dBorrowed<u16, u16, str> =
///     bincode::deserialize(BINCODE_BYTES).expect("Should deserialize successfully");
/// assert_eq!(zero_map.get(&1, &2), Ok("three"));
/// ```
///
/// This can be obtained from a [`ZeroMap2d`](super::ZeroMap2d) via [`ZeroMap2d::as_borrowed`](super::ZeroMap2d::as_borrowed)
pub struct ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    pub(crate) keys0: &'a <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<K0>>::BorrowedVariant,
    pub(crate) joiner: &'a ZeroSlice<u32>,
    pub(crate) keys1: &'a <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<K1>>::BorrowedVariant,
    pub(crate) values: &'a <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<V>>::BorrowedVariant,
}

impl<'a, K0, K1, V> Copy for ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
}

impl<'a, K0, K1, V> Clone for ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    fn clone(&self) -> Self {
        ZeroMap2dBorrowed {
            keys0: self.keys0,
            joiner: self.joiner,
            keys1: self.keys1,
            values: self.values,
        }
    }
}

impl<'a, K0, K1, V> Default for ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<K0>>::BorrowedVariant: 'static,
    <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<K1>>::BorrowedVariant: 'static,
    <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<V>>::BorrowedVariant: 'static,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K0, K1, V> ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<K0>>::BorrowedVariant: 'static,
    <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<K1>>::BorrowedVariant: 'static,
    <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<V>>::BorrowedVariant: 'static,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// Creates a new, empty `ZeroMap2dBorrowed<K0, K1, V>`.
    ///
    /// Note: Since [`ZeroMap2dBorrowed`] is not mutable, the return value will be a stub unless
    /// converted into a [`ZeroMap2d`](super::ZeroMap2d).
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::maps::ZeroMap2dBorrowed;
    ///
    /// let zm: ZeroMap2dBorrowed<u16, u16, str> = ZeroMap2dBorrowed::new();
    /// assert!(zm.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            keys0:
                <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<K0>>::BorrowedVariant::zvl_new_borrowed(
                ),
            joiner: Default::default(),
            keys1:
                <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<K1>>::BorrowedVariant::zvl_new_borrowed(
                ),
            values:
                <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<V>>::BorrowedVariant::zvl_new_borrowed(),
        }
    }
}

impl<'a, K0, K1, V> ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// The number of elements in the [`ZeroMap2dBorrowed`]
    pub fn len(&self) -> usize {
        self.values.zvl_len()
    }

    /// Whether the [`ZeroMap2dBorrowed`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.zvl_len() == 0
    }
}

impl<'a, K0, K1, V> ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + Ord,
    K1: ZeroMapKV<'a> + Ord,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// Get the value associated with `key0` and `key1`, if it exists.
    ///
    /// This is able to return values that live longer than the map itself
    /// since they borrow directly from the backing buffer. This is the
    /// primary advantage of using [`ZeroMap2dBorrowed`](super::ZeroMap2dBorrowed) over [`ZeroMap2d`](super::ZeroMap2d).
    ///
    /// ```rust
    /// use zerovec::maps::{KeyError, ZeroMap2dBorrowed};
    /// use zerovec::ZeroMap2d;
    ///
    /// let mut map = ZeroMap2d::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "one", "bar");
    /// map.insert(&2, "two", "baz");
    ///
    /// let borrowed = map.as_borrowed();
    /// assert_eq!(borrowed.get(&1, "one"), Ok("foo"));
    /// assert_eq!(borrowed.get(&1, "two"), Err(KeyError::K1));
    /// assert_eq!(borrowed.get(&2, "one"), Ok("bar"));
    /// assert_eq!(borrowed.get(&2, "two"), Ok("baz"));
    /// assert_eq!(borrowed.get(&3, "three"), Err(KeyError::K0));
    ///
    /// let borrow = borrowed.get(&1, "one");
    /// drop(borrowed);
    /// // still exists after the ZeroMap2dBorrowed has been dropped
    /// assert_eq!(borrow, Ok("foo"));
    /// ```
    pub fn get(&self, key0: &K0, key1: &K1) -> Result<&'a V::GetType, KeyError> {
        self.get0(key0)
            .ok_or(KeyError::K0)?
            .get1(key1)
            .ok_or(KeyError::K1)
    }
}

impl<'a, K0, K1, V> ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + Ord,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// Gets a cursor for `key0`. If `None`, then `key0` is not in the map. If `Some`,
    /// then `key0` is in the map, and `key1` can be queried.
    ///
    /// ```rust
    /// use zerovec::maps::ZeroMap2dBorrowed;
    /// use zerovec::ZeroMap2d;
    ///
    /// let mut map = ZeroMap2d::new();
    /// map.insert(&1, "one", "foo");
    /// map.insert(&2, "two", "bar");
    /// let borrowed = map.as_borrowed();
    /// assert!(matches!(borrowed.get0(&1), Some(_)));
    /// assert!(matches!(borrowed.get0(&3), None));
    /// ```
    #[inline]
    pub fn get0<'l>(&'l self, key0: &K0) -> Option<ZeroMap2dCursorBorrowed<'a, 'a, K0, K1, V>> {
        let key0_index = self.keys0.zvl_binary_search(key0).ok()?;
        Some(ZeroMap2dCursorBorrowed {
            keys0: self.keys0,
            joiner: self.joiner,
            keys1: self.keys1,
            values: self.values,
            key0_index,
        })
    }
}

impl<'a, K0, K1, V> ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// Produce an ordered iterator over keys0
    pub fn iter0<'l>(
        &'l self,
    ) -> impl Iterator<Item = ZeroMap2dCursorBorrowed<'a, 'a, K0, K1, V>> + '_ {
        (0..self.keys0.zvl_len()).map(move |idx| ZeroMap2dCursorBorrowed {
            keys0: self.keys0,
            joiner: self.joiner,
            keys1: self.keys1,
            values: self.values,
            key0_index: idx,
        })
    }

    /// Produce an iterator over values, ordered by the pair (key0,key1)
    pub fn iter_values<'b>(&'b self) -> impl Iterator<Item = &'b <V as ZeroMapKV<'a>>::GetType> {
        #[allow(clippy::unwrap_used)] // Iterating over a ZeroVecLike requires this pattern
        (0..self.values.zvl_len()).map(move |idx| self.values.zvl_get(idx).unwrap())
    }
}

impl<'a, K0, K1, V> ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a> + Ord,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    // INTERNAL ROUTINES FOLLOW //
}

impl<'a, K0, K1, V> ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized + Ord,
    K1: ZeroMapKV<'a> + ?Sized + Ord,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
    V: AsULE + Copy,
{
    /// For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`
    pub fn get_copied(&self, key0: &K0, key1: &K1) -> Result<V, KeyError> {
        self.get0(key0)
            .ok_or(KeyError::K0)?
            .get1_copied(key1)
            .ok_or(KeyError::K1)
    }
}

// We can't use the default PartialEq because ZeroMap2d is invariant
// so otherwise rustc will not automatically allow you to compare ZeroMaps
// with different lifetimes
impl<'a, 'b, K0, K1, V> PartialEq<ZeroMap2dBorrowed<'b, K0, K1, V>>
    for ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: for<'c> ZeroMapKV<'c> + ?Sized,
    K1: for<'c> ZeroMapKV<'c> + ?Sized,
    V: for<'c> ZeroMapKV<'c> + ?Sized,
    <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<K0>>::BorrowedVariant:
        PartialEq<<<K0 as ZeroMapKV<'b>>::Container as ZeroVecLike<K0>>::BorrowedVariant>,
    <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<K1>>::BorrowedVariant:
        PartialEq<<<K1 as ZeroMapKV<'b>>::Container as ZeroVecLike<K1>>::BorrowedVariant>,
    <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<V>>::BorrowedVariant:
        PartialEq<<<V as ZeroMapKV<'b>>::Container as ZeroVecLike<V>>::BorrowedVariant>,
{
    fn eq(&self, other: &ZeroMap2dBorrowed<'b, K0, K1, V>) -> bool {
        self.keys0.eq(other.keys0)
            && self.joiner.eq(other.joiner)
            && self.keys1.eq(other.keys1)
            && self.values.eq(other.values)
    }
}

impl<'a, K0, K1, V> fmt::Debug for ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<K0>>::BorrowedVariant: fmt::Debug,
    <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<K1>>::BorrowedVariant: fmt::Debug,
    <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<V>>::BorrowedVariant: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("ZeroMap2dBorrowed")
            .field("keys0", &self.keys0)
            .field("joiner", &self.joiner)
            .field("keys1", &self.keys1)
            .field("values", &self.values)
            .finish()
    }
}

pub struct ZeroMap2dCursorBorrowed<'l, 'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    keys0: &'l <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<K0>>::BorrowedVariant,
    joiner: &'l ZeroSlice<u32>,
    keys1: &'l <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<K1>>::BorrowedVariant,
    values: &'l <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<V>>::BorrowedVariant,
    key0_index: usize,
}

impl<'l, 'a, K0, K1, V> ZeroMap2dCursorBorrowed<'l, 'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    pub(crate) fn from_cow(cow: &'l ZeroMap2d<'a, K0, K1, V>, key0_index: usize) -> Self {
        Self {
            keys0: cow.keys0.zvl_as_borrowed(),
            joiner: &cow.joiner,
            keys1: cow.keys1.zvl_as_borrowed(),
            values: cow.values.zvl_as_borrowed(),
            key0_index,
        }
    }

    pub fn key0(&self) -> &'l K0::GetType {
        self.keys0.zvl_get(self.key0_index).unwrap()
    }

    /// Produce an ordered iterator over keys1 for a particular key0, if key0 exists
    pub fn iter1(
        &self,
    ) -> impl Iterator<
        Item = (
            &'l <K1 as ZeroMapKV<'a>>::GetType,
            &'l <V as ZeroMapKV<'a>>::GetType,
        ),
    > + '_ {
        let range = self.get_range();
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        range.map(move |idx| {
            (
                self.keys1.zvl_get(idx).unwrap(),
                self.values.zvl_get(idx).unwrap(),
            )
        })
    }

    /// Given key0_index, returns the corresponding range of keys1
    fn get_range(&self) -> Range<usize> {
        debug_assert!(self.key0_index < self.joiner.len());
        let start = if self.key0_index == 0 {
            0
        } else {
            #[allow(clippy::unwrap_used)] // protected by the debug_assert above
            self.joiner.get(self.key0_index - 1).unwrap()
        };
        #[allow(clippy::unwrap_used)] // protected by the debug_assert above
        let limit = self.joiner.get(self.key0_index).unwrap();
        (start as usize)..(limit as usize)
    }
}

impl<'l, 'a, K0, K1, V> ZeroMap2dCursorBorrowed<'l, 'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a> + Ord,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    pub fn get1(&self, key1: &K1) -> Option<&'l V::GetType> {
        let key1_index = self.get_key1_index(key1)?;
        #[allow(clippy::unwrap_used)] // key1_index is valid
        Some(self.values.zvl_get(key1_index).unwrap())
    }

    // pub fn get1_by(&self, predicate: impl FnMut(&K1) -> Ordering) -> Option<&V::GetType> {
    //     let range = self.map.get_range_for_key0_index(self.key0_index);
    //     debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
    //     debug_assert!(range.end <= self.map.keys1.zvl_len());
    //     #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
    //     let index = range.start
    //         + self
    //             .map
    //             .keys1
    //             .zvl_binary_search_in_range(key1, range)
    //             .expect("in-bounds range")
    //             .ok()?;
    //     // This unwrap is protected by the invariant keys1.len() == values.len(),
    //     // the above debug_assert!, and the contract of zvl_binary_search_in_range.
    //     #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
    //     Some(self.map.values.zvl_get(index).unwrap())
    // }

    /// Given key0_index and key1, returns the index into the values array
    fn get_key1_index(&self, key1: &K1) -> Option<usize> {
        let range = self.get_range();
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        let start = range.start;
        #[allow(clippy::expect_used)] // protected by the debug_assert above
        let binary_search_result = self
            .keys1
            .zvl_binary_search_in_range(key1, range)
            .expect("in-bounds range");
        binary_search_result.ok().map(move |s| s + start)
    }
}

impl<'l, 'a, K0, K1, V> ZeroMap2dCursorBorrowed<'l, 'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized + Ord,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>>,
    V: AsULE + Copy,
{
    pub fn get1_copied(&self, key1: &K1) -> Option<V> {
        let key1_index = self.get_key1_index(key1)?;
        self.values.get(key1_index)
    }
}
