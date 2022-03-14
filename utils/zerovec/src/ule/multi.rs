// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::VarZeroSlice;
use core::mem;

/// This type is used by the custom derive to represent multiple [`VarULE`]
/// fields packed into a single end-of-struct field. It is not recommended
/// to use this type directly.
///
/// Logically, consider it to be `(V1, V2, V3, ..)`
/// where `V1` etc are potentially different [`VarULE`] types.
///
/// Internally, it is represented by a VarZeroSlice.
#[repr(transparent)]
pub struct MultiFieldsULE(VarZeroSlice<[u8]>);

impl MultiFieldsULE {
    /// Compute the amount of bytes needed to support elements with lengths `lengths`
    #[inline]
    pub fn compute_encoded_len_for(lengths: &[usize]) -> usize {
        unsafe {
            // safe since BlankSliceEncoder is transparent over usize
            let lengths = &*(lengths as *const [usize] as *const [BlankSliceEncoder]);
            crate::varzerovec::components::compute_serializable_len(lengths)
                .expect("Too many bytes to encode") as usize
        }
    }

    /// Construct a partially initialized MultiFieldsULE backed by a mutable byte buffer
    pub fn new_from_lengths_partially_initialized<'a>(
        lengths: &[usize],
        output: &'a mut [u8],
    ) -> &'a mut Self {
        unsafe {
            // safe since BlankSliceEncoder is transparent over usize
            let lengths = &*(lengths as *const [usize] as *const [BlankSliceEncoder]);
            crate::varzerovec::components::write_serializable_bytes(lengths, output);
            debug_assert!(
                <VarZeroSlice<[u8]>>::validate_byte_slice(output).is_ok(),
                "Encoded slice must be valid VarZeroSlice"
            );
            // Safe since write_serializable_bytes produces a valid VarZeroSlice buffer
            let slice = <VarZeroSlice<[u8]>>::from_byte_slice_unchecked_mut(output);
            // safe since `Self` is transparent over VarZeroSlice
            mem::transmute::<&mut VarZeroSlice<_>, &mut Self>(slice)
        }
    }

    /// Given a buffer of size obtained by [`Self::compute_encoded_len_for()`], write element A to index idx
    ///
    /// # Safety
    /// - `idx` must be in range
    /// - `T` must be the appropriate type expected by the custom derive in this usage of this type
    #[inline]
    pub unsafe fn set_field_at<T: VarULE, A: EncodeAsVarULE<T>>(&mut self, idx: usize, value: &A) {
        value.encode_var_ule_write(self.0.get_bytes_at_mut(idx))
    }

    /// Validate field at `index` to see if it is a valid `T` VarULE type
    ///
    /// # Safety
    ///
    /// - `index` must be in range
    #[inline]
    pub unsafe fn validate_field<T: VarULE>(&self, index: usize) -> Result<(), ZeroVecError> {
        T::validate_byte_slice(self.0.get_unchecked(index))
    }

    /// Get field at `index` as a value of type T
    ///
    /// # Safety
    ///
    /// - `index` must be in range
    /// - Element at `index` must have been created with the VarULE type T
    #[inline]
    pub unsafe fn get_field<T: VarULE>(&self, index: usize) -> &T {
        T::from_byte_slice_unchecked(self.0.get_unchecked(index))
    }

    /// Construct from a byte slice
    ///
    /// # Safety
    /// - byte slice must be a valid VarZeroSlice<[u8]>
    #[inline]
    pub unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // &Self is transparent over &VZS<..>
        mem::transmute(<VarZeroSlice<[u8]>>::from_byte_slice_unchecked(bytes))
    }
}

/// This lets us conveniently use the EncodeAsVarULE functionality to create
/// `VarZeroVec<[u8]>`s that have the right amount of space for elements
/// without having to duplicate any unsafe code
#[repr(transparent)]
struct BlankSliceEncoder(usize);

unsafe impl EncodeAsVarULE<[u8]> for BlankSliceEncoder {
    fn encode_var_ule_as_slices<R>(&self, _: impl FnOnce(&[&[u8]]) -> R) -> R {
        // unnecessary if the other two are implemented
        unreachable!()
    }

    #[inline]
    fn encode_var_ule_len(&self) -> usize {
        self.0
    }

    #[inline]
    fn encode_var_ule_write(&self, _dst: &mut [u8]) {
        // do nothing
    }
}
