// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for configuring `DurationFormatter`.

/// A bag of options for defining how to format duration using [`DurationFormatter`](crate::duration::DurationFormatter).
#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DurationFormatterOptions {
    /// The style that will be applied to units
    /// unless overridden by a specific style.
    pub base: BaseStyle,

    /// Style for year
    pub year: YearStyle,
    /// Visibility control for year
    pub year_visibility: UnitVisibility,
    /// Style for month
    pub month: MonthStyle,
    /// Visibility control for month
    pub month_visibility: UnitVisibility,
    /// Style for week
    pub week: WeekStyle,
    /// Visibility control for week
    pub week_visibility: UnitVisibility,
    /// Style for day
    pub day: DayStyle,
    /// Visibility control for day
    pub day_visibility: UnitVisibility,
    /// Style for hour
    pub hour: HourStyle,
    /// Visibility control for hour
    pub hour_visibility: UnitVisibility,
    /// Style for minute
    pub minute: MinuteStyle,
    /// Visibility control for minute
    pub minute_visibility: UnitVisibility,
    /// Style for second
    pub second: SecondStyle,
    /// Visibility control for second
    pub second_visibility: UnitVisibility,
    /// Style for millisecond
    pub millisecond: MilliSecondStyle,
    /// Visibility control for millisecond
    pub millisecond_visibility: UnitVisibility,
    /// Style for microsecond
    pub microsecond: MicroSecondStyle,
    /// Visibility control for microsecond
    pub microsecond_visibility: UnitVisibility,
    /// Style for nanosecond
    pub nanosecond: NanoSecondStyle,
    /// Visibility control for nanosecond
    pub nanosecond_visibility: UnitVisibility,

    /// Number of fractional digits to use when formatting sub-second units (milliseconds, microseconds, nanoseconds).
    /// Only takes effect when the subsecond units are styled as `Numeric`.
    /// Zero means no fractional digits.
    pub fractional_digits: FractionalDigits,
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
#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum UnitVisibility {
    #[default]
    /// Only display this field if it is non-zero.
    Auto,
    /// Always display this field.
    Always,
}

/// Configures the style of the duration output.
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
