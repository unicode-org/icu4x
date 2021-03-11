// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The `provider` module contains struct definitions for ICU4X [`DataProvider`].
//!
//! [`DataProvider`]: icu_provider::DataProvider

pub mod gregory;
pub(crate) mod helpers;
pub mod timezones;

pub mod key {
    use icu_provider::{resource_key, ResourceKey};
    pub const GREGORY_V1: ResourceKey = resource_key!(dates, "gregory", 1);
    pub const TIMEZONE_FORMATS_V1: ResourceKey = resource_key!(timezones, "formats", 1);
    pub const TIMEZONE_EXEMPLAR_CITIES_V1: ResourceKey =
        resource_key!(timezones, "exemplar-cities", 1);
    pub const TIMEZONE_GENERIC_NAMES_LONG_V1: ResourceKey =
        resource_key!(timezones, "generic-long", 1);
    pub const TIMEZONE_GENERIC_NAMES_SHORT_V1: ResourceKey =
        resource_key!(timezones, "generic-short", 1);
    pub const TIMEZONE_SPECIFIC_NAMES_LONG_V1: ResourceKey =
        resource_key!(timezones, "specific-long", 1);
    pub const TIMEZONE_SPECIFIC_NAMES_SHORT_V1: ResourceKey =
        resource_key!(timezones, "specific-short", 1);
}
