// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "serde")]
mod serde;

mod ule;

pub use ule::ZeroVecULE;

use crate::ule::*;
use alloc::vec::Vec;
use core::cmp::{Ord, Ordering, PartialOrd};
use core::fmt;
use core::iter::FromIterator;

/// A zero-copy vector for fixed-width types.
///
/// `ZeroVec<T>` is designed as a drop-in replacement for `Vec<T>` in situations where it is
/// desirable to borrow data from an unaligned byte slice, such as zero-copy deserialization.
///
/// `T` must implement [`AsULE`], which is auto-implemented for a number of built-in types,
/// including all fixed-width multibyte integers. For variable-width types like [`str`],
/// see [`VarZeroVec`](crate::VarZeroVec).
///
/// Typically, the zero-copy equivalent of a `Vec<T>` will simply be `ZeroVec<'a, T>`.
///
/// # How it Works
///
/// `ZeroVec<T>` represents a slice of `T` as a slice of `T::ULE`. The difference between `T` and
/// `T::ULE` is that `T::ULE` must be encoded in little-endian with 1-byte alignment. When accessing
/// items from `ZeroVec<T>`, we fetch the `T::ULE`, convert it on the fly to `T`, and return `T` by
/// value.
///
/// Benchmarks can be found in the project repository. We found that for common operations on small
/// and large vectors, `ZeroVec<T>` performs from slightly faster to 15% slower than `Vec<T>`.
/// However, the main performance improvement on `ZeroVec<T>` is when deserializing from a byte
/// array; `ZeroVec<T>` deserializes 80% faster than `Vec<T>` in Serde Bincode, and it does not
/// require any heap allocations.
///
/// # Safety
///
/// `ZeroVec<T>` contains no unsafe code. However, the conversion from `&[u8]` to `&[T::ULE]` may
/// be unsafe. For more information, see the [`ule`] module.
///
/// # Example
///
/// ```
/// use zerovec::ZeroVec;
///
/// // The little-endian bytes correspond to the numbers on the following line.
/// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
/// let nums: &[u16] = &[211, 281, 421, 461];
///
/// // Conversion from &[u8] to &[u16::ULE] is infallible.
/// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
///
/// assert!(matches!(zerovec, ZeroVec::Borrowed(_)));
/// assert_eq!(zerovec.get(2), Some(421));
/// assert_eq!(zerovec, nums);
/// ```
///
/// [`ule`]: crate::ule
#[non_exhaustive]
#[derive(Clone)]
pub enum ZeroVec<'a, T>
where
    T: AsULE + ?Sized,
{
    /// An owned `ZeroVec<T>`. This will typically be constructed by [`ZeroVec::alloc_from_slice()`]
    /// or by calling [`ZeroVec::to_mut()`]/[`ZeroVec::for_each_mut()`]/etc on [`ZeroVec::Borrowed`].
    Owned(Vec<T::ULE>),

    /// A borrowed `ZeroVec<T>`. This will typically be constructed by [`ZeroVec::parse_byte_slice()`],
    /// [`ZeroVec::from_slice()`], or deserializers capable of doing zero-copy deserialization.
    ///
    /// If you already have a slice of `[T::ULE]`s, you can directly construct one of these.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    /// use zerovec::ule::*;
    ///
    /// // The little-endian bytes correspond to the numbers on the following line.
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let nums: &[PlainOldULE<2>] = &[211_u16.as_unaligned(), 281_u16.as_unaligned(),
    ///                                 421_u16.as_unaligned(), 461_u16.as_unaligned()];
    ///
    /// let zerovec = ZeroVec::<u16>::Borrowed(nums);
    ///
    /// assert!(matches!(zerovec, ZeroVec::Borrowed(_)));
    /// assert_eq!(bytes, zerovec.as_bytes());
    /// ```
    Borrowed(&'a [T::ULE]),
}

impl<T> fmt::Debug for ZeroVec<'_, T>
where
    T: AsULE + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Owned(_) => write!(f, "ZeroVec::Owned({:?})", self.to_vec()),
            Self::Borrowed(_) => write!(f, "ZeroVec::Borrowed({:?})", self.to_vec()),
        }
    }
}

impl<T> Eq for ZeroVec<'_, T> where T: AsULE + Eq + ?Sized {}

impl<'a, 'b, T> PartialEq<ZeroVec<'b, T>> for ZeroVec<'a, T>
where
    T: AsULE + PartialEq + ?Sized,
{
    #[inline]
    fn eq(&self, other: &ZeroVec<'b, T>) -> bool {
        // Note: T implements PartialEq but not T::ULE
        self.iter().eq(other.iter())
    }
}

impl<T> PartialEq<&[T]> for ZeroVec<'_, T>
where
    T: AsULE + PartialEq + ?Sized,
{
    #[inline]
    fn eq(&self, other: &&[T]) -> bool {
        self.iter().eq(other.iter().copied())
    }
}

impl<'a, T: AsULE> Default for ZeroVec<'a, T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: AsULE + PartialOrd> PartialOrd for ZeroVec<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<'a, T: AsULE + Ord> Ord for ZeroVec<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<'a, T> ZeroVec<'a, T>
where
    T: AsULE + ?Sized,
{
    #[inline]
    /// Creates a new, empty `ZeroVec<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let zv: ZeroVec<u16> = ZeroVec::new();
    /// assert!(zv.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::Borrowed(&[])
    }

    /// Parses a `&[u8]` buffer into a `ZeroVec<T>`.
    ///
    /// This function is infallible for built-in integer types, but fallible for other types,
    /// such as `char`. For more information, see [`ULE::parse_byte_slice`].
    ///
    /// The bytes within the byte buffer must remain constant for the life of the ZeroVec.
    ///
    /// # Endianness
    ///
    /// The byte buffer must be encoded in little-endian, even if running in a big-endian
    /// environment. This ensures a consistent representation of data across platforms.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert!(matches!(zerovec, ZeroVec::Borrowed(_)));
    /// assert_eq!(zerovec.get(2), Some(421));
    /// ```
    pub fn parse_byte_slice(bytes: &'a [u8]) -> Result<Self, <<T as AsULE>::ULE as ULE>::Error> {
        let slice: &'a [T::ULE] = T::ULE::parse_byte_slice(bytes)?;
        Ok(Self::Borrowed(slice))
    }

    /// Returns a `ZeroVec<T>` as its underlying `&[u8]` byte buffer representation.
    ///
    /// Useful for serialization.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// // The little-endian bytes correspond to the numbers on the following line.
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let nums: &[u16] = &[211, 281, 421, 461];
    ///
    /// let zerovec = ZeroVec::alloc_from_slice(nums);
    ///
    /// assert_eq!(bytes, zerovec.as_bytes());
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        T::ULE::as_byte_slice(self.as_slice())
    }

    /// Dereferences this `ZeroVec<T>` as `&[T::ULE]`. Most other functions on `ZeroVec<T>` use
    /// this function as a building block.
    #[inline]
    pub fn as_slice(&self) -> &[T::ULE] {
        match self {
            Self::Owned(vec) => vec.as_slice(),
            Self::Borrowed(slice) => slice,
        }
    }

    /// Returns the number of elements in this `ZeroVec<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    /// use zerovec::ule::AsULE;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(4, zerovec.len());
    /// assert_eq!(
    ///     bytes.len(),
    ///     zerovec.len() * std::mem::size_of::<<u16 as AsULE>::ULE>()
    /// );
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Returns whether the vec is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    /// assert!(!zerovec.is_empty());
    ///
    /// let emptyvec: ZeroVec<u16> = ZeroVec::parse_byte_slice(&[]).expect("infallible");
    /// assert!(emptyvec.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    /// Converts a `ZeroVec<T>` into a `ZeroVec<u8>`, retaining the current ownership model.
    ///
    /// Note that the length of the ZeroVec may change.
    ///
    /// # Examples
    ///
    /// Convert a borrowed `ZeroVec`:
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    /// let zv_bytes = zerovec.into_bytes();
    ///
    /// assert!(matches!(zv_bytes, ZeroVec::Borrowed(_)));
    /// assert_eq!(zv_bytes.get(0), Some(0xD3));
    /// ```
    ///
    /// Convert an owned `ZeroVec`:
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let nums: &[u16] = &[211, 281, 421, 461];
    /// let zerovec = ZeroVec::alloc_from_slice(nums);
    /// let zv_bytes = zerovec.into_bytes();
    ///
    /// assert!(matches!(zv_bytes, ZeroVec::Owned(_)));
    /// assert_eq!(zv_bytes.get(0), Some(0xD3));
    /// ```
    pub fn into_bytes(self) -> ZeroVec<'a, u8> {
        match self {
            ZeroVec::Borrowed(slice) => {
                let bytes: &'a [u8] = T::ULE::as_byte_slice(slice);
                ZeroVec::Borrowed(bytes)
            }
            ZeroVec::Owned(vec) => {
                let bytes = Vec::from(T::ULE::as_byte_slice(&vec));
                ZeroVec::Owned(bytes)
            }
        }
    }

    /// Converts a `ZeroVec<T>` into a `ZeroVec<P>`, retaining the current ownership model.
    ///
    /// # Panics
    ///
    /// Panics if `T::ULE` and `P::ULE` are not the same size.
    ///
    /// # Examples
    ///
    /// Convert a borrowed `ZeroVec`:
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0x7F, 0xF3, 0x01, 0x00, 0x49, 0xF6, 0x01, 0x00];
    /// let zv_char: ZeroVec<char> = ZeroVec::parse_byte_slice(bytes)
    ///     .expect("valid code points");
    /// let zv_u32: ZeroVec<u32> = zv_char.try_into_converted()
    ///     .expect("infallible conversion");
    ///
    /// assert!(matches!(zv_u32, ZeroVec::Borrowed(_)));
    /// assert_eq!(zv_u32.get(0), Some(u32::from('🍿')));
    /// ```
    ///
    /// Convert an owned `ZeroVec`:
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let chars: &[char] = &['🍿', '🙉'];
    /// let zv_char = ZeroVec::alloc_from_slice(chars);
    /// let zv_u32: ZeroVec<u32> = zv_char.try_into_converted()
    ///     .expect("length is divisible");
    ///
    /// assert!(matches!(zv_u32, ZeroVec::Owned(_)));
    /// assert_eq!(zv_u32.get(0), Some(u32::from('🍿')));
    /// ```
    ///
    /// If the types are not the same size, we refuse to convert:
    ///
    /// ```should_panic
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0x7F, 0xF3, 0x01, 0x00, 0x49, 0xF6, 0x01, 0x00];
    /// let zv_char: ZeroVec<char> = ZeroVec::parse_byte_slice(bytes)
    ///     .expect("valid code points");
    ///
    /// // Panics! mem::size_of::<char::ULE> != mem::size_of::<u16::ULE>
    /// zv_char.try_into_converted::<u16>();
    /// ```
    ///
    /// Instead, convert to bytes and then parse:
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0x7F, 0xF3, 0x01, 0x00, 0x49, 0xF6, 0x01, 0x00];
    /// let zv_char: ZeroVec<char> = ZeroVec::parse_byte_slice(bytes)
    ///     .expect("valid code points");
    /// let zv_u16: ZeroVec<u16> = zv_char.into_bytes().try_into_parsed()
    ///     .expect("infallible");
    ///
    /// assert!(matches!(zv_u16, ZeroVec::Borrowed(_)));
    /// assert_eq!(zv_u16.get(0), Some(0xF37F));
    /// ```
    pub fn try_into_converted<P: AsULE>(self) -> Result<ZeroVec<'a, P>, <P::ULE as ULE>::Error> {
        assert_eq!(
            core::mem::size_of::<<T as AsULE>::ULE>(),
            core::mem::size_of::<<P as AsULE>::ULE>()
        );
        match self {
            ZeroVec::Borrowed(old_slice) => {
                let bytes: &'a [u8] = T::ULE::as_byte_slice(old_slice);
                let new_slice = P::ULE::parse_byte_slice(bytes)?;
                Ok(ZeroVec::Borrowed(new_slice))
            }
            ZeroVec::Owned(old_vec) => {
                let bytes: &[u8] = T::ULE::as_byte_slice(&old_vec);
                P::ULE::validate_byte_slice(bytes)?;
                // Feature "vec_into_raw_parts" is not yet stable (#65816). Polyfill:
                let (ptr, len, cap) = {
                    // Take ownership of the pointer
                    let mut v = core::mem::ManuallyDrop::new(old_vec);
                    // Fetch the pointer, length, and capacity
                    (v.as_mut_ptr(), v.len(), v.capacity())
                };
                // Safety checklist for Vec::from_raw_parts:
                // 1. ptr came from a Vec<T>
                // 2. P and T are asserted above to be the same size
                // 3. length is what it was before
                // 4. capacity is what it was before
                let new_vec = unsafe {
                    let ptr = ptr as *mut P::ULE;
                    Vec::from_raw_parts(ptr, len, cap)
                };
                Ok(ZeroVec::Owned(new_vec))
            }
        }
    }
}

impl<'a> ZeroVec<'a, u8> {
    /// Converts a `ZeroVec<u8>` into a `ZeroVec<T>`, retaining the current ownership model.
    ///
    /// Note that the length of the ZeroVec may change.
    ///
    /// # Examples
    ///
    /// Convert a borrowed `ZeroVec`:
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zv_bytes = ZeroVec::Borrowed(bytes);
    /// let zerovec: ZeroVec<u16> = zv_bytes.try_into_parsed().expect("infallible");
    ///
    /// assert!(matches!(zerovec, ZeroVec::Borrowed(_)));
    /// assert_eq!(zerovec.get(0), Some(211));
    /// ```
    ///
    /// Convert an owned `ZeroVec`:
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: Vec<u8> = vec![0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zv_bytes = ZeroVec::Owned(bytes);
    /// let zerovec: ZeroVec<u16> = zv_bytes.try_into_parsed().expect("infallible");
    ///
    /// assert!(matches!(zerovec, ZeroVec::Owned(_)));
    /// assert_eq!(zerovec.get(0), Some(211));
    /// ```
    pub fn try_into_parsed<T: AsULE>(self) -> Result<ZeroVec<'a, T>, <T::ULE as ULE>::Error> {
        match self {
            ZeroVec::Borrowed(bytes) => {
                let slice: &'a [T::ULE] = T::ULE::parse_byte_slice(bytes)?;
                Ok(ZeroVec::Borrowed(slice))
            }
            ZeroVec::Owned(vec) => {
                let slice = Vec::from(T::ULE::parse_byte_slice(&vec)?);
                Ok(ZeroVec::Owned(slice))
            }
        }
    }
}

impl<T> ZeroVec<'_, T>
where
    T: AsULE,
{
    /// Creates a `ZeroVec<T>` from a `&[T]` by allocating memory.
    ///
    /// This function results in an `Owned` instance of `ZeroVec<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// // The little-endian bytes correspond to the numbers on the following line.
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let nums: &[u16] = &[211, 281, 421, 461];
    ///
    /// let zerovec = ZeroVec::alloc_from_slice(nums);
    ///
    /// assert!(matches!(zerovec, ZeroVec::Owned(_)));
    /// assert_eq!(bytes, zerovec.as_bytes());
    /// ```
    #[inline]
    pub fn alloc_from_slice(other: &[T]) -> Self {
        Self::Owned(other.iter().copied().map(T::as_unaligned).collect())
    }

    /// Creates a `Vec<T>` from a `ZeroVec<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let nums: &[u16] = &[211, 281, 421, 461];
    /// let vec: Vec<u16> = ZeroVec::alloc_from_slice(nums).to_vec();
    ///
    /// assert_eq!(nums, vec.as_slice());
    /// ```
    #[inline]
    pub fn to_vec(&self) -> Vec<T> {
        self.as_slice()
            .iter()
            .copied()
            .map(T::from_unaligned)
            .collect()
    }
}

impl<'a, T> ZeroVec<'a, T>
where
    T: AsULE + SliceAsULE,
{
    /// Attempts to create a `ZeroVec<'a, T>` from a `&'a [T]` by borrowing the argument.
    ///
    /// If this is not possible, such as on a big-endian platform, `None` is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// // The little-endian bytes correspond to the numbers on the following line.
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let nums: &[u16] = &[211, 281, 421, 461];
    ///
    /// if let Some(zerovec) = ZeroVec::try_from_slice(nums) {
    ///     assert!(matches!(zerovec, ZeroVec::Borrowed(_)));
    ///     assert_eq!(bytes, zerovec.as_bytes());
    /// }
    /// ```
    #[inline]
    pub fn try_from_slice(slice: &'a [T]) -> Option<Self> {
        T::slice_as_unaligned(slice).map(|ule_slice| Self::Borrowed(ule_slice))
    }

    /// Creates a `ZeroVec<'a, T>` from a `&'a [T]`, either by borrowing the argument or by
    /// allocating a new vector.
    ///
    /// This is a cheap operation on little-endian platforms, falling back to a more expensive
    /// operation on big-endian platforms.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// // The little-endian bytes correspond to the numbers on the following line.
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let nums: &[u16] = &[211, 281, 421, 461];
    ///
    /// let zerovec = ZeroVec::from_slice(nums);
    ///
    /// // Note: zerovec could be either borrowed or owned.
    /// assert_eq!(bytes, zerovec.as_bytes());
    /// ```
    #[inline]
    pub fn from_slice(slice: &'a [T]) -> Self {
        Self::try_from_slice(slice).unwrap_or_else(|| Self::alloc_from_slice(slice))
    }
}

impl<T> ZeroVec<'_, T>
where
    T: AsULE,
{
    /// Gets the element at the specified index. Returns None if out of range.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.get(2), Some(421));
    /// assert_eq!(zerovec.get(4), None);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<T> {
        self.as_slice().get(index).copied().map(T::from_unaligned)
    }

    pub(crate) fn get_ule_ref(&self, index: usize) -> Option<&T::ULE> {
        self.as_slice().get(index)
    }

    /// Gets the first element. Returns None if empty.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.first(), Some(211));
    /// ```
    #[inline]
    pub fn first(&self) -> Option<T> {
        self.as_slice().first().copied().map(T::from_unaligned)
    }

    /// Gets the last element. Returns None if empty.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.last(), Some(461));
    /// ```
    #[inline]
    pub fn last(&self) -> Option<T> {
        self.as_slice().last().copied().map(T::from_unaligned)
    }

    /// Gets an iterator over the elements.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    /// let mut it = zerovec.iter();
    ///
    /// assert_eq!(it.next(), Some(211));
    /// assert_eq!(it.next(), Some(281));
    /// assert_eq!(it.next(), Some(421));
    /// assert_eq!(it.next(), Some(461));
    /// assert_eq!(it.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.as_slice().iter().copied().map(T::from_unaligned)
    }

    /// Mutates each element according to a given function, meant to be
    /// a more convenient version of calling `.iter_mut()` on
    /// [`ZeroVec::to_mut()`] which serves fewer use cases.
    ///
    /// This will convert the ZeroVec into an owned ZeroVec if not already the case.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    /// use zerovec::ule::AsULE;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let mut zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// zerovec.for_each_mut(|item| *item += 1);
    ///
    /// assert_eq!(zerovec.to_vec(), &[212, 282, 422, 462]);
    /// assert!(matches!(zerovec, ZeroVec::Owned(_)));
    /// ```
    #[inline]
    pub fn for_each_mut(&mut self, mut f: impl FnMut(&mut T)) {
        self.to_mut().iter_mut().for_each(|item| {
            let mut aligned = T::from_unaligned(*item);
            f(&mut aligned);
            *item = aligned.as_unaligned()
        });
    }

    /// Same as [`ZeroVec::for_each_mut()`], but bubbles up errors.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    /// use zerovec::ule::AsULE;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let mut zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// zerovec.try_for_each_mut(|item| {
    ///     *item = item.checked_add(1).ok_or(())?;
    ///     Ok(())   
    /// })?;
    ///
    /// assert_eq!(zerovec.to_vec(), &[212, 282, 422, 462]);
    /// assert!(matches!(zerovec, ZeroVec::Owned(_)));
    /// # Ok::<(), ()>(())
    /// ```
    #[inline]
    pub fn try_for_each_mut<E>(
        &mut self,
        mut f: impl FnMut(&mut T) -> Result<(), E>,
    ) -> Result<(), E> {
        self.to_mut().iter_mut().try_for_each(|item| {
            let mut aligned = T::from_unaligned(*item);
            f(&mut aligned)?;
            *item = aligned.as_unaligned();
            Ok(())
        })
    }

    /// Converts a borrowed ZeroVec to an owned ZeroVec. No-op if already owned.
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    /// assert!(matches!(zerovec, ZeroVec::Borrowed(_)));
    ///
    /// let owned = zerovec.into_owned();
    /// assert!(matches!(owned, ZeroVec::Owned(_)));
    /// ```
    pub fn into_owned(self) -> ZeroVec<'static, T> {
        match self {
            Self::Owned(vec) => ZeroVec::Owned(vec),
            Self::Borrowed(_) => {
                let vec: Vec<T::ULE> = self.iter().map(T::as_unaligned).collect();
                ZeroVec::Owned(vec)
            }
        }
    }

    /// Allows the ZeroVec to be mutated by converting it to an owned variant, and producing
    /// a mutable vector of ULEs.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use crate::zerovec::ule::AsULE;
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let mut zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    /// assert!(matches!(zerovec, ZeroVec::Borrowed(_)));
    ///
    /// zerovec.to_mut().push(12_u16.as_unaligned());
    /// assert!(matches!(zerovec, ZeroVec::Owned(_)));
    /// ```
    pub fn to_mut(&mut self) -> &mut Vec<T::ULE> {
        match self {
            ZeroVec::Owned(ref mut vec) => vec,
            ZeroVec::Borrowed(_) => {
                let vec: Vec<T::ULE> = self.iter().map(T::as_unaligned).collect();
                let new_self = ZeroVec::Owned(vec);
                *self = new_self;
                // recursion is limited since we are guaranteed to hit the Owned branch
                self.to_mut()
            }
        }
    }
}

impl<T> ZeroVec<'_, T>
where
    T: AsULE + Ord,
{
    /// Binary searches a sorted `ZeroVec<T>` for the given element. For more information, see
    /// the primitive function [`binary_search`].
    ///
    /// # Example
    ///
    /// ```
    /// use zerovec::ZeroVec;
    ///
    /// let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];
    /// let zerovec: ZeroVec<u16> = ZeroVec::parse_byte_slice(bytes).expect("infallible");
    ///
    /// assert_eq!(zerovec.binary_search(&281), Ok(1));
    /// assert_eq!(zerovec.binary_search(&282), Err(2));
    /// ```
    ///
    /// [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
    #[inline]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize> {
        self.as_slice()
            .binary_search_by(|probe| T::from_unaligned(*probe).cmp(x))
    }
}

impl<T: AsULE> FromIterator<T> for ZeroVec<'_, T> {
    /// Creates a [`ZeroVec::Owned`] from an iterator of values.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        ZeroVec::Owned(iter.into_iter().map(|t| t.as_unaligned()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::samples::*;

    #[test]
    fn test_get() {
        {
            let zerovec = ZeroVec::from_slice(TEST_SLICE);
            assert_eq!(zerovec.get(0), Some(TEST_SLICE[0]));
            assert_eq!(zerovec.get(1), Some(TEST_SLICE[1]));
            assert_eq!(zerovec.get(2), Some(TEST_SLICE[2]));
        }
        {
            let zerovec = ZeroVec::<u32>::parse_byte_slice(TEST_BUFFER_LE).unwrap();
            assert_eq!(zerovec.get(0), Some(TEST_SLICE[0]));
            assert_eq!(zerovec.get(1), Some(TEST_SLICE[1]));
            assert_eq!(zerovec.get(2), Some(TEST_SLICE[2]));
        }
    }

    #[test]
    fn test_binary_search() {
        {
            let zerovec = ZeroVec::from_slice(TEST_SLICE);
            assert_eq!(Ok(3), zerovec.binary_search(&0x0e0d0c));
            assert_eq!(Err(3), zerovec.binary_search(&0x0c0d0c));
        }
        {
            let zerovec = ZeroVec::<u32>::parse_byte_slice(TEST_BUFFER_LE).unwrap();
            assert_eq!(Ok(3), zerovec.binary_search(&0x0e0d0c));
            assert_eq!(Err(3), zerovec.binary_search(&0x0c0d0c));
        }
    }

    #[test]
    fn test_odd_alignment() {
        assert_eq!(
            Some(0x020100),
            ZeroVec::<u32>::parse_byte_slice(TEST_BUFFER_LE)
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x04000201),
            ZeroVec::<u32>::parse_byte_slice(&TEST_BUFFER_LE[1..77])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x05040002),
            ZeroVec::<u32>::parse_byte_slice(&TEST_BUFFER_LE[2..78])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x06050400),
            ZeroVec::<u32>::parse_byte_slice(&TEST_BUFFER_LE[3..79])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x060504),
            ZeroVec::<u32>::parse_byte_slice(&TEST_BUFFER_LE[4..])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x4e4d4c00),
            ZeroVec::<u32>::parse_byte_slice(&TEST_BUFFER_LE[75..79])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x4e4d4c00),
            ZeroVec::<u32>::parse_byte_slice(&TEST_BUFFER_LE[3..79])
                .unwrap()
                .get(18)
        );
        assert_eq!(
            Some(0x4e4d4c),
            ZeroVec::<u32>::parse_byte_slice(&TEST_BUFFER_LE[76..])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x4e4d4c),
            ZeroVec::<u32>::parse_byte_slice(TEST_BUFFER_LE)
                .unwrap()
                .get(19)
        );
        // TODO(#1144): Check for correct slice length in PlainOldULE
        // assert_eq!(
        //     None,
        //     ZeroVec::<u32>::parse_byte_slice(&TEST_BUFFER_LE[77..])
        //         .unwrap()
        //         .get(0)
        // );
        assert_eq!(
            None,
            ZeroVec::<u32>::parse_byte_slice(TEST_BUFFER_LE)
                .unwrap()
                .get(20)
        );
        assert_eq!(
            None,
            ZeroVec::<u32>::parse_byte_slice(&TEST_BUFFER_LE[3..79])
                .unwrap()
                .get(19)
        );
    }
}
