// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo datetime skeletons (Semantic Skeleta)

use crate::provider::neo::*;
use crate::CldrCalendar;
use icu_provider::prelude::*;
use icu_provider::NeverMarker;

/// Trait for different types of semantic skeleta.
pub trait NeoSkeleton<C: CldrCalendar> {
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
#[derive(Debug, Clone)]
pub enum Length {
    /// A long date, typically spelled-out, as in “January 1, 2000”.
    Long,
    /// A medium-sized date; typically abbreviated, as in “Jan. 1, 2000”.
    Medium,
    /// A short date; typically numeric, as in “1/1/2000”.
    Short,
}

/// A skeleton for a year and a month, like "March 2024".
#[derive(Debug)]
pub struct YearMonthSkeleton {
    /// The desired formatting length.
    pub length: Length,
}

impl<C> NeoSkeleton<C> for YearMonthSkeleton
where
    C: CldrCalendar,
{
    // Data to include
    type YearNamesV1Marker = C::YearNamesV1Marker;
    type MonthNamesV1Marker = C::MonthNamesV1Marker;
    type DateSkeletonPatternsV1Marker = DateSkeletonPatternsV1Marker;

    // Data to exclude
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
    type DateTimeSkeletonPatternsV1Marker = NeverMarker<DateTimeSkeletonsV1<'static>>;
}
