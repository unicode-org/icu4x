// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(not(any(test, feature = "std")), no_std)]

//! The `icu_calendars` crate contains the core types used by ICU4X for dealing
//! with dates, times, and custom calendars.
//!
//! The [`types`] module has a lot of common types for dealing with dates and times.
//!
//! [`Calendar`] is a trait that allows one to define custom calendars, and [`Date`]
//! can represent dates for arbitrary calendars.
//!
//! The [`iso`] module contains an implementation for the ISO calendar.
extern crate alloc;

mod calendar;
mod date;
mod duration;
mod error;
pub mod iso;
pub mod types;

pub use calendar::Calendar;
pub use date::{AsCalendar, Date};
pub use duration::{DateDuration, DateDurationUnit};
pub use error::DateTimeError;
pub use iso::Iso;
