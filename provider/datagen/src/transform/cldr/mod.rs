// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by the JSON files shipped by CLDR.

pub(in crate::provider) mod calendar;
pub(in crate::provider) mod characters;
pub(in crate::provider) mod cldr_serde;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod currency;
pub(in crate::provider) mod datetime;
pub(in crate::provider) mod decimal;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod displaynames;
pub(in crate::provider) mod fallback;
pub(in crate::provider) mod list;
pub(in crate::provider) mod locale_canonicalizer;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod percent;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod personnames;
pub(in crate::provider) mod plurals;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod relativetime;
pub(in crate::provider) mod source;
pub(in crate::provider) mod time_zones;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod transforms;
#[cfg(feature = "experimental_components")]
pub(in crate::provider) mod units;
