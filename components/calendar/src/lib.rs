// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(not(any(test, feature = "std")), no_std)]

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
extern crate alloc;

#[allow(clippy::indexing_slicing, clippy::expect_used, clippy::panic)]
pub mod arithmetic;
pub mod buddhist;
mod calendar;
mod date;
mod datetime;
mod duration;
mod error;
pub mod gregorian;
pub mod iso;
pub mod japanese;
pub mod julian;
pub mod provider;
pub mod types;

pub use calendar::Calendar;
pub use date::{AsCalendar, Date};
pub use datetime::DateTime;
pub use duration::{DateDuration, DateDurationUnit};
pub use error::DateTimeError;
pub use gregorian::Gregorian;
pub use iso::Iso;
