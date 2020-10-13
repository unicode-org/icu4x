// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::parser::errors::ParserError;
use tinystr::TinyStr8;

/// An attribute used in a set of [`Attributes`].
///
/// An attribute has to be a sequence of alphanumerical characters no
/// shorter than three and no longer than eight characters.
///
/// [`Attributes`]: ./struct.Attributes.html
///
/// # Examples
///
/// ```
/// use icu_locale::extensions::unicode::Attribute;
///
/// let attr: Attribute = "buddhist".parse()
///     .expect("Failed to parse an Attribute.");
///
/// assert_eq!(attr, "buddhist");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Attribute(TinyStr8);

const ATTR_LENGTH: RangeInclusive<usize> = 3..=8;

impl Attribute {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Attribute`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::unicode::Attribute;
    ///
    /// let attribute = Attribute::from_bytes(b"foobar")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(attribute, "foobar");
    /// ```
    ///
    /// Notice: No attribute subtags are defined by the current CLDR specification.
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        if !ATTR_LENGTH.contains(&v.len()) {
            return Err(ParserError::InvalidExtension);
        }

        let s = TinyStr8::from_bytes(v).map_err(|_| ParserError::InvalidExtension)?;

        if !s.is_ascii_alphanumeric() {
            return Err(ParserError::InvalidExtension);
        }

        Ok(Self(s.to_ascii_lowercase()))
    }

    /// A helper function for displaying
    /// a `Attribute` subtag as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::unicode::Attribute;
    ///
    /// let attribute = Attribute::from_bytes(b"foobar")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(attribute.as_str(), "foobar");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// `Attribute` implements `PartialEq<&str>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Attribute {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<&str> for Attribute {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
