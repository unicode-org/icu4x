// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Temporary module for neo formatter markers.
//!
//! # Examples
//!
//! ## Alignment
//!
//! By default, datetimes are formatted for a variable-width context. You can
//! give a hint that the strings will be displayed in a column-like context,
//! which will coerce numerics to be padded with zeros.
//!
//! ```
//! use icu::calendar::Date;
//! use icu::calendar::Gregorian;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::datetime::fieldset::NeoYearMonthDayMarker;
//! use icu::datetime::neo_skeleton::Alignment;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let plain_formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!     &locale!("en-US").into(),
//!     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Short),
//! )
//! .unwrap();
//!
//! let column_formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!     &locale!("en-US").into(),
//!     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Short)
//!         .with_alignment(Alignment::Column),
//! )
//! .unwrap();
//!
//! // By default, en-US does not pad the month and day with zeros.
//! assert_try_writeable_eq!(
//!     plain_formatter
//!         .format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "1/1/25"
//! );
//!
//! // The column alignment option hints that they should be padded.
//! assert_try_writeable_eq!(
//!     column_formatter
//!         .format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "01/01/25"
//! );
//! ```
//!
//! ## Year Style
//!
//! The precision of the rendered year can be adjusted using the [`YearStyle`] option.
//!
//! ```
//! use icu::calendar::Date;
//! use icu::calendar::Gregorian;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::datetime::fieldset::NeoYearMonthDayMarker;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::datetime::neo_skeleton::YearStyle;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!     &locale!("en-US").into(),
//!     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Short)
//!         .with_year_style(YearStyle::Auto),
//! )
//! .unwrap();
//!
//! // Era displayed when needed for disambiguation,
//! // such as years before year 0 and small year numbers:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(-1000, 1, 1).unwrap()),
//!     "1/1/1001 BC"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(77, 1, 1).unwrap()),
//!     "1/1/77 AD"
//! );
//! // Era elided for modern years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(1900, 1, 1).unwrap()),
//!     "1/1/1900"
//! );
//! // Era and century both elided for nearby years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "1/1/25"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!     &locale!("en-US").into(),
//!     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Short)
//!         .with_year_style(YearStyle::Full),
//! )
//! .unwrap();
//!
//! // Era still displayed in cases with ambiguity:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(-1000, 1, 1).unwrap()),
//!     "1/1/1001 BC"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(77, 1, 1).unwrap()),
//!     "1/1/77 AD"
//! );
//! // Era elided for modern years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(1900, 1, 1).unwrap()),
//!     "1/1/1900"
//! );
//! // But now we always get a full-precision year:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "1/1/2025"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
//!     &locale!("en-US").into(),
//!     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Short)
//!         .with_year_style(YearStyle::Always),
//! )
//! .unwrap();
//!
//! // Era still displayed in cases with ambiguity:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(-1000, 1, 1).unwrap()),
//!     "1/1/1001 BC"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(77, 1, 1).unwrap()),
//!     "1/1/77 AD"
//! );
//! // But now it is shown even on modern years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(1900, 1, 1).unwrap()),
//!     "1/1/1900 AD"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian(2025, 1, 1).unwrap()),
//!     "1/1/2025 AD"
//! );
//! ```
//!
//! ## Hour Cycle
//!
//! Hours can be switched between 12-hour and 24-hour time via the `u-hc` locale keyword.
//!
//! ```
//! use icu::calendar::Time;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::datetime::fieldset::NeoHourMinuteMarker;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! // By default, en-US uses 12-hour time and fr-FR uses 24-hour time,
//! // but we can set overrides.
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("en-US-u-hc-h12").into(),
//!     NeoHourMinuteMarker::with_length(NeoSkeletonLength::Short),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "4:12 PM"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("en-US-u-hc-h23").into(),
//!     NeoHourMinuteMarker::with_length(NeoSkeletonLength::Short),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "16:12"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("fr-FR-u-hc-h12").into(),
//!     NeoHourMinuteMarker::with_length(NeoSkeletonLength::Short),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "4:12 PM"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("fr-FR-u-hc-h23").into(),
//!     NeoHourMinuteMarker::with_length(NeoSkeletonLength::Short),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "16:12"
//! );
//! ```
//!
//! Hour cycles `h11` and `h24` are supported, too:
//!
//! ```
//! use icu::calendar::Time;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::datetime::fieldset::NeoHourMinuteMarker;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("und-u-hc-h11").into(),
//!     NeoHourMinuteMarker::with_length(NeoSkeletonLength::Short),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(0, 0, 0, 0).unwrap()),
//!     "0:00 AM"
//! );
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("und-u-hc-h24").into(),
//!     NeoHourMinuteMarker::with_length(NeoSkeletonLength::Short),
//! )
//! .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(0, 0, 0, 0).unwrap()),
//!     "24:00"
//! );
//! ```
//!
//! ## Fractional Second Digits
//!
//! Times can be displayed with a custom number of fractional digits from 0-9:
//!
//! ```
//! use icu::calendar::Gregorian;
//! use icu::calendar::Time;
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::datetime::fieldset::NeoHourMinuteSecondMarker;
//! use icu::datetime::neo_skeleton::FractionalSecondDigits;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatter = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("en-US").into(),
//!     NeoHourMinuteSecondMarker::with_length(NeoSkeletonLength::Short)
//!         .with_fractional_second_digits(FractionalSecondDigits::F2),
//! )
//! .unwrap();
//!
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 543200000).unwrap()),
//!     "4:12:20.54 PM"
//! );
//! ```
//!
//! ## Time Zone Formatting
//!
//! Here, we configure a [`DateTimeFormatter`] to format with generic non-location short,
//! which falls back to the offset when unavailable (see [`NeoTimeZoneGenericMarker`]).
//!
//! ```
//! use icu::calendar::{Date, Time};
//! use icu::timezone::{TimeZoneInfo, UtcOffset, TimeZoneIdMapper, TimeZoneBcp47Id};
//! use icu::datetime::FixedCalendarDateTimeFormatter;
//! use icu::datetime::fieldset::NeoTimeZoneGenericMarker;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::datetime::DateTimeWriteError;
//! use icu::locale::locale;
//! use tinystr::tinystr;
//! use writeable::assert_try_writeable_eq;
//!
//! // Set up the time zone. Note: the inputs here are
//! //   1. The offset
//! //   2. The IANA time zone ID
//! //   3. A date and time (for non-location name resolution)
//! //   4. Note: we do not need the zone variant because of `load_generic_*()`
//!
//! // Set up the time zone ID mapper
//! let mapper = TimeZoneIdMapper::new();
//!
//! // Set up the formatter
//! let mut tzf = FixedCalendarDateTimeFormatter::<(), _>::try_new(
//!     &locale!("en").into(),
//!     NeoTimeZoneGenericMarker::with_length(NeoSkeletonLength::Short),
//! )
//! .unwrap();
//!
//! // "uschi" - has symbol data for generic_non_location_short
//! let time_zone = TimeZoneInfo::from_id_and_offset(
//!     mapper.as_borrowed().iana_to_bcp47("America/Chicago"),
//!     UtcOffset::from_eighths_of_hour(-5 * 8),
//! )
//! .at_time((Date::try_new_iso(2022, 8, 29).unwrap(), Time::midnight()));
//! assert_try_writeable_eq!(
//!     tzf.format(&time_zone),
//!     "CT"
//! );
//!
//! // "ushnl" - has time zone override symbol data for generic_non_location_short
//! let time_zone = TimeZoneInfo::from_id_and_offset(
//!     mapper.as_borrowed().iana_to_bcp47("Pacific/Honolulu"),
//!     UtcOffset::from_eighths_of_hour(-10 * 8),
//! )
//! .at_time((Date::try_new_iso(2022, 8, 29).unwrap(), Time::midnight()));
//! assert_try_writeable_eq!(
//!     tzf.format(&time_zone),
//!     "HST"
//! );
//!
//! // Mis-spelling of "America/Chicago" results in a fallback to GMT format
//! let time_zone = TimeZoneInfo::from_id_and_offset(
//!     mapper.as_borrowed().iana_to_bcp47("America/Chigagou"),
//!     UtcOffset::from_eighths_of_hour(-5 * 8),
//! )
//! .at_time((Date::try_new_iso(2022, 8, 29).unwrap(), Time::midnight()));
//! assert_try_writeable_eq!(
//!     tzf.format(&time_zone),
//!     "GMT-5"
//! );
//! ```

#[cfg(doc)]
use crate::neo::DateTimeFormatter;

use core::marker::PhantomData;

use crate::{
    format::neo::*,
    neo_skeleton::*,
    provider::{neo::*, time_zones::tz, *},
    scaffold::*,
};
use icu_calendar::{
    any_calendar::IntoAnyCalendar,
    types::{
        DayOfMonth, DayOfYearInfo, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo,
        NanoSecond, YearInfo,
    },
    AnyCalendar, AnyCalendarKind, AsCalendar, Calendar, Date, DateTime, Iso, Ref, Time,
};
use icu_provider::{marker::NeverMarker, prelude::*};
use icu_timezone::scaffold::IntoOption;
use icu_timezone::{
    CustomZonedDateTime, TimeZoneBcp47Id, TimeZoneInfo, TimeZoneModel, UtcOffset, ZoneVariant,
};

impl GetField<NeoComponents> for NeoDateSkeleton {
    fn get_field(&self) -> NeoComponents {
        self.components.into()
    }
}

impl UnstableSealed for NeoDateSkeleton {}

impl IsRuntimeComponents for NeoDateSkeleton {}

impl DateTimeNamesMarker for NeoDateSkeleton {
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

impl DateInputMarkers for NeoDateSkeleton {
    type YearInput = datetime_marker_helper!(@input/year, yes);
    type MonthInput = datetime_marker_helper!(@input/month, yes);
    type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month, yes);
    type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week, yes);
    type AnyCalendarKindInput = datetime_marker_helper!(@input/any_calendar_kind, yes);
}

impl<C: CldrCalendar> TypedDateDataMarkers<C> for NeoDateSkeleton {
    type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, yes);
    type YearNamesV1Marker = datetime_marker_helper!(@years/typed, yes);
    type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, yes);
}

impl DateDataMarkers for NeoDateSkeleton {
    type Skel = datetime_marker_helper!(@calmarkers, yes);
    type Year = datetime_marker_helper!(@calmarkers, yes);
    type Month = datetime_marker_helper!(@calmarkers, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, yes);
}

impl DateTimeMarkers for NeoDateSkeleton {
    type D = Self;
    type T = NeoNeverMarker;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle, yes);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

impl_get_field!(NeoDateSkeleton, never);
impl_get_field!(NeoDateSkeleton, length, yes);
impl_get_field!(NeoDateSkeleton, alignment, yes);
impl_get_field!(NeoDateSkeleton, year_style, yes);

impl UnstableSealed for NeoCalendarPeriodSkeleton {}

impl GetField<NeoComponents> for NeoCalendarPeriodSkeleton {
    fn get_field(&self) -> NeoComponents {
        self.components.into()
    }
}

impl IsRuntimeComponents for NeoCalendarPeriodSkeleton {}

impl DateTimeNamesMarker for NeoCalendarPeriodSkeleton {
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

impl DateInputMarkers for NeoCalendarPeriodSkeleton {
    type YearInput = datetime_marker_helper!(@input/year, yes);
    type MonthInput = datetime_marker_helper!(@input/month, yes);
    type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month,);
    type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week,);
    type AnyCalendarKindInput = datetime_marker_helper!(@input/any_calendar_kind, yes);
}

impl<C: CldrCalendar> TypedDateDataMarkers<C> for NeoCalendarPeriodSkeleton {
    type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, yes);
    type YearNamesV1Marker = datetime_marker_helper!(@years/typed, yes);
    type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays,);
}

impl DateDataMarkers for NeoCalendarPeriodSkeleton {
    type Skel = datetime_marker_helper!(@calmarkers, yes);
    type Year = datetime_marker_helper!(@calmarkers, yes);
    type Month = datetime_marker_helper!(@calmarkers, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays,);
}

impl DateTimeMarkers for NeoCalendarPeriodSkeleton {
    type D = Self;
    type T = NeoNeverMarker;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle, yes);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

impl_get_field!(NeoCalendarPeriodSkeleton, never);
impl_get_field!(NeoCalendarPeriodSkeleton, length, yes);
impl_get_field!(NeoCalendarPeriodSkeleton, alignment, yes);
impl_get_field!(NeoCalendarPeriodSkeleton, year_style, yes);

impl UnstableSealed for NeoTimeSkeleton {}

impl GetField<NeoComponents> for NeoTimeSkeleton {
    fn get_field(&self) -> NeoComponents {
        self.components.into()
    }
}

impl IsRuntimeComponents for NeoTimeSkeleton {}

impl DateTimeNamesMarker for NeoTimeSkeleton {
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

impl TimeMarkers for NeoTimeSkeleton {
    type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, yes);
    type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, yes);
    type HourInput = datetime_marker_helper!(@input/hour, yes);
    type MinuteInput = datetime_marker_helper!(@input/minute, yes);
    type SecondInput = datetime_marker_helper!(@input/second, yes);
    type NanoSecondInput = datetime_marker_helper!(@input/nanosecond, yes);
}

impl DateTimeMarkers for NeoTimeSkeleton {
    type D = NeoNeverMarker;
    type T = Self;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle,);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits, yes);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

impl_get_field!(NeoTimeSkeleton, never);
impl_get_field!(NeoTimeSkeleton, length, yes);
impl_get_field!(NeoTimeSkeleton, alignment, yes);
impl_get_field!(NeoTimeSkeleton, fractional_second_digits, yes);

impl UnstableSealed for NeoTimeZoneSkeleton {}

impl GetField<NeoComponents> for NeoTimeZoneSkeleton {
    fn get_field(&self) -> NeoComponents {
        self.style.into()
    }
}

impl IsRuntimeComponents for NeoTimeZoneSkeleton {}

impl DateTimeNamesMarker for NeoTimeZoneSkeleton {
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

impl ZoneMarkers for NeoTimeZoneSkeleton {
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

impl DateTimeMarkers for NeoTimeZoneSkeleton {
    type D = NeoNeverMarker;
    type T = NeoNeverMarker;
    type Z = Self;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment,);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle,);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

impl_get_field!(NeoTimeZoneSkeleton, never);
impl_get_field!(NeoTimeZoneSkeleton, length, yes);

impl UnstableSealed for NeoDateTimeSkeleton {}

impl GetField<NeoComponents> for NeoDateTimeSkeleton {
    fn get_field(&self) -> NeoComponents {
        self.components.into()
    }
}

impl IsRuntimeComponents for NeoDateTimeSkeleton {}

impl DateTimeNamesMarker for NeoDateTimeSkeleton {
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

impl DateTimeMarkers for NeoDateTimeSkeleton {
    type D = NeoDateSkeleton;
    type T = NeoTimeSkeleton;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle, yes);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits, yes);
    type GluePatternV1Marker = datetime_marker_helper!(@glue, yes);
}

impl_get_field!(NeoDateTimeSkeleton, never);
impl_get_field!(NeoDateTimeSkeleton, length, yes);
impl_get_field!(NeoDateTimeSkeleton, alignment, yes);
impl_get_field!(NeoDateTimeSkeleton, year_style, yes);
impl_get_field!(NeoDateTimeSkeleton, fractional_second_digits, yes);

impl UnstableSealed for NeoSkeleton {}

impl GetField<NeoComponents> for NeoSkeleton {
    fn get_field(&self) -> NeoComponents {
        self.components
    }
}

impl IsRuntimeComponents for NeoSkeleton {}

impl DateTimeNamesMarker for NeoSkeleton {
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

impl DateTimeMarkers for NeoSkeleton {
    type D = NeoDateSkeleton;
    type T = NeoTimeSkeleton;
    type Z = NeoTimeZoneSkeleton;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type YearStyleOption = datetime_marker_helper!(@option/yearstyle, yes);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits, yes);
    type GluePatternV1Marker = datetime_marker_helper!(@glue, yes);
}

impl_get_field!(NeoSkeleton, never);
impl_get_field!(NeoSkeleton, length, yes);
impl_get_field!(NeoSkeleton, alignment, yes);
impl_get_field!(NeoSkeleton, year_style, yes);
impl_get_field!(NeoSkeleton, fractional_second_digits, yes);
