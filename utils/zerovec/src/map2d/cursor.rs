// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::AsULE;
use crate::{ZeroMap2d, ZeroSlice, ZeroVec};

use core::ops::Range;

use crate::map::ZeroMapKV;
use crate::map::ZeroVecLike;

use super::ZeroMap2dBorrowed;

pub struct ZeroMap2dCursorBorrowed<'l, 'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    keys0: &'l <<K0 as ZeroMapKV<'a>>::Container as ZeroVecLike<K0>>::BorrowedVariant,
    joiner: &'l ZeroSlice<u32>,
    keys1: &'l <<K1 as ZeroMapKV<'a>>::Container as ZeroVecLike<K1>>::BorrowedVariant,
    values: &'l <<V as ZeroMapKV<'a>>::Container as ZeroVecLike<V>>::BorrowedVariant,
    // Invariant: key0_index is in range
    key0_index: usize,
}

impl<'a, K0, K1, V> ZeroMap2dCursorBorrowed<'a, 'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// `key0_index` must be in range
    pub(crate) fn from_borrowed(borrowed: &ZeroMap2dBorrowed<'a, K0, K1, V>, key0_index: usize) -> Self {
        debug_assert!(key0_index < borrowed.joiner.len());
        ZeroMap2dCursorBorrowed {
            keys0: borrowed.keys0,
            joiner: borrowed.joiner,
            keys1: borrowed.keys1,
            values: borrowed.values,
            key0_index,
        }
    }
}

impl<'l, 'a, K0, K1, V> ZeroMap2dCursorBorrowed<'l, 'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    /// `key0_index` must be in range
    pub(crate) fn from_cow(cow: &'l ZeroMap2d<'a, K0, K1, V>, key0_index: usize) -> Self {
        debug_assert!(key0_index < cow.joiner.len());
        Self {
            keys0: cow.keys0.zvl_as_borrowed(),
            joiner: &cow.joiner,
            keys1: cow.keys1.zvl_as_borrowed(),
            values: cow.values.zvl_as_borrowed(),
            key0_index,
        }
    }

    /// Returns the key0 corresponding to the cursor position.
    pub fn key0(&self) -> &'l K0::GetType {
        #[allow(clippy::unwrap_used)] // safe by invariant on `self.key0_index`
        self.keys0.zvl_get(self.key0_index).unwrap()
    }

    /// Produce an ordered iterator over keys1 for a particular key0, if key0 exists
    pub fn iter1(
        &self,
    ) -> impl Iterator<
        Item = (
            &'l <K1 as ZeroMapKV<'a>>::GetType,
            &'l <V as ZeroMapKV<'a>>::GetType,
        ),
    > + '_ {
        let range = self.get_range();
        #[allow(clippy::unwrap_used)] // `self.get_range()` returns a valid range
        range.map(move |idx| {
            (
                self.keys1.zvl_get(idx).unwrap(),
                self.values.zvl_get(idx).unwrap(),
            )
        })
    }

    /// Given key0_index, returns the corresponding range of keys1, which will be valid
    pub(super) fn get_range(&self) -> Range<usize> {
        debug_assert!(self.key0_index < self.joiner.len());
        let start = if self.key0_index == 0 {
            0
        } else {
            #[allow(clippy::unwrap_used)] // protected by the debug_assert above
            self.joiner.get(self.key0_index - 1).unwrap()
        };
        #[allow(clippy::unwrap_used)] // protected by the debug_assert above
        let limit = self.joiner.get(self.key0_index).unwrap();
        // These two assertions are true based on the invariants of ZeroMap2d
        debug_assert!(start < limit);
        debug_assert!((limit as usize) < self.values.zvl_len());
        (start as usize)..(limit as usize)
    }

    pub(super) fn get_key0_index(&self) -> usize {
        self.key0_index
    }
}

impl<'l, 'a, K0, K1, V> ZeroMap2dCursorBorrowed<'l, 'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a> + Ord,
    V: ZeroMapKV<'a>,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    pub fn get1(&self, key1: &K1) -> Option<&'l V::GetType> {
        let key1_index = self.get_key1_index(key1)?;
        #[allow(clippy::unwrap_used)] // key1_index is valid
        Some(self.values.zvl_get(key1_index).unwrap())
    }

    // pub fn get1_by(&self, predicate: impl FnMut(&K1) -> Ordering) -> Option<&V::GetType> {
    //     let range = self.map.get_range_for_key0_index(self.key0_index);
    //     debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
    //     debug_assert!(range.end <= self.map.keys1.zvl_len());
    //     #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
    //     let index = range.start
    //         + self
    //             .map
    //             .keys1
    //             .zvl_binary_search_in_range(key1, range)
    //             .expect("in-bounds range")
    //             .ok()?;
    //     // This unwrap is protected by the invariant keys1.len() == values.len(),
    //     // the above debug_assert!, and the contract of zvl_binary_search_in_range.
    //     #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
    //     Some(self.map.values.zvl_get(index).unwrap())
    // }

    /// Given key0_index and key1, returns the index into the values array
    fn get_key1_index(&self, key1: &K1) -> Option<usize> {
        let range = self.get_range();
        debug_assert!(range.start < range.end); // '<' because every key0 should have a key1
        debug_assert!(range.end <= self.keys1.zvl_len());
        let start = range.start;
        #[allow(clippy::expect_used)] // protected by the debug_assert above
        let binary_search_result = self
            .keys1
            .zvl_binary_search_in_range(key1, range)
            .expect("in-bounds range");
        binary_search_result.ok().map(move |s| s + start)
    }
}

impl<'l, 'a, K0, K1, V> ZeroMap2dCursorBorrowed<'l, 'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized + Ord,
    V: ZeroMapKV<'a, Container = ZeroVec<'a, V>>,
    V: AsULE + Copy,
{
    pub fn get1_copied(&self, key1: &K1) -> Option<V> {
        let key1_index = self.get_key1_index(key1)?;
        self.values.get(key1_index)
    }
}
