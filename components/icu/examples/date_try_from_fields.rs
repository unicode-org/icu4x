// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main]
use icu::calendar::Ref;
icu_benchmark_macros::instrument!();
use icu_benchmark_macros::println;
use icu_calendar::types::DateFields;
use icu_calendar::{AnyCalendar, AnyCalendarKind, Date};

const CALENDAR_KINDS: &[AnyCalendarKind] = &[
    AnyCalendarKind::Buddhist,
    AnyCalendarKind::Chinese,
    AnyCalendarKind::Gregorian,
    AnyCalendarKind::Indian,
    AnyCalendarKind::Japanese,
    AnyCalendarKind::Ethiopian,
];

fn main() {
    for &kind in CALENDAR_KINDS {
        let cal = AnyCalendar::new(kind);

        let mut fields = DateFields::default();
        fields.extended_year = Some(2025);
        fields.month_code = Some(b"M07");
        fields.day = Some(8);

        let date = Date::try_from_fields(fields, Default::default(), Ref(&cal));

        println!("Constructed date for {:?} = {:?}", kind, date);
    }
}
