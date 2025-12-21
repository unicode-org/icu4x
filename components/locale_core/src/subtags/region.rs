// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::extensions::unicode::Value;
use displaydoc::Display;

impl_tinystr_subtag!(
    /// A region subtag (examples: `"US"`, `"CN"`, `"001"` etc.)
    ///
    /// [`Region`] represents a Unicode region code conformant to the
    /// [`unicode_region_subtag`] field of the Language and Locale Identifier,
    /// i.e. an [ISO 3166-1 alpha 2] or [UN M.49] (macro regions only) value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::subtags::Region;
    ///
    /// let region: Region =
    ///     "DE".parse().expect("Failed to parse a region subtag.");
    /// ```
    ///
    /// [`unicode_region_subtag`]: https://unicode.org/reports/tr35/#unicode_region_subtag_validity
    /// [ISO 3166-1 alpha 2]: https://en.wikipedia.org/wiki/ISO_3166-1
    /// [UN M.49]: https://en.wikipedia.org/wiki/UN_M49
    Region,
    subtags,
    region,
    subtags_region,
    2..=3,
    s,
    if s.len() == 2 {
        s.is_ascii_alphabetic()
    } else {
        s.is_ascii_numeric()
    },
    if s.len() == 2 {
        s.to_ascii_uppercase()
    } else {
        s
    },
    if s.len() == 2 {
        s.is_ascii_alphabetic_uppercase()
    } else {
        s.is_ascii_numeric()
    },
    InvalidSubtag,
    ["FR", "009"],
    ["12", "FRA", "b2"],
);

/// An error that occurs when converting a [`Value`] to a [`Region`].
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Display)]
pub enum RegionExtensionError {
    /// The region extension is malformed. For details, see [UTS #35].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::extensions::unicode::value;
    /// use icu::locale::subtags::RegionExtensionError;
    /// use icu::locale::subtags::Region;
    ///
    /// // Value is too short
    /// assert!(matches!(
    ///     Region::try_from_extension_value(&value!("zz")),
    ///     Err(RegionExtensionError::Invalid),
    /// ));
    ///
    /// // Value is too long
    /// assert!(matches!(
    ///     Region::try_from_extension_value(&value!("abcdefg")),
    ///     Err(RegionExtensionError::Invalid),
    /// ));
    ///
    /// // Region is a macroregion (this is not allowed)
    /// assert!(matches!(
    ///     Region::try_from_extension_value(&value!("001abc")),
    ///     Err(RegionExtensionError::Invalid),
    /// ));
    ///
    /// // Value is empty (special value "true")
    /// // See also: https://unicode-org.atlassian.net/browse/CLDR-19163
    /// assert!(matches!(
    ///     Region::try_from_extension_value(&value!("true")),
    ///     Err(RegionExtensionError::Invalid),
    /// ));
    /// ```
    ///
    /// [UTS #35]: https://www.unicode.org/reports/tr35/#RegionOverride
    #[displaydoc("Invalid region or subdivision extension value")]
    Invalid,
}

impl Region {
    /// Returns true if the Region has an alphabetic code.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::subtags::region;
    ///
    /// assert!(region!("us").is_alphabetic());
    /// ```
    pub fn is_alphabetic(&self) -> bool {
        self.0.len() == 2
    }

    /// Extracts the region from a Unicode Region Override or Regional
    /// Subdivision extension value. For details, see [UTS #35].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::locale;
    /// use icu::locale::extensions::unicode::key;
    /// use icu::locale::subtags::Region;
    /// use writeable::assert_writeable_eq;
    ///
    /// // American English with British user preferences
    /// let locale = locale!("en-US-u-rg-gbzzzz");
    ///
    /// let normal_region = locale.id.region.unwrap();
    /// let rg_extension_value = locale.extensions.unicode.keywords.get(&key!("rg")).unwrap();
    /// let override_region = Region::try_from_extension_value(&rg_extension_value).unwrap();
    ///
    /// assert_writeable_eq!(normal_region, "US");
    /// assert_writeable_eq!(override_region, "GB");
    /// ```
    ///
    /// [UTS #35]: https://www.unicode.org/reports/tr35/#RegionOverride
    pub fn try_from_extension_value(value: &Value) -> Result<Self, RegionExtensionError> {
        // From UTS #35:
        // this consists of a unicode_region_subtag for a regular region (not a macroregion),
        // suffixed either by "zzzz" (case is not significant) to designate the region as a
        // whole, or by a unicode_subdivision_suffix to provide more specificity
        let subtag = value
            .as_single_subtag()
            .ok_or(RegionExtensionError::Invalid)?;
        // From the spec language above, length must be 3, 4, 5, or 6
        if subtag.len() < 3 || subtag.len() > 6 {
            return Err(RegionExtensionError::Invalid);
        }
        let maybe_region = subtag.as_tinystr().resize::<2>();
        if !maybe_region.is_ascii_alphabetic() {
            return Err(RegionExtensionError::Invalid);
        }
        Ok(Self(maybe_region.to_ascii_uppercase().resize::<3>()))
    }
}
