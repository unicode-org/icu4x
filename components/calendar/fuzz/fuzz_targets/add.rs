// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main]

mod common;

use common::{Ymd, AnyCalendarKind};

use arbitrary::Arbitrary;
use icu_calendar::options::*;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    ymd: Ymd,
    duration: DateDuration,
    overflow_constrain: bool,
    cal: AnyCalendarKind,
}

#[derive(Arbitrary, Debug)]
pub struct DateDuration {
    pub is_negative: bool,
    pub years: u32,
    pub months: u32,
    pub weeks: u32,
    pub days: u64,
}

impl From<DateDuration> for icu_calendar::types::DateDuration {
    fn from(other: DateDuration) -> Self {
        Self {
            is_negative: other.is_negative,
            years: other.years,
            months: other.months,
            weeks: other.weeks,
            days: other.days,
        }
    }
}

fuzz_target!(|data: FuzzInput| {
    let Some(date) = data.ymd.to_date(data.cal, true) else { return };

    let mut options = DateAddOptions::default();
    options.overflow = if data.overflow_constrain {
        Some(Overflow::Constrain)
    } else {
        Some(Overflow::Reject)
    };

    let _ = date.try_added_with_options(data.duration.into(), options);
});
