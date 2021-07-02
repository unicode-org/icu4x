// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

pub mod gregory;
pub(crate) mod helpers;
pub mod time_zones;

pub mod key {
    use icu_provider::{resource_key, ResourceKey};
    pub const GREGORY_DATE_PATTERNS_V1: ResourceKey = resource_key!(date_patterns, "gregory", 1);
    pub const GREGORY_DATE_SYMBOLS_V1: ResourceKey = resource_key!(date_symbols, "gregory", 1);
    pub const TIMEZONE_FORMATS_V1: ResourceKey = resource_key!(time_zones, "formats", 1);
    pub const TIMEZONE_EXEMPLAR_CITIES_V1: ResourceKey =
        resource_key!(time_zones, "exemplar-cities", 1);
    pub const TIMEZONE_GENERIC_NAMES_LONG_V1: ResourceKey =
        resource_key!(time_zones, "generic-long", 1);
    pub const TIMEZONE_GENERIC_NAMES_SHORT_V1: ResourceKey =
        resource_key!(time_zones, "generic-short", 1);
    pub const TIMEZONE_SPECIFIC_NAMES_LONG_V1: ResourceKey =
        resource_key!(time_zones, "specific-long", 1);
    pub const TIMEZONE_SPECIFIC_NAMES_SHORT_V1: ResourceKey =
        resource_key!(time_zones, "specific-short", 1);
}
