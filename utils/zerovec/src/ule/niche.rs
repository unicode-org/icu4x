// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::mem::size_of;
use core::ops::DerefMut;
use core::{marker::Copy, ops::Deref};

use super::{AsULE, ULE};

/// The types implementing this trait guarantee that [`NicheBytes::INVALID_BIT_PATTERN`]
/// can never occur as a valid byte representation of the type.
/// The implementors guarantee that N == core::mem::sizeo_of::<Self>()
pub trait NicheBytes<const N: usize> {
    const INVALID_BIT_PATTERN: [u8; N];
}

/// [`ULE`] type for [`NichedOption<U,N>`] where U implements [`NicheBytes`].
/// The invalid bit pattern is used as the niche.
///
/// This uses 1 byte less than [`zerovec::OptionULE<U>`] to represent [`NichedOption<U,N>`]
/// which Derefs to [`Option<U>`].
/// The implementors guarantee that N == core::mem::sizeo_of::<Self>()
///
/// Invariants:
/// The union stores [`NichedBytes::INVALID_BIT_PATTERN`] when None.
/// Any other bit pattern is a valid.
///
/// # Example
///
/// ```rust
/// use zerovec::ZeroVec;
/// use zerovec::ule::NichedOption;
/// use core::num::NonZeroU8;
///
/// let bytes = &[0x00, 0x01, 0x02, 0x00 ];
/// let zv_no: ZeroVec<NichedOption<NonZeroU8, 1>> =
///         ZeroVec::parse_byte_slice(bytes).expect("Unable to parse as NichedOption.");
///
/// assert_eq!(zv_no.get(0).as_deref(), Some(&None));
/// assert_eq!(zv_no.get(1).as_deref(), Some(&NonZeroU8::new(1)));
/// assert_eq!(zv_no.get(2).as_deref(), Some(&NonZeroU8::new(2)));
/// assert_eq!(zv_no.get(3).as_deref(), Some(&None));
/// ```
#[repr(packed)]
pub union NichedOptionULE<U: NicheBytes<N> + ULE, const N: usize> {
    invalid: [u8; N],
    valid: U,
}

impl<U: NicheBytes<N> + ULE, const N: usize> NichedOptionULE<U, N> {
    /// New NichedOptionULE<U, N> from Option<U>
    pub fn new(opt: Option<U>) -> Self {
        assert!(N == core::mem::size_of::<U>());
        match opt {
            Some(u) => Self { valid: u },
            None => Self {
                invalid: <U as NicheBytes<N>>::INVALID_BIT_PATTERN,
            },
        }
    }

    /// Convert to an Option<U>
    pub fn get(self) -> Option<U> {
        if unsafe { self.invalid } == <U as NicheBytes<N>>::INVALID_BIT_PATTERN {
            None
        } else {
            // Safety: the inner value is valid
            unsafe { Some(self.valid) }
        }
    }
}

impl<U: NicheBytes<N> + ULE, const N: usize> From<Option<U>> for NichedOptionULE<U, N> {
    fn from(opt: Option<U>) -> Self {
        Self::new(opt)
    }
}

impl<U: NicheBytes<N> + ULE, const N: usize> From<NichedOptionULE<U, N>> for Option<U> {
    fn from(noule: NichedOptionULE<U, N>) -> Self {
        NichedOptionULE::get(noule)
    }
}

impl<U: NicheBytes<N> + ULE, const N: usize> Copy for NichedOptionULE<U, N> {}

impl<U: NicheBytes<N> + ULE, const N: usize> Clone for NichedOptionULE<U, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<U: NicheBytes<N> + ULE + PartialEq, const N: usize> PartialEq for NichedOptionULE<U, N> {
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(&other.get())
    }
}

impl<U: NicheBytes<N> + ULE + Eq, const N: usize> Eq for NichedOptionULE<U, N> {}

/// Safety for ULE trait
/// 1. NichedOptionULE does not have any padding bytes due to `#[repr(packed)]` on a struct
///    containing only ULE fields.
///    NichedOptionULE either contains INVALID_BIT_PATTERN or valid U byte sequences.
///    In both cases the data is initialized.
/// 2. NichedOptionULE is aligned to 1 byte due to `#[repr(packed)]` on a struct containing only
///    ULE fields.
/// 3. validate_byte_slice impl returns an error if invalid bytes are encountered.
/// 4. validate_byte_slice impl returns an error there are extra bytes.
/// 5. The other ULE methods are left to their default impl.
/// 6. NichedOptionULE equality is based on ULE equality of the subfield.
unsafe impl<U: NicheBytes<N> + ULE, const N: usize> ULE for NichedOptionULE<U, N> {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), crate::ZeroVecError> {
        let size = size_of::<Self>();
        // The bytes should fully transmute to a collection of Self
        if bytes.len() % size != 0 {
            return Err(crate::ZeroVecError::length::<Self>(bytes.len()));
        }
        bytes.chunks(size).try_for_each(|chunk| {
            // Associated const cannot be referenced in a pattern
            // https://doc.rust-lang.org/error-index.html#E0158
            if chunk == <U as NicheBytes<N>>::INVALID_BIT_PATTERN {
                Ok(())
            } else {
                U::validate_byte_slice(chunk)
            }
        })
    }
}

/// Optional type which uses [`NichedOptionULE<U,N>`] as ULE type.
/// Derefs to [`Option<U>`]
/// The implementors guarantee that N == core::mem::sizeo_of::<Self>()
/// [`repr(transparent)`] guarantees that the layout is same as [`Option<U>`]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NichedOption<U: AsULE, const N: usize>(Option<U>);

impl<U: AsULE, const N: usize> NichedOption<U, N> {
    pub const fn new(o: Option<U>) -> Self {
        Self(o)
    }
}

impl<U: AsULE, const N: usize> Default for NichedOption<U, N> {
    fn default() -> Self {
        Self(None)
    }
}

impl<U: AsULE, const N: usize> From<Option<U>> for NichedOption<U, N> {
    fn from(o: Option<U>) -> Self {
        Self(o)
    }
}

impl<U: AsULE, const N: usize> Deref for NichedOption<U, N> {
    type Target = Option<U>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<U: AsULE, const N: usize> DerefMut for NichedOption<U, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<U: AsULE, const N: usize> AsULE for NichedOption<U, N>
where
    U::ULE: NicheBytes<N>,
{
    type ULE = NichedOptionULE<U::ULE, N>;

    fn to_unaligned(self) -> Self::ULE {
        NichedOptionULE::new(self.map(U::to_unaligned))
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(unaligned.get().map(U::from_unaligned))
    }
}
