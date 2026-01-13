// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// A signed length of time in terms of days, weeks, months, and years.
///
/// This type represents the abstract concept of a date duration. For example, a duration of
/// "1 month" is represented as "1 month" in the data model, without any context of how many
/// days the month might be.
///
/// Use [`DateDuration`] for calculating the difference between two [`Date`]s and adding
/// date units to a [`Date`].
///
/// [`Date`]: crate::Date
///
/// # Example
///
/// ```rust
/// use icu::calendar::options::DateDifferenceOptions;
/// use icu::calendar::types::DateDuration;
/// use icu::calendar::types::DateDurationUnit;
/// use icu::calendar::types::Weekday;
/// use icu::calendar::Date;
///
/// // Creating ISO date: 1992-09-02.
/// let mut date_iso = Date::try_new_iso(1992, 9, 2)
///     .expect("Failed to initialize ISO Date instance.");
///
/// assert_eq!(date_iso.weekday(), Weekday::Wednesday);
/// assert_eq!(date_iso.era_year().year, 1992);
/// assert_eq!(date_iso.month().ordinal, 9);
/// assert_eq!(date_iso.day_of_month().0, 2);
///
/// // Answering questions about days in month and year.
/// assert_eq!(date_iso.days_in_year(), 366);
/// assert_eq!(date_iso.days_in_month(), 30);
///
/// // Advancing date in-place by 1 year, 2 months, 3 weeks, 4 days.
/// date_iso
///     .try_add_with_options(
///         DateDuration {
///             is_negative: false,
///             years: 1,
///             months: 2,
///             weeks: 3,
///             days: 4,
///         },
///         Default::default(),
///     )
///     .unwrap();
/// assert_eq!(date_iso.era_year().year, 1993);
/// assert_eq!(date_iso.month().ordinal, 11);
/// assert_eq!(date_iso.day_of_month().0, 27);
///
/// // Reverse date advancement.
/// date_iso
///     .try_add_with_options(
///         DateDuration {
///             is_negative: true,
///             years: 1,
///             months: 2,
///             weeks: 3,
///             days: 4,
///         },
///         Default::default(),
///     )
///     .unwrap();
/// assert_eq!(date_iso.era_year().year, 1992);
/// assert_eq!(date_iso.month().ordinal, 9);
/// assert_eq!(date_iso.day_of_month().0, 2);
///
/// // Creating ISO date: 2022-01-30.
/// let newer_date_iso = Date::try_new_iso(2022, 10, 30)
///     .expect("Failed to initialize ISO Date instance.");
///
/// // Comparing dates: 2022-01-30 and 1992-09-02.
/// let mut options = DateDifferenceOptions::default();
/// options.largest_unit = Some(DateDurationUnit::Years);
/// let Ok(duration) =
///     newer_date_iso.try_until_with_options(&date_iso, options);
/// assert_eq!(duration.years, 30);
/// assert_eq!(duration.months, 1);
/// assert_eq!(duration.days, 28);
///
/// // Create new date with date advancement. Reassign to new variable.
/// let mutated_date_iso = date_iso
///     .try_added_with_options(
///         DateDuration {
///             is_negative: false,
///             years: 1,
///             months: 2,
///             weeks: 3,
///             days: 4,
///         },
///         Default::default(),
///     )
///     .unwrap();
/// assert_eq!(mutated_date_iso.era_year().year, 1993);
/// assert_eq!(mutated_date_iso.month().ordinal, 11);
/// assert_eq!(mutated_date_iso.day_of_month().0, 27);
/// ```
///
/// Currently unstable for ICU4X 1.0
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not use this type unless you are prepared for things to occasionally break.
///
/// Graduation tracking issue: [issue #3964](https://github.com/unicode-org/icu4x/issues/3964).
/// </div>
///
/// ✨ *Enabled with the `unstable` Cargo feature.*
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
#[allow(clippy::exhaustive_structs)] // spec-defined in Temporal
pub struct DateDuration {
    /// Whether the duration is negative.
    ///
    /// A negative duration is an abstract concept that could result, for example, from
    /// taking the difference between two [`Date`](crate::Date)s.
    ///
    /// The fields of the duration are either all positive or all negative. Mixed signs
    /// are not allowed.
    ///
    /// By convention, this field should be `false` if the duration is zero.
    pub is_negative: bool,
    /// The number of years
    pub years: u32,
    /// The number of months
    pub months: u32,
    /// The number of weeks
    pub weeks: u32,
    /// The number of days
    pub days: u64,
}

/// A "duration unit" used to specify the minimum or maximum duration of time to
/// care about
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not use this type unless you are prepared for things to occasionally break.
///
/// Graduation tracking issue: [issue #3964](https://github.com/unicode-org/icu4x/issues/3964).
/// </div>
///
/// ✨ *Enabled with the `unstable` Cargo feature.*
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(clippy::exhaustive_enums)] // this type should be stable
pub enum DateDurationUnit {
    /// Duration in years
    Years,
    /// Duration in months
    Months,
    /// Duration in weeks
    Weeks,
    /// Duration in days
    Days,
}

/// Errors that can occur when parsing an ISO 8601 date-only duration string.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DateDurationParseError {
    /// The input does not follow the expected ISO 8601 date only duration structure.
    ///
    /// This error occurs when the duration string is incomplete,
    /// or contains unexpected characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu::calendar::types::{DateDuration, DateDurationParseError};
    ///
    /// assert_eq!(DateDuration::try_from_str("P"),  Err(DateDurationParseError::InvalidStructure));
    /// assert_eq!(DateDuration::try_from_str("P1"), Err(DateDurationParseError::InvalidStructure));
    /// ```
    InvalidStructure,

    /// The duration contains a time component, which is not supported.
    ///
    /// Only date based units (`Y`, `M`, `W`, `D`) are supported. Any duration
    /// containing a `T` time separator is rejected.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu::calendar::types::{DateDuration, DateDurationParseError} ;
    ///
    /// assert_eq!(DateDuration::try_from_str("PT5M"), Err(DateDurationParseError::TimeNotSupported));
    /// assert_eq!(DateDuration::try_from_str("P1DT"), Err(DateDurationParseError::TimeNotSupported));
    /// ```
    TimeNotSupported,

    /// A duration unit appeared without a number before it.
    ///
    /// For example, the string contains `Y`, `M`, `W`, or `D` without a
    /// numeric value directly in front of it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu::calendar::types::{DateDuration, DateDurationParseError};
    ///
    /// assert_eq!(DateDuration::try_from_str("PY"), Err(DateDurationParseError::MissingValue));
    /// assert_eq!(DateDuration::try_from_str("PX1D"), Err(DateDurationParseError::MissingValue));
    /// ```
    MissingValue,

    /// A duration unit was specified more than once.
    ///
    /// Each unit (`Y`, `M`, `W`, `D`) may appear at most once only.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu::calendar::types::{DateDuration, DateDurationParseError};
    ///
    /// assert_eq!(DateDuration::try_from_str("P1Y2Y"), Err(DateDurationParseError::DuplicateUnit));
    /// assert_eq!(DateDuration::try_from_str("P1D1D"), Err(DateDurationParseError::DuplicateUnit));
    /// ```
    DuplicateUnit,

    /// A numeric value exceeded or was more than the supported range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu::calendar::types::{DateDuration, DateDurationParseError};
    ///
    /// assert_eq!(DateDuration::try_from_str("P4294967296Y"), Err(DateDurationParseError::NumberOverflow));
    /// ```
    NumberOverflow,

    /// A duration starts with a `+` sign, which is not allowed.
    ///
    /// Only negative durations using a leading `-` are supported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use icu::calendar::types::{DateDuration, DateDurationParseError};
    ///
    /// assert_eq!(DateDuration::try_from_str("+P1D"), Err(DateDurationParseError::PlusNotAllowed));
    /// ```
    PlusNotAllowed,
}

impl DateDuration {
    /// Returns a new [`DateDuration`] representing a number of years.
    pub fn for_years(years: i32) -> Self {
        Self {
            is_negative: years.is_negative(),
            years: years.unsigned_abs(),
            ..Default::default()
        }
    }
}

impl DateDuration {
    /// Parses an ISO 8601 date-only duration string into a [`DateDuration`].
    ///
    /// This is a wrapper around [`Self::try_from_utf8`] for UTF-8
    /// string inputs.
    pub fn try_from_str(s: &str) -> Result<Self, DateDurationParseError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// Parses an ISO 8601 date-only duration from UTF-8 encoded bytes.
    ///
    /// This function operates directly on byte slices in order to support
    /// unvalidated UTF-8 inputs.
    ///
    /// Supported units are years (`Y`), months (`M`), weeks (`W`), and days (`D`).
    /// Time based components are rejected.
    pub fn try_from_utf8(code_units: &[u8]) -> Result<Self, DateDurationParseError> {
        let mut index = 0;
        let length = code_units.len();

        let mut is_negative = false;
        if index < length && code_units[index] == b'-' {
            is_negative = true;
            index += 1;
        } else if index < length && code_units[index] == b'+' {
            return Err(DateDurationParseError::PlusNotAllowed);
        }

        if index >= length || code_units[index] != b'P' {
            return Err(DateDurationParseError::InvalidStructure);
        }
        index += 1;

        if index >= length {
            return Err(DateDurationParseError::InvalidStructure);
        }

        let mut years: u32 = 0;
        let mut months: u32 = 0;
        let mut weeks: u32 = 0;
        let mut days: u64 = 0;

        let mut seen_years = false;
        let mut seen_months = false;
        let mut seen_weeks = false;
        let mut seen_days = false;

        while index < length {
            if code_units[index] == b'T' {
                return Err(DateDurationParseError::TimeNotSupported);
            }
            let mut value: u64 = 0;
            let mut has_digits = false;

            while index < length && code_units[index].is_ascii_digit() {
                has_digits = true;
                value = value
                    .checked_mul(10)
                    .and_then(|v| v.checked_add((code_units[index] - b'0') as u64))
                    .ok_or(DateDurationParseError::NumberOverflow)?;
                index += 1;
            }

            if !has_digits {
                return Err(DateDurationParseError::MissingValue);
            }

            if index >= length {
                return Err(DateDurationParseError::InvalidStructure);
            }

            match code_units[index] {
                b'Y' => {
                    if seen_years {
                        return Err(DateDurationParseError::DuplicateUnit);
                    }
                    years =
                        u32::try_from(value).map_err(|_| DateDurationParseError::NumberOverflow)?;
                    seen_years = true;
                }
                b'M' => {
                    if seen_months {
                        return Err(DateDurationParseError::DuplicateUnit);
                    }
                    months =
                        u32::try_from(value).map_err(|_| DateDurationParseError::NumberOverflow)?;
                    seen_months = true;
                }
                b'W' => {
                    if seen_weeks {
                        return Err(DateDurationParseError::DuplicateUnit);
                    }
                    weeks =
                        u32::try_from(value).map_err(|_| DateDurationParseError::NumberOverflow)?;
                    seen_weeks = true;
                }
                b'D' => {
                    if seen_days {
                        return Err(DateDurationParseError::DuplicateUnit);
                    }
                    days = value;
                    seen_days = true;
                }
                _ => return Err(DateDurationParseError::InvalidStructure),
            }

            index += 1;
        }

        Ok(Self {
            is_negative,
            years,
            months,
            weeks,
            days,
        })
    }

    /// Returns a new [`DateDuration`] representing a number of months.
    pub fn for_months(months: i32) -> Self {
        Self {
            is_negative: months.is_negative(),
            months: months.unsigned_abs(),
            ..Default::default()
        }
    }

    /// Returns a new [`DateDuration`] representing a number of weeks.
    pub fn for_weeks(weeks: i32) -> Self {
        Self {
            is_negative: weeks.is_negative(),
            weeks: weeks.unsigned_abs(),
            ..Default::default()
        }
    }

    /// Returns a new [`DateDuration`] representing a number of days.
    pub fn for_days(days: i64) -> Self {
        Self {
            is_negative: days.is_negative(),
            days: days.unsigned_abs(),
            ..Default::default()
        }
    }

    /// Returns a new [`DateDuration`] representing a number of days
    /// represented as weeks and days
    pub(crate) fn for_weeks_and_days(days: i64) -> Self {
        let is_negative = days.is_negative();
        let days = days.unsigned_abs();
        let weeks = (days / 7) as u32;
        let days = days % 7;
        Self {
            is_negative,
            weeks,
            days,
            ..Default::default()
        }
    }

    /// Do NOT pass this function values of mixed signs!
    pub(crate) fn from_signed_ymwd(years: i64, months: i64, weeks: i64, days: i64) -> Self {
        let is_negative = years.is_negative()
            || months.is_negative()
            || weeks.is_negative()
            || days.is_negative();
        if is_negative
            && (years.is_positive()
                || months.is_positive()
                || weeks.is_positive()
                || days.is_positive())
        {
            debug_assert!(false, "mixed signs in from_signed_ymd");
        }
        Self {
            is_negative,
            years: match u32::try_from(years.unsigned_abs()) {
                Ok(x) => x,
                Err(_) => {
                    debug_assert!(false, "years out of range");
                    u32::MAX
                }
            },
            months: match u32::try_from(months.unsigned_abs()) {
                Ok(x) => x,
                Err(_) => {
                    debug_assert!(false, "months out of range");
                    u32::MAX
                }
            },
            weeks: match u32::try_from(weeks.unsigned_abs()) {
                Ok(x) => x,
                Err(_) => {
                    debug_assert!(false, "weeks out of range");
                    u32::MAX
                }
            },
            days: days.unsigned_abs(),
        }
    }

    #[inline]
    pub(crate) fn add_years_to(&self, year: i32) -> i32 {
        if !self.is_negative {
            match year.checked_add_unsigned(self.years) {
                Some(x) => x,
                None => {
                    debug_assert!(false, "{year} + {self:?} out of year range");
                    i32::MAX
                }
            }
        } else {
            match year.checked_sub_unsigned(self.years) {
                Some(x) => x,
                None => {
                    debug_assert!(false, "{year} - {self:?} out of year range");
                    i32::MIN
                }
            }
        }
    }

    #[inline]
    pub(crate) fn add_months_to(&self, month: u8) -> i64 {
        if !self.is_negative {
            i64::from(month) + i64::from(self.months)
        } else {
            i64::from(month) - i64::from(self.months)
        }
    }

    #[inline]
    pub(crate) fn add_weeks_and_days_to(&self, day: u8) -> i64 {
        if !self.is_negative {
            let day = i64::from(day) + i64::from(self.weeks) * 7;
            match day.checked_add_unsigned(self.days) {
                Some(x) => x,
                None => {
                    debug_assert!(false, "{day} + {self:?} out of day range");
                    i64::MAX
                }
            }
        } else {
            let day = i64::from(day) - i64::from(self.weeks) * 7;
            match day.checked_sub_unsigned(self.days) {
                Some(x) => x,
                None => {
                    debug_assert!(false, "{day} - {self:?} out of day range");
                    i64::MIN
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_unit_durations() {
        let d = DateDuration::try_from_str("P1D").unwrap();
        assert_eq!(
            d,
            DateDuration {
                days: 1,
                ..Default::default()
            }
        );

        let d = DateDuration::try_from_str("P3W").unwrap();
        assert_eq!(
            d,
            DateDuration {
                weeks: 3,
                ..Default::default()
            }
        );

        let d = DateDuration::try_from_str("P5M").unwrap();
        assert_eq!(
            d,
            DateDuration {
                months: 5,
                ..Default::default()
            }
        );

        let d = DateDuration::try_from_str("P7Y").unwrap();
        assert_eq!(
            d,
            DateDuration {
                years: 7,
                ..Default::default()
            }
        );
    }

    #[test]
    fn parse_multi_unit_durations() {
        let d = DateDuration::try_from_str("P1Y3M5W7D").unwrap();
        assert_eq!(
            d,
            DateDuration {
                years: 1,
                months: 3,
                weeks: 5,
                days: 7,
                ..Default::default()
            }
        );
    }

    #[test]
    fn parse_negative_durations() {
        let d = DateDuration::try_from_str("-P9W").unwrap();
        assert_eq!(
            d,
            DateDuration {
                is_negative: true,
                weeks: 9,
                ..Default::default()
            }
        );
    }
}
