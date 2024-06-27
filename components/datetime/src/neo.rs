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
use crate::neo_marker::{
    ConvertCalendar, DateMarkers, DateTimeMarkers, IsAnyCalendarKind, IsRuntimeComponents,
    NeoFormatterMarker, NeoGetField, TimeMarkers, TypedDateMarkers, TypedDateTimeMarkers,
    TypedNeoFormatterMarker, ZoneMarkers,
};
use crate::neo_pattern::DateTimePattern;
use crate::neo_skeleton::{NeoComponents, NeoSkeletonLength};
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

#[doc(hidden)] // internal
pub mod _internal {
    pub use crate::calendar::CalMarkers;
    pub use crate::calendar::FullDataCalMarkers;
    pub use crate::calendar::NoDataCalMarkers;
}

use _internal::CalMarkers;

/// Helper macro for generating any/buffer constructors in this file.
macro_rules! gen_any_buffer_constructors_with_external_loader {
    ($compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident, $($arg:ident: $ty:path),+) => {
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
                $($arg),+
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
                $($arg),+
            )
        }
    };
    (R, $compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident, $($arg:ident: $ty:path),+) => {
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
                $($arg),+
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
                $($arg),+
            )
        }
    };
    (S: $skel:path | $compts:path, $compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident, $($arg:ident: $ty:path),+) => {
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

size_test!(TypedNeoFormatter<icu_calendar::Gregorian, crate::neo_marker::NeoYearMonthDayMarker>, typed_neo_year_month_day_formatter_size, 536);

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
    selection: DateTimePatternSelectionData,
    names: RawDateTimeNames<R>,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar, R: TypedNeoFormatterMarker<C>> TypedNeoFormatter<C, R> {
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
    ///         NeoSkeletonLength::Long,
    ///     )
    ///     .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&Date::try_new_gregorian_date(2023, 12, 20).unwrap()),
    ///     "20 de diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale, length: NeoSkeletonLength) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            R::COMPONENTS,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        R,
        try_new,
        try_new_with_any_provider,
        try_new_with_buffer_provider,
        try_new_internal,
        length: NeoSkeletonLength
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>
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
            length,
        )
    }
}

impl<C: CldrCalendar, R: TypedDateTimeMarkers<C> + IsRuntimeComponents> TypedNeoFormatter<C, R> {
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
    ///     NeoDateComponents::EraYearMonth,
    ///     NeoSkeletonLength::Medium,
    /// )
    /// .unwrap();
    /// let dt = Date::try_new_gregorian_date(2024, 1, 10).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.format(&dt), "ene 2024 d.C.");
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
    ///     NeoSkeletonLength::Medium,
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
    ///     NeoSkeletonLength::Long,
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
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            components,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_components,
        try_new_with_components_with_any_provider,
        try_new_with_components_with_buffer_provider,
        try_new_internal,
        components: R,
        length: NeoSkeletonLength
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_with_components_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        components: R,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>
            // FixedDecimalFormatter markers
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator markers
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            components,
            length,
        )
    }
}

impl<C: CldrCalendar, R: TypedDateTimeMarkers<C>> TypedNeoFormatter<C, R> {
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        components: impl Into<NeoComponents>,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting markers
            + DataProvider<<R::D as TypedDateMarkers<C>>::YearNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::MonthNamesV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::DateSkeletonPatternsV1Marker>
            + DataProvider<<R::D as TypedDateMarkers<C>>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader,
    {
        let selection = DateTimePatternSelectionData::try_new_with_skeleton(
            &<R::D as TypedDateMarkers<C>>::DateSkeletonPatternsV1Marker::bind(provider),
            &<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker::bind(provider),
            &R::DateTimePatternV1Marker::bind(provider),
            locale,
            length,
            components.into(),
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &<R::D as TypedDateMarkers<C>>::YearNamesV1Marker::bind(provider),
            &<R::D as TypedDateMarkers<C>>::MonthNamesV1Marker::bind(provider),
            &<R::D as TypedDateMarkers<C>>::WeekdayNamesV1Marker::bind(provider),
            &<R::T as TimeMarkers>::DayPeriodNamesV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker::bind(provider),
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
    ///         NeoSkeletonLength::Long,
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
    ///         NeoSkeletonLength::Long,
    ///     )
    ///     .unwrap();
    ///
    /// // the trait `NeoGetField<AnyCalendarKind>`
    /// // is not implemented for `icu::icu_calendar::Time`
    /// formatter.format(&Time::try_new(0, 0, 0, 0).unwrap());
    /// ```
    pub fn format<I>(&self, datetime: &I) -> FormattedNeoDateTime
    where
        I: ?Sized
            + NeoGetField<<R::D as TypedDateMarkers<C>>::TypedInputMarker>
            + NeoGetField<<R::D as TypedDateMarkers<C>>::YearInput>
            + NeoGetField<<R::D as TypedDateMarkers<C>>::MonthInput>
            + NeoGetField<<R::D as TypedDateMarkers<C>>::DayOfMonthInput>
            + NeoGetField<<R::D as TypedDateMarkers<C>>::DayOfWeekInput>
            + NeoGetField<<R::D as TypedDateMarkers<C>>::DayOfYearInput>
            + NeoGetField<<R::D as TypedDateMarkers<C>>::AnyCalendarKindInput>
            + NeoGetField<<R::T as TimeMarkers>::HourInput>
            + NeoGetField<<R::T as TimeMarkers>::MinuteInput>
            + NeoGetField<<R::T as TimeMarkers>::SecondInput>
            + NeoGetField<<R::T as TimeMarkers>::NanoSecondInput>
            + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneInput>,
    {
        let datetime = ExtractedDateTimeInput::extract_from_typed_neo_input::<C, R::D, R::T, R::Z, I>(
            datetime,
        );
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
    592
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
    selection: DateTimePatternSelectionData,
    names: RawDateTimeNames<R>,
    calendar: AnyCalendar,
}

impl<R: NeoFormatterMarker> NeoFormatter<R> {
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
    ///     NeoFormatter::<NeoYearMonthDayMarker>::try_new(&locale.into(), length)
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
    pub fn try_new(locale: &DataLocale, length: NeoSkeletonLength) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            R::COMPONENTS,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        R,
        try_new,
        try_new_with_any_provider,
        try_new_with_buffer_provider,
        try_new_internal,
        length: NeoSkeletonLength
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>
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
            length,
        )
    }
}

impl<R: DateTimeMarkers + IsRuntimeComponents> NeoFormatter<R> {
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
    ///     NeoDateComponents::EraYearMonth,
    ///     NeoSkeletonLength::Medium,
    /// )
    /// .unwrap();
    /// let dt = Date::try_new_iso_date(2024, 1, 10).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.convert_and_format(&dt), "ene 2024 d.C.");
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
    ///     NeoSkeletonLength::Medium,
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
    ///     NeoSkeletonLength::Long,
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
    pub fn try_new_with_components(locale: &DataLocale, components: R, length: NeoSkeletonLength) -> Result<Self, LoadError>
    where
    crate::provider::Baked: Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            components,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_components,
        try_new_with_components_with_any_provider,
        try_new_with_components_with_buffer_provider,
        try_new_internal,
        components: R,
        length: NeoSkeletonLength
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_with_components_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        components: R,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>
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
            components,
            length,
        )
    }
}

impl<R: DateTimeMarkers> NeoFormatter<R> {
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        components: impl Into<NeoComponents>,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
    // Date formatting markers
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<<R::D as DateMarkers>::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<<R::D as DateMarkers>::WeekdayNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::DayPeriodNamesV1Marker>
            + DataProvider<<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker>
            + DataProvider<<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader + AnyCalendarLoader,
    {
        let calendar = AnyCalendarLoader::load(loader, locale).map_err(LoadError::Data)?;
        let kind = calendar.kind();
        let selection = DateTimePatternSelectionData::try_new_with_skeleton(
            &AnyCalendarProvider::<<R::D as DateMarkers>::Skel, _>::new(provider, kind),
            &<R::T as TimeMarkers>::TimeSkeletonPatternsV1Marker::bind(provider),
            &R::DateTimePatternV1Marker::bind(provider),
            locale,
            length,
            components.into(),
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &AnyCalendarProvider::<<R::D as DateMarkers>::Year, _>::new(provider, kind),
            &AnyCalendarProvider::<<R::D as DateMarkers>::Month, _>::new(provider, kind),
            &<R::D as DateMarkers>::WeekdayNamesV1Marker::bind(provider),
            &<R::T as TimeMarkers>::DayPeriodNamesV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::ZoneEssentialsV1Marker::bind(provider),
            &<R::Z as ZoneMarkers>::ZoneGenericShortNamesV1Marker::bind(provider),
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
    ///     NeoSkeletonLength::Long,
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
    ///     NeoSkeletonLength::Long,
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
        I: ?Sized
            + IsAnyCalendarKind
            + NeoGetField<<R::D as DateMarkers>::YearInput>
            + NeoGetField<<R::D as DateMarkers>::MonthInput>
            + NeoGetField<<R::D as DateMarkers>::DayOfMonthInput>
            + NeoGetField<<R::D as DateMarkers>::DayOfWeekInput>
            + NeoGetField<<R::D as DateMarkers>::DayOfYearInput>
            + NeoGetField<<R::D as DateMarkers>::AnyCalendarKindInput>
            + NeoGetField<<R::T as TimeMarkers>::HourInput>
            + NeoGetField<<R::T as TimeMarkers>::MinuteInput>
            + NeoGetField<<R::T as TimeMarkers>::SecondInput>
            + NeoGetField<<R::T as TimeMarkers>::NanoSecondInput>
            + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneInput>,
    {
        if !datetime.is_any_calendar_kind(self.calendar.kind()) {
            return Err(crate::MismatchedCalendarError {
                this_kind: self.calendar.kind(),
                date_kind: NeoGetField::<<R::D as DateMarkers>::AnyCalendarKindInput>::get_field(
                    datetime,
                )
                .into(),
            });
        }
        let datetime =
            ExtractedDateTimeInput::extract_from_any_neo_input::<R::D, R::T, R::Z, I>(datetime);
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
    ///     NeoSkeletonLength::Long,
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
    ///     NeoSkeletonLength::Long,
    /// )
    /// .unwrap();
    ///
    /// // the trait `NeoGetField<AnyCalendarKind>`
    /// // is not implemented for `icu::icu_calendar::Time`
    /// formatter.convert_and_format(&Time::try_new(0, 0, 0, 0).unwrap());
    /// ```
    pub fn convert_and_format<'a, I>(&'a self, datetime: &I) -> FormattedNeoDateTime
    where
        I: ConvertCalendar,
        I::Converted<'a>: NeoGetField<<R::D as DateMarkers>::YearInput>
            + NeoGetField<<R::D as DateMarkers>::MonthInput>
            + NeoGetField<<R::D as DateMarkers>::DayOfMonthInput>
            + NeoGetField<<R::D as DateMarkers>::DayOfWeekInput>
            + NeoGetField<<R::D as DateMarkers>::DayOfYearInput>
            + NeoGetField<<R::D as DateMarkers>::AnyCalendarKindInput>
            + NeoGetField<<R::T as TimeMarkers>::HourInput>
            + NeoGetField<<R::T as TimeMarkers>::MinuteInput>
            + NeoGetField<<R::T as TimeMarkers>::SecondInput>
            + NeoGetField<<R::T as TimeMarkers>::NanoSecondInput>
            + NeoGetField<<R::Z as ZoneMarkers>::TimeZoneInput>,
    {
        let datetime = datetime.to_calendar(&self.calendar);
        let datetime = ExtractedDateTimeInput::extract_from_any_neo_input::<
            R::D,
            R::T,
            R::Z,
            I::Converted<'a>,
        >(&datetime);
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
    pattern: DateTimePatternDataBorrowed<'a>,
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
