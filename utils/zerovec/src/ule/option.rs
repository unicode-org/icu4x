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

impl<T: Copy + Eq> Eq for OptionULE<T> {

}
