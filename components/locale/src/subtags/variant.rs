// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
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

    /// Deconstructs the `Variant` into raw format to be consumed
    /// by `from_raw_unchecked`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Variant;
    ///
    /// let variant = Variant::from_bytes(b"posix")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = variant.into_raw();
    /// let variant = unsafe { Variant::from_raw_unchecked(raw) };
    /// assert_eq!(variant, "posix");
    /// ```
    pub fn into_raw(self) -> u64 {
        self.0.into()
    }

    /// Constructor which takes a raw value returned by
    /// `into_raw`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::subtags::Variant;
    ///
    /// let variant = Variant::from_bytes(b"posix")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = variant.into_raw();
    /// let variant = unsafe { Variant::from_raw_unchecked(raw) };
    /// assert_eq!(variant, "posix");
    /// ```
    ///
    /// # Safety
    ///
    /// This function accepts a `u64` that is expected to be a valid `TinyStr8`
    /// representing a `Variant` subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: u64) -> Self {
        Self(TinyStr8::new_unchecked(v))
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
