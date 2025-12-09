// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main]

mod common;

use common::{Ymd, AnyCalendarKind};

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    ymd: Ymd,
    overflow_constrain: bool,
    cal: AnyCalendarKind,
}

fuzz_target!(|data: FuzzInput| {
    if let Some(date) = data.ymd.to_date(data.cal, data.overflow_constrain) {
        let _ = date.day_of_month();
        let _ = date.weekday();
        let _ = date.day_of_year();
        let _ = date.days_in_month();
        let _ = date.days_in_year();
        let _ = date.extended_year();
        let _ = date.is_in_leap_year();
        let _ = date.month();
        let _ = date.months_in_year();
        let _ = date.year();
    }
});
