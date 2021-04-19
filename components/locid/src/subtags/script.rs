// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::errors::ParserError;
use std::str::FromStr;
use tinystr::TinyStr4;

/// A script subtag (examples: `"Latn"`, `"Arab"`, etc.)
///
/// [`Script`] represents a Unicode base language code conformat to the
/// [`unicode_script_id`] field of the Language and Locale Identifier.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Script;
///
/// let script: Script = "Latn".parse()
///     .expect("Failed to parse a script subtag.");
/// ```
///
/// [`unicode_script_id`]: https://unicode.org/reports/tr35/#unicode_script_id
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Script(TinyStr4);

pub const SCRIPT_LENGTH: usize = 4;

impl Script {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Script`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(script, "Latn");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        if v.len() != SCRIPT_LENGTH {
            return Err(ParserError::InvalidSubtag);
        }

        let s = TinyStr4::from_bytes(v).map_err(|_| ParserError::InvalidSubtag)?;
        if !s.is_ascii_alphabetic() {
            return Err(ParserError::InvalidSubtag);
        }
        Ok(Self(s.to_ascii_titlecase()))
    }

    /// Deconstructs the [`Script`] into raw format to be consumed
    /// by [`from_raw_unchecked()`](Script::from_raw_unchecked()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = script.into_raw();
    /// let script = unsafe { Script::from_raw_unchecked(raw) };
    /// assert_eq!(script, "Latn");
    /// ```
    pub fn into_raw(self) -> u32 {
        self.0.into()
    }

    /// Constructor which takes a raw value returned by
    /// [`into_raw`](Script::into_raw()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = script.into_raw();
    /// let script = unsafe { Script::from_raw_unchecked(raw) };
    /// assert_eq!(script, "Latn");
    /// ```
    ///
    /// # Safety
    ///
    /// This function accepts a [`u32`] that is expected to be a valid [`TinyStr4`]
    /// representing a [`Script`] subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: u32) -> Self {
        Self(TinyStr4::new_unchecked(v))
    }

    /// A helper function for displaying
    /// a [`Script`] subtag as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Script;
    ///
    /// let script = Script::from_bytes(b"Latn")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(script.as_str(), "Latn");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// [`Script`] implements [`PartialEq`]`<&`[`str`]`>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Script {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl_writeable_for_single_subtag!(Script, "Mymr");

impl PartialEq<&str> for Script {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'l> From<&'l Script> for &'l str {
    fn from(input: &'l Script) -> Self {
        input.as_str()
    }
}

impl From<Script> for TinyStr4 {
    fn from(input: Script) -> Self {
        input.0
    }
}
