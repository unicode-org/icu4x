// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::RangeInclusive;
use core::str::FromStr;

use crate::parser::errors::ParserError;
use tinystr::TinyAsciiStr;

/// An attribute used in a set of [`Attributes`](super::Attributes).
///
/// An attribute has to be a sequence of alphanumerical characters no
/// shorter than three and no longer than eight characters.
///
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::unicode::Attribute;
///
/// let attr: Attribute = "Buddhist".parse().expect("Failed to parse an Attribute.");
///
/// assert_eq!(attr.as_str(), "buddhist");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Attribute(TinyAsciiStr<{ *ATTR_LENGTH.end() }>);

const ATTR_LENGTH: RangeInclusive<usize> = 3..=8;

impl Attribute {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Attribute`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Attribute;
    ///
    /// let attribute = Attribute::from_bytes(b"fooBar").expect("Parsing failed.");
    ///
    /// assert_eq!(attribute.as_str(), "foobar");
    /// ```
    ///
    /// Notice: No attribute subtags are defined by the current CLDR specification.
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        Self::from_bytes_manual_slice(v, 0, v.len())
    }

    /// Equivalent to [`from_bytes(bytes[start..end])`](Self::from_bytes) but callable in `const`
    /// context.
    pub const fn from_bytes_manual_slice(
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Self, ParserError> {
        let slice_len = end - start;
        if slice_len < *ATTR_LENGTH.start() || slice_len > *ATTR_LENGTH.end() {
            return Err(ParserError::InvalidExtension);
        }

        match TinyAsciiStr::from_bytes_manual_slice(bytes, start, end) {
            Ok(s) if s.is_ascii_alphanumeric() => Ok(Self(s.to_ascii_lowercase())),
            _ => Err(ParserError::InvalidExtension),
        }
    }

    /// A helper function for displaying
    /// a [`Attribute`] subtag as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Attribute;
    ///
    /// let attribute = Attribute::from_bytes(b"foobar").expect("Parsing failed.");
    ///
    /// assert_eq!(attribute.as_str(), "foobar");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// [`Attribute`] implements [`PartialEq`]`<&`[`str`]`>` which allows for direct comparisons.
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

impl_writeable_for_single_subtag!(Attribute, "buddhist");
