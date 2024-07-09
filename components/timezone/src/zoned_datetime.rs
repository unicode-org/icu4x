// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{AsCalendar, Date, Time};

use crate::CustomTimeZone;

/// A date and time local to a specified custom time zone.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct CustomZonedDateTime<A: AsCalendar> {
    /// The date, local to the time zone
    pub date: Date<A>,
    /// The time, local to the time zone
    pub time: Time,
    /// The time zone
    pub zone: CustomTimeZone,
}
