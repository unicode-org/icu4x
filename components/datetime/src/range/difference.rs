// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(
    not(test),
    allow(
        unused,
        reason = "TODO(#5448): difference resolution is not yet used in non-test code"
    )
)]

use crate::format::DateTimeInputUnchecked;
use crate::provider::names::DayPeriodNames;
use icu_calendar::types::YearInfo;
use icu_time::Hour;

/// The greatest difference between two datetimes.
///
/// Ordered from smallest to largest difference.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Difference {
    /// No difference (inputs are identical).
    None,
    /// Difference in second or subsecond (leads to fallback).
    Second,
    /// Difference in minute.
    Minute,
    /// Difference in hour.
    Hour,
    /// Difference in flexible day period.
    DayPeriodB,
    /// Difference in standard day period (AM/PM).
    DayPeriodA,
    /// Difference in day.
    Day,
    /// Difference in month.
    Month,
    /// Difference in year.
    Year,
    /// Difference in era.
    Era,
    /// Incomparable difference (e.g., timezone difference, different calendars).
    Incomparable,
}

impl Difference {
    /// Returns whether the difference is in a time field (minute, hour, or day period).
    pub(crate) fn is_time_diff(self) -> bool {
        matches!(
            self,
            Self::Minute | Self::Hour | Self::DayPeriodB | Self::DayPeriodA
        )
    }
    /// Returns whether the difference is in a date field (day, month, year, era).
    pub(crate) fn is_date_diff(self) -> bool {
        matches!(self, Self::Day | Self::Month | Self::Year | Self::Era)
    }
}
/// The fields of the input that take part in the comparison.
///
/// A field set only shows some of the fields of a date, so a difference in another field
/// must not turn `December 22` into `December 22 – December 22`. A typed field set is only
/// given its own fields, but a dynamic one (`CompositeDateTimeFieldSet`,
/// `CompositeFieldSet`) is given all of them.
///
/// Time fields are not listed: a time difference finer than the field set is handled by
/// the threshold of `contains_difference`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct DifferenceFields {
    /// The era and the year.
    pub(crate) year: bool,
    pub(crate) month: bool,
    pub(crate) day_of_month: bool,
    pub(crate) weekday: bool,
    pub(crate) zone: bool,
}

impl DifferenceFields {
    /// Compare everything.
    pub(crate) const ALL: Self = Self {
        year: true,
        month: true,
        day_of_month: true,
        weekday: true,
        zone: true,
    };
}

/// Resolves the greatest difference between two datetimes.
///
/// If `dayperiod_names` is provided, it will be used to resolve flexible day periods (`B`).
/// If it is `None`, flexible day periods will be assumed to be the same, and only
/// standard AM/PM (`a`) will be compared.
pub(crate) fn resolve_difference(
    input1: &DateTimeInputUnchecked,
    input2: &DateTimeInputUnchecked,
    dayperiod_names: Option<&DayPeriodNames<'_>>,
    fields: DifferenceFields,
) -> Difference {
    if fields.zone && !input1.has_same_zone(input2) {
        return Difference::Incomparable;
    }

    // Compare Date fields
    let (year1, year2) = if fields.year {
        (input1.year, input2.year)
    } else {
        (None, None)
    };
    match (year1, year2) {
        (Some(YearInfo::Era(e1)), Some(YearInfo::Era(e2))) => {
            if e1.era != e2.era {
                return Difference::Era;
            }
            if e1.year != e2.year {
                return Difference::Year;
            }
        }
        (Some(YearInfo::Cyclic(c1)), Some(YearInfo::Cyclic(c2))) => {
            if c1.related_iso != c2.related_iso {
                return Difference::Year;
            }
        }
        (None, None) => {}
        _ => {
            // One input has a year and the other does not (Some/None mismatch),
            // or they have different year types (e.g., Era vs Cyclic).
            // This is a major structural difference, so we fall back to Era
            // (the largest date-specific difference).
            return Difference::Era;
        }
    }

    let month1 = input1.month.map(|m| m.to_input());
    let month2 = input2.month.map(|m| m.to_input());
    if fields.month && month1 != month2 {
        // This also catches the Some != None case in case
        // one input chooses to not use month codes. This is
        // expected: date time formatting typically needs month codes
        // to work and "unspecified" should count as a difference.
        return Difference::Month;
    }

    if fields.day_of_month && input1.day_of_month != input2.day_of_month {
        return Difference::Day;
    }

    // A weekday-only field set has no day-of-month to compare.
    if fields.weekday && input1.weekday != input2.weekday {
        return Difference::Day;
    }

    // Compare Time fields
    match (input1.hour, input2.hour) {
        (Some(h1), Some(h2)) => {
            if h1 != h2 {
                // Check standard AM/PM (DayPeriodA)
                let ampm1 = h1.number() < 12;
                let ampm2 = h2.number() < 12;
                if ampm1 != ampm2 {
                    return Difference::DayPeriodA;
                }

                // Check flexible day period (DayPeriodB)
                if dayperiod_names.is_some_and(|dp| has_flexible_day_period_difference(dp, h1, h2))
                {
                    return Difference::DayPeriodB;
                }
                return Difference::Hour;
            }
        }
        (None, None) => {}
        _ => {
            // One input has an hour and the other does not (Some/None mismatch).
            return Difference::Hour;
        }
    }

    if input1.minute != input2.minute {
        return Difference::Minute;
    }

    if input1.second != input2.second || input1.subsecond != input2.subsecond {
        return Difference::Second;
    }

    Difference::None
}

fn has_flexible_day_period_difference(
    dayperiod_names: &DayPeriodNames<'_>,
    hour1: Hour,
    hour2: Hour,
) -> bool {
    if let Some(rules) = dayperiod_names.day_period_rules() {
        return rules.name_offset(hour1) != rules.name_offset(hour2);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_calendar::Date;
    use icu_time::Time;

    fn create_input(y: i32, m: u8, d: u8, hr: u8, min: u8, sec: u8) -> DateTimeInputUnchecked {
        let mut input = DateTimeInputUnchecked::default();
        let date = Date::try_new_iso(y, m, d).unwrap();
        input.set_date_fields_unchecked(date);
        let time = Time::try_new(hr, min, sec, 0).unwrap();
        input.set_time_fields(time);
        input
    }

    #[test]
    fn test_identical() {
        let input1 = create_input(2024, 1, 15, 10, 30, 0);
        let input2 = create_input(2024, 1, 15, 10, 30, 0);
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::None
        );
    }

    #[test]
    fn test_year_diff() {
        let input1 = create_input(2024, 1, 15, 10, 30, 0);
        let input2 = create_input(2025, 1, 15, 10, 30, 0);
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::Year
        );
    }

    #[test]
    fn test_month_diff() {
        let input1 = create_input(2024, 1, 15, 10, 30, 0);
        let input2 = create_input(2024, 2, 15, 10, 30, 0);
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::Month
        );
    }

    #[test]
    fn test_day_diff() {
        let input1 = create_input(2024, 1, 15, 10, 30, 0);
        let input2 = create_input(2024, 1, 16, 10, 30, 0);
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::Day
        );
    }

    #[test]
    fn test_hour_diff_same_ampm() {
        let input1 = create_input(2024, 1, 15, 10, 30, 0);
        let input2 = create_input(2024, 1, 15, 11, 30, 0);
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::Hour
        );
    }

    #[test]
    fn test_hour_diff_different_ampm() {
        let input1 = create_input(2024, 1, 15, 10, 30, 0);
        let input2 = create_input(2024, 1, 15, 22, 30, 0); // 10 PM
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::DayPeriodA
        );
    }

    #[test]
    fn test_minute_diff() {
        let input1 = create_input(2024, 1, 15, 10, 30, 0);
        let input2 = create_input(2024, 1, 15, 10, 31, 0);
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::Minute
        );
    }

    #[test]
    fn test_second_diff() {
        let input1 = create_input(2024, 1, 15, 10, 30, 0);
        let input2 = create_input(2024, 1, 15, 10, 30, 1);
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::Second
        );
    }

    #[test]
    fn test_timezone_diff() {
        let mut input1 = create_input(2024, 1, 15, 10, 30, 0);
        let mut input2 = create_input(2024, 1, 15, 10, 30, 0);
        input1.zone_offset = Some(icu_time::zone::UtcOffset::zero());
        input2.zone_offset = Some(icu_time::zone::UtcOffset::try_from_seconds(3600).unwrap());
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::Incomparable
        );
    }

    #[test]
    fn test_fields_outside_the_field_set_are_not_compared() {
        let year_only = create_input(2023, 12, 22, 9, 0, 0);
        let other_year = create_input(2024, 12, 22, 9, 0, 0);
        // `MD`: the year is not shown, and neither is the weekday that follows from it.
        let md = DifferenceFields {
            year: false,
            weekday: false,
            ..DifferenceFields::ALL
        };
        assert_eq!(
            resolve_difference(&year_only, &other_year, None, DifferenceFields::ALL),
            Difference::Year
        );
        assert_eq!(
            resolve_difference(&year_only, &other_year, None, md),
            Difference::None
        );
    }

    #[test]
    fn test_zone_is_compared_only_with_a_zone_field() {
        let mut input1 = create_input(2024, 1, 15, 10, 30, 0);
        let mut input2 = create_input(2024, 1, 15, 10, 30, 0);
        input1.zone_offset = Some(icu_time::zone::UtcOffset::zero());
        input2.zone_offset = Some(icu_time::zone::UtcOffset::try_from_seconds(3600).unwrap());
        let no_zone = DifferenceFields {
            zone: false,
            ..DifferenceFields::ALL
        };
        assert_eq!(
            resolve_difference(&input1, &input2, None, DifferenceFields::ALL),
            Difference::Incomparable
        );
        assert_eq!(
            resolve_difference(&input1, &input2, None, no_zone),
            Difference::None
        );
    }

    /// A weekday-only field set has no day-of-month, so the weekday is what differs.
    #[test]
    fn test_weekday_only_diff() {
        let weekday_only = DifferenceFields {
            year: false,
            month: false,
            day_of_month: false,
            weekday: true,
            zone: false,
        };
        let friday = create_input(2023, 12, 22, 0, 0, 0);
        let saturday = create_input(2023, 12, 23, 0, 0, 0);
        let next_friday = create_input(2023, 12, 29, 0, 0, 0);
        assert_eq!(
            resolve_difference(&friday, &saturday, None, weekday_only),
            Difference::Day
        );
        assert_eq!(
            resolve_difference(&friday, &next_friday, None, weekday_only),
            Difference::None
        );
    }
}
