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

impl DateDuration {
    /// Temporal doesn't care about dates outside of -271821-04-20 to +275760-09-13
    /// and will not let you attempt to add up to them even with overflow: constrain.
    ///
    /// We should eventually be applying some limits to this code in ICU4X.
    /// We currently do not and `Date::try_added()` will panic for large `days` values.
    ///
    /// <https://github.com/unicode-org/icu4x/issues/3964>
    ///
    /// For now, fuzz what we can for Temporal's needs.
    ///
    /// This code is copied from <https://github.com/boa-dev/temporal/pull/615>
    /// so that we are testing temporal_rs behavior.
    fn is_valid_for_temporal(&self) -> bool {
        // Temporal range is -271821-04-20 to +275760-09-13
        // This is (roughly) the maximum year duration that can exist for ISO
        const TEMPORAL_MAX_ISO_YEAR_DURATION: u32 = 275760 + 271821;
        // Double it. No calendar has years that are half the size of ISO years.
        const YEAR_DURATION: u32 = 2 * TEMPORAL_MAX_ISO_YEAR_DURATION;
        // Assume every year is a leap year, calculate a month range
        const MONTH_DURATION: u32 = YEAR_DURATION * 13;
        // Our longest year is 390 days
        const DAY_DURATION: u32 = YEAR_DURATION * 390;
        const WEEK_DURATION: u32 = DAY_DURATION / 7;


        if self.years > YEAR_DURATION {
            return false;
        }
        if self.months > MONTH_DURATION {
            return false;
        }
        if self.weeks > WEEK_DURATION {
            return false;
        }
        if self.days > DAY_DURATION.into() {
            return false;
        }

        true

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
