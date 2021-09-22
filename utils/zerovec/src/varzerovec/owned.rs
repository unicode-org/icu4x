// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use core::fmt;
use core::marker::PhantomData;
use core::mem;
use core::ptr;
use core::slice;

#[derive(Clone)]
pub struct VarZeroVecOwned<T> {
    marker: PhantomData<[T]>,
    // safety invariant: must parse into a valid SliceComponents
    entire_slice: Vec<u8>,
}

impl<T: AsVarULE> VarZeroVecOwned<T> {
    /// Construct an empty VarZeroVecOwned
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
            entire_slice: Vec::new(),
        }
    }

    pub fn from_components(components: SliceComponents<T>) -> Self {
        Self {
            marker: PhantomData,
            entire_slice: components.entire_slice().into(),
        }
    }

    /// Construct a VarZeroVecOwned from a list of elements
    pub fn from_elements(elements: &[T]) -> Self {
        Self {
            marker: PhantomData,
            entire_slice: components::get_serializable_bytes(elements).expect(
                "Attempted to build VarZeroVec out of elements that \
                                     cumulatively are larger than a u32 in size",
            ),
        }
    }

    /// Try to allocate a buffer with enough capacity for `capacity`
    /// elements. Since `T` can take up an arbitrary size this will
    /// just allocate enough space for 4-byte Ts
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            marker: PhantomData,
            entire_slice: Vec::with_capacity(capacity * 8),
        }
    }

    /// Try to reserve space for `capacity`
    /// elements. Since `T` can take up an arbitrary size this will
    /// just allocate enough space for 4-byte Ts
    pub(crate) fn reserve(&mut self, capacity: usize) {
        self.entire_slice.reserve(capacity * 8)
    }

    #[inline]
    pub(crate) fn get_components<'a>(&'a self) -> SliceComponents<'a, T> {
        unsafe {
            // safety: VarZeroVecOwned is guaranteed to parse here
            SliceComponents::from_bytes_unchecked(&self.entire_slice)
        }
    }

    /// Get the number of elements in this vector
    pub fn len(&self) -> usize {
        self.get_components().len()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.get_components().is_empty()
    }

    /// Obtain an iterator over VarZeroVecOwned's elements
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = &'b T::VarULE> {
        self.get_components().iter()
    }

    /// Get one of VarZeroVecOwned's elements, returning None if the index is out of bounds
    pub fn get(&self, idx: usize) -> Option<&T::VarULE> {
        self.get_components().get(idx)
    }

    /// Get this [`VarZeroVecOwned`] as a borrowed [`VarZeroVec`]
    ///
    /// If you wish to repeatedly call methods on this [`VarZeroVecOwned`],
    /// it is more efficient to perform this conversion first
    pub fn as_varzerovec<'a>(&'a self) -> VarZeroVec<'a, T> {
        VarZeroVec(VarZeroVecInner::Borrowed(self.get_components()))
    }

    /// Empty the vector
    pub fn clear(&mut self) {
        self.entire_slice.clear()
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.get_components().to_vec()
    }

    /// Insert an element at index `idx`
    pub fn insert(&mut self, index: usize, element: &T) {
        let len = self.len();
        if index > len {
            panic!(
                "Called out-of-bounds insert() on VarZeroVec, index {} len {}",
                index, len
            );
        }

        if len == 0 {
            // If there is no data, just construct it with the existing procedure
            // for constructing an entire slice
            self.entire_slice = components::get_serializable_bytes(slice::from_ref(element))
                .expect(
                    "attempted to insert an element too large to be encoded\
                         in a VarZeroVec",
                );
            return;
        }

        // The format of the encoded data is:
        //  - four bytes of "len"
        //  - len*4 bytes for an array of indices
        //  - the actual data to which the indices point
        //
        // When inserting an element, space must be made in the `data`
        // segment for the element, and space must be made in the `indices` segment
        // for the new index. Note that making space in the indices segment
        // means that the data segment needs to be shifted by 4 bytes, on top
        // of any reshuffling that needs to be done to make space
        //
        // We do this in the following steps:
        // 1. Shift the data around
        //   1a. We move any data *after* the inserted element by the length of the element + 4
        //       to make space for the element and one extra index
        //   1b. We insert the element at four bytes after the spot where the data we just moved
        //       started, again making space for that extra index
        //   1c. We move the data from *before* the inserted element by 4 bytes to make space for the
        //       extra index
        // 2. Update the indices: Shift indices after the inserted element by one slot, incrementing them
        //       by the length of the inserted element
        // 3. Update the length

        let element = element.as_unaligned();
        let element_slice = element.as_byte_slice();
        // the amount the vector is growing by
        let shift = 4 + element_slice.len();
        self.entire_slice.reserve(shift);
        if self.entire_slice.len() + shift > u32::MAX as usize {
            // (on 32 bit platforms the `.reserve()` itself will panic)
            panic!(
                "Attempted to grow VarZeroVec to an encoded size that does not fit within a u32"
            );
        }
        unsafe {
            // Step 1: Shift the data around
            {
                let (indices, data) = self.entire_slice.split_at_mut(4 + 4 * len);

                let (_len, indices) = indices.split_at_mut(4);
                // safety: the indices at [4...4 + len * 4] form a slice of PlainOldULE<4>
                let indices = PlainOldULE::<4>::from_byte_slice_unchecked(indices);

                // Step 1a: Move the data after the inserted element by the length of
                // the element + 4 to make space for the element and one extra index

                // calculate the data insertion point as an index into `data`
                let insertion_point = if index != len {
                    // Assuming we're not pushing onto the end, we need to shift the tail-end elements
                    // by `shift` to make room for one extra index and `element_slice` data
                    let shift_start = u32::from_unaligned(&indices[index]) as usize;
                    let shift_start_p = data.as_mut_ptr().add(shift_start);
                    // shift elements from shift_start_p to the end of the slice by `shift` elements
                    ptr::copy(
                        shift_start_p,
                        shift_start_p.offset(shift as isize),
                        data.len() - shift_start,
                    );
                    shift_start + 4
                } else {
                    // If we're inserting to the end of the vector, we just need to insert at
                    // the length + 4 (space for one index)
                    data.len() + 4
                };

                let data_p = data.as_mut_ptr();

                // Step 1b: insert the new element
                ptr::copy(
                    element_slice.as_ptr(),
                    data_p.offset(insertion_point as isize),
                    element_slice.len(),
                );

                // Step 1c: shift the remaining elements to make room for the new index
                if insertion_point != 0 {
                    ptr::copy(data_p, data_p.offset(4), insertion_point - 4);
                }
            }
            self.entire_slice.set_len(self.entire_slice.len() + shift);
            // Step 2: Shift indices after the inserted element by one slot, incrementing them
            // by the length of the inserted element
            {
                let len = len + 1;
                let (indices, data) = self.entire_slice.split_at_mut(4 + 4 * len);
                let (len_ule, indices) = indices.split_at_mut(4);
                // safety: the indices at [4...4 + len * 4] form a slice of PlainOldULE<4>
                let indices = PlainOldULE::<4>::from_byte_slice_unchecked_mut(indices);
                // safety: the starting index is a single PlainOldULE<4>
                let len_ule = PlainOldULE::<4>::from_byte_slice_unchecked_mut(len_ule);

                let element_len = element_slice.len() as u32;
                if index + 1 == len {
                    // appending to the end is weird, because there's no end-index for us to copy;
                    // the end-index is simply the old length
                    indices[index] = (data.len() as u32 - element_len).into();
                } else {
                    let mut next = u32::from_unaligned(&indices[index]);
                    for idx in &mut indices[index + 1..] {
                        let incremented: u32 = next + element_len;
                        next = u32::from_unaligned(idx);
                        *idx = incremented.into();
                    }
                }

                // Step 3: update the length
                debug_assert!(u32::from_unaligned(&len_ule[0]) + 1 == len as u32);
                len_ule[0] = (len as u32).into()
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> T
    where
        T: Clone,
    {
        // TODO (#1080) make these faster
        let mut vec = self.to_vec();
        let ret = vec.remove(index);
        *self = Self::from_elements(&vec);
        ret
    }

    pub fn replace(&mut self, index: usize, value: T) -> T
    where
        T: Clone,
    {
        // TODO (#1080) make these faster
        let mut vec = self.to_vec();
        let ret = mem::replace(&mut vec[index], value);
        *self = Self::from_elements(&vec);
        ret
    }
}

impl<T> VarZeroVecOwned<T>
where
    T: AsVarULE,
    T::VarULE: Ord,
{
    /// Binary searches a sorted `VarZeroVecOwned<T>` for the given element. FoGeneralr more information, see
    /// the primitive function [`binary_search`].
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T::VarULE) -> Result<usize, usize> {
        self.get_components().binary_search(x)
    }
}

impl<T> PartialEq<&[T]> for VarZeroVecOwned<T>
where
    T: AsVarULE,
    T::VarULE: PartialEq,
{
    #[inline]
    fn eq(&self, other: &&[T]) -> bool {
        self.iter().eq(other.iter().map(|t| t.as_unaligned()))
    }
}

impl<T: AsVarULE> Index<usize> for VarZeroVecOwned<T> {
    type Output = T::VarULE;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Indexing VarZeroVec out of bounds")
    }
}

impl<T: AsVarULE> fmt::Debug for VarZeroVecOwned<T>
where
    T::VarULE: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod test {
    use super::VarZeroVecOwned;
    #[test]
    fn test_insert_integrity() {
        let mut items: Vec<String> = vec![
            "foo".into(),
            "bar".into(),
            "baz".into(),
            "abcdefghijklmn".into(),
            "five".into(),
        ];
        let mut zerovec = VarZeroVecOwned::from_elements(&items);
        zerovec.insert(1, &"foo3".into());
        items.insert(1, "foo3".into());
        assert_eq!(zerovec, &*items);

        items.insert(0, "1234567890".into());
        zerovec.insert(0, &"1234567890".into());
        assert_eq!(zerovec, &*items);

        // make sure inserting at the end works
        items.insert(items.len(), "qwertyuiop".into());
        zerovec.insert(zerovec.len(), &"qwertyuiop".into());
        assert_eq!(zerovec, &*items);

        items.insert(0, "asdfghjkl;".into());
        zerovec.insert(0, &"asdfghjkl;".into());
        assert_eq!(zerovec, &*items);
    }
}
