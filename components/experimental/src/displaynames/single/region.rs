// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::*;
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
    payload: DataPayload<LocaleNamesRegionMediumV1>,
}

impl RegionDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the long region display name for a given region and locale using compiled data.
        functions: [
            try_new,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D: DataProvider<LocaleNamesRegionMediumV1> + ?Sized>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError> {
        super::try_new_unstable::<LocaleNamesRegionMediumV1, _>(
            provider,
            prefs,
            LocaleNamesRegionMediumV1::make_attributes(&region),
        )
        .map(|payload| Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the short region display name for a given region and locale using compiled data.
        ///
        /// Falls back to the long name if the short name is not available.
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
        D: DataProvider<LocaleNamesRegionShortV1>
            + DataProvider<LocaleNamesRegionMediumV1>
            + ?Sized,
    {
        super::try_new_short_unstable::<LocaleNamesRegionShortV1, LocaleNamesRegionMediumV1, _>(
            provider,
            prefs,
            LocaleNamesRegionShortV1::make_attributes(&region),
        )
        .map(|payload| Self { payload })
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
