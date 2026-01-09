// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::{EncodeAsVarULE, UleError, VarULE, ULE};
use core::fmt;
use core::marker::PhantomData;
use core::ops::Deref;

/// A container for a [`VarULE`] with a fixed byte length.
///
/// This container may be useful if the length of your VarULE is known at compile-time.
///
/// # Examples
///
/// ```
/// use zerovec::ule::SizedVarULEBytes;
///
/// let container = SizedVarULEBytes::<13, str>::try_from_encodeable("hello, world!").unwrap();
///
/// assert_eq!(&*container, "hello, world!");
///
/// // Returns an error if the container is not the correct size:
/// SizedVarULEBytes::<20, str>::try_from_encodeable("hello, world!").unwrap_err();
/// ```
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SizedVarULEBytes<const N: usize, V: VarULE + ?Sized> {
    /// Invariant: The bytes MUST be a valid VarULE representation of `V`.
    bytes: [u8; N],
    _marker: PhantomData<V>,
}

impl<const N: usize, V: VarULE + ?Sized> SizedVarULEBytes<N, V> {
    /// Creates one of these from an [`EncodeAsVarULE`].
    ///
    /// Returns an error if the byte length in the container is not the correct length
    /// for the encodeable object.
    pub fn try_from_encodeable(input: impl EncodeAsVarULE<V>) -> Result<Self, UleError> {
        let len = input.encode_var_ule_len();
        if len != N {
            return Err(UleError::length::<V>(len));
        }
        let mut bytes = [0u8; N];
        input.encode_var_ule_write(&mut bytes);
        // Safety: the bytes were just written from an EncodeAsVarULE impl
        unsafe { Ok(Self::new_unchecked(bytes)) }
    }

    /// Creates one of these directly from bytes.
    ///
    /// # Safety
    ///
    /// The bytes MUST be a valid VarULE representation of `V`.
    pub const unsafe fn new_unchecked(bytes: [u8; N]) -> Self {
        Self {
            bytes,
            _marker: PhantomData,
        }
    }

    /// Returns the bytes backing this [`SizedVarULEBytes`], which are
    /// guaranteed to be a valid VarULE representation of `V`.
    pub const fn as_bytes(&self) -> &[u8; N] {
        &self.bytes
    }

    /// Returns the container as an instance of `V`.
    pub fn as_varule(&self) -> &V {
        debug_assert!(V::validate_bytes(&self.bytes).is_ok());
        // Safety: self.bytes are a valid VarULE representation of `V`.
        unsafe { V::from_bytes_unchecked(&self.bytes) }
    }
}

impl<const N: usize, V: VarULE + ?Sized> fmt::Debug for SizedVarULEBytes<N, V>
where
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_varule().fmt(f)
    }
}

impl<const N: usize, V: VarULE + ?Sized> AsRef<V> for SizedVarULEBytes<N, V> {
    fn as_ref(&self) -> &V {
        self.as_varule()
    }
}

impl<const N: usize, V: VarULE + ?Sized> Deref for SizedVarULEBytes<N, V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        self.as_varule()
    }
}

impl SizedVarULEBytes<0, str> {
    /// The empty string as a [`SizedVarULEBytes`].
    // Safety: the empty slice is a valid str
    pub const EMPTY_STR: Self = unsafe { Self::new_unchecked([]) };
}

impl<T: ULE> SizedVarULEBytes<0, [T]> {
    /// The empty slice as a [`SizedVarULEBytes`].
    // Safety: the empty slice is a valid str
    pub const EMPTY_SLICE: Self = unsafe { Self::new_unchecked([]) };
}
