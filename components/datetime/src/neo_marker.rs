// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo formatter markers.

use core::marker::PhantomData;

use crate::{format::neo::*, neo::_internal::*, neo_skeleton::*, provider::neo::*, CldrCalendar};
use icu_calendar::{
    any_calendar::IntoAnyCalendar,
    types::{
        DayOfMonth, DayOfYearInfo, FormattableMonth, FormattableYear, IsoHour, IsoMinute,
        IsoSecond, IsoWeekday, NanoSecond,
    },
    AnyCalendar, AnyCalendarKind, AsCalendar, Calendar, Date, DateTime, Ref, Time,
};
use icu_provider::{prelude::*, NeverMarker};

mod private {
    pub trait Sealed {}
}

/// A type that can be converted into a specific calendar system.
pub trait ConvertCalendar: Sized {
    /// The converted type. This can be the same as the receiver type.
    type Converted<'a>;
    /// Converts `self` to the specified [`AnyCalendar`].
    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a>;
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> ConvertCalendar for Date<A> {
    type Converted<'a> = Date<Ref<'a, AnyCalendar>>;
    #[inline]
    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        self.to_any().to_calendar(Ref(calendar))
    }
}

impl ConvertCalendar for Time {
    type Converted<'a> = Time;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> ConvertCalendar for DateTime<A> {
    type Converted<'a> = DateTime<Ref<'a, AnyCalendar>>;
    #[inline]
    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        self.to_any().to_calendar(Ref(calendar))
    }
}

/// A type that might be compatible with a specific calendar system.
///
/// All formattable types should implement this trait.
pub trait IsAnyCalendarKind {
    /// Whether this type is compatible with the given calendar.
    ///
    /// Types that are agnostic to calendar systems should return `true`.
    fn is_any_calendar_kind(&self, any_calendar_kind: AnyCalendarKind) -> bool;
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> IsAnyCalendarKind for Date<A> {
    fn is_any_calendar_kind(&self, any_calendar_kind: AnyCalendarKind) -> bool {
        self.calendar().any_calendar_kind() == Some(any_calendar_kind)
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> IsAnyCalendarKind for DateTime<A> {
    fn is_any_calendar_kind(&self, any_calendar_kind: AnyCalendarKind) -> bool {
        self.date.calendar().any_calendar_kind() == Some(any_calendar_kind)
    }
}

impl IsAnyCalendarKind for Time {
    fn is_any_calendar_kind(&self, _: AnyCalendarKind) -> bool {
        true
    }
}

/// An input associated with a specific calendar.
#[derive(Debug, Copy, Clone)]
pub struct NeoTypedInput<C> {
    /// The phantom calendar marker.
    _calendar: PhantomData<C>,
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<NeoTypedInput<C>> for Date<A> {
    #[inline]
    fn get_field(&self) -> NeoTypedInput<C> {
        NeoTypedInput {
            _calendar: PhantomData,
        }
    }
}

impl<C> NeoGetField<NeoTypedInput<C>> for Time {
    #[inline]
    fn get_field(&self) -> NeoTypedInput<C> {
        NeoTypedInput {
            _calendar: PhantomData,
        }
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<NeoTypedInput<C>> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> NeoTypedInput<C> {
        NeoTypedInput {
            _calendar: PhantomData,
        }
    }
}

/// A type that can return a certain field `T`.
pub trait NeoGetField<T> {
    /// Returns the value of this trait's field `T`.
    fn get_field(&self) -> T;
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<FormattableYear> for Date<A> {
    #[inline]
    fn get_field(&self) -> FormattableYear {
        self.year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<FormattableMonth> for Date<A> {
    #[inline]
    fn get_field(&self) -> FormattableMonth {
        self.month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<DayOfMonth> for Date<A> {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.day_of_month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<IsoWeekday> for Date<A> {
    #[inline]
    fn get_field(&self) -> IsoWeekday {
        self.day_of_week()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<DayOfYearInfo> for Date<A> {
    #[inline]
    fn get_field(&self) -> DayOfYearInfo {
        self.day_of_year_info()
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> NeoGetField<AnyCalendarKind> for Date<A> {
    #[inline]
    fn get_field(&self) -> AnyCalendarKind {
        self.calendar().kind()
    }
}

impl NeoGetField<IsoHour> for Time {
    #[inline]
    fn get_field(&self) -> IsoHour {
        self.hour
    }
}

impl NeoGetField<IsoMinute> for Time {
    #[inline]
    fn get_field(&self) -> IsoMinute {
        self.minute
    }
}

impl NeoGetField<IsoSecond> for Time {
    #[inline]
    fn get_field(&self) -> IsoSecond {
        self.second
    }
}

impl NeoGetField<NanoSecond> for Time {
    #[inline]
    fn get_field(&self) -> NanoSecond {
        self.nanosecond
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<FormattableYear> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> FormattableYear {
        self.date.year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<FormattableMonth> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> FormattableMonth {
        self.date.month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<DayOfMonth> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date.day_of_month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<IsoWeekday> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoWeekday {
        self.date.day_of_week()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<DayOfYearInfo> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> DayOfYearInfo {
        self.date.day_of_year_info()
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> NeoGetField<AnyCalendarKind> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> AnyCalendarKind {
        self.date.calendar().kind()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<IsoHour> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoHour {
        self.time.hour
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<IsoMinute> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoMinute {
        self.time.minute
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<IsoSecond> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoSecond {
        self.time.second
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<NanoSecond> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> NanoSecond {
        self.time.nanosecond
    }
}

/// Struct representing the absence of a datetime formatting field.
#[derive(Debug, Copy, Clone)]
#[allow(clippy::exhaustive_structs)] // empty marker struct
pub struct NeverField;

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<NeverField> for Date<A> {
    #[inline]
    fn get_field(&self) -> NeverField {
        NeverField
    }
}

impl NeoGetField<NeverField> for Time {
    #[inline]
    fn get_field(&self) -> NeverField {
        NeverField
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<NeverField> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> NeverField {
        NeverField
    }
}

impl<C> From<NeverField> for Option<NeoTypedInput<C>> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<FormattableYear> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<FormattableMonth> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<DayOfMonth> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<IsoWeekday> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<DayOfYearInfo> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<AnyCalendarKind> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<IsoHour> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<IsoMinute> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<IsoSecond> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<NanoSecond> {
    #[inline]
    fn from(_: NeverField) -> Self {
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
/// required for date formatting in a specific calendar.
pub trait TypedDateMarkers<C>: private::Sealed {
    /// Marker for loading date skeleton patterns.
    type DateSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for associating input with the specific calendar.
    type TypedInputMarker: Into<Option<NeoTypedInput<C>>>;
    /// Marker for resolving the year input field.
    type YearInput: Into<Option<FormattableYear>>;
    /// Marker for resolving the month input field.
    type MonthInput: Into<Option<FormattableMonth>>;
    /// Marker for resolving the day-of-month input field.
    type DayOfMonthInput: Into<Option<DayOfMonth>>;
    /// Marker for resolving the day-of-week input field.
    type DayOfWeekInput: Into<Option<IsoWeekday>>;
    /// Marker for resolving the day-of-year input field.
    type DayOfYearInput: Into<Option<DayOfYearInfo>>;
    /// Marker for resolving the any-calendar-kind input field.
    type AnyCalendarKindInput: Into<Option<AnyCalendarKind>>;
    /// Marker for loading year names.
    type YearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;
    /// Marker for loading month names.
    type MonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
}

/// A trait associating types implementing various other traits
/// required for date formatting in any calendar.
pub trait DateMarkers: private::Sealed {
    /// Cross-calendar data markers for date skeleta.
    type Skel: CalMarkers<SkeletaV1Marker>;
    /// Marker for resolving the year input field.
    type YearInput: Into<Option<FormattableYear>>;
    /// Marker for resolving the month input field.
    type MonthInput: Into<Option<FormattableMonth>>;
    /// Marker for resolving the day-of-month input field.
    type DayOfMonthInput: Into<Option<DayOfMonth>>;
    /// Marker for resolving the day-of-week input field.
    type DayOfWeekInput: Into<Option<IsoWeekday>>;
    /// Marker for resolving the day-of-year input field.
    type DayOfYearInput: Into<Option<DayOfYearInfo>>;
    /// Marker for resolving the any-calendar-kind input field.
    type AnyCalendarKindInput: Into<Option<AnyCalendarKind>>;
    /// Cross-calendar data markers for year names.
    type Year: CalMarkers<YearNamesV1Marker>;
    /// Cross-calendar data markers for month names.
    type Month: CalMarkers<MonthNamesV1Marker>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
}

/// A trait associating types implementing various other traits
/// required for time formatting.
pub trait TimeMarkers: private::Sealed {
    /// Marker for resolving the day-of-month input field.
    type HourInput: Into<Option<IsoHour>>;
    /// Marker for resolving the day-of-week input field.
    type MinuteInput: Into<Option<IsoMinute>>;
    /// Marker for resolving the day-of-year input field.
    type SecondInput: Into<Option<IsoSecond>>;
    /// Marker for resolving the any-calendar-kind input field.
    type NanoSecondInput: Into<Option<NanoSecond>>;
    /// Marker for loading time skeleton patterns.
    type TimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading day period names.
    type DayPeriodNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
}

/// A struct implementing traits for never loading data.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty marker enum
pub enum NeoNeverMarker {}

impl private::Sealed for NeoNeverMarker {}

impl<C> TypedDateMarkers<C> for NeoNeverMarker {
    type DateSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type TypedInputMarker = NeverField;
    type YearInput = NeverField;
    type MonthInput = NeverField;
    type DayOfMonthInput = NeverField;
    type DayOfWeekInput = NeverField;
    type DayOfYearInput = NeverField;
    type AnyCalendarKindInput = NeverField;
    type YearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type MonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

impl DateMarkers for NeoNeverMarker {
    type Skel = NoDataCalMarkers;
    type YearInput = NeverField;
    type MonthInput = NeverField;
    type DayOfMonthInput = NeverField;
    type DayOfWeekInput = NeverField;
    type DayOfYearInput = NeverField;
    type AnyCalendarKindInput = NeverField;
    type Year = NoDataCalMarkers;
    type Month = NoDataCalMarkers;
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

impl TimeMarkers for NeoNeverMarker {
    type HourInput = NeverField;
    type MinuteInput = NeverField;
    type SecondInput = NeverField;
    type NanoSecondInput = NeverField;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

/// A trait associating constants and types implementing various other traits
/// required for datetime formatting in a specific calendar.
pub trait TypedNeoFormatterMarker<C>: private::Sealed {
    /// The associated components.
    const COMPONENTS: NeoComponents;
    /// Associated types for date formatting.
    type D: TypedDateMarkers<C>;
    /// Associated types for time formatting.
    type T: TimeMarkers;
    /// Fields for [`TypedDateTimeNames`].
    type DateTimeNamesMarker: DateTimeNamesMarker;
    /// Marker for loading the date/time glue pattern.
    type DateTimePatternV1Marker: KeyedDataMarker<Yokeable = DateTimePatternV1<'static>>;
}

/// A trait associating constants and types implementing various other traits
/// required for datetime formatting in any calendar.
pub trait NeoFormatterMarker: private::Sealed {
    /// The associated components.
    const COMPONENTS: NeoComponents;
    /// Associated types for date formatting.
    type D: DateMarkers;
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

impl<C, D> TypedNeoFormatterMarker<C> for DateTimeCombo<D, NeoNeverMarker>
where
    D: HasDateComponents + TypedDateMarkers<C>,
{
    const COMPONENTS: NeoComponents = NeoComponents::Date(D::COMPONENTS);
    type D = D;
    type T = NeoNeverMarker;
    type DateTimeNamesMarker = DateMarker;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<D> NeoFormatterMarker for DateTimeCombo<D, NeoNeverMarker>
where
    D: HasDateComponents + DateMarkers,
{
    const COMPONENTS: NeoComponents = NeoComponents::Date(D::COMPONENTS);
    type D = D;
    type T = NeoNeverMarker;
    type DateTimeNamesMarker = DateMarker;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<C, T> TypedNeoFormatterMarker<C> for DateTimeCombo<NeoNeverMarker, T>
where
    T: HasTimeComponents + TimeMarkers,
{
    const COMPONENTS: NeoComponents = NeoComponents::Time(T::COMPONENTS);
    type D = NeoNeverMarker;
    type T = T;
    type DateTimeNamesMarker = TimeMarker;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<T> NeoFormatterMarker for DateTimeCombo<NeoNeverMarker, T>
where
    T: HasTimeComponents + TimeMarkers,
{
    const COMPONENTS: NeoComponents = NeoComponents::Time(T::COMPONENTS);
    type D = NeoNeverMarker;
    type T = T;
    type DateTimeNamesMarker = TimeMarker;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl<C, D, T> TypedNeoFormatterMarker<C> for DateTimeCombo<D, T>
where
    D: HasDayComponents + TypedDateMarkers<C>,
    T: HasTimeComponents + TimeMarkers,
{
    const COMPONENTS: NeoComponents = NeoComponents::DateTime(D::COMPONENTS, T::COMPONENTS);
    type D = D;
    type T = T;
    type DateTimeNamesMarker = DateTimeMarker;
    type DateTimePatternV1Marker = DateTimePatternV1Marker;
}

impl<D, T> NeoFormatterMarker for DateTimeCombo<D, T>
where
    D: HasDayComponents + DateMarkers,
    T: HasTimeComponents + TimeMarkers,
{
    const COMPONENTS: NeoComponents = NeoComponents::DateTime(D::COMPONENTS, T::COMPONENTS);
    type D = D;
    type T = T;
    type DateTimeNamesMarker = DateTimeMarker;
    type DateTimePatternV1Marker = DateTimePatternV1Marker;
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
    (@input/typed, yes) => {
        NeoTypedInput<C>
    };
    (@input/year, yes) => {
        FormattableYear
    };
    (@input/month, yes) => {
        FormattableMonth
    };
    (@input/day_of_month, yes) => {
        DayOfMonth
    };
    (@input/day_of_week, yes) => {
        IsoWeekday
    };
    (@input/day_of_year, yes) => {
        DayOfYearInfo
    };
    (@input/any_calendar_kind, yes) => {
        AnyCalendarKind
    };
    (@input/hour, yes) => {
        IsoHour
    };
    (@input/minute, yes) => {
        IsoMinute
    };
    (@input/second, yes) => {
        IsoSecond
    };
    (@input/nanosecond, yes) => {
        NanoSecond
    };
    (@input/$any:ident, no) => {
        NeverField
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
        input_year = $year_yesno:ident,
        input_month = $month_yesno:ident,
        input_day_of_month = $day_of_month_yesno:ident,
        input_day_of_week = $day_of_week_yesno:ident,
        input_day_of_year = $day_of_year_yesno:ident,
        input_any_calendar_kind = $any_calendar_kind_yesno:ident,
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
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        #[doc = concat!("let fmt = NeoFormatter::<", stringify!($type), ">::try_new(")]
        ///     &locale!("en"),
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
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = TypedNeoFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
        ///     &locale!("en"),
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
        impl<C: CldrCalendar> TypedDateMarkers<C> for $type {
            type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, $dates_yesno);
            type TypedInputMarker = datetime_marker_helper!(@input/typed, yes);
            type YearInput = datetime_marker_helper!(@input/year, $year_yesno);
            type MonthInput = datetime_marker_helper!(@input/month, $month_yesno);
            type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month, $day_of_month_yesno);
            type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week, $day_of_week_yesno);
            type DayOfYearInput = datetime_marker_helper!(@input/day_of_year, $day_of_year_yesno);
            type AnyCalendarKindInput = datetime_marker_helper!(@input/any_calendar_kind, $any_calendar_kind_yesno);
            type YearNamesV1Marker = datetime_marker_helper!(@years/typed, $years_yesno);
            type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, $months_yesno);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $weekdays_yesno);
        }
        impl DateMarkers for $type {
            type Skel = datetime_marker_helper!(@calmarkers, $dates_yesno);
            type YearInput = datetime_marker_helper!(@input/year, $year_yesno);
            type MonthInput = datetime_marker_helper!(@input/month, $month_yesno);
            type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month, $day_of_month_yesno);
            type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week, $day_of_week_yesno);
            type DayOfYearInput = datetime_marker_helper!(@input/day_of_year, $day_of_year_yesno);
            type AnyCalendarKindInput = datetime_marker_helper!(@input/any_calendar_kind, $any_calendar_kind_yesno);
            type Year = datetime_marker_helper!(@calmarkers, $years_yesno);
            type Month = datetime_marker_helper!(@calmarkers, $months_yesno);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $weekdays_yesno);
        }
        impl<C: CldrCalendar> TypedNeoFormatterMarker<C> for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Date($components);
            type D = Self;
            type T = NeoNeverMarker;
            type DateTimeNamesMarker = DateMarker;
            type DateTimePatternV1Marker = datetime_marker_helper!(@datetimes, no);
        }
        impl NeoFormatterMarker for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Date($components);
            type D = Self;
            type T = NeoNeverMarker;
            type DateTimeNamesMarker = DateMarker;
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
        input_year = $year_yesno:ident,
        input_month = $month_yesno:ident,
        input_day_of_month = $day_of_month_yesno:ident,
        input_day_of_week = $day_of_week_yesno:ident,
        input_day_of_year = $day_of_year_yesno:ident,
        input_any_calendar_kind = $any_calendar_kind_yesno:ident,
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
            input_year = $year_yesno,
            input_month = $month_yesno,
            input_day_of_month = $day_of_month_yesno,
            input_day_of_week = $day_of_week_yesno,
            input_day_of_year = $day_of_year_yesno,
            input_any_calendar_kind = $any_calendar_kind_yesno,
        );
        impl HasDayComponents for $type {
            const COMPONENTS: NeoDayComponents = $components;
        }
    };
}

macro_rules! impl_time_marker {
    (
        $type:ident,
        $components:expr,
        description = $description:literal,
        expectation = $expectation:literal,
        dayperiods = $dayperiods_yesno:ident,
        times = $times_yesno:ident,
        input_hour = $hour_yesno:ident,
        input_minute = $minute_yesno:ident,
        input_second = $second_yesno:ident,
        input_nanosecond = $nanosecond_yesno:ident,
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
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        #[doc = concat!("let fmt = NeoFormatter::<", stringify!($type), ">::try_new(")]
        ///     &locale!("en"),
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
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = TypedNeoFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
        ///     &locale!("en"),
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
            type HourInput = datetime_marker_helper!(@input/hour, $hour_yesno);
            type MinuteInput = datetime_marker_helper!(@input/minute, $minute_yesno);
            type SecondInput = datetime_marker_helper!(@input/second, $second_yesno);
            type NanoSecondInput = datetime_marker_helper!(@input/nanosecond, $nanosecond_yesno);
        }
        impl<C> TypedNeoFormatterMarker<C> for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Time($components);
            type D = NeoNeverMarker;
            type T = Self;
            type DateTimeNamesMarker = TimeMarker;
            type DateTimePatternV1Marker = datetime_marker_helper!(@datetimes, no);
        }
        impl NeoFormatterMarker for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Time($components);
            type D = NeoNeverMarker;
            type T = Self;
            type DateTimeNamesMarker = TimeMarker;
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
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = TypedNeoFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
        ///     &locale!("en"),
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
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_day_of_week = no,
    input_day_of_year = no,
    input_any_calendar_kind = yes,
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
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_day_of_week = no,
    input_day_of_year = no,
    input_any_calendar_kind = yes,
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
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_day_of_week = yes,
    input_day_of_year = no,
    input_any_calendar_kind = yes,
);

impl_time_marker!(
    NeoAutoTimeMarker,
    NeoTimeComponents::Auto,
    description = "locale-dependent time fields",
    expectation = "3:47:50 PM",
    dayperiods = yes,
    times = yes,
    input_hour = yes,
    input_minute = yes,
    input_second = yes,
    input_nanosecond = no,
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
    input_year = yes,
    input_month = yes,
    input_day_of_month = no,
    input_day_of_week = no,
    input_day_of_year = no,
    input_any_calendar_kind = yes,
);
