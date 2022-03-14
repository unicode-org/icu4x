// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::int_ops::{Aligned4, Aligned8};
use crate::TinyStrError;
use core::fmt;
use core::ops::Deref;
use core::str::{self, FromStr};

#[repr(transparent)]
#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct TinyAsciiStr<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> TinyAsciiStr<N> {
    pub const fn from_bytes(bytes: &[u8]) -> Result<Self, TinyStrError> {
        Self::from_bytes_inner(bytes, 0, bytes.len(), false)
    }

    pub const fn from_bytes_manual_slice(
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Self, TinyStrError> {
        Self::from_bytes_inner(bytes, start, end, false)
    }

    #[inline]
    pub(crate) const fn from_bytes_inner(
        bytes: &[u8],
        start: usize,
        end: usize,
        allow_trailing_null: bool,
    ) -> Result<Self, TinyStrError> {
        let len = end - start;
        if len > N {
            return Err(TinyStrError::TooLarge { max: N, len });
        }

        let mut out = [0; N];
        let mut i = 0;
        let mut found_null = false;
        // Indexing is protected by TinyStrError::TooLarge
        #[allow(clippy::indexing_slicing)]
        while i < len {
            let b = bytes[start + i];

            if b == 0 {
                found_null = true;
            } else if b >= 0x80 {
                return Err(TinyStrError::NonAscii);
            } else if found_null {
                // Error if there are contentful bytes after null
                return Err(TinyStrError::ContainsNull);
            }
            out[i] = b;

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
        Self::from_bytes_inner(s.as_bytes(), 0, s.len(), false)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &*self
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        if N <= 4 {
            Aligned4::from_bytes(&self.bytes).len()
        } else if N <= 8 {
            Aligned8::from_bytes(&self.bytes).len()
        } else {
            self.bytes.iter().position(|x| *x == 0).unwrap_or(N)
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes[0] == 0
    }

    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[0..self.len()]
    }

    #[inline]
    #[must_use]
    pub const fn all_bytes(&self) -> &[u8; N] {
        &self.bytes
    }

    #[inline]
    #[must_use]
    /// Resizes a TinyAsciiStr<N> to a TinyAsciiStr<M>.
    ///
    /// If M < len() the string gets truncated, otherwise only the
    /// memory representation changes.
    pub const fn resize<const M: usize>(self) -> TinyAsciiStr<M> {
        let mut bytes = [0; M];
        let mut i = 0;
        // Indexing is protected by the loop guard
        #[allow(clippy::indexing_slicing)]
        while i < M && i < N {
            bytes[i] = self.bytes[i];
            i += 1;
        }
        // `self.bytes` only contains ASCII bytes, with no null bytes between
        // ASCII characters, so this also holds for `bytes`.
        unsafe { TinyAsciiStr::from_bytes_unchecked(bytes) }
    }

    /// # Safety
    /// Must be called with a bytes array made of valid ASCII bytes, with no null bytes
    /// between ASCII characters
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(bytes: [u8; N]) -> Self {
        Self { bytes }
    }
}

macro_rules! check_is {
    ($self:ident, $check:ident, $check_u8:ident) => {
        if N <= 4 {
            Aligned4::from_bytes(&$self.bytes).$check()
        } else if N <= 8 {
            Aligned8::from_bytes(&$self.bytes).$check()
        } else {
            let mut i = 0;
            // Won't panic because self.bytes has length N
            #[allow(clippy::indexing_slicing)]
            while i < N && $self.bytes[i] != 0 {
                if !$self.bytes[i].$check_u8() {
                    return false;
                }
                i += 1;
            }
            true
        }
    };
}

impl<const N: usize> TinyAsciiStr<N> {
    /// Checks if the value is composed of ASCII alphabetic characters:
    ///
    ///  * U+0041 'A' ..= U+005A 'Z', or
    ///  * U+0061 'a' ..= U+007A 'z'.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
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
    #[must_use]
    pub const fn is_ascii_alphabetic(&self) -> bool {
        check_is!(self, is_ascii_alphabetic, is_ascii_alphabetic)
    }

    /// Checks if the value is composed of ASCII alphanumeric characters:
    ///
    ///  * U+0041 'A' ..= U+005A 'Z', or
    ///  * U+0061 'a' ..= U+007A 'z', or
    ///  * U+0030 '0' ..= U+0039 '9'.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "A15b".parse()
    ///     .expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "[3@w".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_alphanumeric());
    /// assert!(!s2.is_ascii_alphanumeric());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_alphanumeric(&self) -> bool {
        check_is!(self, is_ascii_alphanumeric, is_ascii_alphanumeric)
    }

    /// Checks if the value is composed of ASCII decimal digits:
    ///
    ///  * U+0030 '0' ..= U+0039 '9'.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "312".parse()
    ///     .expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "3d".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_numeric());
    /// assert!(!s2.is_ascii_numeric());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_numeric(&self) -> bool {
        check_is!(self, is_ascii_numeric, is_ascii_digit)
    }
}

macro_rules! to {
    ($self:ident, $to:ident, $later_char_to:ident $(,$first_char_to:ident)?) => {{
        let mut i = 0;
        if N <= 4 {
            let aligned = Aligned4::from_bytes(&$self.bytes).$to().to_bytes();
            // Won't panic because self.bytes has length N and aligned has length >= N
            #[allow(clippy::indexing_slicing)]
            while i < N {
                $self.bytes[i] = aligned[i];
                i += 1;
            }
        } else if N <= 8 {
            let aligned = Aligned8::from_bytes(&$self.bytes).$to().to_bytes();
            // Won't panic because self.bytes has length N and aligned has length >= N
            #[allow(clippy::indexing_slicing)]
            while i < N {
                $self.bytes[i] = aligned[i];
                i += 1;
            }
        } else {
            // Won't panic because self.bytes has length N
            #[allow(clippy::indexing_slicing)]
            while i < N && $self.bytes[i] != 0 {
                $self.bytes[i] = $self.bytes[i].$later_char_to();
                i += 1;
            }
            $($self.bytes[0] = $self.bytes[0].$first_char_to())?
        }
        $self
    }};
}

impl<const N: usize> TinyAsciiStr<N> {
    /// Converts this type to its ASCII lower case equivalent in-place.
    ///
    /// ASCII letters 'A' to 'Z' are mapped to 'a' to 'z', other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "TeS3".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(&*s1.to_ascii_lowercase(), "tes3");
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_ascii_lowercase(mut self) -> Self {
        to!(self, to_ascii_lowercase, to_ascii_lowercase)
    }

    /// Converts this type to its ASCII title case equivalent in-place.
    ///
    /// First character, if is an ASCII letter 'a' to 'z' is mapped to 'A' to 'Z',
    /// other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "test".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(&*s1.to_ascii_titlecase(), "Test");
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_ascii_titlecase(mut self) -> Self {
        to!(
            self,
            to_ascii_titlecase,
            to_ascii_lowercase,
            to_ascii_uppercase
        )
    }

    /// Converts this type to its ASCII upper case equivalent in-place.
    ///
    /// ASCII letters 'a' to 'z' are mapped to 'A' to 'Z', other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "Tes3".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(&*s1.to_ascii_uppercase(), "TES3");
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_ascii_uppercase(mut self) -> Self {
        to!(self, to_ascii_uppercase, to_ascii_uppercase)
    }
}

impl<const N: usize> fmt::Debug for TinyAsciiStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<const N: usize> fmt::Display for TinyAsciiStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<const N: usize> Deref for TinyAsciiStr<N> {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }
}

impl<const N: usize> FromStr for TinyAsciiStr<N> {
    type Err = TinyStrError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, TinyStrError> {
        Self::from_str(s)
    }
}

impl<const N: usize> PartialEq<str> for TinyAsciiStr<N> {
    fn eq(&self, other: &str) -> bool {
        self.deref() == other
    }
}

impl<const N: usize> PartialEq<&str> for TinyAsciiStr<N> {
    fn eq(&self, other: &&str) -> bool {
        self.deref() == *other
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

    fn check_operation<T, F1, F2, const N: usize>(reference_f: F1, tinystr_f: F2)
    where
        F1: Fn(&str) -> T,
        F2: Fn(TinyAsciiStr<N>) -> T,
        T: core::fmt::Debug + core::cmp::PartialEq,
    {
        for s in STRINGS {
            let t = match TinyAsciiStr::<N>::from_str(s) {
                Ok(t) => t,
                Err(TinyStrError::TooLarge { .. }) => continue,
                Err(e) => panic!("{}", e),
            };
            let expected = reference_f(s);
            let actual = tinystr_f(t);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_is_ascii_alphabetic() {
        fn check<const N: usize>() {
            check_operation(
                |s| s.chars().all(|c| c.is_ascii_alphabetic()),
                |t: TinyAsciiStr<N>| TinyAsciiStr::is_ascii_alphabetic(&t),
            )
        }
        check::<2>();
        check::<3>();
        check::<4>();
        check::<5>();
        check::<8>();
        check::<16>();
    }

    #[test]
    fn test_is_ascii_alphanumeric() {
        fn check<const N: usize>() {
            check_operation(
                |s| s.chars().all(|c| c.is_ascii_alphanumeric()),
                |t: TinyAsciiStr<N>| TinyAsciiStr::is_ascii_alphanumeric(&t),
            )
        }
        check::<2>();
        check::<3>();
        check::<4>();
        check::<5>();
        check::<8>();
        check::<16>();
    }

    #[test]
    fn test_is_ascii_numeric() {
        fn check<const N: usize>() {
            check_operation(
                |s| s.chars().all(|c| c.is_ascii_digit()),
                |t: TinyAsciiStr<N>| TinyAsciiStr::is_ascii_numeric(&t),
            )
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
            check_operation(
                |s| {
                    s.chars()
                        .map(|c| c.to_ascii_lowercase())
                        .collect::<String>()
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::to_ascii_lowercase(t).to_string(),
            )
        }
        check::<2>();
        check::<3>();
        check::<4>();
        check::<5>();
        check::<8>();
        check::<16>();
    }

    #[test]
    fn test_to_ascii_titlecase() {
        fn check<const N: usize>() {
            check_operation(
                |s| {
                    let mut r = s
                        .chars()
                        .map(|c| c.to_ascii_lowercase())
                        .collect::<String>();
                    // Safe because the string is an ASCII string
                    unsafe { r.as_bytes_mut()[0].make_ascii_uppercase() };
                    r
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::to_ascii_titlecase(t).to_string(),
            )
        }
        check::<2>();
        check::<3>();
        check::<4>();
        check::<5>();
        check::<8>();
        check::<16>();
    }

    #[test]
    fn test_to_ascii_uppercase() {
        fn check<const N: usize>() {
            check_operation(
                |s| {
                    s.chars()
                        .map(|c| c.to_ascii_uppercase())
                        .collect::<String>()
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::to_ascii_uppercase(t).to_string(),
            )
        }
        check::<2>();
        check::<3>();
        check::<4>();
        check::<5>();
        check::<8>();
        check::<16>();
    }
}
