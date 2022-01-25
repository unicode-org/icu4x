// This module contains a couple simple and purpose built hash maps. The key
// trade off they make is that they serve as caches rather than true maps. That
// is, inserting a new entry may cause eviction of another entry. This gives
// us two things. First, there's less overhead associated with inserts and
// lookups. Secondly, it lets us control our memory usage.
//
// These maps are used in some fairly hot code when generating NFA states for
// large Unicode character classes.
//
// Instead of exposing a rich hashmap entry API, we just permit the caller to
// produce a hash of the key directly. The hash can then be reused for both
// lookups and insertions at the cost of leaking abstraction a bit. But these
// are for internal use only, so it's fine.
//
// The Utf8BoundedMap is used for Daciuk's algorithm for constructing a
// (almost) minimal DFA for large Unicode character classes in linear time.
// (Daciuk's algorithm is always used when compiling forward NFAs. For reverse
// NFAs, it's only used when the compiler is configured to 'shrink' the NFA,
// since there's a bit more expense in the reverse direction.)
//
// The Utf8SuffixMap is used when compiling large Unicode character classes for
// reverse NFAs when 'shrink' is disabled. Specifically, it augments the naive
// construction of UTF-8 automata by caching common suffixes. This doesn't
// get the same space savings as Daciuk's algorithm, but it's basically as
// fast as the naive approach and typically winds up using less memory (since
// it generates smaller NFAs) despite the presence of the cache.
//
// These maps effectively represent caching mechanisms for CState::Sparse and
// CState::Range, respectively. The former represents a single NFA state with
// many transitions of equivalent priority while the latter represents a single
// NFA state with a single transition. (Neither state ever has or is an
// epsilon transition.) Thus, they have different key types. It's likely we
// could make one generic map, but the machinery didn't seem worth it. They
// are simple enough.

use alloc::{vec, vec::Vec};

use crate::regex_automata::{nfa::thompson::Transition, util::id::StateID};

// Basic FNV-1a hash constants as described in:
// https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
const PRIME: u64 = 1099511628211;
const INIT: u64 = 14695981039346656037;

#[derive(Clone, Debug)]
pub struct Utf8BoundedMap {
                            version: u16,
        capacity: usize,
            map: Vec<Utf8BoundedEntry>,
}

#[derive(Clone, Debug, Default)]
struct Utf8BoundedEntry {
                version: u16,
        key: Vec<Transition>,
            val: StateID,
}

impl Utf8BoundedMap {
                                    pub fn new(capacity: usize) -> Utf8BoundedMap {
        assert!(capacity > 0);
        Utf8BoundedMap { version: 0, capacity, map: vec![] }
    }

                    pub fn clear(&mut self) {
        if self.map.is_empty() {
            self.map = vec![Utf8BoundedEntry::default(); self.capacity];
        } else {
            self.version = self.version.wrapping_add(1);
            // If we loop back to version 0, then we forcefully clear the
            // entire map. Otherwise, it might be possible to incorrectly
            // match entries used to generate other NFAs.
            if self.version == 0 {
                self.map = vec![Utf8BoundedEntry::default(); self.capacity];
            }
        }
    }

        pub fn hash(&self, key: &[Transition]) -> usize {
        let mut h = INIT;
        for t in key {
            h = (h ^ (t.start as u64)).wrapping_mul(PRIME);
            h = (h ^ (t.end as u64)).wrapping_mul(PRIME);
            h = (h ^ (t.next.as_usize() as u64)).wrapping_mul(PRIME);
        }
        (h as usize) % self.map.len()
    }

                        pub fn get(&mut self, key: &[Transition], hash: usize) -> Option<StateID> {
        let entry = &self.map[hash];
        if entry.version != self.version {
            return None;
        }
        // There may be a hash collision, so we need to confirm real equality.
        if entry.key != key {
            return None;
        }
        Some(entry.val)
    }

                            pub fn set(
        &mut self,
        key: Vec<Transition>,
        hash: usize,
        state_id: StateID,
    ) {
        self.map[hash] =
            Utf8BoundedEntry { version: self.version, key, val: state_id };
    }
}

#[derive(Clone, Debug)]
pub struct Utf8SuffixMap {
                version: u16,
        capacity: usize,
            map: Vec<Utf8SuffixEntry>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Utf8SuffixKey {
    pub from: StateID,
    pub start: u8,
    pub end: u8,
}

#[derive(Clone, Debug, Default)]
struct Utf8SuffixEntry {
                version: u16,
        key: Utf8SuffixKey,
        val: StateID,
}

impl Utf8SuffixMap {
                                    pub fn new(capacity: usize) -> Utf8SuffixMap {
        assert!(capacity > 0);
        Utf8SuffixMap { version: 0, capacity, map: vec![] }
    }

                    pub fn clear(&mut self) {
        if self.map.is_empty() {
            self.map = vec![Utf8SuffixEntry::default(); self.capacity];
        } else {
            self.version = self.version.wrapping_add(1);
            if self.version == 0 {
                self.map = vec![Utf8SuffixEntry::default(); self.capacity];
            }
        }
    }

        pub fn hash(&self, key: &Utf8SuffixKey) -> usize {
        // Basic FNV-1a hash as described:
        // https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
        const PRIME: u64 = 1099511628211;
        const INIT: u64 = 14695981039346656037;

        let mut h = INIT;
        h = (h ^ (key.from.as_usize() as u64)).wrapping_mul(PRIME);
        h = (h ^ (key.start as u64)).wrapping_mul(PRIME);
        h = (h ^ (key.end as u64)).wrapping_mul(PRIME);
        (h as usize) % self.map.len()
    }

                    pub fn get(
        &mut self,
        key: &Utf8SuffixKey,
        hash: usize,
    ) -> Option<StateID> {
        let entry = &self.map[hash];
        if entry.version != self.version {
            return None;
        }
        if key != &entry.key {
            return None;
        }
        Some(entry.val)
    }

                            pub fn set(&mut self, key: Utf8SuffixKey, hash: usize, state_id: StateID) {
        self.map[hash] =
            Utf8SuffixEntry { version: self.version, key, val: state_id };
    }
}
