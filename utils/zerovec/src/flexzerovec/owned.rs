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

    pub fn push(&mut self, item: usize) {
        let mut w = self.get_width();
        let bytes = item.to_le_bytes();
        let required_width = USIZE_WIDTH - bytes.iter().rev().take_while(|b| **b == 0).count();
        if required_width > w {
            self.scale_up(required_width);
            w = required_width;
        }
        // Safety: w <= required_width <= USIZE_WIDTH
        let bytes_to_append = unsafe { bytes.get_unchecked(0..w) };
        self.0.extend_from_slice(bytes_to_append);
    }

    pub fn remove(&mut self, item: usize) {
        todo!("If the item being removed has max width, need to check if we need to scale down");
    }

    fn scale_up(&mut self, new_width: usize) {
        let count = self.len();
        let old_width = self.get_width();
        let old_byte_len = self.0.len();
        let new_byte_len = ((old_byte_len - 1) / old_width) * new_width + 1;
        debug_assert!(new_byte_len > old_byte_len);
        self.0.resize(new_byte_len, 0);
        // Copy elements starting from the end into the new empty section of the vector.
        // Note: We could copy fully in place, but we need to set 0 bytes for the high bytes,
        // so we stage the new value on the stack.
        for i in (0..count).rev() {
            // Safety: The item at index i has not been changed since we are walking backwards.
            let usize_bytes = unsafe { self.get_unchecked(i).to_le_bytes() };
            // Safety: The vector has capacity for `new_width` items at the new index, which is
            // later in the array than the bytes that we read above.
            unsafe {
                core::ptr::copy_nonoverlapping(
                    usize_bytes.as_ptr(),
                    self.0.as_mut_ptr().add(1).add(new_width * i),
                    new_width
                );
            }
        }
        // Safety: The vector always has at least 1 element.
        unsafe { *self.0.as_mut_slice().get_unchecked_mut(0) = new_width as u8 };
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
