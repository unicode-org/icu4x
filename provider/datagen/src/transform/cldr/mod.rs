// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by the JSON files shipped by CLDR.

pub mod calendar;
pub mod characters;
pub mod cldr_serde;
pub mod datetime;
pub mod decimal;
pub mod displaynames;
pub mod fallback;
pub mod list;
pub mod locale_canonicalizer;
pub mod plurals;
pub mod relativetime;
pub mod source;
pub mod time_zones;
