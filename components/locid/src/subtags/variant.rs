// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::errors::ParserError;
use core::ops::RangeInclusive;
use core::str::FromStr;
use tinystr::TinyAsciiStr;

/// A variant subtag (examples: `"macos"`, `"posix"`, `"1996"` etc.)
///
/// [`Variant`] represents a Unicode base language code conformat to the
/// [`unicode_variant_id`] field of the Language and Locale Identifier.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Variant;
///
/// let variant: Variant = "macos".parse().expect("Failed to parse a variant subtag.");
/// ```
///
/// [`unicode_variant_id`]: https://unicode.org/reports/tr35/#unicode_variant_id
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variant(TinyAsciiStr<{ *VARIANT_LENGTH.end() }>);

const VARIANT_LENGTH: RangeInclusive<usize> = 4..=8;
const VARIANT_NUM_ALPHA_LENGTH: usize = 4;

impl Variant {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Variant`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Variant;
    ///
    /// let variant = Variant::from_bytes(b"posix").expect("Parsing failed.");
    ///
    /// assert_eq!(variant.as_str(), "posix");
    /// ```
    pub const fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        Self::from_bytes_manual_slice(v, 0, v.len())
    }

    /// Equivalent to [`from_bytes(bytes[start..end])`](Self::from_bytes),
    /// but callable in a `const` context (which range indexing is not).
    pub const fn from_bytes_manual_slice(
        v: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Self, ParserError> {
        let slen = end - start;

        if slen < *VARIANT_LENGTH.start() || *VARIANT_LENGTH.end() < slen {
            return Err(ParserError::InvalidSubtag);
        }

        #[allow(clippy::indexing_slicing)] // length checked
        if slen == VARIANT_NUM_ALPHA_LENGTH && !v[start].is_ascii_digit() {
            return Err(ParserError::InvalidSubtag);
        }

        match TinyAsciiStr::from_bytes_manual_slice(v, start, end) {
            Ok(s) if s.is_ascii_alphanumeric() => Ok(Self(s.to_ascii_lowercase())),
            _ => Err(ParserError::InvalidSubtag),
        }
    }

    /// Safely creates a [`Variant`] from a reference to its raw format
    /// as returned by [`Variant::into_raw()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Variant;
    ///
    /// assert!(matches!(Variant::try_from_raw(*b"fonipa\0\0"), Ok(_)));
    /// assert!(matches!(Variant::try_from_raw(*b"1992\0\0\0\0"), Ok(_)));
    /// assert!(matches!(Variant::try_from_raw(*b"foo\0\0\0\0\0"), Err(_)));
    ///
    /// // Unlike the other constructors, this one is case-sensitive:
    /// assert!(matches!(Variant::try_from_raw(*b"POSIX\0\0\0"), Err(_)));
    /// ```
    pub fn try_from_raw(v: [u8; 8]) -> Result<Self, ParserError> {
        let s = TinyAsciiStr::<{ core::mem::size_of::<Self>() }>::try_from_raw(v)
            .map_err(|_| ParserError::InvalidSubtag)?;
        #[allow(clippy::indexing_slicing)] // length checked
        let is_valid = s.is_ascii_alphanumeric()
            && s.is_ascii_lowercase()
            && match s.len() {
                VARIANT_NUM_ALPHA_LENGTH => s.as_bytes()[0].is_ascii_digit(),
                4..=8 => true, // VARIANT_LENGTH
                _ => false,
            };
        if is_valid {
            Ok(Self(s))
        } else {
            Err(ParserError::InvalidSubtag)
        }
    }

    /// Deconstructs the [`Variant`] into raw format to be consumed
    /// by [`from_raw_unchecked()`](Variant::from_raw_unchecked()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Variant;
    ///
    /// let variant = Variant::from_bytes(b"posix").expect("Parsing failed.");
    ///
    /// let raw = variant.into_raw();
    /// let variant = unsafe { Variant::from_raw_unchecked(raw) };
    /// assert_eq!(variant.as_str(), "posix");
    /// ```
    pub fn into_raw(self) -> [u8; 8] {
        *self.0.all_bytes()
    }

    /// Constructor which takes a raw value returned by
    /// [`into_raw()`](Variant::into_raw()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Variant;
    ///
    /// let variant = Variant::from_bytes(b"posix").expect("Parsing failed.");
    ///
    /// let raw = variant.into_raw();
    /// let variant = unsafe { Variant::from_raw_unchecked(raw) };
    /// assert_eq!(variant.as_str(), "posix");
    /// ```
    ///
    /// # Safety
    ///
    /// This function accepts a [u8; 8] that is expected to be a valid [`TinyAsciiStr<8>`]
    /// representing a [`Variant`] subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: [u8; 8]) -> Self {
        Self(TinyAsciiStr::from_bytes_unchecked(v))
    }

    /// A helper function for displaying
    /// a [`Variant`] subtag as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Variant;
    ///
    /// let variant = Variant::from_bytes(b"macos").expect("Parsing failed.");
    ///
    /// assert_eq!(variant.as_str(), "macos");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// [`Variant`] implements [`PartialEq`]`<&`[`str`]`>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Compare this `Variant` with BCP-47 bytes.
    ///
    /// The return value is equivalent to what would happen if you first converted this
    /// `Variant` to a BCP-47 string and then performed a byte comparison.
    ///
    /// This function is case-sensitive and results in a *total order*, so it is appropriate for
    /// binary search. The only argument producing [`Ordering::Equal`](core::cmp::Ordering::Equal)
    /// is `self.to_string()`.
    #[inline]
    pub fn strict_cmp(&self, other: &[u8]) -> core::cmp::Ordering {
        self.as_str().as_bytes().cmp(other)
    }

    /// Compare this `Variant` with a potentially unnormalized BCP-47 string.
    ///
    /// The return value is equivalent to what would happen if you first parsed the
    /// BCP-47 string to a `Variant` and then performed a structucal comparison.
    ///
    #[inline]
    pub fn normalizing_eq(&self, other: &str) -> bool {
        self.as_str().eq_ignore_ascii_case(other)
    }
}

impl FromStr for Variant {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl_writeable_for_single_subtag!(Variant, "posix");

impl<'l> From<&'l Variant> for &'l str {
    fn from(input: &'l Variant) -> Self {
        input.as_str()
    }
}

impl From<Variant> for TinyAsciiStr<8> {
    fn from(input: Variant) -> Self {
        input.0
    }
}
