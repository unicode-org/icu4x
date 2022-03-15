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
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        match v.len() {
            REGION_ALPHA_LENGTH => {
                let s = TinyAsciiStr::from_bytes(v).map_err(|_| ParserError::InvalidSubtag)?;
                if !s.is_ascii_alphabetic() {
                    return Err(ParserError::InvalidSubtag);
                }
                Ok(Self(s.to_ascii_uppercase()))
            }
            REGION_NUM_LENGTH => {
                let s = TinyAsciiStr::from_bytes(v).map_err(|_| ParserError::InvalidSubtag)?;
                if !s.is_ascii_numeric() {
                    return Err(ParserError::InvalidSubtag);
                }
                Ok(Self(s))
            }
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
    /// assert!(matches!(Region::try_from_raw(b"US\0"), Ok(_)));
    /// assert!(matches!(Region::try_from_raw(b"419"), Ok(_)));
    /// assert!(matches!(Region::try_from_raw(b"foo"), Err(_)));
    ///
    /// // Unlike the other constructors, this one is case-sensitive:
    /// assert!(matches!(Region::try_from_raw(b"us\0"), Err(_)));
    /// ```
    pub fn try_from_raw(v: &[u8; 3]) -> Result<&Self, ParserError> {
        let s = TinyAsciiStr::<REGION_NUM_LENGTH>::try_from_raw(&v)
            .map_err(|_| ParserError::InvalidSubtag)?;
        let is_valid = match s.len() {
            REGION_ALPHA_LENGTH => s.is_ascii_uppercase(),
            REGION_NUM_LENGTH => s.is_ascii_numeric(),
            _ => false,
        };
        if is_valid {
            // Safe since the bytes are valid
            Ok(unsafe { core::mem::transmute(v) })
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

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_byte_slice() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_byte_slice() checks that the given byte slice has a valid length.
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
#[cfg(feature = "zerovec")]
unsafe impl zerovec::ule::ULE for Region {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
        let it = bytes.chunks_exact(core::mem::size_of::<Self>());
        if it.remainder().len() > 0 {
            return Err(zerovec::ZeroVecError::length::<Self>(bytes.len()));
        }
        for v in it {
            // The following can be removed once `array_chunks` is stabilized.
            let mut a = [0; 3];
            a.copy_from_slice(v);
            if Self::try_from_raw(&a).is_err() {
                return Err(zerovec::ZeroVecError::parse::<Self>());
            }
        }
        Ok(())
    }
}

/// Impl enabling `Region` to be used in a [`ZeroVec`]. Enabled with the `"zerovec"` feature.
///
/// # Example
///
/// ```
/// use icu::locid::subtags::Region;
/// use icu::locid::macros::region;
/// use zerovec::ZeroVec;
///
/// let zv = ZeroVec::<Region>::parse_byte_slice(b"GB\0419001DE\0")
///     .expect("Valid region subtags");
/// assert_eq!(zv.get(1), Some(region!("419")));
///
/// ZeroVec::<Region>::parse_byte_slice(b"invalid")
///     .expect_err("Invalid byte slice");
/// ```
///
/// [`ZeroVec`]: zerovec::ZeroVec
#[cfg(feature = "zerovec")]
impl zerovec::ule::AsULE for Region {
    type ULE = Self;
    fn to_unaligned(self) -> Self::ULE {
        self
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

#[cfg(feature = "zerovec")]
impl<'a> zerovec::maps::ZeroMapKV<'a> for Region {
    type Container = zerovec::ZeroVec<'a, Region>;
    type GetType = Region;
    type OwnedType = Region;
}
