use crate::parser::errors::ParserError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use tinystr::TinyStr8;

/// Variant subtag (examples: `"macos"`, `"posix"`, `"1996"` etc.)
///
/// `Variant` represents a Unicode base language code conformat to the
/// [`unicode_variant_id`] field of the Language and Locale Identifier.
///
/// # Examples
///
/// ```
/// use icu_locale::subtags::Variant;
///
/// let variant: Variant = "macos".parse()
///     .expect("Failed to parse a variant subtag.");
/// ```
///
/// [`unicode_variant_id`]: https://unicode.org/reports/tr35/#unicode_variant_id
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub struct Variant(TinyStr8);

const VARIANT_LENGTH: RangeInclusive<usize> = 4..=8;
const VARIANT_NUM_ALPHA_LENGTH: usize = 4;

impl Variant {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Variant`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Variant;
    ///
    /// let variant = Variant::from_bytes(b"posix")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(variant, "posix");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        let slen = v.len();

        if !VARIANT_LENGTH.contains(&slen) {
            return Err(ParserError::InvalidSubtag);
        }

        let s = TinyStr8::from_bytes(v).map_err(|_| ParserError::InvalidSubtag)?;

        if !s.is_ascii_alphanumeric() {
            return Err(ParserError::InvalidSubtag);
        }

        if slen == VARIANT_NUM_ALPHA_LENGTH && !v[0].is_ascii_digit() {
            return Err(ParserError::InvalidSubtag);
        }

        Ok(Self(s.to_ascii_lowercase()))
    }

    /// A helper function for displaying
    /// a `Variant` subtag as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Variant;
    ///
    /// let variant = Variant::from_bytes(b"macos")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(variant.as_str(), "macos");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// `Variant` implements `PartialEq<&str>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Variant {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<&str> for Variant {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<str> for Variant {
    fn eq(&self, other: &str) -> bool {
        *self.as_str() == *other
    }
}

impl<'l> From<&'l Variant> for &'l str {
    fn from(input: &'l Variant) -> Self {
        input.as_str()
    }
}
