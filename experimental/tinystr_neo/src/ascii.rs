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
        Self::from_bytes_inner(bytes, false)
    }

    #[inline]
    pub(crate) const fn from_bytes_inner(
        bytes: &[u8],
        allow_trailing_null: bool,
    ) -> Result<Self, TinyStrError> {
        if bytes.len() > N {
            return Err(TinyStrError::TooLarge {
                max: N,
                found: bytes.len(),
            });
        }

        let mut out = [0; N];
        let mut i = 0;
        let mut found_null = false;
        while i < bytes.len() {
            if bytes[i] == 0 {
                found_null = true;
            } else if bytes[i] >= 0x80 {
                return Err(TinyStrError::NonAscii);
            } else if found_null {
                // Error if there are contentful bytes after null
                return Err(TinyStrError::ContainsNull);
            }
            out[i] = bytes[i];

            i += 1;
        }

        if !allow_trailing_null && found_null {
            // We found some trailing nulls, error
            return Err(TinyStrError::ContainsNull);
        }

        Ok(Self { bytes: out })
    }

    pub const fn from_str(s: &str) -> Result<Self, TinyStrError> {
        Self::from_bytes(s.as_bytes())
    }

    pub fn len(&self) -> usize {
        self.bytes.iter().position(|x| *x == 0).unwrap_or(N)
    }

    pub fn is_empty(&self) -> bool {
        self.bytes[0] == 0
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[0..self.len()]
    }

    pub fn all_bytes(&self) -> &[u8; N] {
        &self.bytes
    }

    /// # Safety
    /// Must be called with a bytes array made of valid ASCII bytes, with no null bytes
    /// between ASCII characters
    pub const unsafe fn from_bytes_unchecked(bytes: [u8; N]) -> Self {
        Self { bytes }
    }
}

impl<const N: usize> Deref for TinyAsciiStr<N> {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }
}

impl<const N: usize> FromStr for TinyAsciiStr<N> {
    type Err = TinyStrError;
    fn from_str(s: &str) -> Result<Self, TinyStrError> {
        Self::from_str(s)
    }
}
