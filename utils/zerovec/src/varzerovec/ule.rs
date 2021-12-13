// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::borrowed::VarZeroVecBorrowed;
use super::*;
use core::marker::PhantomData;
use core::mem;

/// A VarULE version of VarZeroVec allowing for VarZeroVecs to be nested indefinitely
///
/// ```rust
/// use zerovec::VarZeroVec;
/// use zerovec::ZeroVec;
/// use zerovec::varzerovec::VarZeroVecULE;
/// use zerovec::ule::*;
/// let strings_1: Vec<String> = vec!["foo".into(), "bar".into(), "baz".into()];
/// let strings_2: Vec<String> = vec!["twelve".into(), "seventeen".into(), "forty two".into()];
/// let strings_3: Vec<String> = vec!["我".into(), "喜歡".into(), "烏龍茶".into()];
/// let strings_4: Vec<String> = vec!["w".into(), "ω".into(), "文".into(), "𑄃".into()];
/// let strings_12 = vec![strings_1.clone(), strings_2.clone()];
/// let strings_34 = vec![strings_3.clone(), strings_4.clone()];
/// let all_strings = vec![strings_12, strings_34];
///
/// let vzv_1: VarZeroVec<str> = VarZeroVec::from(&*strings_1);
/// let vzv_2: VarZeroVec<str> = VarZeroVec::from(&*strings_2);
/// let vzv_3: VarZeroVec<str> = VarZeroVec::from(&*strings_3);
/// let vzv_4: VarZeroVec<str> = VarZeroVec::from(&*strings_4);
/// let vzv_12 = VarZeroVec::from(&[vzv_1.as_ule(), vzv_2.as_ule()] as &[_]);
/// let vzv_34 = VarZeroVec::from(&[vzv_3.as_ule(), vzv_4.as_ule()] as &[_]);
/// let vzv_all = VarZeroVec::from(&[vzv_12.as_ule(), vzv_34.as_ule()] as &[_]);
///
/// let reconstructed: Vec<Vec<Vec<String>>> = vzv_all.iter()
///        .map(|v: &VarZeroVecULE<VarZeroVecULE<str>>| {
///             v.iter().map(|x: &VarZeroVecULE<_>| x.as_varzerovec().iter().map(|s| s.to_owned()).collect::<Vec<String>>())
///              .collect::<Vec<_>>()
///         }).collect::<Vec<_>>();
/// assert_eq!(reconstructed, all_strings);
///
/// let bytes = vzv_all.get_encoded_slice();
/// let vzv_from_bytes: VarZeroVec<VarZeroVecULE<VarZeroVecULE<str>>> = VarZeroVec::parse_byte_slice(bytes).unwrap();
/// assert_eq!(vzv_from_bytes, vzv_all);
/// ```
//
// safety invariant: The slice MUST be one which parses to
// a valid VarZeroVecBorrowed<T>
#[repr(transparent)]
pub struct VarZeroVecULE<T: ?Sized> {
    marker: PhantomData<T>,
    /// The original slice this was constructed from
    entire_slice: [u8],
}

impl<T: VarULE + ?Sized> VarZeroVecULE<T> {
    /// Obtain a [`VarZeroVecBorrowed`] borrowing from the internal buffer
    #[inline]
    pub fn as_borrowed<'a>(&'a self) -> VarZeroVecBorrowed<'a, T> {
        unsafe {
            // safety: VarZeroVecULE is guaranteed to parse here
            VarZeroVecBorrowed::from_bytes_unchecked(&self.entire_slice)
        }
    }

    /// Get the number of elements in this vector
    pub fn len(&self) -> usize {
        self.as_borrowed().len()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.as_borrowed().is_empty()
    }

    /// Obtain an iterator over VarZeroVecULE's elements
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = &'b T> {
        self.as_borrowed().iter()
    }

    /// Get one of VarZeroVecULE's elements, returning None if the index is out of bounds
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.as_borrowed().get(idx)
    }

    /// Get this [`VarZeroVecULE`] as a borrowed [`VarZeroVec`]
    ///
    /// If you wish to repeatedly call methods on this [`VarZeroVecULE`],
    /// it is more efficient to perform this conversion first
    pub fn as_varzerovec<'a>(&'a self) -> VarZeroVec<'a, T> {
        self.as_borrowed().into()
    }
}

impl<T> VarZeroVecULE<T>
where
    T: VarULE,
    T: ?Sized,
    T: Ord,
{
    /// Binary searches a sorted `VarZeroVecULE<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`].
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize> {
        self.as_borrowed().binary_search(x)
    }
}

// Safety (based on the safety checklist on the VarULE trait):
//  1. VarZeroVecULE does not include any uninitialized or padding bytes (achieved by `#[repr(transparent)]` on a
//     `[u8]` slice which satisfies this invariant)
//  2. VarZeroVecULE is aligned to 1 byte (achieved by `#[repr(transparent)]` on a
//     `[u8]` slice which satisfies this invariant)
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid.
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. `as_byte_slice()` is equivalent to a regular transmute of the underlying data
//  7. VarZeroVecULE byte equality is semantic equality (relying on the guideline of the underlying VarULE type)
unsafe impl<T: VarULE + ?Sized + 'static> VarULE for VarZeroVecULE<T> {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ULEError> {
        let _: VarZeroVecBorrowed<T> = VarZeroVecBorrowed::parse_byte_slice(bytes)?;
        Ok(())
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // self is really just a wrapper around a byte slice
        mem::transmute(bytes)
    }

    fn as_byte_slice(&self) -> &[u8] {
        &self.entire_slice
    }
}

impl<T> PartialEq<VarZeroVecULE<T>> for VarZeroVecULE<T>
where
    T: VarULE,
    T: ?Sized,
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &VarZeroVecULE<T>) -> bool {
        // VarULE has an API guarantee that this is equivalent
        // to `T::VarULE::eq()`
        self.entire_slice.eq(&other.entire_slice)
    }
}

impl<T: VarULE + ?Sized> fmt::Debug for VarZeroVecULE<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: ?Sized> AsRef<VarZeroVecULE<T>> for VarZeroVecULE<T> {
    fn as_ref(&self) -> &VarZeroVecULE<T> {
        self
    }
}
