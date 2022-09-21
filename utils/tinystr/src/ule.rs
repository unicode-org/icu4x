// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::TinyAsciiStr;
use zerovec::maps::ZeroMapKV;
use zerovec::ule::*;
use zerovec::{ZeroSlice, ZeroVec};

// Safety (based on the safety checklist on the ULE trait):
//  1. CharULE does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  2. CharULE is aligned to 1 byte.
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
//  5. The other ULE methods use the default impl.
//  6. CharULE byte equality is semantic equality
unsafe impl<const N: usize> ULE for TinyAsciiStr<N> {
    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        if bytes.len() % N != 0 {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }
        // Validate the bytes
        for chunk in bytes.chunks_exact(N) {
            let _ = TinyAsciiStr::<N>::from_bytes_inner(chunk, 0, N, true)
                .map_err(|_| ZeroVecError::parse::<Self>())?;
        }
        Ok(())
    }
}

impl<const N: usize> AsULE for TinyAsciiStr<N> {
    type ULE = Self;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        self
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

impl<'a, const N: usize> ZeroMapKV<'a> for TinyAsciiStr<N> {
    type Container = ZeroVec<'a, TinyAsciiStr<N>>;
    type Slice = ZeroSlice<TinyAsciiStr<N>>;
    type GetType = TinyAsciiStr<N>;
    type OwnedType = TinyAsciiStr<N>;
}

#[cfg(test)]
mod test {
    use crate::*;
    use zerovec::*;

    #[test]
    fn test_zerovec() {
        let mut vec = ZeroVec::<TinyAsciiStr<7>>::new();

        vec.with_mut(|v| v.push("foobar".parse().unwrap()));
        vec.with_mut(|v| v.push("baz".parse().unwrap()));
        vec.with_mut(|v| v.push("quux".parse().unwrap()));

        let bytes = vec.as_bytes();

        let vec: ZeroVec<TinyAsciiStr<7>> = ZeroVec::parse_byte_slice(bytes).unwrap();

        assert_eq!(&*vec.get(0).unwrap(), "foobar");
        assert_eq!(&*vec.get(1).unwrap(), "baz");
        assert_eq!(&*vec.get(2).unwrap(), "quux");
    }
}
