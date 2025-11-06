use icu_calendar::{
    options::DateAddOptions, types::DateDuration, AnyCalendar, AnyCalendarKind, Date,
};
use insta::assert_debug_snapshot;

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

    for (y, m, d) in &iso_dates {
        for duration in &durations {
            for cal_kind in &calendars {
                let cal = AnyCalendar::new(*cal_kind);

                let mut date = Date::try_new_iso(*y, *m, *d)
                    .expect("Valid ISO date")
                    .to_calendar(cal.clone());

                date.try_add_with_options(*duration, opts)

                    .expect("Addition should succeed");

                let result_iso = date.to_iso();

                assert_debug_snapshot!(
                    format!("{:?}_{:?}_{:?}_{:?}", y, m, d, cal_kind),
                    (
                        format!("Start: {:?}-{:?}-{:?}", y, m, d),
                        format!("Calendar: {:?}", cal_kind),
                        format!("Duration added: {:?}", duration),
                        format!("Resulting ISO date: {:?}", result_iso)
                    )
                );
            }
        }
    }
}
