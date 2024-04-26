// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structures for CLDR JSON.
//!
//! The modules below each contain Rust struct definitions for CLDR JSON files, with Serde
//! deserialization support. These structures can be used in the transformers.

pub(in crate::provider) mod aliases;
pub(in crate::provider) mod ca;
pub(in crate::provider) mod coverage_levels;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod currencies;
pub(in crate::provider) mod currency_data;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod date_fields;
pub(in crate::provider) mod directionality;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod displaynames;
pub(in crate::provider) mod exemplar_chars;
pub(in crate::provider) mod japanese;
pub(in crate::provider) mod likely_subtags;
pub(in crate::provider) mod list_patterns;
pub(in crate::provider) mod locale_resource;
pub(in crate::provider) mod numbering_systems;
pub(in crate::provider) mod numbers;
pub(in crate::provider) mod parent_locales;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod personnames;
pub(in crate::provider) mod plural_ranges;
pub(in crate::provider) mod plurals;
pub(in crate::provider) mod time_zones;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod transforms;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod units;
pub(in crate::provider) mod week_data;

use locale_resource::LocaleResource;
