// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Displaynames component.

use crate::options::*;
use crate::provider::RegionDisplayNamesV1Marker;
use icu_provider::prelude::*;
use icu_provider::{DataError, DataPayload};
use tinystr::TinyAsciiStr;

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
pub struct DisplayNames {
    options: DisplayNamesOptions,
    data: DataPayload<RegionDisplayNamesV1Marker>,
}

impl DisplayNames {
    /// Creates a new [`DisplayNames`] from locale data and an options bag.
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
        let data = data_provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self { options, data })
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

    /// Returns the display name of the region for a given string.
    /// This function is locale-sensitive.
    pub fn of(&self, region_code: &str) -> Option<&str> {
        match <TinyAsciiStr<3>>::from_str(region_code) {
            Ok(key) => {
                let data = self.data.get();
                match self.options.style {
                    Style::Short => data.short_names.get(&key),
                    _ => data.names.get(&key),
                }
            }
            Err(_) => None,
        }
    }
}
