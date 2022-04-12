// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::mem;

const USIZE_WIDTH: usize = mem::size_of::<usize>();

/// A zero-copy "slice" that efficiently represents `[usize]`.
#[repr(packed)]
#[derive(Debug, Eq, PartialEq)]
pub struct FlexZeroSlice {
    // Invariant: width is <= USIZE_WIDTH (which is target_pointer_width)
    width: u8,
    // Invariant: data.len() % width == 0
    data: [u8],
}

#[inline]
fn chunk_to_usize(chunk: &[u8], width: usize) -> usize {
    debug_assert_eq!(chunk.len(), width);
    let mut bytes = [0; USIZE_WIDTH];
    bytes[0..width].copy_from_slice(chunk);
    usize::from_le_bytes(bytes)
}

impl FlexZeroSlice {
    /// Construct a new empty FlexZeroSlice
    #[inline]
    pub fn new_empty() -> &'static Self {
        let arr: &[u8] = &[1u8];
        unsafe { mem::transmute(arr) }
    }

    #[inline]
    pub fn get_width(&self) -> usize {
        usize::from(self.width)
    }

    #[inline]
    pub unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        &*(&bytes[..bytes.len() - 1] as *const [u8] as *const Self)
    }

    #[inline]
    pub(crate) unsafe fn from_byte_slice_mut_unchecked(bytes: &mut [u8]) -> &mut Self {
        let len = bytes.len();
        &mut *(&mut bytes[..len - 1] as *mut [u8] as *mut Self)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len() / usize::from(self.width)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<usize> {
        let w = self.get_width();
        self.data
            .get(index * w..index * w + w)
            .map(|chunk| chunk_to_usize(chunk, w))
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> usize {
        let w = self.get_width();
        let mut bytes = [0; USIZE_WIDTH];
        core::ptr::copy_nonoverlapping(self.data.as_ptr().add(index * w), bytes.as_mut_ptr(), w);
        usize::from_le_bytes(bytes)
    }

    #[inline]
    pub fn first(&self) -> Option<usize> {
        let w = self.get_width();
        self.data.get(0..w).map(|chunk| chunk_to_usize(chunk, w))
    }

    #[inline]
    pub fn last(&self) -> Option<usize> {
        let l = self.data.len();
        if l == 0 {
            None
        } else {
            let w = self.get_width();
            self.data
                .get(l - w..l)
                .map(|chunk| chunk_to_usize(chunk, w))
        }
    }

    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = usize> + '_ {
        let w = self.get_width();
        self.data
            .chunks_exact(w)
            .map(move |chunk| chunk_to_usize(chunk, w))
    }

    /// Binary searches a sorted `FlexZeroSlice` for the given `usize` value.
    pub fn binary_search(
        &self,
        needle: usize
    ) -> Result<usize, usize> {
        // See comments in components.rs regarding the following code.

        let zero_index = self.data.as_ptr() as *const _ as usize;
        debug_assert!(self.len() <= self.data.len());
        // Safety: self.len() <= self.data.len()
        let scaled_slice = unsafe { self.data.get_unchecked(0..self.len()) };

        scaled_slice.binary_search_by(|probe: &_| {
            // Note: `scaled_slice` is a slice of u8
            let index = probe as *const _ as usize - zero_index;
            // Safety: we know this is in bounds
            let actual_probe = unsafe { self.get_unchecked(index) };
            <usize as Ord>::cmp(&actual_probe, &needle)
        })
    }
}

#[inline]
fn get_item_width(item_bytes: &[u8; USIZE_WIDTH]) -> usize {
    USIZE_WIDTH - item_bytes.iter().rev().take_while(|b| **b == 0).count()
}

pub(crate) struct InsertInfo {
    pub item_bytes: [u8; USIZE_WIDTH],
    pub new_width: usize,
    pub new_count: usize,
    pub new_data_len: usize,
}

impl FlexZeroSlice {
    pub(crate) fn get_insert_info(&self, new_item: usize) -> InsertInfo {
        let item_bytes = new_item.to_le_bytes();
        let item_width = get_item_width(&item_bytes);
        let old_width = self.get_width();
        let new_width = core::cmp::max(old_width, item_width);
        let new_count = 1 + (self.data.len() / old_width);
        let new_data_len = new_count * new_width;
        InsertInfo {
            item_bytes,
            new_width,
            new_count,
            new_data_len,
        }
    }

    /// This function should be called on a slice with a data array `new_data_len` long
    /// which previously held `new_count - 1` elements.
    /// 
    /// After calling this function, all bytes in the slice will have been written.
    pub(crate) fn insert_impl(&mut self, insert_info: InsertInfo, insert_index: usize) {
        let InsertInfo { item_bytes, new_width, new_count, new_data_len } = insert_info;
        debug_assert!(new_width <= USIZE_WIDTH);
        debug_assert!(new_width >= self.get_width());
        debug_assert!(insert_index < new_count);
        debug_assert_eq!(new_data_len, new_count * new_width);
        debug_assert_eq!(new_data_len, self.data.len());
        // Copy elements starting from the end into the new empty section of the vector.
        // Note: We could copy fully in place, but we need to set 0 bytes for the high bytes,
        // so we stage the new value on the stack.
        for i in (0..new_count).rev() {
            let bytes_to_write = if i == insert_index {
                item_bytes
            } else {
                let j = if i > insert_index { i - 1 } else { i };
                debug_assert!(j < new_count - 1);
                // Safety: j is in range (assertion on previous line), and it has not been
                // overwritten yet since we are walking backwards.
                unsafe { self.get_unchecked(j).to_le_bytes() }
            };
            // Safety: The vector has capacity for `new_width` items at the new index, which is
            // later in the array than the bytes that we read above.
            unsafe {
                core::ptr::copy_nonoverlapping(
                    bytes_to_write.as_ptr(),
                    self.data.as_mut_ptr().add(new_width * i),
                    new_width
                );
            }
        }
        self.width = new_width as u8;
    }
}

pub(crate) struct RemoveInfo {
    pub remove_index: usize,
    pub new_width: usize,
    pub new_count: usize,
    pub new_data_len: usize,
}

impl FlexZeroSlice {
    pub(crate) fn get_remove_info(&self, remove_index: usize) -> RemoveInfo {
        debug_assert!(remove_index < self.len());
        // Safety: remove_index is in range (assertion on previous line)
        let item_bytes = unsafe { self.get_unchecked(remove_index).to_le_bytes() };
        let item_width = get_item_width(&item_bytes);
        let old_width = self.get_width();
        let old_count = self.data.len() / old_width;
        let new_width = if item_width < old_width {
            old_width
        } else {
            debug_assert_eq!(old_width, item_width);
            // We might be removing the widest element. If so, we need to scale down.
            let mut largest_width = 1;
            for i in 0..old_count {
                if i == remove_index {
                    continue;
                }
                // Safety: i is in range (between 0 and old_count)
                let curr_bytes = unsafe { self.get_unchecked(i).to_le_bytes() };
                let curr_width = get_item_width(&curr_bytes);
                largest_width = core::cmp::max(curr_width, largest_width);
            }
            largest_width
        };
        let new_count = old_count - 1;
        let new_data_len = new_count * new_width;
        RemoveInfo {
            remove_index,
            new_width,
            new_count,
            new_data_len,
        }
    }

    /// This function should be called on a valid slice.
    ///
    /// After calling this function, the slice data should be truncated to `new_data_len` bytes.
    pub(crate) fn remove_impl(&mut self, remove_info: RemoveInfo) {
        let RemoveInfo { remove_index, new_width, new_count, .. } = remove_info;
        debug_assert!(new_width <= self.get_width());
        debug_assert!(new_count < self.len());
        for i in (0..new_count) {
            let j = if i < remove_index {
                i
            } else {
                i + 1
            };
            // Safety: j is in range because j <= new_count < self.len()
            let bytes_to_write = unsafe { self.get_unchecked(j).to_le_bytes() };
            // Safety: The bytes are being copied to a section of the array that is not after
            // the section of the array that currently holds the bytes.
            unsafe {
                core::ptr::copy_nonoverlapping(
                    bytes_to_write.as_ptr(),
                    self.data.as_mut_ptr().add(new_width * i),
                    new_width
                );
            }
        }
        self.width = new_width as u8;
    }
}
