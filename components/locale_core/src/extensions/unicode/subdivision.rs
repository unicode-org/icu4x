// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use crate::parser::ParserError;
use crate::subtags::Region;

impl_tinystr_subtag!(
    /// An subdivision suffix used in a set of [`SubdivisionId`].
    ///
    /// An subdivision suffix has to be a sequence of alphanumerical characters no
    /// shorter than one and no longer than four characters.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{subdivision_suffix, SubdivisionSuffix};
    ///
    /// let ss: SubdivisionSuffix =
    ///     "sct".parse().expect("Failed to parse a SubdivisionSuffix.");
    ///
    /// assert_eq!(ss, subdivision_suffix!("sct"));
    /// ```
    SubdivisionSuffix,
    extensions::unicode,
    subdivision_suffix,
    extensions_unicode_subdivision_suffix,
    1..=4,
    s,
    s.is_ascii_alphanumeric(),
    s.to_ascii_lowercase(),
    s.is_ascii_alphanumeric() && s.is_ascii_lowercase(),
    InvalidExtension,
    ["sct"],
    ["toolooong"],
);

/// A SubDivision Id as defined in [`Unicode Locale Identifier`].
///
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/tr35.html#unicode_subdivision_id
///
/// # Examples
///
/// ```
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub struct SubdivisionId {
    #[doc(hidden)]
    pub region: Region,
    #[doc(hidden)]
    pub suffix: SubdivisionSuffix,
}

impl SubdivisionId {
    #[doc(hidden)]
    pub const fn new(region: Region, suffix: SubdivisionSuffix) -> Self {
        Self { region, suffix }
    }

    #[doc(hidden)]
    pub fn try_from_bytes(input: &[u8]) -> Result<Self, ParserError> {
        let is_alpha = if let Some(b) = input.first() {
            if b.is_ascii_digit() {
                false
            } else if b.is_ascii_alphabetic() {
                true
            } else {
                return Err(ParserError::InvalidExtension);
            }
        } else {
            return Err(ParserError::InvalidExtension);
        };
        let region_len = if is_alpha { 2 } else { 3 };
        if input.len() < region_len + 1 {
            return Err(ParserError::InvalidExtension);
        }
        let (region_bytes, suffix_bytes) = input.split_at(region_len);
        let region =
            Region::try_from_bytes(region_bytes).map_err(|_| ParserError::InvalidExtension)?;
        let suffix = SubdivisionSuffix::try_from_bytes(suffix_bytes)?;
        Ok(Self { region, suffix })
    }
}

impl writeable::Writeable for SubdivisionId {
    #[inline]
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        sink.write_str(self.region.as_str())?;
        sink.write_str(self.suffix.as_str())
    }

    #[inline]
    fn writeable_length_hint(&self) -> writeable::LengthHint {
        todo!()
    }
    #[inline]
    fn write_to_string(&self) -> alloc::borrow::Cow<str> {
        todo!()
    }
}

writeable::impl_display_with_writeable!(SubdivisionId);

impl FromStr for SubdivisionId {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_bytes(s.as_bytes())
    }
}
