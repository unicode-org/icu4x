// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Displaynames component.

use crate::options::*;
use crate::provider::LanguageDisplayNamesV1Marker;
use crate::provider::RegionDisplayNamesV1Marker;
use icu_provider::prelude::*;
use icu_provider::{DataError, DataPayload};
use tinystr::TinyAsciiStr;
use zerovec::ule::UnvalidatedStr;

/// Lookup of the terrritory display names by region code.
///
/// # Example
///
/// ```
/// use icu_displaynames::displaynames::DisplayNames;
/// use icu_displaynames::options::DisplayNamesOptions;
/// use icu_locid::locale;
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = DisplayNames::try_new_region_unstable(
///     &icu_testdata::unstable(),
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// let region_code = "AE";
/// assert_eq!(display_name.of(&region_code), Some("United Arab Emirates"));
/// ```
///
/// ```
/// use icu_displaynames::displaynames::DisplayNames;
/// use icu_displaynames::options::DisplayNamesOptions;
/// use icu_locid::locale;
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = DisplayNames::try_new_language_unstable(
///     &icu_testdata::unstable(),
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// let language_code = "aa";
/// assert_eq!(display_name.of(&language_code), Some("Afar"));
/// ```
#[derive(Default)]
pub struct DisplayNames {
    options: DisplayNamesOptions,
    region_data: DataPayload<RegionDisplayNamesV1Marker>,
    language_data: DataPayload<LanguageDisplayNamesV1Marker>,
    displaynames_type: DisplayNamesType,
}

impl DisplayNames {
    /// Creates a new [`DisplayNames`] from locale data and an options bag for type [`RegionDisplayNames`].
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    pub fn try_new_region_unstable<D: DataProvider<RegionDisplayNamesV1Marker> + ?Sized>(
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
            displaynames_type: DisplayNamesType::RegionDisplayNames,
            ..Default::default()
        })
    }

    /// Creates a new [`DisplayNames`] from locale data and an options bag for type [`LanguageDisplayNames`].
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    pub fn try_new_language_unstable<D: DataProvider<LanguageDisplayNamesV1Marker> + ?Sized>(
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
            displaynames_type: DisplayNamesType::LanguageDisplayNames,
            ..Default::default()
        })
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        functions: [
            Self::try_new_region_unstable,
            try_new_region_with_any_provider,
            try_new_region_with_buffer_provider
        ]
    );

    /// Returns the locale specific display name of a region or a language for a given string.
    /// This function is locale-sensitive.
    pub fn of(&self, code: &str) -> Option<&str> {
        match self.displaynames_type {
            DisplayNamesType::RegionDisplayNames => match <TinyAsciiStr<3>>::from_str(code) {
                Ok(key) => {
                    let data = self.region_data.get();
                    match self.options.style {
                        Style::Short => data.short_names.get(&key),
                        _ => data.names.get(&key),
                    }
                }
                Err(_) => None,
            },
            DisplayNamesType::LanguageDisplayNames => {
                let key = UnvalidatedStr::from_str(code);
                let data = self.language_data.get();
                match self.options.style {
                    Style::Short => data.short_names.get(&key),
                    Style::Long => data.long_names.get(&key),
                    Style::Menu => data.menu_names.get(&key),
                    _ => data.names.get(&key),
                }
            }
        }
    }
}

/// An enum for the type of the display names.
#[allow(missing_docs)] // The variants are self explanotory.
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DisplayNamesType {
    LanguageDisplayNames,
    RegionDisplayNames,
}

impl Default for DisplayNamesType {
    fn default() -> Self {
        Self::RegionDisplayNames
    }
}
