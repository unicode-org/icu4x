// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::TinyStrError;
use core::ops::Deref;
use core::str::{self, FromStr};

#[repr(transparent)]
#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Debug, Hash)]
pub struct TinyAsciiStr<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> TinyAsciiStr<N> {
    pub const fn from_bytes(bytes: &[u8]) -> Result<Self, TinyStrError> {
        if bytes.len() > N {
            return Err(TinyStrError::TooLarge {
                max: N,
                found: bytes.len(),
            });
        }

        let mut out = [0; N];
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == 0 {
                return Err(TinyStrError::ContainsNull);
            } else if bytes[i] >= 0x80 {
                return Err(TinyStrError::NonAscii);
            }
            out[i] = bytes[i];

            i += 1;
        }

        Ok(Self { bytes: out })
    }

    pub const fn from_str(s: &str) -> Result<Self, TinyStrError> {
        Self::from_bytes(s.as_bytes())
    }

    pub fn len(&self) -> usize {
        self.bytes.iter().position(|x| *x == 0).unwrap_or(N)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[0..self.len()]
    }

    pub fn all_bytes(&self) -> &[u8; N] {
        &self.bytes
    }

    pub const unsafe fn from_bytes_unchecked(bytes: [u8; N]) -> Self {
        Self { bytes }
    }
}

impl<const N: usize> Deref for TinyAsciiStr<N> {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.as_bytes()) }
    }
}

impl<const N: usize> FromStr for TinyAsciiStr<N> {
    type Err = TinyStrError;
    fn from_str(s: &str) -> Result<Self, TinyStrError> {
        Self::from_str(s)
    }
}
