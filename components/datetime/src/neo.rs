// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use crate::calendar::AnyCalendarProvider;
use crate::external_loaders::*;
use crate::format::datetime::try_write_pattern_items;
use crate::format::datetime::DateTimeWriteError;
use crate::format::neo::*;
use crate::input::ExtractedDateTimeInput;
use crate::neo_marker::DateInputMarkers;
use crate::neo_marker::HasConstComponents;
use crate::neo_marker::{
    AllInputMarkers, CalMarkers, ConvertCalendar, DateDataMarkers, DateTimeMarkers,
    IsAnyCalendarKind, IsInCalendar, IsRuntimeComponents, NeoGetField, TimeMarkers,
    TypedDateDataMarkers, ZoneMarkers,
};
use crate::neo_pattern::DateTimePattern;
use crate::neo_skeleton::{NeoComponents, NeoSkeletonLength};
use crate::pattern::CoarseHourCycle;
use crate::provider::neo::*;
use crate::raw::neo::*;
use crate::CldrCalendar;
use core::fmt;
use core::marker::PhantomData;
use icu_calendar::provider::{
    ChineseCacheV1Marker, DangiCacheV1Marker, IslamicObservationalCacheV1Marker,
    IslamicUmmAlQuraCacheV1Marker, JapaneseErasV1Marker, JapaneseExtendedErasV1Marker,
    WeekDataV2Marker,
};
use icu_calendar::AnyCalendar;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_provider::prelude::*;
use writeable::TryWriteable;

/// Helper macro for generating any/buffer constructors in this file.
macro_rules! gen_any_buffer_constructors_with_external_loader {
    ($compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident, $($arg:ident: $ty:ty),+) => {
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::$compiled_fn)]
        pub fn $any_fn<P>(
            provider: &P,
            locale: &DataLocale,
            $($arg: $ty),+
        ) -> Result<Self, LoadError>
        where
            P: AnyProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_downcasting(),
                &ExternalLoaderAny(provider),
                locale,
                $($arg.into()),+
            )
        }
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::$compiled_fn)]
        #[cfg(feature = "serde")]
        pub fn $buffer_fn<P>(
            provider: &P,
            locale: &DataLocale,
            $($arg: $ty),+
        ) -> Result<Self, LoadError>
        where
            P: BufferProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_deserializing(),
                &ExternalLoaderBuffer(provider),
                locale,
                $($arg.into()),+
            )
        }
    };
    (R, $compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident, $($arg:ident: $ty:ty),+) => {
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::$compiled_fn)]
        pub fn $any_fn<P>(
            provider: &P,
            locale: &DataLocale,
            $($arg: $ty),+
        ) -> Result<Self, LoadError>
        where
            P: AnyProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_downcasting(),
                &ExternalLoaderAny(provider),
                locale,
                R::COMPONENTS,
                $($arg.into()),+
            )
        }
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::$compiled_fn)]
        #[cfg(feature = "serde")]
        pub fn $buffer_fn<P>(
            provider: &P,
            locale: &DataLocale,
            $($arg: $ty),+
        ) -> Result<Self, LoadError>
        where
            P: BufferProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_deserializing(),
                &ExternalLoaderBuffer(provider),
                locale,
                R::COMPONENTS,
                $($arg.into()),+
            )
        }
    };
    (S: $skel:path | $compts:path, $compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident, $($arg:ident: $ty:ty),+) => {
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::$compiled_fn)]
        pub fn $any_fn<S, P>(
            provider: &P,
            locale: &DataLocale,
            $($arg: $ty),+
        ) -> Result<Self, LoadError>
        where
            S: ?Sized + $skel + $compts,
            P: AnyProvider + ?Sized,
        {
            Self::$internal_fn::<S, _, _>(
                &provider.as_downcasting(),
                &ExternalLoaderAny(provider),
                locale,
                $($arg),+
            )
        }
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::$compiled_fn)]
        #[cfg(feature = "serde")]
        pub fn $buffer_fn<S, P>(
            provider: &P,
            locale: &DataLocale,
            $($arg: $ty),+
        ) -> Result<Self, LoadError>
        where
            S: ?Sized + $skel + $compts,
            P: BufferProvider + ?Sized,
        {
            Self::$internal_fn::<S, _, _>(
                &provider.as_deserializing(),
                &ExternalLoaderBuffer(provider),
                locale,
                $($arg),+
            )
        }
    };
}

/// Options bag for datetime formatting.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct NeoOptions<R: DateTimeMarkers> {
    /// The desired length of the formatted string,
    /// if required for the chosen field set.
    ///
    /// See [`NeoSkeletonLength`].
    pub length: R::LengthOption,
    /// When to display the era field in the formatted string,
    /// if required for the chosen field set.
    ///
    /// See [`EraDisplay`](crate::neo_skeleton::EraDisplay).
    pub era_display: R::EraDisplayOption,
    /// How many fractional seconds to display,
    /// if seconds are included in the field set.
    ///
    /// See [`FractionalSecondDigits`](crate::neo_skeleton::FractionalSecondDigits).
    pub fractional_second_digits: R::FractionalSecondDigitsOption,
}

impl<R> From<NeoSkeletonLength> for NeoOptions<R>
where
    R: DateTimeMarkers,
    R::LengthOption: From<NeoSkeletonLength>,
    R::EraDisplayOption: Default,
    R::FractionalSecondDigitsOption: Default,
{
    #[inline]
    fn from(value: NeoSkeletonLength) -> Self {
        NeoOptions {
            length: value.into(),
            era_display: Default::default(),
            fractional_second_digits: Default::default(),
        }
    }
}

// Note: This is implemented manually because the auto-derive adds an extra
// bound `R: Default` which we don't need.
impl<R> Default for NeoOptions<R>
where
    R: DateTimeMarkers,
    R::LengthOption: Default,
    R::EraDisplayOption: Default,
    R::FractionalSecondDigitsOption: Default,
{
    #[inline]
    fn default() -> Self {
        NeoOptions {
            length: Default::default(),
            era_display: Default::default(),
            fractional_second_digits: Default::default(),
        }
    }
}

size_test!(TypedNeoFormatter<icu_calendar::Gregorian, crate::neo_marker::NeoYearMonthDayMarker>, typed_neo_year_month_day_formatter_size, 504);

/// [`TypedNeoFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at compile time.
///
/// For more details, please read the [crate root docs][crate].
#[doc = typed_neo_year_month_day_formatter_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoFormatter<C: CldrCalendar, R: DateTimeNamesMarker> {
    selection: DateTimeZonePatternSelectionData,
    names: RawDateTimeNames<R>,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar, R: DateTimeMarkers + HasConstComponents> TypedNeoFormatter<C, R>
where
    R::D: TypedDateDataMarkers<C>,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
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
    /// let formatter =
    ///     TypedNeoFormatter::<Gregorian, NeoYearMonthDayMarker>::try_new(
    ///         &locale!("es-MX").into(),
    ///         NeoSkeletonLength::Long.into(),
    ///     )
    ///     .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&Date::try_new_gregorian_date(2023, 12, 20).unwrap()),
    ///     "20 de diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale, options: NeoOptions<R>) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            R::COMPONENTS,
            options,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        R,
        try_new,
        try_new_with_any_provider,
        try_new_with_buffer_provider,
        try_new_internal,
        options: NeoOptions<R>
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        options: NeoOptions<R>,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>
            // FixedDecimalFormatter markers
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator markers
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            R::COMPONENTS,
            options,
        )
    }
}

impl<C: CldrCalendar, R: DateTimeMarkers + IsRuntimeComponents> TypedNeoFormatter<C, R>
where
    R::D: TypedDateDataMarkers<C>,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
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
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = TypedNeoFormatter::<Gregorian, _>::try_new_with_components(
    ///     &locale!("es-MX").into(),
    ///     NeoDateComponents::YearMonth,
    ///     NeoSkeletonLength::Medium.into(),
    /// )
    /// .unwrap();
    /// let dt = Date::try_new_gregorian_date(2024, 1, 10).unwrap();
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
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = TypedNeoFormatter::<Gregorian, _>::try_new_with_components(
    ///     &locale!("es-MX").into(),
    ///     NeoTimeComponents::Hour,
    ///     NeoSkeletonLength::Medium.into(),
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
    /// use icu::datetime::neo_skeleton::NeoDateTimeComponents;
    /// use icu::datetime::neo_skeleton::NeoDayComponents;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = TypedNeoFormatter::<Gregorian, _>::try_new_with_components(
    ///     &locale!("es-MX").into(),
    ///     NeoDateTimeComponents::DateTime(
    ///         NeoDayComponents::Weekday,
    ///         NeoTimeComponents::HourMinute,
    ///     ),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    /// let dt =
    ///     DateTime::try_new_gregorian_datetime(2024, 1, 10, 16, 20, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.format(&dt), "miércoles, 04:20 p.m.");
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_components(
        locale: &DataLocale,
        components: R,
        options: NeoOptions<R>,
    ) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            components.into(),
            options,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_components,
        try_new_with_components_with_any_provider,
        try_new_with_components_with_buffer_provider,
        try_new_internal,
        components: R,
        options: NeoOptions<R>
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_with_components_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        components: R,
        options: NeoOptions<R>,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>
            // FixedDecimalFormatter markers
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator markers
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            components.into(),
            options,
        )
    }
}

impl<C: CldrCalendar, R: DateTimeMarkers> TypedNeoFormatter<C, R>
where
    R::D: TypedDateDataMarkers<C>,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
{
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        components: NeoComponents,
        options: NeoOptions<R>,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader,
    {
        let hour_cycle = locale
            .get_unicode_ext(&icu_locale_core::extensions::unicode::key!("hc"))
            .as_ref()
            .and_then(CoarseHourCycle::from_locale_value);
        let selection = DateTimeZonePatternSelectionData::try_new_with_skeleton(
            &<R::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker::bind(provider),
            &<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker::bind(provider),
            &R::GluePatternV1Marker::bind(provider),
            locale,
            options.length.into(),
            components,
            options.era_display.into(),
            options.fractional_second_digits.into(),
            hour_cycle,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &<R::D as TypedDateDataMarkers<C>>::YearNamesV1Marker::bind(provider),
            &<R::D as TypedDateDataMarkers<C>>::MonthNamesV1Marker::bind(provider),
            &<R::D as TypedDateDataMarkers<C>>::WeekdayNamesV1Marker::bind(provider),
            &<R::T as TimeMarkers>::DayPeriodNamesV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::EssentialsV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::GenericLongV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::GenericShortV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::SpecificLongV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::SpecificShortV1Marker::bind(provider),
            Some(loader), // fixed decimal formatter
            Some(loader), // week calculator
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

impl<C: CldrCalendar, R: DateTimeMarkers> TypedNeoFormatter<C, R>
where
    R::D: DateInputMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
{
    /// Formats a datetime. Calendars and fields must match at compile time.
    ///
    /// # Examples
    ///
    /// Mismatched calendars will not compile:
    ///
    /// ```compile_fail
    /// use icu::calendar::Date;
    /// use icu::calendar::buddhist::Buddhist;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo_marker::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    ///
    /// let formatter =
    ///     TypedNeoFormatter::<Buddhist, NeoYearMonthDayMarker>::try_new(
    ///         &locale!("es-MX").into(),
    ///         NeoSkeletonLength::Long.into(),
    ///     )
    ///     .unwrap();
    ///
    /// // type mismatch resolving `<Gregorian as AsCalendar>::Calendar == Buddhist`
    /// formatter.format(&Date::try_new_gregorian_date(2023, 12, 20).unwrap());
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
    ///     TypedNeoFormatter::<Gregorian, NeoYearMonthDayMarker>::try_new(
    ///         &locale!("es-MX").into(),
    ///         NeoSkeletonLength::Long.into(),
    ///     )
    ///     .unwrap();
    ///
    /// // the trait `NeoGetField<AnyCalendarKind>`
    /// // is not implemented for `icu::icu_calendar::Time`
    /// formatter.format(&Time::try_new(0, 0, 0, 0).unwrap());
    /// ```
    pub fn format<I>(&self, datetime: &I) -> FormattedNeoDateTime
    where
        I: ?Sized + IsInCalendar<C> + AllInputMarkers<R>,
    {
        let datetime =
            ExtractedDateTimeInput::extract_from_neo_input::<R::D, R::T, R::Z, I>(datetime);
        FormattedNeoDateTime {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        }
    }
}

size_test!(
    NeoFormatter<crate::neo_marker::NeoYearMonthDayMarker>,
    neo_year_month_day_formatter_size,
    560
);

/// [`NeoFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at runtime.
///
/// For more details, please read the [crate root docs][crate].
#[doc = neo_year_month_day_formatter_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct NeoFormatter<R: DateTimeNamesMarker> {
    selection: DateTimeZonePatternSelectionData,
    names: RawDateTimeNames<R>,
    calendar: AnyCalendar,
}

impl<R: DateTimeMarkers + HasConstComponents> NeoFormatter<R>
where
    R::D: DateDataMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
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
    /// let formatter =
    ///     NeoFormatter::<NeoYearMonthDayMarker>::try_new(&locale.into(), length.into())
    ///         .unwrap();
    ///
    /// let datetime = DateTime::try_new_iso_datetime(2024, 5, 8, 0, 0, 0).unwrap();
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
    pub fn try_new(locale: &DataLocale, options: NeoOptions<R>) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            R::COMPONENTS,
            options,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        R,
        try_new,
        try_new_with_any_provider,
        try_new_with_buffer_provider,
        try_new_internal,
        options: NeoOptions<R>
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        options: NeoOptions<R>,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>
    // AnyCalendar constructor markers
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<IslamicObservationalCacheV1Marker>
            + DataProvider<IslamicUmmAlQuraCacheV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
    // FixedDecimalFormatter markers
            + DataProvider<DecimalSymbolsV1Marker>
    // WeekCalculator markers
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            R::COMPONENTS,
            options,
        )
    }
}

impl<R: DateTimeMarkers + IsRuntimeComponents> NeoFormatter<R>
where
    R::D: DateDataMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
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
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = NeoFormatter::try_new_with_components(
    ///     &locale!("es-MX").into(),
    ///     NeoDateComponents::YearMonth,
    ///     NeoSkeletonLength::Medium.into(),
    /// )
    /// .unwrap();
    /// let dt = Date::try_new_iso_date(2024, 1, 10).unwrap();
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
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = NeoFormatter::try_new_with_components(
    ///     &locale!("es-MX").into(),
    ///     NeoTimeComponents::Hour,
    ///     NeoSkeletonLength::Medium.into(),
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
    /// use icu::datetime::neo_skeleton::NeoDateTimeComponents;
    /// use icu::datetime::neo_skeleton::NeoDayComponents;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = NeoFormatter::try_new_with_components(
    ///     &locale!("es-MX").into(),
    ///     NeoDateTimeComponents::DateTime(
    ///         NeoDayComponents::Weekday,
    ///         NeoTimeComponents::HourMinute,
    ///     ),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    /// let dt = DateTime::try_new_iso_datetime(2024, 1, 10, 16, 20, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.convert_and_format(&dt),
    ///     "miércoles, 04:20 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_components(locale: &DataLocale, components: R, options: NeoOptions<R>) -> Result<Self, LoadError>
    where
    crate::provider::Baked: Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            components.into(),
            options,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_components,
        try_new_with_components_with_any_provider,
        try_new_with_components_with_buffer_provider,
        try_new_internal,
        components: R,
        options: NeoOptions<R>
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_with_components_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        components: R,
        options: NeoOptions<R>,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>
    // AnyCalendar constructor markers
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<IslamicObservationalCacheV1Marker>
            + DataProvider<IslamicUmmAlQuraCacheV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
    // FixedDecimalFormatter markers
            + DataProvider<DecimalSymbolsV1Marker>
    // WeekCalculator markers
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            components.into(),
            options,
        )
    }
}

impl<R: DateTimeMarkers> NeoFormatter<R>
where
    R::D: DateDataMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
{
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        components: NeoComponents,
        options: NeoOptions<R>,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateDataMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateDataMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::EssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::GenericShortV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificLongV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::SpecificShortV1Marker>
            + DataProvider<R::GluePatternV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader + AnyCalendarLoader,
    {
        let calendar = AnyCalendarLoader::load(loader, locale).map_err(LoadError::Data)?;
        let kind = calendar.kind();
        let hour_cycle = locale
            .get_unicode_ext(&icu_locale_core::extensions::unicode::key!("hc"))
            .as_ref()
            .and_then(CoarseHourCycle::from_locale_value);
        let selection = DateTimeZonePatternSelectionData::try_new_with_skeleton(
            &AnyCalendarProvider::<<R::D as DateDataMarkers>::Skel, _>::new(provider, kind),
            &<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker::bind(provider),
            &R::GluePatternV1Marker::bind(provider),
            locale,
            options.length.into(),
            components,
            options.era_display.into(),
            options.fractional_second_digits.into(),
            hour_cycle,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &AnyCalendarProvider::<<R::D as DateDataMarkers>::Year, _>::new(provider, kind),
            &AnyCalendarProvider::<<R::D as DateDataMarkers>::Month, _>::new(provider, kind),
            &<R::D as DateDataMarkers>::WeekdayNamesV1Marker::bind(provider),
            &<R::T as TimeMarkers>::DayPeriodNamesV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::EssentialsV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::ExemplarCitiesV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::GenericLongV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::GenericShortV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::SpecificLongV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::SpecificShortV1Marker::bind(provider),
            Some(loader), // fixed decimal formatter
            Some(loader), // week calculator
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

impl<R: DateTimeMarkers> NeoFormatter<R>
where
    R::D: DateInputMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
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
    /// let formatter = NeoFormatter::<NeoYearMonthDayMarker>::try_new(
    ///     &locale!("en-u-ca-hebrew").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    ///
    /// let date = Date::try_new_gregorian_date(2023, 12, 20).unwrap();
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
    /// let formatter = NeoFormatter::<NeoYearMonthDayMarker>::try_new(
    ///     &locale!("es-MX").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    ///
    /// // the trait `NeoGetField<AnyCalendarKind>`
    /// // is not implemented for `icu::icu_calendar::Time`
    /// formatter.strict_format(&Time::try_new(0, 0, 0, 0).unwrap());
    /// ```
    pub fn strict_format<I>(
        &self,
        datetime: &I,
    ) -> Result<FormattedNeoDateTime, crate::MismatchedCalendarError>
    where
        I: ?Sized + IsAnyCalendarKind + AllInputMarkers<R>,
    {
        if !datetime.is_any_calendar_kind(self.calendar.kind()) {
            return Err(crate::MismatchedCalendarError {
                this_kind: self.calendar.kind(),
                date_kind:
                    NeoGetField::<<R::D as DateInputMarkers>::AnyCalendarKindInput>::get_field(
                        datetime,
                    )
                    .into(),
            });
        }
        let datetime =
            ExtractedDateTimeInput::extract_from_neo_input::<R::D, R::T, R::Z, I>(datetime);
        Ok(FormattedNeoDateTime {
            pattern: self.selection.select(&datetime),
            datetime,
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
    /// let formatter = NeoFormatter::<NeoYearMonthDayMarker>::try_new(
    ///     &locale!("en-u-ca-hebrew").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    ///
    /// let date = Date::try_new_roc_date(113, 5, 8).unwrap();
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
    /// let formatter = NeoFormatter::<NeoYearMonthDayMarker>::try_new(
    ///     &locale!("es-MX").into(),
    ///     NeoSkeletonLength::Long.into(),
    /// )
    /// .unwrap();
    ///
    /// // the trait `NeoGetField<AnyCalendarKind>`
    /// // is not implemented for `icu::icu_calendar::Time`
    /// formatter.convert_and_format(&Time::try_new(0, 0, 0, 0).unwrap());
    /// ```
    pub fn convert_and_format<'a, I>(&'a self, datetime: &I) -> FormattedNeoDateTime
    where
        I: ?Sized + ConvertCalendar,
        I::Converted<'a>: Sized + AllInputMarkers<R>,
    {
        let datetime = datetime.to_calendar(&self.calendar);
        let datetime =
            ExtractedDateTimeInput::extract_from_neo_input::<R::D, R::T, R::Z, I::Converted<'a>>(
                &datetime,
            );
        FormattedNeoDateTime {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        }
    }
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct FormattedNeoDateTime<'a> {
    pattern: DateTimeZonePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a> TryWriteable for FormattedNeoDateTime<'a> {
    type Error = DateTimeWriteError;

    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        try_write_pattern_items(
            self.pattern.metadata(),
            self.pattern.iter_items(),
            self.pattern.formatting_options(),
            &self.datetime,
            Some(&self.names),
            Some(&self.names),
            Some(&self.names),
            self.names.week_calculator,
            self.names.fixed_decimal_formatter,
            sink,
        )
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'a> FormattedNeoDateTime<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}
