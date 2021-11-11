// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use core::fmt;
use core::mem;

/// We do not have guarantees for the layouts of tuples, so we must define a custom
/// ULE type for pairs. This could potentially be generalized for larger tuples if necessary
#[repr(packed)]
pub struct PairULE<A, B>(pub A, pub B);

unsafe impl<A: ULE, B: ULE> ULE for PairULE<A, B> {
    type Error = PairULEError<A::Error, B::Error>;

    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
        let a_len = mem::size_of::<A>();
        let b_len = mem::size_of::<B>();
        if bytes.len() % (a_len + b_len) != 0 {
            return Err(PairULEError::IndivisibleLength(a_len + b_len, bytes.len()));
        }
        for chunk in bytes.chunks(a_len + b_len) {
            A::validate_byte_slice(&chunk[..a_len]).map_err(PairULEError::First)?;
            B::validate_byte_slice(&chunk[a_len..]).map_err(PairULEError::Second)?;
        }
        Ok(())
    }
}

impl<A: AsULE, B: AsULE> AsULE for (A, B) {
    type ULE = PairULE<A::ULE, B::ULE>;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        PairULE(self.0.as_unaligned(), self.1.as_unaligned())
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        (
            A::from_unaligned(unaligned.0),
            B::from_unaligned(unaligned.1),
        )
    }
}

#[derive(Clone, Debug)]
pub enum PairULEError<E, F> {
    First(E),
    Second(F),
    IndivisibleLength(/* expected */ usize, /* found */ usize),
}

impl<E: fmt::Display, F: fmt::Display> fmt::Display for PairULEError<E, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            PairULEError::First(ref e) => e.fmt(f),
            PairULEError::Second(ref e) => e.fmt(f),
            PairULEError::IndivisibleLength(expected, found) => write!(
                f,
                "Indivisible length for PairULE: expected multiple of {} found {}",
                expected, found
            ),
        }
    }
}

impl<A: fmt::Debug + ULE, B: fmt::Debug + ULE> fmt::Debug for PairULE<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let a = self.0;
        let b = self.1;
        (a, b).fmt(f)
    }
}

impl<A: PartialEq + ULE, B: PartialEq + ULE> PartialEq for PairULE<A, B> {
    fn eq(&self, other: &Self) -> bool {
        let a = self.0;
        let b = self.1;
        let other_a = other.0;
        let other_b = other.1;
        (a, b) == (other_a, other_b)
    }
}

// We need manual impls since `#[derive()]` is disallowed on packed types
impl<A: ULE, B: ULE> Clone for PairULE<A, B> {
    fn clone(&self) -> Self {
        // copy to the stack to avoid hitting a future incompat error
        // https://github.com/rust-lang/rust/issues/82523#issuecomment-947900712
        let zero = self.0;
        let one = self.1;
        PairULE(zero, one)
    }
}

impl<A: ULE, B: ULE> Copy for PairULE<A, B> {}

#[test]
fn test_pairule_validate() {
    use crate::ZeroVec;
    let vec: Vec<(u32, char)> = vec![(1, 'a'), (1234901, '啊'), (100, 'अ')];
    let zerovec: ZeroVec<(u32, char)> = vec.iter().copied().collect();
    let bytes = zerovec.as_bytes();
    let zerovec2 = ZeroVec::parse_byte_slice(bytes).unwrap();
    assert_eq!(zerovec, zerovec2);
}
