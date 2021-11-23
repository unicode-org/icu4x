// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::Gregorian;

/// A calendar that can be found in CLDR
pub trait CldrCalendar {
    /// The Unicode BCP 47 identifier for the calendar
    const IDENTIFIER: &'static str;
}

impl CldrCalendar for Gregorian {
    const IDENTIFIER: &'static str = "gregory";
}