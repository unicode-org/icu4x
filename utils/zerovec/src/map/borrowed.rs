// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::AsULE;
use crate::ZeroVec;

use core::fmt;

pub use super::kv::ZeroMapKV;
pub use super::vecs::{BorrowedZeroVecLike, MutableZeroVecLike, ZeroVecLike};

/// A borrowed-only version of [`ZeroMap`](super::ZeroMap)
///
/// This is useful for fully-zero-copy deserialization from non-human-readable
/// serialization formats. It also has the advantage that it can return references that live for
/// the lifetime of the backing buffer as opposed to that of the [`ZeroMapBorrowed`] instance.
///
/// # Examples
///
/// ```
/// use zerovec::map::ZeroMapBorrowed;
///
/// // Example byte buffer representing the map { 1: "one" }
/// let BINCODE_BYTES: &[u8; 31] = &[
///     4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0,
///     1, 0, 0, 0, 0, 0, 0, 0, 111, 110, 101
/// ];
///
/// // Deserializing to ZeroMap requires no heap allocations.
/// let zero_map: ZeroMapBorrowed<u32, str> = bincode::deserialize(BINCODE_BYTES)
///     .expect("Should deserialize successfully");
/// assert_eq!(zero_map.get(&1), Some("one"));
/// ```
///
/// This can be obtained from a [`ZeroMap`](super::ZeroMap) via [`ZeroMap::as_borrowed`](super::ZeroMap::as_borrowed)
pub struct ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K: ?Sized,
    V: ?Sized,
{
    pub(crate) keys:
        <<K as ZeroMapKV<'a>>::Container as MutableZeroVecLike<'a, K>>::BorrowedVariant,
    pub(crate) values:
        <<V as ZeroMapKV<'a>>::Container as MutableZeroVecLike<'a, V>>::BorrowedVariant,
}

impl<'a, K, V> Copy for ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K: ?Sized,
    V: ?Sized,
{
}
impl<'a, K, V> Clone for ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K: ?Sized,
    V: ?Sized,
{
    fn clone(&self) -> Self {
        ZeroMapBorrowed {
            keys: self.keys,
            values: self.values,
        }
    }
}

impl<'a, K, V> Default for ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K: ?Sized,
    V: ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K, V> ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K: ?Sized,
    V: ?Sized,
{
    /// Creates a new, empty `ZeroMapBorrowed<K, V>`.
    ///
    /// Note: Since [`ZeroMapBorrowed`] is not mutable, the return value will be a stub unless
    /// converted into a [`ZeroMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::map::ZeroMapBorrowed;
    ///
    /// let zm: ZeroMapBorrowed<u16, str> = ZeroMapBorrowed::new();
    /// assert!(zm.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            keys:
                <<K as ZeroMapKV<'a>>::Container as MutableZeroVecLike<'a, K>>::BorrowedVariant::new(
                ),
            values:
                <<V as ZeroMapKV<'a>>::Container as MutableZeroVecLike<'a, V>>::BorrowedVariant::new(
                ),
        }
    }

    /// The number of elements in the [`ZeroMapBorrowed`]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the [`ZeroMapBorrowed`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.len() == 0
    }

    /// Get the value associated with `key`, if it exists.
    ///
    /// This is able to return values that live longer than the map itself
    /// since they borrow directly from the backing buffer. This is the
    /// primary advantage of using [`ZeroMapBorrowed`] over [`ZeroMap`].
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    /// use zerovec::map::ZeroMapBorrowed;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(&1, "one");
    /// map.insert(&2, "two");
    /// let borrowed = map.as_borrowed();
    /// assert_eq!(borrowed.get(&1), Some("one"));
    /// assert_eq!(borrowed.get(&3), None);
    ///
    /// let borrow = borrowed.get(&1);
    /// drop(borrowed);
    /// // still exists after the ZeroMapBorrowed has been dropped
    /// assert_eq!(borrow, Some("one"));
    /// ```
    pub fn get(&self, key: &K::NeedleType) -> Option<&'a V::GetType> {
        let index = self.keys.binary_search(key).ok()?;
        self.values.get_borrowed(index)
    }

    /// Returns whether `key` is contained in this map
    ///
    /// ```rust
    /// use zerovec::ZeroMap;
    /// use zerovec::map::ZeroMapBorrowed;
    ///
    /// let mut map = ZeroMap::new();
    /// map.insert(&1, "one");
    /// map.insert(&2, "two");
    /// let borrowed = map.as_borrowed();
    /// assert_eq!(borrowed.contains_key(&1), true);
    /// assert_eq!(borrowed.contains_key(&3), false);
    /// ```
    pub fn contains_key(&self, key: &K::NeedleType) -> bool {
        self.keys.binary_search(key).is_ok()
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
        (0..self.keys.len()).map(move |idx| {
            (
                self.keys.get_borrowed(idx).unwrap(),
                self.values.get_borrowed(idx).unwrap(),
            )
        })
    }

    /// Produce an ordered iterator over keys
    pub fn iter_keys<'b>(&'b self) -> impl Iterator<Item = &'a <K as ZeroMapKV<'a>>::GetType> + 'b {
        (0..self.keys.len()).map(move |idx| self.keys.get_borrowed(idx).unwrap())
    }

    /// Produce an iterator over values, ordered by keys
    pub fn iter_values<'b>(
        &'b self,
    ) -> impl Iterator<Item = &'a <V as ZeroMapKV<'a>>::GetType> + 'b {
        (0..self.values.len()).map(move |idx| self.values.get_borrowed(idx).unwrap())
    }
}

impl<'a, K, V> ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
    V: AsULE + Ord + Copy,
{
    /// For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`
    pub fn get_copied(&self, key: &K::NeedleType) -> Option<V> {
        let index = self.keys.binary_search(key).ok()?;
        <[V::ULE]>::get(self.values, index)
            .copied()
            .map(V::from_unaligned)
    }

    /// Similar to [`Self::iter()`] except it returns a direct copy of the values instead of references
    /// to `V::ULE`, in cases when `V` is fixed-size
    pub fn iter_copied_values<'b>(
        &'b self,
    ) -> impl Iterator<Item = (&'b <K as ZeroMapKV<'a>>::GetType, V)> {
        (0..self.keys.len()).map(move |idx| {
            (
                self.keys.get(idx).unwrap(),
                <[V::ULE]>::get(self.values, idx)
                    .copied()
                    .map(V::from_unaligned)
                    .unwrap(),
            )
        })
    }
}

impl<'a, K, V> ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a, Container = ZeroVec<'a, K>> + ?Sized,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>> + ?Sized,
    K: AsULE + Copy + Ord,
    V: AsULE + Copy + Ord,
{
    /// Similar to [`Self::iter()`] except it returns a direct copy of the keys values instead of references
    /// to `K::ULE` and `V::ULE`, in cases when `K` and `V` are fixed-size
    #[allow(clippy::needless_lifetimes)] // Lifetime is necessary in impl Trait
    pub fn iter_copied<'b>(&'b self) -> impl Iterator<Item = (K, V)> + 'b {
        let keys = &self.keys;
        let values = &self.values;
        let len = <[K::ULE]>::len(keys);
        (0..len).map(move |idx| {
            (
                <[K::ULE]>::get(keys, idx)
                    .copied()
                    .map(K::from_unaligned)
                    .unwrap(),
                <[V::ULE]>::get(values, idx)
                    .copied()
                    .map(V::from_unaligned)
                    .unwrap(),
            )
        })
    }
}

// We can't use the default PartialEq because ZeroMap is invariant
// so otherwise rustc will not automatically allow you to compare ZeroMaps
// with different lifetimes
impl<'a, 'b, K, V> PartialEq<ZeroMapBorrowed<'b, K, V>> for ZeroMapBorrowed<'a, K, V>
where
    K: for<'c> ZeroMapKV<'c> + ?Sized,
    V: for<'c> ZeroMapKV<'c> + ?Sized,
    <<K as ZeroMapKV<'a>>::Container as MutableZeroVecLike<'a, K>>::BorrowedVariant:
        PartialEq<<<K as ZeroMapKV<'b>>::Container as MutableZeroVecLike<'b, K>>::BorrowedVariant>,
    <<V as ZeroMapKV<'a>>::Container as MutableZeroVecLike<'a, V>>::BorrowedVariant:
        PartialEq<<<V as ZeroMapKV<'b>>::Container as MutableZeroVecLike<'b, V>>::BorrowedVariant>,
{
    fn eq(&self, other: &ZeroMapBorrowed<'b, K, V>) -> bool {
        self.keys.eq(&other.keys) && self.values.eq(&other.values)
    }
}

impl<'a, K, V> fmt::Debug for ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    <<K as ZeroMapKV<'a>>::Container as MutableZeroVecLike<'a, K>>::BorrowedVariant: fmt::Debug,
    <<V as ZeroMapKV<'a>>::Container as MutableZeroVecLike<'a, V>>::BorrowedVariant: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("ZeroMapBorrowed")
            .field("keys", &self.keys)
            .field("values", &self.values)
            .finish()
    }
}
