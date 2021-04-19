// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ule::*;
use std::str;

impl AsVarULE for String {
    type VarULE = str;
    #[inline]
    fn as_unaligned(&self) -> &str {
        self
    }
    #[inline]
    fn from_unaligned(unaligned: &str) -> Self {
        unaligned.into()
    }
}

impl VarULE for str {
    type Error = str::Utf8Error;

    #[inline]
    fn parse_byte_slice(bytes: &[u8]) -> Result<&Self, Self::Error> {
        str::from_utf8(bytes)
    }
    /// Invariant: must be safe to call when called on a slice that previously
    /// succeeded with `parse_byte_slice`
    #[inline]
    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        str::from_utf8_unchecked(bytes)
    }
    #[inline]
    fn as_byte_slice(&self) -> &[u8] {
        self.as_bytes()
    }
}
