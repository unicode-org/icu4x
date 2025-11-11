use icu_calendar::{
    options::DateAddOptions, types::DateDuration, AnyCalendar, AnyCalendarKind, Date,
};
use std::collections::BTreeMap;
use insta::assert_debug_snapshot;

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

    let iso_dates = vec![(2023, 1, 1), (2024, 6, 15), (2025, 12, 31)];

    let durations = vec![
        DateDuration { years: 0, months: 0, weeks: 0, days: 10, is_negative: false },
        DateDuration { years: 0, months: 2, weeks: 0, days: 0, is_negative: false },
        DateDuration { years: 1, months: 0, weeks: 0, days: 0, is_negative: false },
        DateDuration { years: 1, months: 3, weeks: 0, days: 15, is_negative: false },
    ];

    let opts = DateAddOptions::default();

    let mut output = BTreeMap::new();

    for (y, m, d) in &iso_dates {
        let mut cal_map = BTreeMap::new();

        for cal_kind in &calendars {
            let cal = AnyCalendar::new(*cal_kind);
            let mut results = Vec::new();

            for duration in &durations {
                let mut date = Date::try_new_iso(*y, *m, *d)
                    .expect("Valid ISO date")
                    .to_calendar(cal.clone());

                date.try_add_with_options(*duration, opts)
                    .expect("Addition should succeed");

                let result_iso = date.to_iso();

                results.push((
                    format!("{:?}", duration),
                    format!("{:?}", result_iso),
                ));
            }

            cal_map.insert(format!("{:?}", cal_kind), results);
        }

        output.insert(format!("{}-{}-{}", y, m, d), cal_map);
    }

    assert_debug_snapshot!("date_addition_nested_snapshot", output);
}
