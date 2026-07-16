// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
    try_load_markers,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::LocaleNamesVariantExtendedMediumV1;
use icu_locale_core::subtags::Variant;
use icu_provider::prelude::*;

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
        /// Loads the extended variant display name for a given variant and locale using compiled data.
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::experimental::displaynames::single::VariantDisplayNameOwned;
        /// use icu::locale::{locale, subtags::variant};
        /// use writeable::assert_writeable_eq;
        ///
        /// let display_name = VariantDisplayNameOwned::try_new_extended(locale!("en").into(), variant!("posix"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "Computer");
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
        let attrs = LocaleNamesVariantExtendedMediumV1::make_attributes(&variant);
        try_load_markers!(provider, prefs, attrs, [LocaleNamesVariantExtendedMediumV1]).map(
            |payload| Self {
                payload: payload.cast(),
            },
        )
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
    use writeable::Writeable;

    #[test]
    fn test_variant_display_name_owned_table() {
        let prefs = DisplayNamesPreferences::from(locale!("en"));

        let get_row = |f: fn(
            DisplayNamesPreferences,
            Variant,
        ) -> Result<VariantDisplayNameOwned, DataError>| {
            vec![
                match f(prefs, variant!("fonipa")) {
                    Ok(name) => format!("\"{}\"", name.write_to_string()),
                    Err(_) => "❌".to_string(),
                },
                match f(prefs, variant!("posix")) {
                    Ok(name) => format!("\"{}\"", name.write_to_string()),
                    Err(_) => "❌".to_string(),
                },
            ]
        };

        let make_row = |name: &str,
                        f: fn(
            DisplayNamesPreferences,
            Variant,
        ) -> Result<VariantDisplayNameOwned, DataError>| {
            let row = get_row(f);
            format!("| [`{name}`](Self::{name}) | {} |", row.join(" | "))
        };

        assert_eq!(
            make_row(
                "try_new_extended",
                VariantDisplayNameOwned::try_new_extended
            ),
            table_row!(try_new_extended)
        );
    }
}
