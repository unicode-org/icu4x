// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use icu::calendar::Date;
use icu::datetime::{fieldsets, NoCalendarFormatter};
use icu::locale::locale;
use icu::time::{DateTime, Time};
use icu_time::zone::ZoneNameTimestamp;
use icu_time::TimeZone;

fn main() {
    let parser = icu::time::zone::IanaParser::new();
    let offsets = icu::time::zone::VariantOffsetsCalculator::new();

    let prefs = locale!("en").into();

    let offset_formatter =
        NoCalendarFormatter::try_new(prefs, fieldsets::zone::LocalizedOffsetLong).unwrap();
    let non_location_formatter =
        NoCalendarFormatter::try_new(prefs, fieldsets::zone::GenericLong).unwrap();
    let city_formatter =
        NoCalendarFormatter::try_new(prefs, fieldsets::zone::ExemplarCity).unwrap();

    let reference_date_time = DateTime {
        date: Date::try_new_iso(2025, 1, 1).unwrap(),
        time: Time::start_of_day(),
    };

    let mut grouped_tzs = BTreeMap::<_, Vec<_>>::new();

    for tz in parser.iter() {
        if tz == TimeZone::unknown() || tz.as_str().starts_with("utc") || tz.as_str() == "gmt" {
            continue;
        }

        let offsets = offsets
            .compute_offsets_from_time_zone(
                tz,
                ZoneNameTimestamp::from_date_time_iso(&reference_date_time),
            )
            .unwrap();

        let tzi = tz
            .with_offset(Some(offsets.standard))
            .at_date_time_iso(&reference_date_time);

        grouped_tzs
            .entry(non_location_formatter.format(&tzi).to_string())
            .or_default()
            .push((offsets, tzi));
    }

    let mut list = Vec::new();

    for (non_location, zones) in grouped_tzs {
        for (offsets, tzi) in &zones {
            list.push((
                -offsets.standard.to_seconds(),
                format!(
                    "({}{})",
                    offset_formatter.format(tzi),
                    if let Some(daylight) = offsets.daylight {
                        format!(
                            "/{}",
                            offset_formatter.format(
                                &tzi.id()
                                    .with_offset(Some(daylight))
                                    .at_date_time_iso(&reference_date_time)
                            )
                        )
                    } else {
                        String::new()
                    }
                ),
                if zones.len() == 1 {
                    non_location.clone()
                } else {
                    format!("{non_location} - {}", city_formatter.format(tzi))
                },
            ));
        }
    }

    list.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));

    for (_, offset, non_location) in &list {
        println!(
            "{offset:0$} {non_location}",
            list.iter().map(|(_, l, ..)| l.len()).max().unwrap()
        );
    }
}
