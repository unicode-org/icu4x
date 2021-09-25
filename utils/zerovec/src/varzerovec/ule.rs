// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::components::SliceComponents;
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
/// let vzv_1 = VarZeroVec::from(&*strings_1);
/// let vzv_2 = VarZeroVec::from(&*strings_2);
/// let vzv_3 = VarZeroVec::from(&*strings_3);
/// let vzv_4 = VarZeroVec::from(&*strings_4);
/// let vzv_12 = VarZeroVec::from(&[vzv_1, vzv_2] as &[_]);
/// let vzv_34 = VarZeroVec::from(&[vzv_3, vzv_4] as &[_]);
/// let vzv_all = VarZeroVec::from(&[vzv_12, vzv_34] as &[_]);
///
/// let reconstructed = vzv_all.iter()
///        .map(|v: &VarZeroVecULE<_>| {
///             v.iter().map(|x: &VarZeroVecULE<_>| x.as_varzerovec().to_vec()).collect::<Vec<_>>()
///         }).collect::<Vec<_>>();
/// assert_eq!(reconstructed, all_strings);
///
/// let bytes = vzv_all.get_encoded_slice();
/// let vzv_from_bytes: VarZeroVec<VarZeroVec<VarZeroVec<String>>> = VarZeroVec::parse_byte_slice(bytes).unwrap();
/// assert_eq!(vzv_from_bytes, vzv_all);
/// ```
//
// safety invariant: The slice MUST be one which parses to
// a valid SliceComponents<T>
#[repr(transparent)]
pub struct VarZeroVecULE<T> {
    marker: PhantomData<[T]>,
    /// The original slice this was constructed from
    entire_slice: [u8],
}

impl<T: AsVarULE> VarZeroVecULE<T> {
    #[inline]
    fn get_components<'a>(&'a self) -> SliceComponents<'a, T> {
        unsafe {
            // safety: VarZeroVecULE is guaranteed to parse here
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

    /// Obtain an iterator over VarZeroVecULE's elements
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = &'b T::VarULE> {
        self.get_components().iter()
    }

    /// Get one of VarZeroVecULE's elements, returning None if the index is out of bounds
    pub fn get(&self, idx: usize) -> Option<&T::VarULE> {
        self.get_components().get(idx)
    }

    /// Get this [`VarZeroVecULE`] as a borrowed [`VarZeroVec`]
    ///
    /// If you wish to repeatedly call methods on this [`VarZeroVecULE`],
    /// it is more efficient to perform this conversion first
    pub fn as_varzerovec<'a>(&'a self) -> VarZeroVec<'a, T> {
        VarZeroVec(VarZeroVecInner::Borrowed(self.get_components()))
    }
}

impl<T> VarZeroVecULE<T>
where
    T: AsVarULE,
    T::VarULE: Ord,
{
    /// Binary searches a sorted `VarZeroVecULE<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`].
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T::VarULE) -> Result<usize, usize> {
        self.get_components().binary_search(x)
    }
}
unsafe impl<T: AsVarULE + 'static> VarULE for VarZeroVecULE<T> {
    type Error = ParseErrorFor<T>;

    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        let _: SliceComponents<T> = SliceComponents::parse_byte_slice(bytes)?;
        unsafe { Ok(Self::from_byte_slice_unchecked(bytes)) }
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // self is really just a wrapper around a byte slice
        mem::transmute(bytes)
    }

    fn as_byte_slice(&self) -> &[u8] {
        &self.entire_slice
    }
}

impl<T> AsVarULE for VarZeroVec<'static, T>
where
    T: AsVarULE,
    T: Clone,
{
    type VarULE = VarZeroVecULE<T>;
    #[inline]
    fn as_unaligned(&self) -> &VarZeroVecULE<T> {
        let slice = self.get_encoded_slice();
        unsafe {
            // safety: the slice is known to come from a valid parsed VZV
            VarZeroVecULE::from_byte_slice_unchecked(slice)
        }
    }
    #[inline]
    fn from_unaligned(unaligned: &VarZeroVecULE<T>) -> Self {
        unaligned.as_varzerovec().into_owned()
    }
}

impl<T> PartialEq<VarZeroVecULE<T>> for VarZeroVecULE<T>
where
    T: AsVarULE,
    T::VarULE: PartialEq,
{
    #[inline]
    fn eq(&self, other: &VarZeroVecULE<T>) -> bool {
        // VarULE has an API guarantee that this is equivalent
        // to `T::VarULE::eq()`
        self.entire_slice.eq(&other.entire_slice)
    }
}

impl<T: AsVarULE> fmt::Debug for VarZeroVecULE<T>
where
    T::VarULE: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
