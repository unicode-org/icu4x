// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::parser::errors::ParserError;
use tinystr::TinyStr8;

/// A single item used in a list of [`Private`] extensions.
///
/// The key has to be an ASCII alphanumerical string no shorter than
/// one character and no longer than eight.
///
/// # Examples
///
/// ```
/// use icu_locale::extensions::private::Key;
///
/// let key1: Key = "Foo".parse()
///     .expect("Failed to parse a Key.");
///
/// assert_eq!(key1.as_str(), "foo");
/// ```
/// [`Private`]: ./struct.Private.html
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Key(TinyStr8);

const KEY_LENGTH: RangeInclusive<usize> = 1..=8;

impl Key {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::private::Key;
    ///
    /// let key = Key::from_bytes(b"foobar")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(key.as_str(), "foobar");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        if !KEY_LENGTH.contains(&v.len()) {
            return Err(ParserError::InvalidExtension);
        }

        let s = TinyStr8::from_bytes(v).map_err(|_| ParserError::InvalidExtension)?;

        if !s.is_ascii_alphanumeric() {
            return Err(ParserError::InvalidExtension);
        }

        Ok(Self(s.to_ascii_lowercase()))
    }

    /// A helper function for displaying
    /// a `Key` as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::private::Key;
    ///
    /// let key = Key::from_bytes(b"foobar")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(key.as_str(), "foobar");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// `Key` implements `PartialEq<&str>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Key {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
