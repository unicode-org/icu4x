// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::Date;
use icu_datetime::{fieldsets, input::Time, DateTimeFormatter};
use icu_time::DateTime;

fn main() {
    let prefs = icu_host_info::datetime_preferences().expect("Failed to retrieve host info");
    let dtf = DateTimeFormatter::try_new(
        prefs,
        fieldsets::YMDT::long().with_alignment(icu_datetime::options::Alignment::Column),
    )
    .expect("Failed to create datetime formatter.");

    let date = Date::try_new_gregorian(2020, 10, 10).unwrap();
    let time = Time::try_new(18, 56, 0, 0).unwrap();

    let formatted_dt = dtf.format(&DateTime { date, time });

    println!("Today is: {formatted_dt}");
}
