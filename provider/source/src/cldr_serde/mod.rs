// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structures for CLDR JSON.
//!
//! The modules below each contain Rust struct definitions for CLDR JSON files, with Serde
//! deserialization support. These structures can be used in the transformers.

pub(crate) mod aliases;
pub(crate) mod ca;
pub(crate) mod coverage_levels;
#[cfg(feature = "unstable")]
pub(crate) mod currencies;
#[cfg(feature = "unstable")]
pub(crate) mod date_fields;
pub(crate) mod directionality;
#[cfg_attr(not(feature = "unstable"), allow(dead_code))]
pub(crate) mod displaynames;
pub(crate) mod eras;
pub(crate) mod exemplar_chars;
pub(crate) mod likely_subtags;
pub(crate) mod list_patterns;
pub(crate) mod locale_resource;
pub(crate) mod numbering_systems;
pub(crate) mod numbers;
pub(crate) mod parent_locales;
#[cfg(feature = "unstable")]
pub(crate) mod personnames;
#[cfg(feature = "unstable")]
pub(crate) mod plural_ranges;
pub(crate) mod plurals;
pub(crate) mod time_zones;
#[cfg(feature = "unstable")]
pub(crate) mod transforms;
pub(crate) mod units;
pub(crate) mod week_data;

use locale_resource::LocaleResource;
