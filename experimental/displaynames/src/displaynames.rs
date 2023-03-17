// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Displaynames component.

use crate::options::*;
use crate::provider::LanguageDisplayNamesV1Marker;
use crate::provider::RegionDisplayNamesV1Marker;
use icu_locid::{subtags::Region, Locale};
use icu_provider::prelude::*;
use icu_provider::{DataError, DataPayload};

/// Lookup of the locale-specific display names by region code.
///
/// # Example
///
/// ```
/// use icu_displaynames::displaynames::RegionDisplayNames;
/// use icu_displaynames::options::DisplayNamesOptions;
/// use icu_locid::{locale, subtags_region as region};
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = RegionDisplayNames::try_new_unstable(
///     &icu_testdata::unstable(),
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_eq!(display_name.of(region!("AE")), Some("United Arab Emirates"));
/// ```
#[derive(Default)]
pub struct RegionDisplayNames {
    options: DisplayNamesOptions,
    region_data: DataPayload<RegionDisplayNamesV1Marker>,
}

impl RegionDisplayNames {
    /// Creates a new [`RegionDisplayNames`] from locale data and an options bag.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    pub fn try_new_unstable<D: DataProvider<RegionDisplayNamesV1Marker> + ?Sized>(
        data_provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let region_data = data_provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            options,
            region_data,
        })
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        functions: [
            Self::try_new_unstable,
            try_new_with_any_provider,
            try_new_with_buffer_provider
        ]
    );

    /// Returns the display name of a region.
    pub fn of<'a, 'b: 'a, 'c: 'a>(&'b self, region: &'c Region) -> &'a str {
        let data = self.region_data.get();
        match self.options.style {
            Some(Style::Short) => data.short_names.get(&region.into()),
            _ => None,
        }
        .or_else(|| data.names.get(&region.into()))
        .unwrap_or(region.as_str())
    }
}

/// Lookup of the locale-specific display names by language code.
///
/// # Example
///
/// ```
/// use icu_displaynames::displaynames::LanguageDisplayNames;
/// use icu_displaynames::options::DisplayNamesOptions;
/// use icu_locid::locale;
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = LanguageDisplayNames::try_new_unstable(
///     &icu_testdata::unstable(),
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_eq!(display_name.of(&locale!("de")), Some("German"));
/// ```
#[derive(Default)]
pub struct LanguageDisplayNames {
    options: DisplayNamesOptions,
    language_data: DataPayload<LanguageDisplayNamesV1Marker>,
}

impl LanguageDisplayNames {
    /// Creates a new [`LanguageDisplayNames`] from locale data and an options bag.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    pub fn try_new_unstable<D: DataProvider<LanguageDisplayNamesV1Marker> + ?Sized>(
        data_provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let language_data = data_provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            options,
            language_data,
        })
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        functions: [
            Self::try_new_unstable,
            try_new_with_any_provider,
            try_new_with_buffer_provider
        ]
    );

    /// Returns the display name of a locale.
    pub fn of<'a, 'b: 'a, 'c: 'a>(&'b self, langid: &'c Locale) -> Cow<&str> {
        let data = self.language_data.get();
        match self.options.style {
            Some(Style::Short) => data
                .short_names
                .get_by(|bytes| langid.strict_cmp(bytes).reverse()),
            Some(Style::Long) => data
                .long_names
                .get_by(|bytes| langid.strict_cmp(bytes).reverse()),
            Some(Style::Menu) => data
                .menu_names
                .get_by(|bytes| langid.strict_cmp(bytes).reverse()),
            _ => None,
        }
        .or_else(|| {
            data.names
                .get_by(|bytes| langid.strict_cmp(bytes).reverse())
        })
        // TODO: If no match was found, this code should construct a display name
        // by combining the display names of subtags:
        // https://www.unicode.org/reports/tr35/tr35-general.html#Display_Name_Elements:~:text=Spanish%20(Latin%20America)-,es%2DCyrl%2DMX,-Spanish%20(Cyrillic%2C%20Mexico
    }
}
