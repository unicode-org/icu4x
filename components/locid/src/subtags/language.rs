// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::errors::ParserError;
use core::fmt;
use core::ops::RangeInclusive;
use core::str::FromStr;
use tinystr::TinyAsciiStr;

/// A language subtag (examples: `"en"`, `"csb"`, `"zh"`, `"und"`, etc.)
///
/// [`Language`] represents a Unicode base language code conformat to the
/// [`unicode_language_id`] field of the Language and Locale Identifier.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Language;
///
/// let language: Language = "en".parse()
///     .expect("Failed to parse a language subtag.");
/// ```
///
/// If the [`Language`] has no value assigned, it serializes to a string `"und"`, which
/// can be then parsed back to an empty [`Language`] field.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Language;
///
/// assert_eq!(Language::default().as_str(), "und");
/// ```
///
/// `Notice`: ICU4X uses a narrow form of language subtag of 2-3 characters.
/// The specification allows language subtag to optionally also be 5-8 characters
/// but that form has not been used and ICU4X does not support it right now.
///
/// [`unicode_language_id`]: https://unicode.org/reports/tr35/#unicode_language_id
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
#[repr(transparent)]
pub struct Language(TinyAsciiStr<{ *LANGUAGE_LENGTH.end() }>);

const LANGUAGE_LENGTH: RangeInclusive<usize> = 2..=3;
// TODO(#348): Change this to invoke a const function.
// Safe because "und" is a valid language subtag
const UND: Language = Language(unsafe { TinyAsciiStr::from_bytes_unchecked(*b"und") });

impl Language {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Language`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// let lang = Language::from_bytes(b"en")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lang, "en");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        let slen = v.len();

        if !LANGUAGE_LENGTH.contains(&slen) {
            return Err(ParserError::InvalidLanguage);
        }

        let s = TinyAsciiStr::from_bytes(v).map_err(|_| ParserError::InvalidLanguage)?;

        if !s.is_ascii_alphabetic() {
            return Err(ParserError::InvalidLanguage);
        }

        let value = s.to_ascii_lowercase();

        Ok(Self(value))
    }

    /// Safely creates a [`Language`] from a reference to its raw format
    /// as returned by [`Language::into_raw()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// assert!(matches!(Language::try_from_raw(*b"de\0"), Ok(_)));
    /// assert!(matches!(Language::try_from_raw(*b"ars"), Ok(_)));
    /// assert!(matches!(Language::try_from_raw(*b"419"), Err(_)));
    ///
    /// // Unlike the other constructors, this one is case-sensitive:
    /// assert!(matches!(Language::try_from_raw(*b"EN\0"), Err(_)));
    /// ```
    pub fn try_from_raw(v: [u8; 3]) -> Result<Self, ParserError> {
        let s = TinyAsciiStr::<{ core::mem::size_of::<Self>() }>::try_from_raw(v)
            .map_err(|_| ParserError::InvalidSubtag)?;
        let is_valid = match s.len() {
            // LANGUAGE_LENGTH
            2..=3 => s.is_ascii_alphabetic() && s.is_ascii_lowercase(),
            _ => false,
        };
        if is_valid {
            Ok(Self(s))
        } else {
            Err(ParserError::InvalidSubtag)
        }
    }

    /// Deconstructs the [`Language`] into raw format to be consumed
    /// by [`from_raw_unchecked()`](Language::from_raw_unchecked()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// let lang = Language::from_bytes(b"en")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = lang.into_raw();
    /// let lang = unsafe { Language::from_raw_unchecked(raw) };
    /// assert_eq!(lang, "en");
    /// ```
    pub fn into_raw(self) -> [u8; 3] {
        *self.0.all_bytes()
    }

    /// Constructor which takes a raw value returned by
    /// [`into_raw()`](Language::into_raw()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// let lang = Language::from_bytes(b"en")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = lang.into_raw();
    /// let lang = unsafe { Language::from_raw_unchecked(raw) };
    /// assert_eq!(lang, "en");
    /// ```
    ///
    /// # Safety
    ///
    /// This function accepts a [`[u8; 3]`] that is expected to be a valid [`TinyAsciiStr<3>`]
    /// representing a [`Language`] subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: [u8; 3]) -> Self {
        Self(TinyAsciiStr::from_bytes_unchecked(v))
    }

    /// Returns the default undefined language "und". Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// const language: Language = Language::und();
    /// assert_eq!(Language::default(), language);
    /// assert_eq!("und", language.to_string());
    /// ```
    #[inline]
    pub const fn und() -> Self {
        UND
    }

    /// A helper function for displaying
    /// a [`Language`] subtag as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// let lang = Language::from_bytes(b"en")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lang.as_str(), "en");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// [`Language`] implements [`PartialEq`]`<&`[`str`]`>` which allows for direct comparisons.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Resets the [`Language`] subtag to an empty one (equal to `"und"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// let mut lang: Language = "csb".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lang.as_str(), "csb");
    ///
    /// lang.clear();
    ///
    /// assert_eq!(lang.as_str(), "und");
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        *self = UND
    }

    /// Tests if the [`Language`] subtag is empty (equal to `"und"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// let mut lang: Language = "und".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lang.is_empty(), true);
    ///
    /// lang.clear();
    ///
    /// assert_eq!(lang.is_empty(), true);
    /// ```
    #[inline]
    pub fn is_empty(self) -> bool {
        self == UND
    }
}

impl FromStr for Language {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl writeable::Writeable for Language {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str(self.as_str())
    }

    #[inline]
    fn write_len(&self) -> writeable::LengthHint {
        writeable::LengthHint::exact(self.0.len())
    }
}

#[test]
fn test_writeable() {
    writeable::assert_writeable_eq!(&Language::from_str("aa").unwrap(), "aa");
    writeable::assert_writeable_eq!(&Language::from_str("xyz").unwrap(), "xyz");
    writeable::assert_writeable_eq!(&Language::from_str("und").unwrap(), "und");
}

impl PartialEq<&str> for Language {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'l> From<&'l Language> for &'l str {
    fn from(input: &'l Language) -> Self {
        input.as_str()
    }
}

impl From<Language> for TinyAsciiStr<3> {
    fn from(input: Language) -> Self {
        input.0
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::und()
    }
}

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_byte_slice() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_byte_slice() checks that the given byte slice has a valid length.
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
#[cfg(feature = "zerovec")]
unsafe impl zerovec::ule::ULE for Language {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
        let it = bytes.chunks_exact(core::mem::size_of::<Self>());
        if !it.remainder().is_empty() {
            return Err(zerovec::ZeroVecError::length::<Self>(bytes.len()));
        }
        for v in it {
            // The following can be removed once `array_chunks` is stabilized.
            let mut a = [0; core::mem::size_of::<Self>()];
            a.copy_from_slice(v);
            if Self::try_from_raw(a).is_err() {
                return Err(zerovec::ZeroVecError::parse::<Self>());
            }
        }
        Ok(())
    }
}

/// Impl enabling `Language` to be used in a [`ZeroVec`]. Enabled with the `"zerovec"` feature.
///
/// # Example
///
/// ```
/// use icu::locid::subtags::Language;
/// use icu::locid::macros::language;
/// use zerovec::ZeroVec;
///
/// let zv = ZeroVec::<Language>::parse_byte_slice(b"de\0fr\0arsar\0")
///     .expect("Valid language subtags");
/// assert_eq!(zv.get(1), Some(language!("fr")));
///
/// ZeroVec::<Language>::parse_byte_slice(b"invalid")
///     .expect_err("Invalid byte slice");
/// ```
///
/// [`ZeroVec`]: zerovec::ZeroVec
#[cfg(feature = "zerovec")]
impl zerovec::ule::AsULE for Language {
    type ULE = Self;
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

#[cfg(feature = "zerovec")]
impl<'a> zerovec::maps::ZeroMapKV<'a> for Language {
    type Container = zerovec::ZeroVec<'a, Language>;
    type GetType = Language;
    type OwnedType = Language;
}
