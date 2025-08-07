// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::AnyCalendar;
use icu_calendar::AnyCalendarKind;
use icu_calendar::Date;

#[cfg(test)]
use std::rc::Rc;

/// Reference: <https://tc39.es/proposal-intl-era-monthcode/#sec-temporal-calendardatearithmeticyear>
static MONOTONIC_EPOCHS: &[(AnyCalendarKind, i32)] = &[
    (AnyCalendarKind::Buddhist, -543),
    (AnyCalendarKind::Chinese, 0),
    (AnyCalendarKind::Coptic, 283),
    (AnyCalendarKind::Dangi, 0),
    (AnyCalendarKind::Ethiopian, 7),
    (AnyCalendarKind::EthiopianAmeteAlem, -5493),
    (AnyCalendarKind::Gregorian, 0),
    (AnyCalendarKind::Hebrew, -3761),
    (AnyCalendarKind::Indian, 78),
    (AnyCalendarKind::HijriTabularTypeIIFriday, 621),
    (AnyCalendarKind::HijriSimulatedMecca, 621),
    (AnyCalendarKind::HijriTabularTypeIIThursday, 621),
    (AnyCalendarKind::HijriUmmAlQura, 621),
    (AnyCalendarKind::Iso, 0),
    (AnyCalendarKind::Japanese, 0),
    (AnyCalendarKind::JapaneseExtended, 0),
    (AnyCalendarKind::Persian, 621),
    (AnyCalendarKind::Roc, 1911),
];

#[test]
fn test_monotonic_year() {
    for (kind, monotonic_epoch) in MONOTONIC_EPOCHS.iter() {
        let calendar = Rc::new(AnyCalendar::new(*kind));

        let iso_date_in_epoch_year = Date::try_new_iso(*monotonic_epoch, 12, 31).unwrap();
        let date_in_epoch_year = iso_date_in_epoch_year.to_calendar(calendar.clone());
        assert_eq!(
            date_in_epoch_year.monotonic_year(),
            0,
            "Monotonic year for {date_in_epoch_year:?} should be 0"
        );

        let iso_date_in_2025 = Date::try_new_iso(2025, 12, 31).unwrap();
        let date_in_2025 = iso_date_in_2025.to_calendar(calendar.clone());

        // The monotonic year should align with the year in the modern era or related ISO.
        // There is a special case for Japanese since it has a modern era but uses ISO for the monotonic year.
        if matches!(
            kind,
            AnyCalendarKind::Japanese | AnyCalendarKind::JapaneseExtended
        ) {
            assert_eq!(
                date_in_2025.monotonic_year(),
                2025,
                "Monotonic year for {date_in_2025:?} should be 2025"
            );
        } else {
            // Note: This code only works because 2025 is in the modern era for all calendars.
            // These two function calls are not equivalent in general.
            let expected = date_in_2025.year().era_year_or_related_iso();
            assert_eq!(
                date_in_2025.monotonic_year(),
                expected,
                "Monotonic year for {date_in_2025:?} should be {expected}"
            );
        }
    }
}
