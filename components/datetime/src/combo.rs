// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use crate::{format::neo::*, neo_skeleton::*, provider::neo::*, scaffold::*};
use icu_provider::marker::NeverMarker;

/// Struct for combining date, time, and zone fields.
///
/// This struct produces "composite field sets" as defined in UTS 35.
///
/// # Examples
///
/// Format the weekday, hour, and location-based zone:
///
/// ```
/// use icu::timezone::CustomZonedDateTime;
/// use icu::datetime::DateTimeFormatter;
/// use icu::datetime::fieldset::{Combo, E, HM, L};
/// use icu::datetime::neo_skeleton::NeoSkeletonLength;
/// use icu::locale::locale;
/// use writeable::assert_try_writeable_eq;
///
/// let formatter = DateTimeFormatter::try_new(
///     &locale!("en-US").into(),
///     Combo::<E, HM, L>::with_length(NeoSkeletonLength::Short)
/// )
/// .unwrap();
///
/// let zdt = CustomZonedDateTime::try_location_only_from_str("2024-10-18T15:44[America/Los_Angeles]").unwrap();
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
/// use icu::timezone::CustomZonedDateTime;
/// use icu::datetime::FixedCalendarDateTimeFormatter;
/// use icu::datetime::fieldset::{Combo, E, HM, L};
/// use icu::datetime::neo_skeleton::NeoSkeletonLength;
/// use icu::locale::locale;
/// use writeable::assert_try_writeable_eq;
///
/// let formatter = FixedCalendarDateTimeFormatter::try_new(
///     &locale!("en-US").into(),
///     Combo::<E, HM, L>::with_length(NeoSkeletonLength::Short)
/// )
/// .unwrap();
///
/// let zdt = CustomZonedDateTime::try_location_only_iso_from_str("2024-10-18T15:44[America/Los_Angeles]").unwrap().to_calendar(Gregorian);
///
/// assert_try_writeable_eq!(
///     formatter.format(&zdt),
///     "Fri, 3:44 PM Los Angeles Time"
/// );
/// ```
#[derive(Debug)]
pub struct Combo<D, T, Z> {
    _d: PhantomData<D>,
    _t: PhantomData<T>,
    _z: PhantomData<Z>,
    /// Desired formatting length.
    pub length: NeoSkeletonLength,
    /// Alignment option.
    pub alignment: Option<Alignment>,
    /// Era display option.
    pub year_style: Option<YearStyle>,
    /// Fractional second digits option.
    pub fractional_second_digits: Option<FractionalSecondDigits>,
}

impl<D, T, Z> UnstableSealed for Combo<D, T, Z> {}

impl<D, T, Z> Combo<D, T, Z> {
    /// Creates a date/time/zone skeleton with the given formatting length.
    pub fn with_length(length: NeoSkeletonLength) -> Self {
        Self {
            _d: PhantomData,
            _t: PhantomData,
            _z: PhantomData,
            length,
            alignment: None,
            year_style: None,
            fractional_second_digits: None,
        }
    }
}

impl_get_field!(<D, T, Z> Combo<D, T, Z>, never);
impl_get_field!(<D, T, Z> Combo<D, T, Z>, length, yes);
impl_get_field!(<D, T, Z> Combo<D, T, Z>, alignment, yes);
impl_get_field!(<D, T, Z> Combo<D, T, Z>, year_style, yes);
impl_get_field!(<D, T, Z> Combo<D, T, Z>, fractional_second_digits, yes);

impl<D> DateTimeNamesMarker for Combo<D, NeoNeverMarker, NeoNeverMarker>
where
    D: DateTimeNamesMarker,
{
    type YearNames = D::YearNames;
    type MonthNames = D::MonthNames;
    type WeekdayNames = D::WeekdayNames;
    type DayPeriodNames = NeverMarker<()>;
    type ZoneEssentials = NeverMarker<()>;
    type ZoneLocations = NeverMarker<()>;
    type ZoneGenericLong = NeverMarker<()>;
    type ZoneGenericShort = NeverMarker<()>;
    type ZoneSpecificLong = NeverMarker<()>;
    type ZoneSpecificShort = NeverMarker<()>;
    type MetazoneLookup = NeverMarker<()>;
}

impl<D> HasConstComponents for Combo<D, NeoNeverMarker, NeoNeverMarker>
where
    D: HasConstDateComponents,
{
    const COMPONENTS: NeoComponents = NeoComponents::Date(D::COMPONENTS);
}

impl<D> DateTimeMarkers for Combo<D, NeoNeverMarker, NeoNeverMarker>
where
    D: DateTimeMarkers,
{
    type D = D;
    type T = NeoNeverMarker;
    type Z = NeoNeverMarker;
    type LengthOption = NeoSkeletonLength; // always needed for date
    type AlignmentOption = D::AlignmentOption;
    type YearStyleOption = D::YearStyleOption;
    type FractionalSecondDigitsOption = ();
    type GluePatternV1Marker = NeverMarker<GluePatternV1<'static>>;
}

impl<T> DateTimeNamesMarker for Combo<NeoNeverMarker, T, NeoNeverMarker>
where
    T: DateTimeNamesMarker,
{
    type YearNames = NeverMarker<()>;
    type MonthNames = NeverMarker<()>;
    type WeekdayNames = NeverMarker<()>;
    type DayPeriodNames = T::DayPeriodNames;
    type ZoneEssentials = NeverMarker<()>;
    type ZoneLocations = NeverMarker<()>;
    type ZoneGenericLong = NeverMarker<()>;
    type ZoneGenericShort = NeverMarker<()>;
    type ZoneSpecificLong = NeverMarker<()>;
    type ZoneSpecificShort = NeverMarker<()>;
    type MetazoneLookup = NeverMarker<()>;
}

impl<T> HasConstComponents for Combo<NeoNeverMarker, T, NeoNeverMarker>
where
    T: HasConstTimeComponents,
{
    const COMPONENTS: NeoComponents = NeoComponents::Time(T::COMPONENTS);
}

impl<T> DateTimeMarkers for Combo<NeoNeverMarker, T, NeoNeverMarker>
where
    T: DateTimeMarkers,
{
    type D = NeoNeverMarker;
    type T = T;
    type Z = NeoNeverMarker;
    type LengthOption = NeoSkeletonLength; // always needed for time
    type AlignmentOption = Option<Alignment>; // always needed for time
    type YearStyleOption = (); // no year in a time-only format
    type FractionalSecondDigitsOption = T::FractionalSecondDigitsOption;
    type GluePatternV1Marker = NeverMarker<GluePatternV1<'static>>;
}

impl<Z> DateTimeNamesMarker for Combo<NeoNeverMarker, NeoNeverMarker, Z>
where
    Z: DateTimeNamesMarker,
{
    type YearNames = NeverMarker<()>;
    type MonthNames = NeverMarker<()>;
    type WeekdayNames = NeverMarker<()>;
    type DayPeriodNames = NeverMarker<()>;
    type ZoneEssentials = Z::ZoneEssentials;
    type ZoneLocations = Z::ZoneLocations;
    type ZoneGenericLong = Z::ZoneGenericLong;
    type ZoneGenericShort = Z::ZoneGenericShort;
    type ZoneSpecificLong = Z::ZoneSpecificLong;
    type ZoneSpecificShort = Z::ZoneSpecificShort;
    type MetazoneLookup = Z::MetazoneLookup;
}

impl<Z> HasConstComponents for Combo<NeoNeverMarker, NeoNeverMarker, Z>
where
    Z: HasConstZoneComponent,
{
    const COMPONENTS: NeoComponents = NeoComponents::Zone(Z::COMPONENT);
}

impl<Z> DateTimeMarkers for Combo<NeoNeverMarker, NeoNeverMarker, Z>
where
    Z: DateTimeMarkers,
{
    type D = NeoNeverMarker;
    type T = NeoNeverMarker;
    type Z = Z;
    type LengthOption = Z::LengthOption; // no date or time: inherit from zone
    type AlignmentOption = Z::AlignmentOption; // no date or time: inherit from zone
    type YearStyleOption = (); // no year in a zone-only format
    type FractionalSecondDigitsOption = ();
    type GluePatternV1Marker = GluePatternV1Marker;
}

impl<D, T> DateTimeNamesMarker for Combo<D, T, NeoNeverMarker>
where
    D: DateTimeNamesMarker,
    T: DateTimeNamesMarker,
{
    type YearNames = D::YearNames;
    type MonthNames = D::MonthNames;
    type WeekdayNames = D::WeekdayNames;
    type DayPeriodNames = T::DayPeriodNames;
    type ZoneEssentials = NeverMarker<()>;
    type ZoneLocations = NeverMarker<()>;
    type ZoneGenericLong = NeverMarker<()>;
    type ZoneGenericShort = NeverMarker<()>;
    type ZoneSpecificLong = NeverMarker<()>;
    type ZoneSpecificShort = NeverMarker<()>;
    type MetazoneLookup = NeverMarker<()>;
}

impl<D, T> HasConstComponents for Combo<D, T, NeoNeverMarker>
where
    D: HasConstDateComponents,
    T: HasConstTimeComponents,
{
    const COMPONENTS: NeoComponents = NeoComponents::DateTime(D::COMPONENTS, T::COMPONENTS);
}

impl<D, T> DateTimeMarkers for Combo<D, T, NeoNeverMarker>
where
    D: DateTimeMarkers,
    T: DateTimeMarkers,
{
    type D = D;
    type T = T;
    type Z = NeoNeverMarker;
    type LengthOption = NeoSkeletonLength; // always needed for date/time
    type AlignmentOption = Option<Alignment>; // always needed for date/time
    type YearStyleOption = D::YearStyleOption;
    type FractionalSecondDigitsOption = T::FractionalSecondDigitsOption;
    type GluePatternV1Marker = GluePatternV1Marker;
}

impl<D, T, Z> DateTimeNamesMarker for Combo<D, T, Z>
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
    type ZoneLocations = Z::ZoneLocations;
    type ZoneGenericLong = Z::ZoneGenericLong;
    type ZoneGenericShort = Z::ZoneGenericShort;
    type ZoneSpecificLong = Z::ZoneSpecificLong;
    type ZoneSpecificShort = Z::ZoneSpecificShort;
    type MetazoneLookup = Z::MetazoneLookup;
}

impl<D, T, Z> HasConstComponents for Combo<D, T, Z>
where
    D: HasConstDateComponents,
    T: HasConstTimeComponents,
    Z: HasConstZoneComponent,
{
    const COMPONENTS: NeoComponents =
        NeoComponents::DateTimeZone(D::COMPONENTS, T::COMPONENTS, Z::COMPONENT);
}

impl<D, T, Z> DateTimeMarkers for Combo<D, T, Z>
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
    type YearStyleOption = D::YearStyleOption;
    type FractionalSecondDigitsOption = T::FractionalSecondDigitsOption;
    type GluePatternV1Marker = GluePatternV1Marker;
}

// TODO: Fill in the missing Combos, like DZ and TZ
