// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Adapters for composing and manipulating data providers.
//!
//! - Use the [`fork`] module to marshall data requests between multiple possible providers.
//! - Use the [`either`] module to choose between multiple provider types at runtime.
//! - Use the [`filter`] module to programmatically reject certain data requests.
//! - Use the [`fallback`] module to automatically resolve arbitrary locales for data loading.

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

pub mod any_payload;
pub mod either;
pub mod empty;
pub mod fallback;
pub mod filter;
pub mod fork;
mod helpers;
