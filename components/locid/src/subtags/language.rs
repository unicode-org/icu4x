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
/// let language: Language = "en".parse().expect("Failed to parse a language subtag.");
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
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(transparent)]
pub struct Language(TinyAsciiStr<{ *LANGUAGE_LENGTH.end() }>);

const LANGUAGE_LENGTH: RangeInclusive<usize> = 2..=3;

impl Language {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Language`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// let lang = Language::from_bytes(b"en").expect("Parsing failed.");
    ///
    /// assert_eq!(lang.as_str(), "en");
    /// ```
    pub const fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        Self::from_bytes_manual_slice(v, 0, v.len())
    }

    /// Equivalent to [`from_bytes(bytes[start..end])`](Self::from_bytes),
    /// but callable in a `const` context (which range indexing is not).
    pub const fn from_bytes_manual_slice(
        v: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Self, ParserError> {
        let slen = end - start;

        if slen < *LANGUAGE_LENGTH.start() || *LANGUAGE_LENGTH.end() < slen {
            return Err(ParserError::InvalidLanguage);
        }

        match TinyAsciiStr::from_bytes_manual_slice(v, start, end) {
            Ok(s) if s.is_ascii_alphabetic() => Ok(Self(s.to_ascii_lowercase())),
            _ => Err(ParserError::InvalidLanguage),
        }
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
            2..=3 => s.is_ascii_alphabetic_lowercase(),
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
    /// let lang = Language::from_bytes(b"en").expect("Parsing failed.");
    ///
    /// let raw = lang.into_raw();
    /// let lang = unsafe { Language::from_raw_unchecked(raw) };
    /// assert_eq!(lang.as_str(), "en");
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
    /// let lang = Language::from_bytes(b"en").expect("Parsing failed.");
    ///
    /// let raw = lang.into_raw();
    /// let lang = unsafe { Language::from_raw_unchecked(raw) };
    /// assert_eq!(lang.as_str(), "en");
    /// ```
    ///
    /// # Safety
    ///
    /// This function accepts a [`[u8; 3]`] that is expected to be a valid [`TinyAsciiStr<3>`]
    /// representing a [`Language`] subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: [u8; 3]) -> Self {
        Self(TinyAsciiStr::from_bytes_unchecked(v))
    }

    /// The default undefined language "und". Same as [`default()`](Default::default()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// assert_eq!(Language::default(), Language::UND);
    /// assert_eq!("und", Language::UND.to_string());
    /// ```
    pub const UND: Self = crate::subtags_language!("und");

    /// A helper function for displaying
    /// a [`Language`] subtag as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// let lang = Language::from_bytes(b"en").expect("Parsing failed.");
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
    /// let mut lang: Language = "csb".parse().expect("Parsing failed.");
    ///
    /// assert_eq!(lang.as_str(), "csb");
    ///
    /// lang.clear();
    ///
    /// assert_eq!(lang.as_str(), "und");
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::UND
    }

    /// Tests if the [`Language`] subtag is empty (equal to `"und"`).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// let mut lang: Language = "und".parse().expect("Parsing failed.");
    ///
    /// assert_eq!(lang.is_empty(), true);
    ///
    /// lang.clear();
    ///
    /// assert_eq!(lang.is_empty(), true);
    /// ```
    #[inline]
    pub fn is_empty(self) -> bool {
        self == Self::UND
    }

    /// Compare this `Language` with BCP-47 bytes.
    ///
    /// The return value is equivalent to what would happen if you first converted this
    /// `Language` to a BCP-47 string and then performed a byte comparison.
    ///
    /// This function is case-sensitive and results in a *total order*, so it is appropriate for
    /// binary search. The only argument producing [`Ordering::Equal`](core::cmp::Ordering::Equal)
    /// is `self.to_string()`.
    #[inline]
    pub fn strict_cmp(&self, other: &[u8]) -> core::cmp::Ordering {
        self.as_str().as_bytes().cmp(other)
    }

    /// Compare this `Language` with a potentially unnormalized BCP-47 string.
    ///
    /// The return value is equivalent to what would happen if you first parsed the
    /// BCP-47 string to a `Language` and then performed a structucal comparison.
    ///
    #[inline]
    pub fn normalizing_eq(&self, other: &str) -> bool {
        self.as_str().eq_ignore_ascii_case(other)
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
        Language::UND
    }
}
