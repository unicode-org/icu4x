// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use crate::parser::ParseError;
use crate::subtags::{Region, Subtag};

impl_tinystr_subtag!(
    /// A subdivision used in [`RegionAndSubdivision`].
    ///
    /// This subtag represents a specific subdivision code under a given [`Region`].
    /// For example the value of [`RegionAndSubdivision`] may be `gbsct`, where the [`Subdivision`]
    /// is `sct` for Scotland.
    ///
    /// Such a value associated with a key `rg` means that the locale should use Unit Preferences
    /// (default calendar, currency, week data, time cycle, measurement system) for Scotland, even if the
    /// [`LanguageIdentifier`](crate::LanguageIdentifier) is `en-US`.
    ///
    /// A subdivision has to be a sequence of alphanumerical characters no
    /// shorter than one and no longer than four characters.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::extensions::unicode::{subdivision, Subdivision};
    ///
    /// let ss: Subdivision =
    ///     "sct".parse().expect("Failed to parse a Subdivision.");
    ///
    /// assert_eq!(ss, subdivision!("sct"));
    /// ```
    Subdivision,
    extensions::unicode,
    subdivision,
    extensions_unicode_subdivision,
    1..=4,
    s,
    s.is_ascii_alphanumeric(),
    s.to_ascii_lowercase(),
    s.is_ascii_alphanumeric() && s.is_ascii_lowercase(),
    InvalidExtension,
    ["sct"],
    ["toolooong"],
);

impl Subdivision {
    pub(crate) const UNKNOWN: Self = subdivision!("zzzz");

    pub(crate) fn is_unknown(self) -> bool {
        self == Self::UNKNOWN
    }
}

/// A [`RegionAndSubdivision`] represents a `unicode_subdivision_id` as defined in [`Unicode Locale Identifier`].
///
/// It is used in [`Unicode`] extensions:
///  * `rg` - Regional Override
///  * `sd` - Regional Subdivision
///
/// In both cases it is composed of a [`Region`] and a [`Subdivision`] which represents
/// different meaning depending on the key.
///
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/tr35.html#unicode_subdivision_id
/// [`Unicode`]: crate::extensions::unicode::Unicode
///
/// # Examples
///
/// ```
/// use icu::locale::{
///     extensions::unicode::{subdivision, RegionAndSubdivision},
///     subtags::region,
/// };
///
/// let ss = subdivision!("zzzz");
/// let region = region!("gb");
///
/// let si = RegionAndSubdivision::new(region, ss);
///
/// assert_eq!(si.to_string(), "gbzzzz");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
#[non_exhaustive]
pub struct RegionAndSubdivision {
    /// A region field of a `unicode_subdivision_id`.
    pub region: Region,
    /// A subdivision suffix field of a `unicode_subdivision_id`.
    pub suffix: Subdivision,
}

impl RegionAndSubdivision {
    /// Returns a new [`RegionAndSubdivision`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::{
    ///     extensions::unicode::{subdivision, RegionAndSubdivision},
    ///     subtags::region,
    /// };
    ///
    /// let ss = subdivision!("zzzz");
    /// let region = region!("gb");
    ///
    /// let si = RegionAndSubdivision::new(region, ss);
    ///
    /// assert_eq!(si.to_string(), "gbzzzz");
    /// ```
    pub const fn new(region: Region, suffix: Subdivision) -> Self {
        Self { region, suffix }
    }

    /// A constructor which takes a str slice, parses it and
    /// produces a well-formed [`RegionAndSubdivision`].
    #[inline]
    pub fn try_from_str(s: &str) -> Result<Self, ParseError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// See [`Self::try_from_str`]
    pub fn try_from_utf8(code_units: &[u8]) -> Result<Self, ParseError> {
        let is_alpha = code_units
            .first()
            .and_then(|b| {
                b.is_ascii_alphabetic()
                    .then_some(true)
                    .or_else(|| b.is_ascii_digit().then_some(false))
            })
            .ok_or(ParseError::InvalidExtension)?;
        let region_len = if is_alpha { 2 } else { 3 };
        let (region_code_units, suffix_code_units) = code_units
            .split_at_checked(region_len)
            .ok_or(ParseError::InvalidExtension)?;
        let region =
            Region::try_from_utf8(region_code_units).map_err(|_| ParseError::InvalidExtension)?;
        let suffix = Subdivision::try_from_utf8(suffix_code_units)?;
        Ok(Self { region, suffix })
    }

    /// Convert to [`Subtag`]
    pub fn into_subtag(self) -> Subtag {
        let result = self
            .region
            .to_tinystr()
            .to_ascii_lowercase()
            .concat(self.suffix.to_tinystr());
        Subtag::from_tinystr_unvalidated(result)
    }
}

impl writeable::Writeable for RegionAndSubdivision {
    #[inline]
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        sink.write_str(self.region.to_tinystr().to_ascii_lowercase().as_str())?;
        sink.write_str(self.suffix.as_str())
    }

    #[inline]
    fn writeable_length_hint(&self) -> writeable::LengthHint {
        self.region.writeable_length_hint() + self.suffix.writeable_length_hint()
    }
}

writeable::impl_display_with_writeable!(RegionAndSubdivision, #[cfg(feature = "alloc")]);

impl FromStr for RegionAndSubdivision {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_RegionAndSubdivision_fromstr() {
        let si: RegionAndSubdivision = "gbzzzz"
            .parse()
            .expect("Failed to parse RegionAndSubdivision");
        assert_eq!(si.region.to_string(), "GB");
        assert_eq!(si.suffix.to_string(), "zzzz");
        assert_eq!(si.to_string(), "gbzzzz");

        for sample in ["", "gb", "o"] {
            let oe: Result<RegionAndSubdivision, _> = sample.parse();
            assert!(oe.is_err(), "Should fail: {sample}");
        }
    }
}
