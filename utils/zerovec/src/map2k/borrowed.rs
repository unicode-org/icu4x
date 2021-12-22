// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::AsULE;
use crate::{ZeroSlice, ZeroVec};

use core::fmt;

use crate::map::ZeroMapKV;
use crate::map::{BorrowedZeroVecLike, MutableZeroVecLike, ZeroVecLike};

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

    /*
        /// Get the value associated with `key`, if it exists.
        ///
        /// This is able to return values that live longer than the map itself
        /// since they borrow directly from the backing buffer. This is the
        /// primary advantage of using [`ZeroMap2kBorrowed`](super::ZeroMap2kBorrowed) over [`ZeroMap2k`](super::ZeroMap2k).
        ///
        /// ```rust
        /// use zerovec::ZeroMap2k;
        /// use zerovec::map::ZeroMap2kBorrowed;
        ///
        /// let mut map = ZeroMap2k::new();
        /// map.insert(&1, "one");
        /// map.insert(&2, "two");
        /// let borrowed = map.as_borrowed();
        /// assert_eq!(borrowed.get(&1), Some("one"));
        /// assert_eq!(borrowed.get(&3), None);
        ///
        /// let borrow = borrowed.get(&1);
        /// drop(borrowed);
        /// // still exists after the ZeroMap2kBorrowed has been dropped
        /// assert_eq!(borrow, Some("one"));
        /// ```
        pub fn get(&self, key: &K) -> Option<&'a V::GetType> {
            todo!()
        }

        /// Returns whether `key` is contained in this map
        ///
        /// ```rust
        /// use zerovec::ZeroMap2k;
        /// use zerovec::map::ZeroMap2kBorrowed;
        ///
        /// let mut map = ZeroMap2k::new();
        /// map.insert(&1, "one");
        /// map.insert(&2, "two");
        /// let borrowed = map.as_borrowed();
        /// assert_eq!(borrowed.contains_key(&1), true);
        /// assert_eq!(borrowed.contains_key(&3), false);
        /// ```
        pub fn contains_key(&self, key: &K) -> bool {
            self.keys.zvl_binary_search(key).is_ok()
        }

        /// Produce an ordered iterator over key-value pairs
        pub fn iter<'b>(
            &'b self,
        ) -> impl Iterator<
            Item = (
                &'a <K as ZeroMapKV<'a>>::GetType,
                &'a <V as ZeroMapKV<'a>>::GetType,
            ),
        > + 'b {
            (0..self.keys.zvl_len()).map(move |idx| {
                (
                    self.keys.zvl_get_borrowed(idx).unwrap(),
                    self.values.zvl_get_borrowed(idx).unwrap(),
                )
            })
        }

        /// Produce an ordered iterator over keys
        pub fn iter_keys<'b>(&'b self) -> impl Iterator<Item = &'a <K as ZeroMapKV<'a>>::GetType> + 'b {
            (0..self.keys.zvl_len()).map(move |idx| self.keys.zvl_get_borrowed(idx).unwrap())
        }

        /// Produce an iterator over values, ordered by keys
        pub fn iter_values<'b>(
            &'b self,
        ) -> impl Iterator<Item = &'a <V as ZeroMapKV<'a>>::GetType> + 'b {
            (0..self.values.zvl_len()).map(move |idx| self.values.zvl_get_borrowed(idx).unwrap())
        }
    */
}

/*
impl<'a, K0, K1, V> ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
    V: AsULE + Ord + Copy + 'static,
{
    /// For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`
    pub fn get_copied(&self, key: &K) -> Option<V> {
        let index = self.keys.zvl_binary_search(key).ok()?;
        self.values.get(index)
    }

    /// Similar to [`Self::iter()`] except it returns a direct copy of the values instead of references
    /// to `V::ULE`, in cases when `V` is fixed-size
    pub fn iter_copied_values<'b>(
        &'b self,
    ) -> impl Iterator<Item = (&'b <K as ZeroMapKV<'a>>::GetType, V)> {
        (0..self.keys.zvl_len()).map(move |idx| {
            (
                self.keys.zvl_get(idx).unwrap(),
                self.values.get(idx).unwrap(),
            )
        })
    }
}

impl<'a, K0, K1, V> ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K: ZeroMapKV<'a, Container = ZeroVec<'a, K>> + ?Sized,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
    K: AsULE + Copy + Ord + 'static,
    V: AsULE + Copy + Ord + 'static,
{
    /// Similar to [`Self::iter()`] except it returns a direct copy of the keys values instead of references
    /// to `K::ULE` and `V::ULE`, in cases when `K` and `V` are fixed-size
    #[allow(clippy::needless_lifetimes)] // Lifetime is necessary in impl Trait
    pub fn iter_copied<'b: 'a>(&'b self) -> impl Iterator<Item = (K0, K1, V)> + 'b {
        let keys = &*self.keys;
        let values = &*self.values;
        let len = self.keys.zvl_len();
        (0..len).map(move |idx| {
            (
                ZeroSlice::get(keys, idx).unwrap(),
                ZeroSlice::get(values, idx).unwrap(),
            )
        })
    }
}

// We can't use the default PartialEq because ZeroMap2k is invariant
// so otherwise rustc will not automatically allow you to compare ZeroMaps
// with different lifetimes
impl<'a, 'b, K0, K1, V> PartialEq<ZeroMap2kBorrowed<'b, K0, K1, V>> for ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K: for<'c> ZeroMapKV<'c> + ?Sized,
    V: for<'c> ZeroMapKV<'c> + ?Sized,
    <<K as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K>>::BorrowedVariant:
        PartialEq<<<K as ZeroMapKV<'b>>::Container as ZeroVecLike<'b, K>>::BorrowedVariant>,
    <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, V>>::BorrowedVariant:
        PartialEq<<<V as ZeroMapKV<'b>>::Container as ZeroVecLike<'b, V>>::BorrowedVariant>,
{
    fn eq(&self, other: &ZeroMap2kBorrowed<'b, K0, K1, V>) -> bool {
        self.keys.eq(&other.keys) && self.values.eq(&other.values)
    }
}

impl<'a, K0, K1, V> fmt::Debug for ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <<K as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, K>>::BorrowedVariant: fmt::Debug,
    <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, V>>::BorrowedVariant: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("ZeroMap2kBorrowed")
            .field("keys", &self.keys)
            .field("values", &self.values)
            .finish()
    }
}
*/
