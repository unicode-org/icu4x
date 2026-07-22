// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::{
    LocaleNamesRegionCoreMediumV1, LocaleNamesRegionCoreShortV1, LocaleNamesRegionMinimalMediumV1,
    LocaleNamesRegionMinimalShortV1,
};
use crate::displaynames::single::load_one;
use icu_locale_core::subtags::Region;
use icu_provider::prelude::*;

#[inline]
fn make_attributes(subtag: &Region) -> &DataMarkerAttributes {
    // All region markers use the same attributes.
    // Valid Region subtags conform to DataMarkerAttributes syntax.
    DataMarkerAttributes::from_str_or_panic(subtag.as_str())
}

#[inline]
fn make_locale(prefs: DisplayNamesPreferences) -> DataLocale {
    // All region markers use the same locale
    LocaleNamesRegionMinimalMediumV1::make_locale(prefs.locale_preferences)
}

macro_rules! table_row {
    (try_new_minimal) => {
        "| [`try_new_minimal`](Self::try_new_minimal) | \"United States\" | ❌ |"
    };
    (try_new_minimal_short) => {
        "| [`try_new_minimal_short`](Self::try_new_minimal_short) | \"US\" | ❌ |"
    };
    (try_new) => {
        "| [`try_new`](Self::try_new) | \"United States\" | \"Andorra\" |"
    };
    (try_new_short) => {
        "| [`try_new_short`](Self::try_new_short) | \"US\" | \"Andorra\" |"
    };
}

/// A localized display name for a single region, owned version.
///
/// # Constructor Behavior
///
/// There are several constructors, each of which links different data and serve
/// different use cases. The behavior is illustrated in the table below.
///
/// | Constructor | `US` | `AD` |
/// | :--- | :--- | :--- |
#[doc = concat!(table_row!(try_new_minimal), "\n")]
#[doc = concat!(table_row!(try_new_minimal_short), "\n")]
#[doc = concat!(table_row!(try_new), "\n")]
#[doc = concat!(table_row!(try_new_short), "\n")]
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
        let attrs = make_attributes(&region);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesRegionCoreMediumV1, _, _>(provider, &locale, attrs)?
            .map_or_else(
                || load_one::<LocaleNamesRegionMinimalMediumV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the minimal region display name for a given region and locale using compiled data.
        ///
        /// The `minimal` constructor links an extremely limited amount of data: for example,
        /// only those regions where the formatting locale is spoken.
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
        let attrs = make_attributes(&region);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesRegionMinimalMediumV1, _, _>(provider, &locale, attrs)?
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the minimal short region display name for a given region and locale using compiled data.
        ///
        /// The `minimal` constructor links an extremely limited amount of data: for example,
        /// only those regions where the formatting locale is spoken.
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
        let attrs = make_attributes(&region);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesRegionMinimalShortV1, _, _>(provider, &locale, attrs)?
            .map_or_else(
                || load_one::<LocaleNamesRegionMinimalMediumV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
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
        /// // "AD" does not have a short display name, so it falls back to the long display name
        /// let display_name_long = RegionDisplayNameOwned::try_new_short(prefs, region!("AD"))
        ///     .expect("Data should load successfully");
        /// assert_writeable_eq!(display_name_long, "Andorra");
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
        let attrs = make_attributes(&region);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesRegionCoreShortV1, _, _>(provider, &locale, attrs)?
            .map_or_else(
                || load_one::<LocaleNamesRegionMinimalShortV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .map_or_else(
                || load_one::<LocaleNamesRegionCoreMediumV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .map_or_else(
                || load_one::<LocaleNamesRegionMinimalMediumV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
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
    use writeable::assert_writeable_eq;

    #[test]
    fn test_region_display_name_owned_table() {
        let prefs_en = DisplayNamesPreferences::from(locale!("en"));
        let inputs = [region!("US"), region!("AD")];

        macro_rules! check_row {
            ($constructor:ident) => {
                let items = inputs.iter().map(|id| {
                    RegionDisplayNameOwned::$constructor(prefs_en, *id)
                        .map(|name| Ok::<_, ()>(name.to_string()))
                });
                assert_eq!(
                    super::super::format_table_row(stringify!($constructor), items),
                    table_row!($constructor)
                );
            };
        }

        check_row!(try_new_minimal);
        check_row!(try_new_minimal_short);
        check_row!(try_new);
        check_row!(try_new_short);
    }

    #[test]
    fn test_region_display_name_overrides() {
        let prefs_ko = DisplayNamesPreferences::from(locale!("ko"));

        assert_writeable_eq!(
            RegionDisplayNameOwned::try_new(prefs_ko, region!("KR")).unwrap(),
            "대한민국"
        );
        assert_writeable_eq!(
            RegionDisplayNameOwned::try_new_short(prefs_ko, region!("KR")).unwrap(),
            "한국"
        );
        assert_writeable_eq!(
            RegionDisplayNameOwned::try_new_minimal(prefs_ko, region!("KR")).unwrap(),
            "대한민국"
        );
        assert_writeable_eq!(
            RegionDisplayNameOwned::try_new_minimal_short(prefs_ko, region!("KR")).unwrap(),
            "한국"
        );

        let prefs_fa = DisplayNamesPreferences::from(locale!("fa"));

        assert_writeable_eq!(
            RegionDisplayNameOwned::try_new(prefs_fa, region!("SA")).unwrap(),
            "عربستان سعودی"
        );
        assert_writeable_eq!(
            RegionDisplayNameOwned::try_new_short(prefs_fa, region!("SA")).unwrap(),
            "عربستان"
        );
        assert!(RegionDisplayNameOwned::try_new_minimal(prefs_fa, region!("SA")).is_err());
        assert!(RegionDisplayNameOwned::try_new_minimal_short(prefs_fa, region!("SA")).is_err());
    }
}
