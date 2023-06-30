// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains internal collections for the const builder.

use super::super::branch_meta::BranchMeta;

/// A const-friendly slice type.
#[derive(Debug, Copy, Clone)]
pub(crate) struct ConstSlice<'a, T> {
    full_slice: &'a [T],
    start: usize,
    limit: usize,
}

impl<'a, T> ConstSlice<'a, T> {
    pub const fn from_slice(other: &'a [T]) -> Self {
        ConstSlice {
            full_slice: other,
            start: 0,
            limit: other.len(),
        }
    }

    pub const fn from_manual_slice(full_slice: &'a [T], start: usize, limit: usize) -> Self {
        ConstSlice {
            full_slice,
            start,
            limit,
        }
    }

    pub const fn len(&self) -> usize {
        self.limit - self.start
    }

    pub const fn get_or_panic(&self, index: usize) -> &T {
        &self.full_slice[index + self.start]
    }

    #[cfg(test)]
    pub const fn first(&self) -> Option<&T> {
        if self.len() == 0 {
            None
        } else {
            Some(self.get_or_panic(0))
        }
    }

    pub const fn last(&self) -> Option<&T> {
        if self.len() == 0 {
            None
        } else {
            Some(self.get_or_panic(self.len() - 1))
        }
    }

    #[cfg(test)]
    pub const fn get_subslice_or_panic(
        &self,
        new_start: usize,
        new_limit: usize,
    ) -> ConstSlice<'a, T> {
        assert!(new_start <= new_limit);
        assert!(new_limit <= self.len());
        ConstSlice {
            full_slice: self.full_slice,
            start: self.start + new_start,
            limit: self.start + new_limit,
        }
    }

    #[cfg(any(test, feature = "alloc"))]
    pub fn as_slice(&self) -> &'a [T] {
        &self.full_slice[self.start..self.limit]
    }
}

impl<'a, T> From<&'a [T]> for ConstSlice<'a, T> {
    fn from(other: &'a [T]) -> Self {
        Self::from_slice(other)
    }
}

/// A const-friendly mutable data structure backed by an array.
#[derive(Debug, Copy, Clone)]
pub(crate) struct ConstArrayBuilder<const N: usize, T> {
    full_array: [T; N],
    start: usize,
    limit: usize,
}

impl<const N: usize, T: Default> Default for ConstArrayBuilder<N, T> {
    fn default() -> Self {
        Self::new_empty([(); N].map(|_| Default::default()), 0)
    }
}

impl<const N: usize, T> ConstArrayBuilder<N, T> {
    pub const fn new_empty(full_array: [T; N], cursor: usize) -> Self {
        assert!(cursor <= N);
        Self {
            full_array,
            start: cursor,
            limit: cursor,
        }
    }

    pub const fn from_manual_slice(full_array: [T; N], start: usize, limit: usize) -> Self {
        assert!(start <= limit);
        assert!(limit <= N);
        Self {
            full_array,
            start,
            limit,
        }
    }

    pub const fn len(&self) -> usize {
        self.limit - self.start
    }

    #[allow(dead_code)]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn as_const_slice(&self) -> ConstSlice<T> {
        ConstSlice::from_manual_slice(&self.full_array, self.start, self.limit)
    }

    #[cfg(feature = "alloc")]
    pub fn as_slice(&self) -> &[T] {
        &self.full_array[self.start..self.limit]
    }
}

impl<const N: usize> ConstArrayBuilder<N, u8> {
    pub const fn const_bitor_assign(mut self, index: usize, other: u8) -> Self {
        self.full_array[self.start + index] |= other;
        self
    }
    // Can't be generic because T has a destructor
    pub const fn const_take_or_panic(self) -> [u8; N] {
        if self.start != 0 || self.limit != N {
            panic!("AsciiTrieBuilder buffer too large");
        }
        self.full_array
    }
    // Can't be generic because T has a destructor
    pub const fn const_push_front_or_panic(mut self, value: u8) -> Self {
        if self.start == 0 {
            panic!("AsciiTrieBuilder buffer too small");
        }
        self.start -= 1;
        self.full_array[self.start] = value;
        self
    }
    // Can't be generic because T has a destructor
    pub const fn const_extend_front_or_panic(mut self, other: ConstSlice<u8>) -> Self {
        if self.start < other.len() {
            panic!("AsciiTrieBuilder buffer too small");
        }
        self.start -= other.len();
        let mut i = self.start;
        const_for_each!(other, byte, {
            self.full_array[i] = *byte;
            i += 1;
        });
        self
    }
}

impl<const N: usize, T: Copy> ConstArrayBuilder<N, T> {
    pub const fn push_front_or_panic(mut self, value: T) -> Self {
        if self.start == 0 {
            panic!("AsciiTrieBuilder buffer too small");
        }
        self.start -= 1;
        self.full_array[self.start] = value;
        self
    }
    #[cfg(feature = "alloc")]
    pub fn swap_or_panic(mut self, i: usize, j: usize) -> Self {
        self.full_array.swap(self.start + i, self.start + j);
        self
    }
}

macro_rules! const_for_each {
    ($safe_const_slice:expr, $item:tt, $inner:expr) => {{
        let mut i = 0;
        while i < $safe_const_slice.len() {
            let $item = $safe_const_slice.get_or_panic(i);
            $inner;
            i += 1;
        }
    }};
}

pub(crate) use const_for_each;

pub(crate) struct ConstLengthsStack<const N: usize> {
    data: [Option<BranchMeta>; N],
    idx: usize,
}

impl<const N: usize> core::fmt::Debug for ConstLengthsStack<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<const N: usize> ConstLengthsStack<N> {
    pub const fn new() -> Self {
        Self {
            data: [None; N],
            idx: 0,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.idx == 0
    }

    #[must_use]
    pub const fn push_or_panic(mut self, meta: BranchMeta) -> Self {
        if self.idx >= N {
            panic!(concat!(
                "AsciiTrie Builder: Need more stack (max ",
                stringify!(N),
                ")"
            ));
        }
        self.data[self.idx] = Some(meta);
        self.idx += 1;
        self
    }

    pub const fn peek_or_panic(&self) -> BranchMeta {
        if self.idx == 0 {
            panic!("AsciiTrie Builder: Attempted to peek from an empty stack");
        }
        self.get_or_panic(0)
    }

    const fn get_or_panic(&self, index: usize) -> BranchMeta {
        if self.idx <= index {
            panic!("AsciiTrie Builder: Attempted to get too deep in a stack");
        }
        match self.data[self.idx - index - 1] {
            Some(x) => x,
            None => unreachable!(),
        }
    }

    pub const fn pop_many_or_panic(
        mut self,
        len: usize,
    ) -> (Self, ConstArrayBuilder<256, BranchMeta>) {
        debug_assert!(len <= 256);
        let mut result = ConstArrayBuilder::new_empty([BranchMeta::const_default(); 256], 256);
        let mut ix = 0;
        loop {
            if ix == len {
                break;
            }
            let i = self.idx - ix - 1;
            result = result.push_front_or_panic(match self.data[i] {
                Some(x) => x,
                None => panic!("Not enough items in the ConstLengthsStack"),
            });
            ix += 1;
        }
        self.idx -= len;
        (self, result)
    }

    fn as_slice(&self) -> &[Option<BranchMeta>] {
        &self.data[0..self.idx]
    }
}

impl<const N: usize> ConstArrayBuilder<N, BranchMeta> {
    pub const fn map_to_ascii_bytes(&self) -> ConstArrayBuilder<N, u8> {
        let mut result = ConstArrayBuilder::new_empty([0; N], N);
        let self_as_slice = self.as_const_slice();
        const_for_each!(self_as_slice, value, {
            result = result.const_push_front_or_panic(value.ascii);
        });
        result
    }
}
