use icu_calendar::{
    options::DateAddOptions, types::DateDuration, AnyCalendar, AnyCalendarKind, Date,
};
use insta::assert_debug_snapshot;
use std::fmt::Write;

#[test]
fn test_date_add_across_calendars_and_durations() {
    let calendars = vec![
        AnyCalendarKind::Iso,
        AnyCalendarKind::Gregorian,
        AnyCalendarKind::Buddhist,
    ];

    let iso_dates = vec![(2023, 1, 1), (2024, 6, 15), (2025, 12, 31)];

    let durations = vec![
        DateDuration {
            years: 0,
            months: 0,
            weeks: 0,
            days: 10,
            is_negative: false,
        },
        DateDuration {
            years: 0,
            months: 2,
            weeks: 0,
            days: 0,
            is_negative: false,
        },
        DateDuration {
            years: 1,
            months: 0,
            weeks: 0,
            days: 0,
            is_negative: false,
        },
        DateDuration {
            years: 1,
            months: 3,
            weeks: 0,
            days: 15,
            is_negative: false,
        },
    ];

    let opts = DateAddOptions::default();
    let mut output = String::new();

    for (y, m, d) in &iso_dates {
        for duration in &durations {
            for cal_kind in &calendars {
                let cal = AnyCalendar::new(*cal_kind);

                let mut date = Date::try_new_iso(*y, *m, *d)
                    .expect("Valid ISO date")
                    .to_calendar(cal.clone());

                let _ = date.try_add_with_options(*duration, opts);
                let result_iso = date.to_iso();

                // Append to big string instead of making multiple snapshots
                writeln!(
                    &mut output,
                    "Start: {:?}-{:?}-{:?}\nCalendar: {:?}\nDuration: {:?}\nResult: {:?}\n",
                    y, m, d, cal_kind, duration, result_iso
                ).unwrap();
            }
        }
    }

    insta::assert_snapshot!("date_addition_combined_snapshot", output);
}
