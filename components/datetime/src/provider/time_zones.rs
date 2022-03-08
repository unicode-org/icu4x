// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use icu_provider::{yoke, zerofrom};
use tinystr::TinyStr8;
use zerovec::{ZeroMap, ZeroMap2d};

/// An ICU4X mapping to the CLDR timeZoneNames format strings.
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct(TimeZoneFormatsV1Marker = "time_zone/formats@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct TimeZoneFormatsV1<'data> {
    /// The hour format for displaying GMT offsets.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    #[cfg_attr(feature = "provider_serde", serde(deserialize_with = "icu_provider::serde::borrow_de_utils::tuple_of_cow"))]
    pub hour_format: (Cow<'data, str>, Cow<'data, str>),
    /// The localized GMT-offset format.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub gmt_format: Cow<'data, str>,
    /// The localized GMT format with no offset.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub gmt_zero_format: Cow<'data, str>,
    /// The format string for a region.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub region_format: Cow<'data, str>,
    /// The format strings for region format variants
    /// e.g. daylight, standard.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub region_format_variants: ZeroMap<'data, TinyStr8, str>,
    /// The format string to fall back to if data is unavailable.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub fallback_format: Cow<'data, str>,
}

/// An ICU4X mapping to the CLDR timeZoneNames exemplar cities.
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct(ExemplarCitiesV1Marker = "time_zone/exemplar_cities@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct ExemplarCitiesV1<'data>(
    #[cfg_attr(feature = "provider_serde", serde(borrow))] pub ZeroMap<'data, str, str>,
);

/// An ICU4X mapping to the long-form generic metazone names.
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct(MetaZoneGenericNamesLongV1Marker = "time_zone/generic_long@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct MetaZoneGenericNamesLongV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub defaults: ZeroMap<'data, str, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub overrides: ZeroMap<'data, str, str>,
}

/// An ICU4X mapping to the short-form generic metazone names.
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct(MetaZoneGenericNamesShortV1Marker = "time_zone/generic_short@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct MetaZoneGenericNamesShortV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub defaults: ZeroMap<'data, str, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub overrides: ZeroMap<'data, str, str>,
}

/// An ICU4X mapping to the long-form specific metazone names.
/// Specific names include time variants such as "daylight."
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct(MetaZoneSpecificNamesLongV1Marker = "time_zone/specific_long@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct MetaZoneSpecificNamesLongV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub defaults: ZeroMap2d<'data, str, TinyStr8, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub overrides: ZeroMap2d<'data, str, TinyStr8, str>,
}

/// An ICU4X mapping to the short-form specific metazone names.
/// Specific names include time variants such as "daylight."
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct(MetaZoneSpecificNamesShortV1Marker = "time_zone/specific_short@1")]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(prove_covariance_manually)]
pub struct MetaZoneSpecificNamesShortV1<'data> {
    /// The default mapping between metazone id and localized metazone name.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub defaults: ZeroMap2d<'data, str, TinyStr8, str>,
    /// The override mapping between timezone id and localized metazone name.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub overrides: ZeroMap2d<'data, str, TinyStr8, str>,
}
