// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

mod list_formatter;

pub use crate::list_formatter::ListFormatter;
#[cfg(feature = "provider_serde")]
mod deduplicating_array;
pub mod error;
pub mod options;
pub mod provider;
mod string_matcher;
