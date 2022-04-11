// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::mem;

const USIZE_WIDTH: usize = mem::size_of::<usize>();

/// A zero-copy "slice" that efficiently represents `[usize]`.
#[repr(packed)]
#[derive(Eq, PartialEq)]
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
        self.width as usize
    }

    #[inline]
    pub unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        &*(&bytes[..bytes.len() - 1] as *const [u8] as *const Self)
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
