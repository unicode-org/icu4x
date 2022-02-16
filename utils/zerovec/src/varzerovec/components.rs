// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::marker::PhantomData;
use core::ops::Range;
use core::{iter, mem};

fn usizeify(x: RawBytesULE<4>) -> usize {
    u32::from_unaligned(x) as usize
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
pub struct VarZeroVecComponents<'a, T: ?Sized> {
    /// The list of indices into the `things` slice
    indices: &'a [RawBytesULE<4>],
    /// The contiguous list of `T::VarULE`s
    things: &'a [u8],
    /// The original slice this was constructed from
    entire_slice: &'a [u8],
    marker: PhantomData<&'a T>,
}

// #[derive()] won't work here since we do not want it to be
// bound on T: Copy
impl<'a, T: ?Sized> Copy for VarZeroVecComponents<'a, T> {}
impl<'a, T: ?Sized> Clone for VarZeroVecComponents<'a, T> {
    fn clone(&self) -> Self {
        VarZeroVecComponents {
            indices: self.indices,
            things: self.things,
            entire_slice: self.entire_slice,
            marker: PhantomData,
        }
    }
}

impl<'a, T: VarULE + ?Sized> Default for VarZeroVecComponents<'a, T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: VarULE + ?Sized> VarZeroVecComponents<'a, T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            indices: &[],
            things: &[],
            entire_slice: &[],
            marker: PhantomData,
        }
    }

    /// Construct a new VarZeroVecComponents, checking invariants about the overall buffer size:
    ///
    /// - There must be either zero or at least four bytes (if four, this is the "length" parsed as a usize)
    /// - There must be at least `4*length + 4` bytes total, to form the the array `indices` of indices
    /// - `indices[i]..indices[i+1]` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices[len - 1]..things.len()` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    #[inline]
    pub fn parse_byte_slice(slice: &'a [u8]) -> Result<Self, ZeroVecError> {
        if slice.is_empty() {
            return Ok(VarZeroVecComponents {
                indices: &[],
                things: &[],
                entire_slice: slice,
                marker: PhantomData,
            });
        }
        let len_bytes = slice.get(0..4).ok_or(ZeroVecError::VarZeroVecFormatError)?;
        let len_ule = RawBytesULE::<4>::parse_byte_slice(len_bytes)
            .map_err(|_| ZeroVecError::VarZeroVecFormatError)?;

        let len = u32::from_unaligned(*len_ule.get(0).ok_or(ZeroVecError::VarZeroVecFormatError)?)
            as usize;
        let indices_bytes = slice
            .get(4..4 * len + 4)
            .ok_or(ZeroVecError::VarZeroVecFormatError)?;
        let indices = RawBytesULE::<4>::parse_byte_slice(indices_bytes)
            .map_err(|_| ZeroVecError::VarZeroVecFormatError)?;
        let things = slice
            .get(4 * len + 4..)
            .ok_or(ZeroVecError::VarZeroVecFormatError)?;

        let borrowed = VarZeroVecComponents {
            indices,
            things,
            entire_slice: slice,
            marker: PhantomData,
        };

        for thing in borrowed.iter_checked() {
            let _ = thing?;
        }

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
        if slice.is_empty() {
            return VarZeroVecComponents {
                indices: &[],
                things: &[],
                entire_slice: slice,
                marker: PhantomData,
            };
        }
        let len_bytes = slice.get_unchecked(0..4);
        let len_ule = RawBytesULE::<4>::from_byte_slice_unchecked(len_bytes);

        let len = u32::from_unaligned(*len_ule.get_unchecked(0)) as usize;
        let indices_bytes = slice.get_unchecked(4..4 * len + 4);
        let indices = RawBytesULE::<4>::from_byte_slice_unchecked(indices_bytes);
        let things = slice.get_unchecked(4 * len + 4..);

        VarZeroVecComponents {
            indices,
            things,
            entire_slice: slice,
            marker: PhantomData,
        }
    }

    /// Get the number of elements in this vector
    #[inline]
    pub fn len(self) -> usize {
        self.indices.len()
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
    unsafe fn get_unchecked(self, idx: usize) -> &'a T {
        let start = usizeify(*self.indices.get_unchecked(idx));
        let end = if idx + 1 == self.len() {
            self.things.len()
        } else {
            usizeify(*self.indices.get_unchecked(idx + 1))
        };
        debug_assert!(start <= end);
        let things_slice = self.things.get_unchecked(start..end);
        T::from_byte_slice_unchecked(things_slice)
    }

    /// Create an iterator over the Ts contained in VarZeroVecComponents, checking internal invariants:
    ///
    /// - `indices[i]..indices[i+1]` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices[len - 1]..things.len()` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices` is monotonically increasing
    ///
    /// This method is NOT allowed to call any other methods on VarZeroVecComponents since all other methods
    /// assume that the slice has been passed through iter_checked
    #[inline]
    fn iter_checked(self) -> impl Iterator<Item = Result<&'a T, ZeroVecError>> {
        let last = iter::from_fn(move || {
            if !self.is_empty() {
                let start = usizeify(self.indices[self.len() - 1]);
                let end = self.things.len();
                Some(
                    self.things
                        .get(start..end)
                        .ok_or(ZeroVecError::VarZeroVecFormatError),
                )
            } else {
                None
            }
        })
        .take(1);
        self.indices
            .windows(2)
            .map(move |win| {
                let start = usizeify(win[0]);
                let end = usizeify(win[1]);
                // the .get() here automatically verifies that end>=start
                self.things
                    .get(start..end)
                    .ok_or(ZeroVecError::VarZeroVecFormatError)
            })
            .chain(last)
            .map(|s| s.and_then(|s| T::parse_byte_slice(s)))
    }

    /// Create an iterator over the Ts contained in VarZeroVecComponents
    #[inline]
    pub fn iter(self) -> impl Iterator<Item = &'a T> {
        let last = iter::from_fn(move || {
            if !self.is_empty() {
                let start = usizeify(self.indices[self.len() - 1]);
                let end = self.things.len();
                Some(unsafe { self.things.get_unchecked(start..end) })
            } else {
                None
            }
        })
        .take(1);
        self.indices
            .windows(2)
            .map(move |win| {
                let start = usizeify(win[0]);
                let end = usizeify(win[1]);
                unsafe { self.things.get_unchecked(start..end) }
            })
            .chain(last)
            .map(|s| unsafe { T::from_byte_slice_unchecked(s) })
    }

    pub fn to_vec(self) -> Vec<Box<T>> {
        self.iter().map(T::to_boxed).collect()
    }

    // Dump a debuggable representation of this type
    #[allow(unused)] // useful for debugging
    pub(crate) fn dump(&self) -> String {
        let indices = self
            .indices
            .iter()
            .copied()
            .map(u32::from_unaligned)
            .collect::<Vec<_>>();
        format!("VarZeroVecComponents {{ indices: {:?} }}", indices)
    }
}

impl<'a, T> VarZeroVecComponents<'a, T>
where
    T: VarULE,
    T: ?Sized,
    T: Ord,
{
    /// Binary searches a sorted `VarZeroVecComponents<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`](slice::binary_search).
    pub fn binary_search(&self, needle: &T) -> Result<usize, usize> {
        self.binary_search_impl(|probe| probe.cmp(needle), self.indices)
    }

    pub fn binary_search_in_range(
        &self,
        needle: &T,
        range: Range<usize>,
    ) -> Option<Result<usize, usize>> {
        let indices_slice = self.indices.get(range)?;
        Some(self.binary_search_impl(|probe| probe.cmp(needle), indices_slice))
    }
}

impl<'a, T> VarZeroVecComponents<'a, T>
where
    T: VarULE,
    T: ?Sized,
{
    /// Binary searches a sorted `VarZeroVecComponents<T>` for the given predicate. For more information, see
    /// the primitive function [`binary_search_by`](slice::binary_search_by).
    pub fn binary_search_by(&self, predicate: impl FnMut(&T) -> Ordering) -> Result<usize, usize> {
        self.binary_search_impl(predicate, self.indices)
    }

    /// Binary searches a sorted `VarZeroVecComponents<T>` with the given predicate. For more information, see
    /// the primitive function [`binary_search`](slice::binary_search).
    fn binary_search_impl(
        &self,
        mut predicate: impl FnMut(&T) -> Ordering,
        indices_slice: &[RawBytesULE<4>],
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
            // `self.indices` is a vec of unaligned u32s, so we divide by sizeof(u32)
            // to get the actual index
            let index = (probe as *const _ as usize - zero_index) / mem::size_of::<u32>();
            // safety: we know this is in bounds
            let actual_probe = unsafe { self.get_unchecked(index) };
            predicate(actual_probe)
        })
    }
}

/// Collects the bytes for a VarZeroSlice into a Vec.
pub fn get_serializable_bytes<T, A>(elements: &[A]) -> Option<Vec<u8>>
where
    T: VarULE + ?Sized,
    A: custom::EncodeAsVarULE<T>,
{
    let len = compute_serializable_len(elements)?;
    debug_assert!(len >= 4);
    let mut output: Vec<u8> = alloc::vec![0; len as usize];
    write_serializable_bytes(elements, &mut output);
    Some(output)
}

/// Writes the bytes for a VarZeroSlice into an output buffer.
///
/// Every byte in the buffer will be initialized after calling this function.
///
/// # Panics
///
/// Panics if the buffer is not exactly the correct length.
pub fn write_serializable_bytes<T, A>(elements: &[A], output: &mut [u8])
where
    T: VarULE + ?Sized,
    A: custom::EncodeAsVarULE<T>,
{
    let num_elements = u32::try_from(elements.len()).ok().unwrap();
    output[0..4].copy_from_slice(&num_elements.as_unaligned().0);

    // idx_offset = offset from the start of the buffer for the next index
    let mut idx_offset: usize = 4;
    // first_dat_offset = offset from the start of the buffer of the first data block
    let first_dat_offset: usize = 4 + elements.len() * 4;
    // dat_offset = offset from the start of the buffer of the next data block
    let mut dat_offset: usize = first_dat_offset;

    for element in elements.iter() {
        let element_len = element.encode_var_ule_len();

        let idx_limit = idx_offset + 4;
        let idx_slice = &mut output[idx_offset..idx_limit];
        // VZV expects data offsets to be stored relative to the first data block
        let offset = (dat_offset - first_dat_offset) as u32;
        idx_slice.copy_from_slice(&offset.as_unaligned().0);

        let dat_limit = dat_offset + element_len;
        let dat_slice = &mut output[dat_offset..dat_limit];
        element.encode_var_ule_write(dat_slice);
        debug_assert_eq!(T::validate_byte_slice(dat_slice), Ok(()));

        idx_offset = idx_limit;
        dat_offset = dat_limit;
    }

    debug_assert_eq!(idx_offset, 4 + 4 * elements.len());
    assert_eq!(dat_offset, output.len());
}

pub fn compute_serializable_len<T, A>(elements: &[A]) -> Option<u32>
where
    T: VarULE + ?Sized,
    A: custom::EncodeAsVarULE<T>,
{
    // 4 for length + 4 for each offset + the body
    let idx_len: u32 = u32::try_from(elements.len())
        .ok()?
        .checked_mul(4)?
        .checked_add(4)?;
    let data_len: u32 = elements
        .iter()
        .map(|v| u32::try_from(v.encode_var_ule_len()).ok())
        .fold(Some(0u32), |s, v| {
            s.and_then(|s| v.and_then(|v| s.checked_add(v)))
        })?;
    idx_len.checked_add(data_len)
}
