// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo formatter markers.

use core::marker::PhantomData;

use crate::{format::neo::*, neo::_internal::*, neo_skeleton::*, provider::neo::*, CldrCalendar};
use icu_calendar::{
    types::{
        DayOfMonth, DayOfYearInfo, FormattableMonth, FormattableYear, IsoHour, IsoMinute,
        IsoSecond, IsoWeekday, NanoSecond,
    },
    AnyCalendarKind, AsCalendar, Calendar, Date, DateTime,
};
use icu_provider::{prelude::*, NeverMarker};

mod private {
    pub trait Sealed {}
}

/// Input data fields required for formatting dates.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoDateInputFields {
    /// The era and year input.
    pub year: FormattableYear,
    /// The month input.
    pub month: FormattableMonth,
    /// The day-of-month input.
    pub day_of_month: DayOfMonth,
    /// The day-of-week input.
    pub day_of_week: IsoWeekday,
    /// The kind of calendar this date is for, if associated with [`AnyCalendar`].
    ///
    /// [`AnyCalendar`]: icu_calendar::AnyCalendar
    pub any_calendar_kind: Option<AnyCalendarKind>,
}

impl NeoDateInputFields {
    /// Constructor for [`NeoDateInputFields`] with all required fields.
    ///
    /// Optional fields such as [`Self::any_calendar_kind`] should be populated manually.
    pub fn new_with_year_month_day_weekday(
        year: FormattableYear,
        month: FormattableMonth,
        day_of_month: DayOfMonth,
        day_of_week: IsoWeekday,
    ) -> Self {
        Self {
            year,
            month,
            day_of_month,
            day_of_week,
            any_calendar_kind: None,
        }
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> From<&Date<A>> for NeoDateInputFields {
    fn from(value: &Date<A>) -> Self {
        Self {
            year: value.year(),
            month: value.month(),
            day_of_month: value.day_of_month(),
            day_of_week: value.day_of_week(),
            any_calendar_kind: value.calendar().any_calendar_kind(),
        }
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> From<&DateTime<A>> for NeoDateInputFields {
    fn from(value: &DateTime<A>) -> Self {
        Self {
            year: value.date.year(),
            month: value.date.month(),
            day_of_month: value.date.day_of_month(),
            day_of_week: value.date.day_of_week(),
            any_calendar_kind: value.date.calendar().any_calendar_kind(),
        }
    }
}

/// Input data fields required for formatting weeks of the year.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoWeekInputFields {
    /// Information on the position of the day within the year.
    pub day_of_year_info: DayOfYearInfo,
}

impl NeoWeekInputFields {
    /// Constructor for [`NeoWeekInputFields`] with all required fields.
    pub fn new_with_info(day_of_year_info: DayOfYearInfo) -> Self {
        Self { day_of_year_info }
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> From<&Date<A>> for NeoWeekInputFields {
    fn from(value: &Date<A>) -> Self {
        Self {
            day_of_year_info: value.day_of_year_info(),
        }
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> From<&DateTime<A>> for NeoWeekInputFields {
    fn from(value: &DateTime<A>) -> Self {
        Self {
            day_of_year_info: value.date.day_of_year_info(),
        }
    }
}

/// Input data fields required for formatting times.
// TODO: Should we directly use icu_calendar::Time here?
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoTimeInputFields {
    /// 0-based hour.
    pub hour: IsoHour,
    /// 0-based minute.
    pub minute: IsoMinute,
    /// 0-based second.
    pub second: IsoSecond,
    /// Fractional second
    pub nanosecond: NanoSecond,
}

impl NeoTimeInputFields {
    /// Constructor for [`NeoTimeInputFields`] with all required fields.
    pub fn new_with_hour_minute_second_nanosecond(
        hour: IsoHour,
        minute: IsoMinute,
        second: IsoSecond,
        nanosecond: NanoSecond,
    ) -> Self {
        Self {
            hour,
            minute,
            second,
            nanosecond,
        }
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> From<&DateTime<A>> for NeoTimeInputFields {
    fn from(value: &DateTime<A>) -> Self {
        Self {
            hour: value.time.hour,
            minute: value.time.minute,
            second: value.time.second,
            nanosecond: value.time.nanosecond,
        }
    }
}

/// Struct representing the absence of datetime formatting fields.
#[derive(Debug, Copy, Clone)]
#[allow(clippy::exhaustive_structs)] // empty marker struct
pub struct NeverFields;

impl<T> From<&T> for NeverFields {
    #[inline]
    fn from(_: &T) -> Self {
        NeverFields
    }
}

impl From<NeverFields> for Option<NeoDateInputFields> {
    #[inline]
    fn from(_: NeverFields) -> Self {
        None
    }
}

impl From<NeverFields> for Option<NeoWeekInputFields> {
    #[inline]
    fn from(_: NeverFields) -> Self {
        None
    }
}

impl From<NeverFields> for Option<NeoTimeInputFields> {
    #[inline]
    fn from(_: NeverFields) -> Self {
        None
    }
}

pub trait HasDateComponents {
    const COMPONENTS: NeoDateComponents;
}

pub trait HasDayComponents {
    const COMPONENTS: NeoDayComponents;
}

pub trait HasTimeComponents {
    const COMPONENTS: NeoTimeComponents;
}

// TODO: Add WeekCalculator and FixedDecimalFormatter optional bindings here

pub trait DateTypes: private::Sealed {
    /// Marker for loading date skeleton patterns.
    type DateSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for resolving date fields from the input.
    type DateInputMarker: Into<Option<NeoDateInputFields>>;
    /// Marker for resolving week-of-year fields from the input.
    type WeekInputMarker: Into<Option<NeoWeekInputFields>>;
    /// Marker for loading year names.
    type YearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;
    /// Marker for loading month names.
    type MonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
}

pub trait TimeTypes: private::Sealed {
    /// Marker for resolving time fields from the input.
    type TimeInputMarker: Into<Option<NeoTimeInputFields>>;
    /// Marker for loading time skeleton patterns.
    type TimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading day period names.
    type DayPeriodNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
}

#[derive(Debug)]
pub struct NeoNeverMarker;

impl private::Sealed for NeoNeverMarker {}

impl DateTypes for NeoNeverMarker {
    type DateSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateInputMarker = NeverFields;
    type WeekInputMarker = NeverFields;
    type YearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type MonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

impl TimeTypes for NeoNeverMarker {
    type TimeInputMarker = NeverFields;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

pub trait HasNeoComponents: private::Sealed {
    const COMPONENTS: NeoComponents;
    type D: DateTypes;
    type T: TimeTypes;
    /// Fields for [`TypedDateTimeNames`].
    type DateTimeNamesMarker: DateTimeNamesMarker;
    /// Marker for loading the date/time glue pattern.
    type DateTimePatternV1Marker: KeyedDataMarker<Yokeable = DateTimePatternV1<'static>>;
}

struct TypedNeoFormatter<DTZ: HasNeoComponents> {
    _marker: PhantomData<DTZ>,
}

#[derive(Debug)]
pub struct TypedNeoComponents<D, T> {
    _d: PhantomData<D>,
    _t: PhantomData<T>,
}

impl<D, T> private::Sealed for TypedNeoComponents<D, T> {}

// type MonthDayHour = TypedNeoComponents<MonthDay, Hour>;

impl<D> HasNeoComponents for TypedNeoComponents<D, NeoNeverMarker>
where
    D: HasDateComponents + DateTypes,
{
    const COMPONENTS: NeoComponents = NeoComponents::Date(D::COMPONENTS);
    type D = D;
    type T = NeoNeverMarker;
    type DateTimeNamesMarker = DateMarker;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<T> HasNeoComponents for TypedNeoComponents<NeoNeverMarker, T>
where
    T: HasTimeComponents + TimeTypes,
{
    const COMPONENTS: NeoComponents = NeoComponents::Time(T::COMPONENTS);
    type D = NeoNeverMarker;
    type T = T;
    type DateTimeNamesMarker = TimeMarker;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<D, T> HasNeoComponents for TypedNeoComponents<D, T>
where
    D: HasDayComponents + DateTypes,
    T: HasTimeComponents + TimeTypes,
{
    const COMPONENTS: NeoComponents = NeoComponents::DateTime(D::COMPONENTS, T::COMPONENTS);
    type D = D;
    type T = T;
    type DateTimeNamesMarker = DateTimeMarker;
    type DateTimePatternV1Marker = DateTimePatternV1Marker;
}

/// A collection of types and constants for specific variants of [`TypedNeoFormatter`].
///
/// Individual fields can be [`NeverMarker`] if they are not needed for the specific variant.
///
/// [`TypedNeoFormatter`]: crate::neo::TypedNeoFormatter
pub trait TypedNeoFormatterMarker<C: CldrCalendar>: private::Sealed {
    /// Components in the neo skeleton.
    const COMPONENTS: NeoComponents;
    /// Fields for [`TypedDateTimeNames`].
    type DateTimeNamesMarker: DateTimeNamesMarker;
    /// Marker for loading year names.
    type YearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;
    /// Marker for loading month names.
    type MonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;
    /// Marker for loading date skeleton patterns.
    type DateSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading day period names.
    type DayPeriodNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading time skeleton patterns.
    type TimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading the date/time glue pattern.
    type DateTimePatternV1Marker: KeyedDataMarker<Yokeable = DateTimePatternV1<'static>>;
    /// Marker for resolving date fields from the input.
    type DateInputMarker: Into<Option<NeoDateInputFields>>;
    /// Marker for resolving week-of-year fields from the input.
    type WeekInputMarker: Into<Option<NeoWeekInputFields>>;
    /// Marker for resolving time fields from the input.
    type TimeInputMarker: Into<Option<NeoTimeInputFields>>;
    // TODO: Add WeekCalculator and FixedDecimalFormatter optional bindings here
}

/// A collection of types and constants for specific variants of [`NeoFormatter`].
///
/// Individual fields can be [`NeverMarker`] if they are not needed for the specific variant.
///
/// The cross-calendar fields should be either [`FullDataCalMarkers`] or [`NoDataCalMarkers`].
///
/// [`NeoFormatter`]: crate::neo::NeoFormatter
pub trait NeoFormatterMarker {
    /// Components in the neo skeleton.
    const COMPONENTS: NeoComponents;
    /// Fields for [`TypedDateTimeNames`].
    type DateTimeNamesMarker: DateTimeNamesMarker;
    /// Cross-calendar data markers for year names.
    type Year: CalMarkers<YearNamesV1Marker>;
    /// Cross-calendar data markers for month names.
    type Month: CalMarkers<MonthNamesV1Marker>;
    /// Cross-calendar data markers for date skeleta.
    type Skel: CalMarkers<SkeletaV1Marker>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading day period names.
    type DayPeriodNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading time skeleton patterns.
    type TimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading the date/time glue pattern.
    type DateTimePatternV1Marker: KeyedDataMarker<Yokeable = DateTimePatternV1<'static>>;
    // TODO: Add WeekCalculator, FixedDecimalFormatter, and AnyCalendar optional bindings here
}

macro_rules! datetime_marker_helper {
    (@years/typed, yes) => {
        C::YearNamesV1Marker
    };
    (@years/typed, no) => {
        NeverMarker<YearNamesV1<'static>>
    };
    (@months/typed, yes) => {
        C::MonthNamesV1Marker
    };
    (@months/typed, no) => {
        NeverMarker<MonthNamesV1<'static>>
    };
    (@dates/typed, yes) => {
        C::SkeletaV1Marker
    };
    (@dates/typed, no) => {
        NeverMarker<PackedSkeletonDataV1<'static>>
    };
    (@calmarkers, yes) => {
        FullDataCalMarkers
    };
    (@calmarkers, no) => {
        NoDataCalMarkers
    };
    (@weekdays, yes) => {
        WeekdayNamesV1Marker
    };
    (@weekdays, no) => {
        NeverMarker<LinearNamesV1<'static>>
    };
    (@dayperiods, yes) => {
        DayPeriodNamesV1Marker
    };
    (@dayperiods, no) => {
        NeverMarker<LinearNamesV1<'static>>
    };
    (@times, yes) => {
        TimeNeoSkeletonPatternsV1Marker
    };
    (@times, no) => {
        NeverMarker<PackedSkeletonDataV1<'static>>
    };
    (@datetimes, yes) => {
        DateTimePatternV1Marker
    };
    (@datetimes, no) => {
        NeverMarker<DateTimePatternV1<'static>>
    };
    (@dateinput, yes) => {
        NeoDateInputFields
    };
    (@dateinput, no) => {
        NeverFields
    };
    (@weekinput, yes) => {
        NeoWeekInputFields
    };
    (@weekinput, no) => {
        NeverFields
    };
    (@timeinput, yes) => {
        NeoTimeInputFields
    };
    (@timeinput, no) => {
        NeverFields
    };
}

macro_rules! impl_datetime_marker {
    ($type:ident, $components:expr, description = $description:literal, expectation = $expectation:literal, names = $namemarker:path, years = $years_yesno:ident, months = $months_yesno:ident, dates = $dates_yesno:ident, weekdays = $weekdays_yesno:ident, dayperiods = $dayperiods_yesno:ident, times = $times_yesno:ident, datetimes = $datetimes_yesno:ident, dateinput = $dateinput_yesno:ident, weekinput = $weekinput_yesno:ident, timeinput = $timeinput_yesno:ident,) => {
        #[doc = concat!("Marker for ", $description, ": ", $expectation)]
        ///
        /// # Examples
        ///
        /// In [`NeoFormatter`](crate::neo::NeoFormatter):
        ///
        /// ```
        /// use icu::calendar::DateTime;
        /// use icu::datetime::neo::NeoFormatter;
        #[doc = concat!("use icu::datetime::neo_marker::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locid::locale;
        /// use writeable::assert_try_writeable_eq;
        #[doc = concat!("let fmt = NeoFormatter::<", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        ///     NeoSkeletonLength::Medium,
        /// )
        /// .unwrap();
        /// let dt = DateTime::try_new_iso_datetime(2024, 5, 17, 15, 47, 50).unwrap();
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.convert_and_format(&dt),
        #[doc = concat!("    \"", $expectation, "\"")]
        /// );
        /// ```
        ///
        /// In [`TypedNeoFormatter`](crate::neo::TypedNeoFormatter):
        ///
        /// ```
        /// use icu::calendar::DateTime;
        /// use icu::calendar::Gregorian;
        /// use icu::datetime::neo::TypedNeoFormatter;
        #[doc = concat!("use icu::datetime::neo_marker::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locid::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = TypedNeoFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        ///     NeoSkeletonLength::Medium,
        /// )
        /// .unwrap();
        /// let dt = DateTime::try_new_gregorian_datetime(2024, 5, 17, 15, 47, 50).unwrap();
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.format(&dt),
        #[doc = concat!("    \"", $expectation, "\"")]
        /// );
        /// ```
        #[derive(Debug)]
        #[allow(clippy::exhaustive_enums)] // empty enum
        pub enum $type {}
        impl private::Sealed for $type {}
        impl<C: Calendar + CldrCalendar> TypedNeoFormatterMarker<C> for $type {
            const COMPONENTS: NeoComponents = $components;
            type DateTimeNamesMarker = $namemarker;

            type YearNamesV1Marker = datetime_marker_helper!(@years/typed, $years_yesno);
            type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, $months_yesno);
            type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, $dates_yesno);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $weekdays_yesno);
            type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, $dayperiods_yesno);
            type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, $times_yesno);
            type DateTimePatternV1Marker = datetime_marker_helper!(@datetimes, $datetimes_yesno);
            type DateInputMarker = datetime_marker_helper!(@dateinput, $dateinput_yesno);
            type WeekInputMarker = datetime_marker_helper!(@weekinput, $weekinput_yesno);
            type TimeInputMarker = datetime_marker_helper!(@timeinput, $timeinput_yesno);
        }
        impl NeoFormatterMarker for $type {
            const COMPONENTS: NeoComponents = $components;
            type DateTimeNamesMarker = $namemarker;

            type Year = datetime_marker_helper!(@calmarkers, $years_yesno);
            type Month = datetime_marker_helper!(@calmarkers, $months_yesno);
            type Skel = datetime_marker_helper!(@calmarkers, $dates_yesno);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $weekdays_yesno);
            type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, $dayperiods_yesno);
            type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, $times_yesno);
            type DateTimePatternV1Marker = datetime_marker_helper!(@datetimes, $datetimes_yesno);
        }
    };
}

impl_datetime_marker!(
    NeoYearMonthDayMarker,
    { NeoComponents::Date(NeoDateComponents::Day(NeoDayComponents::YearMonthDay)) },
    description = "a Year/Month/Day format",
    expectation = "May 17, 2024",
    names = DateMarker,
    years = yes,
    months = yes,
    dates = yes,
    weekdays = yes,
    dayperiods = no,
    times = no,
    datetimes = no,
    dateinput = yes,
    weekinput = no,
    timeinput = no,
);

impl_datetime_marker!(
    NeoEraYearMonthMarker,
    { NeoComponents::Date(NeoDateComponents::Day(NeoDayComponents::EraYearMonthDay)) },
    description = "an Era/Year/Month/Day format",
    expectation = "May 17, 2024 AD",
    names = DateMarker,
    years = yes,
    months = yes,
    dates = yes,
    weekdays = no,
    dayperiods = no,
    times = no,
    datetimes = no,
    dateinput = yes,
    weekinput = no,
    timeinput = no,
);

impl_datetime_marker!(
    NeoAutoDateMarker,
    { NeoComponents::Date(NeoDateComponents::Day(NeoDayComponents::Auto)) },
    description = "locale-dependent date fields",
    expectation = "May 17, 2024",
    names = DateMarker,
    years = yes,
    months = yes,
    dates = yes,
    weekdays = yes,
    dayperiods = no,
    times = no,
    datetimes = no,
    dateinput = yes,
    weekinput = no,
    timeinput = no,
);

impl_datetime_marker!(
    NeoAutoTimeMarker,
    { NeoComponents::Time(NeoTimeComponents::Auto) },
    description = "locale-dependent time fields",
    expectation = "3:47:50 PM",
    names = TimeMarker,
    years = no,
    months = no,
    dates = no,
    weekdays = no,
    dayperiods = yes,
    times = yes,
    datetimes = no,
    dateinput = no,
    weekinput = no,
    timeinput = yes,
);

impl_datetime_marker!(
    NeoAutoDateTimeMarker,
    { NeoComponents::DateTime(NeoDayComponents::Auto, NeoTimeComponents::Auto) },
    description = "locale-dependent date and time fields",
    expectation = "May 17, 2024, 3:47:50 PM",
    names = DateTimeMarker,
    years = yes,
    months = yes,
    dates = yes,
    weekdays = yes,
    dayperiods = yes,
    times = yes,
    datetimes = yes,
    dateinput = yes,
    weekinput = no,
    timeinput = yes,
);

impl_datetime_marker!(
    NeoYearMonthMarker,
    { NeoComponents::Date(NeoDateComponents::YearMonth) },
    description = "a Year/Month format",
    expectation = "May 2024",
    names = DateMarker,
    years = yes,
    months = yes,
    dates = yes,
    weekdays = no,
    dayperiods = no,
    times = no,
    datetimes = no,
    dateinput = yes,
    weekinput = no,
    timeinput = no,
);
