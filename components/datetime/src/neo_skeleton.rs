// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo datetime skeletons (Semantic Skeleta)

#[cfg(feature = "serde")]
use crate::neo_serde::*;
#[cfg(feature = "datagen")]
use crate::options::{self, length};
use icu_provider::DataMarkerAttributes;
use icu_timezone::scaffold::IntoOption;

/// A specification for the length of a date or component of a date.
///
/// Contrary to [`crate::options::length::Time`] and
/// [`crate::options::length::Date`], this has only three levels, with no
/// `Full`; this is because `Full` corresponds to additional components,
/// rather than to making the components wider than in `Long`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[repr(u8)] // discriminants come from symbol count in UTS 35
#[non_exhaustive]
pub enum NeoSkeletonLength {
    /// A long date, typically spelled-out, as in “January 1, 2000”.
    Long = 4,
    /// A medium-sized date; typically abbreviated, as in “Jan. 1, 2000”.
    Medium = 3,
    /// A short date; typically numeric, as in “1/1/2000”.
    Short = 1,
}

impl IntoOption<NeoSkeletonLength> for NeoSkeletonLength {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl NeoSkeletonLength {
    /// All values of this enum.
    pub const VALUES: &'static [Self] = &[Self::Long, Self::Medium, Self::Short];

    /// Returns the date style corresponding to this length.
    #[cfg(feature = "datagen")]
    pub fn to_date_style(self) -> options::length::Date {
        match self {
            Self::Long => options::length::Date::Long,
            Self::Medium => options::length::Date::Medium,
            Self::Short => options::length::Date::Short,
        }
    }

    /// Returns the time style corresponding to this length.
    #[cfg(feature = "datagen")]
    pub fn to_time_style(self) -> options::length::Time {
        // Note: For now, make "long" and "medium" both map to "medium".
        // This could be improved in light of additional data.
        match self {
            Self::Long => options::length::Time::Medium,
            Self::Medium => options::length::Time::Medium,
            Self::Short => options::length::Time::Short,
        }
    }
}

/// The alignment context of the formatted string.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
pub enum Alignment {
    /// Align fields as the locale specifies them to be aligned.
    ///
    /// This is the default option.
    Auto,
    /// Align fields as appropriate for a column layout. For example:
    ///
    /// | US Holiday   | Date       |
    /// |--------------|------------|
    /// | Memorial Day | 05/26/2025 |
    /// | Labor Day    | 09/01/2025 |
    /// | Veterans Day | 11/11/2025 |
    ///
    /// This option causes numeric fields to be padded when necessary. It does
    /// not impact whether a numeric or spelled-out field is chosen.
    Column,
}

impl IntoOption<Alignment> for Alignment {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

/// A specification of how to render the year and the era.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
pub enum YearStyle {
    /// Display the century and/or era when needed to disambiguate the year,
    /// based on locale preferences.
    ///
    /// This is the default option.
    ///
    /// Examples:
    ///
    /// - `1000 BC`
    /// - `77 AD`
    /// - `1900`
    /// - `'24`
    Auto,
    /// Always display the century, and display the era when needed to
    /// disambiguate the year, based on locale preferences.
    ///
    /// Examples:
    ///
    /// - `1000 BC`
    /// - `77 AD`
    /// - `1900`
    /// - `2024`
    Full,
    /// Always display the century and era.
    ///
    /// Examples:
    ///
    /// - `1000 BC`
    /// - `77 AD`
    /// - `1900 AD`
    /// - `2024 AD`
    Always,
    // TODO(#4478): add Hide and Never options once there is data to back them
}

impl IntoOption<YearStyle> for YearStyle {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

/// A specification for how precisely to display the time of day.
///
/// The examples below are based on the following inputs and hour cycles:
///
/// 1. 11 o'clock with 12-hour time
/// 2. 16:20 (4:20 pm) with 24-hour time
/// 3. 7:15:01.85 with 24-hour time
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "TimePrecisionSerde", into = "TimePrecisionSerde")
)]
#[non_exhaustive]
pub enum TimePrecision {
    /// Always display the hour. Display smaller fields if they are nonzero.
    ///
    /// Examples:
    ///
    /// 1. `11 am`
    /// 2. `16:20`
    /// 3. `07:15:01.85`
    HourPlus,
    /// Always display the hour. Hide all other time fields.
    ///
    /// Examples:
    ///
    /// 1. `11 am`
    /// 2. `16h`
    /// 3. `07h`
    HourExact,
    /// Always display the hour and minute. Display the second if nonzero.
    ///
    /// Examples:
    ///
    /// 1. `11:00 am`
    /// 2. `16:20`
    /// 3. `07:15:01.85`
    MinutePlus,
    /// Always display the hour and minute. Hide the second.
    ///
    /// Examples:
    ///
    /// 1. `11:00 am`
    /// 2. `16:20`
    /// 3. `07:15`
    MinuteExact,
    /// Display the hour, minute, and second. Display fractional seconds if nonzero.
    ///
    /// This is the default.
    ///
    /// Examples:
    ///
    /// 1. `11:00:00 am`
    /// 2. `16:20:00`
    /// 3. `07:15:01.85`
    SecondPlus,
    /// Display the hour, minute, and second with the given number of
    /// fractional second digits.
    ///
    /// Examples with [`FractionalSecondDigits::F1`]:
    ///
    /// 1. `11:00:00.0 am`
    /// 2. `16:20:00.0`
    /// 3. `07:15:01.8`
    SecondExact(FractionalSecondDigits),
}

impl IntoOption<TimePrecision> for TimePrecision {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl TimePrecision {
    /// Converts a [`length::Time`] to its nearest [`TimePrecision`].
    #[doc(hidden)] // the types involved in this mapping may change
    #[cfg(feature = "datagen")]
    pub fn from_time_length(time_length: length::Time) -> Self {
        match time_length {
            length::Time::Full => todo!(),
            length::Time::Long => todo!(),
            length::Time::Medium => Self::SecondPlus,
            length::Time::Short => Self::MinuteExact,
        }
    }
}

/// A specification for how many fractional second digits to display.
///
/// For example, to display the time with millisecond precision, use
/// [`FractionalSecondDigits::F3`].
///
/// Lower-precision digits will be truncated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
#[non_exhaustive]
pub enum FractionalSecondDigits {
    /// Zero fractional digits. This is the default.
    F0,
    /// One fractional digit (tenths of a second).
    F1,
    /// Two fractional digits (hundredths of a second).
    F2,
    /// Three fractional digits (thousandths of a second).
    F3,
    /// Four fractional digits.
    F4,
    /// Five fractional digits.
    F5,
    /// Six fractional digits.
    F6,
    /// Seven fractional digits.
    F7,
    /// Eight fractional digits.
    F8,
    /// Nine fractional digits.
    F9,
}

/// An error from constructing [`FractionalSecondDigits`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, displaydoc::Display)]
#[non_exhaustive]
pub enum FractionalSecondError {
    /// The provided value is out of range (0-9).
    OutOfRange,
}

impl From<FractionalSecondDigits> for u8 {
    fn from(value: FractionalSecondDigits) -> u8 {
        use FractionalSecondDigits::*;
        match value {
            F0 => 0,
            F1 => 1,
            F2 => 2,
            F3 => 3,
            F4 => 4,
            F5 => 5,
            F6 => 6,
            F7 => 7,
            F8 => 8,
            F9 => 9,
        }
    }
}

impl TryFrom<u8> for FractionalSecondDigits {
    type Error = FractionalSecondError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use FractionalSecondDigits::*;
        match value {
            0 => Ok(F0),
            1 => Ok(F1),
            2 => Ok(F2),
            3 => Ok(F3),
            4 => Ok(F4),
            5 => Ok(F5),
            6 => Ok(F6),
            7 => Ok(F7),
            8 => Ok(F8),
            9 => Ok(F9),
            _ => Err(FractionalSecondError::OutOfRange),
        }
    }
}

/// A specification for a set of parts of a date that specifies a single day (as
/// opposed to a whole month or a week).
///
/// Only sets that yield “sensible” dates are allowed: this type cannot
/// describe a date such as “some Tuesday in 2023”.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) enum NeoDateComponents {
    /// The day of the month, as in
    /// “on the 1st”.
    Day,
    /// The month and day of the month, as in
    /// “January 1st”.
    MonthDay,
    /// The year, month, and day of the month, as in
    /// “January 1st, 2000”.
    YearMonthDay,
    /// The day of the month and day of the week, as in
    /// “Saturday 1st”.
    DayWeekday,
    /// The month, day of the month, and day of the week, as in
    /// “Saturday, January 1st”.
    MonthDayWeekday,
    /// The year, month, day of the month, and day of the week, as in
    /// “Saturday, January 1st, 2000”.
    YearMonthDayWeekday,
    /// The day of the week alone, as in
    /// “Saturday”.
    Weekday,
}

impl NeoDateComponents {
    /// All values of this enum.
    pub const VALUES: &'static [Self] = &[
        Self::Day,
        Self::MonthDay,
        Self::YearMonthDay,
        Self::DayWeekday,
        Self::MonthDayWeekday,
        Self::YearMonthDayWeekday,
        Self::Weekday,
    ];

    const DAY: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("d");
    const MONTH_DAY: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("m0d");
    const YEAR_MONTH_DAY: &'static DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ym0d");
    const DAY_WEEKDAY: &'static DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("de");
    const MONTH_DAY_WEEKDAY: &'static DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("m0de");
    const YEAR_MONTH_DAY_WEEKDAY: &'static DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ym0de");
    const WEEKDAY: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("e");

    // For matching
    const DAY_STR: &'static str = Self::DAY.as_str();
    const MONTH_DAY_STR: &'static str = Self::MONTH_DAY.as_str();
    const YEAR_MONTH_DAY_STR: &'static str = Self::YEAR_MONTH_DAY.as_str();
    const DAY_WEEKDAY_STR: &'static str = Self::DAY_WEEKDAY.as_str();
    const MONTH_DAY_WEEKDAY_STR: &'static str = Self::MONTH_DAY_WEEKDAY.as_str();
    const YEAR_MONTH_DAY_WEEKDAY_STR: &'static str = Self::YEAR_MONTH_DAY_WEEKDAY.as_str();
    const WEEKDAY_STR: &'static str = Self::WEEKDAY.as_str();

    pub const fn id_str(self) -> &'static DataMarkerAttributes {
        match self {
            Self::Day => Self::DAY,
            Self::MonthDay => Self::MONTH_DAY,
            Self::YearMonthDay => Self::YEAR_MONTH_DAY,
            Self::DayWeekday => Self::DAY_WEEKDAY,
            Self::MonthDayWeekday => Self::MONTH_DAY_WEEKDAY,
            Self::YearMonthDayWeekday => Self::YEAR_MONTH_DAY_WEEKDAY,
            Self::Weekday => Self::WEEKDAY,
        }
    }

    pub fn from_id_str(id_str: &DataMarkerAttributes) -> Option<Self> {
        match &**id_str {
            Self::DAY_STR => Some(Self::Day),
            Self::MONTH_DAY_STR => Some(Self::MonthDay),
            Self::YEAR_MONTH_DAY_STR => Some(Self::YearMonthDay),
            Self::DAY_WEEKDAY_STR => Some(Self::DayWeekday),
            Self::MONTH_DAY_WEEKDAY_STR => Some(Self::MonthDayWeekday),
            Self::YEAR_MONTH_DAY_WEEKDAY_STR => Some(Self::YearMonthDayWeekday),
            Self::WEEKDAY_STR => Some(Self::Weekday),
            _ => None,
        }
    }

    /// Whether this field set contains the year.
    pub fn has_year(self) -> bool {
        match self {
            Self::Day => false,
            Self::MonthDay => false,
            Self::YearMonthDay => true,
            Self::DayWeekday => false,
            Self::MonthDayWeekday => false,
            Self::YearMonthDayWeekday => true,
            Self::Weekday => false,
        }
    }

    /// Whether this field set contains the month.
    pub fn has_month(self) -> bool {
        match self {
            Self::Day => false,
            Self::MonthDay => true,
            Self::YearMonthDay => true,
            Self::DayWeekday => false,
            Self::MonthDayWeekday => true,
            Self::YearMonthDayWeekday => true,
            Self::Weekday => false,
        }
    }

    /// Whether this field set contains the day of the month.
    pub fn has_day(self) -> bool {
        match self {
            Self::Day => true,
            Self::MonthDay => true,
            Self::YearMonthDay => true,
            Self::DayWeekday => true,
            Self::MonthDayWeekday => true,
            Self::YearMonthDayWeekday => true,
            Self::Weekday => false,
        }
    }

    /// Whether this field set contains the weekday.
    pub fn has_weekday(self) -> bool {
        match self {
            Self::Day => false,
            Self::MonthDay => false,
            Self::YearMonthDay => false,
            Self::DayWeekday => true,
            Self::MonthDayWeekday => true,
            Self::YearMonthDayWeekday => true,
            Self::Weekday => true,
        }
    }

    /// Creates a skeleton for this field set with a long length.
    pub fn long(self) -> NeoDateSkeleton {
        NeoDateSkeleton::for_length_and_components(NeoSkeletonLength::Long, self)
    }

    /// Creates a skeleton for this field set with a medium length.
    pub fn medium(self) -> NeoDateSkeleton {
        NeoDateSkeleton::for_length_and_components(NeoSkeletonLength::Medium, self)
    }

    /// Creates a skeleton for this field set with a short length.
    pub fn short(self) -> NeoDateSkeleton {
        NeoDateSkeleton::for_length_and_components(NeoSkeletonLength::Short, self)
    }
}

/// A specification for a set of parts of a date.
/// Only sets that yield “sensible” dates are allowed: this type cannot describe
/// a date such as “August, Anno Domini”.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) enum NeoCalendarPeriodComponents {
    /// A standalone month, as in
    /// “January”.
    Month,
    /// A month and year, as in
    /// “January 2000”.
    YearMonth,
    /// A year, as in
    /// “2000”.
    Year,
    // TODO(#501): Consider adding support for Quarter and YearQuarter.
}

impl NeoCalendarPeriodComponents {
    /// All values of this enum.
    pub const VALUES: &'static [Self] = &[Self::Month, Self::YearMonth, Self::Year];

    const MONTH: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("m0");
    const YEAR_MONTH: &'static DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ym0");
    const YEAR: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("y");

    // For matching
    const MONTH_STR: &'static str = Self::MONTH.as_str();
    const YEAR_MONTH_STR: &'static str = Self::YEAR_MONTH.as_str();
    const YEAR_STR: &'static str = Self::YEAR.as_str();

    /// Returns a stable string identifying this set of components.
    ///
    /// For details, see [`NeoDateComponents::id_str()`].
    pub const fn id_str(self) -> &'static DataMarkerAttributes {
        match self {
            Self::Month => Self::MONTH,
            Self::YearMonth => Self::YEAR_MONTH,
            Self::Year => Self::YEAR,
        }
    }

    /// Returns the set of components for the given stable string.
    ///
    /// For details, see [`NeoDateComponents::from_id_str()`].
    pub fn from_id_str(id_str: &DataMarkerAttributes) -> Option<Self> {
        match &**id_str {
            Self::MONTH_STR => Some(Self::Month),
            Self::YEAR_MONTH_STR => Some(Self::YearMonth),
            Self::YEAR_STR => Some(Self::Year),
            _ => None,
        }
    }

    /// Whether this field set contains the year.
    pub fn has_year(self) -> bool {
        match self {
            Self::Month => false,
            Self::YearMonth => true,
            Self::Year => true,
        }
    }

    /// Whether this field set contains the month.
    pub fn has_month(self) -> bool {
        match self {
            Self::Month => true,
            Self::YearMonth => true,
            Self::Year => false,
        }
    }

    /// Creates a skeleton for this field set with a long length.
    pub fn long(self) -> NeoCalendarPeriodSkeleton {
        NeoCalendarPeriodSkeleton::for_length_and_components(NeoSkeletonLength::Long, self)
    }

    /// Creates a skeleton for this field set with a medium length.
    pub fn medium(self) -> NeoCalendarPeriodSkeleton {
        NeoCalendarPeriodSkeleton::for_length_and_components(NeoSkeletonLength::Medium, self)
    }

    /// Creates a skeleton for this field set with a short length.
    pub fn short(self) -> NeoCalendarPeriodSkeleton {
        NeoCalendarPeriodSkeleton::for_length_and_components(NeoSkeletonLength::Short, self)
    }
}

/// A specification for a set of parts of a time.
/// Only sets that yield “sensible” time are allowed: this type cannot describe
/// a time such as “am, 5 minutes, 25 milliseconds”.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) enum NeoTimeComponents {
    /// A time of day (12-hour or 24-hour chosen by locale),
    /// with the precision chosen by [`TimePrecision`]
    Time,
    // TODO: Remove these other variants. Collapse this enum to TimePrecision
    /// A time of day with a 12-hour clock,
    /// with the precision chosen by [`TimePrecision`]
    Time12,
    /// A time of day with a 24-hour clock,
    /// with the precision chosen by [`TimePrecision`]
    Time24,
}

impl NeoTimeComponents {
    /// All values of this enum.
    pub const VALUES: &'static [Self] = &[Self::Time, Self::Time12, Self::Time24];

    const HOUR: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("j");
    const HOUR12: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("h");
    const HOUR24: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("h0");

    // For matching
    const HOUR_STR: &'static str = Self::HOUR.as_str();
    const HOUR12_STR: &'static str = Self::HOUR12.as_str();
    const HOUR24_STR: &'static str = Self::HOUR24.as_str();

    /// Returns a stable string identifying this set of components.
    ///
    /// For details, see [`NeoDateComponents::id_str()`].
    pub const fn id_str(self) -> &'static DataMarkerAttributes {
        match self {
            Self::Time => Self::HOUR,
            Self::Time12 => Self::HOUR12,
            Self::Time24 => Self::HOUR24,
        }
    }

    /// Returns the set of components for the given stable string.
    ///
    /// For details, see [`NeoDateComponents::from_id_str()`].
    pub fn from_id_str(id_str: &DataMarkerAttributes) -> Option<Self> {
        match &**id_str {
            Self::HOUR_STR => Some(Self::Time),
            Self::HOUR12_STR => Some(Self::Time12),
            Self::HOUR24_STR => Some(Self::Time24),
            _ => None,
        }
    }

    /// Whether this field set contains the time of day.
    pub fn has_time(self) -> bool {
        true
    }

    /// Creates a skeleton for this field set with a long length.
    pub fn long(self) -> NeoTimeSkeleton {
        NeoTimeSkeleton::for_length_and_components(NeoSkeletonLength::Long, self)
    }

    /// Creates a skeleton for this field set with a medium length.
    pub fn medium(self) -> NeoTimeSkeleton {
        NeoTimeSkeleton::for_length_and_components(NeoSkeletonLength::Medium, self)
    }

    /// Creates a skeleton for this field set with a short length.
    pub fn short(self) -> NeoTimeSkeleton {
        NeoTimeSkeleton::for_length_and_components(NeoSkeletonLength::Short, self)
    }
}

/// A specification of components for parts of a datetime.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) enum NeoDateTimeComponents {
    /// Components for parts of a date.
    Date(NeoDateComponents),
    /// Components for parts of a date with fields larger than a date.
    CalendarPeriod(NeoCalendarPeriodComponents),
    /// Components for parts of a time.
    Time(NeoTimeComponents),
    /// Components for parts of a date and time together.
    DateTime(NeoDateComponents, NeoTimeComponents),
}

impl From<NeoDateComponents> for NeoDateTimeComponents {
    fn from(value: NeoDateComponents) -> Self {
        Self::Date(value)
    }
}

impl From<NeoCalendarPeriodComponents> for NeoDateTimeComponents {
    fn from(value: NeoCalendarPeriodComponents) -> Self {
        Self::CalendarPeriod(value)
    }
}

impl From<NeoTimeComponents> for NeoDateTimeComponents {
    fn from(value: NeoTimeComponents) -> Self {
        Self::Time(value)
    }
}

impl NeoDateTimeComponents {
    /// Returns a [`NeoDateTimeComponents`] if it is a subset of the [`NeoComponents`] argument.
    ///
    /// If the [`NeoComponents`] contains a time zone, this function returns `None`.
    pub fn try_from_components(components: NeoComponents) -> Option<Self> {
        match components {
            NeoComponents::Date(d) => Some(Self::Date(d)),
            NeoComponents::CalendarPeriod(cp) => Some(Self::CalendarPeriod(cp)),
            NeoComponents::Time(t) => Some(Self::Time(t)),
            NeoComponents::Zone(_) => None,
            NeoComponents::DateTime(d, t) => Some(Self::DateTime(d, t)),
            NeoComponents::DateZone(_, _) => None,
            NeoComponents::TimeZone(_, _) => None,
            NeoComponents::DateTimeZone(_, _, _) => None,
        }
    }

    /// Creates a skeleton for this field set with a long length.
    pub fn long(self) -> NeoDateTimeSkeleton {
        NeoDateTimeSkeleton::for_length_and_components(NeoSkeletonLength::Long, self)
    }

    /// Creates a skeleton for this field set with a medium length.
    pub fn medium(self) -> NeoDateTimeSkeleton {
        NeoDateTimeSkeleton::for_length_and_components(NeoSkeletonLength::Medium, self)
    }

    /// Creates a skeleton for this field set with a short length.
    pub fn short(self) -> NeoDateTimeSkeleton {
        NeoDateTimeSkeleton::for_length_and_components(NeoSkeletonLength::Short, self)
    }
}

/// A specification of components for parts of a datetime and/or time zone.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) enum NeoComponents {
    /// Components for a date.
    Date(NeoDateComponents),
    /// Components for a calendar period.
    CalendarPeriod(NeoCalendarPeriodComponents),
    /// Components for a time.
    Time(NeoTimeComponents),
    /// Components for a time zone.
    Zone(NeoTimeZoneStyle),
    /// Components for a date and a time together.
    DateTime(NeoDateComponents, NeoTimeComponents),
    /// Components for a date and a time zone together.
    DateZone(NeoDateComponents, NeoTimeZoneStyle),
    /// Components for a time and a time zone together.
    TimeZone(NeoTimeComponents, NeoTimeZoneStyle),
    /// Components for a date, a time, and a time zone together.
    DateTimeZone(NeoDateComponents, NeoTimeComponents, NeoTimeZoneStyle),
}

impl From<NeoDateComponents> for NeoComponents {
    fn from(value: NeoDateComponents) -> Self {
        Self::Date(value)
    }
}

impl From<NeoCalendarPeriodComponents> for NeoComponents {
    fn from(value: NeoCalendarPeriodComponents) -> Self {
        Self::CalendarPeriod(value)
    }
}

impl From<NeoTimeComponents> for NeoComponents {
    fn from(value: NeoTimeComponents) -> Self {
        Self::Time(value)
    }
}

impl From<NeoDateTimeComponents> for NeoComponents {
    fn from(value: NeoDateTimeComponents) -> Self {
        match value {
            NeoDateTimeComponents::Date(components) => components.into(),
            NeoDateTimeComponents::CalendarPeriod(components) => components.into(),
            NeoDateTimeComponents::Time(components) => components.into(),
            NeoDateTimeComponents::DateTime(day, time) => NeoComponents::DateTime(day, time),
        }
    }
}

impl From<NeoTimeZoneStyle> for NeoComponents {
    fn from(value: NeoTimeZoneStyle) -> Self {
        Self::Zone(value)
    }
}

impl NeoComponents {
    // Attributes for skeleta that span date/time/zone
    // TODO: Add variants for H, h, and B hours
    const WEEKDAY_TIME: &'static DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ej");

    // For matching
    const WEEKDAY_TIME_STR: &'static str = Self::WEEKDAY_TIME.as_str();

    #[doc(hidden)] // for datagen
    pub fn attributes_with_overrides() -> &'static [&'static DataMarkerAttributes] {
        &[Self::WEEKDAY_TIME]
    }

    /// Returns a stable string identifying this field set,
    /// but only if this set has its own pattern override.
    ///
    /// For details, see [`NeoDateComponents::id_str()`].
    pub const fn id_str(self) -> Option<&'static DataMarkerAttributes> {
        match self {
            Self::DateTime(NeoDateComponents::Weekday, NeoTimeComponents::Time) => {
                Some(Self::WEEKDAY_TIME)
            }
            _ => None,
        }
    }

    /// Returns the field set for the given stable string,
    /// but only if this set has its own pattern override.
    ///
    /// For details, see [`NeoDateComponents::from_id_str()`].
    pub fn from_id_str(id_str: &DataMarkerAttributes) -> Option<Self> {
        match &**id_str {
            Self::WEEKDAY_TIME_STR => Some(Self::DateTime(
                NeoDateComponents::Weekday,
                NeoTimeComponents::Time,
            )),
            _ => None,
        }
    }

    /// Whether this field set contains the year.
    pub fn has_year(self) -> bool {
        match self {
            NeoComponents::Date(date_components) => date_components.has_year(),
            NeoComponents::CalendarPeriod(calendar_period_components) => {
                calendar_period_components.has_year()
            }
            NeoComponents::Time(_) => todo!(),
            NeoComponents::Zone(_) => todo!(),
            NeoComponents::DateTime(date_components, _) => date_components.has_year(),
            NeoComponents::DateZone(date_components, _) => date_components.has_year(),
            NeoComponents::TimeZone(_, _) => todo!(),
            NeoComponents::DateTimeZone(date_components, _, _) => date_components.has_year(),
        }
    }

    /// Whether this field set contains the month.
    pub fn has_month(self) -> bool {
        match self {
            NeoComponents::Date(date_components) => date_components.has_month(),
            NeoComponents::CalendarPeriod(calendar_period_components) => {
                calendar_period_components.has_month()
            }
            NeoComponents::Time(_) => todo!(),
            NeoComponents::Zone(_) => todo!(),
            NeoComponents::DateTime(date_components, _) => date_components.has_month(),
            NeoComponents::DateZone(date_components, _) => date_components.has_month(),
            NeoComponents::TimeZone(_, _) => todo!(),
            NeoComponents::DateTimeZone(date_components, _, _) => date_components.has_month(),
        }
    }

    /// Whether this field set contains the day of the month.
    pub fn has_day(self) -> bool {
        match self {
            NeoComponents::Date(date_components) => date_components.has_day(),
            NeoComponents::CalendarPeriod(_) => false,
            NeoComponents::Time(_) => todo!(),
            NeoComponents::Zone(_) => todo!(),
            NeoComponents::DateTime(date_components, _) => date_components.has_day(),
            NeoComponents::DateZone(date_components, _) => date_components.has_day(),
            NeoComponents::TimeZone(_, _) => todo!(),
            NeoComponents::DateTimeZone(date_components, _, _) => date_components.has_day(),
        }
    }

    /// Whether this field set contains the weekday.
    pub fn has_weekday(self) -> bool {
        match self {
            NeoComponents::Date(date_components) => date_components.has_weekday(),
            NeoComponents::CalendarPeriod(_) => false,
            NeoComponents::Time(_) => todo!(),
            NeoComponents::Zone(_) => todo!(),
            NeoComponents::DateTime(date_components, _) => date_components.has_weekday(),
            NeoComponents::DateZone(date_components, _) => date_components.has_weekday(),
            NeoComponents::TimeZone(_, _) => todo!(),
            NeoComponents::DateTimeZone(date_components, _, _) => date_components.has_weekday(),
        }
    }

    /// Whether this field set contains the time of day.
    pub fn has_time(self) -> bool {
        match self {
            NeoComponents::Date(_) => false,
            NeoComponents::CalendarPeriod(_) => false,
            NeoComponents::Time(_) => true,
            NeoComponents::Zone(_) => false,
            NeoComponents::DateTime(_, _) => true,
            NeoComponents::DateZone(_, _) => false,
            NeoComponents::TimeZone(_, _) => true,
            NeoComponents::DateTimeZone(_, _, _) => true,
        }
    }

    /// Creates a skeleton for this field set with a long length.
    pub fn long(self) -> NeoSkeleton {
        NeoSkeleton::for_length_and_components(NeoSkeletonLength::Long, self)
    }

    /// Creates a skeleton for this field set with a medium length.
    pub fn medium(self) -> NeoSkeleton {
        NeoSkeleton::for_length_and_components(NeoSkeletonLength::Medium, self)
    }

    /// Creates a skeleton for this field set with a short length.
    pub fn short(self) -> NeoSkeleton {
        NeoSkeleton::for_length_and_components(NeoSkeletonLength::Short, self)
    }
}

/// Specification of the time zone display style.
///
/// Time zone names contribute a lot of data size. For resource-constrained
/// environments, the following formats require the least amount of data:
///
/// - [`NeoTimeZoneStyle::Specific`] + [`NeoSkeletonLength::Short`]
/// - [`NeoTimeZoneStyle::Offset`]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum NeoTimeZoneStyle {
    /// The location format, e.g., “Los Angeles time” or specific non-location
    /// format “Pacific Daylight Time”, whichever is idiomatic for the locale.
    /// > Note: for now, this is always identical to
    /// > [`NeoTimeZoneStyle::Specific`] (Pacific Daylight Time), but
    /// > whether it is [`NeoTimeZoneStyle::Generic`] or
    /// > [`NeoTimeZoneStyle::Specific`] will be locale-dependent in the
    /// > future; see
    /// > [CLDR-15566](https://unicode-org.atlassian.net/browse/CLDR-15566).
    #[default]
    Default,
    /// The location format, e.g., “Los Angeles time”.
    ///
    /// When unavailable, falls back to [`NeoTimeZoneStyle::Offset`].
    Location,
    /// The generic non-location format, e.g., “Pacific Time”.
    ///
    /// When unavailable, falls back to [`NeoTimeZoneStyle::Location`].
    Generic,
    /// The specific non-location format, e.g., “Pacific Daylight Time”.
    ///
    /// When unavailable, falls back to [`NeoTimeZoneStyle::Offset`].
    Specific,
    /// The offset format, e.g., “GMT−8”.
    Offset,
}

/// A skeleton for formatting a time zone.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct NeoTimeZoneSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// The style, _i.e._, with `length`=[`NeoSkeletonLength::Short`], whether to format as
    /// “GMT−8” ([`NeoTimeZoneStyle::Offset`]) or “PT”
    /// ([`NeoTimeZoneStyle::Generic`]).
    pub style: NeoTimeZoneStyle,
}

impl NeoTimeZoneStyle {
    /// Creates a skeleton for this time zone style with a long length.
    pub fn long(self) -> NeoTimeZoneSkeleton {
        NeoTimeZoneSkeleton::for_length_and_components(NeoSkeletonLength::Long, self)
    }

    /// Creates a skeleton for this time zone style with a medium length.
    pub fn medium(self) -> NeoTimeZoneSkeleton {
        NeoTimeZoneSkeleton::for_length_and_components(NeoSkeletonLength::Medium, self)
    }

    /// Creates a skeleton for this time zone style with a short length.
    pub fn short(self) -> NeoTimeZoneSkeleton {
        NeoTimeZoneSkeleton::for_length_and_components(NeoSkeletonLength::Short, self)
    }
}

impl NeoTimeZoneSkeleton {
    /// Creates a skeleton from its length and components.
    pub fn for_length_and_components(length: NeoSkeletonLength, style: NeoTimeZoneStyle) -> Self {
        Self { length, style }
    }
}

/// A skeleton for formatting parts of a date (without time or time zone).
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoDateSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Date components of the skeleton.
    pub components: NeoDateComponents,
    /// Alignment option.
    pub alignment: Option<Alignment>,
    /// Era display option.
    pub year_style: Option<YearStyle>,
}

impl NeoDateSkeleton {
    /// Creates a skeleton from its length and components.
    pub fn for_length_and_components(
        length: NeoSkeletonLength,
        components: NeoDateComponents,
    ) -> Self {
        Self {
            length,
            components,
            alignment: None,
            year_style: None,
        }
    }
}

/// A skeleton for formatting a calendar period (i.e. month or year).
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoCalendarPeriodSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Date components of the skeleton.
    pub components: NeoCalendarPeriodComponents,
    /// Alignment option.
    pub alignment: Option<Alignment>,
    /// Era display option.
    pub year_style: Option<YearStyle>,
}

impl NeoCalendarPeriodSkeleton {
    /// Creates a skeleton from its length and components.
    pub fn for_length_and_components(
        length: NeoSkeletonLength,
        components: NeoCalendarPeriodComponents,
    ) -> Self {
        Self {
            length,
            components,
            alignment: None,
            year_style: None,
        }
    }
}

/// A skeleton for formatting parts of a time (without date or time zone).
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoTimeSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Time components of the skeleton.
    pub components: NeoTimeComponents,
    /// Alignment option.
    pub alignment: Option<Alignment>,
    /// Time precision option.
    pub time_precision: Option<TimePrecision>,
}

impl NeoTimeSkeleton {
    /// Creates a skeleton from its length and components.
    pub fn for_length_and_components(
        length: NeoSkeletonLength,
        components: NeoTimeComponents,
    ) -> Self {
        Self {
            length,
            components,
            alignment: None,
            time_precision: None,
        }
    }

    /// Sets the time precision to [`TimePrecision::MinuteExact`]
    pub fn hm(mut self) -> Self {
        self.time_precision = Some(TimePrecision::MinuteExact);
        self
    }

    /// Sets the time precision to [`TimePrecision::SecondPlus`]
    pub fn hms(mut self) -> Self {
        self.time_precision = Some(TimePrecision::SecondPlus);
        self
    }
}

/// A skeleton for formatting parts of a date and time (without time zone).
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoDateTimeSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Date and time components of the skeleton.
    pub components: NeoDateTimeComponents,
    /// Alignment option.
    pub alignment: Option<Alignment>,
    /// Era display option.
    pub year_style: Option<YearStyle>,
    /// Time precision option.
    pub time_precision: Option<TimePrecision>,
}

impl NeoDateTimeSkeleton {
    /// Creates a skeleton from its length and components.
    pub fn for_length_and_components(
        length: NeoSkeletonLength,
        components: NeoDateTimeComponents,
    ) -> Self {
        Self {
            length,
            components,
            alignment: None,
            year_style: None,
            time_precision: None,
        }
    }

    /// Sets the time precision to [`TimePrecision::MinuteExact`]
    pub fn hm(mut self) -> Self {
        self.time_precision = Some(TimePrecision::MinuteExact);
        self
    }

    /// Sets the time precision to [`TimePrecision::SecondPlus`]
    pub fn hms(mut self) -> Self {
        self.time_precision = Some(TimePrecision::SecondPlus);
        self
    }
}

/// A skeleton for formatting parts of a date, time, and optional time zone.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct NeoSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Components of the skeleton.
    pub components: NeoComponents,
    /// Alignment option.
    pub alignment: Option<Alignment>,
    /// Era display option.
    pub year_style: Option<YearStyle>,
    /// Fractional second digits option.
    pub time_precision: Option<TimePrecision>,
}

impl From<NeoDateSkeleton> for NeoSkeleton {
    fn from(value: NeoDateSkeleton) -> Self {
        NeoSkeleton {
            length: value.length,
            components: value.components.into(),
            alignment: value.alignment,
            year_style: value.year_style,
            time_precision: None,
        }
    }
}

impl From<NeoTimeSkeleton> for NeoSkeleton {
    fn from(value: NeoTimeSkeleton) -> Self {
        NeoSkeleton {
            length: value.length,
            components: value.components.into(),
            alignment: value.alignment,
            year_style: None,
            time_precision: value.time_precision,
        }
    }
}

impl From<NeoDateTimeSkeleton> for NeoSkeleton {
    fn from(value: NeoDateTimeSkeleton) -> Self {
        NeoSkeleton {
            length: value.length,
            components: value.components.into(),
            alignment: value.alignment,
            year_style: value.year_style,
            time_precision: value.time_precision,
        }
    }
}

impl NeoSkeleton {
    /// Creates a skeleton from its length and components.
    pub fn for_length_and_components(length: NeoSkeletonLength, components: NeoComponents) -> Self {
        Self {
            length,
            components,
            alignment: None,
            year_style: None,
            time_precision: None,
        }
    }
}
