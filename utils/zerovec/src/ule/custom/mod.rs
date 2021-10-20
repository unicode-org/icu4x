// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains utilities that help define custom VarULE types,
//! especially those using complex custom dynamically sized types.
//!
//! # Example
//!
//! For example, if your regular stack type is:
//!
//! ```rust
//!# use zerovec::ZeroVec;
//!# use zerovec::ule::custom::EncodeAsVarULE;
//!# use zerovec::ule::*;
//!
//! struct Foo<'a> {
//!    field1: char,
//!    field2: u32,
//!    field3: ZeroVec<'a, u32>   
//! }
//! ```
//!
//! then the ULE type will be implemented as follows:
//!
//! ```rust
//! use zerovec::{ZeroVec, VarZeroVec};
//! use zerovec::ule::custom::EncodeAsVarULE;
//! use zerovec::ule::*;
//! use core::mem;
//!
//!# struct Foo<'a> {
//!#    field1: char,
//!#    field2: u32,
//!#    field3: ZeroVec<'a, u32>   
//!# }
//!
//! // Must be repr(packed) for safety of VarULE!
//! // Must also only contain ULE types
//! #[repr(packed)]
//! struct FooULE {
//!     field1: <char as AsULE>::ULE,   
//!     field2: <u32 as AsULE>::ULE,
//!     field3: [<u32 as AsULE>::ULE],
//! }
//!
//! unsafe impl VarULE for FooULE {
//!     type Error = &'static str; // use strings for simplicity
//!     fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
//!         // validate each field
//!         <char as AsULE>::ULE::validate_byte_slice(&bytes[0..4]).map_err(|_| "validating char failed")?;
//!         <char as AsULE>::ULE::validate_byte_slice(&bytes[4..8]).map_err(|_| "validating u32 failed")?;
//!         let _ = ZeroVec::<u32>::parse_byte_slice(&bytes[8..]).map_err(|_| "validating ZeroVec failed")?;
//!         Ok(())
//!     }
//!     unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
//!         let ptr = bytes.as_ptr();
//!         let len = bytes.len();
//!         // subtract the length of the char and u32 to get the length of the array
//!         let len_new = (len - 8) / 4;
//!         // it's hard constructing custom DSTs, we fake a pointer/length construction
//!         // eventually we can use the Pointer::Metadata APIs when they stabilize
//!         let fake_slice = core::ptr::slice_from_raw_parts(ptr as *const <u32 as AsULE>::ULE, len_new);
//!         &*(fake_slice as *const Self)
//!     }
//! }
//!
//! unsafe impl EncodeAsVarULE<FooULE> for Foo<'_> {
//!    fn encode_var_ule<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
//!        // take each field, convert to ULE byte slices, and pass them through
//!        cb(&[<char as AsULE>::ULE::as_byte_slice(&[self.field1.as_unaligned()]),
//!             <u32 as AsULE>::ULE::as_byte_slice(&[self.field2.as_unaligned()]),
//!             // the ZeroVec is already in the correct slice format
//!             self.field3.as_bytes()])
//!    }
//! }
//!
//! fn main() {
//!     let mut foos = vec![Foo {field1: 'u', field2: 983, field3: ZeroVec::clone_from_slice(&[1212,2309,500,7000])},
//!                         Foo {field1: 'l', field2: 1010, field3: ZeroVec::clone_from_slice(&[1932, 0, 8888, 91237])}];
//!
//!     let vzv = VarZeroVec::from(&*foos);
//!
//!     assert_eq!(char::from_unaligned(&vzv.get(0).unwrap().field1), 'u');
//!     assert_eq!(u32::from_unaligned(&vzv.get(0).unwrap().field2), 983);
//!     assert_eq!(&vzv.get(0).unwrap().field3, ZeroVec::clone_from_slice(&[1212,2309,500,7000]).as_slice());
//!
//!     assert_eq!(char::from_unaligned(&vzv.get(1).unwrap().field1), 'l');
//!     assert_eq!(u32::from_unaligned(&vzv.get(1).unwrap().field2), 1010);
//!     assert_eq!(&vzv.get(1).unwrap().field3, ZeroVec::clone_from_slice(&[1932, 0, 8888, 91237]).as_slice());
//! }
//! ```

mod encode;

pub use encode::EncodeAsVarULE;
