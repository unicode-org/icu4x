// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::{dynamic::*, format::neo::DateTimeNamesMarker, neo_skeleton::NeoComponents};
use crate::{
    format::neo::*,
    neo_skeleton::*,
    provider::{neo::*, time_zones::tz, *},
    raw::neo::RawNeoOptions,
    scaffold::*,
};
use icu_calendar::{
    types::{
        DayOfMonth, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo, NanoSecond, YearInfo,
    },
    AnyCalendarKind, Date, Iso, Time,
};
use icu_provider::marker::NeverMarker;
use icu_timezone::{TimeZoneBcp47Id, UtcOffset, ZoneVariant};

// impl GetField<NeoComponents> for DateFieldSet {
//     fn get_field(&self) -> NeoComponents {
//         self.components.into()
//     }
// }

impl UnstableSealed for DateFieldSet {}

// impl IsRuntimeComponents for DateFieldSet {}

impl DateTimeNamesMarker for DateFieldSet {
    type YearNames = datetime_marker_helper!(@names/year, yes);
    type MonthNames = datetime_marker_helper!(@names/month, yes);
    type WeekdayNames = datetime_marker_helper!(@names/weekday, yes);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
    type ZoneLocations = datetime_marker_helper!(@names/zone/locations,);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
    type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods,);
}

impl DateInputMarkers for DateFieldSet {
    type YearInput = datetime_marker_helper!(@input/year, yes);
    type MonthInput = datetime_marker_helper!(@input/month, yes);
    type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month, yes);
    type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week, yes);
    type AnyCalendarKindInput = datetime_marker_helper!(@input/any_calendar_kind, yes);
}

impl<C: CldrCalendar> TypedDateDataMarkers<C> for DateFieldSet {
    type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, yes);
    type YearNamesV1Marker = datetime_marker_helper!(@years/typed, yes);
    type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, yes);
}

impl DateDataMarkers for DateFieldSet {
    type Skel = datetime_marker_helper!(@calmarkers, yes);
    type Year = datetime_marker_helper!(@calmarkers, yes);
    type Month = datetime_marker_helper!(@calmarkers, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, yes);
}

impl DateTimeMarkers for DateFieldSet {
    type D = Self;
    type T = NeoNeverMarker;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle, yes);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

// impl_get_field!(DateFieldSet, never);
// impl_get_field!(DateFieldSet, length, yes);
// impl_get_field!(DateFieldSet, alignment, yes);
// impl_get_field!(DateFieldSet, year_style, yes);
