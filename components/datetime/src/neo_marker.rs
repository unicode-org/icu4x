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
/// [`FullDataCalMarkers`]: _internal::FullDataCalMarkers
/// [`NoDataCalMarkers`]: _internal::NoDataCalMarkers
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

/// Marker for a Year/Month/Day format, like "January 1, 2000"
///
/// # Examples
///
/// ```
/// use icu::calendar::DateTime;
/// use icu::datetime::neo::NeoFormatter;
/// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
/// use icu::datetime::neo_skeleton::NeoSkeletonLength;
/// use icu::locid::locale;
/// use writeable::assert_try_writeable_eq;
///
/// let fmt = NeoFormatter::<NeoYearMonthDayMarker>::try_new(
///     &locale!("en").into(),
///     NeoSkeletonLength::Medium,
/// )
/// .unwrap();
/// let dt = DateTime::try_new_iso_datetime(2024, 5, 17, 15, 47, 50).unwrap();
///
/// assert_try_writeable_eq!(fmt.convert_and_format(&dt), "May 17, 2024");
/// ```
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum NeoYearMonthDayMarker {}

impl private::Sealed for NeoYearMonthDayMarker {}

impl<C: CldrCalendar> TypedNeoFormatterMarker<C> for NeoYearMonthDayMarker {
    const COMPONENTS: NeoComponents =
        NeoComponents::Date(NeoDateComponents::Day(NeoDayComponents::YearMonthDay));
    type DateTimeNamesMarker = DateMarker;

    // Data to include
    type YearNamesV1Marker = C::YearNamesV1Marker;
    type MonthNamesV1Marker = C::MonthNamesV1Marker;
    type DateSkeletonPatternsV1Marker = C::SkeletaV1Marker;
    type WeekdayNamesV1Marker = WeekdayNamesV1Marker;

    // Data to exclude
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl NeoFormatterMarker for NeoYearMonthDayMarker {
    const COMPONENTS: NeoComponents =
        NeoComponents::Date(NeoDateComponents::Day(NeoDayComponents::YearMonthDay));
    type DateTimeNamesMarker = DateMarker;

    // Data to include
    type WeekdayNamesV1Marker = WeekdayNamesV1Marker;
    type Year = FullDataCalMarkers;
    type Month = FullDataCalMarkers;
    type Skel = FullDataCalMarkers;

    // Data to exclude
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}
