// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

/// Allows types to be encoded as VarULEs
///
/// This function should be implemented by providing an encoded slice for each field
/// of the VarULE type to the callback, in order. For an implementation to be safe, the slices
/// to the callback must, when concatenated, be a valid instance of the VarULE type.
///
/// For example, if your regular stack type
/// is:
///
/// ```rust, ignore
/// struct Foo<'a> {
///    field1: char,
///    field2: u32,
///    field3: ZeroVec<'a, u32>   
/// }
/// ```
///
/// and your VarULE type is:
///
/// ```rust, ignore
/// #[repr(packed)]
/// struct FooULE {
///     field1: <char as AsULE>:ULE,   
///     field2: <u32: as AsULE>:ULE,  
///     field3: [u32::ULE],
/// }
/// ```
///
/// then this trait may be implemented as
///
/// ```rust, ignore
/// impl EncodedAsVarULE for Foo<'_> {
///    type VarULE = FooULE;
///    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
///        cb(&[field1.as_unaligned().as_byte_slice(), field2.as_unaligned.as_byte_slice(), field3.as_slice()])
///    }
/// ```
pub unsafe trait EncodeAsVarULE {
    type VarULE: VarULE + ?Sized;
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R;
}

unsafe impl<T: VarULE + ?Sized> EncodeAsVarULE for &'_ T {
    type VarULE = T;
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[T::as_byte_slice(self)])
    }
}

unsafe impl<T: VarULE + ?Sized> EncodeAsVarULE for Box<T> {
    type VarULE = T;
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[T::as_byte_slice(&*self)])
    }
}

unsafe impl EncodeAsVarULE for String {
    type VarULE = str;
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[self.as_bytes()])
    }
}

unsafe impl<T: ULE> EncodeAsVarULE for Vec<T> {
    type VarULE = [T];
    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
        cb(&[<[T] as VarULE>::as_byte_slice(self)])
    }
}
