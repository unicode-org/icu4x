// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    impl_writeable_for_single_display_name_borrowed, impl_writeable_for_single_display_name_owned,
};
use crate::displaynames::DisplayNamesPreferences;
use crate::displaynames::provider::{
    LocaleNamesRegionMediumLightV1, LocaleNamesRegionMediumTinyV1, LocaleNamesRegionShortLightV1,
    LocaleNamesRegionShortTinyV1,
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
    LocaleNamesRegionMediumTinyV1::make_locale(prefs.locale_preferences)
}

macro_rules! table_row {
    (try_new_tiny) => {
        "| [`try_new_tiny`](Self::try_new_tiny) | \"United States\" | ❌ |"
    };
    (try_new_short_tiny) => {
        "| [`try_new_short_tiny`](Self::try_new_short_tiny) | \"US\" | ❌ |"
    };
    (try_new_light) => {
        "| [`try_new_light`](Self::try_new_light) | \"United States\" | \"Andorra\" |"
    };
    (try_new_short_light) => {
        "| [`try_new_short_light`](Self::try_new_short_light) | \"US\" | \"Andorra\" |"
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
#[doc = concat!(table_row!(try_new_tiny), "\n")]
#[doc = concat!(table_row!(try_new_short_tiny), "\n")]
#[doc = concat!(table_row!(try_new_light), "\n")]
#[doc = concat!(table_row!(try_new_short_light), "\n")]
///
/// > Note: :x: means that the constructor returns an error.
///
/// # Example
///
/// ```
/// use icu::experimental::displaynames::single::RegionDisplayName;
/// use icu::locale::{locale, subtags::region};
/// use writeable::assert_writeable_eq;
///
/// let display_name = RegionDisplayName::try_new_light(locale!("en").into(), region!("US"))
///     .expect("Data should load successfully");
///
/// assert_writeable_eq!(display_name, "United States");
/// ```
#[derive(Debug)]
pub struct RegionDisplayName {
    pub(crate) payload: DataPayload<LocaleNamesRegionMediumLightV1>,
}

impl RegionDisplayName {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, region: Region) -> result: Result<Self, DataError>,
        /// Loads the region display name for a given region and locale using compiled data.
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
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesRegionMediumLightV1>
            + DataProvider<LocaleNamesRegionMediumTinyV1>,
    {
        let attrs = make_attributes(&region);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesRegionMediumLightV1, _, _>(provider, &locale, attrs)?
            .map_or_else(
                || load_one::<LocaleNamesRegionMediumTinyV1, _, _>(provider, &locale, attrs),
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
        /// use icu::experimental::displaynames::single::RegionDisplayName;
        /// use icu::locale::{locale, subtags::region};
        /// use writeable::assert_writeable_eq;
        ///
        /// let display_name = RegionDisplayName::try_new_tiny(locale!("en").into(), region!("US"))
        ///     .expect("Data should load successfully");
        ///
        /// assert_writeable_eq!(display_name, "United States");
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
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized + DataProvider<LocaleNamesRegionMediumTinyV1>,
    {
        let attrs = make_attributes(&region);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesRegionMediumTinyV1, _, _>(provider, &locale, attrs)?
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
            try_new_short_tiny,
            try_new_short_tiny_with_buffer_provider,
            try_new_short_tiny_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short_tiny)]
    pub fn try_new_short_tiny_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesRegionShortTinyV1>
            + DataProvider<LocaleNamesRegionMediumTinyV1>,
    {
        let attrs = make_attributes(&region);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesRegionShortTinyV1, _, _>(provider, &locale, attrs)?
            .map_or_else(
                || load_one::<LocaleNamesRegionMediumTinyV1, _, _>(provider, &locale, attrs),
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
        ///     DisplayNamesPreferences, single::RegionDisplayName,
        /// };
        /// use icu::locale::{locale, subtags::region};
        /// use writeable::assert_writeable_eq;
        ///
        /// let prefs: DisplayNamesPreferences = locale!("en-US").into();
        ///
        /// // "US" has a short display name in en-US
        /// let display_name_short = RegionDisplayName::try_new_short_light(prefs, region!("US"))
        ///     .expect("Data should load successfully");
        /// assert_writeable_eq!(display_name_short, "US");
        ///
        /// // "AD" does not have a short display name, so it falls back to the long display name
        /// let display_name_long = RegionDisplayName::try_new_short_light(prefs, region!("AD"))
        ///     .expect("Data should load successfully");
        /// assert_writeable_eq!(display_name_long, "Andorra");
        /// ```
        functions: [
            try_new_short_light,
            try_new_short_light_with_buffer_provider,
            try_new_short_light_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short_light)]
    pub fn try_new_short_light_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        region: Region,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesRegionShortLightV1>
            + DataProvider<LocaleNamesRegionShortTinyV1>
            + DataProvider<LocaleNamesRegionMediumLightV1>
            + DataProvider<LocaleNamesRegionMediumTinyV1>,
    {
        let attrs = make_attributes(&region);
        let locale = make_locale(prefs);
        let payload = load_one::<LocaleNamesRegionShortLightV1, _, _>(provider, &locale, attrs)?
            .map_or_else(
                || load_one::<LocaleNamesRegionShortTinyV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .map_or_else(
                || load_one::<LocaleNamesRegionMediumLightV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .map_or_else(
                || load_one::<LocaleNamesRegionMediumTinyV1, _, _>(provider, &locale, attrs),
                |p| Ok(Some(p)),
            )?
            .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
        Ok(Self { payload })
    }

    /// Returns a borrowed version of this display name.
    pub fn as_borrowed(&self) -> RegionDisplayNameBorrowed<'_> {
        RegionDisplayNameBorrowed {
            value: self.payload.get(),
        }
    }
}

impl_writeable_for_single_display_name_owned!(RegionDisplayName);

/// A localized display name for a single region.
#[derive(Debug, Clone, Copy)]
pub struct RegionDisplayNameBorrowed<'a> {
    value: &'a str,
}

impl_writeable_for_single_display_name_borrowed!(RegionDisplayNameBorrowed);

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::{locale, subtags::region};

    #[test]
    fn test_region_display_name_owned_table() {
        let prefs_en = DisplayNamesPreferences::from(locale!("en"));
        let inputs = [region!("US"), region!("AD")];

        macro_rules! check_row {
            ($constructor:ident) => {
                let items = inputs.iter().map(|id| {
                    RegionDisplayName::$constructor(prefs_en, *id)
                        .map(|name| Ok::<_, ()>(name.to_string()))
                });
                assert_eq!(
                    super::super::format_table_row(stringify!($constructor), items),
                    table_row!($constructor)
                );
            };
        }

        check_row!(try_new_tiny);
        check_row!(try_new_short_tiny);
        check_row!(try_new_light);
        check_row!(try_new_short_light);
    }
}
