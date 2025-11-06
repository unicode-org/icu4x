// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main]

mod common;

use common::{Ymd, AnyCalendarKind};

use arbitrary::Arbitrary;
use icu_calendar::options::DateDifferenceOptions;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    ymd1: Ymd,
    ymd2: Ymd,
    cal: AnyCalendarKind,
    unit: DateDurationUnit
}

#[derive(Arbitrary, Debug)]
pub enum DateDurationUnit {
    Years,
    Months,
    Weeks,
    Days,
}

impl From<DateDurationUnit> for icu_calendar::types::DateDurationUnit {
    fn from(other: DateDurationUnit) -> Self {
        match other {
            DateDurationUnit::Years => Self::Years,
            DateDurationUnit::Months => Self::Months,
            DateDurationUnit::Weeks => Self::Weeks,
            DateDurationUnit::Days => Self::Days,
        }
    }
}

fuzz_target!(|data: FuzzInput| {
    let Some(date1) = data.ymd1.to_date(data.cal, true) else { return };
    let Some(date2) = data.ymd2.to_date(data.cal, true) else { return };

    let mut options = DateDifferenceOptions::default();
    options.largest_unit = Some(data.unit.into());

    let _ = date1.try_until_with_options(&date2, options);
});
