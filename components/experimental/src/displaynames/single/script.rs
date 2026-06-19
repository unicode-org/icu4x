// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::*;
use icu_locale::subtags::Script;
use icu_provider::prelude::*;

/// A localized display name for a single script, owned version.
///
/// # Example
///
/// ```
/// use icu::experimental::displaynames::single::ScriptDisplayNameOwned;
/// use icu::locale::{locale, subtags::script};
/// use writeable::assert_writeable_eq;
///
/// let display_name = ScriptDisplayNameOwned::try_new(locale!("en").into(), script!("Xsux"))
///     .expect("Data should load successfully");
///
/// assert_writeable_eq!(display_name, "Sumero-Akkadian Cuneiform");
/// ```
#[derive(Debug)]
pub struct ScriptDisplayNameOwned {
    pub(crate) payload: DataPayload<LocaleNamesScriptMediumV1>,
}

impl ScriptDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, script: Script) -> result: Result<Self, DataError>,
        /// Loads the long script display name for a given script and locale using compiled data.
        functions: [
            try_new,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D: DataProvider<LocaleNamesScriptMediumV1> + ?Sized>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        script: Script,
    ) -> Result<Self, DataError> {
        super::try_new_unstable::<LocaleNamesScriptMediumV1, _>(
            provider,
            prefs,
            LocaleNamesScriptMediumV1::make_attributes(&script),
        )
        .map(|payload| Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, script: Script) -> result: Result<Self, DataError>,
        /// Loads the short script display name for a given script and locale using compiled data.
        ///
        /// Falls back to the long name if the short name is not available.
        ///
        /// # Example
        ///
        /// ```
        /// use icu::experimental::displaynames::{
        ///     DisplayNamesPreferences, single::ScriptDisplayNameOwned,
        /// };
        /// use icu::locale::{locale, subtags::script};
        /// use writeable::assert_writeable_eq;
        ///
        /// let prefs: DisplayNamesPreferences = locale!("en-US").into();
        ///
        /// // "Xsux" has a short display name in en-US
        /// let display_name_short = ScriptDisplayNameOwned::try_new_short(prefs, script!("Xsux"))
        ///     .expect("Data should load successfully");
        /// assert_writeable_eq!(display_name_short, "S-A Cuneiform");
        ///
        /// // "Deva" does not have a short display name, so it falls back to the long display name
        /// let display_name_long = ScriptDisplayNameOwned::try_new_short(prefs, script!("Deva"))
        ///     .expect("Data should load successfully");
        /// assert_writeable_eq!(display_name_long, "Devanagari");
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
        script: Script,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<LocaleNamesScriptShortV1>
            + DataProvider<LocaleNamesScriptMediumV1>
            + ?Sized,
    {
        super::try_new_short_unstable::<LocaleNamesScriptShortV1, LocaleNamesScriptMediumV1, _>(
            provider,
            prefs,
            LocaleNamesScriptShortV1::make_attributes(&script),
        )
        .map(|payload| Self { payload })
    }

    /// Returns a borrowed version of this display name.
    pub fn as_borrowed(&self) -> ScriptDisplayName<'_> {
        ScriptDisplayName {
            value: self.payload.get(),
        }
    }
}

impl_writeable_for_single_display_name_owned!(ScriptDisplayNameOwned);

/// A localized display name for a single script.
#[derive(Debug, Clone, Copy)]
pub struct ScriptDisplayName<'a> {
    value: &'a str,
}

impl_writeable_for_single_display_name_borrowed!(ScriptDisplayName);
