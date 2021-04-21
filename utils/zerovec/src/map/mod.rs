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

impl<'a, K, V> ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
{
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
