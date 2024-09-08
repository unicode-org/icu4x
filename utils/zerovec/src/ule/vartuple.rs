// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types to help compose VarULE primitives.
//!
//! This module exports:
//!
//! - [`VarTuple`] and [`VarTupleULE`] for combining a [`ULE`] and a [`VarULE`]
//! - [`TinyVarVar`] and [`TinyVarVarULE`] for combining a short [`VarULE`] and a [`VarULE`]

use core::marker::PhantomData;
use core::mem::{size_of, transmute, transmute_copy};

use super::{AsULE, EncodeAsVarULE, UleError, VarULE, ULE};

/// A sized type that can be converted to a [`VarTupleULE`].
///
/// # Examples
///
/// ```
/// use zerovec::ule::vartuple::{VarTuple, VarTupleULE};
/// use zerovec::VarZeroVec;
///
/// struct Employee<'a> {
///     id: u32,
///     name: &'a str,
/// };
///
/// let employees = [
///     Employee {
///         id: 12345,
///         name: "Jane Doe",
///     },
///     Employee {
///         id: 67890,
///         name: "John Doe",
///     },
/// ];
///
/// let employees_as_var_tuples = employees
///     .into_iter()
///     .map(|x| VarTuple {
///         sized: x.id,
///         variable: x.name,
///     })
///     .collect::<Vec<_>>();
///
/// let employees_vzv: VarZeroVec<VarTupleULE<u32, str>> =
///     employees_as_var_tuples.as_slice().into();
///
/// assert_eq!(employees_vzv.len(), 2);
///
/// assert_eq!(employees_vzv.get(0).unwrap().sized.as_unsigned_int(), 12345);
/// assert_eq!(&employees_vzv.get(0).unwrap().variable, "Jane Doe");
///
/// assert_eq!(employees_vzv.get(1).unwrap().sized.as_unsigned_int(), 67890);
/// assert_eq!(&employees_vzv.get(1).unwrap().variable, "John Doe");
/// ```
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // well-defined type
pub struct VarTuple<A, B> {
    pub sized: A,
    pub variable: B,
}

/// A dynamically-sized type combining a sized and an unsized type.
///
/// See [`VarTuple`] for examples.
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
        let (_v_ptr, metadata) = transmute_copy::<*const V, (*const u8, usize)>(&variable_ptr);

        // Construct a new DST with the same metadata as V
        assert_eq!(size_of::<*const Self>(), size_of::<(*const u8, usize)>());
        // Safety: Same as above but in the other direction.
        let composed_ptr =
            transmute_copy::<(*const u8, usize), *const Self>(&(bytes.as_ptr(), metadata));
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
fn test_simple_vartuple() {
    let var_tuple = VarTuple {
        sized: 1500u16,
        variable: "hello",
    };
    let var_tuple_ule = super::encode_varule_to_box(&var_tuple);
    assert_eq!(var_tuple_ule.sized.as_unsigned_int(), 1500);
    assert_eq!(&var_tuple_ule.variable, "hello");
}

#[test]
fn test_nested_vartuple() {
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

#[test]
fn test_vartuple_validation() {
    let invalid0 = VarTuple {
        sized: [0x80, 0x80, 0x80],
        variable: "hello",
    };
    let invalid0_ule: Box<VarTupleULE<[u8; 3], str>> = super::encode_varule_to_box(&invalid0);
    assert!(VarTupleULE::<char, str>::parse_byte_slice(invalid0_ule.as_byte_slice()).is_err());

    let invalid1 = VarTuple {
        sized: '🦙',
        variable: b"\x80invalid".as_slice(),
    };
    let invalid1_ule: Box<VarTupleULE<char, [u8]>> = super::encode_varule_to_box(&invalid1);
    assert!(VarTupleULE::<char, str>::parse_byte_slice(invalid1_ule.as_byte_slice()).is_err());
}

/// Errors associated with [`TinyVarVar`].
#[derive(Debug)]
#[non_exhaustive]
pub enum TinyVarVarError {
    /// The value is too long to encode.
    TooLong,
}

/// A sized type combining two types that can be converted to a [`TinyVarVarULE`].
///
/// The first of the two types is expected to be short (no longer than 127 bytes).
/// There is no limit to the length of the second type.
///
/// # Examples
///
/// Store a default string and an array of other strings:
///
/// ```
/// use zerovec::ule::vartuple::{TinyVarVar, TinyVarVarULE};
/// use zerovec::{VarZeroVec, VarZeroSlice};
///
/// let homophones = [
///     ("cense", &["cents", "scents", "sense"][..]),
///     ("peak", &["peek", "peke", "pique"][..]),
///     ("poor", &["pore", "pour"][..]),
///     ("rain", &["reign", "rein"][..]),
/// ];
///
/// let homophones_tvv = homophones
///     .iter()
///     .map(|(word, extras)| TinyVarVar::try_new(*word, *extras).unwrap())
///     .collect::<Vec<_>>();
///
/// let homophones_vzv = VarZeroVec::<TinyVarVarULE<str, VarZeroSlice<str>>>::from(&homophones_tvv);
///
/// assert_eq!(homophones_vzv.len(), 4);
/// assert_eq!(homophones_vzv.get(1).unwrap().get().0, "peak");
/// assert_eq!(homophones_vzv.get(1).unwrap().get().1.get(0), Some("peek"));
/// ```
#[derive(Debug)]
pub struct TinyVarVar<T0, T1, V0: VarULE + ?Sized, V1: VarULE + ?Sized>(
    // Invariant: T0 encoded as V0 has a length that fits in 7 bits
    T0,
    T1,
    PhantomData<V0>,
    PhantomData<V1>,
);

/// A dynamically-sized VarULE type combining two dynamically-sized VarULE types.
///
/// The first of the two types is expected to be short (no longer than 127 bytes).
/// There is no limit to the length of the second type.
///
/// See [`TinyVarVar`] for examples.
#[derive(Debug)]
#[repr(transparent)]
pub struct TinyVarVarULE<V0, V1>
where
    V0: VarULE + ?Sized,
    V1: VarULE + ?Sized,
{
    _v0: PhantomData<V0>,
    _v1: PhantomData<V1>,
    /// Invariants:
    /// - the slice begins with a byte indicating the cutoff between V0 and V1
    /// - the slice from 1 to (1+cutoff) is a valid V0
    /// - the slice from (1+cutoff) to the end is a valid V1
    bytes: [u8],
}

// # Safety
//
// ## Representation
//
// The type `TinyVarVarULE` is repr(transparent) over `[u8]`, so it align(1),
// has no padding, and pointers to it are fat pointers.
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
unsafe impl<V0, V1> VarULE for TinyVarVarULE<V0, V1>
where
    V0: VarULE + ?Sized,
    V1: VarULE + ?Sized,
{
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), UleError> {
        let (v0_bytes, v1_bytes) = Self::bytes_as_parts(bytes)?;
        V0::validate_byte_slice(v0_bytes)?;
        V1::validate_byte_slice(v1_bytes)?;
        Ok(())
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // Safety: Self is a transparent wrapper over the bytes
        transmute(bytes)
    }
}

impl<V0, V1> TinyVarVarULE<V0, V1>
where
    V0: VarULE + ?Sized,
    V1: VarULE + ?Sized,
{
    /// Gets references to the two contained values.
    pub fn get(&self) -> (&V0, &V1) {
        let (v0_bytes, v1_bytes) = match Self::bytes_as_parts(&self.bytes) {
            Ok(tpl) => tpl,
            Err(_) => {
                debug_assert!(false);
                // Safety: the byte structure is valid by the invariant.
                unsafe { core::hint::unreachable_unchecked() }
            }
        };
        // Safety: v0_bytes and v1_bytes are valid slices of V0 and V1 by the invariant.
        unsafe {
            (
                V0::from_byte_slice_unchecked(v0_bytes),
                V1::from_byte_slice_unchecked(v1_bytes),
            )
        }
    }

    #[inline]
    fn bytes_as_parts(bytes: &[u8]) -> Result<(&[u8], &[u8]), UleError> {
        let (cutoff_byte, remainder) = bytes
            .split_first()
            .ok_or_else(|| UleError::length::<Self>(bytes.len()))?;
        let cutoff = *cutoff_byte as usize;
        // TODO (1.80): use split_at_checked
        if remainder.len() < cutoff {
            return Err(UleError::length::<Self>(bytes.len()));
        }
        Ok(remainder.split_at(cutoff))
    }
}

impl<T0, T1, V0, V1> TinyVarVar<T0, T1, V0, V1>
where
    T0: EncodeAsVarULE<V0>,
    T1: EncodeAsVarULE<V1>,
    V0: VarULE + ?Sized,
    V1: VarULE + ?Sized,
{
    /// Creates a new [`TinyVarVar`] that can be converted to a [`TinyVarVarULE`].
    ///
    /// Returns an error if the first element is too long.
    pub fn try_new(t0: T0, t1: T1) -> Result<Self, TinyVarVarError> {
        if t0.encode_var_ule_len() > 0xFF {
            return Err(TinyVarVarError::TooLong);
        }
        Ok(Self(t0, t1, PhantomData, PhantomData))
    }
}

// # Safety
//
// encode_var_ule_len: returns the length of the two ULEs together plus 1 for the cutoff byte.
//
// encode_var_ule_write: writes bytes by deferring to the inner ULE impls plus the cutoff byte.
unsafe impl<T0, T1, V0, V1> EncodeAsVarULE<TinyVarVarULE<V0, V1>> for TinyVarVar<T0, T1, V0, V1>
where
    T0: EncodeAsVarULE<V0>,
    T1: EncodeAsVarULE<V1>,
    V0: VarULE + ?Sized,
    V1: VarULE + ?Sized,
{
    fn encode_var_ule_as_slices<R>(&self, _: impl FnOnce(&[&[u8]]) -> R) -> R {
        // unnecessary if the other two are implemented
        unreachable!()
    }

    fn encode_var_ule_len(&self) -> usize {
        self.0.encode_var_ule_len() + self.1.encode_var_ule_len() + 1
    }

    fn encode_var_ule_write(&self, dst: &mut [u8]) {
        let cutoff = self.0.encode_var_ule_len();
        debug_assert!(cutoff <= 0xFF);
        #[allow(clippy::unwrap_used)] // dst has enough bytes by the trait invariant
        let (cutoff_byte, remainder) = dst.split_first_mut().unwrap();
        *cutoff_byte = cutoff as u8;
        let (v0_bytes, v1_bytes) = remainder.split_at_mut(cutoff);
        self.0.encode_var_ule_write(v0_bytes);
        self.1.encode_var_ule_write(v1_bytes);
    }
}

#[test]
fn test_simple_tinyvarvar() {
    let tiny_var_var = TinyVarVar::try_new("hello", "world").unwrap();
    let tiny_var_var_ule = super::encode_varule_to_box(&tiny_var_var);
    assert_eq!(tiny_var_var_ule.get().0, "hello");
    assert_eq!(tiny_var_var_ule.get().1, "world");
}

#[test]
fn test_tinyvarvar_length() {
    let str_255 = &[0u8; 255][..];
    let str_256 = &[0u8; 256][..];

    assert!(TinyVarVar::<_, _, [u8], [u8]>::try_new(str_255, str_256).is_ok());
    assert!(TinyVarVar::<_, _, [u8], [u8]>::try_new(str_256, str_255).is_err());
}

#[test]
fn test_tinyvarvar_validation() {
    let invalid0 = TinyVarVar::try_new(b"\x80invalid".as_slice(), "world").unwrap();
    let invalid0_ule: Box<TinyVarVarULE<[u8], str>> = super::encode_varule_to_box(&invalid0);
    assert!(TinyVarVarULE::<str, str>::parse_byte_slice(invalid0_ule.as_byte_slice()).is_err());

    let invalid1 = TinyVarVar::try_new("hello", b"\x80invalid".as_slice()).unwrap();
    let invalid1_ule: Box<TinyVarVarULE<str, [u8]>> = super::encode_varule_to_box(&invalid1);
    assert!(TinyVarVarULE::<str, str>::parse_byte_slice(invalid1_ule.as_byte_slice()).is_err());
}
