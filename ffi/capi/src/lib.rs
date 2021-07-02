// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::upper_case_acronyms)]

#[macro_use]
mod macros;

pub mod custom_writeable;
pub mod decimal;
pub mod fixed_decimal;
pub mod locale;
pub mod locale_canonicalizer;
pub mod pluralrules;
pub mod provider;

#[cfg(target_arch = "wasm32")]
mod wasm_glue;
