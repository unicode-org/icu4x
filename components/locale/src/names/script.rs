// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::DisplayNamesPreferences;
use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
    load_one,
};
use crate::provider::names::{
    LocaleNamesScriptMediumHeavyV1, LocaleNamesScriptMediumLightV1, LocaleNamesScriptMediumTinyV1,
    LocaleNamesScriptShortHeavyV1,
};
use icu_locale_core::subtags::Script;
use icu_provider::prelude::*;

#[inline]
fn make_attributes(subtag: &Script) -> &DataMarkerAttributes {
    // All script markers use the same attributes.
    // Valid Script subtags conform to DataMarkerAttributes syntax.
    DataMarkerAttributes::from_str_or_panic(subtag.as_str())
}

#[inline]
fn make_locale(prefs: DisplayNamesPreferences) -> DataLocale {
    // All script markers use the same locale
    LocaleNamesScriptMediumTinyV1::make_locale(prefs.locale_preferences)
}

macro_rules! table_row {
    (try_new_tiny) => {
        "| [`try_new_tiny`](Self::try_new_tiny) | \"Latin\" | ❌ | ❌ |"
    };
    (try_new_light) => {
        "| [`try_new_light`](Self::try_new_light) | \"Latin\" | \"Unknown Script\" | ❌ |"
    };
    (try_new_heavy) => {
        "| [`try_new_heavy`](Self::try_new_heavy) | \"Latin\" | \"Unknown Script\" | \"Sumero-Akkadian Cuneiform\" |"
    };
    (try_new_short_heavy) => {
        "| [`try_new_short_heavy`](Self::try_new_short_heavy) | \"Latin\" | \"Unknown Script\" | \"S-A Cuneiform\" |"
    };
}

/// A localized display name for a single script, owned version.
///
/// # Constructor Behavior
///
/// There are several constructors, each of which links different data and serve
/// different use cases. The behavior is illustrated in the table below.
///
/// | Constructor | `Latn` | `Zzzz` | `Xsux` |
/// | :--- | :--- | :--- | :--- |
#[doc = concat!(table_row!(try_new_tiny), "\n")]
#[doc = concat!(table_row!(try_new_light), "\n")]
#[doc = concat!(table_row!(try_new_heavy), "\n")]
#[doc = concat!(table_row!(try_new_short_heavy), "\n")]
///
/// > Note: :x: means that the constructor returns an error.
///
/// # Example
///
/// ```
/// use icu::locale::names::ScriptDisplayName;
/// use icu::locale::{locale, subtags::script};
/// use writeable::assert_writeable_eq;
///
/// let display_name = ScriptDisplayName::try_new_light(locale!("en").into(), script!("Latn"))
///     .expect("Data should load successfully");
///
/// assert_writeable_eq!(display_name, "Latin");
/// ```
#[derive(Debug)]
pub struct ScriptDisplayName {
    pub(crate) payload: DataPayload<LocaleNamesScriptMediumLightV1>,
}

impl ScriptDisplayName {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, script: Script) -> result: Result<Self, DataError>,
        /// Loads the script display name for a given script and locale using compiled data.
        functions: [
            try_new_light,
            try_new_light_with_buffer_provider,
            try_new_light_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_light)]
    pub fn try_new_light_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        script: Script,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptMediumLightV1>
            + DataProvider<LocaleNamesScriptMediumTinyV1>,
    {
        let attrs = make_attributes(&script);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesScriptMediumLightV1, _, _>(provider, &locale, attrs)?
            .map_or_else(
                || load_one::<LocaleNamesScriptMediumTinyV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, script: Script) -> result: Result<Self, DataError>,
        /// Loads the minimal script display name for a given script and locale using compiled data.
        ///
        /// The `minimal` constructor links an extremely limited amount of data: for example,
        /// only those scripts associated with the formatting locale.
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locale::names::ScriptDisplayName;
        /// use icu::locale::{locale, subtags::script};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Minimal script names contain Latn for en
        /// let display_name = ScriptDisplayName::try_new_tiny(locale!("en").into(), script!("Latn")).unwrap();
        /// assert_writeable_eq!(display_name, "Latin");
        /// ```
        functions: [
            try_new_tiny,
            try_new_tiny_with_buffer_provider,
            try_new_tiny_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_tiny)]
    pub fn try_new_tiny_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        script: Script,
    ) -> Result<Self, DataError>
    where
        D: ?Sized + DataProvider<LocaleNamesScriptMediumTinyV1>,
    {
        let attrs = make_attributes(&script);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesScriptMediumTinyV1, _, _>(provider, &locale, attrs)?
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, script: Script) -> result: Result<Self, DataError>,
        /// Loads the extended script display name for a given script and locale using compiled data.
        ///
        /// The `extended` constructor includes additional data coverage for subtags that are less commonly formatted in the target locale.
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locale::names::ScriptDisplayName;
        /// use icu::locale::{locale, subtags::script};
        /// use writeable::assert_writeable_eq;
        ///
        /// let display_name = ScriptDisplayName::try_new_heavy(locale!("en").into(), script!("Latn"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "Latin");
        /// ```
        functions: [
            try_new_heavy,
            try_new_heavy_with_buffer_provider,
            try_new_heavy_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_heavy)]
    pub fn try_new_heavy_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        script: Script,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptMediumHeavyV1>
            + DataProvider<LocaleNamesScriptMediumLightV1>
            + DataProvider<LocaleNamesScriptMediumTinyV1>,
    {
        let attrs = make_attributes(&script);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesScriptMediumHeavyV1, _, _>(provider, &locale, attrs)?
            .map_or_else(
                || load_one::<LocaleNamesScriptMediumLightV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .map_or_else(
                || load_one::<LocaleNamesScriptMediumTinyV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, script: Script) -> result: Result<Self, DataError>,
        /// Loads the extended short script display name for a given script and locale using compiled data.
        ///
        /// The `extended` constructor includes additional data coverage for subtags that are less commonly formatted in the target locale.
        ///
        /// Falls back to default (medium) length if a short name is not available.
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locale::names::ScriptDisplayName;
        /// use icu::locale::{locale, subtags::script};
        /// use writeable::assert_writeable_eq;
        ///
        /// let display_name = ScriptDisplayName::try_new_short_heavy(locale!("en").into(), script!("Xsux"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "S-A Cuneiform");
        /// ```
        functions: [
            try_new_short_heavy,
            try_new_short_heavy_with_buffer_provider,
            try_new_short_heavy_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short_heavy)]
    pub fn try_new_short_heavy_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        script: Script,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptShortHeavyV1>
            + DataProvider<LocaleNamesScriptMediumHeavyV1>
            + DataProvider<LocaleNamesScriptMediumLightV1>
            + DataProvider<LocaleNamesScriptMediumTinyV1>,
    {
        let attrs = make_attributes(&script);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesScriptShortHeavyV1, _, _>(provider, &locale, attrs)?
            .map_or_else(
                || load_one::<LocaleNamesScriptMediumHeavyV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .map_or_else(
                || load_one::<LocaleNamesScriptMediumLightV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .map_or_else(
                || load_one::<LocaleNamesScriptMediumTinyV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
    }

    /// Returns a borrowed version of this display name.
    pub fn as_borrowed(&self) -> ScriptDisplayNameBorrowed<'_> {
        ScriptDisplayNameBorrowed {
            value: self.payload.get(),
        }
    }
}

impl_writeable_for_single_display_name_owned!(ScriptDisplayName);

/// A localized display name for a single script.
#[derive(Debug, Clone, Copy)]
pub struct ScriptDisplayNameBorrowed<'a> {
    value: &'a str,
}

impl_writeable_for_single_display_name_borrowed!(ScriptDisplayNameBorrowed);

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::{locale, subtags::script};

    #[test]
    fn test_script_display_name_owned_table() {
        let prefs_en = DisplayNamesPreferences::from(locale!("en"));
        let inputs = [script!("Latn"), script!("Zzzz"), script!("Xsux")];

        macro_rules! check_row {
            ($constructor:ident) => {
                let items = inputs.iter().map(|id| {
                    ScriptDisplayName::$constructor(prefs_en, *id)
                        .map(|name| Ok::<_, ()>(name.to_string()))
                });
                assert_eq!(
                    super::super::format_table_row(stringify!($constructor), items),
                    table_row!($constructor)
                );
            };
        }

        check_row!(try_new_tiny);
        check_row!(try_new_light);
        check_row!(try_new_heavy);
        check_row!(try_new_short_heavy);
    }
}
