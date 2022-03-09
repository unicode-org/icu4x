// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::RangeInclusive;
use core::str::FromStr;

use crate::parser::errors::ParserError;
use tinystr::TinyAsciiStr;

/// A single item used in a list of [`Other`](super::Other) extensions.
///
/// The key has to be an ASCII alphanumerical string no shorter than
/// two characters and no longer than eight.
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::other::Key;
///
/// let key1: Key = "Foo".parse()
///     .expect("Failed to parse a Key.");
///
/// assert_eq!(key1.as_str(), "foo");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Key(TinyAsciiStr<{ *KEY_LENGTH.end() }>);

const KEY_LENGTH: RangeInclusive<usize> = 2..=8;

impl Key {
    #[allow(missing_docs)] // TODO(#1028) - Add missing docs.
    pub fn valid_key(v: &[u8]) -> bool {
        KEY_LENGTH.contains(&v.len())
    }

    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Key`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::other::Key;
    ///
    /// let key = Key::from_bytes(b"foobar")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(key.as_str(), "foobar");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        if !Key::valid_key(v) {
            return Err(ParserError::InvalidExtension);
        }

        let s = TinyAsciiStr::from_bytes(v).map_err(|_| ParserError::InvalidExtension)?;

        if !s.is_ascii_alphanumeric() {
            return Err(ParserError::InvalidExtension);
        }

        Ok(Self(s.to_ascii_lowercase()))
    }

    /// A helper function for displaying
    /// a [`Key`] as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::other::Key;
    ///
    /// let key = Key::from_bytes(b"foobar")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(key.as_str(), "foobar");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// [`Key`] implements [`PartialEq`]`<&`[`str`]`>` which allows for direct comparisons.
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

impl_writeable_for_single_subtag!(Key, "foobar");

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
