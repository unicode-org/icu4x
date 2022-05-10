// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec;
use alloc::vec::Vec;
use core::iter::FromIterator;
use core::ops::Deref;
use core::fmt;

use super::slice::FlexZeroSlice;

// Safety invariant: the inner bytes must deref to a valid `FlexZeroSlice`
#[derive(Clone, PartialEq, Eq)]
pub struct FlexZeroVecOwned(Vec<u8>);

impl FlexZeroVecOwned {
    /// Creates a new [`FlexZeroVecOwned`] with zero elements.
    pub fn new_empty() -> Self {
        Self(vec![1])
    }

    /// Creates a [`FlexZeroVecOwned`] from a [`FlexZeroSlice`].
    pub fn from_slice(other: &FlexZeroSlice) -> FlexZeroVecOwned {
        // safety: the bytes originate from a valid FlexZeroSlice
        Self(other.as_bytes().iter().copied().collect())
    }

    /// Obtains this [`FlexZeroVecOwned`] as a [`FlexZeroSlice`].
    pub fn as_slice(&self) -> &FlexZeroSlice {
        let slice: &[u8] = &*self.0;
        unsafe {
            // safety: the slice is known to come from a valid parsed FlexZeroSlice
            FlexZeroSlice::from_byte_slice_unchecked(slice)
        }
    }

    /// Mutably obtains this `FlexZeroVecOwned` as a [`FlexZeroSlice`].
    pub(crate) fn as_mut_slice(&mut self) -> &mut FlexZeroSlice {
        let slice: &mut [u8] = &mut *self.0;
        unsafe {
            // safety: the slice is known to come from a valid parsed FlexZeroSlice
            FlexZeroSlice::from_byte_slice_mut_unchecked(slice)
        }
    }

    pub fn push(&mut self, item: usize) {
        let insert_info = self.get_insert_info(item);
        self.0.resize(insert_info.new_data_len + 1, 0);
        let insert_index = insert_info.new_count - 1;
        self.as_mut_slice().insert_impl(insert_info, insert_index);
    }

    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert(&mut self, index: usize, item: usize) {
        if index > self.len() {
            panic!("index {} out of range {}", index, self.len());
        }
        let insert_info = self.get_insert_info(item);
        self.0.resize(insert_info.new_data_len + 1, 0);
        self.as_mut_slice().insert_impl(insert_info, index);
    }

    /// Inserts an element into a sorted vector at a position that keeps the vector sorted.
    ///
    /// ```
    /// use zerovec::vecs::FlexZeroVecOwned;
    ///
    /// let mut fzv = FlexZeroVecOwned::new_empty();
    /// fzv.insert_sorted(10);
    /// fzv.insert_sorted(5);
    /// fzv.insert_sorted(8);
    ///
    /// assert!(Iterator::eq(
    ///     fzv.iter(),
    ///     [5, 8, 10].iter().copied()
    /// ));
    /// ```
    pub fn insert_sorted(&mut self, item: usize) {
        let index = match self.binary_search(item) {
            Ok(i) => i,
            Err(i) => i,
        };
        let insert_info = self.get_insert_info(item);
        self.0.resize(insert_info.new_data_len + 1, 0);
        self.as_mut_slice().insert_impl(insert_info, index);
    }

    /// # Panics
    ///
    /// Panics if `index >= len`.
    pub fn remove(&mut self, index: usize) {
        if index >= self.len() {
            panic!("index {} out of range {}", index, self.len());
        }
        let remove_info = self.get_remove_info(index);
        let new_bytes_len = remove_info.new_data_len + 1;
        self.as_mut_slice().remove_impl(remove_info);
        self.0.truncate(new_bytes_len);
    }

    /// # Panics
    ///
    /// Panics if `self.is_empty()`
    pub fn pop_sorted(&mut self) -> usize {
        if self.is_empty() {
            panic!("cannot pop from an empty vector");
        }
        let remove_info = self.get_sorted_pop_info();
        // Safety: `remove_index` is a valid index
        let item = unsafe { self.get_unchecked(remove_info.remove_index) };
        let new_bytes_len = remove_info.new_data_len + 1;
        self.as_mut_slice().remove_impl(remove_info);
        self.0.truncate(new_bytes_len);
        item
    }
}

impl Deref for FlexZeroVecOwned {
    type Target = FlexZeroSlice;
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl fmt::Debug for FlexZeroVecOwned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_vec())
    }
}

impl From<&FlexZeroSlice> for FlexZeroVecOwned {
    fn from(other: &FlexZeroSlice) -> Self {
        Self::from_slice(other)
    }
}

impl FromIterator<usize> for FlexZeroVecOwned {
    /// Creates a [`FlexZeroVecOwned`] from an iterator of `usize`.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let mut result = FlexZeroVecOwned::new_empty();
        for item in iter {
            result.push(item);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_contents(fzv: &FlexZeroSlice, expected: &[usize]) {
        assert_eq!(
            fzv.len(),
            expected.len(),
            "len: {:?} != {:?}",
            fzv,
            expected
        );
        assert_eq!(
            fzv.is_empty(),
            expected.is_empty(),
            "is_empty: {:?} != {:?}",
            fzv,
            expected
        );
        assert_eq!(
            fzv.first(),
            expected.first().copied(),
            "first: {:?} != {:?}",
            fzv,
            expected
        );
        assert_eq!(
            fzv.last(),
            expected.last().copied(),
            "last:  {:?} != {:?}",
            fzv,
            expected
        );
        for i in 0..(expected.len() + 1) {
            assert_eq!(
                fzv.get(i),
                expected.get(i).copied(),
                "@{}: {:?} != {:?}",
                i,
                fzv,
                expected
            );
        }
    }

    #[test]
    fn test_basic() {
        let mut fzv = FlexZeroVecOwned::new_empty();
        assert_eq!(fzv.get_width(), 1);
        check_contents(&fzv, &[]);

        fzv.push(42);
        assert_eq!(fzv.get_width(), 1);
        check_contents(&fzv, &[42]);

        fzv.push(77);
        assert_eq!(fzv.get_width(), 1);
        check_contents(&fzv, &[42, 77]);

        // Scale up
        fzv.push(300);
        assert_eq!(fzv.get_width(), 2);
        check_contents(&fzv, &[42, 77, 300]);

        // Does not need to be sorted
        fzv.insert(1, 325);
        assert_eq!(fzv.get_width(), 2);
        check_contents(&fzv, &[42, 325, 77, 300]);

        fzv.remove(3);
        assert_eq!(fzv.get_width(), 2);
        check_contents(&fzv, &[42, 325, 77]);

        // Scale down
        fzv.remove(1);
        assert_eq!(fzv.get_width(), 1);
        check_contents(&fzv, &[42, 77]);
    }

    #[test]
    fn test_build_sorted() {
        let nums: &[usize] = &[0, 50, 0, 77, 831, 29, 89182, 931, 0, 77, 712381];
        let mut fzv = FlexZeroVecOwned::new_empty();

        for num in nums {
            fzv.insert_sorted(*num);
        }
        assert_eq!(fzv.get_width(), 3);
        check_contents(&fzv, &[0, 0, 0, 29, 50, 77, 77, 831, 931, 89182, 712381]);

        for num in nums {
            let index = fzv.binary_search(*num).unwrap();
            fzv.remove(index);
        }
        assert_eq!(fzv.get_width(), 1);
        check_contents(&fzv, &[]);
    }
}
