// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::VarZeroVecFormatError;
use crate::ule::*;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::marker::PhantomData;
use core::mem;
use core::ops::Range;

// Also used by owned.rs
pub(super) const METADATA_WIDTH: usize = 0;
pub(super) const MAX_INDEX: usize = u32::MAX as usize;

/// This trait allows switching between different possible internal
/// representations of VarZeroVec.
///
/// Currently this crate supports two formats: [`Index16`] and [`Index32`],
/// with [`Index16`] being the default for all [`VarZeroVec`](super::VarZeroVec)
/// types unless explicitly specified otherwise.
///
/// Do not implement this trait, its internals may be changed in the future,
/// and all of its associated items are hidden from the docs.
pub trait VarZeroVecFormat: 'static + Sized {
    /// The type to use for the indexing array
    ///
    /// Safety: must be a ULE for which all byte sequences are allowed
    #[doc(hidden)]
    type Index: IntegerULE;
    /// The type to use for the length segment
    ///
    /// Safety: must be a ULE for which all byte sequences are allowed
    #[doc(hidden)]
    type Len: IntegerULE;
}

/// This trait represents various ULE types that can be used to represent an integer
///
/// Do not implement this trait, its internals may be changed in the future,
/// and all of its associated items are hidden from the docs.
#[allow(clippy::missing_safety_doc)] // no safety section for you, don't implement this trait period
#[doc(hidden)]
pub unsafe trait IntegerULE: ULE {
    /// The error to show when unable to construct a vec
    #[doc(hidden)]
    const TOO_LARGE_ERROR: &'static str;

    /// Safety: must be sizeof(self)
    #[doc(hidden)]
    const SIZE: usize;

    /// Safety: must be maximum integral value represented here
    #[doc(hidden)]
    const MAX_VALUE: u32;

    /// Safety: Must roundtrip with from_usize and represent the correct
    /// integral value
    #[doc(hidden)]
    fn iule_to_usize(self) -> usize;

    #[doc(hidden)]
    fn iule_from_usize(x: usize) -> Option<Self>;

    /// Safety: Should always convert a buffer into an array of Self with the correct length
    #[doc(hidden)]
    fn iule_from_byte_slice_unchecked_mut(bytes: &mut [u8]) -> &mut [Self];
}

/// This is a [`VarZeroVecFormat`] that stores u8s in the index array.
/// Will have a smaller data size, but it's *extremely* likely for larger arrays
/// to be unrepresentable (and error on construction). Should probably be used
/// for known-small arrays, where all but the last field are known-small.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // marker
pub struct Index8;

/// This is a [`VarZeroVecFormat`] that stores u16s in the index array.
/// Will have a smaller data size, but it's more likely for larger arrays
/// to be unrepresentable (and error on construction)
///
/// This is the default index size used by all [`VarZeroVec`](super::VarZeroVec) types.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // marker
pub struct Index16;

/// This is a [`VarZeroVecFormat`] that stores u32s in the index array.
/// Will have a larger data size, but will support large arrays without
/// problems.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // marker
pub struct Index32;

impl VarZeroVecFormat for Index8 {
    type Index = u8;
    type Len = u8;
}

impl VarZeroVecFormat for Index16 {
    type Index = RawBytesULE<2>;
    type Len = RawBytesULE<2>;
}

impl VarZeroVecFormat for Index32 {
    type Index = RawBytesULE<4>;
    type Len = RawBytesULE<4>;
}

unsafe impl IntegerULE for u8 {
    const TOO_LARGE_ERROR: &'static str = "Attempted to build VarZeroVec out of elements that \
                                     cumulatively are larger than a u8 in size";
    const SIZE: usize = mem::size_of::<Self>();
    const MAX_VALUE: u32 = u8::MAX as u32;
    #[inline]
    fn iule_to_usize(self) -> usize {
        self as usize
    }
    #[inline]
    fn iule_from_usize(u: usize) -> Option<Self> {
        u8::try_from(u).ok()
    }
    #[inline]
    fn iule_from_byte_slice_unchecked_mut(bytes: &mut [u8]) -> &mut [Self] {
        bytes
    }
}

unsafe impl IntegerULE for RawBytesULE<2> {
    const TOO_LARGE_ERROR: &'static str = "Attempted to build VarZeroVec out of elements that \
                                     cumulatively are larger than a u16 in size";
    const SIZE: usize = mem::size_of::<Self>();
    const MAX_VALUE: u32 = u16::MAX as u32;
    #[inline]
    fn iule_to_usize(self) -> usize {
        self.as_unsigned_int() as usize
    }
    #[inline]
    fn iule_from_usize(u: usize) -> Option<Self> {
        u16::try_from(u).ok().map(u16::to_unaligned)
    }
    #[inline]
    fn iule_from_byte_slice_unchecked_mut(bytes: &mut [u8]) -> &mut [Self] {
        Self::from_byte_slice_unchecked_mut(bytes)
    }
}

unsafe impl IntegerULE for RawBytesULE<4> {
    const TOO_LARGE_ERROR: &'static str = "Attempted to build VarZeroVec out of elements that \
                                     cumulatively are larger than a u32 in size";
    const SIZE: usize = mem::size_of::<Self>();
    const MAX_VALUE: u32 = u32::MAX;
    #[inline]
    fn iule_to_usize(self) -> usize {
        self.as_unsigned_int() as usize
    }
    #[inline]
    fn iule_from_usize(u: usize) -> Option<Self> {
        u32::try_from(u).ok().map(u32::to_unaligned)
    }
    #[inline]
    fn iule_from_byte_slice_unchecked_mut(bytes: &mut [u8]) -> &mut [Self] {
        Self::from_byte_slice_unchecked_mut(bytes)
    }
}

/// A more parsed version of `VarZeroSlice`. This type is where most of the VarZeroVec
/// internal representation code lies.
///
/// This is *basically* an `&'a [u8]` to a zero copy buffer, but split out into
/// the buffer components. Logically this is capable of behaving as
/// a `&'a [T::VarULE]`, but since `T::VarULE` is unsized that type does not actually
/// exist.
///
/// See [`VarZeroVecComponents::parse_byte_slice()`] for information on the internal invariants involved
#[derive(Debug)]
pub struct VarZeroVecComponents<'a, T: ?Sized, F> {
    /// The number of elements
    len: u32,
    /// The list of indices into the `things` slice
    indices: &'a [u8],
    /// The contiguous list of `T::VarULE`s
    things: &'a [u8],
    marker: PhantomData<(&'a T, F)>,
}

// #[derive()] won't work here since we do not want it to be
// bound on T: Copy
impl<'a, T: ?Sized, F> Copy for VarZeroVecComponents<'a, T, F> {}
impl<'a, T: ?Sized, F> Clone for VarZeroVecComponents<'a, T, F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: VarULE + ?Sized, F> Default for VarZeroVecComponents<'a, T, F> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: VarULE + ?Sized, F> VarZeroVecComponents<'a, T, F> {
    #[inline]
    pub fn new() -> Self {
        Self {
            len: 0,
            indices: &[],
            things: &[],
            marker: PhantomData,
        }
    }
}
impl<'a, T: VarULE + ?Sized, F: VarZeroVecFormat> VarZeroVecComponents<'a, T, F> {
    /// Construct a new VarZeroVecComponents, checking invariants about the overall buffer size:
    ///
    /// - There must be either zero or at least four bytes (if four, this is the "length" parsed as a usize)
    /// - There must be at least `4*length + 4` bytes total, to form the array `indices` of indices
    /// - `indices[i]..indices[i+1]` must index into a valid section of
    ///   `things` (the data after `indices`), such that it parses to a `T::VarULE`
    /// - `indices[len - 1]..things.len()` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    #[inline]
    pub fn parse_byte_slice(slice: &'a [u8]) -> Result<Self, VarZeroVecFormatError> {
        // The empty VZV is special-cased to the empty slice
        if slice.is_empty() {
            return Ok(VarZeroVecComponents {
                len: 0,
                indices: &[],
                things: &[],
                marker: PhantomData,
            });
        }
        let len_bytes = slice
            .get(0..F::Len::SIZE)
            .ok_or(VarZeroVecFormatError::Metadata)?;
        let len_ule =
            F::Len::parse_byte_slice(len_bytes).map_err(|_| VarZeroVecFormatError::Metadata)?;

        let len = len_ule
            .first()
            .ok_or(VarZeroVecFormatError::Metadata)?
            .iule_to_usize();

        let rest = slice
            .get(F::Len::SIZE..)
            .ok_or(VarZeroVecFormatError::Metadata)?;
        let len_u32 = u32::try_from(len).map_err(|_| VarZeroVecFormatError::Metadata);
        // We pass down the rest of the invariants
        Self::parse_byte_slice_with_length(len_u32?, rest)
    }

    /// Construct a new VarZeroVecComponents, checking invariants about the overall buffer size:
    ///
    /// - There must be at least `4*len` bytes total, to form the array `indices` of indices.
    /// - `indices[i]..indices[i+1]` must index into a valid section of
    ///   `things` (the data after `indices`), such that it parses to a `T::VarULE`
    /// - `indices[len - 1]..things.len()` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    #[inline]
    pub fn parse_byte_slice_with_length(
        len: u32,
        slice: &'a [u8],
    ) -> Result<Self, VarZeroVecFormatError> {
        // The empty VZV is special-cased to the empty slice
        if len == 0 {
            return Ok(VarZeroVecComponents {
                len: 0,
                indices: &[],
                things: &[],
                marker: PhantomData,
            });
        }
        let indices_bytes = slice
            .get(METADATA_WIDTH..METADATA_WIDTH + F::Index::SIZE * (len as usize))
            .ok_or(VarZeroVecFormatError::Metadata)?;
        let things = slice
            .get(F::Index::SIZE * (len as usize) + METADATA_WIDTH..)
            .ok_or(VarZeroVecFormatError::Metadata)?;

        let borrowed = VarZeroVecComponents {
            len,
            indices: indices_bytes,
            things,
            marker: PhantomData,
        };

        borrowed.check_indices_and_things()?;

        Ok(borrowed)
    }

    /// Construct a [`VarZeroVecComponents`] from a byte slice that has previously
    /// successfully returned a [`VarZeroVecComponents`] when passed to
    /// [`VarZeroVecComponents::parse_byte_slice()`]. Will return the same
    /// object as one would get from calling [`VarZeroVecComponents::parse_byte_slice()`].
    ///
    /// # Safety
    /// The bytes must have previously successfully run through
    /// [`VarZeroVecComponents::parse_byte_slice()`]
    pub unsafe fn from_bytes_unchecked(slice: &'a [u8]) -> Self {
        // The empty VZV is special-cased to the empty slice
        if slice.is_empty() {
            return VarZeroVecComponents {
                len: 0,
                indices: &[],
                things: &[],
                marker: PhantomData,
            };
        }
        let len_bytes = slice.get_unchecked(0..F::Len::SIZE);
        // Safety: F::Len allows all byte sequences
        let len_ule = F::Len::from_byte_slice_unchecked(len_bytes);

        let len = len_ule.get_unchecked(0).iule_to_usize();
        let len_u32 = len as u32;

        // Safety: This method requires the bytes to have passed through `parse_byte_slice()`
        // whereas we're calling something that asks for `parse_byte_slice_with_length()`.
        // The two methods perform similar validation, with parse_byte_slice() validating an additional
        // 4-byte `length` header.
        Self::from_bytes_unchecked_with_length(len_u32, slice.get_unchecked(F::Len::SIZE..))
    }

    /// Construct a [`VarZeroVecComponents`] from a byte slice that has previously
    /// successfully returned a [`VarZeroVecComponents`] when passed to
    /// [`VarZeroVecComponents::parse_byte_slice()`]. Will return the same
    /// object as one would get from calling [`VarZeroVecComponents::parse_byte_slice()`].
    ///
    /// # Safety
    /// The len,bytes must have previously successfully run through
    /// [`VarZeroVecComponents::parse_byte_slice_with_length()`]
    pub unsafe fn from_bytes_unchecked_with_length(len: u32, slice: &'a [u8]) -> Self {
        // The empty VZV is special-cased to the empty slice
        if len == 0 {
            return VarZeroVecComponents {
                len: 0,
                indices: &[],
                things: &[],
                marker: PhantomData,
            };
        }
        let indices_bytes =
            slice.get_unchecked(METADATA_WIDTH..METADATA_WIDTH + F::Index::SIZE * (len as usize));
        let things = slice.get_unchecked(METADATA_WIDTH + F::Index::SIZE * (len as usize)..);

        VarZeroVecComponents {
            len,
            indices: indices_bytes,
            things,
            marker: PhantomData,
        }
    }

    /// Get the number of elements in this vector
    #[inline]
    pub fn len(self) -> usize {
        self.len as usize
    }

    /// Returns `true` if the vector contains no elements.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.indices.is_empty()
    }

    /// Get the idx'th element out of this slice. Returns `None` if out of bounds.
    #[inline]
    pub fn get(self, idx: usize) -> Option<&'a T> {
        if idx >= self.len() {
            return None;
        }
        Some(unsafe { self.get_unchecked(idx) })
    }

    /// Get the idx'th element out of this slice. Does not bounds check.
    ///
    /// Safety:
    /// - `idx` must be in bounds (`idx < self.len()`)
    #[inline]
    pub(crate) unsafe fn get_unchecked(self, idx: usize) -> &'a T {
        let range = self.get_things_range(idx);
        let things_slice = self.things.get_unchecked(range);
        T::from_byte_slice_unchecked(things_slice)
    }

    /// Get the range in `things` for the element at `idx`. Does not bounds check.
    ///
    /// Safety:
    /// - `idx` must be in bounds (`idx < self.len()`)
    #[inline]
    pub(crate) unsafe fn get_things_range(self, idx: usize) -> Range<usize> {
        let start = self.indices_slice().get_unchecked(idx).iule_to_usize();
        let end = if idx + 1 == self.len() {
            self.things.len()
        } else {
            self.indices_slice().get_unchecked(idx + 1).iule_to_usize()
        };
        debug_assert!(start <= end);
        start..end
    }

    /// Get the size, in bytes, of the indices array
    pub(crate) unsafe fn get_indices_size(self) -> usize {
        self.indices.len()
    }

    /// Check the internal invariants of VarZeroVecComponents:
    ///
    /// - `indices[i]..indices[i+1]` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices[len - 1]..things.len()` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices` is monotonically increasing
    ///
    /// This method is NOT allowed to call any other methods on VarZeroVecComponents since all other methods
    /// assume that the slice has been passed through check_indices_and_things
    #[inline]
    #[allow(clippy::len_zero)] // more explicit to enforce safety invariants
    fn check_indices_and_things(self) -> Result<(), VarZeroVecFormatError> {
        assert_eq!(self.len(), self.indices_slice().len());
        if self.len() == 0 {
            if self.things.len() > 0 {
                return Err(VarZeroVecFormatError::Metadata);
            } else {
                return Ok(());
            }
        }
        // Safety: i is in bounds (assertion above)
        let mut start = (unsafe { *self.indices_slice().get_unchecked(0) }).iule_to_usize();
        if start != 0 {
            return Err(VarZeroVecFormatError::Metadata);
        }
        for i in 0..self.len() {
            let end = if i == self.len() - 1 {
                self.things.len()
            } else {
                // Safety: i+1 is in bounds (assertion above)
                (unsafe { *self.indices_slice().get_unchecked(i + 1) }).iule_to_usize()
            };
            if start > end {
                return Err(VarZeroVecFormatError::Metadata);
            }
            if end > self.things.len() {
                return Err(VarZeroVecFormatError::Metadata);
            }
            // Safety: start..end is a valid range in self.things
            let bytes = unsafe { self.things.get_unchecked(start..end) };
            T::parse_byte_slice(bytes).map_err(VarZeroVecFormatError::Values)?;
            start = end;
        }
        Ok(())
    }

    /// Create an iterator over the Ts contained in VarZeroVecComponents
    #[inline]
    pub fn iter(self) -> impl Iterator<Item = &'a T> {
        self.indices_slice()
            .iter()
            .copied()
            .map(IntegerULE::iule_to_usize)
            .zip(
                self.indices_slice()
                    .iter()
                    .copied()
                    .map(IntegerULE::iule_to_usize)
                    .skip(1)
                    .chain([self.things.len()]),
            )
            .map(move |(start, end)| unsafe { self.things.get_unchecked(start..end) })
            .map(|bytes| unsafe { T::from_byte_slice_unchecked(bytes) })
    }

    pub fn to_vec(self) -> Vec<Box<T>> {
        self.iter().map(T::to_boxed).collect()
    }

    #[inline]
    fn indices_slice(&self) -> &'a [F::Index] {
        unsafe { F::Index::from_byte_slice_unchecked(self.indices) }
    }

    // Dump a debuggable representation of this type
    #[allow(unused)] // useful for debugging
    pub(crate) fn dump(&self) -> String {
        let indices = self
            .indices_slice()
            .iter()
            .copied()
            .map(IntegerULE::iule_to_usize)
            .collect::<Vec<_>>();
        format!("VarZeroVecComponents {{ indices: {indices:?} }}")
    }
}

impl<'a, T, F> VarZeroVecComponents<'a, T, F>
where
    T: VarULE,
    T: ?Sized,
    T: Ord,
    F: VarZeroVecFormat,
{
    /// Binary searches a sorted `VarZeroVecComponents<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`](slice::binary_search).
    pub fn binary_search(&self, needle: &T) -> Result<usize, usize> {
        self.binary_search_impl(|probe| probe.cmp(needle), self.indices_slice())
    }

    pub fn binary_search_in_range(
        &self,
        needle: &T,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>> {
        let indices_slice = self.indices_slice().get(range)?;
        Some(self.binary_search_impl(|probe| probe.cmp(needle), indices_slice))
    }
}

impl<'a, T, F> VarZeroVecComponents<'a, T, F>
where
    T: VarULE,
    T: ?Sized,
    F: VarZeroVecFormat,
{
    /// Binary searches a sorted `VarZeroVecComponents<T>` for the given predicate. For more information, see
    /// the primitive function [`binary_search_by`](slice::binary_search_by).
    pub fn binary_search_by(&self, predicate: impl FnMut(&T) -> Ordering) -> Result<usize, usize> {
        self.binary_search_impl(predicate, self.indices_slice())
    }

    pub fn binary_search_in_range_by(
        &self,
        predicate: impl FnMut(&T) -> Ordering,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>> {
        let indices_slice = self.indices_slice().get(range)?;
        Some(self.binary_search_impl(predicate, indices_slice))
    }

    /// Binary searches a sorted `VarZeroVecComponents<T>` with the given predicate. For more information, see
    /// the primitive function [`binary_search`](slice::binary_search).
    fn binary_search_impl(
        &self,
        mut predicate: impl FnMut(&T) -> Ordering,
        indices_slice: &[F::Index],
    ) -> Result<usize, usize> {
        // This code is an absolute atrocity. This code is not a place of honor. This
        // code is known to the State of California to cause cancer.
        //
        // Unfortunately, the stdlib's `binary_search*` functions can only operate on slices.
        // We do not have a slice. We have something we can .get() and index on, but that is not
        // a slice.
        //
        // The `binary_search*` functions also do not have a variant where they give you the element's
        // index, which we could otherwise use to directly index `self`.
        // We do have `self.indices`, but these are indices into a byte buffer, which cannot in
        // isolation be used to recoup the logical index of the element they refer to.
        //
        // However, `binary_search_by()` provides references to the elements of the slice being iterated.
        // Since the layout of Rust slices is well-defined, we can do pointer arithmetic on these references
        // to obtain the index being used by the search.
        //
        // It's worth noting that the slice we choose to search is irrelevant, as long as it has the appropriate
        // length. `self.indices` is defined to have length `self.len()`, so it is convenient to use
        // here and does not require additional allocations.
        //
        // The alternative to doing this is to implement our own binary search. This is significantly less fun.

        // Note: We always use zero_index relative to the whole indices array, even if we are
        // only searching a subslice of it.
        let zero_index = self.indices.as_ptr() as *const _ as usize;
        indices_slice.binary_search_by(|probe: &_| {
            // `self.indices` is a vec of unaligned F::Index::SIZE values, so we divide by F::Index::SIZE
            // to get the actual index
            let index = (probe as *const _ as usize - zero_index) / F::Index::SIZE;
            // safety: we know this is in bounds
            let actual_probe = unsafe { self.get_unchecked(index) };
            predicate(actual_probe)
        })
    }
}

/// Collects the bytes for a VarZeroSlice into a Vec.
pub fn get_serializable_bytes_non_empty<T, A, F>(elements: &[A]) -> Option<Vec<u8>>
where
    T: VarULE + ?Sized,
    A: EncodeAsVarULE<T>,
    F: VarZeroVecFormat,
{
    debug_assert!(!elements.is_empty());
    let len = compute_serializable_len::<T, A, F>(elements)?;
    debug_assert!(
        len >= F::Len::SIZE as u32,
        "Must have at least F::Len::SIZE bytes to hold the length of the vector"
    );
    let mut output: Vec<u8> = alloc::vec![0; len as usize];
    write_serializable_bytes::<T, A, F>(elements, &mut output);
    Some(output)
}

/// Writes the bytes for a VarZeroLengthlessSlice into an output buffer.
/// Usable for a VarZeroSlice if you first write the length bytes.
///
/// Every byte in the buffer will be initialized after calling this function.
///
/// # Panics
///
/// Panics if the buffer is not exactly the correct length.
pub fn write_serializable_bytes_without_length<T, A, F>(elements: &[A], output: &mut [u8])
where
    T: VarULE + ?Sized,
    A: EncodeAsVarULE<T>,
    F: VarZeroVecFormat,
{
    assert!(elements.len() <= F::Len::MAX_VALUE as usize);
    if elements.is_empty() {
        return;
    }

    // idx_offset = offset from the start of the buffer for the next index
    let mut idx_offset: usize = METADATA_WIDTH;
    // first_dat_offset = offset from the start of the buffer of the first data block
    let first_dat_offset: usize = idx_offset + elements.len() * F::Index::SIZE;
    // dat_offset = offset from the start of the buffer of the next data block
    let mut dat_offset: usize = first_dat_offset;

    for element in elements.iter() {
        let element_len = element.encode_var_ule_len();

        let idx_limit = idx_offset + F::Index::SIZE;
        #[allow(clippy::indexing_slicing)] // Function contract allows panicky behavior
        let idx_slice = &mut output[idx_offset..idx_limit];
        // VZV expects data offsets to be stored relative to the first data block
        let idx = dat_offset - first_dat_offset;
        assert!(idx <= MAX_INDEX);
        #[allow(clippy::expect_used)] // this function is explicitly panicky
        let bytes_to_write = F::Index::iule_from_usize(idx).expect(F::Index::TOO_LARGE_ERROR);
        idx_slice.copy_from_slice(ULE::as_byte_slice(&[bytes_to_write]));

        let dat_limit = dat_offset + element_len;
        #[allow(clippy::indexing_slicing)] // Function contract allows panicky behavior
        let dat_slice = &mut output[dat_offset..dat_limit];
        element.encode_var_ule_write(dat_slice);
        debug_assert_eq!(T::validate_byte_slice(dat_slice), Ok(()));

        idx_offset = idx_limit;
        dat_offset = dat_limit;
    }

    debug_assert_eq!(idx_offset, METADATA_WIDTH + F::Index::SIZE * elements.len());
    assert_eq!(dat_offset, output.len());
}

/// Writes the bytes for a VarZeroSlice into an output buffer.
///
/// Every byte in the buffer will be initialized after calling this function.
///
/// # Panics
///
/// Panics if the buffer is not exactly the correct length.
pub fn write_serializable_bytes<T, A, F>(elements: &[A], output: &mut [u8])
where
    T: VarULE + ?Sized,
    A: EncodeAsVarULE<T>,
    F: VarZeroVecFormat,
{
    if elements.is_empty() {
        return;
    }
    assert!(elements.len() <= F::Len::MAX_VALUE as usize);
    #[allow(clippy::expect_used)] // This function is explicitly panicky
    let num_elements_ule = F::Len::iule_from_usize(elements.len()).expect(F::Len::TOO_LARGE_ERROR);
    #[allow(clippy::indexing_slicing)] // Function contract allows panicky behavior
    output[0..F::Len::SIZE].copy_from_slice(ULE::as_byte_slice(&[num_elements_ule]));

    #[allow(clippy::indexing_slicing)] // Function contract allows panicky behavior
    write_serializable_bytes_without_length::<T, A, F>(elements, &mut output[F::Len::SIZE..]);
}

pub fn compute_serializable_len_without_length<T, A, F>(elements: &[A]) -> Option<u32>
where
    T: VarULE + ?Sized,
    A: EncodeAsVarULE<T>,
    F: VarZeroVecFormat,
{
    if elements.is_empty() {
        return Some(0);
    }
    let idx_len: u32 = u32::try_from(elements.len())
        .ok()?
        .checked_mul(F::Index::SIZE as u32)?
        .checked_add(METADATA_WIDTH as u32)?;
    let data_len: u32 = elements
        .iter()
        .map(|v| u32::try_from(v.encode_var_ule_len()).ok())
        .try_fold(0u32, |s, v| s.checked_add(v?))?;
    let ret = idx_len.checked_add(data_len);
    if let Some(r) = ret {
        if r >= F::Index::MAX_VALUE {
            return None;
        }
    }
    ret
}

pub fn compute_serializable_len<T, A, F>(elements: &[A]) -> Option<u32>
where
    T: VarULE + ?Sized,
    A: EncodeAsVarULE<T>,
    F: VarZeroVecFormat,
{
    compute_serializable_len_without_length::<T, A, F>(elements).map(|x| x + F::Len::SIZE as u32)
}
