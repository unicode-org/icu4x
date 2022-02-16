// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains the core transformer code from CLDR JSON to ICU4X Data Provider.
//!
//! Every ICU4X component should have its own private submodule and then export the types from here.

pub(crate) mod calendar;
pub(crate) mod datetime;
pub(crate) mod decimal;
#[cfg(feature = "icu_list")]
pub(crate) mod list;
pub(crate) mod locale_canonicalizer;
pub(crate) mod plurals;
pub(crate) mod time_zones;
