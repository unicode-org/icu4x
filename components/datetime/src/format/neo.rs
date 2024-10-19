// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::datetime::{try_write_pattern_items, DateTimeWriteError};
use super::{
    GetNameForDayPeriodError, GetNameForMonthError, GetNameForWeekdayError, GetSymbolForEraError,
    MonthPlaceholderValue,
};
use crate::external_loaders::*;
use crate::fields::{self, Field, FieldLength, FieldSymbol};
use crate::helpers::size_test;
use crate::input;
use crate::input::ExtractedInput;
use crate::neo_pattern::{DateTimePattern, DateTimePatternBorrowed};
use crate::neo_skeleton::NeoDateTimeSkeleton;
use crate::pattern::PatternItem;
use crate::provider::neo::*;
use crate::provider::time_zones::tz;
use crate::scaffold::*;
use crate::scaffold::{
    AllInputMarkers, DateInputMarkers, DateTimeMarkers, GetField, IsInCalendar, NeoNeverMarker,
    TimeMarkers, TypedDateDataMarkers, ZoneMarkers,
};
use crate::time_zone::ResolvedNeoTimeZoneSkeleton;
use crate::time_zone::TimeZoneDataPayloadsBorrowed;
use core::fmt;
use core::marker::PhantomData;
use icu_calendar::types::Era;
use icu_calendar::types::MonthCode;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::options::GroupingStrategy;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_decimal::FixedDecimalFormatter;
use icu_provider::marker::NeverMarker;
use icu_provider::prelude::*;
use writeable::TryWriteable;
use yoke::Yokeable;

pub trait DateTimeNamesHolderTrait<M: DynamicDataMarker> {
    type Container<Variables: PartialEq + Copy + fmt::Debug>: MaybePayload2<M, Variables>
        + fmt::Debug;
}

impl<M: DynamicDataMarker> DateTimeNamesHolderTrait<M> for NeverMarker<()> {
    type Container<Variables: PartialEq + Copy + fmt::Debug> = ();
}

macro_rules! impl_holder_trait {
    ($marker:path) => {
        impl DateTimeNamesHolderTrait<$marker> for $marker {
            type Container<Variables: PartialEq + Copy + fmt::Debug> =
                DateTimeNamesData2<$marker, Variables>;
        }
    };
}

impl_holder_trait!(YearNamesV1Marker);
impl_holder_trait!(MonthNamesV1Marker);
impl_holder_trait!(WeekdayNamesV1Marker);
impl_holder_trait!(DayPeriodNamesV1Marker);
impl_holder_trait!(tz::EssentialsV1Marker);
impl_holder_trait!(tz::LocationsV1Marker);
impl_holder_trait!(tz::MzGenericLongV1Marker);
impl_holder_trait!(tz::MzGenericShortV1Marker);
impl_holder_trait!(tz::MzSpecificLongV1Marker);
impl_holder_trait!(tz::MzSpecificShortV1Marker);
impl_holder_trait!(tz::MzPeriodV1Marker);

#[non_exhaustive]
pub enum MaybePayloadError2 {
    TypeTooNarrow,
    InsufficientStorage,
}

impl MaybePayloadError2 {
    fn into_single_load_error(self, field: Field) -> SingleLoadError {
        match self {
            Self::TypeTooNarrow => SingleLoadError::TypeTooNarrow(field),
            Self::InsufficientStorage => SingleLoadError::DuplicateField(field),
        }
    }
}

pub trait MaybePayload2<M: DynamicDataMarker, Variables> {
    fn new_empty() -> Self;
    fn load_put<P>(
        &mut self,
        provider: &P,
        req: DataRequest,
        variables: Variables,
    ) -> Result<Result<(), DataError>, MaybePayloadError2>
    where
        P: BoundDataProvider<M> + ?Sized,
        Self: Sized;
    fn get(&self) -> DateTimeNamesData2Borrowed<M, Variables>;
}

pub struct DateTimeNamesData2<M: DynamicDataMarker, Variables> {
    inner: OptionalNames<Variables, DataPayload<M>>,
}

impl<M: DynamicDataMarker, Variables> fmt::Debug for DateTimeNamesData2<M, Variables>
where
    Variables: fmt::Debug,
    DataPayload<M>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<M: DynamicDataMarker, Variables> DateTimeNamesData2<M, Variables> {
    #[inline]
    pub(crate) fn none() -> Self {
        Self {
            inner: OptionalNames::None,
        }
    }
}

pub struct DateTimeNamesData2Borrowed<'data, M: DynamicDataMarker, Variables> {
    inner: OptionalNames<Variables, &'data <M::DataStruct as Yokeable<'data>>::Output>,
}

impl<M: DynamicDataMarker, Variables> MaybePayload2<M, Variables>
    for DateTimeNamesData2<M, Variables>
where
    Variables: PartialEq + Copy,
{
    #[inline]
    fn new_empty() -> Self {
        Self {
            inner: OptionalNames::None,
        }
    }
    fn load_put<P>(
        &mut self,
        provider: &P,
        req: DataRequest,
        variables: Variables,
    ) -> Result<Result<(), DataError>, MaybePayloadError2>
    where
        P: BoundDataProvider<M> + ?Sized,
        Self: Sized,
    {
        let arg_variables = variables;
        match &self.inner {
            OptionalNames::SingleLength { variables, .. } if arg_variables == *variables => {
                return Ok(Ok(()));
            }
            OptionalNames::SingleLength { .. } => {
                return Err(MaybePayloadError2::InsufficientStorage);
            }
            OptionalNames::None => (),
        };
        match provider.load_bound(req) {
            Ok(response) => {
                self.inner = OptionalNames::SingleLength {
                    payload: response.payload,
                    variables: arg_variables,
                };
                Ok(Ok(()))
            }
            Err(e) => Ok(Err(e)),
        }
    }
    #[inline]
    fn get(&self) -> DateTimeNamesData2Borrowed<M, Variables> {
        DateTimeNamesData2Borrowed {
            inner: self.inner.as_borrowed(),
        }
    }
}

impl<M: DynamicDataMarker, Variables> MaybePayload2<M, Variables> for () {
    #[inline]
    fn new_empty() -> Self {}
    #[inline]
    fn load_put<P>(
        &mut self,
        _: &P,
        _: DataRequest,
        _: Variables,
    ) -> Result<Result<(), DataError>, MaybePayloadError2>
    where
        P: BoundDataProvider<M> + ?Sized,
        Self: Sized,
    {
        Err(MaybePayloadError2::TypeTooNarrow)
    }
    #[allow(clippy::needless_lifetimes)] // Yokeable is involved
    #[inline]
    fn get(&self) -> DateTimeNamesData2Borrowed<M, Variables> {
        DateTimeNamesData2Borrowed {
            inner: OptionalNames::None,
        }
    }
}

/// This can be extended in the future to support multiple lengths.
/// For now, this type wraps a symbols object tagged with a single length. See #4337
#[derive(Debug, Copy, Clone)]
enum OptionalNames<Variables, Payload> {
    None,
    SingleLength {
        variables: Variables,
        payload: Payload,
    },
}

impl<Variables, Payload> OptionalNames<Variables, Payload>
where
    Variables: Copy + PartialEq,
    Payload: Copy,
{
    pub(crate) fn get_with_variables(&self, arg_variables: Variables) -> Option<Payload> {
        match self {
            Self::None => None,
            Self::SingleLength { variables, payload } if arg_variables == *variables => {
                Some(*payload)
            }
            _ => None,
        }
    }
}

impl<Payload> OptionalNames<(), Payload>
where
    Payload: Copy,
{
    pub(crate) fn get_option(&self) -> Option<Payload> {
        match self {
            Self::SingleLength {
                variables: (),
                payload,
            } => Some(*payload),
            _ => None,
        }
    }
}

impl<M: DynamicDataMarker, Variables> OptionalNames<Variables, DataPayload<M>>
where
    Variables: Copy,
{
    #[allow(clippy::needless_lifetimes)] // Yokeable is involved
    #[inline]
    pub(crate) fn as_borrowed<'a>(
        &'a self,
    ) -> OptionalNames<Variables, &'a <M::DataStruct as Yokeable<'a>>::Output> {
        match self {
            Self::None => OptionalNames::None,
            Self::SingleLength { variables, payload } => OptionalNames::SingleLength {
                variables: *variables,
                payload: payload.get(),
            },
        }
    }
}

size_test!(
    TypedDateTimeNames<icu_calendar::Gregorian, DateTimeMarker>,
    typed_date_time_names_size,
    456
);

/// A low-level type that formats datetime patterns with localized names.
/// The calendar should be chosen at compile time.
///
#[doc = typed_date_time_names_size!()]
///
/// Type parameters:
///
/// 1. The calendar chosen at compile time for additional type safety
/// 2. A components object type containing the fields that might be formatted
///
/// By default, the components object is set to [`NeoDateTimeComponents`],
/// meaning that dates and times, but not time zones, are supported. A smaller
/// components object results in smaller stack size.
///
/// To support all fields including time zones, use [`NeoComponents`].
///
/// [`NeoComponents`]: crate::neo_skeleton::NeoComponents
/// [`NeoDateTimeComponents`]: crate::neo_skeleton::NeoDateTimeComponents
///
/// # Examples
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::calendar::DateTime;
/// use icu::datetime::TypedDateTimeNames;
/// use icu::datetime::fields::FieldLength;
/// use icu::datetime::fields;
/// use icu::datetime::neo_pattern::DateTimePattern;
/// use icu::locale::locale;
/// use writeable::assert_try_writeable_eq;
///
/// // Create an instance that can format abbreviated month, weekday, and day period names:
/// let mut names: TypedDateTimeNames<Gregorian> =
///     TypedDateTimeNames::try_new(&locale!("uk").into()).unwrap();
/// names
///     .include_month_names(fields::Month::Format, FieldLength::Abbreviated)
///     .unwrap()
///     .include_weekday_names(fields::Weekday::Format, FieldLength::Abbreviated)
///     .unwrap()
///     .include_day_period_names(FieldLength::Abbreviated)
///     .unwrap();
///
/// // Create a pattern from a pattern string (note: K is the hour with h11 hour cycle):
/// let pattern_str = "E MMM d y -- K:mm a";
/// let pattern: DateTimePattern = pattern_str.parse().unwrap();
///
/// // Test it:
/// let datetime = DateTime::try_new_gregorian(2023, 11, 20, 12, 35, 3).unwrap();
/// assert_try_writeable_eq!(names.with_pattern(&pattern).format(&datetime), "пн лист. 20 2023 -- 0:35 пп");
/// ```
///
/// If the correct data is not loaded, and error will occur:
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::calendar::{Date, Time};
/// use icu::datetime::{DateTimeWriteError, TypedDateTimeNames};
/// use icu::datetime::fields::{Field, FieldLength, FieldSymbol, Weekday};
/// use icu::datetime::neo_pattern::DateTimePattern;
/// use icu::datetime::neo_skeleton::NeoSkeleton;
/// use icu::locale::locale;
/// use icu::timezone::{TimeZoneInfo, CustomZonedDateTime};
/// use writeable::{Part, assert_try_writeable_parts_eq};
///
/// // Create an instance that can format all fields (NeoSkeleton):
/// let mut names: TypedDateTimeNames<Gregorian, NeoSkeleton> =
///     TypedDateTimeNames::try_new(&locale!("en").into()).unwrap();
///
/// // Create a pattern from a pattern string:
/// let pattern_str = "'It is:' E MMM d y G 'at' h:mm:ssSSS a zzzz";
/// let pattern: DateTimePattern = pattern_str.parse().unwrap();
///
/// // The pattern string contains lots of symbols including "E", "MMM", and "a",
/// // but we did not load any data!
///
/// let mut dtz = CustomZonedDateTime::try_from_str("2023-11-20T11:35:03+00:00[Europe/London]").unwrap().to_calendar(Gregorian);
///
/// // Missing data is filled in on a best-effort basis, and an error is signaled.
/// assert_try_writeable_parts_eq!(
///     names.with_pattern(&pattern).format(&dtz),
///     "It is: mon M11 20 2023 ce at 11:35:03.000 AM +0000",
///     Err(DateTimeWriteError::MissingNames(Field { symbol: FieldSymbol::Weekday(Weekday::Format), length: FieldLength::One })),
///     [
///         (7, 10, Part::ERROR), // mon
///         (11, 14, Part::ERROR), // M11
///         (23, 25, Part::ERROR), // ce
///         (42, 44, Part::ERROR), // AM
///         (45, 50, Part::ERROR), // +0000
///     ]
/// );
/// ```
///
/// If the pattern contains fields inconsistent with the receiver, an error will occur:
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::calendar::DateTime;
/// use icu::datetime::{DateTimeWriteError, TypedDateTimeNames};
/// use icu::datetime::fields::{Field, FieldLength, FieldSymbol, Weekday};
/// use icu::datetime::neo_pattern::DateTimePattern;
/// use icu::datetime::fieldset::O;
/// use icu::locale::locale;
/// use icu::timezone::TimeZoneInfo;
/// use writeable::{Part, assert_try_writeable_parts_eq};
///
/// // Create an instance that can format abbreviated month, weekday, and day period names:
/// let mut names: TypedDateTimeNames<Gregorian, O> =
///     TypedDateTimeNames::try_new(&locale!("en").into()).unwrap();
///
/// // Create a pattern from a pattern string:
/// let pattern_str = "'It is:' E MMM d y G 'at' h:mm:ssSSS a zzzz";
/// let pattern: DateTimePattern = pattern_str.parse().unwrap();
///
/// // The pattern string contains lots of symbols including "E", "MMM", and "a",
/// // but the `TypedDateTimeNames` is configured to format only time zones!
/// // Further, the time zone we provide doesn't contain any offset into!
/// // Missing data is filled in on a best-effort basis, and an error is signaled.
/// assert_try_writeable_parts_eq!(
///     names.with_pattern(&pattern).format(&TimeZoneInfo::unknown()),
///     "It is: {E} {M} {d} {y} {G} at {h}:{m}:{s} {a} {z}",
///     Err(DateTimeWriteError::MissingInputField("iso_weekday")),
///     [
///         (7, 10, Part::ERROR), // {E}
///         (11, 14, Part::ERROR), // {M}
///         (15, 18, Part::ERROR), // {d}
///         (19, 22, Part::ERROR), // {y}
///         (23, 26, Part::ERROR), // {G}
///         (30, 33, Part::ERROR), // {h}
///         (34, 37, Part::ERROR), // {m}
///         (38, 41, Part::ERROR), // {s}
///         (42, 45, Part::ERROR), // {a}
///         (46, 49, Part::ERROR), // {z}
///     ]
/// );
/// ```
#[derive(Debug)]
pub struct TypedDateTimeNames<C: CldrCalendar, R: DateTimeNamesMarker = NeoDateTimeSkeleton> {
    locale: DataLocale,
    inner: RawDateTimeNames<R>,
    _calendar: PhantomData<C>,
}

pub trait DateTimeNamesMarker {
    type YearNames: DateTimeNamesHolderTrait<YearNamesV1Marker>;
    type MonthNames: DateTimeNamesHolderTrait<MonthNamesV1Marker>;
    type WeekdayNames: DateTimeNamesHolderTrait<WeekdayNamesV1Marker>;
    type DayPeriodNames: DateTimeNamesHolderTrait<DayPeriodNamesV1Marker>;
    type ZoneEssentials: DateTimeNamesHolderTrait<tz::EssentialsV1Marker>;
    type ZoneLocations: DateTimeNamesHolderTrait<tz::LocationsV1Marker>;
    type ZoneGenericLong: DateTimeNamesHolderTrait<tz::MzGenericLongV1Marker>;
    type ZoneGenericShort: DateTimeNamesHolderTrait<tz::MzGenericShortV1Marker>;
    type ZoneSpecificLong: DateTimeNamesHolderTrait<tz::MzSpecificLongV1Marker>;
    type ZoneSpecificShort: DateTimeNamesHolderTrait<tz::MzSpecificShortV1Marker>;
    type MetazoneLookup: DateTimeNamesHolderTrait<tz::MzPeriodV1Marker>;
}

#[derive(Debug)]
pub struct DateMarker {}

impl DateTimeNamesMarker for DateMarker {
    type YearNames = YearNamesV1Marker;
    type MonthNames = MonthNamesV1Marker;
    type WeekdayNames = WeekdayNamesV1Marker;
    type DayPeriodNames = NeverMarker<()>;
    type ZoneEssentials = NeverMarker<()>;
    type ZoneLocations = NeverMarker<()>;
    type ZoneGenericLong = NeverMarker<()>;
    type ZoneGenericShort = NeverMarker<()>;
    type ZoneSpecificLong = NeverMarker<()>;
    type ZoneSpecificShort = NeverMarker<()>;
    type MetazoneLookup = NeverMarker<()>;
}

#[derive(Debug)]
pub struct TimeMarker {}

impl DateTimeNamesMarker for TimeMarker {
    type YearNames = NeverMarker<()>;
    type MonthNames = NeverMarker<()>;
    type WeekdayNames = NeverMarker<()>;
    type DayPeriodNames = DayPeriodNamesV1Marker;
    type ZoneEssentials = NeverMarker<()>;
    type ZoneLocations = NeverMarker<()>;
    type ZoneGenericLong = NeverMarker<()>;
    type ZoneGenericShort = NeverMarker<()>;
    type ZoneSpecificLong = NeverMarker<()>;
    type ZoneSpecificShort = NeverMarker<()>;
    type MetazoneLookup = NeverMarker<()>;
}

#[derive(Debug)]
pub struct DateTimeMarker {}

impl DateTimeNamesMarker for DateTimeMarker {
    type YearNames = YearNamesV1Marker;
    type MonthNames = MonthNamesV1Marker;
    type WeekdayNames = WeekdayNamesV1Marker;
    type DayPeriodNames = DayPeriodNamesV1Marker;
    type ZoneEssentials = NeverMarker<()>;
    type ZoneLocations = NeverMarker<()>;
    type ZoneGenericLong = NeverMarker<()>;
    type ZoneGenericShort = NeverMarker<()>;
    type ZoneSpecificLong = NeverMarker<()>;
    type ZoneSpecificShort = NeverMarker<()>;
    type MetazoneLookup = NeverMarker<()>;
}

#[derive(Debug)]
pub struct ZonedDateTimeMarker {}

impl DateTimeNamesMarker for ZonedDateTimeMarker {
    type YearNames = YearNamesV1Marker;
    type MonthNames = MonthNamesV1Marker;
    type WeekdayNames = WeekdayNamesV1Marker;
    type DayPeriodNames = DayPeriodNamesV1Marker;
    type ZoneEssentials = tz::EssentialsV1Marker;
    type ZoneLocations = tz::LocationsV1Marker;
    type ZoneGenericLong = tz::MzGenericLongV1Marker;
    type ZoneGenericShort = tz::MzGenericShortV1Marker;
    type ZoneSpecificLong = tz::MzSpecificLongV1Marker;
    type ZoneSpecificShort = tz::MzSpecificShortV1Marker;
    type MetazoneLookup = tz::MzPeriodV1Marker;
}

impl From<RawDateTimeNames<DateMarker>> for RawDateTimeNames<DateTimeMarker> {
    fn from(other: RawDateTimeNames<DateMarker>) -> Self {
        Self {
            year_names: other.year_names,
            month_names: other.month_names,
            weekday_names: other.weekday_names,
            dayperiod_names: DateTimeNamesData2::none(),
            zone_essentials: (),
            locations: (),
            mz_generic_long: (),
            mz_generic_short: (),
            mz_specific_long: (),
            mz_specific_short: (),
            mz_periods: (),
            fixed_decimal_formatter: other.fixed_decimal_formatter,
            _marker: PhantomData,
        }
    }
}

impl From<RawDateTimeNames<TimeMarker>> for RawDateTimeNames<DateTimeMarker> {
    fn from(other: RawDateTimeNames<TimeMarker>) -> Self {
        Self {
            year_names: DateTimeNamesData2::none(),
            month_names: DateTimeNamesData2::none(),
            weekday_names: DateTimeNamesData2::none(),
            dayperiod_names: other.dayperiod_names,
            zone_essentials: (),
            locations: (),
            mz_generic_long: (),
            mz_generic_short: (),
            mz_specific_long: (),
            mz_specific_short: (),
            mz_periods: (),
            fixed_decimal_formatter: other.fixed_decimal_formatter,
            _marker: PhantomData,
        }
    }
}

pub(crate) struct RawDateTimeNames<R: DateTimeNamesMarker> {
    year_names:
        <R::YearNames as DateTimeNamesHolderTrait<YearNamesV1Marker>>::Container<FieldLength>,
    month_names: <R::MonthNames as DateTimeNamesHolderTrait<MonthNamesV1Marker>>::Container<(
        fields::Month,
        FieldLength,
    )>,
    weekday_names: <R::WeekdayNames as DateTimeNamesHolderTrait<WeekdayNamesV1Marker>>::Container<
        (fields::Weekday, FieldLength),
    >,
    dayperiod_names:
        <R::DayPeriodNames as DateTimeNamesHolderTrait<DayPeriodNamesV1Marker>>::Container<
            FieldLength,
        >,
    zone_essentials:
        <R::ZoneEssentials as DateTimeNamesHolderTrait<tz::EssentialsV1Marker>>::Container<()>,
    locations: <R::ZoneLocations as DateTimeNamesHolderTrait<tz::LocationsV1Marker>>::Container<()>,
    mz_generic_long:
        <R::ZoneGenericLong as DateTimeNamesHolderTrait<tz::MzGenericLongV1Marker>>::Container<()>,
    mz_generic_short: <R::ZoneGenericShort as DateTimeNamesHolderTrait<
        tz::MzGenericShortV1Marker,
    >>::Container<()>,
    mz_specific_long: <R::ZoneSpecificLong as DateTimeNamesHolderTrait<
        tz::MzSpecificLongV1Marker,
    >>::Container<()>,
    mz_specific_short: <R::ZoneSpecificShort as DateTimeNamesHolderTrait<
        tz::MzSpecificShortV1Marker,
    >>::Container<()>,
    mz_periods: <R::MetazoneLookup as DateTimeNamesHolderTrait<tz::MzPeriodV1Marker>>::Container<()>,
    // TODO(#4340): Make the FixedDecimalFormatter optional
    fixed_decimal_formatter: Option<FixedDecimalFormatter>,
    _marker: PhantomData<R>,
}

// Need a custom impl because not all of the associated types impl Debug
impl<R: DateTimeNamesMarker> fmt::Debug for RawDateTimeNames<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawDateTimeNames")
            .field("year_names", &self.year_names)
            .field("month_names", &self.month_names)
            .field("weekday_names", &self.weekday_names)
            .field("dayperiod_names", &self.dayperiod_names)
            .field("zone_essentials", &self.zone_essentials)
            .field("locations", &self.locations)
            .field("mz_generic_long", &self.mz_generic_long)
            .field("mz_generic_short", &self.mz_generic_short)
            .field("mz_specific_long", &self.mz_specific_long)
            .field("mz_specific_short", &self.mz_specific_short)
            .field("fixed_decimal_formatter", &self.fixed_decimal_formatter)
            .finish()
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct RawDateTimeNamesBorrowed<'l> {
    year_names: OptionalNames<FieldLength, &'l YearNamesV1<'l>>,
    month_names: OptionalNames<(fields::Month, FieldLength), &'l MonthNamesV1<'l>>,
    weekday_names: OptionalNames<(fields::Weekday, FieldLength), &'l LinearNamesV1<'l>>,
    dayperiod_names: OptionalNames<FieldLength, &'l LinearNamesV1<'l>>,
    zone_essentials: OptionalNames<(), &'l tz::EssentialsV1<'l>>,
    locations: OptionalNames<(), &'l tz::LocationsV1<'l>>,
    mz_generic_long: OptionalNames<(), &'l tz::MzGenericV1<'l>>,
    mz_generic_short: OptionalNames<(), &'l tz::MzGenericV1<'l>>,
    mz_specific_long: OptionalNames<(), &'l tz::MzSpecificV1<'l>>,
    mz_specific_short: OptionalNames<(), &'l tz::MzSpecificV1<'l>>,
    mz_periods: OptionalNames<(), &'l tz::MzPeriodV1<'l>>,
    pub(crate) fixed_decimal_formatter: Option<&'l FixedDecimalFormatter>,
}

impl<C: CldrCalendar, R: DateTimeNamesMarker> TypedDateTimeNames<C, R> {
    /// Constructor that takes a selected locale and creates an empty instance.
    ///
    /// For an example, see [`TypedDateTimeNames`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale) -> Result<Self, DataError> {
        let mut names = Self {
            locale: locale.clone(),
            inner: RawDateTimeNames::new_without_number_formatting(),
            _calendar: PhantomData,
        };
        names.include_fixed_decimal_formatter()?;
        Ok(names)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(provider: &P, locale: &DataLocale) -> Result<Self, DataError>
    where
        P: DataProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        let mut names = Self {
            locale: locale.clone(),
            inner: RawDateTimeNames::new_without_number_formatting(),
            _calendar: PhantomData,
        };
        names.load_fixed_decimal_formatter(provider)?;
        Ok(names)
    }

    /// Creates a completely empty instance, not even with number formatting.
    ///
    /// # Examples
    ///
    /// Errors occur if a number formatter is not loaded but one is required:
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::calendar::Date;
    /// use icu::datetime::{DateTimeWriteError, TypedDateTimeNames};
    /// use icu::datetime::fields::{Field, FieldLength, FieldSymbol, Weekday};
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::neo_skeleton::NeoDateSkeleton;
    /// use icu::locale::locale;
    /// use writeable::{Part, assert_try_writeable_parts_eq};
    ///
    /// // Create an instance that can format all fields (NeoSkeleton):
    /// let names: TypedDateTimeNames<Gregorian, NeoDateSkeleton> =
    ///     TypedDateTimeNames::new_without_number_formatting(&locale!("en").into());
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'It is:' y-MM-dd";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // The pattern string contains lots of numeric symbols,
    /// // but we did not load any data!
    ///
    /// let date = Date::try_new_gregorian(2024, 7, 1).unwrap();
    ///
    /// // Missing data is filled in on a best-effort basis, and an error is signaled.
    /// // (note that the padding is ignored in this fallback mode)
    /// assert_try_writeable_parts_eq!(
    ///     names.with_pattern(&pattern).format(&date),
    ///     "It is: 2024-7-1",
    ///     Err(DateTimeWriteError::MissingFixedDecimalFormatter),
    ///     [
    ///         (7, 11, Part::ERROR), // 2024
    ///         (12, 13, Part::ERROR), // 7
    ///         (14, 15, Part::ERROR), // 1
    ///     ]
    /// );
    /// ```
    pub fn new_without_number_formatting(locale: &DataLocale) -> Self {
        Self {
            locale: locale.clone(),
            inner: RawDateTimeNames::new_without_number_formatting(),
            _calendar: PhantomData,
        }
    }

    /// Loads year (era or cycle) names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_year_names<P>(
        &mut self,
        provider: &P,
        field_length: FieldLength,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<C::YearNamesV1Marker> + ?Sized,
    {
        self.inner.load_year_names(
            &C::YearNamesV1Marker::bind(provider),
            &self.locale,
            field_length,
        )?;
        Ok(self)
    }

    /// Includes year (era or cycle) names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::SingleLoadError;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("und").into())
    ///         .unwrap();
    ///
    /// // First length is successful:
    /// names.include_year_names(FieldLength::Wide).unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names.include_year_names(FieldLength::Wide).unwrap();
    ///
    /// // But loading a new length fails:
    /// assert!(matches!(
    ///     names.include_year_names(FieldLength::Abbreviated),
    ///     Err(SingleLoadError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_year_names(
        &mut self,
        field_length: FieldLength,
    ) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<<C as CldrCalendar>::YearNamesV1Marker>,
    {
        self.load_year_names(&crate::provider::Baked, field_length)
    }

    /// Loads month names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_month_names<P>(
        &mut self,
        provider: &P,
        field_symbol: fields::Month,
        field_length: FieldLength,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<C::MonthNamesV1Marker> + ?Sized,
    {
        self.inner.load_month_names(
            &C::MonthNamesV1Marker::bind(provider),
            &self.locale,
            field_symbol,
            field_length,
        )?;
        Ok(self)
    }

    /// Includes month names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::SingleLoadError;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("und").into())
    ///         .unwrap();
    /// let field_symbol = icu::datetime::fields::Month::Format;
    /// let alt_field_symbol = icu::datetime::fields::Month::StandAlone;
    ///
    /// // First length is successful:
    /// names
    ///     .include_month_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names
    ///     .include_month_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // But loading a new symbol or length fails:
    /// assert!(matches!(
    ///     names.include_month_names(alt_field_symbol, FieldLength::Wide),
    ///     Err(SingleLoadError::DuplicateField(_))
    /// ));
    /// assert!(matches!(
    ///     names.include_month_names(field_symbol, FieldLength::Abbreviated),
    ///     Err(SingleLoadError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_month_names(
        &mut self,
        field_symbol: fields::Month,
        field_length: FieldLength,
    ) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<<C as CldrCalendar>::MonthNamesV1Marker>,
    {
        self.load_month_names(&crate::provider::Baked, field_symbol, field_length)
    }

    /// Loads day period names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_day_period_names<P>(
        &mut self,
        provider: &P,
        field_length: FieldLength,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<DayPeriodNamesV1Marker> + ?Sized,
    {
        let provider = DayPeriodNamesV1Marker::bind(provider);
        self.inner
            .load_day_period_names(&provider, &self.locale, field_length)?;
        Ok(self)
    }

    /// Includes day period names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::SingleLoadError;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("und").into())
    ///         .unwrap();
    ///
    /// // First length is successful:
    /// names.include_day_period_names(FieldLength::Wide).unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names.include_day_period_names(FieldLength::Wide).unwrap();
    ///
    /// // But loading a new length fails:
    /// assert!(matches!(
    ///     names.include_day_period_names(FieldLength::Abbreviated),
    ///     Err(SingleLoadError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_day_period_names(
        &mut self,
        field_length: FieldLength,
    ) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<DayPeriodNamesV1Marker>,
    {
        self.load_day_period_names(&crate::provider::Baked, field_length)
    }

    /// Loads weekday names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_weekday_names<P>(
        &mut self,
        provider: &P,
        field_symbol: fields::Weekday,
        field_length: FieldLength,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<WeekdayNamesV1Marker> + ?Sized,
    {
        self.inner.load_weekday_names(
            &WeekdayNamesV1Marker::bind(provider),
            &self.locale,
            field_symbol,
            field_length,
        )?;
        Ok(self)
    }

    /// Includes weekday names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::SingleLoadError;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("und").into())
    ///         .unwrap();
    /// let field_symbol = icu::datetime::fields::Weekday::Format;
    /// let alt_field_symbol = icu::datetime::fields::Weekday::StandAlone;
    ///
    /// // First length is successful:
    /// names
    ///     .include_weekday_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names
    ///     .include_weekday_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // But loading a new symbol or length fails:
    /// assert!(matches!(
    ///     names.include_weekday_names(alt_field_symbol, FieldLength::Wide),
    ///     Err(SingleLoadError::DuplicateField(_))
    /// ));
    /// assert!(matches!(
    ///     names.include_weekday_names(field_symbol, FieldLength::Abbreviated),
    ///     Err(SingleLoadError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_weekday_names(
        &mut self,
        field_symbol: fields::Weekday,
        field_length: FieldLength,
    ) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<WeekdayNamesV1Marker>,
    {
        self.load_weekday_names(&crate::provider::Baked, field_symbol, field_length)
    }

    /// Loads shared essential patterns for time zone formatting.
    pub fn load_time_zone_essentials<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<tz::EssentialsV1Marker> + ?Sized,
    {
        self.inner
            .load_time_zone_essentials(&tz::EssentialsV1Marker::bind(provider), &self.locale)?;
        Ok(self)
    }

    /// Includes shared essential patterns for time zone formatting.
    ///
    /// This data should always be loaded when performing time zone formatting.
    /// By itself, it allows localized offset formats.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::neo_skeleton::NeoTimeZoneSkeleton;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::CustomZonedDateTime;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = CustomZonedDateTime::try_from_str(
    ///     "2024-01-01T00:00:00+00:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    /// let mut zone_london_summer = CustomZonedDateTime::try_from_str(
    ///     "2024-07-01T00:00:00+01:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian, NeoTimeZoneSkeleton>::try_new(
    ///         &locale!("en-GB").into(),
    ///     )
    ///     .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    ///
    /// // Create a pattern with symbol `OOOO`:
    /// let pattern_str = "'Your time zone is:' OOOO";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_winter),
    ///     "Your time zone is: GMT",
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_summer),
    ///     "Your time zone is: GMT+01:00",
    /// );
    ///
    /// // Now try `V`:
    /// let pattern_str = "'Your time zone is:' V";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_winter),
    ///     "Your time zone is: gblon",
    /// );
    ///
    /// // Now try `Z`:
    /// let pattern_str = "'Your time zone is:' Z";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_winter),
    ///     "Your time zone is: +0000",
    /// );
    ///
    /// // Now try `ZZZZZ`:
    /// let pattern_str = "'Your time zone is:' ZZZZZ";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_winter),
    ///     "Your time zone is: Z",
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_summer),
    ///     "Your time zone is: +01:00",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_essentials(&mut self) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::EssentialsV1Marker>,
    {
        self.load_time_zone_essentials(&crate::provider::Baked)
    }

    /// Loads location names for time zone formatting.
    pub fn load_time_zone_location_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<tz::LocationsV1Marker> + ?Sized,
    {
        self.inner
            .load_time_zone_location_names(&tz::LocationsV1Marker::bind(provider), &self.locale)?;
        Ok(self)
    }

    /// Includes location names for time zone formatting.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::neo_skeleton::NeoTimeZoneSkeleton;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::CustomZonedDateTime;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = CustomZonedDateTime::try_from_str(
    ///     "2024-01-01T00:00:00+00:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian, NeoTimeZoneSkeleton>::try_new(
    ///         &locale!("en-GB").into(),
    ///     )
    ///     .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_location_names().unwrap();
    ///
    /// // Try `VVVV`:
    /// let pattern_str = "'Your time zone is:' VVVV";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_winter),
    ///     "Your time zone is: United Kingdom Time",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_location_names(&mut self) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzGenericShortV1Marker>,
    {
        self.load_time_zone_location_names(&crate::provider::Baked)
    }

    /// Loads generic non-location long time zone names.
    pub fn load_time_zone_generic_long_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<tz::MzGenericLongV1Marker> + DataProvider<tz::MzPeriodV1Marker> + ?Sized,
    {
        self.inner.load_time_zone_generic_long_names(
            &tz::MzGenericLongV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            &self.locale,
        )?;
        Ok(self)
    }

    /// Includes generic non-location long time zone names.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::neo_skeleton::NeoTimeZoneSkeleton;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::CustomZonedDateTime;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = CustomZonedDateTime::try_from_str(
    ///     "2024-01-01T00:00:00+00:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    /// let mut zone_london_summer = CustomZonedDateTime::try_from_str(
    ///     "2024-07-01T00:00:00+01:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian, NeoTimeZoneSkeleton>::try_new(
    ///         &locale!("en-GB").into(),
    ///     )
    ///     .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_generic_long_names().unwrap();
    ///
    /// // Create a pattern with symbol `vvvv`:
    /// let pattern_str = "'Your time zone is:' vvvv";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_winter),
    ///     "Your time zone is: Greenwich Mean Time",
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_summer),
    ///     "Your time zone is: Greenwich Mean Time", // TODO
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_generic_long_names(&mut self) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzGenericLongV1Marker>,
    {
        self.load_time_zone_generic_long_names(&crate::provider::Baked)
    }

    /// Loads generic non-location short time zone names.
    pub fn load_time_zone_generic_short_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<tz::MzGenericShortV1Marker> + DataProvider<tz::MzPeriodV1Marker> + ?Sized,
    {
        self.inner.load_time_zone_generic_short_names(
            &tz::MzGenericShortV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            &self.locale,
        )?;
        Ok(self)
    }

    /// Includes generic non-location short time zone names.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::neo_skeleton::NeoTimeZoneSkeleton;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::CustomZonedDateTime;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = CustomZonedDateTime::try_from_str(
    ///     "2024-01-01T00:00:00+00:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    /// let mut zone_london_summer = CustomZonedDateTime::try_from_str(
    ///     "2024-07-01T00:00:00+01:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian, NeoTimeZoneSkeleton>::try_new(
    ///         &locale!("en-GB").into(),
    ///     )
    ///     .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_generic_short_names().unwrap();
    ///
    /// // Create a pattern with symbol `v`:
    /// let pattern_str = "'Your time zone is:' v";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_winter),
    ///     "Your time zone is: GMT",
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_summer),
    ///     "Your time zone is: GMT", // TODO
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_generic_short_names(&mut self) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzGenericShortV1Marker>,
    {
        self.load_time_zone_generic_short_names(&crate::provider::Baked)
    }

    /// Loads specific non-location long time zone names.
    pub fn load_time_zone_specific_long_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<tz::MzSpecificLongV1Marker> + DataProvider<tz::MzPeriodV1Marker> + ?Sized,
    {
        self.inner.load_time_zone_specific_long_names(
            &tz::MzSpecificLongV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            &self.locale,
        )?;
        Ok(self)
    }

    /// Includes specific non-location long time zone names.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::neo_skeleton::NeoTimeZoneSkeleton;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::CustomZonedDateTime;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = CustomZonedDateTime::try_from_str(
    ///     "2024-01-01T00:00:00+00:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    /// let mut zone_london_summer = CustomZonedDateTime::try_from_str(
    ///     "2024-07-01T00:00:00+01:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian, NeoTimeZoneSkeleton>::try_new(
    ///         &locale!("en-GB").into(),
    ///     )
    ///     .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_specific_long_names().unwrap();
    ///
    /// // Create a pattern with symbol `zzzz`:
    /// let pattern_str = "'Your time zone is:' zzzz";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_winter),
    ///     "Your time zone is: Greenwich Mean Time",
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_summer),
    ///     "Your time zone is: British Summer Time",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_specific_long_names(&mut self) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzSpecificLongV1Marker>,
    {
        self.load_time_zone_specific_long_names(&crate::provider::Baked)
    }

    /// Loads specific non-location short time zone names.
    pub fn load_time_zone_specific_short_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, SingleLoadError>
    where
        P: DataProvider<tz::MzSpecificShortV1Marker> + DataProvider<tz::MzPeriodV1Marker> + ?Sized,
    {
        self.inner.load_time_zone_specific_short_names(
            &tz::MzSpecificShortV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            &self.locale,
        )?;
        Ok(self)
    }

    /// Includes specific non-location short time zone names.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::neo_skeleton::NeoTimeZoneSkeleton;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::CustomZonedDateTime;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = CustomZonedDateTime::try_from_str(
    ///     "2024-01-01T00:00:00+00:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    /// let mut zone_london_summer = CustomZonedDateTime::try_from_str(
    ///     "2024-07-01T00:00:00+01:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .zone;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian, NeoTimeZoneSkeleton>::try_new(
    ///         &locale!("en-GB").into(),
    ///     )
    ///     .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_specific_short_names().unwrap();
    ///
    /// // Create a pattern with symbol `z`:
    /// let pattern_str = "'Your time zone is:' z";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_winter),
    ///     "Your time zone is: GMT",
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&zone_london_summer),
    ///     "Your time zone is: BST",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_specific_short_names(&mut self) -> Result<&mut Self, SingleLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzSpecificShortV1Marker>,
    {
        self.load_time_zone_specific_short_names(&crate::provider::Baked)
    }

    /// Loads a [`FixedDecimalFormatter`] from a data provider.
    #[inline]
    pub fn load_fixed_decimal_formatter<P>(&mut self, provider: &P) -> Result<&mut Self, DataError>
    where
        P: DataProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        self.inner
            .load_fixed_decimal_formatter(&ExternalLoaderUnstable(provider), &self.locale)?;
        Ok(self)
    }

    /// Loads a [`FixedDecimalFormatter`] with compiled data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Time;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::datetime::neo_skeleton::NeoTimeSkeleton;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let locale = &locale!("bn").into();
    ///
    /// let mut names = TypedDateTimeNames::<(), NeoTimeSkeleton>::try_new(&locale).unwrap();
    /// names.include_fixed_decimal_formatter();
    ///
    /// // Create a pattern for the time, which is all numbers
    /// let pattern_str = "'The current 24-hour time is:' HH:mm";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// let time = Time::try_new(6, 40, 33, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format(&time),
    ///     "The current 24-hour time is: ০৬:৪০",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub fn include_fixed_decimal_formatter(&mut self) -> Result<&mut Self, DataError> {
        self.inner
            .load_fixed_decimal_formatter(&ExternalLoaderCompiledData, &self.locale)?;
        Ok(self)
    }

    /// Associates this [`TypedDateTimeNames`] with a pattern
    /// without loading additional data for that pattern.
    #[inline]
    pub fn with_pattern<'l>(
        &'l self,
        pattern: &'l DateTimePattern,
    ) -> DateTimePatternFormatter<'l, C, R> {
        DateTimePatternFormatter {
            inner: self.inner.with_pattern(pattern.as_borrowed()),
            _calendar: PhantomData,
            _marker: PhantomData,
        }
    }

    /// Associates this [`TypedDateTimeNames`] with a datetime pattern
    /// and loads all data required for that pattern.
    ///
    /// Does not duplicate textual field symbols. See #4337
    pub fn load_for_pattern<'l, P>(
        &'l mut self,
        provider: &P,
        pattern: &'l DateTimePattern,
    ) -> Result<DateTimePatternFormatter<'l, C, R>, LoadError>
    where
        P: DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            + DataProvider<tz::EssentialsV1Marker>
            + DataProvider<tz::LocationsV1Marker>
            + DataProvider<tz::MzGenericLongV1Marker>
            + DataProvider<tz::MzGenericShortV1Marker>
            + DataProvider<tz::MzSpecificLongV1Marker>
            + DataProvider<tz::MzSpecificShortV1Marker>
            + DataProvider<tz::MzPeriodV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        let locale = &self.locale;
        self.inner.load_for_pattern(
            &C::YearNamesV1Marker::bind(provider),
            &C::MonthNamesV1Marker::bind(provider),
            &WeekdayNamesV1Marker::bind(provider),
            &DayPeriodNamesV1Marker::bind(provider),
            // TODO: Consider making time zone name loading optional here (lots of data)
            &tz::EssentialsV1Marker::bind(provider),
            &tz::LocationsV1Marker::bind(provider),
            &tz::MzGenericLongV1Marker::bind(provider),
            &tz::MzGenericShortV1Marker::bind(provider),
            &tz::MzSpecificLongV1Marker::bind(provider),
            &tz::MzSpecificShortV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            &ExternalLoaderUnstable(provider),
            locale,
            pattern
                .iter_items()
                .filter_map(FieldForDataLoading::try_from_pattern_item),
        )?;
        Ok(DateTimePatternFormatter {
            inner: self.inner.with_pattern(pattern.as_borrowed()),
            _calendar: PhantomData,
            _marker: PhantomData,
        })
    }

    /// Associates this [`TypedDateTimeNames`] with a pattern
    /// and includes all data required for that pattern.
    ///
    /// Does not support duplicate textual field symbols. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("en").into())
    ///         .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "MMM d (EEEE) 'of year' y G 'at' h:mm a";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // Load data for the pattern and format:
    /// let datetime =
    ///     DateTime::try_new_gregorian(2023, 12, 5, 17, 43, 12).unwrap();
    /// assert_try_writeable_eq!(
    ///     names
    ///         .include_for_pattern(&pattern)
    ///         .unwrap()
    ///         .format(&datetime),
    ///     "Dec 5 (Tuesday) of year 2023 AD at 5:43 PM"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_for_pattern<'l>(
        &'l mut self,
        pattern: &'l DateTimePattern,
    ) -> Result<DateTimePatternFormatter<'l, C, R>, LoadError>
    where
        crate::provider::Baked: DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            + DataProvider<tz::EssentialsV1Marker>
            + DataProvider<tz::MzGenericShortV1Marker>,
    {
        let locale = &self.locale;
        self.inner.load_for_pattern(
            &C::YearNamesV1Marker::bind(&crate::provider::Baked),
            &C::MonthNamesV1Marker::bind(&crate::provider::Baked),
            &WeekdayNamesV1Marker::bind(&crate::provider::Baked),
            &DayPeriodNamesV1Marker::bind(&crate::provider::Baked),
            &tz::EssentialsV1Marker::bind(&crate::provider::Baked),
            &tz::LocationsV1Marker::bind(&crate::provider::Baked),
            &tz::MzGenericLongV1Marker::bind(&crate::provider::Baked),
            &tz::MzGenericShortV1Marker::bind(&crate::provider::Baked),
            &tz::MzSpecificLongV1Marker::bind(&crate::provider::Baked),
            &tz::MzSpecificShortV1Marker::bind(&crate::provider::Baked),
            &tz::MzPeriodV1Marker::bind(&crate::provider::Baked),
            &ExternalLoaderCompiledData,
            locale,
            pattern
                .iter_items()
                .filter_map(FieldForDataLoading::try_from_pattern_item),
        )?;
        Ok(DateTimePatternFormatter {
            inner: self.inner.with_pattern(pattern.as_borrowed()),
            _calendar: PhantomData,
            _marker: PhantomData,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, displaydoc::Display)]
#[non_exhaustive]
/// Error returned from [`TypedDateTimeNames`]'s load methods.
pub enum SingleLoadError {
    /// Duplicate field in pattern
    DuplicateField(Field),
    /// ICU4X does not support this field
    UnsupportedField(Field),
    /// The specific type does not support this field
    TypeTooNarrow(Field),
    /// An error arising from the [`DataProvider`]
    Data(DataError),
}

#[derive(Debug, Clone, Copy, PartialEq, displaydoc::Display)]
/// Error returned from [`TypedDateTimeNames`]'s pattern load methods.
#[non_exhaustive]
pub enum LoadError {
    /// DuplicateField
    DuplicateField(Field),
    /// ICU4X does not support this field
    UnsupportedField(Field),
    /// The specific type does not support this field
    TypeTooNarrow(Field),
    /// An error arising from the [`DataProvider`]
    Data(DataError),
    /// MissingNames
    MissingNames(Field),
}

impl From<SingleLoadError> for LoadError {
    fn from(value: SingleLoadError) -> Self {
        match value {
            SingleLoadError::Data(e) => LoadError::Data(e),
            SingleLoadError::UnsupportedField(f) => LoadError::UnsupportedField(f),
            SingleLoadError::TypeTooNarrow(f) => LoadError::TypeTooNarrow(f),
            SingleLoadError::DuplicateField(f) => LoadError::DuplicateField(f),
        }
    }
}

pub(crate) enum FieldForDataLoading {
    Field(Field),
    TimeZone(ResolvedNeoTimeZoneSkeleton),
}

impl FieldForDataLoading {
    pub(crate) fn try_from_pattern_item(pattern_item: PatternItem) -> Option<Self> {
        match pattern_item {
            PatternItem::Field(field) => Some(Self::Field(field)),
            PatternItem::Literal(_) => None,
        }
    }
}

impl<R: DateTimeNamesMarker> RawDateTimeNames<R> {
    pub(crate) fn new_without_number_formatting() -> Self {
        Self {
            year_names: <R::YearNames as DateTimeNamesHolderTrait<YearNamesV1Marker>>::Container::<_>::new_empty(),
            month_names: <R::MonthNames as DateTimeNamesHolderTrait<MonthNamesV1Marker>>::Container::<_>::new_empty(),
            weekday_names: <R::WeekdayNames as DateTimeNamesHolderTrait<WeekdayNamesV1Marker>>::Container::<_>::new_empty(),
            dayperiod_names: <R::DayPeriodNames as DateTimeNamesHolderTrait<DayPeriodNamesV1Marker>>::Container::<_>::new_empty(),
            zone_essentials: <R::ZoneEssentials as DateTimeNamesHolderTrait<tz::EssentialsV1Marker>>::Container::<_>::new_empty(),
            locations: <R::ZoneLocations as DateTimeNamesHolderTrait<tz::LocationsV1Marker>>::Container::<_>::new_empty(),
            mz_generic_long: <R::ZoneGenericLong as DateTimeNamesHolderTrait<tz::MzGenericLongV1Marker>>::Container::<_>::new_empty(),
            mz_generic_short: <R::ZoneGenericShort as DateTimeNamesHolderTrait<tz::MzGenericShortV1Marker>>::Container::<_>::new_empty(),
            mz_specific_long: <R::ZoneSpecificLong as DateTimeNamesHolderTrait<tz::MzSpecificLongV1Marker>>::Container::<_>::new_empty(),
            mz_specific_short: <R::ZoneSpecificShort as DateTimeNamesHolderTrait<tz::MzSpecificShortV1Marker>>::Container::<_>::new_empty(),
            mz_periods: <R::MetazoneLookup as DateTimeNamesHolderTrait<tz::MzPeriodV1Marker>>::Container::<_>::new_empty(),
            fixed_decimal_formatter: None,
            _marker: PhantomData,
        }
    }

    pub(crate) fn as_borrowed(&self) -> RawDateTimeNamesBorrowed {
        RawDateTimeNamesBorrowed {
            year_names: self.year_names.get().inner,
            month_names: self.month_names.get().inner,
            weekday_names: self.weekday_names.get().inner,
            dayperiod_names: self.dayperiod_names.get().inner,
            zone_essentials: self.zone_essentials.get().inner,
            locations: self.locations.get().inner,
            mz_generic_long: self.mz_generic_long.get().inner,
            mz_generic_short: self.mz_generic_short.get().inner,
            mz_specific_long: self.mz_specific_long.get().inner,
            mz_specific_short: self.mz_specific_short.get().inner,
            mz_periods: self.mz_periods.get().inner,
            fixed_decimal_formatter: self.fixed_decimal_formatter.as_ref(),
        }
    }

    pub(crate) fn load_year_names<P>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
        field_length: FieldLength,
    ) -> Result<(), SingleLoadError>
    where
        P: BoundDataProvider<YearNamesV1Marker> + ?Sized,
    {
        let field = fields::Field {
            symbol: FieldSymbol::Era,
            length: field_length,
        };
        // UTS 35 says that "G..GGG" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        let variables = field_length;
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                marker_attrs::name_attr_for(
                    marker_attrs::Context::Format,
                    match field_length {
                        FieldLength::Abbreviated => marker_attrs::Length::Abbr,
                        FieldLength::Narrow => marker_attrs::Length::Narrow,
                        FieldLength::Wide => marker_attrs::Length::Wide,
                        _ => return Err(SingleLoadError::UnsupportedField(field)),
                    },
                ),
                locale,
            ),
            ..Default::default()
        };
        self.year_names
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_month_names<P>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
        field_symbol: fields::Month,
        field_length: FieldLength,
    ) -> Result<(), SingleLoadError>
    where
        P: BoundDataProvider<MonthNamesV1Marker> + ?Sized,
    {
        let field = fields::Field {
            symbol: FieldSymbol::Month(field_symbol),
            length: field_length,
        };
        let variables = (field_symbol, field_length);
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                marker_attrs::name_attr_for(
                    match field_symbol {
                        fields::Month::Format => marker_attrs::Context::Format,
                        fields::Month::StandAlone => marker_attrs::Context::Standalone,
                    },
                    match field_length {
                        FieldLength::Abbreviated => marker_attrs::Length::Abbr,
                        FieldLength::Narrow => marker_attrs::Length::Narrow,
                        FieldLength::Wide => marker_attrs::Length::Wide,
                        _ => return Err(SingleLoadError::UnsupportedField(field)),
                    },
                ),
                locale,
            ),
            ..Default::default()
        };
        self.month_names
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_day_period_names<P>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
        field_length: FieldLength,
    ) -> Result<(), SingleLoadError>
    where
        P: BoundDataProvider<DayPeriodNamesV1Marker> + ?Sized,
    {
        let field = fields::Field {
            // Names for 'a' and 'b' are stored in the same data marker
            symbol: FieldSymbol::DayPeriod(fields::DayPeriod::NoonMidnight),
            length: field_length,
        };
        // UTS 35 says that "a..aaa" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        let variables = field_length;
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                marker_attrs::name_attr_for(
                    marker_attrs::Context::Format,
                    match field_length {
                        FieldLength::Abbreviated => marker_attrs::Length::Abbr,
                        FieldLength::Narrow => marker_attrs::Length::Narrow,
                        FieldLength::Wide => marker_attrs::Length::Wide,
                        _ => return Err(SingleLoadError::UnsupportedField(field)),
                    },
                ),
                locale,
            ),
            ..Default::default()
        };
        self.dayperiod_names
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_weekday_names<P>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
        field_symbol: fields::Weekday,
        field_length: FieldLength,
    ) -> Result<(), SingleLoadError>
    where
        P: BoundDataProvider<WeekdayNamesV1Marker> + ?Sized,
    {
        let field = fields::Field {
            symbol: FieldSymbol::Weekday(field_symbol),
            length: field_length,
        };
        // UTS 35 says that "E..EEE" are all Abbreviated
        // However, this doesn't apply to "e" and "c".
        let field_length = if matches!(field_symbol, fields::Weekday::Format) {
            field_length.numeric_to_abbr()
        } else {
            field_length
        };
        let variables = (field_symbol, field_length);
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                marker_attrs::name_attr_for(
                    match field_symbol {
                        // UTS 35 says that "e" and "E" have the same non-numeric names
                        fields::Weekday::Format | fields::Weekday::Local => {
                            marker_attrs::Context::Format
                        }
                        fields::Weekday::StandAlone => marker_attrs::Context::Standalone,
                    },
                    match field_length {
                        FieldLength::Abbreviated => marker_attrs::Length::Abbr,
                        FieldLength::Narrow => marker_attrs::Length::Narrow,
                        FieldLength::Wide => marker_attrs::Length::Wide,
                        FieldLength::Six => marker_attrs::Length::Short,
                        _ => return Err(SingleLoadError::UnsupportedField(field)),
                    },
                ),
                locale,
            ),
            ..Default::default()
        };
        self.weekday_names
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_time_zone_essentials<P>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
    ) -> Result<(), SingleLoadError>
    where
        P: BoundDataProvider<tz::EssentialsV1Marker> + ?Sized,
    {
        let field = fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::UpperZ),
            length: FieldLength::Wide,
        };
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(locale),
            ..Default::default()
        };
        self.zone_essentials
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_time_zone_location_names<P>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
    ) -> Result<(), SingleLoadError>
    where
        P: BoundDataProvider<tz::LocationsV1Marker> + ?Sized,
    {
        let field = fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::UpperV),
            length: FieldLength::Wide,
        };
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(locale),
            ..Default::default()
        };
        self.locations
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_time_zone_generic_long_names(
        &mut self,
        mz_generic_long_provider: &(impl BoundDataProvider<tz::MzGenericLongV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        locale: &DataLocale,
    ) -> Result<(), SingleLoadError> {
        let field = fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerV),
            length: FieldLength::Wide,
        };
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(locale),
            ..Default::default()
        };
        self.mz_generic_long
            .load_put(mz_generic_long_provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        self.mz_periods
            .load_put(mz_period_provider, Default::default(), variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_time_zone_generic_short_names(
        &mut self,
        mz_generic_short_provider: &(impl BoundDataProvider<tz::MzGenericShortV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        locale: &DataLocale,
    ) -> Result<(), SingleLoadError> {
        let field = fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerV),
            length: FieldLength::One,
        };
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(locale),
            ..Default::default()
        };
        self.mz_generic_short
            .load_put(mz_generic_short_provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        self.mz_periods
            .load_put(mz_period_provider, Default::default(), variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_time_zone_specific_long_names(
        &mut self,
        mz_specific_long_provider: &(impl BoundDataProvider<tz::MzSpecificLongV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        locale: &DataLocale,
    ) -> Result<(), SingleLoadError> {
        let field = fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerZ),
            length: FieldLength::Wide,
        };
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(locale),
            ..Default::default()
        };
        self.mz_specific_long
            .load_put(mz_specific_long_provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        self.mz_periods
            .load_put(mz_period_provider, Default::default(), variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_time_zone_specific_short_names(
        &mut self,
        mz_specific_short_provider: &(impl BoundDataProvider<tz::MzSpecificShortV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        locale: &DataLocale,
    ) -> Result<(), SingleLoadError> {
        let field = fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerZ),
            length: FieldLength::One,
        };
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(locale),
            ..Default::default()
        };
        self.mz_specific_short
            .load_put(mz_specific_short_provider, req, variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        self.mz_periods
            .load_put(mz_period_provider, Default::default(), variables)
            .map_err(|e| MaybePayloadError2::into_single_load_error(e, field))?
            .map_err(SingleLoadError::Data)?;
        Ok(())
    }

    pub(crate) fn load_fixed_decimal_formatter(
        &mut self,
        loader: &impl FixedDecimalFormatterLoader,
        locale: &DataLocale,
    ) -> Result<(), DataError> {
        if self.fixed_decimal_formatter.is_some() {
            return Ok(());
        }
        let mut options = FixedDecimalFormatterOptions::default();
        options.grouping_strategy = GroupingStrategy::Never;
        self.fixed_decimal_formatter =
            Some(FixedDecimalFormatterLoader::load(loader, locale, options)?);
        Ok(())
    }

    /// Associates this [`TypedDateTimeNames`] with a pattern
    /// without loading additional data for that pattern.
    #[inline]
    pub(crate) fn with_pattern<'l>(
        &'l self,
        pattern: DateTimePatternBorrowed<'l>,
    ) -> RawDateTimePatternFormatter<'l> {
        RawDateTimePatternFormatter {
            pattern,
            names: self.as_borrowed(),
        }
    }

    /// Loads all data required for formatting the given [`PatternItem`]s.
    ///
    /// This function has a lot of arguments because many of the arguments are generic,
    /// and pulling them out to an options struct would be cumbersome.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn load_for_pattern(
        &mut self,
        year_provider: &(impl BoundDataProvider<YearNamesV1Marker> + ?Sized),
        month_provider: &(impl BoundDataProvider<MonthNamesV1Marker> + ?Sized),
        weekday_provider: &(impl BoundDataProvider<WeekdayNamesV1Marker> + ?Sized),
        dayperiod_provider: &(impl BoundDataProvider<DayPeriodNamesV1Marker> + ?Sized),
        zone_essentials_provider: &(impl BoundDataProvider<tz::EssentialsV1Marker> + ?Sized),
        locations_provider: &(impl BoundDataProvider<tz::LocationsV1Marker> + ?Sized),
        mz_generic_long_provider: &(impl BoundDataProvider<tz::MzGenericLongV1Marker> + ?Sized),
        mz_generic_short_provider: &(impl BoundDataProvider<tz::MzGenericShortV1Marker> + ?Sized),
        mz_specific_long_provider: &(impl BoundDataProvider<tz::MzSpecificLongV1Marker> + ?Sized),
        mz_specific_short_provider: &(impl BoundDataProvider<tz::MzSpecificShortV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        fixed_decimal_formatter_loader: &impl FixedDecimalFormatterLoader,
        locale: &DataLocale,
        pattern_items: impl Iterator<Item = FieldForDataLoading>,
    ) -> Result<(), LoadError> {
        let mut numeric_field = None;
        for item in pattern_items {
            let item = match item {
                FieldForDataLoading::Field(Field {
                    symbol: FieldSymbol::TimeZone(field_symbol),
                    length,
                }) => {
                    match ResolvedNeoTimeZoneSkeleton::from_field(field_symbol, length) {
                        Some(time_zone) => FieldForDataLoading::TimeZone(time_zone),
                        None => {
                            // Unknown time zone field: ignore for data loading
                            continue;
                        }
                    }
                }
                _ => item,
            };
            let field = match item {
                FieldForDataLoading::Field(field) => field,
                FieldForDataLoading::TimeZone(time_zone) => {
                    match time_zone {
                        // `z..zzz`
                        ResolvedNeoTimeZoneSkeleton::SpecificShort => {
                            self.load_time_zone_essentials(zone_essentials_provider, locale)?;
                            self.load_fixed_decimal_formatter(
                                fixed_decimal_formatter_loader,
                                locale,
                            )
                            .map_err(LoadError::Data)?;
                            self.load_time_zone_specific_short_names(
                                mz_specific_short_provider,
                                mz_period_provider,
                                locale,
                            )?;
                        }
                        // `zzzz`
                        ResolvedNeoTimeZoneSkeleton::SpecificLong => {
                            self.load_time_zone_essentials(zone_essentials_provider, locale)?;
                            self.load_fixed_decimal_formatter(
                                fixed_decimal_formatter_loader,
                                locale,
                            )
                            .map_err(LoadError::Data)?;
                            self.load_time_zone_specific_long_names(
                                mz_specific_long_provider,
                                mz_period_provider,
                                locale,
                            )?;
                        }
                        // 'v'
                        ResolvedNeoTimeZoneSkeleton::GenericShort => {
                            self.load_time_zone_essentials(zone_essentials_provider, locale)?;
                            self.load_fixed_decimal_formatter(
                                fixed_decimal_formatter_loader,
                                locale,
                            )
                            .map_err(LoadError::Data)?;
                            self.load_time_zone_generic_short_names(
                                mz_generic_short_provider,
                                mz_period_provider,
                                locale,
                            )?;
                            // For fallback:
                            self.load_time_zone_location_names(locations_provider, locale)?;
                        }
                        // 'vvvv'
                        ResolvedNeoTimeZoneSkeleton::GenericLong => {
                            self.load_time_zone_essentials(zone_essentials_provider, locale)?;
                            self.load_fixed_decimal_formatter(
                                fixed_decimal_formatter_loader,
                                locale,
                            )
                            .map_err(LoadError::Data)?;
                            self.load_time_zone_generic_long_names(
                                mz_generic_long_provider,
                                mz_period_provider,
                                locale,
                            )?;
                            // For fallback:
                            self.load_time_zone_location_names(locations_provider, locale)?;
                        }
                        // 'VVVV' (note: `V..VV` are for identifiers only)
                        ResolvedNeoTimeZoneSkeleton::Location => {
                            self.load_time_zone_essentials(zone_essentials_provider, locale)?;
                            self.load_fixed_decimal_formatter(
                                fixed_decimal_formatter_loader,
                                locale,
                            )
                            .map_err(LoadError::Data)?;
                            self.load_time_zone_location_names(locations_provider, locale)?;
                        }
                        ResolvedNeoTimeZoneSkeleton::OffsetShort
                        | ResolvedNeoTimeZoneSkeleton::OffsetLong => {
                            self.load_time_zone_essentials(zone_essentials_provider, locale)?;
                            self.load_fixed_decimal_formatter(
                                fixed_decimal_formatter_loader,
                                locale,
                            )
                            .map_err(LoadError::Data)?;
                        }
                        ResolvedNeoTimeZoneSkeleton::Isox
                        | ResolvedNeoTimeZoneSkeleton::Isoxx
                        | ResolvedNeoTimeZoneSkeleton::Isoxxx
                        | ResolvedNeoTimeZoneSkeleton::Isoxxxx
                        | ResolvedNeoTimeZoneSkeleton::Isoxxxxx
                        | ResolvedNeoTimeZoneSkeleton::IsoX
                        | ResolvedNeoTimeZoneSkeleton::IsoXX
                        | ResolvedNeoTimeZoneSkeleton::IsoXXX
                        | ResolvedNeoTimeZoneSkeleton::IsoXXXX
                        | ResolvedNeoTimeZoneSkeleton::IsoXXXXX
                        | ResolvedNeoTimeZoneSkeleton::Bcp47Id => {
                            // no data required
                        }
                    };
                    continue;
                }
            };
            match field.symbol {
                ///// Textual symbols /////
                FieldSymbol::Era => {
                    self.load_year_names(year_provider, locale, field.length)?;
                }
                FieldSymbol::Month(symbol) => match field.length {
                    FieldLength::One => numeric_field = Some(field),
                    FieldLength::TwoDigit => numeric_field = Some(field),
                    _ => {
                        self.load_month_names(month_provider, locale, symbol, field.length)?;
                    }
                },
                // 'E' is always text
                // 'e' and 'c' are either numeric or text
                FieldSymbol::Weekday(symbol) => match field.length {
                    FieldLength::One | FieldLength::TwoDigit
                        if !matches!(symbol, fields::Weekday::Format) =>
                    {
                        numeric_field = Some(field)
                    }
                    _ => {
                        self.load_weekday_names(weekday_provider, locale, symbol, field.length)?;
                    }
                },
                FieldSymbol::DayPeriod(_) => {
                    self.load_day_period_names(dayperiod_provider, locale, field.length)?;
                }
                FieldSymbol::TimeZone(_) => {
                    debug_assert!(false, "handled above");
                }

                ///// Numeric symbols /////
                FieldSymbol::Year(_) => numeric_field = Some(field),
                FieldSymbol::Week(_) => numeric_field = Some(field),
                FieldSymbol::Day(_) => numeric_field = Some(field),
                FieldSymbol::Hour(_) => numeric_field = Some(field),
                FieldSymbol::Minute => numeric_field = Some(field),
                FieldSymbol::Second(_) => numeric_field = Some(field),
                FieldSymbol::DecimalSecond(_) => numeric_field = Some(field),
            };
        }

        if numeric_field.is_some() {
            self.load_fixed_decimal_formatter(fixed_decimal_formatter_loader, locale)
                .map_err(LoadError::Data)?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DateTimePatternFormatter<'a, C: CldrCalendar, R> {
    inner: RawDateTimePatternFormatter<'a>,
    _calendar: PhantomData<C>,
    _marker: PhantomData<R>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct RawDateTimePatternFormatter<'a> {
    pattern: DateTimePatternBorrowed<'a>,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a, C: CldrCalendar, R: DateTimeMarkers> DateTimePatternFormatter<'a, C, R>
where
    R::D: TypedDateDataMarkers<C> + DateInputMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
{
    /// Formats a date and time of day.
    ///
    /// For an example, see [`TypedDateTimeNames`].
    pub fn format<I>(&self, datetime: &I) -> FormattedDateTimePattern<'a>
    where
        I: ?Sized + IsInCalendar<C> + AllInputMarkers<R>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            input: ExtractedInput::extract_from_neo_input::<R::D, R::T, R::Z, I>(datetime),
            names: self.inner.names,
        }
    }

    /// Formats a date without a time of day.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// // Create an instance that can format wide month and era names:
    /// let mut names: TypedDateTimeNames<Gregorian> =
    ///     TypedDateTimeNames::try_new(&locale!("en-GB").into()).unwrap();
    /// names
    ///     .include_month_names(fields::Month::Format, FieldLength::Wide)
    ///     .unwrap()
    ///     .include_year_names(FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'The date is:' MMMM d, y GGGG";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // Test it with some different dates:
    /// // Note: extended year -50 is year 51 BCE
    /// let date_bce = Date::try_new_gregorian(-50, 3, 15).unwrap();
    /// let date_ce = Date::try_new_gregorian(1700, 11, 20).unwrap();
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_date(&date_bce),
    ///     "The date is: March 15, 51 Before Christ"
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_date(&date_ce),
    ///     "The date is: November 20, 1700 Anno Domini"
    /// );
    /// ```
    pub fn format_date<I>(&self, datetime: &'a I) -> FormattedDateTimePattern<'a>
    where
        I: ?Sized
            + IsInCalendar<C>
            + GetField<<R::D as DateInputMarkers>::YearInput>
            + GetField<<R::D as DateInputMarkers>::MonthInput>
            + GetField<<R::D as DateInputMarkers>::DayOfMonthInput>
            + GetField<<R::D as DateInputMarkers>::DayOfWeekInput>
            + GetField<<R::D as DateInputMarkers>::AnyCalendarKindInput>
            + GetField<()>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            input: ExtractedInput::extract_from_neo_input::<R::D, NeoNeverMarker, NeoNeverMarker, I>(
                datetime,
            ),
            names: self.inner.names,
        }
    }

    /// Formats a time of day without a date.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::calendar::Time;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// // Create an instance that can format abbreviated day periods:
    /// let mut names: TypedDateTimeNames<Gregorian> =
    ///     TypedDateTimeNames::try_new(&locale!("en-US").into()).unwrap();
    /// names
    ///     .include_day_period_names(FieldLength::Abbreviated)
    ///     .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'The time is:' h:mm b";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // Test it with different times of day:
    /// let time_am = Time::try_new(11, 4, 14, 0).unwrap();
    /// let time_pm = Time::try_new(13, 41, 28, 0).unwrap();
    /// let time_noon = Time::try_new(12, 0, 0, 0).unwrap();
    /// let time_midnight = Time::try_new(0, 0, 0, 0).unwrap();
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_am),
    ///     "The time is: 11:04 AM"
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_pm),
    ///     "The time is: 1:41 PM"
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_noon),
    ///     "The time is: 12:00 noon"
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_midnight),
    ///     "The time is: 12:00 midnight"
    /// );
    /// ```
    pub fn format_time<I>(&self, datetime: &'a I) -> FormattedDateTimePattern<'a>
    where
        I: ?Sized
            + IsInCalendar<C>
            + GetField<<R::T as TimeMarkers>::HourInput>
            + GetField<<R::T as TimeMarkers>::MinuteInput>
            + GetField<<R::T as TimeMarkers>::SecondInput>
            + GetField<<R::T as TimeMarkers>::NanoSecondInput>
            + GetField<()>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            input: ExtractedInput::extract_from_neo_input::<NeoNeverMarker, R::T, NeoNeverMarker, I>(
                datetime,
            ),
            names: self.inner.names,
        }
    }

    /// Formats a timezone without a date or time.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::neo_skeleton::NeoTimeZoneSkeleton;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::CustomZonedDateTime;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut london_winter = CustomZonedDateTime::try_from_str(
    ///     "2024-01-01T00:00:00+00:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .to_calendar(Gregorian);
    /// let mut london_summer = CustomZonedDateTime::try_from_str(
    ///     "2024-07-01T00:00:00+01:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .to_calendar(Gregorian);
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian, NeoTimeZoneSkeleton>::try_new(
    ///         &locale!("en-GB").into(),
    ///     )
    ///     .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_specific_short_names().unwrap();
    ///
    /// // Create a pattern with symbol `z`:
    /// let pattern_str = "'Your time zone is:' z";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_timezone(&london_winter),
    ///     "Your time zone is: GMT",
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_timezone(&london_summer),
    ///     "Your time zone is: BST",
    /// );
    /// ```
    pub fn format_timezone<I>(&self, datetime: &'a I) -> FormattedDateTimePattern<'a>
    where
        I: ?Sized
            + IsInCalendar<C>
            + GetField<<R::Z as ZoneMarkers>::TimeZoneIdInput>
            + GetField<<R::Z as ZoneMarkers>::TimeZoneOffsetInput>
            + GetField<<R::Z as ZoneMarkers>::TimeZoneVariantInput>
            + GetField<<R::Z as ZoneMarkers>::TimeZoneLocalTimeInput>
            + GetField<()>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            input: ExtractedInput::extract_from_neo_input::<NeoNeverMarker, NeoNeverMarker, R::Z, I>(
                datetime,
            ),
            names: self.inner.names,
        }
    }
}

/// A pattern that has been interpolated and implements [`TryWriteable`].
#[derive(Debug)]
pub struct FormattedDateTimePattern<'a> {
    pattern: DateTimePatternBorrowed<'a>,
    input: ExtractedInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl TryWriteable for FormattedDateTimePattern<'_> {
    type Error = DateTimeWriteError;
    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        try_write_pattern_items(
            self.pattern.0.as_borrowed().metadata,
            self.pattern.0.as_borrowed().items.iter(),
            &self.input,
            &self.names,
            self.names.fixed_decimal_formatter,
            sink,
        )
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'data> RawDateTimeNamesBorrowed<'data> {
    pub(crate) fn get_name_for_month(
        &self,
        field_symbol: fields::Month,
        field_length: FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, GetNameForMonthError> {
        let field = fields::Field {
            symbol: FieldSymbol::Month(field_symbol),
            length: field_length,
        };
        let month_names = self
            .month_names
            .get_with_variables((field_symbol, field_length))
            .ok_or(GetNameForMonthError::MissingNames(field))?;
        let Some((month_number, is_leap)) = code.parsed() else {
            return Err(GetNameForMonthError::Missing);
        };
        let Some(month_index) = month_number.checked_sub(1) else {
            return Err(GetNameForMonthError::Missing);
        };
        let month_index = usize::from(month_index);
        let name = match month_names {
            MonthNamesV1::Linear(linear) => {
                if is_leap {
                    None
                } else {
                    linear.get(month_index)
                }
            }
            MonthNamesV1::LeapLinear(leap_linear) => {
                let num_months = leap_linear.len() / 2;
                if is_leap {
                    leap_linear.get(month_index + num_months)
                } else if month_index < num_months {
                    leap_linear.get(month_index)
                } else {
                    None
                }
            }
            MonthNamesV1::LeapNumeric(leap_numeric) => {
                if is_leap {
                    return Ok(MonthPlaceholderValue::NumericPattern(leap_numeric));
                } else {
                    return Ok(MonthPlaceholderValue::Numeric);
                }
            }
        };
        // Note: Always return `false` for the second argument since neo MonthNames
        // knows how to handle leap months and we don't need the fallback logic
        name.map(MonthPlaceholderValue::PlainString)
            .ok_or(GetNameForMonthError::Missing)
    }

    pub(crate) fn get_name_for_weekday(
        &self,
        field_symbol: fields::Weekday,
        field_length: FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, GetNameForWeekdayError> {
        let field = fields::Field {
            symbol: FieldSymbol::Weekday(field_symbol),
            length: field_length,
        };
        // UTS 35 says that "e" and "E" have the same non-numeric names
        let field_symbol = field_symbol.to_format_symbol();
        // UTS 35 says that "E..EEE" are all Abbreviated
        // However, this doesn't apply to "e" and "c".
        let field_length = if matches!(field_symbol, fields::Weekday::Format) {
            field_length.numeric_to_abbr()
        } else {
            field_length
        };
        let weekday_names = self
            .weekday_names
            .get_with_variables((field_symbol, field_length))
            .ok_or(GetNameForWeekdayError::MissingNames(field))?;
        weekday_names
            .names
            .get((day as usize) % 7)
            .ok_or(GetNameForWeekdayError::Missing)
    }

    /// Gets the era symbol, or `None` if data is loaded but symbol isn't found.
    ///
    /// `None` should fall back to the era code directly, if, for example,
    /// a japanext datetime is formatted with a `DateTimeFormat<Japanese>`
    pub(crate) fn get_name_for_era(
        &self,
        field_length: FieldLength,
        era_code: Era,
    ) -> Result<&str, GetSymbolForEraError> {
        let field = fields::Field {
            symbol: FieldSymbol::Era,
            length: field_length,
        };
        // UTS 35 says that "G..GGG" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        let year_names = self
            .year_names
            .get_with_variables(field_length)
            .ok_or(GetSymbolForEraError::MissingNames(field))?;
        let YearNamesV1::Eras(era_names) = year_names else {
            return Err(GetSymbolForEraError::MissingNames(field));
        };
        era_names
            .get(era_code.0.as_str().into())
            .ok_or(GetSymbolForEraError::Missing)
    }
}

impl RawDateTimeNamesBorrowed<'_> {
    pub(crate) fn get_name_for_day_period(
        &self,
        field_symbol: fields::DayPeriod,
        field_length: FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, GetNameForDayPeriodError> {
        use fields::DayPeriod::NoonMidnight;
        let field = fields::Field {
            symbol: FieldSymbol::DayPeriod(field_symbol),
            length: field_length,
        };
        // UTS 35 says that "a..aaa" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        let dayperiod_names = self
            .dayperiod_names
            .get_with_variables(field_length)
            .ok_or(GetNameForDayPeriodError::MissingNames(field))?;
        let option_value: Option<&str> = match (field_symbol, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => dayperiod_names.midnight().or_else(|| dayperiod_names.am()),
            (NoonMidnight, 12, true) => dayperiod_names.noon().or_else(|| dayperiod_names.pm()),
            (_, hour, _) if hour < 12 => dayperiod_names.am(),
            _ => dayperiod_names.pm(),
        };
        option_value.ok_or(GetNameForDayPeriodError::MissingNames(field))
    }
}

impl<'data> RawDateTimeNamesBorrowed<'data> {
    pub(crate) fn get_payloads(&self) -> crate::time_zone::TimeZoneDataPayloadsBorrowed<'data> {
        TimeZoneDataPayloadsBorrowed {
            essentials: self.zone_essentials.get_option(),
            locations: self.locations.get_option(),
            mz_generic_long: self.mz_generic_long.get_option(),
            mz_generic_short: self.mz_generic_short.get_option(),
            mz_specific_long: self.mz_specific_long.get_option(),
            mz_specific_short: self.mz_specific_short.get_option(),
            mz_periods: self.mz_periods.get_option(),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use icu_calendar::{DateTime, Gregorian};
    use icu_locale_core::locale;
    use writeable::assert_try_writeable_eq;

    #[test]
    fn test_basic_pattern_formatting() {
        let locale = locale!("en").into();
        let mut names: TypedDateTimeNames<Gregorian> =
            TypedDateTimeNames::try_new(&locale).unwrap();
        names
            .load_month_names(
                &crate::provider::Baked,
                fields::Month::Format,
                fields::FieldLength::Wide,
            )
            .unwrap()
            .load_weekday_names(
                &crate::provider::Baked,
                fields::Weekday::Format,
                fields::FieldLength::Abbreviated,
            )
            .unwrap()
            .load_year_names(&crate::provider::Baked, FieldLength::Narrow)
            .unwrap()
            .load_day_period_names(&crate::provider::Baked, FieldLength::Abbreviated)
            .unwrap();
        let pattern: DateTimePattern = "'It is' E, MMMM d, y GGGGG 'at' hh:mm a'!'"
            .parse()
            .unwrap();
        let datetime = DateTime::try_new_gregorian(2023, 10, 25, 15, 0, 55).unwrap();
        let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

        assert_try_writeable_eq!(
            formatted_pattern,
            "It is Wed, October 25, 2023 A at 03:00 PM!",
            Ok(()),
        );
    }

    #[test]
    fn test_era_coverage() {
        let locale = locale!("uk").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<G>",
                field_length: FieldLength::Abbreviated,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GG>",
                field_length: FieldLength::Abbreviated,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GGG>",
                field_length: FieldLength::Abbreviated,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GGGG>",
                field_length: FieldLength::Wide,
                expected: "<нашої ери>",
            },
            TestCase {
                pattern: "<GGGGG>",
                field_length: FieldLength::Narrow,
                expected: "<н.е.>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_year_names(&crate::provider::Baked, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

            assert_try_writeable_eq!(formatted_pattern, expected, Ok(()), "{cas:?}");
        }
    }

    #[test]
    fn test_month_coverage() {
        // Ukrainian has different values for format and standalone
        let locale = locale!("uk").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_symbol: fields::Month,
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            // 'M' and 'MM' are numeric
            TestCase {
                pattern: "<MMM>",
                field_symbol: fields::Month::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<лист.>",
            },
            TestCase {
                pattern: "<MMMM>",
                field_symbol: fields::Month::Format,
                field_length: FieldLength::Wide,
                expected: "<листопада>",
            },
            TestCase {
                pattern: "<MMMMM>",
                field_symbol: fields::Month::Format,
                field_length: FieldLength::Narrow,
                expected: "<л>",
            },
            // 'L' and 'LL' are numeric
            TestCase {
                pattern: "<LLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: FieldLength::Abbreviated,
                expected: "<лист.>",
            },
            TestCase {
                pattern: "<LLLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: FieldLength::Wide,
                expected: "<листопад>",
            },
            TestCase {
                pattern: "<LLLLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: FieldLength::Narrow,
                expected: "<Л>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_symbol,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_month_names(&crate::provider::Baked, field_symbol, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

            assert_try_writeable_eq!(formatted_pattern, expected, Ok(()), "{cas:?}");
        }
    }

    #[test]
    fn test_weekday_coverage() {
        let locale = locale!("uk").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_symbol: fields::Weekday,
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<E>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Wide,
                expected: "<пʼятниця>",
            },
            TestCase {
                pattern: "<EEEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Narrow,
                expected: "<П>",
            },
            TestCase {
                pattern: "<EEEEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Six,
                expected: "<пт>",
            },
            // 'e' and 'ee' are numeric
            TestCase {
                pattern: "<eee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<eeee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Wide,
                expected: "<пʼятниця>",
            },
            TestCase {
                pattern: "<eeeee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Narrow,
                expected: "<П>",
            },
            TestCase {
                pattern: "<eeeeee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Six,
                expected: "<пт>",
            },
            // 'c' and 'cc' are numeric
            TestCase {
                pattern: "<ccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<cccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Wide,
                expected: "<пʼятниця>",
            },
            TestCase {
                pattern: "<ccccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Narrow,
                expected: "<П>",
            },
            TestCase {
                pattern: "<cccccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Six,
                expected: "<пт>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_symbol,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_weekday_names(&crate::provider::Baked, field_symbol, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

            assert_try_writeable_eq!(formatted_pattern, expected, Ok(()), "{cas:?}");
        }
    }

    #[test]
    fn test_dayperiod_coverage() {
        // Thai has different values for different lengths of day periods
        // TODO(#487): Support flexible day periods, too
        let locale = locale!("th").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<a>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aa>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aaa>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aaaa>",
                field_length: FieldLength::Wide,
                expected: "<หลังเที่ยง>",
            },
            TestCase {
                pattern: "<aaaaa>",
                field_length: FieldLength::Narrow,
                expected: "<p>",
            },
            TestCase {
                pattern: "<b>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bb>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bbb>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bbbb>",
                field_length: FieldLength::Wide,
                expected: "<หลังเที่ยง>",
            },
            TestCase {
                pattern: "<bbbbb>",
                field_length: FieldLength::Narrow,
                expected: "<p>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_day_period_names(&crate::provider::Baked, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

            assert_try_writeable_eq!(formatted_pattern, expected, Ok(()), "{cas:?}");
        }
    }
}
