// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::*;
use core::fmt;
use core::marker::PhantomData;
use core::ops::Range;
use core::ptr;
use core::slice;

#[derive(Clone)]
pub struct VarZeroVecOwned<T: ?Sized> {
    marker: PhantomData<Box<T>>,
    // safety invariant: must parse into a valid SliceComponents
    entire_slice: Vec<u8>,
}

// The effect of a shift on the indices in the varzerovec.
#[derive(PartialEq)]
enum ShiftType {
    Insert,
    Replace,
    Remove,
}

impl<T: VarULE + ?Sized> VarZeroVecOwned<T> {
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
    pub fn from_elements<A: AsRef<T>>(elements: &[A]) -> Self {
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
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = &'b T> {
        self.get_components().iter()
    }

    /// Get one of VarZeroVecOwned's elements, returning None if the index is out of bounds
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.get_components().get(idx)
    }

    /// Get the position of a specific element in the data segment.
    ///
    /// If `idx == self.len()`, it will return the size of the data segment (where a new element would go).
    ///
    /// ## Safety
    /// `idx <= self.len()` and `self.entire_slice()` is well-formed.
    unsafe fn element_position_unchecked(&self, idx: usize) -> usize {
        let len = self.len();
        let out = if idx == len {
            self.entire_slice.len() - 4 - (4 * len)
        } else {
            u32::from_unaligned(self.index_data(idx)) as usize
        };
        debug_assert!(out + 4 + len * 4 <= self.entire_slice.len());
        out
    }

    /// Get the range of a specific element in the data segment.
    ///
    /// ## Safety
    /// `idx < self.len()` and `self.entire_slice()` is well-formed.
    unsafe fn element_range_unchecked(&self, idx: usize) -> core::ops::Range<usize> {
        let start = self.element_position_unchecked(idx);
        let end = self.element_position_unchecked(idx + 1);
        debug_assert!(start <= end, "{} > {}", start, end);
        start..end
    }

    /// Set the number of elements in the list without any checks.
    ///
    /// ## Safety
    /// No safe functions may be called until `self.entire_slice()` is well-formed.
    unsafe fn set_len(&mut self, len: u32) {
        PlainOldULE::<4>::from_byte_slice_unchecked_mut(&mut self.entire_slice[..4])[0] =
            len.into();
    }

    fn index_range(index: usize) -> Range<usize> {
        let pos = 4 + 4 * index;
        pos..pos + 4
    }

    /// Return the slice representing the given `index`.
    ///
    /// ## Safety
    /// The index must be valid, and self.entire_slice() must be well-formed
    unsafe fn index_data(&self, index: usize) -> &PlainOldULE<4> {
        &PlainOldULE::<4>::from_byte_slice_unchecked(&self.entire_slice[Self::index_range(index)])
            [0]
    }

    /// Return the mutable slice representing the given `index`.
    ///
    /// ## Safety
    /// The index must be valid. self.entire_slice() must have allocated space
    /// for this index, but need not have its length appropriately set.
    unsafe fn index_data_mut(&mut self, index: usize) -> &mut PlainOldULE<4> {
        let ptr = self.entire_slice.as_mut_ptr();
        let range = Self::index_range(index);

        // Doing this instead of just `get_unchecked_mut()` because it's unclear
        // if `get_unchecked_mut()` can be called out of bounds on a slice even
        // if we know the buffer is larger.
        let data = slice::from_raw_parts_mut(ptr.add(range.start), 4);

        &mut PlainOldULE::<4>::from_byte_slice_unchecked_mut(data)[0]
    }

    /// Shift the indices starting with and after `starting_index` by the provided `amount`.
    ///
    /// ## Safety
    /// Adding `amount` to each index after `starting_index` must not result in the slice from becoming malformed.
    /// The length of the slice must be correctly set.
    unsafe fn shift_indices(&mut self, starting_index: usize, amount: i32) {
        let len = self.len();
        let indices =
            PlainOldULE::<4>::from_byte_slice_unchecked_mut(&mut self.entire_slice[4..4 + 4 * len]);
        for idx in &mut indices[starting_index..] {
            *idx = (u32::from_unaligned(idx).wrapping_add(amount as u32)).into();
        }
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

    pub fn to_vec(&self) -> Vec<Box<T>> {
        self.get_components().to_vec()
    }

    #[inline]
    pub fn entire_slice(&self) -> &[u8] {
        &self.entire_slice
    }

    /// Invalidate and resize the data at an index, optionally inserting or removing the index.
    /// Also updates affected indices and the length.
    /// Returns a slice to the new element data - it doesn't contain uninitialized data but its value is indeterminate.
    ///
    /// ## Safety
    /// - `index` must be a valid index, or, if `shift_type == ShiftType::Insert`, `index == self.len()` is allowed.
    /// - `new_size` musn't result in the data segment growing larger than `u32::MAX`.
    unsafe fn shift(&mut self, index: usize, new_size: u32, shift_type: ShiftType) -> &mut [u8] {
        // The format of the encoded data is:
        //  - four bytes of "len"
        //  - len*4 bytes for an array of indices
        //  - the actual data to which the indices point
        //
        // When inserting or removing an element, the size of the indices segment must be changed,
        // so the data before the target element must be shifted by 4 bytes in addition to the
        // shifting needed for the new element size.
        let len = self.len();
        let slice_len = self.entire_slice.len();

        let prev_element = match shift_type {
            ShiftType::Insert => {
                let pos = self.element_position_unchecked(index);
                // In the case of an insert, there's no previous element,
                // so it's an empty range at the new position.
                pos..pos
            }
            _ => self.element_range_unchecked(index),
        };

        // How much shifting must be done in bytes due to removal/insertion of an index.
        let index_shift: i64 = match shift_type {
            ShiftType::Insert => 4,
            ShiftType::Replace => 0,
            ShiftType::Remove => -4,
        };
        // The total shift in byte size of the owned slice.
        let shift: i64 =
            new_size as i64 - (prev_element.end - prev_element.start) as i64 + index_shift;
        let new_slice_len = slice_len.wrapping_add(shift as usize);
        if shift > 0 {
            if new_slice_len > u32::MAX as usize {
                panic!(
                    "Attempted to grow VarZeroVec to an encoded size that does not fit within a u32"
                );
            }
            self.entire_slice.reserve(shift as usize);
        }

        // Now that we've ensured there's enough space, we can shift the data around.
        {
            // Note: There are no references introduced between pointer creation and pointer use, and all
            //       raw pointers are derived from a single &mut. This preserves pointer provenance.
            let slice_range = self.entire_slice.as_mut_ptr_range();
            let data_start = slice_range.start.add(4 + len * 4);
            let prev_element_p =
                data_start.add(prev_element.start)..data_start.add(prev_element.end);

            // The memory range of the affected index.
            // When inserting: where the new index goes.
            // When removing:  where the index being removed is.
            // When replacing: unused.
            let index_range = {
                let index_start = slice_range.start.add(4 + 4 * index);
                index_start..index_start.add(4)
            };

            unsafe fn shift_bytes(block: Range<*const u8>, to: *mut u8) {
                debug_assert!(block.end >= block.start);
                ptr::copy(block.start, to, block.end.offset_from(block.start) as usize);
            }

            if shift_type == ShiftType::Remove {
                // Move the data before the element back by 4 to remove the index.
                shift_bytes(index_range.end..prev_element_p.start, index_range.start);
            }

            // Shift data after the element to its new position.
            shift_bytes(
                prev_element_p.end..slice_range.end,
                prev_element_p
                    .start
                    .offset((new_size as i64 + index_shift) as isize),
            );

            let first_affected_index = match shift_type {
                ShiftType::Insert => {
                    // Move data before the element forward by 4 to make space for a new index.
                    shift_bytes(index_range.start..prev_element_p.start, index_range.end);

                    *self.index_data_mut(index) = (prev_element.start as u32).into();
                    self.set_len((len + 1) as u32);
                    index + 1
                }
                ShiftType::Remove => {
                    self.set_len((len - 1) as u32);
                    index
                }
                ShiftType::Replace => index + 1,
            };
            // No raw pointer use should occur after this point (because of self.index_data and self.set_len).

            // Set the new slice length. This must be done after shifting data around to avoid uninitialized data.
            self.entire_slice.set_len(new_slice_len);

            // Shift the affected indices.
            self.shift_indices(first_affected_index, (shift - index_shift) as i32);
        };

        debug_assert!(self.verify_integrity());

        // Return a mut slice to the new element data.
        let element_pos = 4 + self.len() * 4 + self.element_position_unchecked(index);
        &mut self.entire_slice[element_pos..element_pos + new_size as usize]
    }

    /// Checks the internal invariants of the vec to ensure safe code will not cause UB.
    /// Returns whether integrity was verified.
    ///
    /// Note: an index is valid if it doesn't point to data past the end of the slice and is
    /// less than or equal to all future indices. The length of the index segment is not part of each index.
    fn verify_integrity(&self) -> bool {
        if self.is_empty() && !self.entire_slice.is_empty() {
            return false;
        }
        let slice_len = self.entire_slice.len();
        match slice_len {
            0 => return true,
            1..=3 => return false,
            _ => (),
        }
        let len = unsafe {
            u32::from_unaligned(
                &PlainOldULE::<4>::from_byte_slice_unchecked(&self.entire_slice[..4])[0],
            )
        };
        if len == 0 {
            // An empty vec must have an empty slice: there is only a single valid byte representation.
            return false;
        }
        if slice_len <= 4 + len as usize * 4 {
            // Not enough room for the indices.
            return false;
        }
        let data_len = self.entire_slice.len() - 4 - len as usize * 4;
        if data_len > u32::MAX as usize {
            // The data segment is too long.
            return false;
        }
        let data_len = data_len as u32;

        // Test index validity.
        let indices = unsafe {
            PlainOldULE::<4>::from_byte_slice_unchecked(&self.entire_slice[4..4 + len as usize * 4])
        };
        for idx in indices {
            if u32::from_unaligned(idx) > data_len {
                // Indices must not point past the data segment.
                return false;
            }
        }
        for window in indices.windows(2) {
            if u32::from_unaligned(&window[0]) > u32::from_unaligned(&window[1]) {
                // Indices must be in non-decreasing order.
                return false;
            }
        }
        true
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

        let value = element.as_byte_slice();

        if len == 0 {
            // 4 bytes for length, 4 bytes for the index, remaining for element
            self.reserve(8 + value.len());
            let len_u32 = 1u32;
            let index_u32 = 0u32;
            self.entire_slice.extend(&len_u32.as_unaligned().0);
            self.entire_slice.extend(&index_u32.as_unaligned().0);
            self.entire_slice.extend(value);
            return;
        }

        assert!(value.len() < u32::MAX as usize);
        unsafe {
            self.shift(index, value.len() as u32, ShiftType::Insert)
                .copy_from_slice(value)
        }
    }

    pub fn remove(&mut self, index: usize) {
        let len = self.len();
        if index >= len {
            panic!(
                "Called out-of-bounds remove() on VarZeroVec, index {} len {}",
                index, len
            );
        }
        if len == 1 {
            // This is removing the last element. Set the slice to empty to ensure all empty vecs have empty data slices.
            self.entire_slice.clear();
            return;
        }
        unsafe {
            self.shift(index, 0, ShiftType::Remove);
        }
    }

    pub fn replace(&mut self, index: usize, value: &T) {
        let len = self.len();
        if index >= len {
            panic!(
                "Called out-of-bounds replace() on VarZeroVec, index {} len {}",
                index, len
            );
        }

        let value = value.as_byte_slice();
        assert!(value.len() < u32::MAX as usize);
        unsafe {
            self.shift(index, value.len() as u32, ShiftType::Replace)
                .copy_from_slice(value);
        }
    }
}

impl<T> VarZeroVecOwned<T>
where
    T: VarULE,
    T: ?Sized,
    T: Ord,
{
    /// Binary searches a sorted `VarZeroVecOwned<T>` for the given element. FoGeneralr more information, see
    /// the primitive function [`binary_search`].
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize> {
        self.get_components().binary_search(x)
    }
}

impl<T: VarULE + ?Sized> Index<usize> for VarZeroVecOwned<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Indexing VarZeroVec out of bounds")
    }
}

impl<T: VarULE + ?Sized> fmt::Debug for VarZeroVecOwned<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: VarULE + ?Sized> Default for VarZeroVecOwned<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> PartialEq<&'_ [A]> for VarZeroVecOwned<T>
where
    T: VarULE + ?Sized,
    T: PartialEq,
    A: AsRef<T>,
{
    #[inline]
    fn eq(&self, other: &&[A]) -> bool {
        self.iter().eq(other.iter().map(|t| t.as_ref()))
    }
}

#[cfg(test)]
mod test {
    use super::VarZeroVecOwned;
    #[test]
    fn test_insert_integrity() {
        let mut items: Vec<String> = Vec::new();
        let mut zerovec = VarZeroVecOwned::<str>::new();

        // Insert into an empty vec.
        items.insert(0, "1234567890".into());
        zerovec.insert(0, "1234567890");
        assert_eq!(zerovec, &*items);

        zerovec.insert(1, "foo3");
        items.insert(1, "foo3".into());
        assert_eq!(zerovec, &*items);

        // Insert at the end.
        items.insert(items.len(), "qwertyuiop".into());
        zerovec.insert(zerovec.len(), "qwertyuiop");
        assert_eq!(zerovec, &*items);

        items.insert(0, "asdfghjkl;".into());
        zerovec.insert(0, "asdfghjkl;");
        assert_eq!(zerovec, &*items);

        items.insert(2, "".into());
        zerovec.insert(2, "");
        assert_eq!(zerovec, &*items);
    }

    #[test]
    fn test_small_insert_integrity() {
        // Tests that insert() works even when there
        // is not enough space for the new index in entire_slice.len()
        let mut items: Vec<String> = Vec::new();
        let mut zerovec = VarZeroVecOwned::<str>::new();

        // Insert into an empty vec.
        items.insert(0, "abc".into());
        zerovec.insert(0, "abc");
        assert_eq!(zerovec, &*items);

        zerovec.insert(1, "def");
        items.insert(1, "def".into());
        assert_eq!(zerovec, &*items);
    }

    #[test]
    #[should_panic]
    fn test_insert_past_end() {
        VarZeroVecOwned::<str>::new().insert(1, "");
    }

    #[test]
    fn test_remove_integrity() {
        let mut items: Vec<String> = vec![
            "apples".into(),
            "bananas".into(),
            "eeples".into(),
            "".into(),
            "baneenees".into(),
            "five".into(),
            "".into(),
        ];
        let mut zerovec = VarZeroVecOwned::<str>::from_elements(&items);

        for index in [0, 2, 4, 0, 1, 1, 0] {
            items.remove(index);
            zerovec.remove(index);
            assert_eq!(zerovec, &*items, "index {}, len {}", index, items.len());
        }
    }

    #[test]
    fn test_removing_last_element_clears() {
        let mut zerovec = VarZeroVecOwned::<str>::from_elements(&["buy some apples".to_string()]);
        assert!(!zerovec.get_components().entire_slice().is_empty());
        zerovec.remove(0);
        assert!(zerovec.get_components().entire_slice().is_empty());
    }

    #[test]
    #[should_panic]
    fn test_remove_past_end() {
        VarZeroVecOwned::<str>::new().remove(0);
    }

    #[test]
    fn test_replace_integrity() {
        let mut items: Vec<String> = vec![
            "apples".into(),
            "bananas".into(),
            "eeples".into(),
            "".into(),
            "baneenees".into(),
            "five".into(),
            "".into(),
        ];
        let mut zerovec = VarZeroVecOwned::<str>::from_elements(&items);

        // Replace with an element of the same size (and the first element)
        items[0] = "blablah".into();
        zerovec.replace(0, "blablah");
        assert_eq!(zerovec, &*items);

        // Replace with a smaller element
        items[1] = "twily".into();
        zerovec.replace(1, "twily");
        assert_eq!(zerovec, &*items);

        // Replace an empty element
        items[3] = "aoeuidhtns".into();
        zerovec.replace(3, "aoeuidhtns");
        assert_eq!(zerovec, &*items);

        // Replace the last element
        items[6] = "0123456789".into();
        zerovec.replace(6, "0123456789");
        assert_eq!(zerovec, &*items);

        // Replace with an empty element
        items[2] = "".into();
        zerovec.replace(2, "");
        assert_eq!(zerovec, &*items);
    }

    #[test]
    #[should_panic]
    fn test_replace_past_end() {
        VarZeroVecOwned::<str>::new().replace(0, "");
    }
}
