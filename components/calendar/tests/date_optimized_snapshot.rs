// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{AnyCalendar, AnyCalendarKind, Date};
use std::fmt::Write;

#[test]
#[ignore = "Snapshot not committed; run locally when requested"]
fn test_generate_dates_and_durations_for_review() {
    let calendars = vec![
        AnyCalendarKind::Iso,
        AnyCalendarKind::Gregorian,
        AnyCalendarKind::Buddhist,
        AnyCalendarKind::Chinese,
        AnyCalendarKind::Coptic,
        AnyCalendarKind::Dangi,
        AnyCalendarKind::Ethiopian,
        AnyCalendarKind::EthiopianAmeteAlem,
        AnyCalendarKind::Hebrew,
        AnyCalendarKind::HijriSimulatedMecca,
        AnyCalendarKind::HijriTabularTypeIIFriday,
        AnyCalendarKind::HijriTabularTypeIIThursday,
        AnyCalendarKind::HijriUmmAlQura,
        AnyCalendarKind::Indian,
        AnyCalendarKind::Japanese,
        AnyCalendarKind::JapaneseExtended,
        AnyCalendarKind::Persian,
        AnyCalendarKind::Roc,
    ];

    let iso_dates = vec![
        (2000, 1, 1),
        (2000, 2, 28),
        (2000, 2, 29),
        (2000, 3, 1),
        (2001, 2, 28),
        (2004, 2, 28),
        (2004, 2, 29),
        (2004, 3, 1),
        (2004, 12, 31),
    ];

    let mut durations: Vec<(i32, i32, i32)> = Vec::new();

    for d in -65..=65 {
        durations.push((0, 0, d));
    }

    for m in -30..=30 {
        durations.push((0, m, 0));
    }

    for md in -30..=30 {
        let day = if md < 0 {
            -1
        } else if md == 0 {
            0
        } else {
            1
        };
        durations.push((0, md, day));
    }

    for y in -10..=10 {
        durations.push((y, 0, 0));
    }

    for ym in -10..=10 {
        let month = if ym < 0 {
            -1
        } else if ym == 0 {
            0
        } else {
            1
        };
        durations.push((ym, month, 0));
    }

    for yd in -10..=10 {
        let day = if yd < 0 {
            -1
        } else if yd == 0 {
            0
        } else {
            1
        };
        durations.push((yd, 0, day));
    }

    for ymd in -10..=10 {
        let (month, day) = if ymd < 0 {
            (-1, -1)
        } else if ymd == 0 {
            (0, 0)
        } else {
            (1, 1)
        };
        durations.push((ymd, month, day));
    }

    let mut output = String::new();

    writeln!(&mut output, "Durations total: {}", durations.len()).unwrap();
    writeln!(&mut output).unwrap();

    for cal_kind in &calendars {
        let cal = AnyCalendar::new(*cal_kind);
        writeln!(&mut output, "Calendar: {:?}", cal_kind).unwrap();

        for (y, m, d) in &iso_dates {
            let start_date = Date::try_new_iso(*y, *m, *d)
                .expect("Valid ISO date")
                .to_calendar(cal.clone());

            writeln!(
                &mut output,
                "  START: {:04}-{:02}-{:02} -> {:?}",
                y, m, d, start_date
            )
            .unwrap();

            let show_n = 8usize;
            for (i, dur) in durations.iter().enumerate() {
                if i < show_n || i + show_n >= durations.len() {
                    writeln!(
                        &mut output,
                        "    +{:>4}Y {:>4}M {:>4}D",
                        dur.0, dur.1, dur.2
                    )
                    .unwrap();
                } else if i == show_n {
                    writeln!(
                        &mut output,
                        "    ... {} omitted ...",
                        durations.len() - (show_n * 2)
                    )
                    .unwrap();
                }
            }

            writeln!(&mut output).unwrap();
        }

        writeln!(&mut output).unwrap();
    }

    assert!(!output.is_empty());

    println!("{}", output);
}
