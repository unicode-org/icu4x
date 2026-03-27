// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::DisplayNamesPreferences;
use crate::displaynames::provider::*;
use alloc::borrow::Cow;
use icu_locale_core::subtags::Region;
use icu_provider::prelude::*;

/// A localized region display name.
#[derive(Debug)]
pub struct RegionDisplayName {
    payload: DataPayload<LocaleNamesRegionLongV1>,
}

impl RegionDisplayName {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the long region display name for a given region and locale using compiled data.
        ///
        /// # Example
        ///
        /// ```
        /// use icu::experimental::displaynames::{
        ///     DisplayNamesPreferences, RegionDisplayName,
        /// };
        /// use icu::locale::{locale, subtags::region};
        /// use writeable::assert_writeable_eq;
        ///
        /// let prefs: DisplayNamesPreferences = locale!("en-001").into();
        /// let display_name = RegionDisplayName::try_new(prefs, region!("AE"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "United Arab Emirates");
        /// ```
        functions: [
            try_new,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D: DataProvider<LocaleNamesRegionLongV1> + ?Sized>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError> {
        let locale = LocaleNamesRegionLongV1::make_locale(prefs.locale_preferences);
        let payload = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(region.as_str())
                        .map_err(|_| DataError::custom("Invalid region"))?,
                    &locale,
                ),
                ..Default::default()
            })?
            .payload;
        Ok(Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the short region display name for a given region and locale using compiled data.
        /// It will fall back to the long name if the short name is not available.
        ///
        /// # Example
        ///
        /// ```
        /// use icu::experimental::displaynames::{
        ///     DisplayNamesPreferences, RegionDisplayName,
        /// };
        /// use icu::locale::{locale, subtags::region};
        /// use writeable::assert_writeable_eq;
        ///
        /// let prefs: DisplayNamesPreferences = locale!("en-US").into();
        ///
        /// // "US" has a short display name in en-US
        /// let display_name_us = RegionDisplayName::try_new_short(prefs, region!("US"))
        ///     .expect("Data should load successfully");
        /// assert_writeable_eq!(display_name_us, "US");
        ///
        /// // "AE" does not have a short display name, so it falls back to the long display name
        /// let display_name_ae = RegionDisplayName::try_new_short(prefs, region!("AE"))
        ///     .expect("Data should load successfully");
        /// assert_writeable_eq!(display_name_ae, "United Arab Emirates");
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
        D: DataProvider<LocaleNamesRegionShortV1> + DataProvider<LocaleNamesRegionLongV1> + ?Sized,
    {
        let locale = LocaleNamesRegionShortV1::make_locale(prefs.locale_preferences);
        let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
            DataMarkerAttributes::try_from_str(region.as_str())
                .map_err(|_| DataError::custom("Invalid region"))?,
            &locale,
        );
        let mut metadata = DataRequestMetadata::default();
        metadata.silent = true;
        let result: Result<DataResponse<LocaleNamesRegionShortV1>, DataError> =
            provider.load(DataRequest { id, metadata });

        match result {
            Ok(response) => Ok(Self {
                payload: response.payload.cast(),
            }),
            Err(DataError {
                kind: DataErrorKind::IdentifierNotFound,
                ..
            }) => Self::try_new_unstable(provider, prefs, region),
            Err(e) => Err(e),
        }
    }
}

impl writeable::Writeable for RegionDisplayName {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        sink.write_str(self.payload.get())
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        writeable::LengthHint::exact(self.payload.get().len())
    }

    fn write_to_string(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.payload.get())
    }
}

writeable::impl_display_with_writeable!(RegionDisplayName);
