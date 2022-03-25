// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use core::mem::{self, MaybeUninit};

/// This type is the [`ULE`] type for `Option<T>`.
///
/// # Example
///
/// ```rust
/// use zerovec::ZeroVec;
///
/// let z = ZeroVec::alloc_from_slice(&[Some('a'), Some('á'), Some('ø'), Some('ł')]);
///
/// assert_eq!(z.get(2), Some(Some(('ø'))));
/// ```
// Invariants:
// The MaybeUninit is zeroed when None (bool = false),
// and is valid when Some (bool = true)
#[repr(packed)]
pub struct OptionULE<T>(bool, MaybeUninit<T>);

impl<T: Copy> OptionULE<T> {
    /// Obtain this as an Option<T>
    pub fn get(self) -> Option<T> {
        if self.0 {
            unsafe {
                // safety: self.0 is true so the MaybeUninit is valid
                Some(self.1.assume_init())
            }
        } else {
            None
        }
    }

    /// Construct an OptionULE<T> from an equivalent Option<T>
    pub fn new(opt: Option<T>) -> Self {
        if let Some(inner) = opt {
            Self(true, MaybeUninit::new(inner))
        } else {
            Self(false, MaybeUninit::zeroed())
        }
    }
}

// Safety (based on the safety checklist on the ULE trait):
//  1. OptionULE does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(packed)]` on a struct containing only ULE fields)
//  2. OptionULE is aligned to 1 byte.
//     (achieved by `#[repr(packed)]` on a struct containing only ULE fields)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
//  5. The other ULE methods use the default impl.
//  6. OptionULE byte equality is semantic equality by relying on the ULE equality
//     invariant on the subfields
unsafe impl<T: ULE> ULE for OptionULE<T> {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        let size = mem::size_of::<Self>();
        if bytes.len() % size != 0 {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }
        for chunk in bytes.chunks(size) {
            match chunk[0] {
                0 => {
                    if !chunk[1..].iter().all(|x| *x == 0) {
                        return Err(ZeroVecError::parse::<Self>());
                    }
                }
                1 => T::validate_byte_slice(&chunk[1..])?,
                _ => return Err(ZeroVecError::parse::<Self>()),
            }
        }
        Ok(())
    }
}

impl<T: AsULE> AsULE for Option<T> {
    type ULE = OptionULE<T::ULE>;
    fn to_unaligned(self) -> OptionULE<T::ULE> {
        OptionULE::new(self.map(T::to_unaligned))
    }

    fn from_unaligned(other: OptionULE<T::ULE>) -> Self {
        other.get().map(T::from_unaligned)
    }
}

impl<T: Copy> Copy for OptionULE<T> {}

impl<T: Copy> Clone for OptionULE<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Copy + PartialEq> PartialEq for OptionULE<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(&other.get())
    }
}

impl<T: Copy + Eq> Eq for OptionULE<T> {}

use core::marker::PhantomData;

/// A type allowing one to represent `Option<T>` for [`VarULE`] `T` types.
// The slice field is empty when None (bool = false),
// and is a valid T when Some (bool = true)
#[repr(packed)]
pub struct OptionVarULE<T: VarULE + ?Sized>(PhantomData<T>, bool, [u8]);

impl<T: VarULE + ?Sized> OptionVarULE<T> {
    /// Obtain this as an `Option<&T>`
    pub fn as_ref(&self) -> Option<&T> {
        if self.1 {
            unsafe {
                // Safety: byte field is a valid T if boolean field is true
                Some(T::from_byte_slice_unchecked(&self.2))
            }
        } else {
            None
        }
    }
}

// Safety (based on the safety checklist on the VarULE trait):
//  1. OptionVarULE<T> does not include any uninitialized or padding bytes (achieved by being repr(packed) on ULE types)
//  2. OptionVarULE<T> is aligned to 1 byte (achieved by being repr(packed) on ULE types)
//  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid.
//  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety
//  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
//  6. All other methods are defaulted
//  7. OptionVarULE<T> byte equality is semantic equality (achieved by being an aggregate)
unsafe impl<T: VarULE + ?Sized> VarULE for OptionVarULE<T> {
    #[inline]
    fn validate_byte_slice(slice: &[u8]) -> Result<(), ZeroVecError> {
        if slice.is_empty() {
            return Err(ZeroVecError::length::<Self>(slice.len()));
        }
        match slice[0] {
            0 => {
                if slice.len() != 1 {
                    Err(ZeroVecError::length::<Self>(slice.len()))
                } else {
                    Ok(())
                }
            }
            1 => T::validate_byte_slice(&slice[1..]),
            _ => Err(ZeroVecError::parse::<Self>()),
        }
    }

    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        let metadata = bytes.len() - 1;
        let entire_struct_as_slice: *const [u8] =
            ::core::slice::from_raw_parts(bytes.as_ptr(), metadata);
        &*(entire_struct_as_slice as *const Self)
    }
}

unsafe impl<T, U> EncodeAsVarULE<OptionVarULE<U>> for Option<T>
where
    T: EncodeAsVarULE<U>,
    U: VarULE,
{
    fn encode_var_ule_as_slices<R>(&self, _: impl FnOnce(&[&[u8]]) -> R) -> R {
        // unnecessary if the other two are implemented
        unreachable!()
    }

    #[inline]
    fn encode_var_ule_len(&self) -> usize {
        if let Some(ref inner) = *self {
            // slice + boolean
            1 + inner.encode_var_ule_len()
        } else {
            // boolean + empty slice
            1
        }
    }

    fn encode_var_ule_write(&self, dst: &mut [u8]) {
        if let Some(ref inner) = *self {
            dst[0] = 1;
            inner.encode_var_ule_write(&mut dst[1..]);
        } else {
            dst[0] = 0;
        }
    }
}
