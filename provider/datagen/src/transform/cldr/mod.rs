// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by the JSON files shipped by CLDR.

pub mod calendar;
pub mod characters;
pub mod cldr_serde;
#[cfg(feature = "icu_singlenumberformatter")]
pub mod currency;
pub mod datetime;
pub mod decimal;
#[cfg(feature = "icu_displaynames")]
pub mod displaynames;
pub mod fallback;
pub mod list;
pub mod locale_canonicalizer;
pub mod plurals;
#[cfg(feature = "icu_relativetime")]
pub mod relativetime;
pub mod source;
pub mod time_zones;
#[cfg(feature = "icu_transliteration")]
pub mod transforms;
#[cfg(feature = "icu_unitsconversion")]
pub mod units;
