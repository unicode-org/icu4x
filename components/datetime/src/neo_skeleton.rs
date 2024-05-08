// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo datetime skeletons (Semantic Skeleta)

use crate::calendar::NeverCalendar;
use crate::options::components;
use crate::provider::neo::*;
use crate::CldrCalendar;
use icu_provider::prelude::*;
use icu_provider::NeverMarker;
use tinystr::tinystr;
use tinystr::TinyAsciiStr;

/// Sealed trait implemented by neo skeleton marker types.
pub trait NeoSkeletonCommonData {
    /// Marker for loading weekday names.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading day period names.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type DayPeriodNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading time skeleton patterns.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type TimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading the date/time glue pattern.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type DateTimePatternV1Marker: KeyedDataMarker<Yokeable = DateTimePatternV1<'static>>;
}

/// Sealed trait implemented by neo skeleton marker types.
pub trait TypedNeoSkeletonData<C>: NeoSkeletonCommonData
where
    C: CldrCalendar + ?Sized,
{
    /// Marker for loading year names.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type YearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;
    /// Marker for loading month names.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type MonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;
    /// Marker for loading date skeleton patterns.
    /// Can be [`NeverMarker`] if not needed for this skeleton.
    type DateSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
}

/// Sealed trait implemented by neo skeleton marker types.
pub trait NeoSkeletonData: NeoSkeletonCommonData {
    type BuddhistYearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;
    type BuddhistMonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;
    type BuddhistDateNeoSkeletonPatternsV1Marker: KeyedDataMarker<
        Yokeable = PackedSkeletonDataV1<'static>,
    >;
    type GregorianYearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;
    type GregorianMonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;
    type GregorianDateNeoSkeletonPatternsV1Marker: KeyedDataMarker<
        Yokeable = PackedSkeletonDataV1<'static>,
    >;
}

/// Sealed trait implemented by neo skeleton marker types.
pub trait NeoSkeletonComponents {
    /// Components in the neo skeleton.
    const COMPONENTS: NeoComponents;
}

/// A specification for the length of a date or component of a date.
///
/// Contrary to [`crate::options::length::Time`] and
/// [`crate::options::length::Date`], this has only three levels, with no
/// `Full`; this is because `Full` corresponds to additional components,
/// rather than to making the components wider than in `Long`.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum NeoSkeletonLength {
    /// A long date, typically spelled-out, as in “January 1, 2000”.
    Long,
    /// A medium-sized date; typically abbreviated, as in “Jan. 1, 2000”.
    Medium,
    /// A short date; typically numeric, as in “1/1/2000”.
    Short,
}

impl NeoSkeletonLength {
    /// All values of this enum.
    pub const VALUES: &'static [Self] = &[Self::Long, Self::Medium, Self::Short];

    fn to_components_text(self) -> components::Text {
        match self {
            Self::Long => components::Text::Long,
            Self::Medium => components::Text::Short,
            // Note: Do not use narrow in skeleton. It may still appear in patterns.
            Self::Short => components::Text::Short,
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

/// Marker for a month and year, as in
/// “January 2000”.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum YearMonthMarker {}

impl NeoSkeletonCommonData for YearMonthMarker {
    // Data to exclude
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<C> TypedNeoSkeletonData<C> for YearMonthMarker
where
    C: CldrCalendar,
{
    // Data to include
    type YearNamesV1Marker = C::YearNamesV1Marker;
    type MonthNamesV1Marker = C::MonthNamesV1Marker;
    type DateSkeletonPatternsV1Marker = C::DateSkeletonPatternsV1Marker;
}

impl NeoSkeletonData for YearMonthMarker {
    // Data to include
    type BuddhistYearNamesV1Marker = BuddhistYearNamesV1Marker;
    type BuddhistMonthNamesV1Marker = BuddhistMonthNamesV1Marker;
    type BuddhistDateNeoSkeletonPatternsV1Marker = BuddhistDateNeoSkeletonPatternsV1Marker;
    type GregorianYearNamesV1Marker = GregorianYearNamesV1Marker;
    type GregorianMonthNamesV1Marker = GregorianMonthNamesV1Marker;
    type GregorianDateNeoSkeletonPatternsV1Marker = GregorianDateNeoSkeletonPatternsV1Marker;
}

impl NeoSkeletonComponents for YearMonthMarker {
    const COMPONENTS: NeoComponents = NeoComponents::Date(NeoDateComponents::YearMonth);
}

/// Marker for a day, month, and year, as in
/// “January 1, 2000”.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum YearMonthDayMarker {}

impl NeoSkeletonCommonData for YearMonthDayMarker {
    // Data to include
    type WeekdayNamesV1Marker = WeekdayNamesV1Marker;

    // Data to exclude
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<C> TypedNeoSkeletonData<C> for YearMonthDayMarker
where
    C: CldrCalendar,
{
    // Data to include
    type YearNamesV1Marker = C::YearNamesV1Marker;
    type MonthNamesV1Marker = C::MonthNamesV1Marker;
    type DateSkeletonPatternsV1Marker = C::DateSkeletonPatternsV1Marker;
}

impl NeoSkeletonData for YearMonthDayMarker {
    // Data to include
    type BuddhistYearNamesV1Marker = BuddhistYearNamesV1Marker;
    type BuddhistMonthNamesV1Marker = BuddhistMonthNamesV1Marker;
    type BuddhistDateNeoSkeletonPatternsV1Marker = BuddhistDateNeoSkeletonPatternsV1Marker;
    type GregorianYearNamesV1Marker = GregorianYearNamesV1Marker;
    type GregorianMonthNamesV1Marker = GregorianMonthNamesV1Marker;
    type GregorianDateNeoSkeletonPatternsV1Marker = GregorianDateNeoSkeletonPatternsV1Marker;
}

impl NeoSkeletonComponents for YearMonthDayMarker {
    const COMPONENTS: NeoComponents =
        NeoComponents::Date(NeoDateComponents::Day(NeoDayComponents::YearMonthDay));
}

/// Marker for an hour and minute (12-hour or 24-hour chosen by locale), as in
/// "4:03 pm" or "16:03"
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum HourMinuteMarker {}

impl NeoSkeletonCommonData for HourMinuteMarker {
    // Data to include
    type DayPeriodNamesV1Marker = DayPeriodNamesV1Marker;
    type TimeSkeletonPatternsV1Marker = TimeNeoSkeletonPatternsV1Marker;

    // Data to exclude
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl TypedNeoSkeletonData<NeverCalendar> for HourMinuteMarker {
    // Data to exclude
    type YearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type MonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type DateSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
}

impl NeoSkeletonData for HourMinuteMarker {
    // Data to exclude
    type BuddhistYearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type BuddhistMonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type BuddhistDateNeoSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type GregorianYearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type GregorianMonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type GregorianDateNeoSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
}

impl NeoSkeletonComponents for HourMinuteMarker {
    const COMPONENTS: NeoComponents = NeoComponents::Time(NeoTimeComponents::HourMinute);
}

// TODO: Add more of these TypedNeoSkeletonData marker types.

/// A specification for a set of parts of a date that specifies a single day (as
/// opposed to a whole month, week, or quarter).
/// Only sets that yield “sensible” dates are allowed: this type cannot
/// describe a date such as “some Tuesday in 2023”.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NeoDayComponents {
    /// The day of the month, as in
    /// “on the 1st”.
    Day,
    /// The month and day of the month, as in
    /// “January 1st”.
    MonthDay,
    /// The year, month, and day of the month, as in
    /// “January 1st, 2000”.
    YearMonthDay,
    /// The era, year, month, and day of the month, as in
    /// “January 1st, 2000 A.D.”.
    EraYearMonthDay,
    /// The day of the month and day of the week, as in
    /// “Saturday 1st”.
    DayWeekday,
    /// The month, day of the month, and day of the week, as in
    /// “Saturday, January 1st”.
    MonthDayWeekday,
    /// The year, month, day of the month, and day of the week, as in
    /// “Saturday, January 1st, 2000”.
    YearMonthDayWeekday,
    /// The era, year, month, day of the month, and day of the week, as in
    /// “Saturday, January 1st, 2000 A.D.”.
    EraYearMonthDayWeekday,
    /// The day of the week alone, as in
    /// “Saturday”.
    Weekday,
}

impl NeoDayComponents {
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

    const DAY_STR: TinyAsciiStr<8> = tinystr!(8, "d");
    const MONTH_DAY_STR: TinyAsciiStr<8> = tinystr!(8, "m0d");
    const YEAR_MONTH_DAY_STR: TinyAsciiStr<8> = tinystr!(8, "ym0d");
    const ERA_YEAR_MONTH_DAY_STR: TinyAsciiStr<8> = tinystr!(8, "gym0d");
    const DAY_WEEKDAY_STR: TinyAsciiStr<8> = tinystr!(8, "de");
    const MONTH_DAY_WEEKDAY_STR: TinyAsciiStr<8> = tinystr!(8, "m0de");
    const YEAR_MONTH_DAY_WEEKDAY_STR: TinyAsciiStr<8> = tinystr!(8, "ym0de");
    const ERA_YEAR_MONTH_DAY_WEEKDAY_STR: TinyAsciiStr<8> = tinystr!(8, "gym0de");
    const WEEKDAY_STR: TinyAsciiStr<8> = tinystr!(8, "e");

    /// Returns a stable string identifying this set of components.
    ///
    /// # Encoding Details
    ///
    /// The string is based roughly on the UTS 35 symbol table with the following exceptions:
    ///
    /// 1. Lowercase letters are chosen where there is no ambiguity: `G` becomes `g`\*
    /// 2. Capitals are replaced with their lowercase and a number 0: `M` becomes `m0`
    /// 3. A single symbol is included for each component: length doesn't matter
    ///
    /// \* `g` represents a different field, but it is never used in skeleta.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_datetime::neo_skeleton::NeoDayComponents;
    ///
    /// assert_eq!("gym0de", &*NeoDayComponents::EraYearMonthDayWeekday.id_str());
    /// ```
    pub const fn id_str(self) -> TinyAsciiStr<8> {
        match self {
            Self::Day => Self::DAY_STR,
            Self::MonthDay => Self::MONTH_DAY_STR,
            Self::YearMonthDay => Self::YEAR_MONTH_DAY_STR,
            Self::EraYearMonthDay => Self::ERA_YEAR_MONTH_DAY_STR,
            Self::DayWeekday => Self::DAY_WEEKDAY_STR,
            Self::MonthDayWeekday => Self::MONTH_DAY_WEEKDAY_STR,
            Self::YearMonthDayWeekday => Self::YEAR_MONTH_DAY_WEEKDAY_STR,
            Self::EraYearMonthDayWeekday => Self::ERA_YEAR_MONTH_DAY_WEEKDAY_STR,
            Self::Weekday => Self::WEEKDAY_STR,
        }
    }

    /// Returns the set of components for the given stable string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_datetime::neo_skeleton::NeoDayComponents;
    /// use tinystr::tinystr;
    ///
    /// assert_eq!(NeoDayComponents::from_id_str(tinystr!(8, "gym0de")), Some(NeoDayComponents::EraYearMonthDayWeekday));
    /// ```
    pub const fn from_id_str(id_str: TinyAsciiStr<8>) -> Option<Self> {
        match id_str {
            Self::DAY_STR => Some(Self::Day),
            Self::MONTH_DAY_STR => Some(Self::MonthDay),
            Self::YEAR_MONTH_DAY_STR => Some(Self::YearMonthDay),
            Self::ERA_YEAR_MONTH_DAY_STR => Some(Self::EraYearMonthDay),
            Self::DAY_WEEKDAY_STR => Some(Self::DayWeekday),
            Self::MONTH_DAY_WEEKDAY_STR => Some(Self::MonthDayWeekday),
            Self::YEAR_MONTH_DAY_WEEKDAY_STR => Some(Self::YearMonthDayWeekday),
            Self::ERA_YEAR_MONTH_DAY_WEEKDAY_STR => Some(Self::EraYearMonthDayWeekday),
            Self::WEEKDAY_STR => Some(Self::Weekday),
            _ => None,
        }
    }

    fn to_components_bag_with_length(self, length: NeoSkeletonLength) -> components::Bag {
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NeoDateComponents {
    /// A date that specifies a single day. See [`NeoDayComponents`].
    Day(NeoDayComponents),
    /// A standalone month, as in
    /// “January”.
    Month,
    /// A month and year, as in
    /// “January 2000”.
    YearMonth,
    /// A month, year, and era, as in
    /// “January 2000 A.D”.
    EraYearMonth,
    /// A year, as in
    /// “2000”.
    Year,
    /// A year with era, as in
    /// “2000 A.D.”.
    EraYear,
    /// The year and week of the year, as in
    /// “52nd week of 1999”.
    YearWeek,
    /// The quarter of the year, as in
    /// “1st quarter”.
    Quarter,
    /// The year and quarter of the year, as in
    /// “1st quarter of 2000”.
    YearQuarter,
}

impl NeoDateComponents {
    /// All values of this enum.
    pub const VALUES: &'static [Self] = &[
        Self::Day(NeoDayComponents::Day),
        Self::Day(NeoDayComponents::MonthDay),
        Self::Day(NeoDayComponents::YearMonthDay),
        Self::Day(NeoDayComponents::EraYearMonthDay),
        Self::Day(NeoDayComponents::DayWeekday),
        Self::Day(NeoDayComponents::MonthDayWeekday),
        Self::Day(NeoDayComponents::YearMonthDayWeekday),
        Self::Day(NeoDayComponents::EraYearMonthDayWeekday),
        Self::Day(NeoDayComponents::Weekday),
        Self::Month,
        Self::YearMonth,
        Self::EraYearMonth,
        Self::Year,
        Self::EraYear,
        Self::YearWeek,
        Self::Quarter,
        Self::YearQuarter,
    ];

    const MONTH_STR: TinyAsciiStr<8> = tinystr!(8, "m0");
    const YEAR_MONTH_STR: TinyAsciiStr<8> = tinystr!(8, "ym0");
    const ERA_YEAR_MONTH_STR: TinyAsciiStr<8> = tinystr!(8, "gym0");
    const YEAR_STR: TinyAsciiStr<8> = tinystr!(8, "y");
    const ERA_YEAR_STR: TinyAsciiStr<8> = tinystr!(8, "gy");
    const YEAR_WEEK_STR: TinyAsciiStr<8> = tinystr!(8, "y0w");
    const QUARTER_STR: TinyAsciiStr<8> = tinystr!(8, "q");
    const YEAR_QUARTER_STR: TinyAsciiStr<8> = tinystr!(8, "yq");

    /// Returns a stable string identifying this set of components.
    ///
    /// For details, see [`NeoDayComponents::id_str()`].
    pub const fn id_str(self) -> TinyAsciiStr<8> {
        match self {
            Self::Day(day_components) => day_components.id_str(),
            Self::Month => Self::MONTH_STR,
            Self::YearMonth => Self::YEAR_MONTH_STR,
            Self::EraYearMonth => Self::ERA_YEAR_MONTH_STR,
            Self::Year => Self::YEAR_STR,
            Self::EraYear => Self::ERA_YEAR_STR,
            Self::YearWeek => Self::YEAR_WEEK_STR,
            Self::Quarter => Self::QUARTER_STR,
            Self::YearQuarter => Self::YEAR_QUARTER_STR,
        }
    }

    /// Returns the set of components for the given stable string.
    ///
    /// For details, see [`NeoDayComponents::from_id_str()`].
    pub const fn from_id_str(id_str: TinyAsciiStr<8>) -> Option<Self> {
        match id_str {
            Self::MONTH_STR => Some(Self::Month),
            Self::YEAR_MONTH_STR => Some(Self::YearMonth),
            Self::ERA_YEAR_MONTH_STR => Some(Self::EraYearMonth),
            Self::YEAR_STR => Some(Self::Year),
            Self::ERA_YEAR_STR => Some(Self::EraYear),
            Self::YEAR_WEEK_STR => Some(Self::YearWeek),
            Self::QUARTER_STR => Some(Self::Quarter),
            Self::YEAR_QUARTER_STR => Some(Self::YearQuarter),
            other => match NeoDayComponents::from_id_str(other) {
                Some(day_components) => Some(Self::Day(day_components)),
                None => None,
            },
        }
    }

    fn to_components_bag_with_length(self, length: NeoSkeletonLength) -> components::Bag {
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

/// A specification for a set of parts of a time.
/// Only sets that yield “sensible” time are allowed: this type cannot describe
/// a time such as “am, 5 minutes, 25 milliseconds”.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NeoTimeComponents {
    /// An hour (12-hour or 24-hour chosen by locale), as in
    /// "4 pm" or "16h"
    Hour,
    /// An hour and minute (12-hour or 24-hour chosen by locale), as in
    /// "4:03 pm" or "16:03"
    HourMinute,
    /// An hour, minute, and second (12-hour or 24-hour chosen by locale), as in
    /// "4:03:51 pm" or "16:03:51"
    HourMinuteSecond,
    /// An hour with a 12-hour clock and day period, as in
    /// "4 in the afternoon"
    DayPeriodHour12,
    /// An hour with a 12-hour clock, as in
    /// "4 pm"
    Hour12,
    /// An hour and minute with a 12-hour clock and a day period, as in
    /// "4:03 in the afternoon"
    DayPeriodHour12Minute,
    /// An hour and minute with a 12-hour clock, as in
    /// "4:03 pm"
    Hour12Minute,
    /// An hour, minute, and second with a 12-hour clock and day period, as in
    /// "4:03:51 in the afternoon"
    DayPeriodHour12MinuteSecond,
    /// An hour, minute, and second with a 12-hour clock, as in
    /// "4:03:51 pm"
    Hour12MinuteSecond,
    /// An hour with a 24-hour clock, as in
    /// "16h"
    Hour24,
    /// An hour and minute with a 24-hour clock, as in
    /// "16:03"
    Hour24Minute,
    /// An hour, minute, and second with a 24-hour clock, as in
    /// "16:03:51"
    Hour24MinuteSecond,
}

impl NeoTimeComponents {
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

    const HOUR_STR: TinyAsciiStr<8> = tinystr!(8, "j");
    const HOUR_MINUTE_STR: TinyAsciiStr<8> = tinystr!(8, "jm");
    const HOUR_MINUTE_SECOND_STR: TinyAsciiStr<8> = tinystr!(8, "jms");
    const DAY_PERIOD_HOUR12_STR: TinyAsciiStr<8> = tinystr!(8, "bh");
    const HOUR12_STR: TinyAsciiStr<8> = tinystr!(8, "h");
    const DAY_PERIOD_HOUR12_MINUTE_STR: TinyAsciiStr<8> = tinystr!(8, "bhm");
    const HOUR12_MINUTE_STR: TinyAsciiStr<8> = tinystr!(8, "hm");
    const DAY_PERIOD_HOUR12_MINUTE_SECOND_STR: TinyAsciiStr<8> = tinystr!(8, "bhms");
    const HOUR12_MINUTE_SECOND_STR: TinyAsciiStr<8> = tinystr!(8, "hms");
    const HOUR24_STR: TinyAsciiStr<8> = tinystr!(8, "h0");
    const HOUR24_MINUTE_STR: TinyAsciiStr<8> = tinystr!(8, "h0m");
    const HOUR24_MINUTE_SECOND_STR: TinyAsciiStr<8> = tinystr!(8, "h0ms");

    /// Returns a stable string identifying this set of components.
    ///
    /// For details, see [`NeoDayComponents::id_str()`].
    pub const fn id_str(self) -> TinyAsciiStr<8> {
        match self {
            Self::Hour => Self::HOUR_STR,
            Self::HourMinute => Self::HOUR_MINUTE_STR,
            Self::HourMinuteSecond => Self::HOUR_MINUTE_SECOND_STR,
            Self::DayPeriodHour12 => Self::DAY_PERIOD_HOUR12_STR,
            Self::Hour12 => Self::HOUR12_STR,
            Self::DayPeriodHour12Minute => Self::DAY_PERIOD_HOUR12_MINUTE_STR,
            Self::Hour12Minute => Self::HOUR12_MINUTE_STR,
            Self::DayPeriodHour12MinuteSecond => Self::DAY_PERIOD_HOUR12_MINUTE_SECOND_STR,
            Self::Hour12MinuteSecond => Self::HOUR12_MINUTE_SECOND_STR,
            Self::Hour24 => Self::HOUR24_STR,
            Self::Hour24Minute => Self::HOUR24_MINUTE_STR,
            Self::Hour24MinuteSecond => Self::HOUR24_MINUTE_SECOND_STR,
        }
    }

    /// Returns the set of components for the given stable string.
    ///
    /// For details, see [`NeoDayComponents::from_id_str()`].
    pub const fn from_id_str(id_str: TinyAsciiStr<8>) -> Option<Self> {
        match id_str {
            Self::HOUR_STR => Some(Self::Hour),
            Self::HOUR_MINUTE_STR => Some(Self::HourMinute),
            Self::HOUR_MINUTE_SECOND_STR => Some(Self::HourMinuteSecond),
            Self::DAY_PERIOD_HOUR12_STR => Some(Self::DayPeriodHour12),
            Self::HOUR12_STR => Some(Self::Hour12),
            Self::DAY_PERIOD_HOUR12_MINUTE_STR => Some(Self::DayPeriodHour12Minute),
            Self::HOUR12_MINUTE_STR => Some(Self::Hour12Minute),
            Self::DAY_PERIOD_HOUR12_MINUTE_SECOND_STR => Some(Self::DayPeriodHour12MinuteSecond),
            Self::HOUR12_MINUTE_SECOND_STR => Some(Self::Hour12MinuteSecond),
            Self::HOUR24_STR => Some(Self::Hour24),
            Self::HOUR24_MINUTE_STR => Some(Self::Hour24Minute),
            Self::HOUR24_MINUTE_SECOND_STR => Some(Self::Hour24MinuteSecond),
            _ => None,
        }
    }

    fn to_components_bag_with_length(self, length: NeoSkeletonLength) -> components::Bag {
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

/// A specification of components for parts of a datetime.
#[derive(Debug, Copy, Clone)]
#[allow(clippy::exhaustive_enums)] // well-defined type
pub enum NeoComponents {
    /// Components for parts of a date.
    Date(NeoDateComponents),
    /// Components for parts of a time.
    Time(NeoTimeComponents),
    /// Components for parts of a date and time together.
    DateTime(NeoDayComponents, NeoTimeComponents),
}

impl NeoComponents {
    fn to_components_bag_with_length(self, length: NeoSkeletonLength) -> components::Bag {
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
pub enum NeoTimeZoneStyle {
    /// The location format, e.g., “Los Angeles time” or specific non-location
    /// format “Pacific Daylight Time”, whichever is idiomatic for the locale.
    /// > Note: for now, this is always identical to
    /// > [`NeoTimeZoneStyle::SpecificNonLocation`] (Pacific Daylight Time), but
    /// > whether it is [`NeoTimeZoneStyle::NonLocation`] or
    /// > [`NeoTimeZoneStyle::SpecificNonLocation`] will be locale-dependent in the
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

/// A skeleton for formatting a time zone.
#[derive(Debug, Copy, Clone, Default)]
#[non_exhaustive]
pub struct NeoTimeZoneSkeleton {
    /// The length of the time zone format, _i.e._, with
    /// `style`=[`NeoTimeZoneStyle::NonLocation`], whether to format as “Pacific
    /// Time” ([`NeoSkeletonLength::Long`]) or “PT” ([`NeoSkeletonLength::Short`]).
    /// If this is [`None`], the length is deduced from the [`NeoSkeletonLength`] of the
    /// enclosing [`NeoSkeleton`] when formatting.
    pub length: Option<NeoSkeletonLength>,
    /// The style, _i.e._, with `length`=[`NeoSkeletonLength::Short`], whether to format as
    /// “GMT−8” ([`NeoTimeZoneStyle::Offset`]) or “PT”
    /// ([`NeoTimeZoneStyle::NonLocation`]).
    pub style: NeoTimeZoneStyle,
}

/// A skeleton for formatting parts of a date (without time or time zone).
#[derive(Debug, Copy, Clone)]
#[allow(clippy::exhaustive_structs)] // well-defined type
pub struct NeoDateSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Date components of the skeleton.
    pub components: NeoDateComponents,
}

impl NeoDateSkeleton {
    /// Creates a skeleton from its length and components.
    pub fn for_length_and_components(
        length: NeoSkeletonLength,
        components: NeoDateComponents,
    ) -> Self {
        Self { length, components }
    }

    /// Converts this [`NeoDateSkeleton`] to a [`components::Bag`].
    pub fn to_components_bag(self) -> components::Bag {
        self.components.to_components_bag_with_length(self.length)
    }
}

/// A skeleton for formatting parts of a time (without date or time zone).
#[derive(Debug, Copy, Clone)]
#[allow(clippy::exhaustive_structs)] // well-defined type
pub struct NeoTimeSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Time components of the skeleton.
    pub components: NeoTimeComponents,
}

impl NeoTimeSkeleton {
    /// Creates a skeleton from its length and components.
    pub fn for_length_and_components(
        length: NeoSkeletonLength,
        components: NeoTimeComponents,
    ) -> Self {
        Self { length, components }
    }

    /// Converts this [`NeoTimeSkeleton`] to a [`components::Bag`].
    pub fn to_components_bag(self) -> components::Bag {
        self.components.to_components_bag_with_length(self.length)
    }
}

/// A skeleton for formatting parts of a date and time (without time zone).
#[derive(Debug, Copy, Clone)]
#[allow(clippy::exhaustive_structs)] // well-defined type
pub struct NeoDateTimeSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Date components of the skeleton.
    pub date: NeoDayComponents,
    /// Time components of the skeleton.
    pub time: NeoTimeComponents,
}

impl NeoDateTimeSkeleton {
    /// Creates a skeleton from its length and components.
    pub fn for_length_and_components(
        length: NeoSkeletonLength,
        date: NeoDayComponents,
        time: NeoTimeComponents,
    ) -> Self {
        Self { length, date, time }
    }

    /// Converts this [`NeoDateTimeSkeleton`] to a [`components::Bag`].
    pub fn to_components_bag(self) -> components::Bag {
        self.date
            .to_components_bag_with_length(self.length)
            .merge(self.time.to_components_bag_with_length(self.length))
    }
}

/// A skeleton for formatting parts of a date, time, and optional time zone.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoSkeleton {
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Components of the skeleton.
    pub components: NeoComponents,
    /// Optional time zone skeleton.
    pub time_zone: Option<NeoTimeZoneSkeleton>,
}

impl NeoSkeleton {
    /// Converts this [`NeoSkeleton`] to a [`components::Bag`].
    pub fn to_components_bag(self) -> components::Bag {
        self.components.to_components_bag_with_length(self.length)
    }
}

impl From<NeoDateSkeleton> for NeoSkeleton {
    fn from(value: NeoDateSkeleton) -> Self {
        NeoSkeleton {
            length: value.length,
            components: NeoComponents::Date(value.components),
            time_zone: None,
        }
    }
}

impl From<NeoTimeSkeleton> for NeoSkeleton {
    fn from(value: NeoTimeSkeleton) -> Self {
        NeoSkeleton {
            length: value.length,
            components: NeoComponents::Time(value.components),
            time_zone: None,
        }
    }
}

impl From<NeoDateTimeSkeleton> for NeoSkeleton {
    fn from(value: NeoDateTimeSkeleton) -> Self {
        NeoSkeleton {
            length: value.length,
            components: NeoComponents::DateTime(value.date, value.time),
            time_zone: None,
        }
    }
}
