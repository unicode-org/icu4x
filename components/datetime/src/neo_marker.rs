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
//! use icu::datetime::neo::NeoOptions;
//! use icu::datetime::neo::TypedNeoFormatter;
//! use icu::datetime::neo_marker::NeoYearMonthDayMarker;
//! use icu::datetime::neo_skeleton::Alignment;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let plain_formatter =
//!     TypedNeoFormatter::<Gregorian, NeoYearMonthDayMarker>::try_new(
//!         &locale!("en-US").into(),
//!         NeoSkeletonLength::Short.into(),
//!     )
//!     .unwrap();
//!
//! let column_formatter =
//!     TypedNeoFormatter::<Gregorian, NeoYearMonthDayMarker>::try_new(
//!         &locale!("en-US").into(),
//!         {
//!             let mut options = NeoOptions::from(NeoSkeletonLength::Short);
//!             options.alignment = Some(Alignment::Column);
//!             options
//!         }
//!     )
//!     .unwrap();
//!
//! // By default, en-US does not pad the month and day with zeros.
//! assert_try_writeable_eq!(
//!     plain_formatter.format(&Date::try_new_gregorian_date(2025, 1, 1).unwrap()),
//!     "1/1/25"
//! );
//!
//! // The column alignment option hints that they should be padded.
//! assert_try_writeable_eq!(
//!     column_formatter.format(&Date::try_new_gregorian_date(2025, 1, 1).unwrap()),
//!     "01/01/25"
//! );
//! ```
//!
//! ## Era Display
//!
//! The era field can be toggled on and off using the [`EraDisplay`] option.
//!
//! ```
//! use icu::calendar::Date;
//! use icu::calendar::Gregorian;
//! use icu::datetime::neo::NeoOptions;
//! use icu::datetime::neo::TypedNeoFormatter;
//! use icu::datetime::neo_marker::NeoEraYearMonthDayMarker;
//! use icu::datetime::neo_skeleton::EraDisplay;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatter =
//!     TypedNeoFormatter::<Gregorian, NeoEraYearMonthDayMarker>::try_new(
//!         &locale!("en-US").into(),
//!         {
//!             let mut options = NeoOptions::from(NeoSkeletonLength::Medium);
//!             options.era_display = Some(EraDisplay::Auto);
//!             options
//!         }
//!     )
//!     .unwrap();
//!
//! // Era displayed when needed for disambiguation,
//! // such as years before year 0 and small year numbers:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian_date(-1000, 1, 1).unwrap()),
//!     "Jan 1, 1001 BC"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian_date(77, 1, 1).unwrap()),
//!     "Jan 1, 77 AD"
//! );
//! // Era elided for modern years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian_date(2023, 12, 20).unwrap()),
//!     "Dec 20, 2023"
//! );
//!
//! let formatter =
//!     TypedNeoFormatter::<Gregorian, NeoEraYearMonthDayMarker>::try_new(
//!         &locale!("en-US").into(),
//!         {
//!             let mut options = NeoOptions::from(NeoSkeletonLength::Medium);
//!             options.era_display = Some(EraDisplay::Always);
//!             options
//!         }
//!     )
//!     .unwrap();
//!
//! // Era still displayed in cases with ambiguity:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian_date(-1000, 1, 1).unwrap()),
//!     "Jan 1, 1001 BC"
//! );
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian_date(77, 1, 1).unwrap()),
//!     "Jan 1, 77 AD"
//! );
//! // But now it is shown even on modern years:
//! assert_try_writeable_eq!(
//!     formatter.format(&Date::try_new_gregorian_date(2023, 12, 20).unwrap()),
//!     "Dec 20, 2023 AD"
//! );
//! ```
//!
//! ## Hour Cycle
//!
//! Hours can be switched between 12-hour and 24-hour time via the `u-hc` locale keyword.
//!
//! ```
//! use icu::calendar::Time;
//! use icu::datetime::neo::TypedNeoFormatter;
//! use icu::datetime::neo_marker::NeoHourMinuteMarker;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::datetime::NeverCalendar;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! // By default, en-US uses 12-hour time and fr-FR uses 24-hour time,
//! // but we can set overrides.
//!
//! let formatter =
//!     TypedNeoFormatter::<NeverCalendar, NeoHourMinuteMarker>::try_new(
//!         &locale!("en-US-u-hc-h12").into(),
//!         NeoSkeletonLength::Short.into(),
//!     )
//!     .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "4:12 PM"
//! );
//!
//! let formatter =
//!     TypedNeoFormatter::<NeverCalendar, NeoHourMinuteMarker>::try_new(
//!         &locale!("en-US-u-hc-h23").into(),
//!         NeoSkeletonLength::Short.into(),
//!     )
//!     .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "16:12"
//! );
//!
//! let formatter =
//!     TypedNeoFormatter::<NeverCalendar, NeoHourMinuteMarker>::try_new(
//!         &locale!("fr-FR-u-hc-h12").into(),
//!         NeoSkeletonLength::Short.into(),
//!     )
//!     .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 0).unwrap()),
//!     "4:12 PM"
//! );
//!
//! let formatter =
//!     TypedNeoFormatter::<NeverCalendar, NeoHourMinuteMarker>::try_new(
//!         &locale!("fr-FR-u-hc-h23").into(),
//!         NeoSkeletonLength::Short.into(),
//!     )
//!     .unwrap();
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
//! use icu::datetime::neo::TypedNeoFormatter;
//! use icu::datetime::neo_marker::NeoHourMinuteMarker;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::datetime::NeverCalendar;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatter =
//!     TypedNeoFormatter::<NeverCalendar, NeoHourMinuteMarker>::try_new(
//!         &locale!("und-u-hc-h11").into(),
//!         NeoSkeletonLength::Short.into(),
//!     )
//!     .unwrap();
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(0, 0, 0, 0).unwrap()),
//!     "0:00 AM"
//! );
//!
//! let formatter =
//!     TypedNeoFormatter::<NeverCalendar, NeoHourMinuteMarker>::try_new(
//!         &locale!("und-u-hc-h24").into(),
//!         NeoSkeletonLength::Short.into(),
//!     )
//!     .unwrap();
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
//! use icu::datetime::neo::NeoOptions;
//! use icu::datetime::neo::TypedNeoFormatter;
//! use icu::datetime::neo_marker::NeoHourMinuteSecondMarker;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::datetime::neo_skeleton::FractionalSecondDigits;
//! use icu::datetime::NeverCalendar;
//! use icu::locale::locale;
//! use writeable::assert_try_writeable_eq;
//!
//! let formatter =
//!     TypedNeoFormatter::<NeverCalendar, NeoHourMinuteSecondMarker>::try_new(
//!         &locale!("en-US").into(),
//!         {
//!             let mut options = NeoOptions::from(NeoSkeletonLength::Short);
//!             options.fractional_second_digits = Some(FractionalSecondDigits::F2);
//!             options
//!         }
//!     )
//!     .unwrap();
//!
//! assert_try_writeable_eq!(
//!     formatter.format(&Time::try_new(16, 12, 20, 543200000).unwrap()),
//!     "4:12:20.54 PM"
//! );
//! ```
//!
//! ## Time Zone Formatting
//!
//! Here, we configure a [`NeoFormatter`] to format with generic non-location short,
//! which falls back to the offset when unavailable (see [`NeoTimeZoneGenericMarker`]).
//!
//! ```
//! use icu::calendar::DateTime;
//! use icu::timezone::{CustomTimeZone, MetazoneCalculator, TimeZoneIdMapper, TimeZoneBcp47Id};
//! use icu::datetime::neo::TypedNeoFormatter;
//! use icu::datetime::neo_marker::NeoTimeZoneGenericMarker;
//! use icu::datetime::neo_skeleton::NeoSkeletonLength;
//! use icu::datetime::NeverCalendar;
//! use icu::locale::locale;
//! use tinystr::tinystr;
//! use writeable::assert_try_writeable_eq;
//!
//! // Set up the time zone. Note: the inputs here are
//! //   1. The offset
//! //   2. The IANA time zone ID
//! //   3. A datetime (for metazone resolution)
//! //   4. Note: we do not need the zone variant because of `load_generic_*()`
//!
//! // Set up the Metazone calculator, time zone ID mapper,
//! // and the DateTime to use in calculation
//! let mzc = MetazoneCalculator::new();
//! let mapper = TimeZoneIdMapper::new();
//! let datetime = DateTime::try_new_iso_datetime(2022, 8, 29, 0, 0, 0)
//!     .unwrap();
//!
//! // Set up the formatter
//! let mut tzf = TypedNeoFormatter::<NeverCalendar, NeoTimeZoneGenericMarker>::try_new(
//!     &locale!("en").into(),
//!     NeoSkeletonLength::Short.into(),
//! )
//! .unwrap();
//!
//! // "uschi" - has metazone symbol data for generic_non_location_short
//! let mut time_zone = "-0600".parse::<CustomTimeZone>().unwrap();
//! time_zone.time_zone_id = mapper.as_borrowed().iana_to_bcp47("America/Chicago");
//! time_zone.maybe_calculate_metazone(&mzc, &datetime);
//! assert_try_writeable_eq!(
//!     tzf.format(&time_zone),
//!     "CT"
//! );
//!
//! // "ushnl" - has time zone override symbol data for generic_non_location_short
//! let mut time_zone = "-1000".parse::<CustomTimeZone>().unwrap();
//! time_zone.time_zone_id = Some(TimeZoneBcp47Id(tinystr!(8, "ushnl")));
//! time_zone.maybe_calculate_metazone(&mzc, &datetime);
//! assert_try_writeable_eq!(
//!     tzf.format(&time_zone),
//!     "HST"
//! );
//!
//! // Raw offset - used when metazone is not available
//! let mut time_zone = "+0530".parse::<CustomTimeZone>().unwrap();
//! assert_try_writeable_eq!(
//!     tzf.format(&time_zone),
//!     "GMT+5:30"
//! );
//! ```

#[cfg(doc)]
use crate::neo::NeoFormatter;

use core::marker::PhantomData;

use crate::{
    format::neo::*,
    neo_skeleton::*,
    provider::{neo::*, time_zones::tz},
    CldrCalendar,
};
use icu_calendar::{
    any_calendar::IntoAnyCalendar,
    types::{
        DayOfMonth, DayOfYearInfo, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo,
        NanoSecond, YearInfo,
    },
    AnyCalendar, AnyCalendarKind, AsCalendar, Calendar, Date, DateTime, Ref, Time,
};
use icu_provider::{marker::NeverMarker, prelude::*};
use icu_timezone::{
    CustomTimeZone, CustomZonedDateTime, MetazoneId, TimeZoneBcp47Id, UtcOffset, ZoneVariant,
};

// TODO: Figure out where to export these traits
#[doc(inline)]
pub use crate::calendar::CalMarkers;
#[doc(inline)]
pub use crate::calendar::FullDataCalMarkers;
#[doc(inline)]
pub use crate::calendar::NoDataCalMarkers;

pub(crate) mod private {
    pub trait Sealed {}
}

/// A type that can be converted into a specific calendar system.
pub trait ConvertCalendar {
    /// The converted type. This can be the same as the receiver type.
    type Converted<'a>: Sized;
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

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> ConvertCalendar for CustomZonedDateTime<A> {
    type Converted<'a> = CustomZonedDateTime<Ref<'a, AnyCalendar>>;
    #[inline]
    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        let date = self.date.to_any().to_calendar(Ref(calendar));
        CustomZonedDateTime {
            date,
            time: self.time,
            zone: self.zone,
        }
    }
}

impl ConvertCalendar for CustomTimeZone {
    type Converted<'a> = CustomTimeZone;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
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

impl IsAnyCalendarKind for Time {
    fn is_any_calendar_kind(&self, _: AnyCalendarKind) -> bool {
        true
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> IsAnyCalendarKind for DateTime<A> {
    fn is_any_calendar_kind(&self, any_calendar_kind: AnyCalendarKind) -> bool {
        self.date.calendar().any_calendar_kind() == Some(any_calendar_kind)
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> IsAnyCalendarKind for CustomZonedDateTime<A> {
    fn is_any_calendar_kind(&self, _: AnyCalendarKind) -> bool {
        true
    }
}

/// An input associated with a specific calendar.
pub trait IsInCalendar<C> {}

impl<C: Calendar, A: AsCalendar<Calendar = C>> IsInCalendar<C> for Date<A> {}

impl<C> IsInCalendar<C> for Time {}

impl<C: Calendar, A: AsCalendar<Calendar = C>> IsInCalendar<C> for DateTime<A> {}

impl<C: Calendar, A: AsCalendar<Calendar = C>> IsInCalendar<C> for CustomZonedDateTime<A> {}

impl<C> IsInCalendar<C> for CustomTimeZone {}

/// A type that can return a certain field `T`.
pub trait NeoGetField<T> {
    /// Returns the value of this trait's field `T`.
    fn get_field(&self) -> T;
}

impl<T> NeoGetField<T> for T
where
    T: Copy,
{
    #[inline]
    fn get_field(&self) -> T {
        *self
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<YearInfo> for Date<A> {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<MonthInfo> for Date<A> {
    #[inline]
    fn get_field(&self) -> MonthInfo {
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

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<YearInfo> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date.year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<MonthInfo> for DateTime<A> {
    #[inline]
    fn get_field(&self) -> MonthInfo {
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

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<YearInfo> for CustomZonedDateTime<A> {
    #[inline]
    fn get_field(&self) -> YearInfo {
        self.date.year()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<MonthInfo> for CustomZonedDateTime<A> {
    #[inline]
    fn get_field(&self) -> MonthInfo {
        self.date.month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<DayOfMonth> for CustomZonedDateTime<A> {
    #[inline]
    fn get_field(&self) -> DayOfMonth {
        self.date.day_of_month()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<IsoWeekday> for CustomZonedDateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoWeekday {
        self.date.day_of_week()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<DayOfYearInfo>
    for CustomZonedDateTime<A>
{
    #[inline]
    fn get_field(&self) -> DayOfYearInfo {
        self.date.day_of_year_info()
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> NeoGetField<AnyCalendarKind>
    for CustomZonedDateTime<A>
{
    #[inline]
    fn get_field(&self) -> AnyCalendarKind {
        self.date.calendar().kind()
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<IsoHour> for CustomZonedDateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoHour {
        self.time.hour
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<IsoMinute> for CustomZonedDateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoMinute {
        self.time.minute
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<IsoSecond> for CustomZonedDateTime<A> {
    #[inline]
    fn get_field(&self) -> IsoSecond {
        self.time.second
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<NanoSecond> for CustomZonedDateTime<A> {
    #[inline]
    fn get_field(&self) -> NanoSecond {
        self.time.nanosecond
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<Option<UtcOffset>>
    for CustomZonedDateTime<A>
{
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        self.zone.offset
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<Option<TimeZoneBcp47Id>>
    for CustomZonedDateTime<A>
{
    #[inline]
    fn get_field(&self) -> Option<TimeZoneBcp47Id> {
        self.zone.time_zone_id
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<Option<MetazoneId>>
    for CustomZonedDateTime<A>
{
    #[inline]
    fn get_field(&self) -> Option<MetazoneId> {
        self.zone.metazone_id
    }
}

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<Option<ZoneVariant>>
    for CustomZonedDateTime<A>
{
    #[inline]
    fn get_field(&self) -> Option<ZoneVariant> {
        self.zone.zone_variant
    }
}

impl NeoGetField<Option<UtcOffset>> for CustomTimeZone {
    #[inline]
    fn get_field(&self) -> Option<UtcOffset> {
        self.offset
    }
}

impl NeoGetField<Option<TimeZoneBcp47Id>> for CustomTimeZone {
    #[inline]
    fn get_field(&self) -> Option<TimeZoneBcp47Id> {
        self.time_zone_id
    }
}

impl NeoGetField<Option<MetazoneId>> for CustomTimeZone {
    #[inline]
    fn get_field(&self) -> Option<MetazoneId> {
        self.metazone_id
    }
}

impl NeoGetField<Option<ZoneVariant>> for CustomTimeZone {
    #[inline]
    fn get_field(&self) -> Option<ZoneVariant> {
        self.zone_variant
    }
}

/// Struct representing the absence of a datetime formatting field.
#[derive(Debug, Copy, Clone, Default)]
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

impl<C: Calendar, A: AsCalendar<Calendar = C>> NeoGetField<NeverField> for CustomZonedDateTime<A> {
    #[inline]
    fn get_field(&self) -> NeverField {
        NeverField
    }
}

impl NeoGetField<NeverField> for CustomTimeZone {
    #[inline]
    fn get_field(&self) -> NeverField {
        NeverField
    }
}

impl From<NeverField> for Option<YearInfo> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<MonthInfo> {
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

impl From<NeverField> for Option<UtcOffset> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<TimeZoneBcp47Id> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<MetazoneId> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<ZoneVariant> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<NeoSkeletonLength> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<Alignment> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<EraDisplay> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

impl From<NeverField> for Option<FractionalSecondDigits> {
    #[inline]
    fn from(_: NeverField) -> Self {
        None
    }
}

/// A trait associating [`NeoComponents`].
pub trait HasConstComponents {
    /// The associated components.
    const COMPONENTS: NeoComponents;
}

/// A trait associating [`NeoDateComponents`].
pub trait HasConstDateComponents {
    /// The associated components.
    const COMPONENTS: NeoDateComponents;
}

/// A trait associating [`NeoDayComponents`].
pub trait HasConstDayComponents {
    /// The associated components.
    const COMPONENTS: NeoDayComponents;
}

/// A trait associating [`NeoTimeComponents`].
pub trait HasConstTimeComponents {
    /// The associated components.
    const COMPONENTS: NeoTimeComponents;
}

/// A trait associating [`NeoTimeZoneSkeleton`].
pub trait HasConstZoneComponent {
    /// The associated component.
    const COMPONENT: NeoTimeZoneSkeleton;
}

// TODO: Add WeekCalculator and FixedDecimalFormatter optional bindings here

/// A trait associating types for date formatting in any calendar
/// (input types only).
pub trait DateInputMarkers: private::Sealed {
    /// Marker for resolving the year input field.
    type YearInput: Into<Option<YearInfo>>;
    /// Marker for resolving the month input field.
    type MonthInput: Into<Option<MonthInfo>>;
    /// Marker for resolving the day-of-month input field.
    type DayOfMonthInput: Into<Option<DayOfMonth>>;
    /// Marker for resolving the day-of-week input field.
    type DayOfWeekInput: Into<Option<IsoWeekday>>;
    /// Marker for resolving the day-of-year input field.
    type DayOfYearInput: Into<Option<DayOfYearInfo>>;
    /// Marker for resolving the any-calendar-kind input field.
    type AnyCalendarKindInput: Into<Option<AnyCalendarKind>>;
}

/// A trait associating types for date formatting in a specific calendar
/// (data markers only).
pub trait TypedDateDataMarkers<C>: private::Sealed {
    /// Marker for loading date skeleton patterns.
    type DateSkeletonPatternsV1Marker: DataMarker<DataStruct = PackedSkeletonDataV1<'static>>;
    /// Marker for loading year names.
    type YearNamesV1Marker: DataMarker<DataStruct = YearNamesV1<'static>>;
    /// Marker for loading month names.
    type MonthNamesV1Marker: DataMarker<DataStruct = MonthNamesV1<'static>>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: DataMarker<DataStruct = LinearNamesV1<'static>>;
}

/// A trait associating types for date formatting in any calendar
/// (data markers only).
pub trait DateDataMarkers: private::Sealed {
    /// Cross-calendar data markers for date skeleta.
    type Skel: CalMarkers<SkeletaV1Marker>;
    /// Cross-calendar data markers for year names.
    type Year: CalMarkers<YearNamesV1Marker>;
    /// Cross-calendar data markers for month names.
    type Month: CalMarkers<MonthNamesV1Marker>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: DataMarker<DataStruct = LinearNamesV1<'static>>;
}

/// A trait associating types for time formatting
/// (input types and data markers).
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
    type TimeSkeletonPatternsV1Marker: DataMarker<DataStruct = PackedSkeletonDataV1<'static>>;
    /// Marker for loading day period names.
    type DayPeriodNamesV1Marker: DataMarker<DataStruct = LinearNamesV1<'static>>;
}

/// A trait associating types for time zone formatting
/// (input types and data markers).
pub trait ZoneMarkers: private::Sealed {
    /// Marker for resolving the time zone offset input field.
    type TimeZoneOffsetInput: Into<Option<UtcOffset>>;
    /// Marker for resolving the time zone id input field.
    type TimeZoneIdInput: Into<Option<TimeZoneBcp47Id>>;
    /// Marker for resolving the time zone metazone input field.
    type TimeZoneMetazoneInput: Into<Option<MetazoneId>>;
    /// Marker for resolving the time zone variant input field.
    type TimeZoneVariantInput: Into<Option<ZoneVariant>>;
    /// Marker for loading core time zone data.
    type EssentialsV1Marker: DataMarker<DataStruct = tz::EssentialsV1<'static>>;
    /// Marker for loading exemplar city names for time zone formatting
    type ExemplarCitiesV1Marker: DataMarker<DataStruct = tz::ExemplarCitiesV1<'static>>;
    /// Marker for loading generic short time zone names.
    type GenericLongV1Marker: DataMarker<DataStruct = tz::MzGenericV1<'static>>;
    /// Marker for loading generic short time zone names.
    type GenericShortV1Marker: DataMarker<DataStruct = tz::MzGenericV1<'static>>;
    /// Marker for loading generic short time zone names.
    type SpecificLongV1Marker: DataMarker<DataStruct = tz::MzSpecificV1<'static>>;
    /// Marker for loading generic short time zone names.
    type SpecificShortV1Marker: DataMarker<DataStruct = tz::MzSpecificV1<'static>>;
}

/// A trait associating constants and types implementing various other traits
/// required for datetime formatting.
pub trait DateTimeMarkers: private::Sealed + DateTimeNamesMarker {
    /// Associated types for date formatting.
    ///
    /// Should implement [`DateDataMarkers`], [`TypedDateDataMarkers`], and [`DateInputMarkers`].
    type D;
    /// Associated types for time formatting.
    ///
    /// Should implement [`TimeMarkers`].
    type T;
    /// Associated types for time zone formatting.
    ///
    /// Should implement [`ZoneMarkers`].
    type Z;
    /// Type of the length option in the constructor.
    type LengthOption: Into<Option<NeoSkeletonLength>>;
    /// Type of the alignment option in the constructor.
    type AlignmentOption: Into<Option<Alignment>>;
    /// Type of the era display option in the constructor.
    type EraDisplayOption: Into<Option<EraDisplay>>;
    /// Type of the fractional seconds display option in the constructor.
    type FractionalSecondDigitsOption: Into<Option<FractionalSecondDigits>>;
    /// Marker for loading the date/time glue pattern.
    type GluePatternV1Marker: DataMarker<DataStruct = GluePatternV1<'static>>;
}

/// Trait to consolidate input markers.
pub trait AllInputMarkers<R: DateTimeMarkers>:
    NeoGetField<<R::D as DateInputMarkers>::YearInput>
    + NeoGetField<<R::D as DateInputMarkers>::MonthInput>
    + NeoGetField<<R::D as DateInputMarkers>::DayOfMonthInput>
    + NeoGetField<<R::D as DateInputMarkers>::DayOfWeekInput>
    + NeoGetField<<R::D as DateInputMarkers>::DayOfYearInput>
    + NeoGetField<<R::D as DateInputMarkers>::AnyCalendarKindInput>
    + NeoGetField<<R::T as TimeMarkers>::HourInput>
    + NeoGetField<<R::T as TimeMarkers>::MinuteInput>
    + NeoGetField<<R::T as TimeMarkers>::SecondInput>
    + NeoGetField<<R::T as TimeMarkers>::NanoSecondInput>
    + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneOffsetInput>
    + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneIdInput>
    + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneMetazoneInput>
    + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneVariantInput>
where
    R::D: DateInputMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
{
}

impl<T, R> AllInputMarkers<R> for T
where
    R: DateTimeMarkers,
    R::D: DateInputMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
    T: NeoGetField<<R::D as DateInputMarkers>::YearInput>
        + NeoGetField<<R::D as DateInputMarkers>::MonthInput>
        + NeoGetField<<R::D as DateInputMarkers>::DayOfMonthInput>
        + NeoGetField<<R::D as DateInputMarkers>::DayOfWeekInput>
        + NeoGetField<<R::D as DateInputMarkers>::DayOfYearInput>
        + NeoGetField<<R::D as DateInputMarkers>::AnyCalendarKindInput>
        + NeoGetField<<R::T as TimeMarkers>::HourInput>
        + NeoGetField<<R::T as TimeMarkers>::MinuteInput>
        + NeoGetField<<R::T as TimeMarkers>::SecondInput>
        + NeoGetField<<R::T as TimeMarkers>::NanoSecondInput>
        + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneOffsetInput>
        + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneIdInput>
        + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneMetazoneInput>
        + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneVariantInput>,
{
}

/// A struct implementing traits for never loading data.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty marker enum
pub enum NeoNeverMarker {}

impl private::Sealed for NeoNeverMarker {}

impl DateInputMarkers for NeoNeverMarker {
    type YearInput = NeverField;
    type MonthInput = NeverField;
    type DayOfMonthInput = NeverField;
    type DayOfWeekInput = NeverField;
    type DayOfYearInput = NeverField;
    type AnyCalendarKindInput = NeverField;
}

impl<C> TypedDateDataMarkers<C> for NeoNeverMarker {
    type DateSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type YearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type MonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

impl DateDataMarkers for NeoNeverMarker {
    type Skel = NoDataCalMarkers;
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

impl ZoneMarkers for NeoNeverMarker {
    type TimeZoneOffsetInput = NeverField;
    type TimeZoneIdInput = NeverField;
    type TimeZoneMetazoneInput = NeverField;
    type TimeZoneVariantInput = NeverField;
    type EssentialsV1Marker = NeverMarker<tz::EssentialsV1<'static>>;
    type ExemplarCitiesV1Marker = NeverMarker<tz::ExemplarCitiesV1<'static>>;
    type GenericLongV1Marker = NeverMarker<tz::MzGenericV1<'static>>;
    type GenericShortV1Marker = NeverMarker<tz::MzGenericV1<'static>>;
    type SpecificLongV1Marker = NeverMarker<tz::MzSpecificV1<'static>>;
    type SpecificShortV1Marker = NeverMarker<tz::MzSpecificV1<'static>>;
}

/// A struct that supports formatting both a date and a time.
///
/// It should be composed from types implementing [`HasConstDayComponents`]
/// and [`HasConstTimeComponents`].
#[derive(Debug)]
pub struct DateTimeCombo<D, T, Z> {
    _d: PhantomData<D>,
    _t: PhantomData<T>,
    _z: PhantomData<Z>,
}

impl<D, T, Z> private::Sealed for DateTimeCombo<D, T, Z> {}

impl<D> DateTimeNamesMarker for DateTimeCombo<D, NeoNeverMarker, NeoNeverMarker>
where
    D: DateTimeNamesMarker,
{
    type YearNames = D::YearNames;
    type MonthNames = D::MonthNames;
    type WeekdayNames = D::WeekdayNames;
    type DayPeriodNames = NeverMarker<()>;
    type ZoneEssentials = NeverMarker<()>;
    type ZoneExemplarCities = NeverMarker<()>;
    type ZoneGenericLong = NeverMarker<()>;
    type ZoneGenericShort = NeverMarker<()>;
    type ZoneSpecificLong = NeverMarker<()>;
    type ZoneSpecificShort = NeverMarker<()>;
}

impl<D> HasConstComponents for DateTimeCombo<D, NeoNeverMarker, NeoNeverMarker>
where
    D: HasConstDateComponents,
{
    const COMPONENTS: NeoComponents = NeoComponents::Date(D::COMPONENTS);
}

impl<D> DateTimeMarkers for DateTimeCombo<D, NeoNeverMarker, NeoNeverMarker>
where
    D: DateTimeMarkers,
{
    type D = D;
    type T = NeoNeverMarker;
    type Z = NeoNeverMarker;
    type LengthOption = NeoSkeletonLength; // always needed for date
    type AlignmentOption = D::AlignmentOption;
    type EraDisplayOption = D::EraDisplayOption;
    type FractionalSecondDigitsOption = NeverField;
    type GluePatternV1Marker = NeverMarker<GluePatternV1<'static>>;
}

impl<T> DateTimeNamesMarker for DateTimeCombo<NeoNeverMarker, T, NeoNeverMarker>
where
    T: DateTimeNamesMarker,
{
    type YearNames = NeverMarker<()>;
    type MonthNames = NeverMarker<()>;
    type WeekdayNames = NeverMarker<()>;
    type DayPeriodNames = T::DayPeriodNames;
    type ZoneEssentials = NeverMarker<()>;
    type ZoneExemplarCities = NeverMarker<()>;
    type ZoneGenericLong = NeverMarker<()>;
    type ZoneGenericShort = NeverMarker<()>;
    type ZoneSpecificLong = NeverMarker<()>;
    type ZoneSpecificShort = NeverMarker<()>;
}

impl<T> HasConstComponents for DateTimeCombo<NeoNeverMarker, T, NeoNeverMarker>
where
    T: HasConstTimeComponents,
{
    const COMPONENTS: NeoComponents = NeoComponents::Time(T::COMPONENTS);
}

impl<T> DateTimeMarkers for DateTimeCombo<NeoNeverMarker, T, NeoNeverMarker>
where
    T: DateTimeMarkers,
{
    type D = NeoNeverMarker;
    type T = T;
    type Z = NeoNeverMarker;
    type LengthOption = NeoSkeletonLength; // always needed for time
    type AlignmentOption = Option<Alignment>; // always needed for time
    type EraDisplayOption = NeverField; // no year in a time-only format
    type FractionalSecondDigitsOption = T::FractionalSecondDigitsOption;
    type GluePatternV1Marker = NeverMarker<GluePatternV1<'static>>;
}

impl<Z> DateTimeNamesMarker for DateTimeCombo<NeoNeverMarker, NeoNeverMarker, Z>
where
    Z: DateTimeNamesMarker,
{
    type YearNames = NeverMarker<()>;
    type MonthNames = NeverMarker<()>;
    type WeekdayNames = NeverMarker<()>;
    type DayPeriodNames = NeverMarker<()>;
    type ZoneEssentials = Z::ZoneEssentials;
    type ZoneExemplarCities = Z::ZoneExemplarCities;
    type ZoneGenericLong = Z::ZoneGenericLong;
    type ZoneGenericShort = Z::ZoneGenericShort;
    type ZoneSpecificLong = Z::ZoneSpecificLong;
    type ZoneSpecificShort = Z::ZoneSpecificShort;
}

impl<Z> HasConstComponents for DateTimeCombo<NeoNeverMarker, NeoNeverMarker, Z>
where
    Z: HasConstZoneComponent,
{
    const COMPONENTS: NeoComponents = NeoComponents::Zone(Z::COMPONENT);
}

impl<Z> DateTimeMarkers for DateTimeCombo<NeoNeverMarker, NeoNeverMarker, Z>
where
    Z: DateTimeMarkers,
{
    type D = NeoNeverMarker;
    type T = NeoNeverMarker;
    type Z = Z;
    type LengthOption = Z::LengthOption; // no date or time: inherit from zone
    type AlignmentOption = Z::AlignmentOption; // no date or time: inherit from zone
    type EraDisplayOption = NeverField; // no year in a zone-only format
    type FractionalSecondDigitsOption = NeverField;
    type GluePatternV1Marker = GluePatternV1Marker;
}

impl<D, T> DateTimeNamesMarker for DateTimeCombo<D, T, NeoNeverMarker>
where
    D: DateTimeNamesMarker,
    T: DateTimeNamesMarker,
{
    type YearNames = D::YearNames;
    type MonthNames = D::MonthNames;
    type WeekdayNames = D::WeekdayNames;
    type DayPeriodNames = T::DayPeriodNames;
    type ZoneEssentials = NeverMarker<()>;
    type ZoneExemplarCities = NeverMarker<()>;
    type ZoneGenericLong = NeverMarker<()>;
    type ZoneGenericShort = NeverMarker<()>;
    type ZoneSpecificLong = NeverMarker<()>;
    type ZoneSpecificShort = NeverMarker<()>;
}

impl<D, T> HasConstComponents for DateTimeCombo<D, T, NeoNeverMarker>
where
    D: HasConstDayComponents,
    T: HasConstTimeComponents,
{
    const COMPONENTS: NeoComponents = NeoComponents::DateTime(D::COMPONENTS, T::COMPONENTS);
}

impl<D, T> DateTimeMarkers for DateTimeCombo<D, T, NeoNeverMarker>
where
    D: DateTimeMarkers,
    T: DateTimeMarkers,
{
    type D = D;
    type T = T;
    type Z = NeoNeverMarker;
    type LengthOption = NeoSkeletonLength; // always needed for date/time
    type AlignmentOption = Option<Alignment>; // always needed for date/time
    type EraDisplayOption = D::EraDisplayOption;
    type FractionalSecondDigitsOption = T::FractionalSecondDigitsOption;
    type GluePatternV1Marker = GluePatternV1Marker;
}

impl<D, T, Z> DateTimeNamesMarker for DateTimeCombo<D, T, Z>
where
    D: DateTimeNamesMarker,
    T: DateTimeNamesMarker,
    Z: DateTimeNamesMarker,
{
    type YearNames = D::YearNames;
    type MonthNames = D::MonthNames;
    type WeekdayNames = D::WeekdayNames;
    type DayPeriodNames = T::DayPeriodNames;
    type ZoneEssentials = Z::ZoneEssentials;
    type ZoneExemplarCities = Z::ZoneExemplarCities;
    type ZoneGenericLong = Z::ZoneGenericLong;
    type ZoneGenericShort = Z::ZoneGenericShort;
    type ZoneSpecificLong = Z::ZoneSpecificLong;
    type ZoneSpecificShort = Z::ZoneSpecificShort;
}

impl<D, T, Z> HasConstComponents for DateTimeCombo<D, T, Z>
where
    D: HasConstDayComponents,
    T: HasConstTimeComponents,
    Z: HasConstZoneComponent,
{
    const COMPONENTS: NeoComponents =
        NeoComponents::DateTimeZone(D::COMPONENTS, T::COMPONENTS, Z::COMPONENT);
}

impl<D, T, Z> DateTimeMarkers for DateTimeCombo<D, T, Z>
where
    D: DateTimeMarkers,
    T: DateTimeMarkers,
    Z: DateTimeMarkers,
{
    type D = D;
    type T = T;
    type Z = Z;
    type LengthOption = NeoSkeletonLength; // always needed for date/time
    type AlignmentOption = Option<Alignment>; // always needed for date/time
    type EraDisplayOption = D::EraDisplayOption;
    type FractionalSecondDigitsOption = T::FractionalSecondDigitsOption;
    type GluePatternV1Marker = GluePatternV1Marker;
}

// TODO: Fill in the missing DateTimeCombos, like DZ and TZ

macro_rules! datetime_marker_helper {
    (@years/typed, yes) => {
        C::YearNamesV1Marker
    };
    (@years/typed,) => {
        NeverMarker<YearNamesV1<'static>>
    };
    (@months/typed, yes) => {
        C::MonthNamesV1Marker
    };
    (@months/typed,) => {
        NeverMarker<MonthNamesV1<'static>>
    };
    (@dates/typed, yes) => {
        C::SkeletaV1Marker
    };
    (@dates/typed,) => {
        NeverMarker<PackedSkeletonDataV1<'static>>
    };
    (@calmarkers, yes) => {
        FullDataCalMarkers
    };
    (@calmarkers,) => {
        NoDataCalMarkers
    };
    (@weekdays, yes) => {
        WeekdayNamesV1Marker
    };
    (@weekdays,) => {
        NeverMarker<LinearNamesV1<'static>>
    };
    (@dayperiods, yes) => {
        DayPeriodNamesV1Marker
    };
    (@dayperiods,) => {
        NeverMarker<LinearNamesV1<'static>>
    };
    (@times, yes) => {
        TimeNeoSkeletonPatternsV1Marker
    };
    (@times,) => {
        NeverMarker<PackedSkeletonDataV1<'static>>
    };
    (@glue, yes) => {
        GluePatternV1Marker
    };
    (@glue,) => {
        NeverMarker<GluePatternV1<'static>>
    };
    (@option/length, yes) => {
        NeoSkeletonLength
    };
    (@option/length, Long) => {
        NeoSkeletonLength
    };
    (@option/length, Medium) => {
        NeoSkeletonLength
    };
    (@option/length, Short) => {
        NeoSkeletonLength
    };
    (@option/eradisplay, yes) => {
        Option<EraDisplay>
    };
    (@option/alignment, yes) => {
        Option<Alignment>
    };
    (@option/fractionalsecondigits, yes) => {
        Option<FractionalSecondDigits>
    };
    (@option/$any:ident,) => {
        NeverField
    };
    (@input/year, yes) => {
        YearInfo
    };
    (@input/month, yes) => {
        MonthInfo
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
    (@input/timezone/offset, yes) => {
        Option<UtcOffset>
    };
    (@input/timezone/id, yes) => {
        Option<TimeZoneBcp47Id>
    };
    (@input/timezone/metazone, yes) => {
        Option<MetazoneId>
    };
    (@input/timezone/variant, yes) => {
        Option<ZoneVariant>
    };
    (@input/$any:ident,) => {
        NeverField
    };
    (@data/zone/essentials, yes) => {
        tz::EssentialsV1Marker
    };
    (@data/zone/exemplar_cities, yes) => {
        tz::ExemplarCitiesV1Marker
    };
    (@data/zone/generic_long, yes) => {
        tz::MzGenericLongV1Marker
    };
    (@data/zone/generic_short, yes) => {
        tz::MzGenericShortV1Marker
    };
    (@data/zone/specific_long, yes) => {
        tz::MzSpecificLongV1Marker
    };
    (@data/zone/specific_short, yes) => {
        tz::MzSpecificShortV1Marker
    };
    (@data/zone/essentials,) => {
        NeverMarker<tz::EssentialsV1<'static>>
    };
    (@data/zone/exemplar_cities,) => {
        NeverMarker<tz::ExemplarCitiesV1<'static>>
    };
    (@data/zone/generic_long,) => {
        NeverMarker<tz::MzGenericV1<'static>>
    };
    (@data/zone/generic_short,) => {
        NeverMarker<tz::MzGenericV1<'static>>
    };
    (@data/zone/specific_long,) => {
        NeverMarker<tz::MzSpecificV1<'static>>
    };
    (@data/zone/specific_short,) => {
        NeverMarker<tz::MzSpecificV1<'static>>
    };
    (@names/year, yes) => {
        YearNamesV1Marker
    };
    (@names/month, yes) => {
        MonthNamesV1Marker
    };
    (@names/weekday, yes) => {
        WeekdayNamesV1Marker
    };
    (@names/dayperiod, yes) => {
        DayPeriodNamesV1Marker
    };
    (@names/zone/essentials, yes) => {
        tz::EssentialsV1Marker
    };
    (@names/zone/exemplar_cities, yes) => {
        tz::ExemplarCitiesV1Marker
    };
    (@names/zone/generic_long, yes) => {
        tz::MzGenericLongV1Marker
    };
    (@names/zone/generic_short, yes) => {
        tz::MzGenericShortV1Marker
    };
    (@names/zone/specific_long, yes) => {
        tz::MzSpecificLongV1Marker
    };
    (@names/zone/specific_short, yes) => {
        tz::MzSpecificShortV1Marker
    };
    (@names/$any:ident,) => {
        NeverMarker<()>
    };
    (@names/$any:ident,) => {
        NeverMarker<()>
    };
    (@names/zone/$any:ident,) => {
        NeverMarker<()>
    };
    () => {
        unreachable!() // prevent bugs
    };
}

/// Maps the token `yes` to the given ident
macro_rules! yes_to {
    ($any:ident, yes) => {
        $any
    };
    () => {
        unreachable!() // prevent bugs
    };
}

/// Generates the options argument passed into the docs test constructor
macro_rules! length_option_helper {
    () => {
        stringify!(Default::default())
    };
    ($length:ident) => {
        concat!("NeoSkeletonLength::", stringify!($length), ".into()")
    };
}

/// Generates an impl of [`NeoGetField`]
macro_rules! impl_get_field {
    ($type:path, never) => {
        impl NeoGetField<NeverField> for $type {
            fn get_field(&self) -> NeverField {
                NeverField
            }
        }
    };
    ($type:path, length, yes) => {
        impl NeoGetField<NeoSkeletonLength> for $type {
            fn get_field(&self) -> NeoSkeletonLength {
                self.length
            }
        }
    };
    ($type:path, alignment, yes) => {
        impl NeoGetField<Option<Alignment>> for $type {
            fn get_field(&self) -> Option<Alignment> {
                self.alignment
            }
        }
    };
    ($type:path, era_display, yes) => {
        impl NeoGetField<Option<EraDisplay>> for $type {
            fn get_field(&self) -> Option<EraDisplay> {
                self.era_display
            }
        }
    };
    ($type:path, fractional_second_digits, yes) => {
        impl NeoGetField<Option<FractionalSecondDigits>> for $type {
            fn get_field(&self) -> Option<FractionalSecondDigits> {
                self.fractional_second_digits
            }
        }
    };
}

macro_rules! impl_marker_with_options {
    (
        $(#[$attr:meta])*
        $type:ident,
        $(sample_length: $sample_length:ident,)?
        $(alignment: $alignment_yes:ident,)?
        $(era_display: $eradisplay_yes:ident,)?
        $(fractional_second_digits: $fractionalsecondigits_yes:ident,)?
    ) => {
        $(#[$attr])*
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct $type {
            $(
                /// The desired length of the formatted string.
                ///
                /// See: [`NeoSkeletonLength`]
                pub length: datetime_marker_helper!(@option/length, $sample_length),
            )?
            $(
                /// Whether fields should be aligned for a column-like layout.
                ///
                /// See: [`Alignment`]
                pub alignment: datetime_marker_helper!(@option/alignment, $alignment_yes),
            )?
            $(
                /// When to display the era field in the formatted string.
                ///
                /// See: [`EraDisplay`]
                pub era_display: datetime_marker_helper!(@option/eradisplay, $eradisplay_yes),
            )?
            $(
                /// How many fractional seconds to display.
                ///
                /// See: [`FractionalSecondDigits`]
                pub fractional_second_digits: datetime_marker_helper!(@option/fractionalsecondigits, $fractionalsecondigits_yes),
            )?
        }
        impl $type {
            #[doc = concat!("Creates a ", stringify!($type), " skeleton with the given formatting length.")]
            pub const fn with_length(length: NeoSkeletonLength) -> Self {
                Self {
                    length,
                    $(
                        alignment: yes_to!(None, $alignment_yes),
                    )?
                    $(
                        era_display: yes_to!(None, $eradisplay_yes),
                    )?
                    $(
                        fractional_second_digits: yes_to!(None, $fractionalsecondigits_yes),
                    )?
                }
            }
        }
        impl_get_field!($type, never);
        $(
            impl_get_field!($type, length, $alignment_yes);
        )?
        $(
            impl_get_field!($type, alignment, $alignment_yes);
        )?
        $(
            impl_get_field!($type, era_display, $eradisplay_yes);
        )?
        $(
            impl_get_field!($type, fractional_second_digits, $fractionalsecondigits_yes);
        )?
    };
}

/// Implements a field set of time fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// Documentation for each option is shown inline below.
macro_rules! impl_date_marker {
    (
        // The name of the type being created.
        $type:ident,
        // An expression for the field set.
        $components:expr,
        // A plain language description of the field set for documentation.
        description = $description:literal,
        // Length of the sample string below.
        sample_length = $sample_length:ident,
        // A sample string. A docs test will be generated!
        sample = $sample:literal,
        // Whether years can occur.
        $(years = $years_yes:ident,)?
        // Whether months can occur.
        $(months = $months_yes:ident,)?
        // Whether weekdays can occur.
        $(weekdays = $weekdays_yes:ident,)?
        // Whether the input should contain years.
        $(input_year = $year_yes:ident,)?
        // Whether the input should contain months.
        $(input_month = $month_yes:ident,)?
        // Whether the input should contain the day of the month.
        $(input_day_of_month = $day_of_month_yes:ident,)?
        // Whether the input should contain the day of the week.
        $(input_day_of_week = $day_of_week_yes:ident,)?
        // Whether the input should contain the day of the year.
        $(input_day_of_year = $day_of_year_yes:ident,)?
        // Whether the input should declare its calendar kind.
        $(input_any_calendar_kind = $any_calendar_kind_yes:ident,)?
        // Whether the alignment option should be available.
        // According to UTS 35, it should be available with years, months, and days.
        $(option_alignment = $option_alignment_yes:ident,)?
    ) => {
        impl_marker_with_options!(
            #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
            ///
            /// # Examples
            ///
            /// In [`NeoFormatter`](crate::neo::NeoFormatter):
            ///
            /// ```
            /// use icu::calendar::Date;
            /// use icu::datetime::neo::NeoFormatter;
            #[doc = concat!("use icu::datetime::neo_marker::", stringify!($type), ";")]
            /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
            /// use icu::locale::locale;
            /// use writeable::assert_try_writeable_eq;
            #[doc = concat!("let fmt = NeoFormatter::<", stringify!($type), ">::try_new(")]
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = Date::try_new_iso_date(2024, 5, 17).unwrap();
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.convert_and_format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
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
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = Date::try_new_gregorian_date(2024, 5, 17).unwrap();
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            $type,
            sample_length: $sample_length,
            $(alignment: $option_alignment_yes,)?
            $(era_display: $year_yes,)?
        );
        impl private::Sealed for $type {}
        impl DateTimeNamesMarker for $type {
            type YearNames = datetime_marker_helper!(@names/year, $($years_yes)?);
            type MonthNames = datetime_marker_helper!(@names/month, $($months_yes)?);
            type WeekdayNames = datetime_marker_helper!(@names/weekday, $($weekdays_yes)?);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
            type ZoneExemplarCities = datetime_marker_helper!(@names/zone/exemplar_cities,);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
        }
        impl HasConstDateComponents for $type {
            const COMPONENTS: NeoDateComponents = $components;
        }
        impl DateInputMarkers for $type {
            type YearInput = datetime_marker_helper!(@input/year, $($year_yes)?);
            type MonthInput = datetime_marker_helper!(@input/month, $($month_yes)?);
            type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month, $($day_of_month_yes)?);
            type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week, $($day_of_week_yes)?);
            type DayOfYearInput = datetime_marker_helper!(@input/day_of_year, $($day_of_year_yes)?);
            type AnyCalendarKindInput = datetime_marker_helper!(@input/any_calendar_kind, $($any_calendar_kind_yes)?);
        }
        impl<C: CldrCalendar> TypedDateDataMarkers<C> for $type {
            type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, yes);
            type YearNamesV1Marker = datetime_marker_helper!(@years/typed, $($years_yes)?);
            type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, $($months_yes)?);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $($weekdays_yes)?);
        }
        impl DateDataMarkers for $type {
            type Skel = datetime_marker_helper!(@calmarkers, yes);
            type Year = datetime_marker_helper!(@calmarkers, $($years_yes)?);
            type Month = datetime_marker_helper!(@calmarkers, $($months_yes)?);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $($weekdays_yes)?);
        }
        impl DateTimeMarkers for $type {
            type D = Self;
            type T = NeoNeverMarker;
            type Z = NeoNeverMarker;
            type LengthOption = datetime_marker_helper!(@option/length, $sample_length);
            type AlignmentOption = datetime_marker_helper!(@option/alignment, $($months_yes)?);
            type EraDisplayOption = datetime_marker_helper!(@option/eradisplay, $($year_yes)?);
            type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
            type GluePatternV1Marker = datetime_marker_helper!(@glue,);
        }
        impl HasConstComponents for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Date($components);
        }
    };
}

/// Implements a field set of date fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// See [`impl_date_marker`].
macro_rules! impl_day_marker {
    (
        $type:ident,
        $components:expr,
        description = $description:literal,
        sample_length = $sample_length:ident,
        sample = $sample:literal,
        $(years = $years_yes:ident,)?
        $(months = $months_yes:ident,)?
        $(dates = $dates_yes:ident,)?
        $(weekdays = $weekdays_yes:ident,)?
        $(input_year = $year_yes:ident,)?
        $(input_month = $month_yes:ident,)?
        $(input_day_of_month = $day_of_month_yes:ident,)?
        $(input_day_of_week = $day_of_week_yes:ident,)?
        $(input_day_of_year = $day_of_year_yes:ident,)?
        $(input_any_calendar_kind = $any_calendar_kind_yes:ident,)?
        $(option_alignment = $option_alignment_yes:ident,)?
    ) => {
        impl_date_marker!(
            $type,
            NeoDateComponents::Day($components),
            description = $description,
            sample_length = $sample_length,
            sample = $sample,
            $(years = $years_yes,)?
            $(months = $months_yes,)?
            $(dates = $dates_yes,)?
            $(weekdays = $weekdays_yes,)?
            $(input_year = $year_yes,)?
            $(input_month = $month_yes,)?
            $(input_day_of_month = $day_of_month_yes,)?
            $(input_day_of_week = $day_of_week_yes,)?
            $(input_day_of_year = $day_of_year_yes,)?
            $(input_any_calendar_kind = $any_calendar_kind_yes,)?
            $(option_alignment = $option_alignment_yes,)?
        );
        impl HasConstDayComponents for $type {
            const COMPONENTS: NeoDayComponents = $components;
        }
    };
}

/// Implements a field set of time fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// Documentation for each option is shown inline below.
macro_rules! impl_time_marker {
    (
        // The name of the type being created.
        $type:ident,
        // An expression for the field set.
        $components:expr,
        // A plain language description of the field set for documentation.
        description = $description:literal,
        // Length of the sample string below.
        sample_length = $sample_length:ident,
        // A sample string. A docs test will be generated!
        sample = $sample:literal,
        // Whether day periods can occur.
        $(dayperiods = $dayperiods_yes:ident,)?
        // Whether the input should include hours.
        $(input_hour = $hour_yes:ident,)?
        // Whether the input should contain minutes.
        $(input_minute = $minute_yes:ident,)?
        // Whether the input should contain seconds.
        $(input_second = $second_yes:ident,)?
        // Whether the input should contain fractional seconds.
        $(input_nanosecond = $nanosecond_yes:ident,)?
    ) => {
        impl_marker_with_options!(
            #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
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
            ///
            #[doc = concat!("let fmt = NeoFormatter::<", stringify!($type), ">::try_new(")]
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = DateTime::try_new_iso_datetime(2024, 5, 17, 15, 47, 50).unwrap();
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.convert_and_format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
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
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = Time::try_new(15, 47, 50, 0).unwrap();
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            $type,
            sample_length: $sample_length,
            alignment: yes,
            $(fractional_second_digits: $nanosecond_yes,)?
        );
        impl private::Sealed for $type {}
        impl DateTimeNamesMarker for $type {
            type YearNames = datetime_marker_helper!(@names/year,);
            type MonthNames = datetime_marker_helper!(@names/month,);
            type WeekdayNames = datetime_marker_helper!(@names/weekday,);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, $($dayperiods_yes)?);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
            type ZoneExemplarCities = datetime_marker_helper!(@names/zone/exemplar_cities,);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
        }
        impl HasConstTimeComponents for $type {
            const COMPONENTS: NeoTimeComponents = $components;
        }
        impl TimeMarkers for $type {
            type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, $($dayperiods_yes)?);
            type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, yes);
            type HourInput = datetime_marker_helper!(@input/hour, $($hour_yes)?);
            type MinuteInput = datetime_marker_helper!(@input/minute, $($minute_yes)?);
            type SecondInput = datetime_marker_helper!(@input/second, $($second_yes)?);
            type NanoSecondInput = datetime_marker_helper!(@input/nanosecond, $($nanosecond_yes)?);
        }
        impl DateTimeMarkers for $type {
            type D = NeoNeverMarker;
            type T = Self;
            type Z = NeoNeverMarker;
            type LengthOption = datetime_marker_helper!(@option/length, $sample_length);
            type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
            type EraDisplayOption = datetime_marker_helper!(@option/eradisplay,);
            type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits, $($nanosecond_yes)?);
            type GluePatternV1Marker = datetime_marker_helper!(@glue,);
        }
        impl HasConstComponents for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Time($components);
        }
    };
}

/// Implements a field set of time zone fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// Documentation for each option is shown inline below.
macro_rules! impl_zone_marker {
    (
        $(#[$attr:meta])*
        // The name of the type being created.
        $type:ident,
        // An expression for the field set.
        $components:expr,
        // A plain language description of the field set for documentation.
        description = $description:literal,
        // Length of the sample string below.
        sample_length = $sample_length:ident,
        // A sample string. A docs test will be generated!
        sample = $sample:literal,
        // Whether zone-essentials should be loaded.
        $(zone_essentials = $zone_essentials_yes:ident,)?
        // Whether exemplar cities can occur.
        $(zone_exemplar_cities = $zone_exemplar_cities_yes:ident,)?
        // Whether generic long formats can occur.
        $(zone_generic_long = $zone_generic_long_yes:ident,)?
        // Whether generic short formats can occur.
        $(zone_generic_short = $zone_generic_short_yes:ident,)?
        // Whether specific long formats can occur.
        $(zone_specific_long = $zone_specific_long_yes:ident,)?
        // Whether specific short formats can occur.
        $(zone_specific_short = $zone_specific_short_yes:ident,)?
    ) => {
        #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
        ///
        /// # Examples
        ///
        /// In [`NeoFormatter`](crate::neo::NeoFormatter):
        ///
        /// ```
        /// use icu::timezone::CustomTimeZone;
        /// use icu::datetime::neo::NeoFormatter;
        #[doc = concat!("use icu::datetime::neo_marker::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locale::locale;
        /// use tinystr::tinystr;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = NeoFormatter::<", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
        /// )
        /// .unwrap();
        ///
        /// // Time zone for America/Chicago in the summer
        /// let zone = CustomTimeZone::from_parts(
        ///     -40, // offset eighths of hour
        ///     tinystr!(8, "uschi"), // time zone ID
        ///     tinystr!(4, "amce"), // metazone ID
        ///     tinystr!(2, "dt"), // zone variant: daylight time
        /// );
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.convert_and_format(&zone),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        ///
        /// In [`TypedNeoFormatter`](crate::neo::TypedNeoFormatter):
        ///
        /// ```
        /// use icu::calendar::{Date, Time};
        /// use icu::timezone::{CustomTimeZone, CustomZonedDateTime};
        /// use icu::calendar::Gregorian;
        /// use icu::datetime::neo::TypedNeoFormatter;
        #[doc = concat!("use icu::datetime::neo_marker::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locale::locale;
        /// use tinystr::tinystr;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = TypedNeoFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
        /// )
        /// .unwrap();
        ///
        /// // Time zone for America/Chicago in the summer
        /// let zone = CustomTimeZone::from_parts(
        ///     -40, // offset eighths of hour
        ///     tinystr!(8, "uschi"), // time zone ID
        ///     tinystr!(4, "amce"), // metazone ID
        ///     tinystr!(2, "dt"), // zone variant: daylight time
        /// );
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.format(&zone),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        $(#[$attr])*
        #[derive(Debug)]
        #[allow(clippy::exhaustive_enums)] // empty enum
        pub enum $type {}
        impl private::Sealed for $type {}
        impl DateTimeNamesMarker for $type {
            type YearNames = datetime_marker_helper!(@names/year,);
            type MonthNames = datetime_marker_helper!(@names/month,);
            type WeekdayNames = datetime_marker_helper!(@names/weekday,);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials, $($zone_essentials_yes)?);
            type ZoneExemplarCities = datetime_marker_helper!(@names/zone/exemplar_cities, $($zone_exemplar_cities_yes)?);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long, $($zone_generic_long_yes)?);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short, $($zone_generic_short_yes)?);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long, $($zone_specific_long_yes)?);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short, $($zone_specific_short_yes)?);
        }
        impl HasConstZoneComponent for $type {
            const COMPONENT: NeoTimeZoneSkeleton = $components;
        }
        impl ZoneMarkers for $type {
            type TimeZoneOffsetInput = datetime_marker_helper!(@input/timezone/offset, yes);
            type TimeZoneIdInput = datetime_marker_helper!(@input/timezone/id, yes);
            type TimeZoneMetazoneInput = datetime_marker_helper!(@input/timezone/metazone, yes);
            type TimeZoneVariantInput = datetime_marker_helper!(@input/timezone/variant, yes);
            type EssentialsV1Marker = datetime_marker_helper!(@data/zone/essentials, $($zone_essentials_yes)?);
            type ExemplarCitiesV1Marker = datetime_marker_helper!(@data/zone/exemplar_cities, $($zone_exemplar_cities_yes)?);
            type GenericLongV1Marker = datetime_marker_helper!(@data/zone/generic_long, $($zone_generic_long_yes)?);
            type GenericShortV1Marker = datetime_marker_helper!(@data/zone/generic_short, $($zone_generic_short_yes)?);
            type SpecificLongV1Marker = datetime_marker_helper!(@data/zone/specific_long, $($zone_specific_long_yes)?);
            type SpecificShortV1Marker = datetime_marker_helper!(@data/zone/specific_short, $($zone_specific_short_yes)?);
        }
        impl DateTimeMarkers for $type {
            type D = NeoNeverMarker;
            type T = NeoNeverMarker;
            type Z = Self;
            type LengthOption = datetime_marker_helper!(@option/length, yes);
            type AlignmentOption = datetime_marker_helper!(@option/alignment,);
            type EraDisplayOption = datetime_marker_helper!(@option/eradisplay,);
            type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
            type GluePatternV1Marker = datetime_marker_helper!(@glue,);
        }
        impl HasConstComponents for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Zone($components);
        }
    };
}

macro_rules! impl_datetime_marker {
    (
        $type:ident,
        description = $description:literal,
        sample_length = $sample_length:ident,
        sample = $sample:literal,
        date = $date:path,
        time = $time:path,
    ) => {
        #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
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
        ///
        #[doc = concat!("let fmt = NeoFormatter::<", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
        /// )
        /// .unwrap();
        /// let dt = DateTime::try_new_iso_datetime(2024, 5, 17, 15, 47, 50).unwrap();
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.convert_and_format(&dt),
        #[doc = concat!("    \"", $sample, "\"")]
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
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = TypedNeoFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
        /// )
        /// .unwrap();
        /// let dt = DateTime::try_new_gregorian_datetime(2024, 5, 17, 15, 47, 50).unwrap();
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.format(&dt),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        pub type $type = DateTimeCombo<$date, $time, NeoNeverMarker>;
    }
}

macro_rules! impl_zoneddatetime_marker {
    (
        $type:ident,
        description = $description:literal,
        sample_length = $sample_length:ident,
        sample = $sample:literal,
        date = $date:path,
        time = $time:path,
        zone = $zone:path,
    ) => {
        #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
        ///
        /// # Examples
        ///
        /// In [`NeoFormatter`](crate::neo::NeoFormatter):
        ///
        /// ```
        /// use icu::calendar::{Date, Time};
        /// use icu::timezone::{CustomTimeZone, CustomZonedDateTime};
        /// use icu::datetime::neo::NeoFormatter;
        #[doc = concat!("use icu::datetime::neo_marker::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = NeoFormatter::<", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
        /// )
        /// .unwrap();
        ///
        /// let mut dtz = CustomZonedDateTime::try_from_str("2024-05-17T15:47:50+01:00[Europe/London]").unwrap();
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.convert_and_format(&dtz),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        ///
        /// In [`TypedNeoFormatter`](crate::neo::TypedNeoFormatter):
        ///
        /// ```
        /// use icu::calendar::{Date, Time};
        /// use icu::timezone::{CustomTimeZone, CustomZonedDateTime};
        /// use icu::calendar::Gregorian;
        /// use icu::datetime::neo::TypedNeoFormatter;
        #[doc = concat!("use icu::datetime::neo_marker::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = TypedNeoFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        #[doc = concat!("    ", length_option_helper!($sample_length), ",")]
        /// )
        /// .unwrap();
        ///
        /// let mut dtz = CustomZonedDateTime::try_from_str("2024-05-17T15:47:50+01:00[Europe/London]")
        ///     .unwrap()
        ///     .to_calendar(Gregorian);
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.format(&dtz),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        pub type $type = DateTimeCombo<$date, $time, $zone>;
    }
}

impl_day_marker!(
    NeoYearMonthDayMarker,
    NeoDayComponents::YearMonthDay,
    description = "year, month, and day (year might be abbreviated)",
    sample_length = Short,
    sample = "5/17/24",
    years = yes,
    months = yes,
    weekdays = yes,
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

// TODO: Rename this to FullYear instead of EraYear
impl_day_marker!(
    NeoEraYearMonthDayMarker,
    NeoDayComponents::EraYearMonthDay,
    description = "year, month, and day (year always with full precision)",
    sample_length = Medium,
    sample = "May 17, 2024",
    years = yes,
    months = yes,
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_day_marker!(
    NeoMonthDayMarker,
    NeoDayComponents::MonthDay,
    description = "month and day",
    sample_length = Medium,
    sample = "May 17",
    months = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_day_marker!(
    NeoAutoDateMarker,
    NeoDayComponents::Auto,
    description = "locale-dependent date fields",
    sample_length = Medium,
    sample = "May 17, 2024",
    years = yes,
    months = yes,
    weekdays = yes,
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_day_of_week = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_time_marker!(
    NeoHourMinuteMarker,
    NeoTimeComponents::HourMinute,
    description = "hour and minute (locale-dependent hour cycle)",
    sample_length = Medium,
    sample = "3:47 PM",
    dayperiods = yes,
    input_hour = yes,
    input_minute = yes,
);

impl_time_marker!(
    NeoHourMinuteSecondMarker,
    NeoTimeComponents::HourMinuteSecond,
    description = "hour, minute, and second (locale-dependent hour cycle)",
    sample_length = Medium,
    sample = "3:47:50 PM",
    dayperiods = yes,
    input_hour = yes,
    input_minute = yes,
    input_second = yes,
    input_nanosecond = yes,
);

impl_time_marker!(
    NeoAutoTimeMarker,
    NeoTimeComponents::Auto,
    description = "locale-dependent time fields",
    sample_length = Medium,
    sample = "3:47:50 PM",
    dayperiods = yes,
    input_hour = yes,
    input_minute = yes,
    input_second = yes,
    input_nanosecond = yes,
);

// TODO: Make NeoAutoZoneMarker, derived from time length patterns

impl_datetime_marker!(
    NeoAutoDateTimeMarker,
    description = "locale-dependent date and time fields",
    sample_length = Medium,
    sample = "May 17, 2024, 3:47:50 PM",
    date = NeoAutoDateMarker,
    time = NeoAutoTimeMarker,
);

// TODO: Type aliases like this are excessive; make a curated set
impl_datetime_marker!(
    NeoYearMonthDayHourMinuteMarker,
    description = "locale-dependent date and time fields",
    sample_length = Medium,
    sample = "May 17, 2024, 3:47 PM",
    date = NeoYearMonthDayMarker,
    time = NeoHourMinuteMarker,
);

impl_date_marker!(
    NeoYearMonthMarker,
    NeoDateComponents::YearMonth,
    description = "year and month (era elided when possible)",
    sample_length = Medium,
    sample = "May 2024",
    years = yes,
    months = yes,
    input_year = yes,
    input_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_zone_marker!(
    /// When a display name is unavailable, falls back to the offset format:
    ///
    /// ```
    /// use icu::calendar::{Date, Time};
    /// use icu::timezone::{CustomTimeZone, CustomZonedDateTime};
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_marker::NeoTimeZoneSpecificMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use tinystr::tinystr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = TypedNeoFormatter::<Gregorian, NeoTimeZoneSpecificMarker>::try_new(
    ///     &locale!("en").into(),
    ///     NeoSkeletonLength::Short.into(),
    /// )
    /// .unwrap();
    ///
    /// // Time zone for America/Sao_Paulo year-round
    /// let zone = CustomTimeZone::from_parts(
    ///     -24, // offset eighths of hour
    ///     tinystr!(8, "brsao"), // time zone ID
    ///     tinystr!(4, "bras"), // metazone ID
    ///     tinystr!(2, "st"), // zone variant: standard time
    /// );
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.format(&zone),
    ///     "GMT-3"
    /// );
    /// ```
    NeoTimeZoneSpecificMarker,
    NeoTimeZoneSkeleton::specific(),
    description = "specific time zone, or raw offset if unavailable",
    sample_length = Long,
    sample = "Central Daylight Time",
    zone_essentials = yes,
    zone_specific_long = yes,
    zone_specific_short = yes,
);

impl_zone_marker!(
    /// This marker only loads data for the short length. Useful when combined with other fields:
    ///
    /// ```
    /// use icu::calendar::{Date, Time};
    /// use icu::timezone::{CustomTimeZone, CustomZonedDateTime};
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_marker::NeoMonthDayMarker;
    /// use icu::datetime::neo_marker::NeoHourMinuteMarker;
    /// use icu::datetime::neo_marker::NeoTimeZoneSpecificShortMarker;
    /// use icu::datetime::neo_marker::DateTimeCombo;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use tinystr::tinystr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// type MyDateTimeZoneSet = DateTimeCombo<
    ///     NeoMonthDayMarker,
    ///     NeoHourMinuteMarker,
    ///     NeoTimeZoneSpecificShortMarker,
    /// >;
    ///
    /// let fmt = NeoFormatter::<MyDateTimeZoneSet>::try_new(
    ///     &locale!("en-US").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    ///
    /// let dtz = CustomZonedDateTime::try_from_str("2024-09-17T15:47:50-05:00[America/Chicago]").unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.convert_and_format(&dtz),
    ///     "September 17, 3:47 PM CDT"
    /// );
    /// ```
    ///
    /// Don't use long length if it is the only field:
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_marker::NeoTimeZoneSpecificShortMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::LoadError;
    /// use icu::locale::locale;
    ///
    /// let result = TypedNeoFormatter::<Gregorian, NeoTimeZoneSpecificShortMarker>::try_new(
    ///     &locale!("en").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// );
    ///
    /// assert!(matches!(result, Err(LoadError::TypeTooNarrow(_))));
    /// ```
    NeoTimeZoneSpecificShortMarker,
    NeoTimeZoneSkeleton::specific(),
    description = "specific time zone (only short), or raw offset if unavailable",
    sample_length = Short,
    sample = "CDT",
    zone_essentials = yes,
    zone_specific_short = yes,
);

impl_zone_marker!(
    NeoTimeZoneOffsetMarker,
    NeoTimeZoneSkeleton::offset(),
    description = "UTC offset time zone",
    sample_length = Medium,
    sample = "GMT-5",
    zone_essentials = yes,
);

impl_zone_marker!(
    /// When a display name is unavailable, falls back to the location format:
    ///
    /// ```
    /// use icu::calendar::{Date, Time};
    /// use icu::timezone::{CustomTimeZone, CustomZonedDateTime};
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_marker::NeoTimeZoneGenericMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use tinystr::tinystr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = TypedNeoFormatter::<Gregorian, NeoTimeZoneGenericMarker>::try_new(
    ///     &locale!("en").into(),
    ///     NeoSkeletonLength::Short.into(),
    /// )
    /// .unwrap();
    ///
    /// // Time zone for America/Sao_Paulo year-round
    /// let zone = CustomTimeZone::from_parts(
    ///     -24, // offset eighths of hour
    ///     tinystr!(8, "brsao"), // time zone ID
    ///     tinystr!(4, "bras"), // metazone ID
    ///     tinystr!(2, "st"), // zone variant: standard time
    /// );
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.format(&zone),
    ///     "Sao Paulo Time"
    /// );
    /// ```
    NeoTimeZoneGenericMarker,
    NeoTimeZoneSkeleton::generic(),
    description = "generic time zone, or location if unavailable",
    sample_length = Long,
    sample = "Central Time",
    zone_essentials = yes,
    zone_exemplar_cities = yes,
    zone_generic_long = yes,
    zone_generic_short = yes,
);

impl_zone_marker!(
    /// This marker only loads data for the short length. Useful when combined with other fields:
    ///
    /// ```
    /// use icu::calendar::{Date, Time};
    /// use icu::timezone::{CustomTimeZone, CustomZonedDateTime};
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_marker::NeoMonthDayMarker;
    /// use icu::datetime::neo_marker::NeoHourMinuteMarker;
    /// use icu::datetime::neo_marker::NeoTimeZoneGenericShortMarker;
    /// use icu::datetime::neo_marker::DateTimeCombo;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use tinystr::tinystr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// type MyDateTimeZoneSet = DateTimeCombo<
    ///     NeoMonthDayMarker,
    ///     NeoHourMinuteMarker,
    ///     NeoTimeZoneGenericShortMarker,
    /// >;
    ///
    /// let fmt = NeoFormatter::<MyDateTimeZoneSet>::try_new(
    ///     &locale!("en-US").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    ///
    /// let dtz = CustomZonedDateTime::try_from_str("2024-09-17T15:47:50-05:00[America/Chicago]").unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.convert_and_format(&dtz),
    ///     "September 17, 3:47 PM CT"
    /// );
    /// ```
    ///
    /// Don't use long length if it is the only field:
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_marker::NeoTimeZoneGenericShortMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::LoadError;
    /// use icu::locale::locale;
    ///
    /// let result = TypedNeoFormatter::<Gregorian, NeoTimeZoneGenericShortMarker>::try_new(
    ///     &locale!("en").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// );
    ///
    /// assert!(matches!(result, Err(LoadError::TypeTooNarrow(_))));
    /// ```
    NeoTimeZoneGenericShortMarker,
    NeoTimeZoneSkeleton::generic(),
    description = "generic time zone (only short), or location if unavailable",
    sample_length = Short,
    sample = "CT",
    zone_essentials = yes,
    zone_exemplar_cities = yes,
    zone_generic_short = yes,
);

impl_zone_marker!(
    NeoTimeZoneLocationMarker,
    NeoTimeZoneSkeleton::location(),
    description = "location time zone",
    sample_length = Long,
    sample = "Chicago Time",
    zone_essentials = yes,
    zone_exemplar_cities = yes,
);

// TODO: Type aliases like this are excessive; make a curated set
impl_zoneddatetime_marker!(
    NeoYearMonthDayHourMinuteSecondTimeZoneGenericShortMarker,
    description = "locale-dependent date and time fields with a time zone",
    sample_length = Medium,
    sample = "May 17, 2024, 3:47:50 PM GMT",
    date = NeoAutoDateMarker,
    time = NeoAutoTimeMarker,
    zone = NeoTimeZoneGenericMarker,
);

/// Trait for components that can be formatted at runtime.
pub trait IsRuntimeComponents: private::Sealed + Into<NeoComponents> {}

impl private::Sealed for NeoDateComponents {}

impl IsRuntimeComponents for NeoDateComponents {}

impl DateTimeNamesMarker for NeoDateComponents {
    type YearNames = datetime_marker_helper!(@names/year, yes);
    type MonthNames = datetime_marker_helper!(@names/month, yes);
    type WeekdayNames = datetime_marker_helper!(@names/weekday, yes);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
    type ZoneExemplarCities = datetime_marker_helper!(@names/zone/exemplar_cities,);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
}

impl DateInputMarkers for NeoDateComponents {
    type YearInput = datetime_marker_helper!(@input/year, yes);
    type MonthInput = datetime_marker_helper!(@input/month, yes);
    type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month, yes);
    type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week, yes);
    type DayOfYearInput = datetime_marker_helper!(@input/day_of_year, yes);
    type AnyCalendarKindInput = datetime_marker_helper!(@input/any_calendar_kind, yes);
}

impl<C: CldrCalendar> TypedDateDataMarkers<C> for NeoDateComponents {
    type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, yes);
    type YearNamesV1Marker = datetime_marker_helper!(@years/typed, yes);
    type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, yes);
}

impl DateDataMarkers for NeoDateComponents {
    type Skel = datetime_marker_helper!(@calmarkers, yes);
    type Year = datetime_marker_helper!(@calmarkers, yes);
    type Month = datetime_marker_helper!(@calmarkers, yes);
    type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, yes);
}

impl DateTimeMarkers for NeoDateComponents {
    type D = Self;
    type T = NeoNeverMarker;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type EraDisplayOption = datetime_marker_helper!(@option/eradisplay, yes);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

impl private::Sealed for NeoTimeComponents {}

impl IsRuntimeComponents for NeoTimeComponents {}

impl DateTimeNamesMarker for NeoTimeComponents {
    type YearNames = datetime_marker_helper!(@names/year,);
    type MonthNames = datetime_marker_helper!(@names/month,);
    type WeekdayNames = datetime_marker_helper!(@names/weekday,);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, yes);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
    type ZoneExemplarCities = datetime_marker_helper!(@names/zone/exemplar_cities,);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
}

impl TimeMarkers for NeoTimeComponents {
    type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, yes);
    type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, yes);
    type HourInput = datetime_marker_helper!(@input/hour, yes);
    type MinuteInput = datetime_marker_helper!(@input/minute, yes);
    type SecondInput = datetime_marker_helper!(@input/second, yes);
    type NanoSecondInput = datetime_marker_helper!(@input/nanosecond, yes);
}

impl DateTimeMarkers for NeoTimeComponents {
    type D = NeoNeverMarker;
    type T = Self;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type EraDisplayOption = datetime_marker_helper!(@option/eradisplay,);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits, yes);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

impl private::Sealed for NeoTimeZoneSkeleton {}

impl IsRuntimeComponents for NeoTimeZoneSkeleton {}

impl DateTimeNamesMarker for NeoTimeZoneSkeleton {
    type YearNames = datetime_marker_helper!(@names/year,);
    type MonthNames = datetime_marker_helper!(@names/month,);
    type WeekdayNames = datetime_marker_helper!(@names/weekday,);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials, yes);
    type ZoneExemplarCities = datetime_marker_helper!(@names/zone/exemplar_cities, yes);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long, yes);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short, yes);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long, yes);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short, yes);
}

impl ZoneMarkers for NeoTimeZoneSkeleton {
    type TimeZoneOffsetInput = datetime_marker_helper!(@input/timezone/offset, yes);
    type TimeZoneIdInput = datetime_marker_helper!(@input/timezone/id, yes);
    type TimeZoneMetazoneInput = datetime_marker_helper!(@input/timezone/metazone, yes);
    type TimeZoneVariantInput = datetime_marker_helper!(@input/timezone/variant, yes);
    type EssentialsV1Marker = datetime_marker_helper!(@data/zone/essentials, yes);
    type ExemplarCitiesV1Marker = datetime_marker_helper!(@data/zone/exemplar_cities, yes);
    type GenericLongV1Marker = datetime_marker_helper!(@data/zone/generic_long, yes);
    type GenericShortV1Marker = datetime_marker_helper!(@data/zone/generic_short, yes);
    type SpecificLongV1Marker = datetime_marker_helper!(@data/zone/specific_long, yes);
    type SpecificShortV1Marker = datetime_marker_helper!(@data/zone/specific_short, yes);
}

impl DateTimeMarkers for NeoTimeZoneSkeleton {
    type D = NeoNeverMarker;
    type T = NeoNeverMarker;
    type Z = Self;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment,);
    type EraDisplayOption = datetime_marker_helper!(@option/eradisplay,);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
    type GluePatternV1Marker = datetime_marker_helper!(@glue,);
}

impl private::Sealed for NeoDateTimeComponents {}

impl IsRuntimeComponents for NeoDateTimeComponents {}

impl DateTimeNamesMarker for NeoDateTimeComponents {
    type YearNames = datetime_marker_helper!(@names/year, yes);
    type MonthNames = datetime_marker_helper!(@names/month, yes);
    type WeekdayNames = datetime_marker_helper!(@names/weekday, yes);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, yes);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
    type ZoneExemplarCities = datetime_marker_helper!(@names/zone/exemplar_cities,);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
}

impl DateTimeMarkers for NeoDateTimeComponents {
    type D = NeoDateComponents;
    type T = NeoTimeComponents;
    type Z = NeoNeverMarker;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type EraDisplayOption = datetime_marker_helper!(@option/eradisplay, yes);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits, yes);
    type GluePatternV1Marker = datetime_marker_helper!(@glue, yes);
}

impl private::Sealed for NeoComponents {}

impl IsRuntimeComponents for NeoComponents {}

impl DateTimeNamesMarker for NeoComponents {
    type YearNames = datetime_marker_helper!(@names/year, yes);
    type MonthNames = datetime_marker_helper!(@names/month, yes);
    type WeekdayNames = datetime_marker_helper!(@names/weekday, yes);
    type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, yes);
    type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials, yes);
    type ZoneExemplarCities = datetime_marker_helper!(@names/zone/exemplar_cities, yes);
    type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long, yes);
    type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short, yes);
    type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long, yes);
    type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short, yes);
}

impl DateTimeMarkers for NeoComponents {
    type D = NeoDateComponents;
    type T = NeoTimeComponents;
    type Z = NeoTimeZoneSkeleton;
    type LengthOption = datetime_marker_helper!(@option/length, yes);
    type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
    type EraDisplayOption = datetime_marker_helper!(@option/eradisplay, yes);
    type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits, yes);
    type GluePatternV1Marker = datetime_marker_helper!(@glue, yes);
}
