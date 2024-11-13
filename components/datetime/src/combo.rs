// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use crate::{format::neo::*, neo_skeleton::*, provider::neo::*, scaffold::*};

/// Struct for combining date/time fields with zone fields.
///
/// This struct produces "composite field sets" as defined in UTS 35.
///
/// This struct does not have its own constructor, but it can be produced via
/// factory functions on the other field sets.
///
/// # Examples
///
/// Format the weekday, hour, and location-based zone:
///
/// ```
/// use icu::datetime::fieldset::{Combo, ET, L};
/// use icu::datetime::DateTimeFormatter;
/// use icu::locale::locale;
/// use icu::timezone::IxdtfParser;
/// use writeable::assert_try_writeable_eq;
///
/// // Note: Combo type can be elided, but it is shown here for demonstration
/// let formatter = DateTimeFormatter::<Combo<ET, L>>::try_new(
///     &locale!("en-US").into(),
///     ET::short().hm().l(),
/// )
/// .unwrap();
///
/// let zdt = IxdtfParser::new().try_location_only_from_str(
///     "2024-10-18T15:44[America/Los_Angeles]",
/// )
/// .unwrap();
///
/// assert_try_writeable_eq!(
///     formatter.convert_and_format(&zdt),
///     "Fri, 3:44 PM Los Angeles Time"
/// );
/// ```
///
/// Same thing with a fixed calendar formatter:
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::datetime::fieldset::{Combo, ET, L};
/// use icu::datetime::FixedCalendarDateTimeFormatter;
/// use icu::locale::locale;
/// use icu::timezone::IxdtfParser;
/// use writeable::assert_try_writeable_eq;
///
/// // Note: Combo type can be elided, but it is shown here for demonstration
/// let formatter = FixedCalendarDateTimeFormatter::<_, Combo<ET, L>>::try_new(
///     &locale!("en-US").into(),
///     ET::short().hm().l(),
/// )
/// .unwrap();
///
/// let zdt = IxdtfParser::new().try_location_only_iso_from_str(
///     "2024-10-18T15:44[America/Los_Angeles]",
/// )
/// .unwrap()
/// .to_calendar(Gregorian);
///
/// assert_try_writeable_eq!(
///     formatter.format(&zdt),
///     "Fri, 3:44 PM Los Angeles Time"
/// );
/// ```
///
/// Mix a dynamic [`DateFieldSet`](crate::fieldset::dynamic::DateFieldSet)
/// with a static time zone:
///
/// ```
/// use icu::datetime::fieldset::{Combo, YMD, Vs, dynamic::DateFieldSet};
/// use icu::datetime::DateTimeFormatter;
/// use icu::locale::locale;
/// use icu::timezone::IxdtfParser;
/// use writeable::assert_try_writeable_eq;
///
/// // Note: Combo type can be elided, but it is shown here for demonstration
/// let formatter = DateTimeFormatter::<Combo<DateFieldSet, Vs>>::try_new(
///     &locale!("en-US").into(),
///     DateFieldSet::YMD(YMD::long()).v(),
/// )
/// .unwrap();
///
/// let zdt = IxdtfParser::new().try_location_only_from_str(
///     "2024-10-18T15:44[America/Los_Angeles]",
/// )
/// .unwrap();
///
/// assert_try_writeable_eq!(
///     formatter.convert_and_format(&zdt),
///     "October 18, 2024 PT"
/// );
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Combo<DT, Z> {
    date_time_field_set: DT,
    _z: PhantomData<Z>,
}

impl<DT, Z> Combo<DT, Z> {
    #[inline]
    pub(crate) const fn new(date_time_field_set: DT) -> Self {
        Self {
            date_time_field_set,
            _z: PhantomData,
        }
    }
}

impl<DT, Z> UnstableSealed for Combo<DT, Z> {}

impl<DT, Z> Combo<DT, Z> {
    #[inline]
    pub(crate) fn dt(self) -> DT {
        self.date_time_field_set
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
    type LengthOption = NeoSkeletonLength; // always needed for date
    type AlignmentOption = DT::AlignmentOption;
    type YearStyleOption = DT::YearStyleOption;
    type TimePrecisionOption = DT::TimePrecisionOption;
    type GluePatternV1Marker = datetime_marker_helper!(@glue, yes);
}
