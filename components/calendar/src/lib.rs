// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for dealing with dates, times, and custom calendars.
//!
//! This module is published as its own crate ([`icu_calendar`](https://docs.rs/icu_calendar/latest/icu_calendar/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
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
//!
//! # Examples
//!
//! Examples of date manipulation using `Date` object. `Date` objects are useful
//! for working with dates, encompassing information about the day, month, year,
//! as well as the calendar type.
//!
//! ```rust
//! use icu_calendar::{types::IsoWeekday, Date, DateDuration, DateDurationUnit};
//!
//! // Creating ISO date: 1992-09-02.
//! let mut date_iso = Date::new_iso_date(1992, 9, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//!
//! assert_eq!(date_iso.day_of_week(), IsoWeekday::Wednesday);
//! assert_eq!(date_iso.year().number, 1992);
//! assert_eq!(date_iso.month().ordinal, 9);
//! assert_eq!(date_iso.day_of_month().0, 2);
//!
//! // Answering questions about days in month and year.
//! assert_eq!(date_iso.days_in_year(), 366);
//! assert_eq!(date_iso.days_in_month(), 30);
//!
//! // Advancing date in-place by 1 year, 2 months, 3 weeks, 4 days.
//! date_iso.add(DateDuration::new(1, 2, 3, 4));
//! assert_eq!(date_iso.year().number, 1993);
//! assert_eq!(date_iso.month().ordinal, 11);
//! assert_eq!(date_iso.day_of_month().0, 27);
//!
//! // Reverse date advancement.
//! date_iso.add(DateDuration::new(-1, -2, -3, -4));
//! assert_eq!(date_iso.year().number, 1992);
//! assert_eq!(date_iso.month().ordinal, 9);
//! assert_eq!(date_iso.day_of_month().0, 2);
//!
//! // Creating ISO date: 2022-01-30.
//! let newer_date_iso = Date::new_iso_date(2022, 1, 30)
//!     .expect("Failed to initialize ISO Date instance.");
//!
//! // Comparing dates: 2022-01-30 and 1992-09-02.
//! let duration = newer_date_iso.until(&date_iso, DateDurationUnit::Years, DateDurationUnit::Days);
//! assert_eq!(duration.years, 30);
//! assert_eq!(duration.months, -8);
//! assert_eq!(duration.days, 28);
//!
//! // Create new date with date advancement. Reassign to new variable.
//! let mutated_date_iso = date_iso.added(DateDuration::new(1, 2, 3, 4));
//! assert_eq!(mutated_date_iso.year().number, 1993);
//! assert_eq!(mutated_date_iso.month().ordinal, 11);
//! assert_eq!(mutated_date_iso.day_of_month().0, 27);
//! ```
//!
//! Example of converting an ISO date across Indian and Buddhist calendars.
//!
//! ```rust
//! use icu_calendar::{buddhist::Buddhist, indian::Indian, Date};
//!
//! // Creating ISO date: 1992-09-02.
//! let mut date_iso = Date::new_iso_date(1992, 9, 2)
//!     .expect("Failed to initialize ISO Date instance.");
//!
//! assert_eq!(date_iso.year().number, 1992);
//! assert_eq!(date_iso.month().ordinal, 9);
//! assert_eq!(date_iso.day_of_month().0, 2);
//!
//! // Conversion into Indian calendar: 1914-08-02.
//! let date_indian = date_iso.to_calendar(Indian);
//! assert_eq!(date_indian.year().number, 1914);
//! assert_eq!(date_indian.month().ordinal, 8);
//! assert_eq!(date_indian.day_of_month().0, 30);
//!
//! // Conversion into Buddhist calendar: 2535-09-02.
//! let date_buddhist = date_iso.to_calendar(Buddhist);
//! assert_eq!(date_buddhist.year().number, 2535);
//! assert_eq!(date_buddhist.month().ordinal, 9);
//! assert_eq!(date_buddhist.day_of_month().0, 2);
//! ```
//!
//! Example using `DateTime` object. Similar to `Date` objects, `DateTime` objects
//! contain an accessible `Date` object containing information about the day, month,
//! year, and calendar type. Additionally, `DateTime` objects contain an accessible
//! `Time` object, including granularity of hour, minute, second, and nanosecond.
//!
//! ```rust
//! use icu_calendar::{types::IsoWeekday, types::Time, DateDuration, DateTime};
//!
//! // Creating ISO date: 1992-09-02 8:59
//! let mut datetime_iso = DateTime::new_iso_datetime(1992, 9, 2, 8, 59, 0)
//!     .expect("Failed to initialize ISO DateTime instance.");
//!
//! assert_eq!(datetime_iso.date.day_of_week(), IsoWeekday::Wednesday);
//! assert_eq!(datetime_iso.date.year().number, 1992);
//! assert_eq!(datetime_iso.date.month().ordinal, 9);
//! assert_eq!(datetime_iso.date.day_of_month().0, 2);
//! assert_eq!(datetime_iso.time.hour.number(), 8);
//! assert_eq!(datetime_iso.time.minute.number(), 59);
//! assert_eq!(datetime_iso.time.second.number(), 0);
//! assert_eq!(datetime_iso.time.nanosecond.number(), 0);
//!
//! // Advancing date by 1 year, 2 months, 3 weeks, 4 days.
//! datetime_iso.date.add(DateDuration::new(1, 2, 3, 4));
//! // New time of 14:30
//! datetime_iso.time = Time::try_new(14, 30, 0, 0).expect("Failed to initialize Time instance.");
//!
//! assert_eq!(datetime_iso.date.year().number, 1993);
//! assert_eq!(datetime_iso.date.month().ordinal, 11);
//! assert_eq!(datetime_iso.date.day_of_month().0, 27);
//! assert_eq!(datetime_iso.time.hour.number(), 14);
//! assert_eq!(datetime_iso.time.minute.number(), 30);
//! assert_eq!(datetime_iso.time.second.number(), 0);
//! assert_eq!(datetime_iso.time.nanosecond.number(), 0);
//! ```
//! [`ICU4X`]: ../icu/index.html

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
        // TODO(#2266): enable missing_debug_implementations,
    )
)]

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
pub use date::{AsCalendar, Date, Ref};
pub use datetime::DateTime;
pub use duration::{DateDuration, DateDurationUnit};
pub use error::DateTimeError;
pub use gregorian::Gregorian;
pub use iso::Iso;
