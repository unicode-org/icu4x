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
/// # Safety
///
/// The safety invariants of this function are:
/// - It must call `cb` (only once)
/// - The slices passed to `cb`, if concatenated, should be a valid instance of the `T` [`VarULE`] type
///   (i.e. if fed to [`VarULE::validate_byte_slice()`] they must produce a successful result)
/// - It must return the return value of `cb` to the caller
///
/// # Implementation example
///
/// For example, if your regular stack type is:
///
/// ```rust
///# use zerovec::ZeroVec;
///# use zerovec::varzerovec::encode::EncodeAsVarULE;
///# use zerovec::ule::*;
///
/// struct Foo<'a> {
///    field1: char,
///    field2: u32,
///    field3: ZeroVec<'a, u32>   
/// }
/// ```
///
/// then the ULE type will be implemented as follows:
///
/// ```rust
/// use zerovec::{ZeroVec, VarZeroVec};
/// use zerovec::varzerovec::encode::EncodeAsVarULE;
/// use zerovec::ule::*;
/// use core::mem;
///
///# struct Foo<'a> {
///#    field1: char,
///#    field2: u32,
///#    field3: ZeroVec<'a, u32>   
///# }
///
/// // must be repr(packed) for safety of VarULE!
/// #[repr(packed)]
/// struct FooULE {
///     field1: <char as AsULE>::ULE,   
///     field2: <u32 as AsULE>::ULE,
///     field3: [<u32 as AsULE>::ULE],
/// }
///
/// unsafe impl VarULE for FooULE {
///     type Error = &'static str; // use strings for simplicity
///     fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
///         <char as AsULE>::ULE::validate_byte_slice(&bytes[0..4]).map_err(|_| "validating char failed")?;
///         <char as AsULE>::ULE::validate_byte_slice(&bytes[4..8]).map_err(|_| "validating u32 failed")?;
///         let _ = ZeroVec::<u32>::parse_byte_slice(&bytes[8..]).map_err(|_| "validating ZeroVec failed")?;
///         Ok(())
///     }
///     unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
///         let ptr = bytes.as_ptr();
///         let len = bytes.len();
///         // subtract the length of the char and u32 to get the length of the array
///         let len_new = (len - 8) / 4;
///         // it's hard constructing custom DSTs, we fake a pointer/length construction
///         let fake_slice = core::ptr::slice_from_raw_parts(ptr as *const <u32 as AsULE>::ULE, len_new);
///         &*(fake_slice as *const Self)
///     }
/// }
///
/// unsafe impl EncodeAsVarULE<FooULE> for Foo<'_> {
///    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
///        cb(&[<char as AsULE>::ULE::as_byte_slice(&[self.field1.as_unaligned()]),
///             <u32 as AsULE>::ULE::as_byte_slice(&[self.field2.as_unaligned()]),
///             self.field3.as_bytes()])
///    }
/// }
///
/// fn main() {
///     let mut foos = vec![Foo {field1: 'u', field2: 983, field3: ZeroVec::clone_from_slice(&[1212,2309,500,7000])},
///                         Foo {field1: 'l', field2: 1010, field3: ZeroVec::clone_from_slice(&[1932, 0, 8888, 91237])}];
///
///     let vzv = VarZeroVec::from(&*foos);
///
///     assert_eq!(char::from_unaligned(&vzv.get(0).unwrap().field1), 'u');
///     assert_eq!(u32::from_unaligned(&vzv.get(0).unwrap().field2), 983);
///     assert_eq!(&vzv.get(0).unwrap().field3, ZeroVec::clone_from_slice(&[1212,2309,500,7000]).as_slice());
///
///     assert_eq!(char::from_unaligned(&vzv.get(1).unwrap().field1), 'l');
///     assert_eq!(u32::from_unaligned(&vzv.get(1).unwrap().field2), 1010);
///     assert_eq!(&vzv.get(1).unwrap().field3, ZeroVec::clone_from_slice(&[1932, 0, 8888, 91237]).as_slice());
/// }
/// ```
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
