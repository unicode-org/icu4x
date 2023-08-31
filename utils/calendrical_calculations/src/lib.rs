// This file is part of ICU4X.
//
// This file is licensed under the Apache License, Version 2.0,
// which can be found in the LICENSE file in the
// calendrical_calculations package root or online at
// <https://www.apache.org/licenses/LICENSE-2.0>.

//! Calendrical calculations
//!
//! This crate implements algorithms from
//! Calendrical Calculations by Reingold & Dershowitz, Cambridge University Press, 4th edition (2018)
//! as needed by [ICU4X](https://github.com/unicode-org/icu4x).
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

// TODO
// #![warn(missing_docs)]

// TODO: once everything is moved, go through and make stuff private again
pub mod astronomy;
pub mod coptic;
pub mod error;
pub mod ethiopian;
pub mod helpers;
pub mod iso;
pub mod julian;
pub mod rata_die;
