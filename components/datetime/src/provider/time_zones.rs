// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider structs for time zones.

use alloc::borrow::Cow;
use icu_pattern::{DoublePlaceholderPattern, SinglePlaceholderPattern};
use icu_provider::prelude::*;
use zerovec::{ZeroMap, ZeroMap2d};

pub use icu_timezone::provider::{MetazoneId, TimeZoneBcp47Id};
use icu_timezone::ZoneVariant;

/// Time zone type aliases for cleaner code
pub(crate) mod tz {
    pub(crate) use super::LocationsV1;
    pub(crate) use super::LocationsV1Marker;
    pub(crate) use super::MetazoneGenericNamesLongV1Marker as MzGenericLongV1Marker;
    pub(crate) use super::MetazoneGenericNamesShortV1Marker as MzGenericShortV1Marker;
    pub(crate) use super::MetazoneGenericNamesV1 as MzGenericV1;
    pub(crate) use super::MetazoneSpecificNamesLongV1Marker as MzSpecificLongV1Marker;
    pub(crate) use super::MetazoneSpecificNamesShortV1Marker as MzSpecificShortV1Marker;
    pub(crate) use super::MetazoneSpecificNamesV1 as MzSpecificV1;
    pub(crate) use super::TimeZoneEssentialsV1 as EssentialsV1;
    pub(crate) use super::TimeZoneEssentialsV1Marker as EssentialsV1Marker;
}

/// An ICU4X mapping to the CLDR timeZoneNames format strings.
/// See CLDR-JSON timeZoneNames.json and <https://cldr.unicode.org/translation/time-zones-and-city-names>
/// for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(TimeZoneEssentialsV1Marker = "time_zone/essentials@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct TimeZoneEssentialsV1<'data> {
    /// The hour format for displaying offsets.
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "icu_provider::serde_borrow_de_utils::tuple_of_cow")
    )]
    pub hour_format: (Cow<'data, str>, Cow<'data, str>),
    /// The localized offset format.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
        )
    )]
    pub offset_format: Cow<'data, SinglePlaceholderPattern>,
    /// The localized zero-offset format.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub offset_zero_format: Cow<'data, str>,
    /// Metazone Name with Location Pattern.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::DoublePlaceholder, _>"
        )
    )]
    pub fallback_format: Cow<'data, DoublePlaceholderPattern>,
}

/// An ICU4X mapping to the CLDR timeZoneNames exemplar cities.
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(LocationsV1Marker = "time_zone/locations@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LocationsV1<'data> {
    /// Per-zone location display name
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub locations: ZeroMap<'data, TimeZoneBcp47Id, str>,
    /// The format string for a region's generic time.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
        )
    )]
    pub pattern_generic: Cow<'data, SinglePlaceholderPattern>,
    /// The format string for a region's standard time.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
        )
    )]
    pub pattern_standard: Cow<'data, SinglePlaceholderPattern>,
    /// The format string for a region's daylight time.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
        )
    )]
    pub pattern_daylight: Cow<'data, SinglePlaceholderPattern>,
}

/// An ICU4X mapping to generic metazone names.
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(MetazoneGenericNamesLongV1Marker, "time_zone/generic_long@1"),
    marker(MetazoneGenericNamesShortV1Marker, "time_zone/generic_short@1")
)]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct MetazoneGenericNamesV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub defaults: ZeroMap<'data, MetazoneId, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub overrides: ZeroMap<'data, TimeZoneBcp47Id, str>,
}

/// An ICU4X mapping to specific metazone names.
/// Specific names include time variants such as "daylight."
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    marker(MetazoneSpecificNamesLongV1Marker, "time_zone/specific_long@1"),
    marker(MetazoneSpecificNamesShortV1Marker, "time_zone/specific_short@1")
)]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::time_zones))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct MetazoneSpecificNamesV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub defaults: ZeroMap2d<'data, MetazoneId, ZoneVariant, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub overrides: ZeroMap2d<'data, TimeZoneBcp47Id, ZoneVariant, str>,
}
