use alloc::{boxed::Box, vec, vec::Vec};

use crate::regex_automata::util::id::StateID;

#[derive(Clone, Debug)]
pub(crate) struct SparseSets {
    pub(crate) set1: SparseSet,
    pub(crate) set2: SparseSet,
}

impl SparseSets {
    pub(crate) fn new(capacity: usize) -> SparseSets {
        SparseSets {
            set1: SparseSet::new(capacity),
            set2: SparseSet::new(capacity),
        }
    }

    #[inline]
    pub(crate) fn resize(&mut self, new_capacity: usize) {
        self.set1.resize(new_capacity);
        self.set2.resize(new_capacity);
    }

    pub(crate) fn clear(&mut self) {
        self.set1.clear();
        self.set2.clear();
    }

    pub(crate) fn swap(&mut self) {
        core::mem::swap(&mut self.set1, &mut self.set2);
    }

    pub(crate) fn memory_usage(&self) -> usize {
        self.set1.memory_usage() + self.set2.memory_usage()
    }
}

#[derive(Clone)]
pub(crate) struct SparseSet {
    len: usize,
    dense: Vec<StateID>,
    sparse: Vec<StateID>,
}

impl SparseSet {
    #[inline]
    pub(crate) fn new(capacity: usize) -> SparseSet {
        let mut set = SparseSet {
            len: 0,
            dense: vec![],
            sparse: vec![],
        };
        set.resize(capacity);
        set
    }

    #[inline]
    pub(crate) fn resize(&mut self, new_capacity: usize) {
        assert!(
            new_capacity <= StateID::LIMIT,
            "sparse set capacity cannot excced {:?}",
            StateID::LIMIT
        );
        self.clear();
        self.dense.resize(new_capacity, StateID::ZERO);
        self.sparse.resize(new_capacity, StateID::ZERO);
    }

    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        self.dense.len()
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub(crate) fn insert(&mut self, value: StateID) -> bool {
        if self.contains(value) {
            return false;
        }

        let i = self.len();
        assert!(
            i < self.capacity(),
            "{:?} exceeds capacity of {:?} when inserting {:?}",
            i,
            self.capacity(),
            value,
        );
        // OK since i < self.capacity() and self.capacity() is guaranteed to
        // be <= StateID::LIMIT.
        let id = StateID::new_unchecked(i);
        self.dense[id] = value;
        self.sparse[value] = id;
        self.len += 1;
        true
    }

    #[inline]
    pub(crate) fn contains(&self, value: StateID) -> bool {
        let i = self.sparse[value];
        i.as_usize() < self.len() && self.dense[i] == value
    }

    #[inline]
    pub(crate) fn get(&self, i: usize) -> StateID {
        self.dense[i]
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub(crate) fn memory_usage(&self) -> usize {
        2 * self.dense.len() * StateID::SIZE
    }
}

impl core::fmt::Debug for SparseSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let elements: Vec<StateID> = self.into_iter().collect();
        f.debug_tuple("SparseSet").field(&elements).finish()
    }
}

#[derive(Debug)]
pub(crate) struct SparseSetIter<'a>(core::slice::Iter<'a, StateID>);

impl<'a> IntoIterator for &'a SparseSet {
    type Item = StateID;
    type IntoIter = SparseSetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SparseSetIter(self.dense[..self.len()].iter())
    }
}

impl<'a> Iterator for SparseSetIter<'a> {
    type Item = StateID;

    #[inline(always)]
    fn next(&mut self) -> Option<StateID> {
        self.0.next().map(|value| *value)
    }
}
