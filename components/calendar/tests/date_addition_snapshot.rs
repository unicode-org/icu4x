// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{
    options::DateAddOptions, types::DateDuration, AnyCalendar, AnyCalendarKind, Date,
};
use insta::assert_snapshot;
use std::fmt::Write;

#[test]
fn test_date_add_across_calendars_and_durations() {
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
        (2000, 01, 01),
        (2000, 01, 15),
        (2000, 01, 31),
        (2000, 02, 01),
        (2000, 02, 15),
        (2000, 02, 28),
        (2000, 02, 29),
        (2000, 03, 01),
        (2000, 03, 15),
        (2000, 03, 31),
        (2000, 04, 01),
        (2000, 04, 15),
        (2000, 04, 30),
        (2000, 05, 01),
        (2000, 05, 15),
        (2000, 05, 31),
        (2000, 06, 01),
        (2000, 06, 15),
        (2000, 06, 30),
        (2000, 07, 01),
        (2000, 07, 15),
        (2000, 07, 31),
        (2000, 08, 01),
        (2000, 08, 15),
        (2000, 08, 31),
        (2000, 09, 01),
        (2000, 09, 15),
        (2000, 09, 30),
        (2000, 10, 01),
        (2000, 10, 15),
        (2000, 10, 31),
        (2000, 11, 01),
        (2000, 11, 15),
        (2000, 11, 30),
        (2000, 12, 01),
        (2000, 12, 15),
        (2000, 12, 31),
        (2001, 01, 01),
        (2001, 01, 15),
        (2001, 01, 31),
        (2001, 02, 01),
        (2001, 02, 15),
        (2001, 02, 28),
        (2001, 03, 01),
        (2001, 03, 15),
        (2001, 03, 31),
        (2001, 04, 01),
        (2001, 04, 15),
        (2001, 04, 30),
        (2001, 05, 01),
        (2001, 05, 15),
        (2001, 05, 31),
        (2001, 06, 01),
        (2001, 06, 15),
        (2001, 06, 30),
        (2001, 07, 01),
        (2001, 07, 15),
        (2001, 07, 31),
        (2001, 08, 01),
        (2001, 08, 15),
        (2001, 08, 31),
        (2001, 09, 01),
        (2001, 09, 15),
        (2001, 09, 30),
        (2001, 10, 01),
        (2001, 10, 15),
        (2001, 10, 31),
        (2001, 11, 01),
        (2001, 11, 15),
        (2001, 11, 30),
        (2001, 12, 01),
        (2001, 12, 15),
        (2001, 12, 31),
        (2002, 01, 01),
        (2002, 01, 15),
        (2002, 01, 31),
        (2002, 02, 01),
        (2002, 02, 15),
        (2002, 02, 28),
        (2002, 03, 01),
        (2002, 03, 15),
        (2002, 03, 31),
        (2002, 04, 01),
        (2002, 04, 15),
        (2002, 04, 30),
        (2002, 05, 01),
        (2002, 05, 15),
        (2002, 05, 31),
        (2002, 06, 01),
        (2002, 06, 15),
        (2002, 06, 30),
        (2002, 07, 01),
        (2002, 07, 15),
        (2002, 07, 31),
        (2002, 08, 01),
        (2002, 08, 15),
        (2002, 08, 31),
        (2002, 09, 01),
        (2002, 09, 15),
        (2002, 09, 30),
        (2002, 10, 01),
        (2002, 10, 15),
        (2002, 10, 31),
        (2002, 11, 01),
        (2002, 11, 15),
        (2002, 11, 30),
        (2002, 12, 01),
        (2002, 12, 15),
        (2002, 12, 31),
        (2003, 01, 01),
        (2003, 01, 15),
        (2003, 01, 31),
        (2003, 02, 01),
        (2003, 02, 15),
        (2003, 02, 28),
        (2003, 03, 01),
        (2003, 03, 15),
        (2003, 03, 31),
        (2003, 04, 01),
        (2003, 04, 15),
        (2003, 04, 30),
        (2003, 05, 01),
        (2003, 05, 15),
        (2003, 05, 31),
        (2003, 06, 01),
        (2003, 06, 15),
        (2003, 06, 30),
        (2003, 07, 01),
        (2003, 07, 15),
        (2003, 07, 31),
        (2003, 08, 01),
        (2003, 08, 15),
        (2003, 08, 31),
        (2003, 09, 01),
        (2003, 09, 15),
        (2003, 09, 30),
        (2003, 10, 01),
        (2003, 10, 15),
        (2003, 10, 31),
        (2003, 11, 01),
        (2003, 11, 15),
        (2003, 11, 30),
        (2003, 12, 01),
        (2003, 12, 15),
        (2003, 12, 31),
        (2004, 01, 01),
        (2004, 01, 15),
        (2004, 01, 31),
        (2004, 02, 01),
        (2004, 02, 15),
        (2004, 02, 28),
        (2004, 02, 29),
        (2004, 03, 01),
        (2004, 03, 15),
        (2004, 03, 31),
        (2004, 04, 01),
        (2004, 04, 15),
        (2004, 04, 30),
        (2004, 05, 01),
        (2004, 05, 15),
        (2004, 05, 31),
        (2004, 06, 01),
        (2004, 06, 15),
        (2004, 06, 30),
        (2004, 07, 01),
        (2004, 07, 15),
        (2004, 07, 31),
        (2004, 08, 01),
        (2004, 08, 15),
        (2004, 08, 31),
        (2004, 09, 01),
        (2004, 09, 15),
        (2004, 09, 30),
        (2004, 10, 01),
        (2004, 10, 15),
        (2004, 10, 31),
        (2004, 11, 01),
        (2004, 11, 15),
        (2004, 11, 30),
        (2004, 12, 01),
        (2004, 12, 15),
        (2004, 12, 31),
    ];

    let mut durations: Vec<DateDuration> = Vec::new();

    for d in -65i32..=65 {
        durations.push(DateDuration {
            is_negative: d < 0,
            years: 0,
            months: 0,
            weeks: 0,
            days: d.abs() as u64,
        });
    }

    for m in -30i32..=30 {
        durations.push(DateDuration {
            is_negative: m < 0,
            years: 0,
            months: m.abs() as u32,
            weeks: 0,
            days: 0,
        });
    }

    for md in -30i32..=30 {
        let day: i32 = if md < 0 {
            -1
        } else if md == 0 {
            0
        } else {
            1
        };

        durations.push(DateDuration {
            is_negative: md < 0,
            years: 0,
            months: md.abs() as u32,
            weeks: 0,
            days: day.abs() as u64,
        });
    }

    for y in -10i32..=10 {
        durations.push(DateDuration {
            is_negative: y < 0,
            years: y.abs() as u32,
            months: 0,
            weeks: 0,
            days: 0,
        });
    }

    for ym in -10i32..=10 {
        let month: i32 = if ym < 0 {
            -1
        } else if ym == 0 {
            0
        } else {
            1
        };

        durations.push(DateDuration {
            is_negative: ym < 0,
            years: ym.abs() as u32,
            months: month.abs() as u32,
            weeks: 0,
            days: 0,
        });
    }

    for yd in -10i32..=10 {
        let day: i32 = if yd < 0 {
            -1
        } else if yd == 0 {
            0
        } else {
            1
        };

        durations.push(DateDuration {
            is_negative: yd < 0,
            years: yd.abs() as u32,
            months: 0,
            weeks: 0,
            days: day.abs() as u64,
        });
    }

    for ymd in -10i32..=10 {
        let (month, day): (i32, i32) = if ymd < 0 {
            (-1, -1)
        } else if ymd == 0 {
            (0, 0)
        } else {
            (1, 1)
        };

        durations.push(DateDuration {
            is_negative: ymd < 0,
            years: ymd.abs() as u32,
            months: month.abs() as u32,
            weeks: 0,
            days: day.abs() as u64,
        });
    }

    println!("Total durations: {}", durations.len());

    let opts = DateAddOptions::default();

    for cal_kind in &calendars {
        let mut calendar_output = String::new();
        writeln!(&mut calendar_output, "Calendar: {:?}\n", cal_kind).unwrap();

        let cal = AnyCalendar::new(*cal_kind);

        for (y, m, d) in &iso_dates {
            writeln!(&mut calendar_output, "DATE: {:04}-{:02}-{:02}", y, m, d).unwrap();

            let start_date = Date::try_new_iso(*y, *m, *d)
                .expect("Valid ISO date")
                .to_calendar(cal.clone());

            for duration in &durations {
                let mut date = start_date.clone();
                date.try_add_with_options(*duration, opts)
                    .expect("Addition should succeed");

                let duration_str = format!(
                    "{}{}{}{}",
                    if duration.years != 0 {
                        format!("{}Y ", duration.years)
                    } else {
                        "".into()
                    },
                    if duration.months != 0 {
                        format!("{}M ", duration.months)
                    } else {
                        "".into()
                    },
                    if duration.weeks != 0 {
                        format!("{}W ", duration.weeks)
                    } else {
                        "".into()
                    },
                    if duration.days != 0 {
                        format!("{}D", duration.days)
                    } else {
                        "".into()
                    },
                );

                writeln!(&mut calendar_output, "  +{} → {:?}", duration_str, date).unwrap();
            }

            writeln!(&mut calendar_output).unwrap();
        }

        let snapshot_name = format!("date_add_{:?}", cal_kind);
        assert_snapshot!(snapshot_name, calendar_output);
    }
}
