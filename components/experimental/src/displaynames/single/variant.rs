// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
    try_load_markers,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::{
    LocaleNamesVariantCoreMediumV1, LocaleNamesVariantExtendedMediumV1,
    LocaleNamesVariantMinimalMediumV1,
};
use icu_locale_core::subtags::Variant;
use icu_provider::prelude::*;

/// A localized display name for a single variant, owned version.
///
/// # Example
///
/// ```
/// use icu::experimental::displaynames::single::VariantDisplayNameOwned;
/// use icu::locale::{locale, subtags::variant};
/// use writeable::assert_writeable_eq;
///
/// let display_name = VariantDisplayNameOwned::try_new_extended(locale!("en").into(), variant!("fonipa"))
///     .expect("Data should load successfully");
///
/// assert_writeable_eq!(display_name, "IPA Phonetics");
/// ```
#[derive(Debug)]
pub struct VariantDisplayNameOwned {
    pub(crate) payload: DataPayload<LocaleNamesVariantCoreMediumV1>,
}

impl VariantDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, variant: Variant) -> result: Result<Self, DataError>,
        /// Loads the variant display name for a given variant and locale using compiled data.
        ///
        /// # Example
        ///
        /// ```
        /// use icu::experimental::displaynames::single::VariantDisplayNameOwned;
        /// use icu::locale::{locale, subtags::variant};
        /// use writeable::assert_writeable_eq;
        ///
        /// let display_name = VariantDisplayNameOwned::try_new(locale!("en").into(), variant!("fonipa"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "IPA Phonetics");
        /// ```
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
        variant: Variant,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesVariantCoreMediumV1>
            + DataProvider<LocaleNamesVariantMinimalMediumV1>,
    {
        let attrs = LocaleNamesVariantCoreMediumV1::make_attributes(&variant);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesVariantCoreMediumV1,
                LocaleNamesVariantMinimalMediumV1
            ]
        )
        .map(|payload| Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, variant: Variant) -> result: Result<Self, DataError>,
        /// Loads the minimal variant display name for a given variant and locale using compiled data.
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
        variant: Variant,
    ) -> Result<Self, DataError>
    where
        D: ?Sized + DataProvider<LocaleNamesVariantMinimalMediumV1>,
    {
        let attrs = LocaleNamesVariantMinimalMediumV1::make_attributes(&variant);
        try_load_markers!(provider, prefs, attrs, [LocaleNamesVariantMinimalMediumV1]).map(
            |payload| Self {
                payload: payload.cast(),
            },
        )
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, variant: Variant) -> result: Result<Self, DataError>,
        /// Loads the extended variant display name for a given variant and locale using compiled data.
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
        variant: Variant,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesVariantCoreMediumV1>
            + DataProvider<LocaleNamesVariantMinimalMediumV1>,
    {
        let attrs = LocaleNamesVariantExtendedMediumV1::make_attributes(&variant);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesVariantExtendedMediumV1,
                LocaleNamesVariantCoreMediumV1,
                LocaleNamesVariantMinimalMediumV1
            ]
        )
        .map(|payload| Self {
            payload: payload.cast(),
        })
    }

    /// Returns a borrowed version of this display name.
    pub fn as_borrowed(&self) -> VariantDisplayName<'_> {
        VariantDisplayName {
            value: self.payload.get(),
        }
    }
}

impl_writeable_for_single_display_name_owned!(VariantDisplayNameOwned);

/// A localized display name for a single variant.
#[derive(Debug, Clone, Copy)]
pub struct VariantDisplayName<'a> {
    value: &'a str,
}

impl_writeable_for_single_display_name_borrowed!(VariantDisplayName);
