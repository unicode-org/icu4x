// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::asciibyte::AsciiByte;
use crate::int_ops::{Aligned4, Aligned8};
use crate::TinyStrError;
use core::fmt;
use core::ops::Deref;
use core::str::{self, FromStr};

#[repr(transparent)]
#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct TinyAsciiStr<const N: usize> {
    bytes: [AsciiByte; N],
}

impl<const N: usize> TinyAsciiStr<N> {
    /// Creates a `TinyAsciiStr<N>` from the given byte slice.
    /// `bytes` may contain at most `N` non-null ASCII bytes.
    pub const fn from_bytes(bytes: &[u8]) -> Result<Self, TinyStrError> {
        Self::from_bytes_inner(bytes, 0, bytes.len(), false)
    }

    /// Attempts to parse a fixed-length byte array to a `TinyAsciiStr`.
    ///
    /// The byte array may contain trailing NUL bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use tinystr::tinystr;
    /// use tinystr::TinyAsciiStr;
    ///
    /// assert_eq!(
    ///     TinyAsciiStr::<3>::try_from_raw(*b"GB\0"),
    ///     Ok(tinystr!(3, "GB"))
    /// );
    /// assert_eq!(
    ///     TinyAsciiStr::<3>::try_from_raw(*b"USD"),
    ///     Ok(tinystr!(3, "USD"))
    /// );
    /// assert!(matches!(TinyAsciiStr::<3>::try_from_raw(*b"\0A\0"), Err(_)));
    /// ```
    pub const fn try_from_raw(raw: [u8; N]) -> Result<Self, TinyStrError> {
        Self::from_bytes_inner(&raw, 0, N, true)
    }

    /// Equivalent to [`from_bytes(bytes[start..end])`](Self::from_bytes),
    /// but callable in a `const` context (which range indexing is not).
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

        Ok(Self {
            // SAFETY: `out` only contains ASCII bytes and has same size as `self.bytes`
            bytes: unsafe { AsciiByte::to_ascii_byte_array(&out) },
        })
    }

    // TODO: This function shadows the FromStr trait. Rename?
    #[inline]
    pub const fn from_str(s: &str) -> Result<Self, TinyStrError> {
        Self::from_bytes_inner(s.as_bytes(), 0, s.len(), false)
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        // as_bytes is valid utf8
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        if N <= 4 {
            Aligned4::from_ascii_bytes(&self.bytes).len()
        } else if N <= 8 {
            Aligned8::from_ascii_bytes(&self.bytes).len()
        } else {
            let mut i = 0;
            #[allow(clippy::indexing_slicing)] // < N is safe
            while i < N && self.bytes[i] as u8 != AsciiByte::B0 as u8 {
                i += 1
            }
            i
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes[0] as u8 == AsciiByte::B0 as u8
    }

    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        // Safe because `self.bytes.as_slice()` pointer-casts to `&[u8]`,
        // and changing the length of that slice to self.len() < N is safe.
        unsafe { core::mem::transmute((self.bytes.as_slice().as_ptr(), self.len())) }
    }

    #[inline]
    #[must_use]
    pub const fn all_bytes(&self) -> &[u8; N] {
        // SAFETY: `self.bytes` has same size as [u8; N]
        unsafe { core::mem::transmute(&self.bytes) }
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
            bytes[i] = self.bytes[i] as u8;
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
        Self {
            bytes: AsciiByte::to_ascii_byte_array(&bytes),
        }
    }
}

macro_rules! check_is {
    ($self:ident, $check_int:ident, $check_u8:ident) => {
        if N <= 4 {
            Aligned4::from_ascii_bytes(&$self.bytes).$check_int()
        } else if N <= 8 {
            Aligned8::from_ascii_bytes(&$self.bytes).$check_int()
        } else {
            let mut i = 0;
            // Won't panic because self.bytes has length N
            #[allow(clippy::indexing_slicing)]
            while i < N && $self.bytes[i] as u8 != AsciiByte::B0 as u8 {
                if !($self.bytes[i] as u8).$check_u8() {
                    return false;
                }
                i += 1;
            }
            true
        }
    };
    ($self:ident, $check_int:ident, !$check_u8_0_inv:ident, !$check_u8_1_inv:ident) => {
        if N <= 4 {
            Aligned4::from_ascii_bytes(&$self.bytes).$check_int()
        } else if N <= 8 {
            Aligned8::from_ascii_bytes(&$self.bytes).$check_int()
        } else {
            // Won't panic because N is > 8
            if ($self.bytes[0] as u8).$check_u8_0_inv() {
                return false;
            }
            let mut i = 1;
            // Won't panic because self.bytes has length N
            #[allow(clippy::indexing_slicing)]
            while i < N && $self.bytes[i] as u8 != AsciiByte::B0 as u8 {
                if ($self.bytes[i] as u8).$check_u8_1_inv() {
                    return false;
                }
                i += 1;
            }
            true
        }
    };
    ($self:ident, $check_int:ident, $check_u8_0_inv:ident, $check_u8_1_inv:ident) => {
        if N <= 4 {
            Aligned4::from_ascii_bytes(&$self.bytes).$check_int()
        } else if N <= 8 {
            Aligned8::from_ascii_bytes(&$self.bytes).$check_int()
        } else {
            // Won't panic because N is > 8
            if !($self.bytes[0] as u8).$check_u8_0_inv() {
                return false;
            }
            let mut i = 1;
            // Won't panic because self.bytes has length N
            #[allow(clippy::indexing_slicing)]
            while i < N && $self.bytes[i] as u8 != AsciiByte::B0 as u8 {
                if !($self.bytes[i] as u8).$check_u8_1_inv() {
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
    /// let s1: TinyAsciiStr<4> = "Test".parse().expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "Te3t".parse().expect("Failed to parse.");
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
    /// let s1: TinyAsciiStr<4> = "A15b".parse().expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "[3@w".parse().expect("Failed to parse.");
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
    /// let s1: TinyAsciiStr<4> = "312".parse().expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "3d".parse().expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_numeric());
    /// assert!(!s2.is_ascii_numeric());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_numeric(&self) -> bool {
        check_is!(self, is_ascii_numeric, is_ascii_digit)
    }

    /// Checks if the value is in ASCII lower case.
    ///
    /// All letter characters are checked for case. Non-letter characters are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "teSt".parse().expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "test".parse().expect("Failed to parse.");
    /// let s3: TinyAsciiStr<4> = "001z".parse().expect("Failed to parse.");
    ///
    /// assert!(!s1.is_ascii_lowercase());
    /// assert!(s2.is_ascii_lowercase());
    /// assert!(s3.is_ascii_lowercase());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_lowercase(&self) -> bool {
        check_is!(
            self,
            is_ascii_lowercase,
            !is_ascii_uppercase,
            !is_ascii_uppercase
        )
    }

    /// Checks if the value is in ASCII title case.
    ///
    /// This verifies that the first character is ASCII uppercase and all others ASCII lowercase.
    /// Non-letter characters are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "teSt".parse().expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "Test".parse().expect("Failed to parse.");
    /// let s3: TinyAsciiStr<4> = "001z".parse().expect("Failed to parse.");
    ///
    /// assert!(!s1.is_ascii_titlecase());
    /// assert!(s2.is_ascii_titlecase());
    /// assert!(s3.is_ascii_titlecase());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_titlecase(&self) -> bool {
        check_is!(
            self,
            is_ascii_titlecase,
            !is_ascii_lowercase,
            !is_ascii_uppercase
        )
    }

    /// Checks if the value is in ASCII upper case.
    ///
    /// All letter characters are checked for case. Non-letter characters are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "teSt".parse().expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "TEST".parse().expect("Failed to parse.");
    /// let s3: TinyAsciiStr<4> = "001z".parse().expect("Failed to parse.");
    ///
    /// assert!(!s1.is_ascii_uppercase());
    /// assert!(s2.is_ascii_uppercase());
    /// assert!(!s3.is_ascii_uppercase());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_uppercase(&self) -> bool {
        check_is!(
            self,
            is_ascii_uppercase,
            !is_ascii_lowercase,
            !is_ascii_lowercase
        )
    }

    /// Checks if the value is composed of ASCII alphabetic lower case characters:
    ///
    ///  * U+0061 'a' ..= U+007A 'z',
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "Test".parse().expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "Te3t".parse().expect("Failed to parse.");
    /// let s3: TinyAsciiStr<4> = "teSt".parse().expect("Failed to parse.");
    /// let s4: TinyAsciiStr<4> = "test".parse().expect("Failed to parse.");
    /// let s5: TinyAsciiStr<4> = "001z".parse().expect("Failed to parse.");
    ///
    /// assert!(!s1.is_ascii_alphabetic_lowercase());
    /// assert!(!s2.is_ascii_alphabetic_lowercase());
    /// assert!(!s3.is_ascii_alphabetic_lowercase());
    /// assert!(s4.is_ascii_alphabetic_lowercase());
    /// assert!(!s5.is_ascii_alphabetic_lowercase());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_alphabetic_lowercase(&self) -> bool {
        check_is!(
            self,
            is_ascii_alphabetic_lowercase,
            is_ascii_lowercase,
            is_ascii_lowercase
        )
    }

    /// Checks if the value is composed of ASCII alphabetic, with the first character being ASCII uppercase, and all others ASCII lowercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "Test".parse().expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "Te3t".parse().expect("Failed to parse.");
    /// let s3: TinyAsciiStr<4> = "teSt".parse().expect("Failed to parse.");
    /// let s4: TinyAsciiStr<4> = "test".parse().expect("Failed to parse.");
    /// let s5: TinyAsciiStr<4> = "001z".parse().expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_alphabetic_titlecase());
    /// assert!(!s2.is_ascii_alphabetic_titlecase());
    /// assert!(!s3.is_ascii_alphabetic_titlecase());
    /// assert!(!s4.is_ascii_alphabetic_titlecase());
    /// assert!(!s5.is_ascii_alphabetic_titlecase());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_alphabetic_titlecase(&self) -> bool {
        check_is!(
            self,
            is_ascii_alphabetic_titlecase,
            is_ascii_uppercase,
            is_ascii_lowercase
        )
    }

    /// Checks if the value is composed of ASCII alphabetic upper case characters:
    ///
    ///  * U+0041 'A' ..= U+005A 'Z',
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "Test".parse().expect("Failed to parse.");
    /// let s2: TinyAsciiStr<4> = "Te3t".parse().expect("Failed to parse.");
    /// let s3: TinyAsciiStr<4> = "teSt".parse().expect("Failed to parse.");
    /// let s4: TinyAsciiStr<4> = "TEST".parse().expect("Failed to parse.");
    /// let s5: TinyAsciiStr<4> = "001z".parse().expect("Failed to parse.");
    ///
    /// assert!(!s1.is_ascii_alphabetic_uppercase());
    /// assert!(!s2.is_ascii_alphabetic_uppercase());
    /// assert!(!s3.is_ascii_alphabetic_uppercase());
    /// assert!(s4.is_ascii_alphabetic_uppercase());
    /// assert!(!s5.is_ascii_alphabetic_uppercase());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_ascii_alphabetic_uppercase(&self) -> bool {
        check_is!(
            self,
            is_ascii_alphabetic_uppercase,
            is_ascii_uppercase,
            is_ascii_uppercase
        )
    }
}

macro_rules! to {
    ($self:ident, $to:ident, $later_char_to:ident $(,$first_char_to:ident)?) => {{
        let mut i = 0;
        if N <= 4 {
            let aligned = Aligned4::from_ascii_bytes(&$self.bytes).$to().to_ascii_bytes();
            // Won't panic because self.bytes has length N and aligned has length >= N
            #[allow(clippy::indexing_slicing)]
            while i < N {
                $self.bytes[i] = aligned[i];
                i += 1;
            }
        } else if N <= 8 {
            let aligned = Aligned8::from_ascii_bytes(&$self.bytes).$to().to_ascii_bytes();
            // Won't panic because self.bytes has length N and aligned has length >= N
            #[allow(clippy::indexing_slicing)]
            while i < N {
                $self.bytes[i] = aligned[i];
                i += 1;
            }
        } else {
            // Won't panic because self.bytes has length N
            #[allow(clippy::indexing_slicing)]
            while i < N && $self.bytes[i] as u8 != AsciiByte::B0 as u8 {
                // SAFETY: AsciiByte is repr(u8) and has same size as u8
                unsafe {
                    $self.bytes[i] = core::mem::transmute(
                        ($self.bytes[i] as u8).$later_char_to()
                    );
                }
                i += 1;
            }
            // SAFETY: AsciiByte is repr(u8) and has same size as u8
            $(
                $self.bytes[0] = unsafe {
                    core::mem::transmute(($self.bytes[0] as u8).$first_char_to())
                };
            )?
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
    /// let s1: TinyAsciiStr<4> = "TeS3".parse().expect("Failed to parse.");
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
    /// The first character is converted to ASCII uppercase; the remaining characters
    /// are converted to ASCII lowercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyAsciiStr;
    ///
    /// let s1: TinyAsciiStr<4> = "teSt".parse().expect("Failed to parse.");
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
    /// let s1: TinyAsciiStr<4> = "Tes3".parse().expect("Failed to parse.");
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
        self.as_str()
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
    use rand::distributions::Distribution;
    use rand::distributions::Standard;
    use rand::rngs::SmallRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;

    const STRINGS: [&str; 26] = [
        "Latn",
        "laTn",
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
        "a3z",
        "A3z",
        "A3Z",
        "a3Z",
        "3A",
        "3Z",
        "3a",
        "3z",
        "@@[`{",
        "UK",
        "E12",
    ];

    fn gen_strings(num_strings: usize, allowed_lengths: &[usize]) -> Vec<String> {
        let mut rng = SmallRng::seed_from_u64(2022);
        // Need to do this in 2 steps since the RNG is needed twice
        let string_lengths = core::iter::repeat_with(|| *allowed_lengths.choose(&mut rng).unwrap())
            .take(num_strings)
            .collect::<Vec<usize>>();
        string_lengths
            .iter()
            .map(|len| {
                Standard
                    .sample_iter(&mut rng)
                    .filter(|b: &u8| *b > 0 && *b < 0x80)
                    .take(*len)
                    .collect::<Vec<u8>>()
            })
            .map(|byte_vec| String::from_utf8(byte_vec).expect("All ASCII"))
            .collect()
    }

    fn check_operation<T, F1, F2, const N: usize>(reference_f: F1, tinystr_f: F2)
    where
        F1: Fn(&str) -> T,
        F2: Fn(TinyAsciiStr<N>) -> T,
        T: core::fmt::Debug + core::cmp::PartialEq,
    {
        for s in STRINGS
            .into_iter()
            .map(str::to_owned)
            .chain(gen_strings(100, &[3, 4, 5, 8, 12]))
        {
            let t = match TinyAsciiStr::<N>::from_str(&s) {
                Ok(t) => t,
                Err(TinyStrError::TooLarge { .. }) => continue,
                Err(e) => panic!("{}", e),
            };
            let expected = reference_f(&s);
            let actual = tinystr_f(t);
            assert_eq!(expected, actual, "TinyAsciiStr<{}>: {:?}", N, s);
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
    fn test_is_ascii_lowercase() {
        fn check<const N: usize>() {
            check_operation(
                |s| {
                    s == TinyAsciiStr::<16>::from_str(s)
                        .unwrap()
                        .to_ascii_lowercase()
                        .as_str()
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::is_ascii_lowercase(&t),
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
    fn test_is_ascii_titlecase() {
        fn check<const N: usize>() {
            check_operation(
                |s| {
                    s == TinyAsciiStr::<16>::from_str(s)
                        .unwrap()
                        .to_ascii_titlecase()
                        .as_str()
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::is_ascii_titlecase(&t),
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
    fn test_is_ascii_uppercase() {
        fn check<const N: usize>() {
            check_operation(
                |s| {
                    s == TinyAsciiStr::<16>::from_str(s)
                        .unwrap()
                        .to_ascii_uppercase()
                        .as_str()
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::is_ascii_uppercase(&t),
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
    fn test_is_ascii_alphabetic_lowercase() {
        fn check<const N: usize>() {
            check_operation(
                |s| {
                    // Check alphabetic
                    s.chars().all(|c| c.is_ascii_alphabetic()) &&
                    // Check lowercase
                    s == TinyAsciiStr::<16>::from_str(s)
                        .unwrap()
                        .to_ascii_lowercase()
                        .as_str()
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::is_ascii_alphabetic_lowercase(&t),
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
    fn test_is_ascii_alphabetic_titlecase() {
        fn check<const N: usize>() {
            check_operation(
                |s| {
                    // Check alphabetic
                    s.chars().all(|c| c.is_ascii_alphabetic()) &&
                    // Check titlecase
                    s == TinyAsciiStr::<16>::from_str(s)
                        .unwrap()
                        .to_ascii_titlecase()
                        .as_str()
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::is_ascii_alphabetic_titlecase(&t),
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
    fn test_is_ascii_alphabetic_uppercase() {
        fn check<const N: usize>() {
            check_operation(
                |s| {
                    // Check alphabetic
                    s.chars().all(|c| c.is_ascii_alphabetic()) &&
                    // Check uppercase
                    s == TinyAsciiStr::<16>::from_str(s)
                        .unwrap()
                        .to_ascii_uppercase()
                        .as_str()
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::is_ascii_alphabetic_uppercase(&t),
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
                |t: TinyAsciiStr<N>| TinyAsciiStr::to_ascii_lowercase(t).as_str().to_owned(),
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
                    // Safe because the string is nonempty and an ASCII string
                    unsafe { r.as_bytes_mut()[0].make_ascii_uppercase() };
                    r
                },
                |t: TinyAsciiStr<N>| TinyAsciiStr::to_ascii_titlecase(t).as_str().to_owned(),
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
                |t: TinyAsciiStr<N>| TinyAsciiStr::to_ascii_uppercase(t).as_str().to_owned(),
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
