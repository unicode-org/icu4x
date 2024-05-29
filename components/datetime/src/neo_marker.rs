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
    AnyCalendarKind, AsCalendar, Calendar, Date, DateTime, Time,
};
use icu_provider::{prelude::*, NeverMarker};

mod private {
    pub trait Sealed {}
}

/// Input data fields required for formatting dates.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoDateInputFields<C> {
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
    /// The phantom calendar marker.
    _calendar: PhantomData<C>,
}

impl<C> NeoDateInputFields<C> {
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
            _calendar: PhantomData,
        }
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> From<&Date<A>> for NeoDateInputFields<C> {
    fn from(value: &Date<A>) -> Self {
        Self {
            year: value.year(),
            month: value.month(),
            day_of_month: value.day_of_month(),
            day_of_week: value.day_of_week(),
            any_calendar_kind: value.calendar().any_calendar_kind(),
            _calendar: PhantomData,
        }
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> From<&DateTime<A>> for NeoDateInputFields<C> {
    fn from(value: &DateTime<A>) -> Self {
        Self {
            year: value.date.year(),
            month: value.date.month(),
            day_of_month: value.date.day_of_month(),
            day_of_week: value.date.day_of_week(),
            any_calendar_kind: value.date.calendar().any_calendar_kind(),
            _calendar: PhantomData,
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

impl From<&Time> for NeoTimeInputFields {
    fn from(value: &Time) -> Self {
        Self {
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            nanosecond: value.nanosecond,
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

impl<C> From<NeverFields> for Option<NeoDateInputFields<C>> {
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

/// A trait associating [`NeoDateComponents`].
pub trait HasDateComponents {
    /// The associated components.
    const COMPONENTS: NeoDateComponents;
}

/// A trait associating [`HasDayComponents`].
pub trait HasDayComponents {
    /// The associated components.
    const COMPONENTS: NeoDayComponents;
}

/// A trait associating [`NeoTimeComponents`].
pub trait HasTimeComponents {
    /// The associated components.
    const COMPONENTS: NeoTimeComponents;
}

// TODO: Add WeekCalculator and FixedDecimalFormatter optional bindings here

/// A trait associating types implementing various other traits
/// required for date formatting.
pub trait DateMarkers<C>: private::Sealed {
    /// Marker for loading date skeleton patterns.
    type DateSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for resolving date fields from the input.
    type DateInputMarker: Into<Option<NeoDateInputFields<C>>>;
    /// Marker for resolving week-of-year fields from the input.
    type WeekInputMarker: Into<Option<NeoWeekInputFields>>;
    /// Marker for loading year names.
    type YearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;
    /// Marker for loading month names.
    type MonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
}

/// A trait associating types implementing various other traits
/// required for time formatting.
pub trait TimeMarkers: private::Sealed {
    /// Marker for resolving time fields from the input.
    type TimeInputMarker: Into<Option<NeoTimeInputFields>>;
    /// Marker for loading time skeleton patterns.
    type TimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading day period names.
    type DayPeriodNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
}

/// A struct implementing traits for never loading data.
#[derive(Debug)]
pub struct NeoNeverMarker;

impl private::Sealed for NeoNeverMarker {}

impl<C> DateMarkers<C> for NeoNeverMarker {
    type DateSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateInputMarker = NeverFields;
    type WeekInputMarker = NeverFields;
    type YearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type MonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

impl TimeMarkers for NeoNeverMarker {
    type TimeInputMarker = NeverFields;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

/// A trait associating constants and types implementing various other traits
/// required for datetime formatting.
pub trait HasNeoComponents<C>: private::Sealed {
    /// The associated components.
    const COMPONENTS: NeoComponents;
    /// Associated types for date formatting.
    type D: DateMarkers<C>;
    /// Associated types for time formatting.
    type T: TimeMarkers;
    /// Fields for [`TypedDateTimeNames`].
    type DateTimeNamesMarker: DateTimeNamesMarker;
    /// Marker for loading the date/time glue pattern.
    type DateTimePatternV1Marker: KeyedDataMarker<Yokeable = DateTimePatternV1<'static>>;
}

/// A struct that supports formatting both a date and a time.
///
/// It should be composed from types implementing [`HasDayComponents`]
/// and [`HasTimeComponents`].
#[derive(Debug)]
pub struct DateTimeCombo<D, T> {
    _d: PhantomData<D>,
    _t: PhantomData<T>,
}

impl<D, T> private::Sealed for DateTimeCombo<D, T> {}

// type MonthDayHour = DateTimeCombo<MonthDay, Hour>;

impl<C, D> HasNeoComponents<C> for DateTimeCombo<D, NeoNeverMarker>
where
    D: HasDateComponents + DateMarkers<C>,
{
    const COMPONENTS: NeoComponents = NeoComponents::Date(D::COMPONENTS);
    type D = D;
    type T = NeoNeverMarker;
    type DateTimeNamesMarker = DateMarker;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<C, T> HasNeoComponents<C> for DateTimeCombo<NeoNeverMarker, T>
where
    T: HasTimeComponents + TimeMarkers,
{
    const COMPONENTS: NeoComponents = NeoComponents::Time(T::COMPONENTS);
    type D = NeoNeverMarker;
    type T = T;
    type DateTimeNamesMarker = TimeMarker;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<C, D, T> HasNeoComponents<C> for DateTimeCombo<D, T>
where
    D: HasDayComponents + DateMarkers<C>,
    T: HasTimeComponents + TimeMarkers,
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
    type DateInputMarker: Into<Option<NeoDateInputFields<C>>>;
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
        NeoDateInputFields<C>
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

macro_rules! impl_date_marker {
    (
        $type:ident,
        $components:expr,
        description = $description:literal,
        expectation = $expectation:literal,
        years = $years_yesno:ident,
        months = $months_yesno:ident,
        dates = $dates_yesno:ident,
        weekdays = $weekdays_yesno:ident,
        dateinput = $dateinput_yesno:ident,
        weekinput = $weekinput_yesno:ident,
    ) => {
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
        /// use icu::calendar::Date;
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
        /// let dt = Date::try_new_gregorian_date(2024, 5, 17).unwrap();
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
        impl HasDateComponents for $type {
            const COMPONENTS: NeoDateComponents = $components;
        }
        impl<C: CldrCalendar> DateMarkers<C> for $type {
            type YearNamesV1Marker = datetime_marker_helper!(@years/typed, $years_yesno);
            type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, $months_yesno);
            type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, $dates_yesno);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $weekdays_yesno);
            type DateInputMarker = datetime_marker_helper!(@dateinput, $dateinput_yesno);
            type WeekInputMarker = datetime_marker_helper!(@weekinput, $weekinput_yesno);
        }
        impl<C: CldrCalendar> HasNeoComponents<C> for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Date($components);
            type D = Self;
            type T = NeoNeverMarker;
            type DateTimeNamesMarker = DateMarker;
            type DateTimePatternV1Marker = datetime_marker_helper!(@datetimes, no);
        }
        impl NeoFormatterMarker for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Date($components);
            type DateTimeNamesMarker = DateMarker;

            type Year = datetime_marker_helper!(@calmarkers, $years_yesno);
            type Month = datetime_marker_helper!(@calmarkers, $months_yesno);
            type Skel = datetime_marker_helper!(@calmarkers, $dates_yesno);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $weekdays_yesno);

            type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, no);
            type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, no);
            type DateTimePatternV1Marker = datetime_marker_helper!(@datetimes, no);
        }
    };
}

macro_rules! impl_day_marker {
    (
        $type:ident,
        $components:expr,
        description = $description:literal,
        expectation = $expectation:literal,
        years = $years_yesno:ident,
        months = $months_yesno:ident,
        dates = $dates_yesno:ident,
        weekdays = $weekdays_yesno:ident,
        dateinput = $dateinput_yesno:ident,
        weekinput = $weekinput_yesno:ident,
    ) => {
        impl_date_marker!(
            $type,
            NeoDateComponents::Day($components),
            description = $description,
            expectation = $expectation,
            years = $years_yesno,
            months = $months_yesno,
            dates = $dates_yesno,
            weekdays = $weekdays_yesno,
            dateinput = $dateinput_yesno,
            weekinput = $weekinput_yesno,
        );
        impl HasDayComponents for $type {
            const COMPONENTS: NeoDayComponents = $components;
        }
    }
}

macro_rules! impl_time_marker {
    (
        $type:ident,
        $components:expr,
        description = $description:literal,
        expectation = $expectation:literal,
        dayperiods = $dayperiods_yesno:ident,
        times = $times_yesno:ident,
        timeinput = $timeinput_yesno:ident,
    ) => {
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
        /// use icu::calendar::Time;
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
        /// let dt = Time::try_new(15, 47, 50, 0).unwrap();
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
        impl HasTimeComponents for $type {
            const COMPONENTS: NeoTimeComponents = $components;
        }
        impl TimeMarkers for $type {
            type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, $dayperiods_yesno);
            type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, $times_yesno);
            type TimeInputMarker = datetime_marker_helper!(@timeinput, $timeinput_yesno);
        }
        impl<C> HasNeoComponents<C> for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Time($components);
            type D = NeoNeverMarker;
            type T = Self;
            type DateTimeNamesMarker = TimeMarker;
            type DateTimePatternV1Marker = datetime_marker_helper!(@datetimes, no);
        }
        impl NeoFormatterMarker for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Time($components);
            type DateTimeNamesMarker = TimeMarker;

            type Year = datetime_marker_helper!(@calmarkers, no);
            type Month = datetime_marker_helper!(@calmarkers, no);
            type Skel = datetime_marker_helper!(@calmarkers, no);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, no);
            type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, $dayperiods_yesno);
            type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, $times_yesno);
            type DateTimePatternV1Marker = datetime_marker_helper!(@datetimes, no);
        }
    };
}

macro_rules! impl_datetime_marker {
    (
        $type:ident,
        description = $description:literal,
        expectation = $expectation:literal,
        date = $date:path,
        time = $time:path,
    ) => {
        #[doc = concat!("Marker for ", $description, ": ", $expectation)]
        ///
        /// # Examples
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
        pub type NeoAutoDateTimeMarker = DateTimeCombo<$date, $time>;
    }
}

impl_day_marker!(
    NeoYearMonthDayMarker,
    NeoDayComponents::YearMonthDay,
    description = "a Year/Month/Day format",
    expectation = "May 17, 2024",
    years = yes,
    months = yes,
    dates = yes,
    weekdays = yes,
    dateinput = yes,
    weekinput = no,
);

impl_day_marker!(
    NeoEraYearMonthMarker,
    NeoDayComponents::EraYearMonthDay,
    description = "an Era/Year/Month/Day format",
    expectation = "May 17, 2024 AD",
    years = yes,
    months = yes,
    dates = yes,
    weekdays = no,
    dateinput = yes,
    weekinput = no,
);

impl_day_marker!(
    NeoAutoDateMarker,
    NeoDayComponents::Auto,
    description = "locale-dependent date fields",
    expectation = "May 17, 2024",
    years = yes,
    months = yes,
    dates = yes,
    weekdays = yes,
    dateinput = yes,
    weekinput = no,
);

impl_time_marker!(
    NeoAutoTimeMarker,
    NeoTimeComponents::Auto,
    description = "locale-dependent time fields",
    expectation = "3:47:50 PM",
    dayperiods = yes,
    times = yes,
    timeinput = yes,
);

impl_datetime_marker!(
    NeoAutoDateTimeMarker,
    description = "locale-dependent date and time fields",
    expectation = "May 17, 2024, 3:47:50 PM",
    date = NeoAutoDateMarker,
    time = NeoAutoTimeMarker,
);

impl_date_marker!(
    NeoYearMonthMarker,
    NeoDateComponents::YearMonth,
    description = "a Year/Month format",
    expectation = "May 2024",
    years = yes,
    months = yes,
    dates = yes,
    weekdays = no,
    dateinput = yes,
    weekinput = no,
);
