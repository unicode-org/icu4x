// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::calendar::Date;
use icu::datetime::{fieldsets, DateTimeFormatter};
use icu::locale::locale;
use icu::timezone::Time;

fn main() {
    let mapper = icu::timezone::TimeZoneIdMapper::new();
    let offsets = icu::timezone::ZoneOffsetCalculator::new();

    let prefs = locale!("en").into();

    let offset_formatter = DateTimeFormatter::try_new(prefs, fieldsets::O::new()).unwrap();
    let non_location_formatter = DateTimeFormatter::try_new(prefs, fieldsets::V::new()).unwrap();
    let city_formatter = DateTimeFormatter::try_new(prefs, fieldsets::X::new()).unwrap();

    let max_date = (Date::try_new_iso(2025, 1, 1).unwrap(), Time::midnight());

    let mut tzs = Vec::new();

    for tz in mapper.iter_bcp47() {
        if tz.0 == "unk" || tz.starts_with("utc") || tz.0 == "gmt" {
            continue;
        }

        let standard_offset = offsets
            .compute_offsets_from_time_zone(tz, max_date)
            .unwrap()
            .standard;

        let tzi = tz.with_offset(Some(standard_offset)).at_time(max_date);

        tzs.push((
            -tzi.offset().unwrap().to_seconds(),
            format!("({})", offset_formatter.format_any_calendar(&tzi)),
            non_location_formatter.format_any_calendar(&tzi).to_string(),
            city_formatter.format_any_calendar(&tzi).to_string(),
        ));
    }

    tzs.sort();

    for (_, offset, non_location, city) in &tzs {
        println!(
            "{offset:0$} {non_location} - {city}",
            tzs.iter().map(|(_, l, ..)| l.len()).max().unwrap()
        );
    }
}
