// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::int_ops::Aligned4;
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

    #[inline]
    pub const fn from_str(s: &str) -> Result<Self, TinyStrError> {
        Self::from_bytes(s.as_bytes())
    }

    pub fn len(&self) -> usize {
        self.bytes.iter().position(|x| *x == 0).unwrap_or(N)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes[0] == 0
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[0..self.len()]
    }

    #[inline]
    pub fn all_bytes(&self) -> &[u8; N] {
        &self.bytes
    }

    /// Checks if the value is composed of ASCII alphabetic characters:
    ///
    ///  * U+0041 'A' ..= U+005A 'Z', or
    ///  * U+0061 'a' ..= U+007A 'z'.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr_neo::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "Test".parse()
    ///     .expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "Te3t".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_alphabetic());
    /// assert!(!s2.is_ascii_alphabetic());
    /// ```
    #[inline]
    pub fn is_ascii_alphabetic(&self) -> bool {
        if N <= 4 {
            let aligned = Aligned4::from_bytes(&self.bytes);
            aligned.is_ascii_alphabetic()
        } else {
            self.as_bytes().iter().all(u8::is_ascii_alphabetic)
        }
    }

    /// Converts this type to its ASCII lower case equivalent in-place.
    ///
    /// ASCII letters 'A' to 'Z' are mapped to 'a' to 'z', other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr_neo::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "TeS3".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1.to_ascii_lowercase(), "tes3");
    /// ```
    #[inline]
    pub fn to_ascii_lowercase(mut self) -> Self {
        if N <= 4 {
            let aligned = Aligned4::from_bytes(&self.bytes);
            let aligned = aligned.to_ascii_lowercase();
            Self::from_slice(&aligned.to_bytes()[0..N])
        } else {
            self.bytes.iter_mut().for_each(u8::make_ascii_lowercase);
            self
        }
    }

    /// # Panics
    /// Panics if src is not exactly N bytes long
    fn from_slice(src: &[u8]) -> Self {
        let mut bytes = [0; N];
        bytes[0..N].copy_from_slice(src);
        Self { bytes }
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

#[cfg(feature = "alloc")]
impl<const N: usize> PartialEq<alloc::string::String> for TinyAsciiStr<N> {
    fn eq(&self, other: &alloc::string::String) -> bool {
        self.deref() == other.deref()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> PartialEq<TinyAsciiStr<N>> for alloc::string::String {
    fn eq(&self, other: &TinyAsciiStr<N>) -> bool {
        self.deref() == other.deref()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const STRINGS: &[&str] = &[
        "Latn",
        "windows",
        "AR",
        "Hans",
        "macos",
        "AT",
        "infiniband",
        "FR",
        "en",
        "Cyrl",
        "FromIntegral",
        "NO",
        "419",
        "MacintoshOSX2019",
        "UK",
        "E12",
    ];

    #[test]
    fn test_is_ascii_alphabetic() {
        fn check<const N: usize>() {
            for s in STRINGS {
                let t = match TinyAsciiStr::<N>::from_str(s) {
                    Ok(t) => t,
                    Err(TinyStrError::TooLarge { .. }) => continue,
                    Err(e) => panic!("{}", e),
                };
                let expected = s.chars().all(|c| c.is_ascii_alphabetic());
                let actual = t.is_ascii_alphabetic();
                assert_eq!(expected, actual);
            }
        }
        check::<2>();
        check::<3>();
        check::<4>();
        check::<5>();
        check::<8>();
        check::<16>();
    }

    #[test]
    fn test_to_ascii_lowercase() {
        fn check<const N: usize>() {
            for s in STRINGS {
                let t = match TinyAsciiStr::<N>::from_str(s) {
                    Ok(t) => t,
                    Err(TinyStrError::TooLarge { .. }) => continue,
                    Err(e) => panic!("{}", e),
                };
                let expected = s
                    .chars()
                    .map(|c| c.to_ascii_lowercase())
                    .collect::<String>();
                let actual = t.to_ascii_lowercase();
                assert_eq!(expected, actual);
            }
        }
        check::<2>();
        check::<3>();
        check::<4>();
        check::<5>();
        check::<8>();
        check::<16>();
    }
}
