// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::upper_case_acronyms)]
//! Traits over unaligned little-endian data (ULE, pronounced "yule").

mod chars;
mod plain;
mod string;

pub use chars::CharULE;
pub use plain::PlainOldULE;

/// Fixed-width, byte-aligned data that can be cast to and from a little-endian byte slice.
///
/// "ULE" stands for "Unaligned little-endian"
pub trait ULE
where
    Self: Sized,
    Self: 'static,
{
    /// The error that occurs if a byte array is not valid for this ULE.
    type Error;

    /// Parses a byte slice, `&[u8]`, and return it as `&[Self]` with the same lifetime.
    ///
    /// If `Self` is not well-defined for all possible bit values, the bytes should be validated,
    /// and `Self::Error` should be returned if they are not valid.
    ///
    /// # Safety
    ///
    /// Implementations of this method may involve `unsafe{}` blocks to cast the pointer to the
    /// correct type. It is up to the implementation to reason about the safety. Keep in mind that
    /// `&[Self]` and `&[u8]` may have different lengths.
    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error>;

    /// Given `&[Self]`, returns a `&[u8]` with the same lifetime.
    ///
    /// # Safety
    ///
    /// In most cases, the implementation of this function should involve re-casting the pointer.
    /// It is up to the implementation to reason about the safety. Keep in mind that `&[Self]` and
    /// `&[u8]` may have different lengths.
    #[allow(clippy::wrong_self_convention)] // https://github.com/rust-lang/rust-clippy/issues/7219
    fn as_byte_slice(slice: &[Self]) -> &[u8];
}

/// A trait for any type that has a 1:1 mapping with an unaligned little-endian (ULE) type.
pub trait AsULE {
    /// The ULE type corresponding to `Self`.
    ///
    /// Types having infallible conversions from all bit values (Plain Old Data) can use
    /// `PlainOldULE` with the desired width; for example, `u32` uses `PlainOldULE<4>`.
    ///
    /// Types that are not well-defined for all bit values should implement a custom ULE.
    type ULE: ULE;

    /// Converts from `&Self` to `Self::ULE`.
    ///
    /// This function may involve byte order swapping (native-endian to little-endian).
    ///
    /// For best performance, mark your implementation of this function `#[inline]`.
    fn as_unaligned(&self) -> Self::ULE;

    /// Converts from `&Self::ULE` to `Self`.
    ///
    /// This function may involve byte order swapping (little-endian to native-endian).
    ///
    /// For best performance, mark your implementation of this function `#[inline]`.
    ///
    /// # Safety
    ///
    /// This function is infallible because bit validation should have occured when `Self::ULE`
    /// was first constructed. An implementation may therefore involve an `unsafe{}` block, like
    /// `from_bytes_unchecked()`.
    fn from_unaligned(unaligned: &Self::ULE) -> Self;
}

/// A trait for any type that has a 1:1 mapping with an variable-width unaligned little-endian (VarULE) type.
///
/// One such type is `String`, which can be handled as an [`str`], which has no alignment or endianness requirements.
pub trait AsVarULE {
    /// The VarULE type corresponding to `Self`.
    type VarULE: VarULE + ?Sized;

    /// Converts from `&Self` to `Self::ULE`.
    ///
    /// This function will almost always be a `Deref` or similar.
    ///
    /// For best performance, mark your implementation of this function `#[inline]`.
    fn as_unaligned(&self) -> &Self::VarULE;

    /// Converts from `&Self::ULE` to an owned `Self`.
    ///
    /// This function may involve allocation.
    ///
    /// For best performance, mark your implementation of this function `#[inline]`.
    ///
    /// # Safety
    ///
    /// This function is infallible because bit validation should have occured when `Self::ULE`
    /// was first constructed. An implementation may therefore involve an `unsafe{}` block, like
    /// `from_bytes_unchecked()`.
    fn from_unaligned(unaligned: &Self::VarULE) -> Self;
}

/// Variable-width, byte-aligned data that can be cast to and from a little-endian byte slice.
///
/// This trait is mostly for unsized types like `str` and `[T]`. It can be implemented on sized types,
/// however it is much more preferable to use [`ULE`] for that purpose.
pub trait VarULE: 'static {
    /// The error type to used by [`VarULE::parse_byte_slice()`]
    type Error;

    /// Parses a byte slice, `&[u8]`, and return it as `&self` with the same lifetime.
    ///
    /// If `Self` is not well-defined for all possible bit values, the bytes should be validated,
    /// and `Self::Error` should be returned if they are not valid.
    ///
    /// # Safety
    ///
    /// Implementations of this method may involve `unsafe{}` blocks to cast the pointer to the
    /// correct type. It is up to the implementation to reason about the safety.
    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error>;

    /// Takes a byte slice, `&[u8]`, and return it as `&self` with the same lifetime, assuming that
    /// this byte slice has previously been run through [`VarULE::parse_byte_slice()`] with success.
    ///
    /// There is no need to perform any validation here, this should almost always be a straight pointer
    /// cast.
    ///
    /// # Safety
    ///
    /// Callers of this method must take care to ensure that `bytes` was previously passed through
    /// [`VarULE::parse_byte_slice()`] with success (and was not changed since then).
    ///
    /// Implementations of this method may involve `unsafe{}` blocks to cast the pointer to the
    /// correct type. It is up to the implementation to reason about the safety, assuming the invariant
    /// above.
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self;

    /// Given `&Self`, returns a `&[u8]` with the same lifetime.
    ///
    /// # Safety
    ///
    /// In most cases, the implementation of this function should involve re-casting the pointer.
    /// It is up to the implementation to reason about the safety.
    fn as_byte_slice(&self) -> &[u8];
}
