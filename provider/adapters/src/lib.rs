// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Adapters for composing and manipulating data providers.
//!
//! - Use the [`fork`] module to marshall data requests between multiple possible providers.
//! - Use the [`either`] module to choose between multiple provider types at runtime.
//! - Use the [`filter`] module to programmatically reject certain data requests.

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

pub mod either;
pub mod filter;
pub mod fork;
mod helpers;
pub mod struct_provider;
