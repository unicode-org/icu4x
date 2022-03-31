// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use crate::parser::errors::ParserError;
use tinystr::TinyAsciiStr;

/// A key used in a list of [`Keywords`](super::Keywords).
///
/// The key has to be a two ASCII alphanumerical characters long, with the first
/// character being alphanumeric, and the second being alphabetic.
///
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::unicode::Key;
///
/// let key1: Key = "ca".parse()
///     .expect("Failed to parse a Key.");
///
/// assert_eq!(key1, "ca");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Key(TinyAsciiStr<KEY_LENGTH>);

const KEY_LENGTH: usize = 2;

impl Key {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Key`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Key;
    ///
    /// let key = Key::from_bytes(b"ca")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(key, "ca");
    /// ```
    pub fn from_bytes(key: &[u8]) -> Result<Self, ParserError> {
        #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
        if key.len() != KEY_LENGTH
            || !key[0].is_ascii_alphanumeric()
            || !key[1].is_ascii_alphabetic()
        {
            return Err(ParserError::InvalidExtension);
        }

        let key = TinyAsciiStr::from_bytes(key).map_err(|_| ParserError::InvalidSubtag)?;
        Ok(Self(key.to_ascii_lowercase()))
    }

    /// A constructor which takes a TinyAsciiStr and produces a [`Key`]
    /// without doing any checking.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Key;
    /// use tinystr::tinystr;
    ///
    /// let key = Key::from_tinystr_unchecked(tinystr!(2, "ca"));
    ///
    /// assert_eq!(key, "ca");
    /// ```
    pub fn from_tinystr_unchecked(key: TinyAsciiStr<KEY_LENGTH>) -> Self {
        Self(key)
    }

    /// A helper function for displaying
    /// a [`Key`] subtag as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Key;
    ///
    /// let key: Key = "ca".parse().expect("Parsing failed.");
    ///
    /// assert_eq!(key.as_str(), "ca");
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

impl_writeable_for_single_subtag!(Key, "ca");

impl PartialEq<str> for Key {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
