// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::upper_case_acronyms)]
//! Traits over unaligned little-endian data (ULE, pronounced "yule").

mod chars;
mod plain;
mod string;
mod vec;

pub use chars::CharULE;
pub use plain::PlainOldULE;

use alloc::alloc::Layout;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use core::{fmt, mem, slice};

/// Fixed-width, byte-aligned data that can be cast to and from a little-endian byte slice.
///
/// "ULE" stands for "Unaligned little-endian"
///
/// # Safety
///
/// There must be no padding bytes involved in this type: [`Self::as_byte_slice()`] *must* return
/// a slice of initialized bytes provided that `Self` is initialized.
///
/// [`ULE::validate_byte_slice()`] *must* be implemented to validate a byte slice and return error
/// for the same slices as [`ULE::parse_byte_slice()`].
///
/// [`ULE::from_bytes_unchecked()`] *must* be implemented to return the same result as [`ULE::parse_byte_slice()`],
/// and both should return slices to the same region of memory.
///
/// [`ULE::as_byte_slice()`] should return a slice that is the in-memory representation of `Self`,
/// i.e. it should be just a pointer cast, and `mem::size_of_val(self) == mem::size_of_val(self.as_byte_slice())`=
///
/// # Equality invariant
///
/// A non-safety invariant is that if `Self` implements `PartialEq`, it *must* be logically equivalent to
/// byte equality on `.as_byte_slice()`. If `PartialEq` does not imply byte-for-byte equality, then
/// `.validate_byte_slice()` should return an error code for any values that are not in canonical form.
///
/// Failure to follow this invariant will cause surprising behavior in `PartialEq`, which may result in
/// unpredictable operations on `ZeroVec`, `VarZeroVec`, and `ZeroMap`.
pub unsafe trait ULE
where
    Self: Sized,
    Self: 'static,
{
    /// The error that occurs if a byte array is not valid for this ULE.
    type Error: fmt::Display;

    /// Validates a byte slice, `&[u8]`.
    ///
    /// If `Self` is not well-defined for all possible bit values, the bytes should be validated,
    /// that they can be transumted to a `Self` and `Self::Error` should be returned otherwise.
    fn validate_byte_slice(_bytes: &[u8]) -> Result<(), Self::Error>;

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
    ///
    /// Implementors must return a slice to the same region of memory if parsing succeeds.
    ///
    /// Ideally, implementations call [`ULE::from_byte_slice_unchecked()`] after validation.
    ///
    /// The default implementation executes `Self::validate_byte_slice` followed by
    /// `Self::from_byte_slice_unchecked`.
    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        Self::validate_byte_slice(bytes)?;
        Ok(unsafe { Self::from_byte_slice_unchecked(bytes) })
    }

    /// Takes a byte slice, `&[u8]`, and return it as `&[Self]` with the same lifetime, assuming that
    /// this byte slice has previously been run through [`ULE::parse_byte_slice()`] with success.
    ///
    /// There is no need to perform any validation here, this should almost always be a straight pointer
    /// cast.
    ///
    /// # Safety
    ///
    /// ## Callers
    /// Callers of this method must take care to ensure that `bytes` was previously passed through
    /// [`ULE::validate_byte_slice()`] with success (and was not changed since then).
    ///
    /// ## Implementors
    /// This method _must_ be implemented to return the same result as [`ULE::parse_byte_slice()`].
    ///
    /// Implementors must return a slice to the same region of memory. The default implementation
    /// does this directly.
    ///
    /// Implementations of this method may involve `unsafe{}` blocks to cast the pointer to the
    /// correct type. It is up to the implementation to reason about the safety, assuming the invariant
    /// above.
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &[Self] {
        let data = bytes.as_ptr();
        let len = bytes.len() / mem::size_of::<Self>();
        core::slice::from_raw_parts(data as *const Self, len)
    }

    /// Given `&[Self]`, returns a `&[u8]` with the same lifetime.
    ///
    /// # Safety
    ///
    /// The implementation of this function should involve re-casting the pointer.
    /// It is up to the implementation to reason about the safety. Keep in mind that `&[Self]` and
    /// `&[u8]` may have different lengths (but should cover the same data).
    ///
    /// The default implementation already does this, however it can be overridden with
    /// a fully-safe method if possible.
    #[inline]
    #[allow(clippy::wrong_self_convention)] // https://github.com/rust-lang/rust-clippy/issues/7219
    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        unsafe {
            slice::from_raw_parts(slice as *const [Self] as *const u8, mem::size_of_val(slice))
        }
    }
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

/// An [`EqULE`] type is one whose byte sequence equals the byte sequence of its ULE type on
/// little-endian platforms. This enables certain performance optimizations.
///
/// # Implementation safety
///
/// This trait is safe to implement if the type's ULE (as defined by `impl `[`AsULE`]` for T`)
/// has an equal byte sequence as the type itself on little-endian platforms; i.e., one where
/// `*const T` can be cast to a valid `*const T::ULE`.
pub unsafe trait EqULE: AsULE {}

/// A trait for a type where aligned slices can be cast to unaligned slices.
///
/// Auto-implemented on all types implementing [`EqULE`].
pub trait SliceAsULE
where
    Self: AsULE + Sized,
{
    /// Converts from `&[Self]` to `&[Self::ULE]` if possible.
    ///
    /// In general, this function returns `Some` on little-endian and `None` on big-endian.
    fn slice_as_unaligned(slice: &[Self]) -> Option<&[Self::ULE]>;
}

#[cfg(target_endian = "little")]
impl<T> SliceAsULE for T
where
    T: EqULE,
{
    #[inline]
    fn slice_as_unaligned(slice: &[Self]) -> Option<&[Self::ULE]> {
        // This is safe because on little-endian platforms, the byte sequence of &[T]
        // is equivalent to the byte sequence of &[T::ULE] by the contract of EqULE,
        // and &[T::ULE] has equal or looser alignment than &[T].
        let ule_slice =
            unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const Self::ULE, slice.len()) };
        Some(ule_slice)
    }
}

#[cfg(not(target_endian = "little"))]
impl<T> SliceAsULE for T
where
    T: EqULE,
{
    #[inline]
    fn slice_as_unaligned(_: &[Self]) -> Option<&[Self::ULE]> {
        None
    }
}

/// Variable-width, byte-aligned data that can be cast to and from a little-endian byte slice.
///
/// This trait is mostly for unsized types like `str` and `[T]`. It can be implemented on sized types,
/// however it is much more preferable to use [`ULE`] for that purpose.
///
/// If deserialization with `VarZeroVec` is desired is recommended to implement `Deserialize` for
/// `Box<T>` (serde does not do this automatically for unsized `T`).
///
/// # Safety
///
/// There must be no padding bytes involved in this type: [`Self::as_byte_slice()`] MUST return
/// a slice of initialized bytes provided that `Self` is initialized.
///
/// [`ULE::validate_byte_slice()`] *must* be implemented to validate a byte slice and return error
/// for the same slices as [`ULE::parse_byte_slice()`].
///
/// [`VarULE::from_byte_slice_unchecked()`] *must* be implemented to return the same result
/// as [`VarULE::parse_byte_slice()`] provided both are passed the same validly parsing byte slices.
/// Both should return a pointer to the same region of memory that was passed in, covering that region
/// completely.
///
/// [`VarULE::as_byte_slice()`] should return a slice that is the in-memory representation of `Self`,
/// i.e. it should be just a pointer cast, and `mem::size_of_val(self) == mem::size_of_val(self.as_byte_slice())`
///
/// # Equality invariant
///
/// A non-safety invariant is that if `Self` implements `PartialEq`, it *must* be logically equivalent to
/// byte equality on `.as_byte_slice()`. If `PartialEq` does not imply byte-for-byte equality, then
/// `.validate_byte_slice()` should return an error code for any values that are not in canonical form.
///
/// Failure to follow this invariant will cause surprising behavior in `PartialEq`, which may result in
/// unpredictable operations on `ZeroVec`, `VarZeroVec`, and `ZeroMap`.
pub unsafe trait VarULE: 'static {
    /// The error type to used by [`VarULE::parse_byte_slice()`]
    type Error: fmt::Display;

    /// Validates a byte slice, `&[u8]`.
    ///
    /// If `Self` is not well-defined for all possible bit values, the bytes should be validated,
    /// that they can be transumted to a `Self` and `Self::Error` should be returned otherwise.
    fn validate_byte_slice(_bytes: &[u8]) -> Result<(), Self::Error>;

    /// Parses a byte slice, `&[u8]`, and return it as `&Self` with the same lifetime.
    ///
    /// If `Self` is not well-defined for all possible bit values, the bytes should be validated,
    /// and `Self::Error` should be returned if they are not valid.
    ///
    /// # Safety
    ///
    /// Implementations of this method may involve `unsafe{}` blocks to cast the pointer to the
    /// correct type. It is up to the implementation to reason about the safety.
    ///
    /// Implementors must return a pointer to the same region of memory if parsing succeeds.
    ///
    /// The default implementation executes `Self::validate_byte_slice` followed by
    /// `Self::from_byte_slice_unchecked`.
    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        Self::validate_byte_slice(bytes)?;
        Ok(unsafe { Self::from_byte_slice_unchecked(bytes) })
    }

    /// Takes a byte slice, `&[u8]`, and return it as `&self` with the same lifetime, assuming that
    /// this byte slice has previously been run through [`VarULE::parse_byte_slice()`] with success.
    ///
    /// There is no need to perform any validation here, this should almost always be a straight pointer
    /// cast.
    ///
    /// # Safety
    ///
    /// ## Callers
    /// Callers of this method must take care to ensure that `bytes` was previously passed through
    /// [`VarULE::validate_byte_slice()`] with success (and was not changed since then).
    ///
    /// ## Implementors
    /// This method _must_ be implemented to return the same result as [`VarULE::parse_byte_slice()`].
    ///
    /// Implementors must return a pointer to the same region of memory.
    ///
    /// Implementations of this method may involve `unsafe{}` blocks to cast the pointer to the
    /// correct type. It is up to the implementation to reason about the safety, assuming the invariant
    /// above.
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self;

    /// Given `&Self`, returns a `&[u8]` with the same lifetime.
    ///
    /// # Safety
    ///
    /// The implementation of this function should involve re-casting the pointer.
    /// It is up to the implementation to reason about the safety.
    ///
    /// The default implementation already does this, however it can be overridden with
    /// a fully-safe method if possible.
    #[inline]
    fn as_byte_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, mem::size_of_val(self)) }
    }

    /// Allocate on the heap as a `Box<T>`
    #[inline]
    fn to_boxed(&self) -> Box<Self> {
        let bytesvec = self.as_byte_slice().to_owned().into_boxed_slice();
        unsafe {
            // Get the pointer representation
            let ptr: *mut Self =
                Self::from_byte_slice_unchecked(&bytesvec) as *const Self as *mut Self;
            assert_eq!(Layout::for_value(&*ptr), Layout::for_value(&*bytesvec));
            // Forget the allocation
            mem::forget(bytesvec);
            // Transmute the pointer to an owned pointer
            Box::from_raw(ptr)
        }
    }
}
