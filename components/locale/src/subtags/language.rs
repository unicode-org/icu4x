// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::script::SCRIPT_LENGTH;
use crate::parser::errors::ParserError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use tinystr::TinyStr8;

/// Language subtag (examples: `"en"`, `"csb"`, `"zh"`, `"und"`, etc.)
///
/// `Language` represents a Unicode base language code conformat to the
/// [`unicode_language_id`] field of the Language and Locale Identifier.
///
/// # Examples
///
/// ```
/// use icu_locale::subtags::Language;
///
/// let language: Language = "en".parse()
///     .expect("Failed to parse a language subtag.");
/// ```
///
/// If the `Language` has no value assigned, it serializes to a string `"und"`, which
/// can be then parsed back to an empty `Language` field.
///
/// # Examples
///
/// ```
/// use icu_locale::subtags::Language;
///
/// assert_eq!(Language::default().as_str(), "und");
/// ```
///
/// [`unicode_language_id`]: https://unicode.org/reports/tr35/#unicode_language_id
#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Language(Option<TinyStr8>);

const LANGUAGE_LENGTH: RangeInclusive<usize> = 2..=8;
const UND_VALUE: TinyStr8 = unsafe { TinyStr8::new_unchecked(6_581_877u64) }; // "und"

impl Language {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Language`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Language;
    ///
    /// let lang = Language::from_bytes(b"en")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lang, "en");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        let slen = v.len();

        let s = TinyStr8::from_bytes(v).map_err(|_| ParserError::InvalidLanguage)?;
        if !LANGUAGE_LENGTH.contains(&slen) || slen == SCRIPT_LENGTH || !s.is_ascii_alphabetic() {
            return Err(ParserError::InvalidLanguage);
        }

        let value = s.to_ascii_lowercase();

        if value == UND_VALUE {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value)))
        }
    }

    /// Deconstructs the `Language` into raw format to be consumed
    /// by `from_raw_unchecked`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Language;
    ///
    /// let lang = Language::from_bytes(b"en")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = lang.into_raw();
    /// let lang = unsafe { Language::from_raw_unchecked(raw) };
    /// assert_eq!(lang, "en");
    /// ```
    pub fn into_raw(self) -> Option<u64> {
        self.0.map(Into::<u64>::into)
    }

    /// Constructor which takes a raw value returned by
    /// `into_raw`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Language;
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
    /// This function accepts a `u64` that is expected to be a valid `TinyStr8`
    /// representing a `Language` subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: Option<u64>) -> Self {
        Self(match v {
            Some(v) => Some(TinyStr8::new_unchecked(v)),
            None => None,
        })
    }

    /// A helper function for displaying
    /// a `Language` subtag as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Language;
    ///
    /// let lang = Language::from_bytes(b"en")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lang.as_str(), "en");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// `Language` implements `PartialEq<&str>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_deref().unwrap_or("und")
    }

    /// Resets the `Language` subtag to an empty one.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Language;
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

    /// Tests if the `Language` subtag is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Language;
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

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
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
