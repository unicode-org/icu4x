// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
    load_one,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::LocaleNamesVariantExtendedMediumV1;
use icu_locale_core::subtags::Variant;
use icu_provider::prelude::*;

#[inline]
fn make_attributes(subtag: &Variant) -> &DataMarkerAttributes {
    // All variant markers use the same attributes.
    // Valid Variant subtags conform to DataMarkerAttributes syntax.
    DataMarkerAttributes::from_str_or_panic(subtag.as_str())
}

#[inline]
fn make_locale(prefs: DisplayNamesPreferences) -> DataLocale {
    // All variant markers use the same locale
    LocaleNamesVariantExtendedMediumV1::make_locale(prefs.locale_preferences)
}

macro_rules! table_row {
    (try_new_extended) => {
        "| [`try_new_extended`](Self::try_new_extended) | \"IPA Phonetics\" | \"Computer\" |"
    };
}

/// A localized display name for a single variant, owned version.
///
/// # Constructor Behavior
///
/// There is currently just one constructor, which is named "extended"
/// since there are no variants with guaranteed display names.
///
/// | Constructor | `fonipa` | `posix` |
/// | :--- | :--- | :--- |
#[doc = concat!(table_row!(try_new_extended), "\n")]
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
    pub(crate) payload: DataPayload<LocaleNamesVariantExtendedMediumV1>,
}

impl VariantDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, variant: Variant) -> result: Result<Self, DataError>,
        /// Loads the display name for a given variant in a given locale using compiled data.
        ///
        /// # Examples
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
        D: ?Sized + DataProvider<LocaleNamesVariantExtendedMediumV1>,
    {
        let attrs = make_attributes(&variant);
        let locale = make_locale(prefs);
        let payload =
            load_one::<LocaleNamesVariantExtendedMediumV1, _, _>(provider, &locale, attrs)?
                .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
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

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::{locale, subtags::variant};

    #[test]
    fn test_variant_display_name_owned_table() {
        let prefs_en = DisplayNamesPreferences::from(locale!("en"));
        let inputs = [variant!("fonipa"), variant!("posix")];

        macro_rules! check_row {
            ($constructor:ident) => {
                let items = inputs.iter().map(|id| {
                    VariantDisplayNameOwned::$constructor(prefs_en, *id)
                        .map(|name| Ok::<_, ()>(name.to_string()))
                });
                assert_eq!(
                    super::super::format_table_row(stringify!($constructor), items),
                    table_row!($constructor)
                );
            };
        }

        check_row!(try_new_extended);
    }
}
