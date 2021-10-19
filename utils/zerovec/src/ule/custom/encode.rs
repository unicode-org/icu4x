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
/// This function should be implemented by providing an encoded slice for each field
/// of the VarULE type to the callback, in order. For an implementation to be safe, the slices
/// to the callback must, when concatenated, be a valid instance of the VarULE type.
///
/// See the [module level documentation](crate::ule::custom) for examples.
///
/// A typical implementation will take each field in the order found in the [`VarULE`] type,
/// convert it to ULE, call [`ULE::as_byte_slice()`] on them, and pass the slices to `cb` in order.
/// A trailing [`ZeroVec`](crate::ZeroVec) or [`VarZeroVec`](crate::VarZeroVec) can have their underlying
/// byte representation passed through.
///
/// # Safety
///
/// The safety invariants of this function are:
/// - It must call `cb` (only once)
/// - The slices passed to `cb`, if concatenated, should be a valid instance of the `T` [`VarULE`] type
///   (i.e. if fed to [`VarULE::validate_byte_slice()`] they must produce a successful result)
/// - It must return the return value of `cb` to the caller
///
pub unsafe trait EncodeAsVarULE<T: VarULE + ?Sized> {
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R;
}

unsafe impl<T: VarULE + ?Sized> EncodeAsVarULE<T> for T {
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[T::as_byte_slice(self)])
    }
}

unsafe impl<T: VarULE + ?Sized> EncodeAsVarULE<T> for &'_ T {
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[T::as_byte_slice(self)])
    }
}

unsafe impl<T: VarULE + ?Sized> EncodeAsVarULE<T> for Box<T> {
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[T::as_byte_slice(&*self)])
    }
}

unsafe impl EncodeAsVarULE<str> for String {
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[self.as_bytes()])
    }
}

unsafe impl<T: ULE> EncodeAsVarULE<[T]> for Vec<T> {
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[<[T] as VarULE>::as_byte_slice(self)])
    }
}
