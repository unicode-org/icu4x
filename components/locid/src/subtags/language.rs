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
pub struct Language(TinyAsciiStr<{ *LANGUAGE_LENGTH.end() }>);

const LANGUAGE_LENGTH: RangeInclusive<usize> = 2..=3;
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

    /// Returns an `Option<Language>`, which is `None` if the subtag is `"und"`,
    /// and `Some(self)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Language;
    ///
    /// assert!(matches!(
    ///     Language::und().to_option(),
    ///     None));
    /// assert!(matches!(
    ///     Language::from_bytes(b"uk").unwrap().to_option(),
    ///     Some(_)));
    /// ```
    #[inline]
    pub fn to_option(self) -> Option<Language> {
        if !self.is_empty() {
            Some(self)
        } else {
            None
        }
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
