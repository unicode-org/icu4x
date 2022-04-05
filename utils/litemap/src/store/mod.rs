// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for pluggable LiteMap backends.
//!
//! By default, LiteMap is backed by a [`Vec`]. However, in some environments, it may be desirable
//! to use a different data store while still using LiteMap to manage proper ordering of items.
//!
//! The general guidelines for a performant data store are:
//!
//! 1. Must support efficient random access for binary search
//! 2. Should support efficient append operations for deserialization
//!
//! To plug a custom data store into LiteMap, implement [`Store`] plus one or the other of:
//!
//! 1. [`StoreSlice`] if the store is able to return a slice reference, or
//! 2. [`StoreBase`] and [`StoreIterable`] if not.
//!
//! Also implement [`StoreFromIterator`] to enable `FromIterator` for LiteMap.

mod slice_impl;
mod vec_impl;

use core::cmp::Ordering;
use core::iter::DoubleEndedIterator;
use core::iter::FromIterator;
use core::iter::Iterator;

/// Trait to enable pluggable backends for LiteMap.
///
/// Some methods have default implementations provided for convenience; however, it is generally
/// better to implement all methods that your data store supports.
pub trait Store<K, V>: StoreBase<K, V> {
    type KeyValueIntoIter: Iterator<Item = (K, V)>;

    /// Creates a new store with the specified capacity hint.
    ///
    /// Implementations may ignore the argument if they do not support pre-allocating capacity.
    fn lm_with_capacity(capacity: usize) -> Self;

    /// Reserves additional capacity in the store.
    ///
    /// Implementations may ignore the argument if they do not support pre-allocating capacity.
    fn lm_reserve(&mut self, additional: usize);

    /// Pushes one additional item onto the store.
    fn lm_push(&mut self, key: K, value: V);

    /// Inserts an item at a specific index in the store.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than the length.
    fn lm_insert(&mut self, index: usize, key: K, value: V);

    /// Removes an item at a specific index in the store.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than the length.
    fn lm_remove(&mut self, index: usize) -> (K, V);

    /// Removes all items from the store.
    fn lm_clear(&mut self);

    /// Adds items from another store to the end of this store.
    fn lm_extend_end(&mut self, other: Self)
    where
        Self: Sized,
    {
        for item in other.lm_into_iter() {
            self.lm_push(item.0, item.1);
        }
    }

    /// Adds items from another store to the beginning of this store.
    fn lm_extend_start(&mut self, other: Self)
    where
        Self: Sized,
    {
        let mut i = 0;
        for item in other.lm_into_iter() {
            self.lm_insert(i, item.0, item.1);
            i += 1;
        }
    }

    /// Retains items satisfying a predicate in this store.
    fn lm_retain<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        let mut i = 0;
        while i < self.lm_len() {
            let (k, v) = self.lm_get(i).expect("i is in range");
            if predicate(k, v) {
                i += 1;
            } else {
                self.lm_remove(i);
            }
        }
    }

    /// Returns an iterator that moves every item from this store.
    fn lm_into_iter(self) -> Self::KeyValueIntoIter;
}

/// Basic methods for the LiteMap store.
///
/// This trait is auto-implemented on anything implementing [`StoreSlice`].
pub trait StoreBase<K, V> {
    /// Returns the number of elements in the store.
    fn lm_len(&self) -> usize;

    /// Returns whether the store is empty (contains 0 elements).
    fn lm_is_empty(&self) -> bool;

    /// Gets a key/value pair at the specified index.
    fn lm_get(&self, index: usize) -> Option<(&K, &V)>;

    /// Gets a key/value pair at the specified index, with a mutable value.
    fn lm_get_mut(&mut self, index: usize) -> Option<(&K, &mut V)>;

    /// Gets the last element in the store, or None if the store is empty.
    fn lm_last(&self) -> Option<(&K, &V)>;

    /// Searches the store for a particular element with a comparator function.
    ///
    /// See the binary search implementation on `slice` for more information.
    fn lm_binary_search_by<F>(&self, cmp: F) -> Result<usize, usize>
    where
        F: FnMut(&K) -> Ordering;
}

/// Iterator methods for the LiteMap store.
///
/// This trait is auto-implemented on anything implementing [`StoreSlice`].
pub trait StoreIterable<'a, K: 'a, V: 'a>: Store<K, V> {
    type KeyValueIter: Iterator<Item = (&'a K, &'a V)> + DoubleEndedIterator + 'a;
    type KeyValueIterMut: Iterator<Item = (&'a K, &'a mut V)> + DoubleEndedIterator + 'a;

    /// Returns an iterator over key/value pairs.
    fn lm_iter(&'a self) -> Self::KeyValueIter;

    /// Returns an iterator over key/value pairs, with a mutable value.
    fn lm_iter_mut(&'a mut self) -> Self::KeyValueIterMut;
}

/// Stores that support being borrowed as slices.
///
/// Implementing this trait causes [`StoreBase`] and [`StoreIterable`] to be auto-implemented.
pub trait StoreSlice<K, V>: StoreBase<K, V> {
    /// Borrows the store as a slice of tuples.
    fn lm_as_slice(&self) -> &[(K, V)];

    /// Borrows the store as a mutable slice of tuples.
    fn lm_as_mut_slice(&mut self) -> &mut [(K, V)];
}

/// A store that can be built from a tuple iterator.
pub trait StoreFromIterator<K, V>: FromIterator<(K, V)> {}
