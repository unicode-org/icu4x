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
        DayOfMonth, DayOfYearInfo, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo,
        NanoSecond, YearInfo,
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
    type DayOfYearInput = datetime_marker_helper!(@input/day_of_year, yes);
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
    type TimePrecisionOption = datetime_marker_helper!(@option/timeprecision,);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

// impl_get_field!(DateFieldSet, never);
// impl_get_field!(DateFieldSet, length, yes);
// impl_get_field!(DateFieldSet, alignment, yes);
// impl_get_field!(DateFieldSet, year_style, yes);

impl UnstableSealed for CalendarPeriodFieldSet {}

// impl GetField<NeoComponents> for CalendarPeriodFieldSet {
//     fn get_field(&self) -> NeoComponents {
//         self.components.into()
//     }
// }

// impl IsRuntimeComponents for CalendarPeriodFieldSet {}

impl DateTimeNamesMarker for CalendarPeriodFieldSet {
    type YearNames = datetime_marker_helper!(@names/year, yes);
    type MonthNames = datetime_marker_helper!(@names/month, yes);
    type WeekdayNames = datetime_marker_helper!(@names/weekday,);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
    type ZoneLocations = datetime_marker_helper!(@names/zone/locations,);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
    type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods,);
}

impl DateInputMarkers for CalendarPeriodFieldSet {
    type YearInput = datetime_marker_helper!(@input/year, yes);
    type MonthInput = datetime_marker_helper!(@input/month, yes);
    type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month,);
    type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week,);
    type DayOfYearInput = datetime_marker_helper!(@input/day_of_year,);
    type AnyCalendarKindInput = datetime_marker_helper!(@input/any_calendar_kind, yes);
}

impl<C: CldrCalendar> TypedDateDataMarkers<C> for CalendarPeriodFieldSet {
    type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, yes);
    type YearNamesV1Marker = datetime_marker_helper!(@years/typed, yes);
    type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays,);
}

impl DateDataMarkers for CalendarPeriodFieldSet {
    type Skel = datetime_marker_helper!(@calmarkers, yes);
    type Year = datetime_marker_helper!(@calmarkers, yes);
    type Month = datetime_marker_helper!(@calmarkers, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays,);
}

impl DateTimeMarkers for CalendarPeriodFieldSet {
    type D = Self;
    type T = NeoNeverMarker;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle, yes);
    type TimePrecisionOption = datetime_marker_helper!(@option/fractionalsecondigits,);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

// impl_get_field!(CalendarPeriodFieldSet, never);
// impl_get_field!(CalendarPeriodFieldSet, length, yes);
// impl_get_field!(CalendarPeriodFieldSet, alignment, yes);
// impl_get_field!(CalendarPeriodFieldSet, year_style, yes);

impl UnstableSealed for TimeFieldSet {}

// impl GetField<NeoComponents> for TimeFieldSet {
//     fn get_field(&self) -> NeoComponents {
//         self.components.into()
//     }
// }

// impl IsRuntimeComponents for TimeFieldSet {}

impl DateTimeNamesMarker for TimeFieldSet {
    type YearNames = datetime_marker_helper!(@names/year,);
    type MonthNames = datetime_marker_helper!(@names/month,);
    type WeekdayNames = datetime_marker_helper!(@names/weekday,);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, yes);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
    type ZoneLocations = datetime_marker_helper!(@names/zone/locations,);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
    type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods,);
}

impl TimeMarkers for TimeFieldSet {
    type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, yes);
    type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, yes);
    type HourInput = datetime_marker_helper!(@input/hour, yes);
    type MinuteInput = datetime_marker_helper!(@input/minute, yes);
    type SecondInput = datetime_marker_helper!(@input/second, yes);
    type NanoSecondInput = datetime_marker_helper!(@input/nanosecond, yes);
}

impl DateTimeMarkers for TimeFieldSet {
    type D = NeoNeverMarker;
    type T = Self;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle,);
    type TimePrecisionOption = datetime_marker_helper!(@option/timeprecision, yes);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

// impl_get_field!(TimeFieldSet, never);
// impl_get_field!(TimeFieldSet, length, yes);
// impl_get_field!(TimeFieldSet, alignment, yes);
// impl_get_field!(TimeFieldSet, time_precision, yes);

impl UnstableSealed for TimeZoneStyleWithLength {}

// impl GetField<NeoComponents> for TimeZoneStyleWithLength {
//     fn get_field(&self) -> NeoComponents {
//         self.style.into()
//     }
// }

// impl IsRuntimeComponents for TimeZoneStyleWithLength {}

impl DateTimeNamesMarker for TimeZoneStyleWithLength {
    type YearNames = datetime_marker_helper!(@names/year,);
    type MonthNames = datetime_marker_helper!(@names/month,);
    type WeekdayNames = datetime_marker_helper!(@names/weekday,);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials, yes);
    type ZoneLocations = datetime_marker_helper!(@names/zone/locations, yes);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long, yes);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short, yes);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long, yes);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short, yes);
    type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods, yes);
}

impl ZoneMarkers for TimeZoneStyleWithLength {
    type TimeZoneIdInput = datetime_marker_helper!(@input/timezone/id, yes);
    type TimeZoneOffsetInput = datetime_marker_helper!(@input/timezone/offset, yes);
    type TimeZoneVariantInput = datetime_marker_helper!(@input/timezone/variant, yes);
    type TimeZoneLocalTimeInput = datetime_marker_helper!(@input/timezone/local_time, yes);
    type EssentialsV1Marker = datetime_marker_helper!(@data/zone/essentials, yes);
    type LocationsV1Marker = datetime_marker_helper!(@data/zone/locations, yes);
    type GenericLongV1Marker = datetime_marker_helper!(@data/zone/generic_long, yes);
    type GenericShortV1Marker = datetime_marker_helper!(@data/zone/generic_short, yes);
    type SpecificLongV1Marker = datetime_marker_helper!(@data/zone/specific_long, yes);
    type SpecificShortV1Marker = datetime_marker_helper!(@data/zone/specific_short, yes);
    type MetazonePeriodV1Marker = datetime_marker_helper!(@data/zone/metazone_periods, yes);
}

impl DateTimeMarkers for TimeZoneStyleWithLength {
    type D = NeoNeverMarker;
    type T = NeoNeverMarker;
    type Z = Self;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment,);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle,);
    type TimePrecisionOption = datetime_marker_helper!(@option/fractionalsecondigits,);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

// impl_get_field!(TimeZoneStyleWithLength, never);
// impl_get_field!(TimeZoneStyleWithLength, length, yes);

impl UnstableSealed for DateAndTimeFieldSet {}

// impl GetField<NeoComponents> for DateAndTimeFieldSet {
//     fn get_field(&self) -> NeoComponents {
//         self.components.into()
//     }
// }

// impl IsRuntimeComponents for DateAndTimeFieldSet {}

impl DateTimeNamesMarker for DateAndTimeFieldSet {
    type YearNames = datetime_marker_helper!(@names/year, yes);
    type MonthNames = datetime_marker_helper!(@names/month, yes);
    type WeekdayNames = datetime_marker_helper!(@names/weekday, yes);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, yes);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
    type ZoneLocations = datetime_marker_helper!(@names/zone/locations,);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
    type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods,);
}

impl DateTimeMarkers for DateAndTimeFieldSet {
    type D = NeoDateSkeleton;
    type T = TimeFieldSet;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle, yes);
    type TimePrecisionOption = datetime_marker_helper!(@option/timeprecision, yes);
    type GluePatternV1Marker = datetime_marker_helper!(@glue, yes);
}

// impl_get_field!(DateAndTimeFieldSet, never);
// impl_get_field!(DateAndTimeFieldSet, length, yes);
// impl_get_field!(DateAndTimeFieldSet, alignment, yes);
// impl_get_field!(DateAndTimeFieldSet, year_style, yes);
// impl_get_field!(DateAndTimeFieldSet, time_precision, yes);

impl UnstableSealed for CompositeFieldSet {}

// impl GetField<NeoComponents> for CompositeFieldSet {
//     fn get_field(&self) -> NeoComponents {
//         self.components
//     }
// }

// impl IsRuntimeComponents for CompositeFieldSet {}

impl DateTimeNamesMarker for CompositeFieldSet {
    type YearNames = datetime_marker_helper!(@names/year, yes);
    type MonthNames = datetime_marker_helper!(@names/month, yes);
    type WeekdayNames = datetime_marker_helper!(@names/weekday, yes);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, yes);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials, yes);
    type ZoneLocations = datetime_marker_helper!(@names/zone/locations, yes);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long, yes);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short, yes);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long, yes);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short, yes);
    type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods, yes);
}

impl DateTimeMarkers for CompositeFieldSet {
    type D = NeoDateSkeleton;
    type T = TimeFieldSet;
    type Z = TimeZoneStyleWithLength;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle, yes);
    type TimePrecisionOption = datetime_marker_helper!(@option/timeprecision, yes);
    type GluePatternV1Marker = datetime_marker_helper!(@glue, yes);
}

// impl_get_field!(CompositeFieldSet, never);
// impl_get_field!(CompositeFieldSet, length, yes);
// impl_get_field!(CompositeFieldSet, alignment, yes);
// impl_get_field!(CompositeFieldSet, year_style, yes);
// impl_get_field!(CompositeFieldSet, time_precision, yes);
