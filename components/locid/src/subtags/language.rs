// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::errors::ParserError;
use core::fmt;
use core::ops::RangeInclusive;
use core::str::FromStr;
use tinystr::{tinystr, TinyStr4};

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
#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Language(Option<TinyStr4>);

const LANGUAGE_LENGTH: RangeInclusive<usize> = 2..=3;
const UND_VALUE: TinyStr4 = tinystr!(4, "und");

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

        let s = match TinyStr4::from_bytes_manual_slice(v, start, end) {
            Ok(s) => s,
            _ => return Err(ParserError::InvalidLanguage),
        };

        if !s.is_ascii_alphabetic() {
            return Err(ParserError::InvalidLanguage);
        }

        let value = s.to_ascii_lowercase();

        if slen == 3
            && value.all_bytes()[0] == UND_VALUE.all_bytes()[0]
            && value.all_bytes()[1] == UND_VALUE.all_bytes()[1]
            && value.all_bytes()[2] == UND_VALUE.all_bytes()[2]
        {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value)))
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
    pub fn into_raw(self) -> Option<[u8; 4]> {
        self.0.as_ref().map(TinyStr4::all_bytes).copied()
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
    /// This function accepts a [`u32`] that is expected to be a valid [`TinyStr4`]
    /// representing a [`Language`] subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: Option<[u8; 4]>) -> Self {
        Self(match v {
            Some(v) => Some(TinyStr4::from_bytes_unchecked(v)),
            None => None,
        })
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
        Self(None)
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
    pub fn as_str(&self) -> &str {
        self.0.as_deref().unwrap_or("und")
    }

    /// Resets the [`Language`] subtag to an empty one.
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
    pub fn clear(&mut self) {
        self.0.take();
    }

    /// Tests if the [`Language`] subtag is empty.
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
    pub fn is_empty(self) -> bool {
        self.0.is_none()
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
        writeable::LengthHint::exact(self.0.map_or(3, |t| t.len()))
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

impl From<Language> for Option<TinyStr4> {
    fn from(input: Language) -> Self {
        input.0.map(Into::into)
    }
}
