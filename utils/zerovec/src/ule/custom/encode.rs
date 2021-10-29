// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

/// Allows types to be encoded as VarULEs. This is highly useful for implementing VarULE on
/// custom DSTs where the type cannot be obtained as a reference to some other type.
///
/// [`Self::encode_var_ule_as_slices()`] should be implemented by providing an encoded slice for each field
/// of the VarULE type to the callback, in order. For an implementation to be safe, the slices
/// to the callback must, when concatenated, be a valid instance of the VarULE type.
///
/// See the [module level documentation](crate::ule::custom) for examples.
///
/// [`Self::encode_var_ule_as_slices()`] is only used to provide default implementations for [`Self::encode_var_ule_write()`]
/// and [`Self::encode_var_ule_len()`]. If you override the default implementations it is totally valid to
/// replace [`Self::encode_var_ule_as_slices()`]'s body with `unreachable!()`. This can be done for cases where
/// it is not possible to implement [`Self::encode_var_ule_as_slices()`] but the other methods still work.
///
/// A typical implementation will take each field in the order found in the [`VarULE`] type,
/// convert it to ULE, call [`ULE::as_byte_slice()`] on them, and pass the slices to `cb` in order.
/// A trailing [`ZeroVec`](crate::ZeroVec) or [`VarZeroVec`](crate::VarZeroVec) can have their underlying
/// byte representation passed through.
///
/// In case the compiler is not optimizing [`Self::encode_var_ule_len()`], it can be overridden. A typical
/// implementation will add up the sizes of each field on the [`VarULE`] type and then add in the byte length of the
/// dynamically-sized part.
///
/// # Safety
///
/// The safety invariants of [`Self::encode_var_ule_as_slices()`] are:
/// - It must call `cb` (only once)
/// - The slices passed to `cb`, if concatenated, should be a valid instance of the `T` [`VarULE`] type
///   (i.e. if fed to [`VarULE::validate_byte_slice()`] they must produce a successful result)
/// - It must return the return value of `cb` to the caller
///
/// One or more of [`Self::encode_var_ule_len()`] and [`Self::encode_var_ule_write()`] may be provided.
/// If both are, then `zerovec` code is guaranteed to not call [`Self::encode_var_ule_as_slices()`], and it may be replaced
/// with `unreachable!()`.
///
/// The safety invariants of [`Self::encode_var_ule_len()`] are:
/// - It must return the length of the corresponding VarULE type
///
/// The safety invariants of [`Self::encode_var_ule_write()`] are:
/// - The slice written to `dst` must be a valid instance of the `T` [`VarULE`] type
///
///
pub unsafe trait EncodeAsVarULE<T: VarULE + ?Sized> {
    /// Calls `cb` with a piecewise list of byte slices that when concatenated
    /// produce the memory pattern of the corresponding instance of `T`.
    ///
    /// Do not call this function directly; instead use the other two. Some implementors
    /// may define this function to panic.
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R;

    /// Return the length, in bytes, of the corresponding [`VarULE`] type
    fn encode_var_ule_len(&self) -> usize {
        self.encode_var_ule_as_slices(|slices| slices.iter().map(|s| s.len()).sum())
    }

    /// Write the corresponding [`VarULE`] type to the `dst` buffer. `dst` should
    /// be the size of [`Self::encode_var_ule_len()`]
    fn encode_var_ule_write(&self, mut dst: &mut [u8]) {
        debug_assert_eq!(self.encode_var_ule_len(), dst.len());
        self.encode_var_ule_as_slices(move |slices| {
            for slice in slices {
                dst[..slice.len()].copy_from_slice(slice);
                dst = &mut dst[slice.len()..];
            }
        });
    }
}

unsafe impl<T: VarULE + ?Sized> EncodeAsVarULE<T> for T {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[T::as_byte_slice(self)])
    }
}

unsafe impl<T: VarULE + ?Sized> EncodeAsVarULE<T> for &'_ T {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[T::as_byte_slice(self)])
    }
}

unsafe impl<T: VarULE + ?Sized> EncodeAsVarULE<T> for Box<T> {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[T::as_byte_slice(&*self)])
    }
}

unsafe impl EncodeAsVarULE<str> for String {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[self.as_bytes()])
    }
}

unsafe impl<T: ULE> EncodeAsVarULE<[T]> for Vec<T> {
    fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[<[T] as VarULE>::as_byte_slice(self)])
    }
}
