// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types to help compose VarULE primitives.
//!
//! This module exports [`VarTuple`] and [`VarTupleULE`], which allow a single sized type and
//! a single unsized type to be stored together as a [`VarULE`].
//!
//! # Examples
//!
//! ```
//! use zerovec::ule::vartuple::{VarTuple, VarTupleULE};
//! use zerovec::VarZeroVec;
//!
//! struct Employee<'a> {
//!     id: u32,
//!     name: &'a str,
//! };
//!
//! let employees = [
//!     Employee {
//!         id: 12345,
//!         name: "Jane Doe",
//!     },
//!     Employee {
//!         id: 67890,
//!         name: "John Doe",
//!     },
//! ];
//!
//! let employees_as_var_tuples = employees
//!     .into_iter()
//!     .map(|x| VarTuple {
//!         sized: x.id,
//!         variable: x.name,
//!     })
//!     .collect::<Vec<_>>();
//!
//! let employees_vzv: VarZeroVec<VarTupleULE<u32, str>> =
//!     employees_as_var_tuples.as_slice().into();
//!
//! assert_eq!(employees_vzv.len(), 2);
//!
//! assert_eq!(employees_vzv.get(0).unwrap().sized.as_unsigned_int(), 12345);
//! assert_eq!(&employees_vzv.get(0).unwrap().variable, "Jane Doe");
//!
//! assert_eq!(employees_vzv.get(1).unwrap().sized.as_unsigned_int(), 67890);
//! assert_eq!(&employees_vzv.get(1).unwrap().variable, "John Doe");
//! ```

use core::mem::size_of;

use super::{AsULE, EncodeAsVarULE, UleError, VarULE, ULE};

/// A sized type that can be converted to a [`VarTupleULE`].
///
/// See the module for examples.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // well-defined type
pub struct VarTuple<A, B> {
    pub sized: A,
    pub variable: B,
}

/// A dynamically-sized type combining a sized and an unsized type.
///
/// See the module for examples.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // well-defined type
#[repr(C)]
pub struct VarTupleULE<A: AsULE, V: VarULE + ?Sized> {
    pub sized: A::ULE,
    pub variable: V,
}

// # Safety
//
// ## Representation
//
// The type `VarTupleULE` is align(1) because it is repr(C) and its fields
// are all align(1), since they are themselves ULE and VarULE, which have
// this same safety constraint. Further, there is no padding, because repr(C)
// does not add padding when all fields are align(1).
//
// <https://doc.rust-lang.org/reference/type-layout.html#the-c-representation>
//
// Pointers to `VarTupleULE` are fat pointers with metadata equal to the
// metadata of the inner DST field V.
//
// <https://doc.rust-lang.org/stable/std/ptr/trait.Pointee.html>
//
// ## Checklist
//
// Safety checklist for `VarULE`:
//
// 1. align(1): see "Representation" above.
// 2. No padding: see "Representation" above.
// 3. `validate_byte_slice` checks length and defers to the inner ULEs.
// 4. `validate_byte_slice` checks length and defers to the inner ULEs.
// 5. `from_byte_slice_unchecked` returns a fat pointer to the bytes.
// 6. All other methods are left at their default impl.
// 7. The two ULEs have byte equality, so this composition has byte equality.
unsafe impl<A, V> VarULE for VarTupleULE<A, V>
where
    A: AsULE + 'static,
    V: VarULE + ?Sized,
{
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), UleError> {
        // TODO: use split_first_chunk_mut in 1.77
        if bytes.len() < size_of::<A::ULE>() {
            return Err(UleError::length::<Self>(bytes.len()));
        }
        let (sized_chunk, variable_chunk) = bytes.split_at(size_of::<A::ULE>());
        A::ULE::validate_byte_slice(sized_chunk)?;
        V::validate_byte_slice(variable_chunk)?;
        Ok(())
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        #[allow(clippy::panic)] // panic is documented in function contract
        if bytes.len() < size_of::<A::ULE>() {
            panic!("from_byte_slice_unchecked called with short slice")
        }
        let (_sized_chunk, variable_chunk) = bytes.split_at(size_of::<A::ULE>());
        // Safety: variable_chunk is a valid V because of this function's precondition: bytes is a valid Self,
        // and a valid Self contains a valid V after the space needed for A::ULE.
        let variable_ref = V::from_byte_slice_unchecked(variable_chunk);
        let variable_ptr: *const V = variable_ref;

        // Safety: The DST of VarTupleULE is a pointer to the `sized` element and has a metadata
        // equal to the metadata of the `variable` field (see "Representation" comments on the impl).

        // We should use the pointer metadata APIs here when they are stable: https://github.com/rust-lang/rust/issues/81513
        // For now we rely on all DST metadata being a usize.

        // Extract metadata from V's DST
        // Rust doesn't know that `&V` is a fat pointer so we have to use transmute_copy
        assert_eq!(size_of::<*const V>(), size_of::<(*const u8, usize)>());
        // Safety: We have asserted that the transmute Src and Dst are the same size. Furthermore,
        // DST pointers are a pointer and usize length metadata
        let (_v_ptr, metadata) = core::mem::transmute_copy::<*const V, (*const u8, usize)>(&variable_ptr);

        // Construct a new DST with the same metadata as V
        assert_eq!(size_of::<*const Self>(), size_of::<(*const u8, usize)>());
        // Safety: Same as above but in the other direction.
        let composed_ptr = core::mem::transmute_copy::<(*const u8, usize), *const Self>(&(bytes.as_ptr(), metadata));
        &*(composed_ptr)
    }
}

// # Safety
//
// encode_var_ule_len: returns the length of the two ULEs together.
//
// encode_var_ule_write: writes bytes by deferring to the inner ULE impls.
unsafe impl<A, B, V> EncodeAsVarULE<VarTupleULE<A, V>> for VarTuple<A, B>
where
    A: AsULE + 'static,
    B: EncodeAsVarULE<V>,
    V: VarULE + ?Sized,
{
    fn encode_var_ule_as_slices<R>(&self, _: impl FnOnce(&[&[u8]]) -> R) -> R {
        // unnecessary if the other two are implemented
        unreachable!()
    }

    #[inline]
    fn encode_var_ule_len(&self) -> usize {
        size_of::<A::ULE>() + self.variable.encode_var_ule_len()
    }

    #[inline]
    fn encode_var_ule_write(&self, dst: &mut [u8]) {
        // TODO: use split_first_chunk_mut in 1.77
        let (sized_chunk, variable_chunk) = dst.split_at_mut(size_of::<A::ULE>());
        sized_chunk.clone_from_slice([self.sized.to_unaligned()].as_byte_slice());
        self.variable.encode_var_ule_write(variable_chunk);
    }
}

#[test]
fn test_simple() {
    let var_tuple = VarTuple {
        sized: 1500u16,
        variable: "hello",
    };
    let var_tuple_ule = super::encode_varule_to_box(&var_tuple);
    assert_eq!(var_tuple_ule.sized.as_unsigned_int(), 1500);
    assert_eq!(&var_tuple_ule.variable, "hello");
}

#[test]
fn test_nested() {
    use crate::{ZeroSlice, ZeroVec};
    let var_tuple = VarTuple {
        sized: 2000u16,
        variable: VarTuple {
            sized: '🦙',
            variable: ZeroVec::alloc_from_slice(&[b'I', b'C', b'U']),
        },
    };
    let var_tuple_ule = super::encode_varule_to_box(&var_tuple);
    assert_eq!(var_tuple_ule.sized.as_unsigned_int(), 2000u16);
    assert_eq!(var_tuple_ule.variable.sized.to_char(), '🦙');
    assert_eq!(
        &var_tuple_ule.variable.variable,
        ZeroSlice::from_ule_slice(&[b'I', b'C', b'U'])
    );
}
