// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider structs for time zones.

use alloc::borrow::Cow;
use icu_provider::prelude::*;
use tinystr::TinyStr8;
use zerovec::{ZeroMap, ZeroMap2d};

pub use icu_timezone::provider::{MetazoneId, TimeZoneBcp47Id};
use icu_timezone::ZoneVariant;

/// An ICU4X mapping to the CLDR timeZoneNames format strings.
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(TimeZoneFormatsV1Marker = "time_zone/formats@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::time_zones),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct TimeZoneFormatsV1<'data> {
    /// The hour format for displaying GMT offsets.
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "icu_provider::serde::borrow_de_utils::tuple_of_cow")
    )]
    pub hour_format: (Cow<'data, str>, Cow<'data, str>),
    /// The localized GMT-offset format.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub gmt_format: Cow<'data, str>,
    /// The localized GMT format with no offset.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub gmt_zero_format: Cow<'data, str>,
    /// The format string for a region.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub region_format: Cow<'data, str>,
    /// The format strings for region format variants
    /// e.g. daylight, standard.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub region_format_variants: ZeroMap<'data, TinyStr8, str>,
    /// The format string to fall back to if data is unavailable.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub fallback_format: Cow<'data, str>,
    /// The fallback of GMT-offset.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub gmt_offset_fallback: Cow<'data, str>,
}

/// An ICU4X mapping to the CLDR timeZoneNames exemplar cities.
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(ExemplarCitiesV1Marker = "time_zone/exemplar_cities@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::time_zones),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct ExemplarCitiesV1<'data>(
    #[cfg_attr(feature = "serde", serde(borrow))] pub ZeroMap<'data, TimeZoneBcp47Id, str>,
);

/// An ICU4X mapping to the long-form generic metazone names.
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(MetazoneGenericNamesLongV1Marker = "time_zone/generic_long@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::time_zones),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct MetazoneGenericNamesLongV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub defaults: ZeroMap<'data, MetazoneId, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub overrides: ZeroMap<'data, TimeZoneBcp47Id, str>,
}

/// An ICU4X mapping to the short-form generic metazone names.
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(MetazoneGenericNamesShortV1Marker = "time_zone/generic_short@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::time_zones),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct MetazoneGenericNamesShortV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub defaults: ZeroMap<'data, MetazoneId, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub overrides: ZeroMap<'data, TimeZoneBcp47Id, str>,
}

/// An ICU4X mapping to the long-form specific metazone names.
/// Specific names include time variants such as "daylight."
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(MetazoneSpecificNamesLongV1Marker = "time_zone/specific_long@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::time_zones),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct MetazoneSpecificNamesLongV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub defaults: ZeroMap2d<'data, MetazoneId, ZoneVariant, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub overrides: ZeroMap2d<'data, TimeZoneBcp47Id, ZoneVariant, str>,
}

/// An ICU4X mapping to the short-form specific metazone names.
/// Specific names include time variants such as "daylight."
/// See CLDR-JSON timeZoneNames.json for more context.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(MetazoneSpecificNamesShortV1Marker = "time_zone/specific_short@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::time_zones),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct MetazoneSpecificNamesShortV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub defaults: ZeroMap2d<'data, MetazoneId, ZoneVariant, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub overrides: ZeroMap2d<'data, TimeZoneBcp47Id, ZoneVariant, str>,
}
