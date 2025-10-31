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

const TWO_POWER_FIFTY_THREE: i128 = 9_007_199_254_740_992;
const SECONDS_PER_DAY: i128 = 24 * 60 * 60;

impl DateDuration {
    /// Temporal caps y/m/weeks within u32, units of days and below are capped within
    /// the maximum safe seconds value, which is 2⁵³ s.
    ///
    /// <https://tc39.es/proposal-temporal/#sec-isvalidduration>
    ///
    /// We should eventually be applying some limits to this code in ICU4X.
    /// We currently do not and `Date::try_added()` will panic for large `days` values.
    ///
    /// <https://github.com/unicode-org/icu4x/issues/3964>
    ///
    /// For now, fuzz what we can for Temporal's needs.
    fn is_valid_for_temporal(&self) -> bool {
        (self.days as i128).saturating_mul(SECONDS_PER_DAY) < TWO_POWER_FIFTY_THREE
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
    if !data.duration.is_valid_for_temporal() {
        return;
    }
    let _ = date.try_added_with_options(data.duration.into(), options);
});
