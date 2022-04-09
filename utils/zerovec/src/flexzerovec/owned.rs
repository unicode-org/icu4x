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

    fn scale_up(&mut self, new_width: usize) {
        let count = self.len();
        let old_width = self.get_width();
        let old_byte_len = self.0.len();
        let new_byte_len = ((old_byte_len - 1) / old_width) * new_width + 1;
        debug_assert!(new_byte_len > old_byte_len);
        self.0.resize(new_byte_len, 0);
        // Copy elements starting from the end into the new empty section of the vector
        for i in (0..count).rev() {
            // Safety: The vector previously held `count` items at `old_width` and now has the
            // capacity for `count` items at `new_width`.
            unsafe {
                core::ptr::copy(
                    self.0.as_ptr().add(1).add(old_width * i),
                    self.0.as_mut_ptr().add(1).add(new_width * i),
                    old_width
                );
            }
        }
        // Safety: The vector always has at least 1 element.
        unsafe { *self.0.as_mut_slice().get_unchecked_mut(0) = new_width as u8 };
    }
}

impl Deref for FlexZeroVecOwned {
    type Target = FlexZeroSlice;
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
