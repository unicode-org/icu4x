// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
        clippy::exhaustive_enums
    )
)]

//! The `icu_calendar` crate contains the core types used by ICU4X for dealing
//! with dates, times, and custom calendars.
//!
//! The [`types`] module has a lot of common types for dealing with dates and times.
//!
//! [`Calendar`] is a trait that allows one to define custom calendars, and [`Date`]
//! can represent dates for arbitrary calendars.
//!
//! The [`iso`] and [`gregorian`] modules contain implementations for the ISO and
//! Gregorian calendars respectively.
//!
//! Some of the algorithms implemented here are based on
//! Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
//! with associated Lisp code found at <https://github.com/EdReingold/calendar-code2>.
extern crate alloc;

pub mod any_calendar;
pub mod arithmetic;
pub mod buddhist;
mod calendar;
mod calendar_arithmetic;
pub mod coptic;
mod date;
mod datetime;
mod duration;
mod error;
pub mod ethiopic;
pub mod gregorian;
pub mod indian;
pub mod iso;
pub mod japanese;
pub mod julian;
pub mod provider;
pub mod types;

pub use calendar::Calendar;
pub use calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
pub use date::{AsCalendar, Date};
pub use datetime::DateTime;
pub use duration::{DateDuration, DateDurationUnit};
pub use error::DateTimeError;
pub use gregorian::Gregorian;
pub use iso::Iso;
