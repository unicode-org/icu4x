// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo formatter markers.

use crate::{format::neo::*, neo::_internal::*, neo_skeleton::*, provider::neo::*, CldrCalendar};
use icu_provider::{prelude::*, NeverMarker};

mod private {
    pub trait Sealed {}
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
}

macro_rules! impl_datetime_marker {
    ($type:ident, $components:expr, description = $description:literal, expectation = $expectation:literal, names = $namemarker:path, years = $years_yesno:ident, months = $months_yesno:ident, dates = $dates_yesno:ident, weekdays = $weekdays_yesno:ident, dayperiods = $dayperiods_yesno:ident, times = $times_yesno:ident, datetimes = $datetimes_yesno:ident) => {
        #[doc = concat!("Marker for ", $description, ": ", $expectation)]
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::calendar::DateTime;
        /// use icu::datetime::neo::NeoFormatter;
        #[doc = concat!("use icu::datetime::neo_marker::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locid::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
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
        #[derive(Debug)]
        #[allow(clippy::exhaustive_enums)] // empty enum
        pub enum $type {}
        impl private::Sealed for $type {}
        impl<C: CldrCalendar> TypedNeoFormatterMarker<C> for $type {
            const COMPONENTS: NeoComponents = $components;
            type DateTimeNamesMarker = $namemarker;

            type YearNamesV1Marker = datetime_marker_helper!(@years/typed, $years_yesno);
            type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, $months_yesno);
            type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, $dates_yesno);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $weekdays_yesno);
            type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, $dayperiods_yesno);
            type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, $times_yesno);
            type DateTimePatternV1Marker = datetime_marker_helper!(@datetimes, $datetimes_yesno);
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
    datetimes = no
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
    datetimes = no
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
    datetimes = no
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
    datetimes = no
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
    datetimes = yes
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
    datetimes = no
);
