// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use icu::datetime::{fieldsets, NoCalendarFormatter};
use icu::locale::locale;
use icu::time::zone::ZoneNameTimestamp;
use icu_time::zone::UtcOffset;

fn main() {
    let parser = icu::time::zone::iana::IanaParserExtended::new();

    let prefs = locale!("en").into();

    let offset_formatter =
        NoCalendarFormatter::try_new(prefs, fieldsets::zone::LocalizedOffsetLong).unwrap();
    let non_location_formatter =
        NoCalendarFormatter::try_new(prefs, fieldsets::zone::GenericLong).unwrap();
    let city_formatter =
        NoCalendarFormatter::try_new(prefs, fieldsets::zone::ExemplarCity).unwrap();

    let reference_timestamp = ZoneNameTimestamp::far_in_future();

    let mut grouped_tzs = BTreeMap::<_, Vec<_>>::new();

    for tz in parser.iter() {
        if tz.time_zone.is_unknown()
            || tz.time_zone.as_str().starts_with("utc")
            || tz.time_zone.as_str() == "gmt"
        {
            continue;
        }

        let jiff = jiff::tz::TimeZone::get(tz.canonical).unwrap();
        let curr_offset = jiff.to_offset(jiff::Timestamp::now());
        let next_offset = jiff
            .following(jiff::Timestamp::now())
            .next()
            .map(|t| t.offset());

        let (lo, hi) = if let Some(next_offset) = next_offset {
            if next_offset.seconds() < curr_offset.seconds() {
                (next_offset, Some(curr_offset))
            } else {
                (curr_offset, Some(next_offset))
            }
        } else {
            (curr_offset, None)
        };

        let tzi = tz
            .time_zone
            .with_offset(Some(UtcOffset::from_seconds_unchecked(lo.seconds())))
            .with_zone_name_timestamp(reference_timestamp);

        grouped_tzs
            .entry(non_location_formatter.format(&tzi).to_string())
            .or_default()
            .push(((lo, hi), tzi));
    }

    let mut list = Vec::new();

    for (non_location, zones) in grouped_tzs {
        for ((lo, hi), tzi) in &zones {
            list.push((
                -lo.seconds(),
                format!(
                    "({}{})",
                    offset_formatter.format(tzi),
                    if let Some(hi) = hi {
                        format!(
                            "/{}",
                            offset_formatter.format(
                                &tzi.id()
                                    .with_offset(Some(UtcOffset::from_seconds_unchecked(
                                        hi.seconds()
                                    )))
                                    .with_zone_name_timestamp(reference_timestamp)
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
