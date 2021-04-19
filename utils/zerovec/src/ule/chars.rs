// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::upper_case_acronyms)]
//! ULE implementation for the `char` type.

use super::*;
use std::convert::TryFrom;

/// A u8 array of little-endian data corresponding to a Unicode code point.
///
/// The bytes of a `CharULE` are guaranteed to represent a little-endian-encoded u32 that is a
/// valid `char` and can be converted without validation.
///
/// # Examples
///
/// Convert a `char` to a `CharULE` and back again:
///
/// ```
/// use zerovec::ule::{ULE, AsULE, CharULE};
///
/// let c1 = '𑄃';
/// let ule = c1.as_unaligned();
/// assert_eq!(CharULE::as_byte_slice(&[ule]), &[0x03, 0x11, 0x01, 0x00]);
/// let c2 = char::from_unaligned(&ule);
/// assert_eq!(c1, c2);
/// ```
///
/// Attempt to parse invalid bytes to a `CharULE`:
///
/// ```
/// use zerovec::ule::{ULE, CharULE};
///
/// let bytes: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF];
/// CharULE::parse_byte_slice(bytes).expect_err("Invalid bytes");
/// ```
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CharULE([u8; 4]);

impl ULE for CharULE {
    type Error = std::char::CharTryFromError;

    #[inline]
    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
        // Validate the bytes
        for chunk in bytes.chunks_exact(4) {
            // TODO: Use slice::as_chunks() when stabilized
            let u = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            char::try_from(u)?;
        }
        let data = bytes.as_ptr();
        let len = bytes.len() / 4;
        // Safe because Self is transparent over [u8; 4]
        Ok(unsafe { std::slice::from_raw_parts(data as *const Self, len) })
    }

    #[inline]
    fn as_byte_slice(slice: &[Self]) -> &[u8] {
        let data = slice.as_ptr();
        let len = slice.len() * 4;
        // Safe because Self is transparent over [u8; 4]
        unsafe { std::slice::from_raw_parts(data as *const u8, len) }
    }
}

impl AsULE for char {
    type ULE = CharULE;

    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        let u = u32::from(*self);
        CharULE(u.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        let u = u32::from_le_bytes(unaligned.0);
        // Safe because the bytes of CharULE are defined to represent a valid Unicode code point.
        // TODO: Use char::from_u32_unchecked() when stabilized
        Self::try_from(u).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        // 1-byte, 2-byte, 3-byte, and 4-byte character in UTF-8 (not as relevant in UTF-32)
        let chars = ['w', 'ω', '文', '𑄃'];
        let char_ules: Vec<CharULE> = chars.iter().map(char::as_unaligned).collect();
        let char_bytes: &[u8] = CharULE::as_byte_slice(&char_ules);

        // Check parsing
        let parsed_ules: &[CharULE] = CharULE::parse_byte_slice(&char_bytes).unwrap();
        assert_eq!(char_ules, parsed_ules);
        let parsed_chars: Vec<char> = parsed_ules.iter().map(char::from_unaligned).collect();
        assert_eq!(&chars, parsed_chars.as_slice());

        // Compare to u32
        let u32s: Vec<u32> = chars.iter().copied().map(u32::from).collect();
        let u32_ules: Vec<PlainOldULE<4>> = u32s.iter().map(<u32 as AsULE>::as_unaligned).collect();
        let u32_bytes: &[u8] = PlainOldULE::<4>::as_byte_slice(&u32_ules);
        assert_eq!(char_bytes, u32_bytes);

        // Compare to golden expected data
        assert_eq!(
            &[119, 0, 0, 0, 201, 3, 0, 0, 135, 101, 0, 0, 3, 17, 1, 0],
            char_bytes
        );
    }

    #[test]
    fn test_failures() {
        // 119 and 120 are valid, but not 0xD800 (high surrogate)
        let u32s = [119, 0xD800, 120];
        let u32_ules: Vec<PlainOldULE<4>> = u32s.iter().map(<u32 as AsULE>::as_unaligned).collect();
        let u32_bytes: &[u8] = PlainOldULE::<4>::as_byte_slice(&u32_ules);
        let parsed_ules_result = CharULE::parse_byte_slice(&u32_bytes);
        assert!(matches!(parsed_ules_result, Err(_)));

        // 0x20FFFF is out of range for a char
        let u32s = [0x20FFFF];
        let u32_ules: Vec<PlainOldULE<4>> = u32s.iter().map(<u32 as AsULE>::as_unaligned).collect();
        let u32_bytes: &[u8] = PlainOldULE::<4>::as_byte_slice(&u32_ules);
        let parsed_ules_result = CharULE::parse_byte_slice(&u32_bytes);
        assert!(matches!(parsed_ules_result, Err(_)));
    }
}
