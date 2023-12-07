// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data for the icu_collator crate
//!
//! This data was generated with CLDR version 44.0.0, ICU version release-74-1, and
//! LSTM segmenter version v0.1.0.

#![no_std]

#[cfg(icu4x_custom_data)]
include!(concat!(core::env!("ICU4X_DATA_DIR"), "/macros.rs"));
#[cfg(not(icu4x_custom_data))]
include!("../data/macros.rs");
