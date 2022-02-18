// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;
use std::sync::RwLock;

#[derive(Debug)]
pub struct FrozenBTreeMap<K: Clone + Ord, V>(RwLock<BTreeMap<K, Box<V>>>);

impl<K: Clone + Ord, V> FrozenBTreeMap<K, V> {
    pub fn new() -> Self {
        Self(RwLock::new(BTreeMap::new()))
    }

    pub fn get_or_insert<'a, E>(
        &'a self,
        k: &K,
        value_fn: impl FnOnce() -> Result<V, E>,
    ) -> Result<&'a V, E> {
        // The RHS of an if-let statement doesn't get dropped before the else block, so we have
        // to do this manually
        let guard = self.0.read().unwrap();
        let ptr: *const V = if let Some(boxx) = guard.get(k) {
            &**boxx
        } else {
            drop(guard);

            // We compute the value outside the or_insert_with because
            // * it can fail
            // * we don't want to hold the lock while computing it
            let value = value_fn()?;

            let mut guard = self.0.write().unwrap();

            &**guard.entry(k.clone()).or_insert_with(|| Box::new(value))
        };

        // Even though we've given up the lock on it, the value is still there
        // because Box is a stable reference and we never drop anything.
        Ok(unsafe { &*ptr })
    }
}
