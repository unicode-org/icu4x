// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::error::Error;
use alloc::collections::btree_map::Entry;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub struct PerfectByteHashMapCacheOwned {
    // Note: This should probably be a HashMap but that isn't in `alloc`
    data: BTreeMap<Vec<u8>, PerfectByteHashMap<Vec<u8>>>,
}

impl PerfectByteHashMapCacheOwned {
    pub fn new_empty() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    pub fn try_get_or_insert(&mut self, keys: Vec<u8>) -> Result<&PerfectByteHashMap<[u8]>, Error> {
        let mut_phf = match self.data.entry(keys) {
            Entry::Vacant(entry) => {
                let value = PerfectByteHashMap::try_new(entry.key())?;
                entry.insert(value)
            }
            Entry::Occupied(entry) => entry.into_mut(),
        };
        Ok(mut_phf.as_borrowed())
    }

    pub fn get(&self, keys: &[u8]) -> Option<&PerfectByteHashMap<[u8]>> {
        self.data.get(keys).map(|p| p.as_borrowed())
    }
}
