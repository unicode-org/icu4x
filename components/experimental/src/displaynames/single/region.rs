// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
    try_load_markers,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::{
    LocaleNamesRegionCoreMediumV1, LocaleNamesRegionCoreShortV1, LocaleNamesRegionExtendedShortV1,
    LocaleNamesRegionMinimalMediumV1, LocaleNamesRegionMinimalShortV1,
};
use icu_locale_core::subtags::Region;
use icu_provider::prelude::*;

macro_rules! table_row {
    (try_new_minimal) => {
        "| [`try_new_minimal`](Self::try_new_minimal) | \"United States\" | ❌ | ❌ |"
    };
    (try_new_minimal_short) => {
        "| [`try_new_minimal_short`](Self::try_new_minimal_short) | \"US\" | ❌ | ❌ |"
    };
    (try_new) => {
        "| [`try_new`](Self::try_new) | \"United States\" | \"France\" | \"대한민국\" |"
    };
    (try_new_short) => {
        "| [`try_new_short`](Self::try_new_short) | \"US\" | \"France\" | \"대한민국\" |"
    };
    (try_new_extended_short) => {
        "| [`try_new_extended_short`](Self::try_new_extended_short) | \"US\" | \"France\" | \"한국\" |"
    };
}

/// A localized display name for a single region, owned version.
///
/// # Constructor Behavior
///
/// There are several constructors, each of which links different data and serve
/// different use cases. The behavior is illustrated in the table below.
///
/// | Constructor | `US` | `FR` | `KR` (`ko`) |
/// | :--- | :--- | :--- | :--- |
#[doc = concat!(table_row!(try_new_minimal), "\n")]
#[doc = concat!(table_row!(try_new_minimal_short), "\n")]
#[doc = concat!(table_row!(try_new), "\n")]
#[doc = concat!(table_row!(try_new_short), "\n")]
#[doc = concat!(table_row!(try_new_extended_short), "\n")]
///
/// > Note: :x: means that the constructor returns an error.
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
        /// Loads the region display name for a given region and locale using compiled data.
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
        ///
        /// Minimal constructors retain data only for high-frequency subtags to minimize data size.
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::experimental::displaynames::single::RegionDisplayNameOwned;
        /// use icu::locale::{locale, subtags::region};
        /// use writeable::assert_writeable_eq;
        ///
        /// let display_name = RegionDisplayNameOwned::try_new_minimal(locale!("en").into(), region!("US"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "United States");
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
        /// Minimal constructors retain data only for high-frequency subtags to minimize data size.
        ///
        /// Falls back to default (medium) length if a short name is not available.
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
        /// Falls back to default (medium) length if a short name is not available.
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
        /// Loads the extended short region display name for a given region and locale using compiled data.
        ///
        /// Extended constructors include additional display name coverage for rare and uncommon subtags.
        ///
        /// Falls back to default (medium) length if a short name is not available.
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::experimental::displaynames::single::RegionDisplayNameOwned;
        /// use icu::locale::{locale, subtags::region};
        /// use writeable::assert_writeable_eq;
        ///
        /// let display_name = RegionDisplayNameOwned::try_new_extended_short(locale!("en").into(), region!("US"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "US");
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
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesRegionExtendedShortV1>
            + DataProvider<LocaleNamesRegionCoreShortV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
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

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::{locale, subtags::region};
    use writeable::Writeable;

    #[test]
    fn test_region_display_name_owned_table() {
        let prefs_en = DisplayNamesPreferences::from(locale!("en"));
        let prefs_ko = DisplayNamesPreferences::from(locale!("ko"));

        let get_row = |f: fn(
            DisplayNamesPreferences,
            Region,
        ) -> Result<RegionDisplayNameOwned, DataError>| {
            vec![
                match f(prefs_en, region!("US")) {
                    Ok(name) => format!("\"{}\"", name.write_to_string()),
                    Err(_) => "❌".to_string(),
                },
                match f(prefs_en, region!("FR")) {
                    Ok(name) => format!("\"{}\"", name.write_to_string()),
                    Err(_) => "❌".to_string(),
                },
                match f(prefs_ko, region!("KR")) {
                    Ok(name) => format!("\"{}\"", name.write_to_string()),
                    Err(_) => "❌".to_string(),
                },
            ]
        };

        let make_row = |name: &str,
                        f: fn(
            DisplayNamesPreferences,
            Region,
        ) -> Result<RegionDisplayNameOwned, DataError>| {
            let row = get_row(f);
            format!("| [`{name}`](Self::{name}) | {} |", row.join(" | "))
        };

        assert_eq!(
            make_row("try_new_minimal", RegionDisplayNameOwned::try_new_minimal),
            table_row!(try_new_minimal)
        );
        assert_eq!(
            make_row(
                "try_new_minimal_short",
                RegionDisplayNameOwned::try_new_minimal_short
            ),
            table_row!(try_new_minimal_short)
        );
        assert_eq!(
            make_row("try_new", RegionDisplayNameOwned::try_new),
            table_row!(try_new)
        );
        assert_eq!(
            make_row("try_new_short", RegionDisplayNameOwned::try_new_short),
            table_row!(try_new_short)
        );
        assert_eq!(
            make_row(
                "try_new_extended_short",
                RegionDisplayNameOwned::try_new_extended_short
            ),
            table_row!(try_new_extended_short)
        );
    }
}
