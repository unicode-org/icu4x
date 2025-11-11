
use icu_calendar::{
    options::DateAddOptions, types::DateDuration, AnyCalendar, AnyCalendarKind, Date,
};
use std::fmt::Write;
use insta::assert_snapshot;

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
        (2023, 1, 1),
        (2024, 6, 15),
        (2025, 12, 31),
    ];

    let durations = vec![
        DateDuration { years: 0, months: 0, weeks: 0, days: 10, is_negative: false },
        DateDuration { years: 0, months: 2, weeks: 0, days: 0, is_negative: false },
        DateDuration { years: 1, months: 0, weeks: 0, days: 0, is_negative: false },
        DateDuration { years: 1, months: 3, weeks: 0, days: 15, is_negative: false },
    ];

    let opts = DateAddOptions::default();
    let mut output = String::new();

    for (y, m, d) in &iso_dates {
        writeln!(&mut output, "{}-{}-{}", y, m, d).unwrap();

        for cal_kind in &calendars {
            let cal = AnyCalendar::new(*cal_kind);
            // Compute per-calendar starting date
            let start_date = Date::try_new_iso(*y, *m, *d)
                .expect("Valid ISO date")
                .to_calendar(cal.clone());

            writeln!(
                &mut output,
                "  {:?} (start: {:?})",
                cal_kind,
                start_date
            )
                .unwrap();

            for duration in &durations {
                let mut date = start_date.clone();

                date.try_add_with_options(*duration, opts)
                    .expect("Addition should succeed");

                let result_iso = date.to_iso();

                let duration_str = format!(
                    "{}{}{}{}",
                    if duration.years != 0 { format!("{}y ", duration.years) } else { "".into() },
                    if duration.months != 0 { format!("{}m ", duration.months) } else { "".into() },
                    if duration.weeks != 0 { format!("{}w ", duration.weeks) } else { "".into() },
                    if duration.days != 0 { format!("{}d", duration.days) } else { "".into() },
                )
                    .trim()
                    .to_string();

                writeln!(
                    &mut output,
                    "    +{} → {:?}",
                    duration_str,
                    result_iso
                )
                    .unwrap();
            }
        }

        writeln!(&mut output).unwrap();
    }

    assert_snapshot!("date_addition_readable_nested_snapshot_with_starts", output);
}
