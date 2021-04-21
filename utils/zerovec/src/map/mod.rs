// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::cmp::Ordering;

mod kv;
mod vecs;

pub use kv::ZeroMapKV;
pub use vecs::ZeroVecLike;

pub struct ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
{
    keys: K::Container,
    values: V::Container,
}

impl<'a, K, V> Default for ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>, {
        fn default() -> Self {
            Self {
                keys: K::Container::new(),
                values: V::Container::new()
            }
        }
    }

impl<'a, K, V> ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
{
    /// Construct a new [`ZeroMap`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a new [`ZeroMap`] with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
                keys: K::Container::with_capacity(capacity),
                values: V::Container::with_capacity(capacity)
        }
    }

    /// The number of elements in the [`ZeroMap`]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the [`ZeroMap`] is empty
    pub fn is_empty(&self) -> bool {
        self.values.len() == 0
    }

    /// Remove all elements from the [`ZeroMap`]
    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }

    /// Reserve capacity for `additional` more elements to be inserted into
    /// the [`ZeroMap`] to avoid frequent reallocations.
    ///
    /// See [`Vec::reserve()`] for more information.
    pub fn reserve(&mut self, additional: usize) {
        self.keys.reserve(additional);
        self.values.reserve(additional);
    }

    pub fn get(&self, key: &K::NeedleType) -> Option<&V::GetType> {
        let index = self.keys.binary_search(key).ok()?;
        self.values.get(index)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let key_needle = key.as_needle();
        self.keys.binary_search(key_needle).is_ok()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let key_needle = key.as_needle();
        match self.keys.binary_search(key_needle) {
            Ok(index) => Some(self.values.replace(index, value)),
            Err(index) => {
                self.keys.insert(index, key);
                self.values.insert(index, value);
                None
            }
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let key_needle = key.as_needle();
        let idx = self.keys.binary_search(key_needle).ok()?;
        self.keys.remove(idx);
        Some(self.values.remove(idx))
    }

    pub fn try_append(&mut self, key: K, value: V) -> Option<(K, V)> {
        if self.keys.len() == 0 {
            return Some((key, value))
        }
        if let Some(last) = self.keys.get(self.keys.len() - 1) {
            if key.cmp_get(last) != Ordering::Greater {
                return Some((key, value));
            }
        }

        self.keys.push(key);
        self.values.push(value);
        None
    }
}
