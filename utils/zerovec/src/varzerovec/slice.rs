// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::borrowed::VarZeroVecBorrowed;
use super::*;
use core::marker::PhantomData;
use core::mem;

/// A zero-copy "slice", that works for unsized types, i.e. the zero-copy version of `[T]`
/// where `T` is not `Sized`.
///
/// This behaves similarly to [`VarZeroVec<T>`], however [`VarZeroVec<T>`] is allowed to contain
/// owned data and as such is ideal for deserialization since most human readable
/// serialization formats cannot unconditionally deserialize zero-copy.
///
/// This type can be nested within [`VarZeroVec<T>`] to allow for multi-level nested `Vec`s, for
/// example the following code constructs the conceptual zero-copy equivalent of `Vec<Vec<Vec<str>>>`
///
/// ```rust
/// use zerovec::VarZeroVec;
/// use zerovec::ZeroVec;
/// use zerovec::varzerovec::VarZeroSlice;
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
/// let vzv_12 = VarZeroVec::from(&[vzv_1.as_slice(), vzv_2.as_slice()] as &[_]);
/// let vzv_34 = VarZeroVec::from(&[vzv_3.as_slice(), vzv_4.as_slice()] as &[_]);
/// let vzv_all = VarZeroVec::from(&[vzv_12.as_slice(), vzv_34.as_slice()] as &[_]);
///
/// let reconstructed: Vec<Vec<Vec<String>>> = vzv_all.iter()
///        .map(|v: &VarZeroSlice<VarZeroSlice<str>>| {
///             v.iter().map(|x: &VarZeroSlice<_>| x.as_varzerovec().iter().map(|s| s.to_owned()).collect::<Vec<String>>())
///              .collect::<Vec<_>>()
///         }).collect::<Vec<_>>();
/// assert_eq!(reconstructed, all_strings);
///
/// let bytes = vzv_all.get_encoded_slice();
/// let vzv_from_bytes: VarZeroVec<VarZeroSlice<VarZeroSlice<str>>> = VarZeroVec::parse_byte_slice(bytes).unwrap();
/// assert_eq!(vzv_from_bytes, vzv_all);
/// ```
//
// safety invariant: The slice MUST be one which parses to
// a valid VarZeroVecBorrowed<T>
#[repr(transparent)]
pub struct VarZeroSlice<T: ?Sized> {
    marker: PhantomData<T>,
    /// The original slice this was constructed from
    entire_slice: [u8],
}

impl<T: VarULE + ?Sized> VarZeroSlice<T> {
    /// Obtain a [`VarZeroVecBorrowed`] borrowing from the internal buffer
    #[inline]
    pub fn as_borrowed<'a>(&'a self) -> VarZeroVecBorrowed<'a, T> {
        unsafe {
            // safety: VarZeroSlice is guaranteed to parse here
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

    /// Obtain an iterator over VarZeroSlice's elements
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = &'b T> {
        self.as_borrowed().iter()
    }

    /// Get one of VarZeroSlice's elements, returning None if the index is out of bounds
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.as_borrowed().get(idx)
    }

    /// Get this [`VarZeroSlice`] as a borrowed [`VarZeroVec`]
    ///
    /// If you wish to repeatedly call methods on this [`VarZeroSlice`],
    /// it is more efficient to perform this conversion first
    pub fn as_varzerovec<'a>(&'a self) -> VarZeroVec<'a, T> {
        self.as_borrowed().into()
    }
}

impl<T> VarZeroSlice<T>
where
    T: VarULE,
    T: ?Sized,
    T: Ord,
{
    /// Binary searches a sorted `VarZeroSlice<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`].
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize> {
        self.as_borrowed().binary_search(x)
    }
}

// Safety (based on the safety checklist on the VarULE trait):
//  1. VarZeroSlice does not include any uninitialized or padding bytes (achieved by `#[repr(transparent)]` on a
//     `[u8]` slice which satisfies this invariant)
//  2. VarZeroSlice is aligned to 1 byte (achieved by `#[repr(transparent)]` on a
//     `[u8]` slice which satisfies this invariant)
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid.
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. `as_byte_slice()` is equivalent to a regular transmute of the underlying data
//  7. VarZeroSlice byte equality is semantic equality (relying on the guideline of the underlying VarULE type)
unsafe impl<T: VarULE + ?Sized + 'static> VarULE for VarZeroSlice<T> {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
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

impl<T> PartialEq<VarZeroSlice<T>> for VarZeroSlice<T>
where
    T: VarULE,
    T: ?Sized,
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &VarZeroSlice<T>) -> bool {
        // VarULE has an API guarantee that this is equivalent
        // to `T::VarULE::eq()`
        self.entire_slice.eq(&other.entire_slice)
    }
}

impl<T: VarULE + ?Sized> fmt::Debug for VarZeroSlice<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: ?Sized> AsRef<VarZeroSlice<T>> for VarZeroSlice<T> {
    fn as_ref(&self) -> &VarZeroSlice<T> {
        self
    }
}
