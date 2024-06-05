// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for configuring [`DurationFormatter`](crate::duration::DurationFormatter).

/// A bag of options for defining how to format duration using [`DurationFormatter`](crate::duration::DurationFormatter).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DurationFormatterOptions {
    base: BaseStyle,

    year: YearStyle,
    year_visibility: UnitVisibility,
    month: MonthStyle,
    month_visibility: UnitVisibility,
    week: WeekStyle,
    week_visibility: UnitVisibility,
    day: DayStyle,
    day_visibility: UnitVisibility,
    hour: HourStyle,
    hour_visibility: UnitVisibility,
    minute: MinuteStyle,
    minute_visibility: UnitVisibility,
    second: SecondStyle,
    second_visibility: UnitVisibility,
    millisecond: MilliSecondStyle,
    millisecond_visibility: UnitVisibility,
    microsecond: MicroSecondStyle,
    microsecond_visibility: UnitVisibility,
    nanosecond: NanoSecondStyle,
    nanosecond_visibility: UnitVisibility,

    /// Number of fractional digits to use when formatting sub-second units (milliseconds, microseconds, nanoseconds).
    /// Only takes effect when the subsecond units are styled as `Numeric`.
    /// Zero means no fractional digits.
    fractional_digits: FractionalDigits,
}

/// Options for configuring the number of fractional digits to display.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FractionalDigits {
    /// Show as many fractional digits as necessary to display the whole duration,
    /// omitting trailing zeroes after the decimal point.
    #[default]
    ShowAll,
    /// Use the given number of fractional digits.
    /// Rounded to zero if necessary.
    Fixed(u8),
}

/// Configures visibility of fields in the formatted string.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum UnitVisibility {
    #[default]
    /// Only display this field if it is non-zero.
    Auto,
    /// Always display this field.
    Always,
}

/// Configures the style of the duration output.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BaseStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
    /// Digital formatting
    Digital,
}

/// Configures the style of the year field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum YearStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
}

/// Configures the style of the month field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MonthStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
}

/// Configures the style of the week field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WeekStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
}

/// Configures the style of the day field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DayStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
}

/// Configures the style of the hour field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HourStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
    /// Ensure formatted value is at least two digits long (by appending leading zeroes, if necessary)
    TwoDigit,
    /// Numeric style
    Numeric,
}

/// Configures the style of the minute field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MinuteStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
    /// Ensure formatted value is at least two digits long (by appending leading zeroes, if necessary)
    TwoDigit,
    /// Numeric style
    Numeric,
}

/// Configures the style of the second field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SecondStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
    /// Ensure formatted value is at least two digits long (by appending leading zeroes, if necessary)
    TwoDigit,
    /// Numeric style
    Numeric,
}

/// Configures the style of the milliseconds field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MilliSecondStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
    /// Numeric style
    Numeric,
}

/// Configures the style of the microsecond field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MicroSecondStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
    /// Numeric style
    Numeric,
}

/// Configures the style of the nanosecond field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NanoSecondStyle {
    /// Narrow style (most compact)
    Narrow,
    #[default]
    /// Short style (default)
    Short,
    /// Long style (most verbose)
    Long,
    /// Numeric style
    Numeric,
}
