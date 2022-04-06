// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::errors::ParserError;
use core::str::FromStr;
use tinystr::TinyAsciiStr;

/// A region subtag (examples: `"US"`, `"CN"`, `"AR"` etc.)
///
/// [`Region`] represents a Unicode base language code conformat to the
/// [`unicode_region_id`] field of the Language and Locale Identifier.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Region;
///
/// let region: Region = "DE".parse()
///     .expect("Failed to parse a region subtag.");
/// ```
///
/// [`unicode_region_id`]: https://unicode.org/reports/tr35/#unicode_region_id
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[repr(transparent)]
pub struct Region(TinyAsciiStr<REGION_NUM_LENGTH>);

const REGION_ALPHA_LENGTH: usize = 2;
const REGION_NUM_LENGTH: usize = 3;

impl Region {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Region`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Region;
    ///
    /// let region = Region::from_bytes(b"fr")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(region, "FR");
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
        match end - start {
            REGION_ALPHA_LENGTH => match TinyAsciiStr::from_bytes_manual_slice(v, start, end) {
                Ok(s) if s.is_ascii_alphabetic() => Ok(Self(s.to_ascii_uppercase())),
                _ => Err(ParserError::InvalidSubtag),
            },
            REGION_NUM_LENGTH => match TinyAsciiStr::from_bytes_manual_slice(v, start, end) {
                Ok(s) if s.is_ascii_numeric() => Ok(Self(s)),
                _ => Err(ParserError::InvalidSubtag),
            },
            _ => Err(ParserError::InvalidSubtag),
        }
    }

    /// Safely creates a [`Region`] from a reference to its raw format
    /// as returned by [`Region::into_raw()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Region;
    ///
    /// assert!(matches!(Region::try_from_raw(*b"US\0"), Ok(_)));
    /// assert!(matches!(Region::try_from_raw(*b"419"), Ok(_)));
    /// assert!(matches!(Region::try_from_raw(*b"foo"), Err(_)));
    /// assert!(matches!(Region::try_from_raw(*b"US0"), Err(_)));
    ///
    /// // Unlike the other constructors, this one is case-sensitive:
    /// assert!(matches!(Region::try_from_raw(*b"us\0"), Err(_)));
    /// ```
    pub fn try_from_raw(v: [u8; 3]) -> Result<Self, ParserError> {
        let s = TinyAsciiStr::<{ core::mem::size_of::<Self>() }>::try_from_raw(v)
            .map_err(|_| ParserError::InvalidSubtag)?;
        let is_valid = match s.len() {
            REGION_ALPHA_LENGTH => s.is_ascii_alphabetic() && s.is_ascii_uppercase(),
            REGION_NUM_LENGTH => s.is_ascii_numeric(),
            _ => false,
        };
        if is_valid {
            Ok(Self(s))
        } else {
            Err(ParserError::InvalidSubtag)
        }
    }

    /// Deconstructs the [`Region`] into raw format to be consumed
    /// by [`from_raw_unchecked()`](Region::from_raw_unchecked()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Region;
    ///
    /// let region = Region::from_bytes(b"us")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = region.into_raw();
    /// let region = unsafe { Region::from_raw_unchecked(raw) };
    /// assert_eq!(region, "US");
    /// ```
    pub fn into_raw(self) -> [u8; 3] {
        *self.0.all_bytes()
    }

    /// Constructor which takes a raw value returned by
    /// [`into_raw()`](Region::into_raw()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Region;
    ///
    /// let region = Region::from_bytes(b"us")
    ///     .expect("Parsing failed.");
    ///
    /// let raw = region.into_raw();
    /// let region = unsafe { Region::from_raw_unchecked(raw) };
    /// assert_eq!(region, "US");
    /// ```
    ///
    /// # Safety
    ///
    /// This function accepts a [`[u8; 3]`] that is expected to be a valid [`TinyAsciiStr<3>`]
    /// representing a [`Region`] subtag in canonical syntax.
    pub const unsafe fn from_raw_unchecked(v: [u8; 3]) -> Self {
        Self(TinyAsciiStr::from_bytes_unchecked(v))
    }

    /// A helper function for displaying
    /// a [`Region`] subtag as a `&`[`str`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Region;
    ///
    /// let region = Region::from_bytes(b"it")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(region.as_str(), "IT");
    /// ```
    ///
    /// `Notice`: For many use cases, such as comparison,
    /// [`Region`] implements [`PartialEq`]`<&`[`str`]`>` which allows for direct comparisons.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns true if the Region has an alphabetic code.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::subtags::Region;
    ///
    /// let region = Region::from_bytes(b"us")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(region.is_alphabetic(), true);
    /// ```
    pub fn is_alphabetic(&self) -> bool {
        self.0.is_ascii_alphabetic()
    }
}

impl FromStr for Region {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl_writeable_for_single_subtag!(Region, "GB");

impl PartialEq<&str> for Region {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'l> From<&'l Region> for &'l str {
    fn from(input: &'l Region) -> Self {
        input.as_str()
    }
}

impl From<Region> for TinyAsciiStr<3> {
    fn from(input: Region) -> Self {
        input.0
    }
}
