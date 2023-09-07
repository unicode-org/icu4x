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
#![warn(missing_docs)]

mod astronomy;
/// Chinese-like lunar calendars (Chinese, Dangi)
pub mod chinese_based;
/// The Coptic calendar
pub mod coptic;
/// Error handling
mod error;
/// The ethiopian calendar
pub mod ethiopian;
/// The Hebrew calendar
pub mod hebrew;
/// Additional math helpers
pub mod helpers;
/// Various islamic lunar calendars
pub mod islamic;
/// The ISO calendar (also usable as Gregorian)
pub mod iso;
/// The Julian calendar
pub mod julian;
/// The persian calendar
pub mod persian;
/// Representation of Rata Die (R.D., also called J.D. for Julain Date)
/// dates, which are represented as the number of days since ISO date 0001-01-01.
pub mod rata_die;
