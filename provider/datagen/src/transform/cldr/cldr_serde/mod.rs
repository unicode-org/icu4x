// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structures for CLDR JSON.
//!
//! The modules below each contain Rust struct definitions for CLDR JSON files, with Serde
//! deserialization support. These structures can be used in the transformers.

pub mod aliases;
pub mod ca;
pub mod coverage_levels;
#[cfg(feature = "icu_singlenumberformatter")]
pub mod currencies;
pub mod currency_data;
#[cfg(feature = "icu_relativetime")]
pub mod date_fields;
pub mod directionality;
#[cfg(feature = "icu_displaynames")]
pub mod displaynames;
pub mod exemplar_chars;
pub mod japanese;
pub mod likely_subtags;
pub mod list_patterns;
pub mod locale_resource;
pub mod numbering_systems;
pub mod numbers;
pub mod parent_locales;
pub mod plurals;
pub mod time_zones;
#[cfg(feature = "icu_transliteration")]
pub mod transforms;
#[cfg(feature = "icu_unitsconversion")]
pub mod units;
pub mod week_data;

use locale_resource::LocaleResource;
