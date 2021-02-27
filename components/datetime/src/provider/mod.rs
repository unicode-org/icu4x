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
    pub const TIMEZONES_V1: ResourceKey = resource_key!(timezones, "timezones", 1);
}
