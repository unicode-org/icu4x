// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
    try_load_markers,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::{
    LocaleNamesRegionCoreMediumV1, LocaleNamesRegionCoreShortV1, LocaleNamesRegionExtendedMediumV1,
    LocaleNamesRegionExtendedShortV1, LocaleNamesRegionMinimalMediumV1,
    LocaleNamesRegionMinimalShortV1,
};
use icu_locale_core::subtags::Region;
use icu_provider::prelude::*;

/// A localized display name for a single region, owned version.
///
/// # Example
///
/// ```
/// use icu::experimental::displaynames::single::RegionDisplayNameOwned;
/// use icu::locale::{locale, subtags::region};
/// use writeable::assert_writeable_eq;
///
/// let display_name = RegionDisplayNameOwned::try_new(locale!("en").into(), region!("US"))
///     .expect("Data should load successfully");
///
/// assert_writeable_eq!(display_name, "United States");
/// ```
#[derive(Debug)]
pub struct RegionDisplayNameOwned {
    pub(crate) payload: DataPayload<LocaleNamesRegionCoreMediumV1>,
}

impl RegionDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the core medium region display name for a given region and locale using compiled data.
        ///
        /// Falls back to minimal medium if core medium is not available.
        functions: [
            try_new,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>,
    {
        let attrs = LocaleNamesRegionCoreMediumV1::make_attributes(&region);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesRegionCoreMediumV1,
                LocaleNamesRegionMinimalMediumV1
            ]
        )
        .map(|payload| Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the minimal region display name for a given region and locale using compiled data.
        functions: [
            try_new_minimal,
            try_new_minimal_with_buffer_provider,
            try_new_minimal_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_minimal)]
    pub fn try_new_minimal_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized + DataProvider<LocaleNamesRegionMinimalMediumV1>,
    {
        let attrs = LocaleNamesRegionMinimalMediumV1::make_attributes(&region);
        try_load_markers!(provider, prefs, attrs, [LocaleNamesRegionMinimalMediumV1]).map(
            |payload| Self {
                payload: payload.cast(),
            },
        )
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the minimal short region display name for a given region and locale using compiled data.
        ///
        /// Falls back to minimal medium if minimal short is not available.
        functions: [
            try_new_minimal_short,
            try_new_minimal_short_with_buffer_provider,
            try_new_minimal_short_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_minimal_short)]
    pub fn try_new_minimal_short_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>,
    {
        let attrs = LocaleNamesRegionMinimalShortV1::make_attributes(&region);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesRegionMinimalShortV1,
                LocaleNamesRegionMinimalMediumV1
            ]
        )
        .map(|payload| Self {
            payload: payload.cast(),
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the short region display name for a given region and locale using compiled data.
        ///
        /// Cascades through fallback down to minimal medium if short is not available.
        ///
        /// # Example
        ///
        /// ```
        /// use icu::experimental::displaynames::{
        ///     DisplayNamesPreferences, single::RegionDisplayNameOwned,
        /// };
        /// use icu::locale::{locale, subtags::region};
        /// use writeable::assert_writeable_eq;
        ///
        /// let prefs: DisplayNamesPreferences = locale!("en-US").into();
        ///
        /// // "US" has a short display name in en-US
        /// let display_name_short = RegionDisplayNameOwned::try_new_short(prefs, region!("US"))
        ///     .expect("Data should load successfully");
        /// assert_writeable_eq!(display_name_short, "US");
        ///
        /// // "FR" does not have a short display name, so it falls back to the long display name
        /// let display_name_long = RegionDisplayNameOwned::try_new_short(prefs, region!("FR"))
        ///     .expect("Data should load successfully");
        /// assert_writeable_eq!(display_name_long, "France");
        /// ```
        functions: [
            try_new_short,
            try_new_short_with_buffer_provider,
            try_new_short_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short)]
    pub fn try_new_short_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesRegionCoreShortV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>,
    {
        let attrs = LocaleNamesRegionCoreShortV1::make_attributes(&region);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesRegionCoreShortV1,
                LocaleNamesRegionMinimalShortV1,
                LocaleNamesRegionCoreMediumV1,
                LocaleNamesRegionMinimalMediumV1
            ]
        )
        .map(|payload| Self {
            payload: payload.cast(),
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the extended region display name for a given region and locale using compiled data.
        functions: [
            try_new_extended,
            try_new_extended_with_buffer_provider,
            try_new_extended_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_extended)]
    pub fn try_new_extended_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesRegionExtendedMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>,
    {
        let attrs = LocaleNamesRegionExtendedMediumV1::make_attributes(&region);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesRegionExtendedMediumV1,
                LocaleNamesRegionCoreMediumV1,
                LocaleNamesRegionMinimalMediumV1
            ]
        )
        .map(|payload| Self {
            payload: payload.cast(),
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the extended short region display name for a given region and locale using compiled data.
        functions: [
            try_new_extended_short,
            try_new_extended_short_with_buffer_provider,
            try_new_extended_short_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_extended_short)]
    pub fn try_new_extended_short_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesRegionExtendedShortV1>
            + DataProvider<LocaleNamesRegionCoreShortV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionExtendedMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>,
    {
        let attrs = LocaleNamesRegionExtendedShortV1::make_attributes(&region);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesRegionExtendedShortV1,
                LocaleNamesRegionCoreShortV1,
                LocaleNamesRegionMinimalShortV1,
                LocaleNamesRegionExtendedMediumV1,
                LocaleNamesRegionCoreMediumV1,
                LocaleNamesRegionMinimalMediumV1
            ]
        )
        .map(|payload| Self {
            payload: payload.cast(),
        })
    }

    /// Returns a borrowed version of this display name.
    pub fn as_borrowed(&self) -> RegionDisplayName<'_> {
        RegionDisplayName {
            value: self.payload.get(),
        }
    }
}

impl_writeable_for_single_display_name_owned!(RegionDisplayNameOwned);

/// A localized display name for a single region.
#[derive(Debug, Clone, Copy)]
pub struct RegionDisplayName<'a> {
    value: &'a str,
}

impl_writeable_for_single_display_name_borrowed!(RegionDisplayName);
