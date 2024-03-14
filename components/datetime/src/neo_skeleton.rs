// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo datetime skeletons (Semantic Skeleta)

use core::marker::PhantomData;

use crate::options::components;
use crate::provider::neo::*;
use crate::CldrCalendar;
use icu_provider::prelude::*;
use icu_provider::NeverMarker;

/// Sealed trait implemented by neo skeleton marker types.
pub trait TypedNeoSkeletonData<C: CldrCalendar> {
    /// Components in the neo skeleton.
    const COMPONENTS: NeoComponents;
    /// Marker for loading year names.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type YearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;
    /// Marker for loading month names.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type MonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;
    /// Marker for loading weekday names.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading day period names.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type DayPeriodNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading date skeleton patterns.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type DateSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading time skeleton patterns.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type TimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading the date/time glue pattern.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type DateTimePatternV1Marker: KeyedDataMarker<Yokeable = DateTimePatternV1<'static>>;
    /// Marker for loading date/time combined patterns.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type DateTimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = DateTimeSkeletonsV1<'static>>;
}

/// A specification for the length of a date or component of a date.
///
/// Contrary to [`crate::options::length::Time`] and
/// [`crate::options::length::Date`], this has only three levels, with no
/// `Full`; this is because `Full` corresponds to additional components,
/// rather than to making the components wider than in `Long`.
#[derive(Debug, Copy, Clone)]
pub enum Length {
    /// A long date, typically spelled-out, as in “January 1, 2000”.
    Long,
    /// A medium-sized date; typically abbreviated, as in “Jan. 1, 2000”.
    Medium,
    /// A short date; typically numeric, as in “1/1/2000”.
    Short,
}

impl Length {
    fn to_components_text(self) -> components::Text {
        match self {
            Self::Long => components::Text::Long,
            Self::Medium => components::Text::Short,
            Self::Short => components::Text::Narrow,
        }
    }

    fn to_components_numeric(self) -> components::Numeric {
        components::Numeric::Numeric
    }

    fn to_components_day(self) -> components::Day {
        components::Day::NumericDayOfMonth
    }

    fn to_components_month(self) -> components::Month {
        match self {
            Self::Long => components::Month::Long,
            Self::Medium => components::Month::Short,
            Self::Short => components::Month::Numeric,
        }
    }

    fn to_components_year(self) -> components::Year {
        components::Year::Numeric
    }

    fn to_components_year_of_week(self) -> components::Year {
        components::Year::NumericWeekOf
    }

    fn to_components_week_of_year(self) -> components::Week {
        components::Week::NumericWeekOfYear
    }
}

/// Marker for a skeleton with a year and a month, like "March 2024".
#[derive(Debug)]
pub enum YearMonthMarker {}

impl<C> TypedNeoSkeletonData<C> for YearMonthMarker
where
    C: CldrCalendar,
{
    const COMPONENTS: NeoComponents = NeoComponents::Date(DateComponents::YearMonth);

    // Data to include
    type YearNamesV1Marker = C::YearNamesV1Marker;
    type MonthNamesV1Marker = C::MonthNamesV1Marker;
    type DateSkeletonPatternsV1Marker = C::DateSkeletonPatternsV1Marker;

    // Data to exclude
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
    type DateTimeSkeletonPatternsV1Marker = NeverMarker<DateTimeSkeletonsV1<'static>>;
}

/// Representation of a semantic skeleton with marker types.
#[derive(Debug, Copy, Clone)]
pub struct TypedNeoSkeleton<C: CldrCalendar, T: TypedNeoSkeletonData<C>> {
    /// Desired formatting length.
    pub length: Length,
    _phantom: PhantomData<(C, T)>,
}

impl<C: CldrCalendar, T: TypedNeoSkeletonData<C>> TypedNeoSkeleton<C, T> {
    /// Converts this [`TypedNeoSkeleton`] to a [`NeoSkeleton`].
    pub fn to_neo_skeleton(self) -> NeoSkeleton {
        NeoSkeleton {
            length: self.length,
            components: T::COMPONENTS,
            time_zone: None,
        }
    }
}

/// A specification for a set of parts of a date that specifies a single day (as
/// opposed to a whole month, week, or quarter).
/// Only sets that yield “sensible” dates are allowed: this type cannot
/// describe a date such as “some Tuesday in 2023”.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DayComponents {
    /// The day of the month, as in “on the 1st”.
    Day,
    /// The month and day of the month, as in “January 1st”.
    MonthDay,
    /// The year, month, and day of the month, as in “January 1st, 2000”.
    YearMonthDay,
    /// The era, year, month, and day of the month, as in “January 1st, 2000 A.D.”.
    EraYearMonthDay,
    /// The day of the month and day of the week, as in “Saturday 1st”.
    DayWeekday,
    /// The month, day of the month, and day of the week, as in “Saturday, January 1st”.
    MonthDayWeekday,
    /// The year, month, day of the month, and day of the week, as in
    /// “Saturday, January 1st, 2000”.
    YearMonthDayWeekday,
    /// The era, year, month, day of the month, and day of the week, as in
    /// “Saturday, January 1st, 2000 A.D.”.
    EraYearMonthDayWeekday,
    /// The day of the week alone, as in “Saturday”.
    Weekday,
}

impl DayComponents {
    /// All values of this enum.
    pub const VALUES: &'static [Self] = &[
        Self::Day,
        Self::MonthDay,
        Self::YearMonthDay,
        Self::EraYearMonthDay,
        Self::DayWeekday,
        Self::MonthDayWeekday,
        Self::YearMonthDayWeekday,
        Self::EraYearMonthDayWeekday,
        Self::Weekday,
    ];

    fn to_components_bag_with_length(self, length: Length) -> components::Bag {
        match self {
            Self::Day => components::Bag {
                day: Some(length.to_components_day()),
                ..Default::default()
            },
            Self::MonthDay => components::Bag {
                month: Some(length.to_components_month()),
                day: Some(length.to_components_day()),
                ..Default::default()
            },
            Self::YearMonthDay => components::Bag {
                year: Some(length.to_components_year()),
                month: Some(length.to_components_month()),
                day: Some(length.to_components_day()),
                ..Default::default()
            },
            Self::EraYearMonthDay => components::Bag {
                era: Some(length.to_components_text()),
                year: Some(length.to_components_year()),
                month: Some(length.to_components_month()),
                day: Some(length.to_components_day()),
                ..Default::default()
            },
            Self::DayWeekday => components::Bag {
                day: Some(length.to_components_day()),
                weekday: Some(length.to_components_text()),
                ..Default::default()
            },
            Self::MonthDayWeekday => components::Bag {
                month: Some(length.to_components_month()),
                day: Some(length.to_components_day()),
                weekday: Some(length.to_components_text()),
                ..Default::default()
            },
            Self::YearMonthDayWeekday => components::Bag {
                year: Some(length.to_components_year()),
                month: Some(length.to_components_month()),
                day: Some(length.to_components_day()),
                weekday: Some(length.to_components_text()),
                ..Default::default()
            },
            Self::EraYearMonthDayWeekday => components::Bag {
                era: Some(length.to_components_text()),
                year: Some(length.to_components_year()),
                month: Some(length.to_components_month()),
                day: Some(length.to_components_day()),
                weekday: Some(length.to_components_text()),
                ..Default::default()
            },
            Self::Weekday => components::Bag {
                weekday: Some(length.to_components_text()),
                ..Default::default()
            },
        }
    }
}

/// A specification for a set of parts of a date.
/// Only sets that yield “sensible” dates are allowed: this type cannot describe
/// a date such as “fourth quarter, Anno Domini”.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DateComponents {
    /// A date that specifies a single day.  Prefer constructing using [`Into`].
    Day(DayComponents),
    /// A standalone month, as in “January”.
    Month,
    /// A month and year, as in “January 2000”.
    YearMonth,
    /// A month, year, and era, as in “January 2000 A.D”.
    EraYearMonth,
    /// A year, as in “2000”.
    Year,
    /// A year with era, as in “2000 A.D.”.
    EraYear,
    /// The year and week of the year, as in “52nd week of 1999”.
    YearWeek,
    /// The quarter of the year, as in “1st quarter”.
    Quarter,
    /// The year and quarter of the year, as in “1st quarter of 2000”.
    YearQuarter,
}

impl DateComponents {
    /// All values of this enum.
    pub const VALUES: &'static [Self] = &[
        Self::Day(DayComponents::Day),
        Self::Day(DayComponents::MonthDay),
        Self::Day(DayComponents::YearMonthDay),
        Self::Day(DayComponents::EraYearMonthDay),
        Self::Day(DayComponents::DayWeekday),
        Self::Day(DayComponents::MonthDayWeekday),
        Self::Day(DayComponents::YearMonthDayWeekday),
        Self::Day(DayComponents::EraYearMonthDayWeekday),
        Self::Day(DayComponents::Weekday),
        Self::Month,
        Self::YearMonth,
        Self::EraYearMonth,
        Self::Year,
        Self::EraYear,
        Self::YearWeek,
        Self::Quarter,
        Self::YearQuarter,
    ];

    fn to_components_bag_with_length(self, length: Length) -> components::Bag {
        match self {
            Self::Day(day_components) => day_components.to_components_bag_with_length(length),
            Self::Month => components::Bag {
                month: Some(length.to_components_month()),
                ..Default::default()
            },
            Self::YearMonth => components::Bag {
                year: Some(length.to_components_year()),
                month: Some(length.to_components_month()),
                ..Default::default()
            },
            Self::EraYearMonth => components::Bag {
                era: Some(length.to_components_text()),
                year: Some(length.to_components_year()),
                month: Some(length.to_components_month()),
                ..Default::default()
            },
            Self::Year => components::Bag {
                year: Some(length.to_components_year()),
                ..Default::default()
            },
            Self::EraYear => components::Bag {
                era: Some(length.to_components_text()),
                year: Some(length.to_components_year()),
                ..Default::default()
            },
            Self::YearWeek => components::Bag {
                year: Some(length.to_components_year_of_week()),
                week: Some(length.to_components_week_of_year()),
                ..Default::default()
            },
            Self::Quarter => todo!(),
            Self::YearQuarter => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(missing_docs)] // TODO
#[non_exhaustive]
pub enum TimeComponents {
    Hour,
    HourMinute,
    HourMinuteSecond,
    DayPeriodHour12,
    Hour12,
    DayPeriodHour12Minute,
    Hour12Minute,
    DayPeriodHour12MinuteSecond,
    Hour12MinuteSecond,
    Hour24,
    Hour24Minute,
    Hour24MinuteSecond,
}

impl TimeComponents {
    /// All values of this enum.
    pub const VALUES: &'static [Self] = &[
        Self::Hour,
        Self::HourMinute,
        Self::HourMinuteSecond,
        Self::DayPeriodHour12,
        Self::Hour12,
        Self::DayPeriodHour12Minute,
        Self::Hour12Minute,
        Self::DayPeriodHour12MinuteSecond,
        Self::Hour12MinuteSecond,
        Self::Hour24,
        Self::Hour24Minute,
        Self::Hour24MinuteSecond,
    ];

    fn to_components_bag_with_length(self, length: Length) -> components::Bag {
        match self {
            Self::Hour => components::Bag {
                hour: Some(length.to_components_numeric()),
                ..Default::default()
            },
            Self::HourMinute => components::Bag {
                hour: Some(length.to_components_numeric()),
                minute: Some(length.to_components_numeric()),
                ..Default::default()
            },
            Self::HourMinuteSecond => components::Bag {
                hour: Some(length.to_components_numeric()),
                minute: Some(length.to_components_numeric()),
                second: Some(length.to_components_numeric()),
                ..Default::default()
            },
            Self::DayPeriodHour12 => todo!(),
            Self::Hour12 => todo!(),
            Self::DayPeriodHour12Minute => todo!(),
            Self::Hour12Minute => todo!(),
            Self::DayPeriodHour12MinuteSecond => todo!(),
            Self::Hour12MinuteSecond => todo!(),
            Self::Hour24 => todo!(),
            Self::Hour24Minute => todo!(),
            Self::Hour24MinuteSecond => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NeoComponents {
    Date(DateComponents),
    Time(TimeComponents),
    DateTime(DayComponents, TimeComponents),
}

impl NeoComponents {
    fn to_components_bag_with_length(self, length: Length) -> components::Bag {
        match self {
            Self::Date(inner) => inner.to_components_bag_with_length(length),
            Self::Time(inner) => inner.to_components_bag_with_length(length),
            Self::DateTime(inner0, inner1) => inner0
                .to_components_bag_with_length(length)
                .merge(inner1.to_components_bag_with_length(length)),
        }
    }
}

/// Specification of the time zone display style
#[derive(Debug, Copy, Clone, Default)]
#[non_exhaustive]
pub enum TimeZoneStyle {
    /// The location format, e.g., “Los Angeles time” or specific non-location
    /// format “Pacific Daylight Time”, whichever is idiomatic for the locale.
    /// > Note: for now, this is always identical to
    /// > [`TimeZoneStyle::SpecificNonLocation`] (Pacific Daylight Time), but
    /// > whether it is [`TimeZoneStyle::NonLocation`] or
    /// > [`TimeZoneStyle::SpecificNonLocation`] will be locale-dependent in the
    /// > future; see
    /// > [CLDR-15566](https://unicode-org.atlassian.net/browse/CLDR-15566).
    #[default]
    Default,
    /// The location format, e.g., “Los Angeles time”.
    Location,
    /// The generic non-location format, e.g., “Pacific Time”.
    NonLocation,
    /// The specific non-location format, e.g., “Pacific Daylight Time”.
    SpecificNonLocation,
    /// The offset from UTC format, e.g., “GMT−8”.
    Offset,
}

/// Specification of a time zone style with an optional length.
#[derive(Debug, Copy, Clone, Default)]
pub struct TimeZone {
    /// The length of the time zone format, _i.e._, with
    /// `style`=[`TimeZoneStyle::NonLocation`], whether to format as “Pacific
    /// Time” ([`Length::Long`]) or “PT” ([`Length::Short`]).
    /// If this is [`None`], the length is deduced from the [`Length`] of the
    /// enclosing [`SemanticSkeleton`] when formatting.
    pub length: Option<Length>,
    /// The style, _i.e._, with `length`=[`Length::Short`], whether to format as
    /// “GMT−8” ([`TimeZoneStyle::Offset`]) or “PT”
    /// ([`TimeZoneStyle::NonLocation`]).
    pub style: TimeZoneStyle,
}

#[derive(Debug, Copy, Clone)]
pub struct DateSkeleton {
    pub length: Length,
    pub components: DateComponents,
}

impl DateSkeleton {
    /// Converts this [`DateSkeleton`] to a [`components::Bag`].
    pub fn to_components_bag(self) -> components::Bag {
        self.components.to_components_bag_with_length(self.length)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TimeSkeleton {
    /// Desired formatting length.
    pub length: Length,
    /// Time time components of the skeleton.
    pub components: TimeComponents,
}

impl TimeSkeleton {
    /// Converts this [`TimeSkeleton`] to a [`components::Bag`].
    pub fn to_components_bag(self) -> components::Bag {
        self.components.to_components_bag_with_length(self.length)
    }
}

/// Representation of a semantic skeleton, agnostic to fields or calendar system.
#[derive(Debug, Copy, Clone)]
pub struct DateTimeSkeleton {
    /// Desired formatting length.
    pub length: Length,
    pub date: DayComponents,
    pub time: TimeComponents,
}

impl DateTimeSkeleton {
    /// Converts this [`DateTimeSkeleton`] to a [`components::Bag`].
    pub fn to_components_bag(self) -> components::Bag {
        self.date
            .to_components_bag_with_length(self.length)
            .merge(self.time.to_components_bag_with_length(self.length))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NeoSkeleton {
    pub length: Length,
    pub components: NeoComponents,
    pub time_zone: Option<TimeZone>,
}

impl NeoSkeleton {
    /// Converts this [`NeoSkeleton`] to a [`components::Bag`].
    pub fn to_components_bag(self) -> components::Bag {
        self.components.to_components_bag_with_length(self.length)
    }
}
