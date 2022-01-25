use core::{
    convert::{Infallible, TryFrom},
    mem, ops,
};

#[cfg(feature = "std")]
use alloc::vec::Vec;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PatternID(u32);

impl PatternID {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    pub const MAX: PatternID = PatternID::new_unchecked(core::i32::MAX as usize - 1);

    #[cfg(target_pointer_width = "16")]
    pub const MAX: PatternID = PatternID::new_unchecked(core::isize::MAX - 1);

    pub const LIMIT: usize = PatternID::MAX.as_usize() + 1;

    pub const ZERO: PatternID = PatternID::new_unchecked(0);

    pub const SIZE: usize = core::mem::size_of::<PatternID>();

    #[inline]
    pub fn new(id: usize) -> Result<PatternID, PatternIDError> {
        PatternID::try_from(id)
    }

    #[inline]
    pub const fn new_unchecked(id: usize) -> PatternID {
        PatternID(id as u32)
    }

    #[inline]
    pub fn must(id: usize) -> PatternID {
        PatternID::new(id).unwrap()
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn as_i32(&self) -> i32 {
        self.0 as i32
    }

    #[inline]
    pub fn one_more(&self) -> usize {
        self.as_usize().checked_add(1).unwrap()
    }

    #[inline]
    pub fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError> {
        let id = u32::from_ne_bytes(bytes);
        if id > PatternID::MAX.as_u32() {
            return Err(PatternIDError {
                attempted: id as u64,
            });
        }
        Ok(PatternID::new_unchecked(id as usize))
    }

    #[inline]
    pub fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID {
        PatternID::new_unchecked(u32::from_ne_bytes(bytes) as usize)
    }

    #[inline]
    pub fn to_ne_bytes(&self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }

    #[cfg(feature = "std")]
    pub(crate) fn iter(len: usize) -> PatternIDIter {
        PatternIDIter::new(len)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternIDError {
    attempted: u64,
}

impl PatternIDError {
    pub fn attempted(&self) -> u64 {
        self.attempted
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PatternIDError {}

impl core::fmt::Display for PatternIDError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "failed to create PatternID from {:?}, which exceeds {:?}",
            self.attempted(),
            PatternID::MAX,
        )
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct StateID(u32);

impl StateID {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    pub const MAX: StateID = StateID::new_unchecked(core::i32::MAX as usize - 1);

    #[cfg(target_pointer_width = "16")]
    pub const MAX: StateID = StateID::new_unchecked(core::isize::MAX - 1);

    pub const LIMIT: usize = StateID::MAX.as_usize() + 1;

    pub const ZERO: StateID = StateID::new_unchecked(0);

    pub const SIZE: usize = core::mem::size_of::<StateID>();

    #[inline]
    pub fn new(id: usize) -> Result<StateID, StateIDError> {
        StateID::try_from(id)
    }

    #[inline]
    pub const fn new_unchecked(id: usize) -> StateID {
        StateID(id as u32)
    }

    #[inline]
    pub fn must(id: usize) -> StateID {
        StateID::new(id).unwrap()
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn as_i32(&self) -> i32 {
        self.0 as i32
    }

    #[inline]
    pub fn one_more(&self) -> usize {
        self.as_usize().checked_add(1).unwrap()
    }

    #[inline]
    pub fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError> {
        let id = u32::from_ne_bytes(bytes);
        if id > StateID::MAX.as_u32() {
            return Err(StateIDError {
                attempted: id as u64,
            });
        }
        Ok(StateID::new_unchecked(id as usize))
    }

    #[inline]
    pub fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID {
        StateID::new_unchecked(u32::from_ne_bytes(bytes) as usize)
    }

    #[inline]
    pub fn to_ne_bytes(&self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }

    #[cfg(feature = "std")]
    pub(crate) fn iter(len: usize) -> StateIDIter {
        StateIDIter::new(len)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateIDError {
    attempted: u64,
}

impl StateIDError {
    pub fn attempted(&self) -> u64 {
        self.attempted
    }
}

#[cfg(feature = "std")]
impl std::error::Error for StateIDError {}

impl core::fmt::Display for StateIDError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "failed to create StateID from {:?}, which exceeds {:?}",
            self.attempted(),
            StateID::MAX,
        )
    }
}

macro_rules! impls {
    ($ty:ident, $tyerr:ident, $tyiter:ident) => {
        #[derive(Clone, Debug)]
        pub(crate) struct $tyiter {
            rng: ops::Range<usize>,
        }

        impl $tyiter {
            #[cfg(feature = "std")]
            fn new(len: usize) -> $tyiter {
                assert!(
                    len <= $ty::LIMIT,
                    "cannot create iterator with IDs when number of \
                     elements exceed {:?}",
                    $ty::LIMIT,
                );
                $tyiter { rng: 0..len }
            }
        }

        impl Iterator for $tyiter {
            type Item = $ty;

            fn next(&mut self) -> Option<$ty> {
                if self.rng.start >= self.rng.end {
                    return None;
                }
                let next_id = self.rng.start + 1;
                let id = mem::replace(&mut self.rng.start, next_id);
                // new_unchecked is OK since we asserted that the number of
                // elements in this iterator will fit in an ID at construction.
                Some($ty::new_unchecked(id))
            }
        }

        impl<T> core::ops::Index<$ty> for [T] {
            type Output = T;

            #[inline]
            fn index(&self, index: $ty) -> &T {
                &self[index.as_usize()]
            }
        }

        impl<T> core::ops::IndexMut<$ty> for [T] {
            #[inline]
            fn index_mut(&mut self, index: $ty) -> &mut T {
                &mut self[index.as_usize()]
            }
        }

        #[cfg(feature = "std")]
        impl<T> core::ops::Index<$ty> for Vec<T> {
            type Output = T;

            #[inline]
            fn index(&self, index: $ty) -> &T {
                &self[index.as_usize()]
            }
        }

        #[cfg(feature = "std")]
        impl<T> core::ops::IndexMut<$ty> for Vec<T> {
            #[inline]
            fn index_mut(&mut self, index: $ty) -> &mut T {
                &mut self[index.as_usize()]
            }
        }

        impl TryFrom<usize> for $ty {
            type Error = $tyerr;

            fn try_from(id: usize) -> Result<$ty, $tyerr> {
                if id > $ty::MAX.as_usize() {
                    return Err($tyerr {
                        attempted: id as u64,
                    });
                }
                Ok($ty::new_unchecked(id))
            }
        }

        impl TryFrom<u8> for $ty {
            type Error = Infallible;

            fn try_from(id: u8) -> Result<$ty, Infallible> {
                Ok($ty::new_unchecked(id as usize))
            }
        }

        impl TryFrom<u16> for $ty {
            type Error = $tyerr;

            fn try_from(id: u16) -> Result<$ty, $tyerr> {
                if id as u32 > $ty::MAX.as_u32() {
                    return Err($tyerr {
                        attempted: id as u64,
                    });
                }
                Ok($ty::new_unchecked(id as usize))
            }
        }

        impl TryFrom<u32> for $ty {
            type Error = $tyerr;

            fn try_from(id: u32) -> Result<$ty, $tyerr> {
                if id > $ty::MAX.as_u32() {
                    return Err($tyerr {
                        attempted: id as u64,
                    });
                }
                Ok($ty::new_unchecked(id as usize))
            }
        }

        impl TryFrom<u64> for $ty {
            type Error = $tyerr;

            fn try_from(id: u64) -> Result<$ty, $tyerr> {
                if id > $ty::MAX.as_u32() as u64 {
                    return Err($tyerr { attempted: id });
                }
                Ok($ty::new_unchecked(id as usize))
            }
        }
    };
}

impls!(PatternID, PatternIDError, PatternIDIter);
impls!(StateID, StateIDError, StateIDIter);

#[cfg(feature = "std")]
pub(crate) trait IteratorIDExt: Iterator {
    fn with_pattern_ids(self) -> WithPatternIDIter<Self>
    where
        Self: Sized + ExactSizeIterator,
    {
        WithPatternIDIter::new(self)
    }

    fn with_state_ids(self) -> WithStateIDIter<Self>
    where
        Self: Sized + ExactSizeIterator,
    {
        WithStateIDIter::new(self)
    }
}

#[cfg(feature = "std")]
impl<I: Iterator> IteratorIDExt for I {}

#[cfg(feature = "std")]
macro_rules! iditer {
    ($ty:ident, $iterty:ident, $withiterty:ident) => {
        #[derive(Clone, Debug)]
        pub(crate) struct $withiterty<I> {
            it: I,
            ids: $iterty,
        }

        impl<I: Iterator + ExactSizeIterator> $withiterty<I> {
            fn new(it: I) -> $withiterty<I> {
                let ids = $ty::iter(it.len());
                $withiterty { it, ids }
            }
        }

        impl<I: Iterator + ExactSizeIterator> Iterator for $withiterty<I> {
            type Item = ($ty, I::Item);

            fn next(&mut self) -> Option<($ty, I::Item)> {
                let item = self.it.next()?;
                // Number of elements in this iterator must match, according
                // to contract of ExactSizeIterator.
                let id = self.ids.next().unwrap();
                Some((id, item))
            }
        }
    };
}

#[cfg(feature = "std")]
iditer!(PatternID, PatternIDIter, WithPatternIDIter);
#[cfg(feature = "std")]
iditer!(StateID, StateIDIter, WithStateIDIter);
