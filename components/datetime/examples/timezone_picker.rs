// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::io::{BufWriter, Write};

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
    let location_formatter = DateTimeFormatter::try_new(prefs, fieldsets::L::new()).unwrap();
    let non_location_specific_formatter =
        DateTimeFormatter::try_new(prefs, fieldsets::Z::new()).unwrap();
    let city_formatter = DateTimeFormatter::try_new(prefs, fieldsets::X::new()).unwrap();

    let reference_date = (Date::try_new_iso(2025, 1, 1).unwrap(), Time::midnight());

    let mut list = Vec::new();

    for tz in mapper.iter_bcp47() {
        let offsets = offsets
            .compute_offsets_from_time_zone(tz, reference_date)
            .unwrap();

        let tzi = tz
            .with_offset(Some(offsets.standard))
            .at_time(reference_date)
            .with_zone_variant(icu_timezone::ZoneVariant::Standard);

        list.push((
            -offsets.standard.to_seconds(),
            format!(
                "{}{}",
                offset_formatter.format(&tzi),
                if let Some(daylight) = offsets.daylight {
                    format!(
                        "/{}",
                        offset_formatter.format(
                            &tzi.time_zone_id()
                                .with_offset(Some(daylight))
                                .at_time(reference_date)
                        )
                    )
                } else {
                    String::new()
                }
            ),
            non_location_formatter.format(&tzi).to_string(),
            non_location_specific_formatter.format(&tzi).to_string(),
            location_formatter.format(&tzi).to_string(),
            city_formatter.format(&tzi).to_string(),
            tzi.time_zone_id().to_string(),
        ));
    }

    list.sort_by(|a, b| (a.0, &a.6).cmp(&(b.0, &b.6)));

    let o_len = list.iter().map(|(_, o, ..)| o.len()).max().unwrap();
    let v_len = list.iter().map(|(_, _, v, ..)| v.len()).max().unwrap();
    let z_len = list.iter().map(|(_, _, _, z, ..)| z.len()).max().unwrap();
    let l_len = list
        .iter()
        .map(|(_, _, _, _, l, ..)| l.len())
        .max()
        .unwrap();
    let x_len = list
        .iter()
        .map(|(_, _, _, _, _, x, _)| x.len())
        .max()
        .unwrap();
    let i_len = list
        .iter()
        .map(|(_, _, _, _, _, _, i)| i.len())
        .max()
        .unwrap();

    let mut file = BufWriter::new(
        std::fs::File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/tz_list.md")).unwrap(),
    );

    writeln!(
        &mut file,
        "|{:o_len$}|{:v_len$}|{:z_len$}|{:l_len$}|{:x_len$}|{:i_len$}",
        "OOOO", "vvvv", "zzzz (standard)", "VVVV", "VVV", "V"
    )
    .unwrap();
    writeln!(
        &mut file,
        "|{:-<o_len$}|{:-<v_len$}|{:-<z_len$}|{:-<l_len$}|{:-<x_len$}|{:-<i_len$}",
        "", "", "", "", "", ""
    )
    .unwrap();

    for (_, o, v, z, l, x, i) in &list {
        writeln!(
            &mut file,
            "|{o:o_len$}|{v:v_len$}|{z:z_len$}|{l:l_len$}|{x:x_len$}|{i:i_len$}|",
        )
        .unwrap();
    }
}
