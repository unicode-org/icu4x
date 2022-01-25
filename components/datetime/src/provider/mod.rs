// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

#[cfg(doc)]
use icu_provider::prelude::ResourceKey;

/// Data providers for the Gregorian Calendar.
pub mod calendar;

/// Data providers for time zones.
pub mod time_zones;

/// Traits for managing data needed by [`DateTimeFormat`](crate::DateTimeFormat).
pub(crate) mod date_time;

/// A collection of [`ResourceKey`] structs for DateTime providers.
pub mod key {
    #[cfg(doc)]
    use crate::provider::{calendar, time_zones};

    use icu_provider::{resource_key, ResourceKey};

    /// A [`ResourceKey`] to [`calendar::DatePatternsV1`].
    pub const DATE_PATTERNS_V1: ResourceKey = resource_key!(DateTime, "lengths", 1);

    /// A [`ResourceKey`] to [`calendar::DateSkeletonPatternsV1`].
    pub const DATE_SKELETON_PATTERNS_V1: ResourceKey = resource_key!(DateTime, "skeletons", 1);

    /// A [`ResourceKey`] to [`calendar::DateSymbolsV1`]
    pub const DATE_SYMBOLS_V1: ResourceKey = resource_key!(DateTime, "symbols", 1);

    /// A [`ResourceKey`] to [`time_zones::TimeZoneFormatsV1`].
    pub const TIMEZONE_FORMATS_V1: ResourceKey = resource_key!(TimeZone, "formats", 1);

    /// A [`ResourceKey`] to [`time_zones::ExemplarCitiesV1`].
    pub const TIMEZONE_EXEMPLAR_CITIES_V1: ResourceKey =
        resource_key!(TimeZone, "exemplar_cities", 1);

    /// A [`ResourceKey`] to [`time_zones::MetaZoneGenericNamesLongV1`].
    pub const TIMEZONE_GENERIC_NAMES_LONG_V1: ResourceKey =
        resource_key!(TimeZone, "generic_long", 1);

    /// A [`ResourceKey`] to [`time_zones::MetaZoneGenericNamesShortV1`].
    pub const TIMEZONE_GENERIC_NAMES_SHORT_V1: ResourceKey =
        resource_key!(TimeZone, "generic_short", 1);

    /// A [`ResourceKey`] to [`time_zones::MetaZoneSpecificNamesLongV1`].
    pub const TIMEZONE_SPECIFIC_NAMES_LONG_V1: ResourceKey =
        resource_key!(TimeZone, "specific_long", 1);

    /// A [`ResourceKey`] to [`time_zones::MetaZoneSpecificNamesShortV1`].
    pub const TIMEZONE_SPECIFIC_NAMES_SHORT_V1: ResourceKey =
        resource_key!(TimeZone, "specific_short", 1);
}
