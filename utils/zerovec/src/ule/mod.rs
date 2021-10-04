// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::upper_case_acronyms)]
//! Traits over unaligned little-endian data (ULE, pronounced "yule").

mod chars;
mod error;
mod plain;
mod string;
mod vec;

pub use chars::CharULE;
pub use error::ULEError;
pub use plain::PlainOldULE;

use alloc::alloc::Layout;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use core::{fmt, mem, slice};

/// Fixed-width, byte-aligned data that can be cast to and from a little-endian byte slice.
///
/// Types that are not fixed-width can implement [`VarULE`] instead.
///
/// "ULE" stands for "Unaligned little-endian"
///
/// # Safety
///
/// Safety checklist for `ULE`:
///
/// 1. The type *must not* include any uninitialized or padding bytes.
/// 2. The impl of [`ULE::validate_byte_slice()`] *must* return an error if the given byte slice
///    would not represent a valid slice of this type.
/// 3. The impl of [`ULE::validate_byte_slice()`] *must* return an error if the given byte slice
///    cannot be used in its entirety (if its length is not a multiple of `size_of::<Self>()`).
/// 4. All other methods *must* be left with their default impl, or else implemented according to
///    their respective safety guidelines.
/// 5. Acknowledge the following note about the equality invariant.
///
/// # Equality invariant
///
/// A non-safety invariant is that if `Self` implements `PartialEq`, the it *must* be logically
/// equivalent to byte equality on [`Self::as_byte_slice()`].
///
/// It may be necessary to introduce a "canonical form" of the ULE if logical equality does not
/// equal byte equality. In such a case, [`Self::validate_byte_slice()`] should return an error
/// for any values that are not in canonical form. For example, the decimal strings "1.23e4" and
/// "12.3e3" are logically equal, but not byte-for-byte equal, so we could define a canonical form
/// where only a single digit is allowed before `.`.
///
/// Failure to follow this invariant will cause surprising behavior in `PartialEq`, which may
/// result in unpredictable operations on `ZeroVec`, `VarZeroVec`, and `ZeroMap`.
pub unsafe trait ULE
where
    Self: Sized,
    Self: 'static,
{
    /// The error that occurs if a byte array is not valid for this ULE.
    type Error: fmt::Display;

    /// Validates a byte slice, `&[u8]`.
    ///
    /// If `Self` is not well-defined for all possible bit values, the bytes should be validated.
    /// If the bytes can be transmuted, *in their entirety*, to a valid slice of `Self`, then `Ok`
    /// should be returned; otherwise, `Self::Error` should be returned.
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error>;

    /// Parses a byte slice, `&[u8]`, and return it as `&[Self]` with the same lifetime.
    ///
    /// If `Self` is not well-defined for all possible bit values, the bytes should be validated,
    /// and an error should be returned in the same cases as [`Self::validate_byte_slice()`].
    ///
    /// The default implementation executes [`Self::validate_byte_slice()`] followed by
    /// [`Self::from_byte_slice_unchecked`].
    ///
    /// Note: The following equality should hold: `bytes.len() % size_of::<Self>() == 0`. This
    /// means that the returned slice can span the entire byte slice.
    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        Self::validate_byte_slice(bytes)?;
        debug_assert_eq!(bytes.len() % mem::size_of::<Self>(), 0);
        Ok(unsafe { Self::from_byte_slice_unchecked(bytes) })
    }

    /// Takes a byte slice, `&[u8]`, and return it as `&[Self]` with the same lifetime, assuming
    /// that this byte slice has previously been run through [`Self::parse_byte_slice()`] with
    /// success.
    ///
    /// The default implementation performs a pointer cast to the same region of memory.
    ///
    /// # Safety
    ///
    /// ## Callers
    ///
    /// Callers of this method must take care to ensure that `bytes` was previously passed through
    /// [`Self::validate_byte_slice()`] with success (and was not changed since then).
    ///
    /// ## Implementors
    ///
    /// Implementations of this method may call unsafe functions to cast the pointer to the correct
    /// type, assuming the "Callers" invariant above.
    ///
    /// Keep in mind that `&[Self]` and `&[u8]` may have different lengths.
    ///
    /// Safety checklist:
    ///
    /// 1. This method *must* return the same result as [`Self::parse_byte_slice()`].
    /// 2. This method *must* return a slice to the same region of memory as the argument.
    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &[Self] {
        let data = bytes.as_ptr();
        let len = bytes.len() / mem::size_of::<Self>();
        debug_assert_eq!(bytes.len() % mem::size_of::<Self>(), 0);
        core::slice::from_raw_parts(data as *const Self, len)
    }

    /// Given `&[Self]`, returns a `&[u8]` with the same lifetime.
    ///
    /// The default implementation performs a pointer cast to the same region of memory.
    ///
    /// # Safety
    ///
    /// Implementations of this method should call potentially unsafe functions to cast the
    /// pointer to the correct type.
    ///
    /// Keep in mind that `&[Self]` and `&[u8]` may have different lengths.
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
    /// This function is infallible because bit validation should have occurred when `Self::ULE`
    /// was first constructed. An implementation may therefore involve an `unsafe{}` block, like
    /// `from_bytes_unchecked()`.
    fn from_unaligned(unaligned: &Self::ULE) -> Self;
}

/// An [`EqULE`] type is one whose byte sequence equals the byte sequence of its ULE type on
/// little-endian platforms. This enables certain performance optimizations, such as
/// [`ZeroVec::try_from_slice`](crate::ZeroVec::try_from_slice).
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
/// This trait is mostly for unsized types like `str` and `[T]`. It can be implemented on sized types;
/// however, it is much more preferable to use [`ULE`] for that purpose.
///
/// If deserialization with `VarZeroVec` is desired is recommended to implement `Deserialize` for
/// `Box<T>` (serde does not do this automatically for unsized `T`).
///
/// # Safety
///
/// Safety checklist for `VarULE`:
///
/// 1. The type *must not* include any uninitialized or padding bytes.
/// 2. The impl of [`ULE::validate_byte_slice()`] *must* return an error if the given byte slice
///    would not represent a valid slice of this type.
/// 3. The impl of [`ULE::validate_byte_slice()`] *must* return an error if the given byte slice
///    cannot be used in its entirety.
/// 4. All other methods *must* be left with their default impl, or else implemented according to
///    their respective safety guidelines.
/// 5. Acknowledge the following note about the equality invariant.
///
/// # Equality invariant
///
/// A non-safety invariant is that if `Self` implements `PartialEq`, the it *must* be logically
/// equivalent to byte equality on [`Self::as_byte_slice()`].
///
/// It may be necessary to introduce a "canonical form" of the ULE if logical equality does not
/// equal byte equality. In such a case, [`Self::validate_byte_slice()`] should return an error
/// for any values that are not in canonical form. For example, the decimal strings "1.23e4" and
/// "12.3e3" are logically equal, but not byte-for-byte equal, so we could define a canonical form
/// where only a single digit is allowed before `.`.
///
/// Failure to follow this invariant will cause surprising behavior in `PartialEq`, which may
/// result in unpredictable operations on `ZeroVec`, `VarZeroVec`, and `ZeroMap`.
pub unsafe trait VarULE: 'static {
    /// The error that occurs if a byte array is not valid for this ULE.
    type Error: fmt::Display;

    /// Validates a byte slice, `&[u8]`.
    ///
    /// If `Self` is not well-defined for all possible bit values, the bytes should be validated.
    /// If the bytes can be transmuted, *in their entirety*, to a valid `&Self`, then `Ok` should
    /// be returned; otherwise, `Self::Error` should be returned.
    fn validate_byte_slice(_bytes: &[u8]) -> Result<(), Self::Error>;

    /// Parses a byte slice, `&[u8]`, and return it as `&Self` with the same lifetime.
    ///
    /// If `Self` is not well-defined for all possible bit values, the bytes should be validated,
    /// and an error should be returned in the same cases as [`Self::validate_byte_slice()`].
    ///
    /// The default implementation executes [`Self::validate_byte_slice()`] followed by
    /// [`Self::from_byte_slice_unchecked`].
    ///
    /// Note: The following equality should hold: `size_of_val(result) == size_of_val(bytes)`,
    /// where `result` is the successful return value of the method. This means that the return
    /// value spans the entire byte slice.
    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        Self::validate_byte_slice(bytes)?;
        let result = unsafe { Self::from_byte_slice_unchecked(bytes) };
        debug_assert_eq!(mem::size_of_val(result), mem::size_of_val(bytes));
        Ok(result)
    }

    /// Takes a byte slice, `&[u8]`, and return it as `&Self` with the same lifetime, assuming
    /// that this byte slice has previously been run through [`Self::parse_byte_slice()`] with
    /// success.
    ///
    /// # Safety
    ///
    /// ## Callers
    ///
    /// Callers of this method must take care to ensure that `bytes` was previously passed through
    /// [`Self::validate_byte_slice()`] with success (and was not changed since then).
    ///
    /// ## Implementors
    ///
    /// Implementations of this method may call unsafe functions to cast the pointer to the correct
    /// type, assuming the "Callers" invariant above.
    ///
    /// Safety checklist:
    ///
    /// 1. This method *must* return the same result as [`Self::parse_byte_slice()`].
    /// 2. This method *must* return a slice to the same region of memory as the argument.
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self;

    /// Given `&Self`, returns a `&[u8]` with the same lifetime.
    ///
    /// The default implementation performs a pointer cast to the same region of memory.
    ///
    /// # Safety
    ///
    /// Implementations of this method should call potentially unsafe functions to cast the
    /// pointer to the correct type.
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
