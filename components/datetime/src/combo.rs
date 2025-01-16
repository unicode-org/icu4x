// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{provider::neo::*, scaffold::*};

/// Struct for combining date/time fields with zone fields.
///
/// This struct produces "composite field sets" as defined in UTS 35.
/// See [`fieldsets`](crate::fieldsets).
///
/// # Examples
///
/// Only one way to construct a combo field set (in this case, weekday with location-based zone):
///
/// ```
/// use icu::datetime::fieldsets::{Combo, E, L};
///
/// let field_set = E::long().with_zone_location();
/// ```
///
/// Format the weekday, hour, and location-based zone:
///
/// ```
/// use icu::datetime::fieldsets::{Combo, ET, L};
/// use icu::datetime::DateTimeFormatter;
/// use icu::locale::locale;
/// use icu::timezone::IxdtfParser;
/// use writeable::assert_writeable_eq;
///
/// // Note: Combo type can be elided, but it is shown here for demonstration
/// let formatter = DateTimeFormatter::<Combo<ET, L>>::try_new(
///     locale!("en-US").into(),
///     ET::short().hm().with_zone_location(),
/// )
/// .unwrap();
///
/// let zdt = IxdtfParser::new()
///     .try_location_only_from_str("2024-10-18T15:44[America/Los_Angeles]", formatter.calendar())
///     .unwrap();
///
/// assert_writeable_eq!(
///     formatter.format_any_calendar(&zdt),
///     "Fri, 3:44 PM Los Angeles Time"
/// );
/// ```
///
/// Same thing with a fixed calendar formatter:
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::datetime::fieldsets::{Combo, ET, L};
/// use icu::datetime::FixedCalendarDateTimeFormatter;
/// use icu::locale::locale;
/// use icu::timezone::{IxdtfParser, ZonedDateTime};
/// use writeable::assert_writeable_eq;
///
/// // Note: Combo type can be elided, but it is shown here for demonstration
/// let formatter = FixedCalendarDateTimeFormatter::<_, Combo<ET, L>>::try_new(
///     locale!("en-US").into(),
///     ET::short().hm().with_zone_location(),
/// )
/// .unwrap();
///
/// let zdt = IxdtfParser::new()
///     .try_location_only_from_str("2024-10-18T15:44[America/Los_Angeles]", Gregorian)
///     .unwrap();
///
/// assert_writeable_eq!(
///     formatter.format(&zdt),
///     "Fri, 3:44 PM Los Angeles Time"
/// );
/// ```
///
/// Mix a dynamic [`DateFieldSet`](crate::fieldsets::enums::DateFieldSet)
/// with a static time zone:
///
/// ```
/// use icu::datetime::fieldsets::{enums::DateFieldSet, Combo, Vs, YMD};
/// use icu::datetime::DateTimeFormatter;
/// use icu::locale::locale;
/// use icu::timezone::IxdtfParser;
/// use writeable::assert_writeable_eq;
///
/// // Note: Combo type can be elided, but it is shown here for demonstration
/// let formatter = DateTimeFormatter::<Combo<DateFieldSet, Vs>>::try_new(
///     locale!("en-US").into(),
///     DateFieldSet::YMD(YMD::long()).with_zone_generic(),
/// )
/// .unwrap();
///
/// let zdt = IxdtfParser::new()
///     .try_location_only_from_str("2024-10-18T15:44[America/Los_Angeles]", formatter.calendar())
///     .unwrap();
///
/// assert_writeable_eq!(
///     formatter.format_any_calendar(&zdt),
///     "October 18, 2024 PT"
/// );
/// ```
///
/// Format with a time of day and long time zone:
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::datetime::fieldsets::T;
/// use icu::datetime::FixedCalendarDateTimeFormatter;
/// use icu::locale::locale;
/// use icu::timezone::{IxdtfParser, ZonedDateTime};
/// use writeable::assert_writeable_eq;
///
/// let formatter = FixedCalendarDateTimeFormatter::try_new(
///     locale!("en-US").into(),
///     T::medium().with_zone_specific_long(),
/// )
/// .unwrap();
///
/// let zdt = IxdtfParser::new()
///     .try_from_str("2024-10-18T15:44-0700[America/Los_Angeles]", Gregorian)
///     .unwrap();
///
/// assert_writeable_eq!(
///     formatter.format(&zdt),
///     "3:44:00 PM Pacific Daylight Time"
/// );
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Combo<DT, Z> {
    date_time_field_set: DT,
    zone_field_set: Z,
}

impl<DT, Z> Combo<DT, Z> {
    #[inline]
    pub(crate) const fn new(date_time_field_set: DT, zone_field_set: Z) -> Self {
        Self {
            date_time_field_set,
            zone_field_set,
        }
    }
}

impl<DT, Z> Sealed for Combo<DT, Z> {}

impl<DT, Z> Combo<DT, Z> {
    #[inline]
    pub(crate) fn dt(self) -> DT {
        self.date_time_field_set
    }
    #[inline]
    pub(crate) fn z(self) -> Z {
        self.zone_field_set
    }
}

impl<DT, Z> DateTimeNamesMarker for Combo<DT, Z>
where
    DT: DateTimeNamesMarker,
    Z: DateTimeNamesMarker,
{
    type YearNames = DT::YearNames;
    type MonthNames = DT::MonthNames;
    type WeekdayNames = DT::WeekdayNames;
    type DayPeriodNames = DT::DayPeriodNames;
    type ZoneEssentials = Z::ZoneEssentials;
    type ZoneLocations = Z::ZoneLocations;
    type ZoneGenericLong = Z::ZoneGenericLong;
    type ZoneGenericShort = Z::ZoneGenericShort;
    type ZoneSpecificLong = Z::ZoneSpecificLong;
    type ZoneSpecificShort = Z::ZoneSpecificShort;
    type MetazoneLookup = Z::MetazoneLookup;
}

impl<DT, Z> DateTimeMarkers for Combo<DT, Z>
where
    DT: DateTimeMarkers,
    Z: DateTimeMarkers,
{
    type D = DT::D;
    type T = DT::T;
    type Z = Z::Z;
    type GluePatternV1Marker = datetime_marker_helper!(@glue, yes);
}
