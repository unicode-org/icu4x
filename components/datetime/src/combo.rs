// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use crate::{format::neo::*, neo_skeleton::*, provider::neo::*, scaffold::*};

/// Struct for combining date, time, and zone fields.
///
/// This struct produces "composite field sets" as defined in UTS 35.
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
/// let field_set: Combo<ET, L> = ET::short().hm().l();
///
/// let formatter = DateTimeFormatter::try_new(
///     &locale!("en-US").into(),
///     field_set,
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
/// let field_set: Combo<ET, L> = ET::short().hm().l();
///
/// let formatter = FixedCalendarDateTimeFormatter::try_new(
///     &locale!("en-US").into(),
///     field_set,
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Combo<DT, Z> {
    date_time_field_set: DT,
    _z: PhantomData<Z>,
}

impl<DT, Z> Combo<DT, Z> {
    /// Creates a new [`Combo`] from a datetime field set.
    ///
    /// It may be more convenient to use the helper functions on the field set types.
    #[inline]
    pub const fn new(date_time_field_set: DT) -> Self {
        Self {
            date_time_field_set,
            _z: PhantomData,
        }
    }
}

impl<DT, Z> UnstableSealed for Combo<DT, Z> {}

impl<DT, Z> Combo<DT, Z> {
    /// Gets a mutable reference to the inner date/time field set.
    ///
    /// # Examples
    ///
    /// Override the length option:
    ///
    /// ```no_run
    /// use icu::datetime::fieldset::{self, Combo};
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    ///
    /// let mut field_set: Combo<fieldset::YMD, fieldset::L> = unimplemented!();
    ///
    /// field_set.as_mut_dt().length = NeoSkeletonLength::Medium;
    /// ```
    pub fn as_mut_dt(&mut self) -> &mut DT {
        &mut self.date_time_field_set
    }

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
    type D = DT;
    type T = DT;
    type Z = Z;
    type LengthOption = NeoSkeletonLength; // always needed for date
    type AlignmentOption = DT::AlignmentOption;
    type YearStyleOption = DT::YearStyleOption;
    type TimePrecisionOption = DT::TimePrecisionOption;
    type GluePatternV1Marker = datetime_marker_helper!(@glue, yes);
}
