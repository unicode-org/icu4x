// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;
use core::mem;

use super::slice::FlexZeroSlice;

const USIZE_WIDTH: usize = mem::size_of::<usize>();

pub struct FlexZeroVecOwned(Vec<u8>);

impl FlexZeroVecOwned {
    pub fn new_empty() -> Self {
        Self(vec![1])
    }

    /// Obtain this `FlexZeroVecOwned` as a [`FlexZeroSlice`]
    pub fn as_slice(&self) -> &FlexZeroSlice {
        let slice: &[u8] = &*self.0;
        unsafe {
            // safety: the slice is known to come from a valid parsed FlexZeroSlice
            FlexZeroSlice::from_byte_slice_unchecked(slice)
        }
    }

    /// Mutably obtain this `FlexZeroVecOwned` as a [`FlexZeroSlice`]
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

    pub fn remove(&mut self, item: usize) {
        todo!("If the item being removed has max width, need to check if we need to scale down");
    }

    fn scale_down(&mut self, new_width: usize) {
        todo!("Similar to scale_up, but walk forward instead of backward");
    }
}

impl Deref for FlexZeroVecOwned {
    type Target = FlexZeroSlice;
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_push() {
        let mut fzv = FlexZeroVecOwned::new_empty();
        assert_eq!(fzv.get_width(), 1);
        assert_eq!(fzv.len(), 0);
        assert_eq!(fzv.is_empty(), true);
        assert_eq!(fzv.get(0), None);
        assert_eq!(fzv.first(), None);
        assert_eq!(fzv.last(), None);

        fzv.push(42);
        assert_eq!(fzv.get_width(), 1);
        assert_eq!(fzv.len(), 1);
        assert_eq!(fzv.is_empty(), false);
        assert_eq!(fzv.get(0), Some(42));
        assert_eq!(fzv.get(1), None);
        assert_eq!(fzv.first(), Some(42));
        assert_eq!(fzv.last(), Some(42));

        fzv.push(77);
        assert_eq!(fzv.get_width(), 1);
        assert_eq!(fzv.len(), 2);
        assert_eq!(fzv.is_empty(), false);
        assert_eq!(fzv.get(0), Some(42));
        assert_eq!(fzv.get(1), Some(77));
        assert_eq!(fzv.get(2), None);
        assert_eq!(fzv.first(), Some(42));
        assert_eq!(fzv.last(), Some(77));

        // Scale up
        fzv.push(300);
        assert_eq!(fzv.get_width(), 2);
        assert_eq!(fzv.len(), 3);
        assert_eq!(fzv.is_empty(), false);
        assert_eq!(fzv.get(0), Some(42));
        assert_eq!(fzv.get(1), Some(77));
        assert_eq!(fzv.get(2), Some(300));
        assert_eq!(fzv.get(3), None);
        assert_eq!(fzv.first(), Some(42));
        assert_eq!(fzv.last(), Some(300));
    }
}
