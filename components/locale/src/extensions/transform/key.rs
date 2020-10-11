// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::parser::errors::ParserError;
use std::str::FromStr;
use tinystr::TinyStr4;

/// A key used in a list of [`Fields`].
///
/// The key has to be a two ASCII characters long, with the first
/// character being alphabetic, and the second being a number.
///
/// [`Fields`]: ./struct.Fields.html
///
/// # Examples
///
/// ```
/// use icu_locale::extensions::transform::Key;
///
/// let key1: Key = "k0".parse()
///     .expect("Failed to parse a Key.");
///
/// assert_eq!(key1.as_str(), "k0");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Key(TinyStr4);

const KEY_LENGTH: usize = 2;

impl Key {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::transform::Key;
    ///
    /// let key = Key::from_bytes(b"i0")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(key, "i0");
    /// ```
    pub fn from_bytes(key: &[u8]) -> Result<Self, ParserError> {
        if key.len() != KEY_LENGTH || !key[0].is_ascii_alphabetic() || !key[1].is_ascii_digit() {
            return Err(ParserError::InvalidExtension);
        }
        let tkey = TinyStr4::from_bytes(key).map_err(|_| ParserError::InvalidExtension)?;
        Ok(Self(tkey.to_ascii_lowercase()))
    }

    /// A helper function for displaying
    /// a `Key` as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::transform::Key;
    ///
    /// let key: Key = "s0".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(key.as_str(), "s0");
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

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
