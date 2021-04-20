// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use std::convert::TryInto;
use std::marker::PhantomData;
use std::{iter, mem};

fn usizeify(x: PlainOldULE<4>) -> usize {
    u32::from_unaligned(&x) as usize
}

/// A logical representation of the backing `&[u8]` buffer.
///
/// This is where the actual work involved in VarZeroVec happens
///
/// See [`SliceComponents::try_from_bytes()`] for information on the internal invariants involved
pub struct SliceComponents<'a, T> {
    /// The list of indices into the `things` slice
    indices: &'a [PlainOldULE<4>],
    /// The contiguous list of `T::VarULE`s
    things: &'a [u8],
    /// The original slice this was constructed from
    entire_slice: &'a [u8],
    marker: PhantomData<&'a [T]>,
}

// #[derive()] won't work here since we do not want it to be
// bound on T: Copy
impl<'a, T> Copy for SliceComponents<'a, T> {}
impl<'a, T> Clone for SliceComponents<'a, T> {
    fn clone(&self) -> Self {
        SliceComponents {
            indices: self.indices,
            things: self.things,
            entire_slice: self.entire_slice,
            marker: PhantomData,
        }
    }
}

impl<'a, T: AsVarULE> SliceComponents<'a, T> {
    /// Construct a new SliceComponents, checking invariants about the overall buffer size:
    ///
    /// - There must be either zero or at least four bytes (if four, this is the "length" parsed as a usize)
    /// - There must be at least `4*length + 4` bytes total, to form the the array `indices` of indices
    /// - `indices[i]..indices[i+1]` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices[len - 1]..things.len()` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    #[inline]
    pub fn try_from_bytes(slice: &'a [u8]) -> Result<Self, ParseErrorFor<T>> {
        if slice.is_empty() {
            return Ok(SliceComponents {
                indices: &[],
                things: &[],
                entire_slice: slice,
                marker: PhantomData,
            });
        }
        let len_bytes = slice.get(0..4).ok_or(VarZeroVecError::FormatError)?;
        let len_ule = PlainOldULE::<4>::parse_byte_slice(len_bytes)
            .map_err(|_| VarZeroVecError::FormatError)?;

        let len = u32::from_unaligned(len_ule.get(0).ok_or(VarZeroVecError::FormatError)?) as usize;
        let indices_bytes = slice
            .get(4..4 * len + 4)
            .ok_or(VarZeroVecError::FormatError)?;
        let indices = PlainOldULE::<4>::parse_byte_slice(indices_bytes)
            .map_err(|_| VarZeroVecError::FormatError)?;
        let things = slice
            .get(4 * len + 4..)
            .ok_or(VarZeroVecError::FormatError)?;

        let components = SliceComponents {
            indices,
            things,
            entire_slice: slice,
            marker: PhantomData,
        };

        for thing in components.iter_checked() {
            let _ = thing?;
        }

        Ok(components)
    }

    #[inline]
    pub fn len(self) -> usize {
        self.indices.len()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.indices.is_empty()
    }

    #[inline]
    #[cfg(feature = "serde")]
    pub fn entire_slice(self) -> &'a [u8] {
        self.entire_slice
    }

    /// Get the idx'th element out of this slice. Returns `None` if out of bounds.
    #[inline]
    pub fn get(self, idx: usize) -> Option<&'a T::VarULE> {
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
    unsafe fn get_unchecked(self, idx: usize) -> &'a T::VarULE {
        let start = usizeify(*self.indices.get_unchecked(idx));
        let end = if idx + 1 == self.len() {
            self.things.len()
        } else {
            usizeify(*self.indices.get_unchecked(idx + 1))
        };
        debug_assert!(start <= end);
        let things_slice = self.things.get_unchecked(start..end);
        T::VarULE::from_byte_slice_unchecked(things_slice)
    }

    /// Create an iterator over the Ts contained in SliceComponents, checking internal invariants:
    ///
    /// - `indices[i]..indices[i+1]` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices[len - 1]..things.len()` must index into a valid section of
    ///   `things`, such that it parses to a `T::VarULE`
    /// - `indices` is monotonically increasing
    ///
    /// This method is NOT allowed to call any other methods on SliceComponents since all other methods
    /// assume that the slice has been passed through iter_checked
    #[inline]
    fn iter_checked(self) -> impl Iterator<Item = Result<&'a T::VarULE, ParseErrorFor<T>>> {
        let last = iter::from_fn(move || {
            if !self.is_empty() {
                let start = usizeify(self.indices[self.len() - 1]);
                let end = self.things.len();
                Some(
                    self.things
                        .get(start..end)
                        .ok_or(VarZeroVecError::FormatError),
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
                    .ok_or(VarZeroVecError::FormatError)
            })
            .chain(last)
            .map(|s| s.and_then(|s| T::VarULE::parse_byte_slice(s).map_err(|e| e.into())))
    }

    /// Create an iterator over the Ts contained in SliceComponents
    #[inline]
    pub fn iter(self) -> impl Iterator<Item = &'a T::VarULE> {
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
            .map(|s| unsafe { T::VarULE::from_byte_slice_unchecked(s) })
    }
}

impl<'a, T> SliceComponents<'a, T>
where
    T: AsVarULE,
    T::VarULE: Ord,
{
    pub fn binary_search(&self, needle: &T::VarULE) -> Result<usize, usize> {
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

        let zero_index = self.indices.as_ptr() as *const _ as usize;
        self.indices.binary_search_by(|probe: &_| {
            // `self.indices` is a vec of unaligned u32s, so we divide by sizeof(u32)
            // to get the actual index
            let index = (probe as *const _ as usize - zero_index) / mem::size_of::<u32>();
            // safety: we know this is in bounds
            let actual_probe = unsafe { self.get_unchecked(index) };
            actual_probe.cmp(needle)
        })
    }
}

pub fn get_serializable_bytes<T: AsVarULE>(elements: &[T]) -> Option<Vec<u8>> {
    let mut vec = Vec::with_capacity(4 + 4 * elements.len());
    let len_u32: u32 = elements.len().try_into().ok()?;
    vec.extend(&len_u32.as_unaligned().0);
    let mut offset: u32 = 0;
    for element in elements {
        vec.extend(&offset.as_unaligned().0);
        let len_u32: u32 = element
            .as_unaligned()
            .as_byte_slice()
            .len()
            .try_into()
            .ok()?;
        offset = offset.checked_add(len_u32)?;
    }
    vec.reserve(offset as usize);
    for element in elements {
        vec.extend(element.as_unaligned().as_byte_slice())
    }
    Some(vec)
}
