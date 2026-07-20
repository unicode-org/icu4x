// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
    try_load_markers,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::{
    LocaleNamesScriptCoreMediumV1, LocaleNamesScriptExtendedMediumV1,
    LocaleNamesScriptExtendedShortV1, LocaleNamesScriptMinimalMediumV1,
};
use icu_locale_core::subtags::Script;
use icu_provider::prelude::*;

macro_rules! table_row {
    (try_new_minimal) => {
        "| [`try_new_minimal`](Self::try_new_minimal) | \"Latin\" | ❌ | ❌ |"
    };
    (try_new) => {
        "| [`try_new`](Self::try_new) | \"Latin\" | \"Unknown Script\" | ❌ |"
    };
    (try_new_extended) => {
        "| [`try_new_extended`](Self::try_new_extended) | \"Latin\" | \"Unknown Script\" | \"Sumero-Akkadian Cuneiform\" |"
    };
    (try_new_extended_short) => {
        "| [`try_new_extended_short`](Self::try_new_extended_short) | \"Latin\" | \"Unknown Script\" | \"S-A Cuneiform\" |"
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
#[doc = concat!(table_row!(try_new_minimal), "\n")]
#[doc = concat!(table_row!(try_new), "\n")]
#[doc = concat!(table_row!(try_new_extended), "\n")]
#[doc = concat!(table_row!(try_new_extended_short), "\n")]
///
/// > Note: :x: means that the constructor returns an error.
///
/// # Example
///
/// ```
/// use icu::experimental::displaynames::single::ScriptDisplayNameOwned;
/// use icu::locale::{locale, subtags::script};
/// use writeable::assert_writeable_eq;
///
/// let display_name = ScriptDisplayNameOwned::try_new(locale!("en").into(), script!("Latn"))
///     .expect("Data should load successfully");
///
/// assert_writeable_eq!(display_name, "Latin");
/// ```
#[derive(Debug)]
pub struct ScriptDisplayNameOwned {
    pub(crate) payload: DataPayload<LocaleNamesScriptCoreMediumV1>,
}

impl ScriptDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, script: Script) -> result: Result<Self, DataError>,
        /// Loads the script display name for a given script and locale using compiled data.
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
        script: Script,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>,
    {
        let attrs = LocaleNamesScriptCoreMediumV1::make_attributes(&script);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesScriptCoreMediumV1,
                LocaleNamesScriptMinimalMediumV1
            ]
        )
        .map(|payload| Self { payload })
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
        /// use icu::experimental::displaynames::single::ScriptDisplayNameOwned;
        /// use icu::locale::{locale, subtags::script};
        /// use writeable::assert_writeable_eq;
        ///
        /// // Minimal script names currently contain no data in CLDR for en
        /// let display_name = ScriptDisplayNameOwned::try_new_minimal(locale!("en").into(), script!("Latn"));
        /// assert!(display_name.is_err());
        /// ```
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
        script: Script,
    ) -> Result<Self, DataError>
    where
        D: ?Sized + DataProvider<LocaleNamesScriptMinimalMediumV1>,
    {
        let attrs = LocaleNamesScriptMinimalMediumV1::make_attributes(&script);
        try_load_markers!(provider, prefs, attrs, [LocaleNamesScriptMinimalMediumV1]).map(
            |payload| Self {
                payload: payload.cast(),
            },
        )
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
        /// use icu::experimental::displaynames::single::ScriptDisplayNameOwned;
        /// use icu::locale::{locale, subtags::script};
        /// use writeable::assert_writeable_eq;
        ///
        /// let display_name = ScriptDisplayNameOwned::try_new_extended(locale!("en").into(), script!("Latn"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "Latin");
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
        script: Script,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptExtendedMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>,
    {
        let attrs = LocaleNamesScriptExtendedMediumV1::make_attributes(&script);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesScriptExtendedMediumV1,
                LocaleNamesScriptCoreMediumV1,
                LocaleNamesScriptMinimalMediumV1
            ]
        )
        .map(|payload| Self {
            payload: payload.cast(),
        })
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
        /// use icu::experimental::displaynames::single::ScriptDisplayNameOwned;
        /// use icu::locale::{locale, subtags::script};
        /// use writeable::assert_writeable_eq;
        ///
        /// let display_name = ScriptDisplayNameOwned::try_new_extended_short(locale!("en").into(), script!("Xsux"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "S-A Cuneiform");
        /// ```
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
        script: Script,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptExtendedShortV1>
            + DataProvider<LocaleNamesScriptExtendedMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>,
    {
        let attrs = LocaleNamesScriptExtendedShortV1::make_attributes(&script);
        try_load_markers!(
            provider,
            prefs,
            attrs,
            [
                LocaleNamesScriptExtendedShortV1,
                LocaleNamesScriptExtendedMediumV1,
                LocaleNamesScriptCoreMediumV1,
                LocaleNamesScriptMinimalMediumV1
            ]
        )
        .map(|payload| Self {
            payload: payload.cast(),
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::{locale, subtags::script};
    use writeable::Writeable;

    #[test]
    fn test_script_display_name_owned_table() {
        let prefs = DisplayNamesPreferences::from(locale!("en"));

        let get_row = |f: fn(
            DisplayNamesPreferences,
            Script,
        ) -> Result<ScriptDisplayNameOwned, DataError>| {
            vec![
                match f(prefs, script!("Latn")) {
                    Ok(name) => format!("\"{}\"", name.write_to_string()),
                    Err(_) => "❌".to_string(),
                },
                match f(prefs, script!("Zzzz")) {
                    Ok(name) => format!("\"{}\"", name.write_to_string()),
                    Err(_) => "❌".to_string(),
                },
                match f(prefs, script!("Xsux")) {
                    Ok(name) => format!("\"{}\"", name.write_to_string()),
                    Err(_) => "❌".to_string(),
                },
            ]
        };

        let make_row = |name: &str,
                        f: fn(
            DisplayNamesPreferences,
            Script,
        ) -> Result<ScriptDisplayNameOwned, DataError>| {
            let row = get_row(f);
            format!("| [`{name}`](Self::{name}) | {} |", row.join(" | "))
        };

        assert_eq!(
            make_row("try_new_minimal", ScriptDisplayNameOwned::try_new_minimal),
            table_row!(try_new_minimal)
        );
        assert_eq!(
            make_row("try_new", ScriptDisplayNameOwned::try_new),
            table_row!(try_new)
        );
        assert_eq!(
            make_row("try_new_extended", ScriptDisplayNameOwned::try_new_extended),
            table_row!(try_new_extended)
        );
        assert_eq!(
            make_row(
                "try_new_extended_short",
                ScriptDisplayNameOwned::try_new_extended_short
            ),
            table_row!(try_new_extended_short)
        );
    }
}
