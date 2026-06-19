// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::LocaleNamesVariantMediumV1;
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
/// let display_name = VariantDisplayNameOwned::try_new(locale!("en").into(), variant!("fonipa"))
///     .expect("Data should load successfully");
///
/// assert_writeable_eq!(display_name, "IPA Phonetics");
/// ```
#[derive(Debug)]
pub struct VariantDisplayNameOwned {
    pub(crate) payload: DataPayload<LocaleNamesVariantMediumV1>,
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
    pub fn try_new_unstable<D: DataProvider<LocaleNamesVariantMediumV1> + ?Sized>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        variant: Variant,
    ) -> Result<Self, DataError> {
        super::try_new_unstable::<LocaleNamesVariantMediumV1, _>(
            provider,
            prefs,
            LocaleNamesVariantMediumV1::make_attributes(&variant),
        )
        .map(|payload| Self { payload })
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
