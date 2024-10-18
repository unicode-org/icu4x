// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use crate::external_loaders::*;
use crate::format::datetime::try_write_pattern_items;
use crate::format::datetime::DateTimeWriteError;
use crate::format::neo::*;
use crate::input::ExtractedInput;
use crate::neo_marker::DateInputMarkers;
use crate::neo_marker::HasConstComponents;
use crate::neo_marker::{
    AllInputMarkers, ConvertCalendar, DateDataMarkers, DateTimeMarkers, GetField,
    IsAnyCalendarKind, IsInCalendar, IsRuntimeComponents, TimeMarkers, TypedDateDataMarkers,
    ZoneMarkers,
};
use crate::neo_pattern::DateTimePattern;
use crate::neo_skeleton::{NeoComponents, NeoSkeletonLength};
use crate::options::preferences::HourCycle;
use crate::provider::neo::*;
use crate::provider::ErasedPackedPatterns;
use crate::raw::neo::*;
use crate::scaffold::*;
use crate::MismatchedCalendarError;
use core::fmt;
use core::marker::PhantomData;
use icu_calendar::any_calendar::IntoAnyCalendar;
use icu_calendar::provider::{
    ChineseCacheV1Marker, DangiCacheV1Marker, IslamicObservationalCacheV1Marker,
    IslamicUmmAlQuraCacheV1Marker, JapaneseErasV1Marker, JapaneseExtendedErasV1Marker,
};
use icu_calendar::AnyCalendar;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_provider::prelude::*;
use icu_timezone::scaffold::IntoOption;
use writeable::TryWriteable;

/// Helper macro for generating any/buffer constructors in this file.
macro_rules! gen_any_buffer_constructors_with_external_loader {
    (@runtime_fset, $fset:ident, $compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident) => {
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::$compiled_fn)]
        pub fn $any_fn<P>(
            provider: &P,
            locale: &DataLocale,
            skeleton: $fset,
        ) -> Result<Self, LoadError>
        where
            P: AnyProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_downcasting(),
                &ExternalLoaderAny(provider),
                locale,
                RawNeoOptions::from_field_set_and_locale(&skeleton, locale),
                skeleton.get_field(),
            )
        }
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::$compiled_fn)]
        #[cfg(feature = "serde")]
        pub fn $buffer_fn<P>(
            provider: &P,
            locale: &DataLocale,
            skeleton: $fset,
        ) -> Result<Self, LoadError>
        where
            P: BufferProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_deserializing(),
                &ExternalLoaderBuffer(provider),
                locale,
                RawNeoOptions::from_field_set_and_locale(&skeleton, locale),
                skeleton.get_field(),
            )
        }
    };
    (@compiletime_fset, $fset:ident, $compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident) => {
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::$compiled_fn)]
        pub fn $any_fn<P>(
            provider: &P,
            locale: &DataLocale,
            options: $fset,
        ) -> Result<Self, LoadError>
        where
            P: AnyProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_downcasting(),
                &ExternalLoaderAny(provider),
                locale,
                RawNeoOptions::from_field_set_and_locale(&options, locale),
                $fset::COMPONENTS,
            )
        }
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::$compiled_fn)]
        #[cfg(feature = "serde")]
        pub fn $buffer_fn<P>(
            provider: &P,
            locale: &DataLocale,
            options: $fset,
        ) -> Result<Self, LoadError>
        where
            P: BufferProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_deserializing(),
                &ExternalLoaderBuffer(provider),
                locale,
                RawNeoOptions::from_field_set_and_locale(&options, locale),
                $fset::COMPONENTS,
            )
        }
    };
}

impl RawNeoOptions {
    pub(crate) fn from_field_set_and_locale<FSet>(field_set: &FSet, locale: &DataLocale) -> Self
    where
        FSet: DateTimeMarkers,
        FSet: GetField<FSet::LengthOption>,
        FSet: GetField<FSet::AlignmentOption>,
        FSet: GetField<FSet::YearStyleOption>,
        FSet: GetField<FSet::FractionalSecondDigitsOption>,
    {
        // TODO: Return an error if there are more options than field set
        let hour_cycle = locale
            .get_unicode_ext(&icu_locale_core::extensions::unicode::key!("hc"))
            .as_ref()
            .and_then(HourCycle::from_locale_value);
        Self {
            length: match GetField::<FSet::LengthOption>::get_field(field_set).into_option() {
                Some(length) => length,
                None => {
                    debug_assert!(false, "unreachable");
                    NeoSkeletonLength::Medium
                }
            },
            alignment: GetField::<FSet::AlignmentOption>::get_field(field_set).into_option(),
            year_style: GetField::<FSet::YearStyleOption>::get_field(field_set).into_option(),
            fractional_second_digits: GetField::<FSet::FractionalSecondDigitsOption>::get_field(
                field_set,
            )
            .into_option(),
            hour_cycle,
        }
    }
}

size_test!(TypedNeoFormatter<icu_calendar::Gregorian, crate::neo_marker::NeoYearMonthDayMarker>, typed_neo_year_month_day_formatter_size, 496);

/// [`TypedNeoFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at compile time.
///
/// For more details, please read the [crate root docs][crate].
///
#[doc = typed_neo_year_month_day_formatter_size!()]
#[derive(Debug)]
pub struct TypedNeoFormatter<C: CldrCalendar, FSet: DateTimeNamesMarker> {
    selection: DateTimeZonePatternSelectionData,
    names: RawDateTimeNames<FSet>,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar, FSet: DateTimeMarkers + HasConstComponents> TypedNeoFormatter<C, FSet>
where
    FSet::D: TypedDateDataMarkers<C>,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<FSet::LengthOption>,
    FSet: GetField<FSet::AlignmentOption>,
    FSet: GetField<FSet::YearStyleOption>,
    FSet: GetField<FSet::FractionalSecondDigitsOption>,
{
    /// Creates a new [`TypedNeoFormatter`] from compiled data with
    /// datetime components specified at build time.
    ///
    /// Use this constructor for optimal data size and memory use
    /// if you know the required datetime components at build time.
    /// If you do not know the datetime components until runtime,
    /// use a `with_components` constructor.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = TypedNeoFormatter::try_new(
    ///     &locale!("es-MX").into(),
    ///     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Long),
    /// )
    /// .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&Date::try_new_gregorian(2023, 12, 20).unwrap()),
    ///     "20 de diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale, field_set: FSet) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Date formatting markers
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            RawNeoOptions::from_field_set_and_locale(&field_set, locale),
            FSet::COMPONENTS,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        @compiletime_fset,
        FSet,
        try_new,
        try_new_with_any_provider,
        try_new_with_buffer_provider,
        try_new_internal
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        field_set: FSet,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting markers
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>
            // FixedDecimalFormatter markers
            + DataProvider<DecimalSymbolsV1Marker>,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            RawNeoOptions::from_field_set_and_locale(&field_set, locale),
            FSet::COMPONENTS,
        )
    }
}

impl<C: CldrCalendar, FSet: DateTimeMarkers + IsRuntimeComponents> TypedNeoFormatter<C, FSet>
where
    FSet::D: TypedDateDataMarkers<C>,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<FSet::LengthOption>,
    FSet: GetField<FSet::AlignmentOption>,
    FSet: GetField<FSet::YearStyleOption>,
    FSet: GetField<FSet::FractionalSecondDigitsOption>,
{
    /// Creates a new [`TypedNeoFormatter`] from compiled data with
    /// datetime components specified at runtime.
    ///
    /// If you know the datetime components at build time, use
    /// [`TypedNeoFormatter::try_new`] for smaller data size and memory use.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// Date components:
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_skeleton::NeoDateComponents;
    /// use icu::datetime::neo_skeleton::NeoDateSkeleton;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = TypedNeoFormatter::<Gregorian, _>::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoDateSkeleton::for_length_and_components(
    ///         NeoSkeletonLength::Medium,
    ///         NeoDateComponents::DayWeekday,
    ///     ),
    /// )
    /// .unwrap();
    /// let dt = Date::try_new_gregorian(2024, 1, 10).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.format(&dt), "mié 10");
    /// ```
    ///
    /// Calendar period components:
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_skeleton::NeoCalendarPeriodComponents;
    /// use icu::datetime::neo_skeleton::NeoCalendarPeriodSkeleton;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = TypedNeoFormatter::<Gregorian, _>::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoCalendarPeriodSkeleton::for_length_and_components(
    ///         NeoSkeletonLength::Medium,
    ///         NeoCalendarPeriodComponents::YearMonth,
    ///     ),
    /// )
    /// .unwrap();
    /// let dt = Date::try_new_gregorian(2024, 1, 10).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.format(&dt), "ene 2024");
    /// ```
    ///
    /// Time components:
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::calendar::Time;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::datetime::neo_skeleton::NeoTimeSkeleton;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = TypedNeoFormatter::<Gregorian, _>::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoTimeSkeleton::for_length_and_components(
    ///         NeoSkeletonLength::Medium,
    ///         NeoTimeComponents::Hour,
    ///     ),
    /// )
    /// .unwrap();
    /// let dt = Time::try_new(16, 20, 0, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.format(&dt), "04 p.m.");
    /// ```
    ///
    /// Date and time components:
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_skeleton::NeoDateComponents;
    /// use icu::datetime::neo_skeleton::NeoDateTimeComponents;
    /// use icu::datetime::neo_skeleton::NeoDateTimeSkeleton;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = TypedNeoFormatter::<Gregorian, _>::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoDateTimeSkeleton::for_length_and_components(
    ///         NeoSkeletonLength::Long,
    ///         NeoDateTimeComponents::DateTime(
    ///             NeoDateComponents::Weekday,
    ///             NeoTimeComponents::HourMinute,
    ///         ),
    ///     ),
    /// )
    /// .unwrap();
    /// let dt =
    ///     DateTime::try_new_gregorian(2024, 1, 10, 16, 20, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.format(&dt), "miércoles 4:20 p.m.");
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_skeleton(locale: &DataLocale, skeleton: FSet) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Date formatting markers
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            RawNeoOptions::from_field_set_and_locale(&skeleton, locale),
            skeleton.get_field(),
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        @runtime_fset,
        FSet,
        try_new_with_skeleton,
        try_new_with_skeleton_with_any_provider,
        try_new_with_skeleton_with_buffer_provider,
        try_new_internal
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_with_skeleton_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        skeleton: FSet,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting markers
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>
            // FixedDecimalFormatter markers
            + DataProvider<DecimalSymbolsV1Marker>,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            RawNeoOptions::from_field_set_and_locale(&skeleton, locale),
            skeleton.get_field(),
        )
    }
}

impl<C: CldrCalendar, FSet: DateTimeMarkers> TypedNeoFormatter<C, FSet>
where
    FSet::D: TypedDateDataMarkers<C>,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        options: RawNeoOptions,
        components: NeoComponents,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting markers
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<FSet::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>,
        L: FixedDecimalFormatterLoader,
    {
        // TODO: Remove this when NeoOptions is gone
        let mut options = options;
        options.hour_cycle = locale
            .get_unicode_ext(&icu_locale_core::extensions::unicode::key!("hc"))
            .as_ref()
            .and_then(HourCycle::from_locale_value);
        // END TODO

        let selection = DateTimeZonePatternSelectionData::try_new_with_skeleton(
            &<FSet::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker::bind(provider),
            &<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker::bind(provider),
            &FSet::GluePatternV1Marker::bind(provider),
            locale,
            components,
            options,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_number_formatting();
        names.load_for_pattern(
            &<FSet::D as TypedDateDataMarkers<C>>::YearNamesV1Marker::bind(provider),
            &<FSet::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker::bind(provider),
            &<FSet::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker::bind(provider),
            &<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::EssentialsV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::LocationsV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::GenericLongV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::GenericShortV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::SpecificLongV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::SpecificShortV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker::bind(provider),
            loader, // fixed decimal formatter
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self {
            selection,
            names,
            _calendar: PhantomData,
        })
    }
}

impl<C: CldrCalendar, FSet: DateTimeMarkers> TypedNeoFormatter<C, FSet>
where
    FSet::D: DateInputMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    /// Formats a datetime. Calendars and fields must match at compile time.
    ///
    /// # Examples
    ///
    /// Mismatched calendars will not compile:
    ///
    /// ```compile_fail
    /// use icu::calendar::Date;
    /// use icu::calendar::cal::Buddhist;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    ///
    /// let formatter =
    ///     TypedNeoFormatter::<Buddhist, _>::try_new(
    ///         &locale!("es-MX").into(),
    ///         NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Long),
    ///     )
    ///     .unwrap();
    ///
    /// // type mismatch resolving `<Gregorian as AsCalendar>::Calendar == Buddhist`
    /// formatter.format(&Date::try_new_gregorian(2023, 12, 20).unwrap());
    /// ```
    ///
    /// A time cannot be passed into the formatter when a date is expected:
    ///
    /// ```compile_fail
    /// use icu::calendar::Time;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    ///
    /// let formatter =
    ///     TypedNeoFormatter::<Gregorian, _>::try_new(
    ///         &locale!("es-MX").into(),
    ///         NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Long),
    ///     )
    ///     .unwrap();
    ///
    /// // the trait `GetField<AnyCalendarKind>`
    /// // is not implemented for `icu::icu_calendar::Time`
    /// formatter.format(&Time::try_new(0, 0, 0, 0).unwrap());
    /// ```
    pub fn format<I>(&self, input: &I) -> FormattedNeoDateTime
    where
        I: ?Sized + IsInCalendar<C> + AllInputMarkers<FSet>,
    {
        let input = ExtractedInput::extract_from_neo_input::<FSet::D, FSet::T, FSet::Z, I>(input);
        FormattedNeoDateTime {
            pattern: self.selection.select(&input),
            input,
            names: self.names.as_borrowed(),
        }
    }
}

size_test!(
    NeoFormatter<crate::neo_marker::NeoYearMonthDayMarker>,
    neo_year_month_day_formatter_size,
    552
);

/// [`NeoFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at runtime.
///
/// For more details, please read the [crate root docs][crate].
///
#[doc = neo_year_month_day_formatter_size!()]
#[derive(Debug)]
pub struct NeoFormatter<FSet: DateTimeNamesMarker> {
    selection: DateTimeZonePatternSelectionData,
    names: RawDateTimeNames<FSet>,
    calendar: AnyCalendar,
}

impl<FSet: DateTimeMarkers + HasConstComponents> NeoFormatter<FSet>
where
    FSet::D: DateDataMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<FSet::LengthOption>,
    FSet: GetField<FSet::AlignmentOption>,
    FSet: GetField<FSet::YearStyleOption>,
    FSet: GetField<FSet::FractionalSecondDigitsOption>,
{
    /// Creates a new [`NeoFormatter`] from compiled data with
    /// datetime components specified at build time.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// Use this constructor for optimal data size and memory use
    /// if you know the required datetime components at build time.
    /// If you do not know the datetime components until runtime,
    /// use a `with_components` constructor.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use icu::calendar::{any_calendar::AnyCalendar, DateTime};
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use std::str::FromStr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let length = NeoSkeletonLength::Medium;
    /// let locale = locale!("en-u-ca-hebrew");
    ///
    /// let formatter = NeoFormatter::try_new(
    ///     &locale.into(),
    ///     NeoYearMonthDayMarker::with_length(length),
    /// )
    /// .unwrap();
    ///
    /// let datetime = DateTime::try_new_iso(2024, 5, 8, 0, 0, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.convert_and_format(&datetime),
    ///     "30 Nisan 5784"
    /// );
    /// ```
    ///
    /// [`AnyCalendarKind`]: icu_calendar::AnyCalendarKind
    #[inline(never)]
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale, field_set: FSet) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
    // Date formatting markers
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Roc>
            + DataProvider<<FSet::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            RawNeoOptions::from_field_set_and_locale(&field_set, locale),
            FSet::COMPONENTS,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        @compiletime_fset,
        FSet,
        try_new,
        try_new_with_any_provider,
        try_new_with_buffer_provider,
        try_new_internal
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        field_set: FSet,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
    // Date formatting markers
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Roc>
            + DataProvider<<FSet::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>
    // AnyCalendar constructor markers
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<IslamicObservationalCacheV1Marker>
            + DataProvider<IslamicUmmAlQuraCacheV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
    // FixedDecimalFormatter markers
            + DataProvider<DecimalSymbolsV1Marker>
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            RawNeoOptions::from_field_set_and_locale(&field_set, locale),
            FSet::COMPONENTS,
        )
    }
}

impl<FSet: DateTimeMarkers + IsRuntimeComponents> NeoFormatter<FSet>
where
    FSet::D: DateDataMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<FSet::LengthOption>,
    FSet: GetField<FSet::AlignmentOption>,
    FSet: GetField<FSet::YearStyleOption>,
    FSet: GetField<FSet::FractionalSecondDigitsOption>,
{
    /// Creates a new [`NeoFormatter`] from compiled data with
    /// datetime components specified at runtime.
    ///
    /// If you know the datetime components at build time, use
    /// [`NeoFormatter::try_new`] for smaller data size and memory use.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// Date components:
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_skeleton::NeoDateComponents;
    /// use icu::datetime::neo_skeleton::NeoDateSkeleton;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = NeoFormatter::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoDateSkeleton::for_length_and_components(
    ///         NeoSkeletonLength::Medium,
    ///         NeoDateComponents::DayWeekday,
    ///     ),
    /// )
    /// .unwrap();
    /// let dt = Date::try_new_iso(2024, 1, 10).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.convert_and_format(&dt), "mié 10");
    /// ```
    ///
    /// Calendar period components:
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_skeleton::NeoCalendarPeriodComponents;
    /// use icu::datetime::neo_skeleton::NeoCalendarPeriodSkeleton;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = NeoFormatter::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoCalendarPeriodSkeleton::for_length_and_components(
    ///         NeoSkeletonLength::Medium,
    ///         NeoCalendarPeriodComponents::YearMonth,
    ///     ),
    /// )
    /// .unwrap();
    /// let dt = Date::try_new_iso(2024, 1, 10).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.convert_and_format(&dt), "ene 2024");
    /// ```
    ///
    /// Time components:
    ///
    /// ```
    /// use icu::calendar::Time;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::datetime::neo_skeleton::NeoTimeSkeleton;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = NeoFormatter::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoTimeSkeleton::for_length_and_components(
    ///         NeoSkeletonLength::Medium,
    ///         NeoTimeComponents::Hour,
    ///     ),
    /// )
    /// .unwrap();
    /// let dt = Time::try_new(16, 20, 0, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.convert_and_format(&dt), "04 p.m.");
    /// ```
    ///
    /// Date and time components:
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_skeleton::NeoDateComponents;
    /// use icu::datetime::neo_skeleton::NeoDateTimeComponents;
    /// use icu::datetime::neo_skeleton::NeoDateTimeSkeleton;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = NeoFormatter::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoDateTimeSkeleton::for_length_and_components(
    ///         NeoSkeletonLength::Long,
    ///         NeoDateTimeComponents::DateTime(
    ///             NeoDateComponents::Weekday,
    ///             NeoTimeComponents::HourMinute,
    ///         ),
    ///     ),
    /// )
    /// .unwrap();
    /// let dt = DateTime::try_new_iso(2024, 1, 10, 16, 20, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.convert_and_format(&dt),
    ///     "miércoles 4:20 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_skeleton(locale: &DataLocale, skeleton: FSet) -> Result<Self, LoadError>
    where
    crate::provider::Baked: Sized
    // Date formatting markers
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Roc>
            + DataProvider<<FSet::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            RawNeoOptions::from_field_set_and_locale(&skeleton, locale),
            skeleton.get_field(),
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        @runtime_fset,
        FSet,
        try_new_with_skeleton,
        try_new_with_skeleton_with_any_provider,
        try_new_with_skeleton_with_buffer_provider,
        try_new_internal
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_with_skeleton_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        skeleton: FSet,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
    // Date formatting markers
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Roc>
            + DataProvider<<FSet::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>
    // AnyCalendar constructor markers
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<IslamicObservationalCacheV1Marker>
            + DataProvider<IslamicUmmAlQuraCacheV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
    // FixedDecimalFormatter markers
            + DataProvider<DecimalSymbolsV1Marker>
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            RawNeoOptions::from_field_set_and_locale(&skeleton, locale),
            skeleton.get_field(),
        )
    }
}

impl<FSet: DateTimeMarkers> NeoFormatter<FSet>
where
    FSet::D: DateDataMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        options: RawNeoOptions,
        components: NeoComponents,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
    // Date formatting markers
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Buddhist>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Chinese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Coptic>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Dangi>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Ethiopian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::EthiopianAmeteAlem>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Gregorian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Hebrew>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Indian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicCivil>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicObservational>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicTabular>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::IslamicUmmAlQura>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Japanese>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::JapaneseExtended>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Persian>
            + DataProvider<<<FSet::D as DateDataMarkers>::Skel as CalMarkers<ErasedPackedPatterns>>::Roc>
            + DataProvider<<FSet::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::LocationsV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker>
            + DataProvider<FSet::GluePatternV1Marker>,
        L: FixedDecimalFormatterLoader + AnyCalendarLoader,
    {
        // TODO: Remove this when NeoOptions is gone
        let mut options = options;
        options.hour_cycle = locale
            .get_unicode_ext(&icu_locale_core::extensions::unicode::key!("hc"))
            .as_ref()
            .and_then(HourCycle::from_locale_value);
        // END TODO

        let calendar = AnyCalendarLoader::load(loader, locale).map_err(LoadError::Data)?;
        let kind = calendar.kind();
        let selection = DateTimeZonePatternSelectionData::try_new_with_skeleton(
            &AnyCalendarProvider::<<FSet::D as DateDataMarkers>::Skel, _>::new(provider, kind),
            &<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker::bind(provider),
            &FSet::GluePatternV1Marker::bind(provider),
            locale,
            components,
            options,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_number_formatting();
        names.load_for_pattern(
            &AnyCalendarProvider::<<FSet::D as DateDataMarkers>::Year, _>::new(provider, kind),
            &AnyCalendarProvider::<<FSet::D as DateDataMarkers>::Month, _>::new(provider, kind),
            &<FSet::D as DateDataMarkers>::WeekdayNamesV1Marker::bind(provider),
            &<FSet::T as TimeMarkers>::DayPeriodNamesV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::EssentialsV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::LocationsV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::GenericLongV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::GenericShortV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::SpecificLongV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::SpecificShortV1Marker::bind(provider),
            &<FSet::Z as ZoneMarkers>::MetazonePeriodV1Marker::bind(provider),
            loader, // fixed decimal formatter
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self {
            selection,
            names,
            calendar,
        })
    }
}

impl<FSet: DateTimeMarkers> NeoFormatter<FSet>
where
    FSet::D: DateInputMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    /// Formats a datetime, checking that the calendar system is correct.
    ///
    /// If the datetime is not in the same calendar system as the formatter,
    /// an error is returned.
    ///
    /// # Examples
    ///
    /// Mismatched calendars will return an error:
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::MismatchedCalendarError;
    /// use icu::locale::locale;
    ///
    /// let formatter = NeoFormatter::try_new(
    ///     &locale!("en-u-ca-hebrew").into(),
    ///     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Long),
    /// )
    /// .unwrap();
    ///
    /// let date = Date::try_new_gregorian(2023, 12, 20).unwrap();
    ///
    /// assert!(matches!(
    ///     formatter.strict_format(&date),
    ///     Err(MismatchedCalendarError { .. })
    /// ));
    /// ```
    ///
    /// A time cannot be passed into the formatter when a date is expected:
    ///
    /// ```compile_fail
    /// use icu::calendar::Time;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    ///
    /// let formatter = NeoFormatter::try_new(
    ///     &locale!("es-MX").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    ///
    /// // the trait `GetField<AnyCalendarKind>`
    /// // is not implemented for `icu::icu_calendar::Time`
    /// formatter.strict_format(&Time::try_new(0, 0, 0, 0).unwrap());
    /// ```
    pub fn strict_format<I>(
        &self,
        datetime: &I,
    ) -> Result<FormattedNeoDateTime, crate::MismatchedCalendarError>
    where
        I: ?Sized + IsAnyCalendarKind + AllInputMarkers<FSet>,
    {
        if !datetime.is_any_calendar_kind(self.calendar.kind()) {
            return Err(crate::MismatchedCalendarError {
                this_kind: self.calendar.kind(),
                date_kind:
                    GetField::<<FSet::D as DateInputMarkers>::AnyCalendarKindInput>::get_field(
                        datetime,
                    )
                    .into_option(),
            });
        }
        let datetime =
            ExtractedInput::extract_from_neo_input::<FSet::D, FSet::T, FSet::Z, I>(datetime);
        Ok(FormattedNeoDateTime {
            pattern: self.selection.select(&datetime),
            input: datetime,
            names: self.names.as_borrowed(),
        })
    }

    /// Formats a datetime after first converting it
    /// to the formatter's calendar.
    ///
    /// # Examples
    ///
    /// Mismatched calendars convert and format automatically:
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::MismatchedCalendarError;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = NeoFormatter::try_new(
    ///     &locale!("en-u-ca-hebrew").into(),
    ///     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Long),
    /// )
    /// .unwrap();
    ///
    /// let date = Date::try_new_roc(113, 5, 8).unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.convert_and_format(&date),
    ///     "30 Nisan 5784"
    /// );
    /// ```
    ///
    /// A time cannot be passed into the formatter when a date is expected:
    ///
    /// ```compile_fail
    /// use icu::calendar::Time;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    ///
    /// let formatter = NeoFormatter::try_new(
    ///     &locale!("es-MX").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    ///
    /// // the trait `GetField<AnyCalendarKind>`
    /// // is not implemented for `icu::icu_calendar::Time`
    /// formatter.convert_and_format(&Time::try_new(0, 0, 0, 0).unwrap());
    /// ```
    pub fn convert_and_format<'a, I>(&'a self, datetime: &I) -> FormattedNeoDateTime<'a>
    where
        I: ?Sized + ConvertCalendar,
        I::Converted<'a>: Sized + AllInputMarkers<FSet>,
    {
        let datetime = datetime.to_calendar(&self.calendar);
        let datetime =
            ExtractedInput::extract_from_neo_input::<FSet::D, FSet::T, FSet::Z, I::Converted<'a>>(
                &datetime,
            );
        FormattedNeoDateTime {
            pattern: self.selection.select(&datetime),
            input: datetime,
            names: self.names.as_borrowed(),
        }
    }
}

impl<C: CldrCalendar, FSet: DateTimeMarkers> TypedNeoFormatter<C, FSet> {
    /// Make this [`TypedNeoFormatter`] adopt a calendar so it can format any date.
    ///
    /// This is useful if you need a [`NeoFormatter`] but know the calendar system ahead of time,
    /// so that you do not need to link extra data you aren't using.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::cal::Hebrew;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = TypedNeoFormatter::try_new(
    ///     &locale!("en").into(),
    ///     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Long),
    /// )
    /// .unwrap()
    /// .into_formatter(Hebrew::new());
    ///
    /// let date = Date::try_new_iso(2024, 10, 14).unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.convert_and_format(&date),
    ///     "12 Tishri 5785"
    /// );
    /// ```
    pub fn into_formatter(self, calendar: C) -> NeoFormatter<FSet>
    where
        C: IntoAnyCalendar,
    {
        NeoFormatter {
            selection: self.selection,
            names: self.names,
            calendar: calendar.to_any(),
        }
    }
}

impl<FSet: DateTimeMarkers> NeoFormatter<FSet> {
    /// Attempt to convert this [`NeoFormatter`] into one with a specific calendar.
    ///
    /// Returns an error if the type parameter does not match the inner calendar.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::cal::Hebrew;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = NeoFormatter::try_new(
    ///     &locale!("en-u-ca-hebrew").into(),
    ///     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Long),
    /// )
    /// .unwrap()
    /// .try_into_typed_formatter::<Hebrew>()
    /// .unwrap();
    ///
    /// let date = Date::try_new_hebrew(5785, 1, 12).unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&date),
    ///     "12 Tishri 5785"
    /// );
    /// ```
    ///
    /// An error occurs if the calendars don't match:
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::cal::Hebrew;
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::MismatchedCalendarError;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    ///
    /// let result = NeoFormatter::try_new(
    ///     &locale!("en-u-ca-buddhist").into(),
    ///     NeoYearMonthDayMarker::with_length(NeoSkeletonLength::Long),
    /// )
    /// .unwrap()
    /// .try_into_typed_formatter::<Hebrew>();
    ///
    /// assert!(matches!(
    ///     result,
    ///     Err(MismatchedCalendarError { .. })
    /// ));
    /// ```
    pub fn try_into_typed_formatter<C>(
        self,
    ) -> Result<TypedNeoFormatter<C, FSet>, MismatchedCalendarError>
    where
        C: CldrCalendar + IntoAnyCalendar,
    {
        if let Err(cal) = C::from_any(self.calendar) {
            return Err(MismatchedCalendarError {
                this_kind: cal.kind(),
                date_kind: None,
            });
        }
        Ok(TypedNeoFormatter {
            selection: self.selection,
            names: self.names,
            _calendar: PhantomData,
        })
    }
}

/// An intermediate type during a datetime formatting operation.
///
/// Not intended to be stored: convert to a string first.
#[derive(Debug)]
pub struct FormattedNeoDateTime<'a> {
    pattern: DateTimeZonePatternDataBorrowed<'a>,
    input: ExtractedInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl TryWriteable for FormattedNeoDateTime<'_> {
    type Error = DateTimeWriteError;

    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        try_write_pattern_items(
            self.pattern.metadata(),
            self.pattern.iter_items(),
            &self.input,
            &self.names,
            self.names.fixed_decimal_formatter,
            sink,
        )
    }

    // TODO(#489): Implement writeable_length_hint
}

impl FormattedNeoDateTime<'_> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }

    /// Gets the formatted result as a string.
    pub fn to_string_lossy(&self) -> alloc::string::String {
        match self.try_write_to_string() {
            Err((_, s)) | Ok(s) => s.into_owned(),
        }
    }
}
