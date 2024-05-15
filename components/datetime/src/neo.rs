// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use crate::calendar::AnyCalendarProvider;
use crate::external_loaders::*;
use crate::format::datetime::DateTimeWriteError;
use crate::format::datetime::{try_write_field, try_write_pattern};
use crate::format::neo::*;
use crate::input::ExtractedDateTimeInput;
use crate::input::{DateInput, DateTimeInput, IsoTimeInput};
use crate::neo_pattern::DateTimePattern;
use crate::neo_skeleton::{
    NeoComponents, NeoDateComponents, NeoDayComponents, NeoSkeletonCommonData,
    NeoSkeletonComponents, NeoSkeletonLength, TypedNeoSkeletonData,
};
use crate::options::length;
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
use icu_provider::{prelude::*, NeverMarker};
use writeable::TryWriteable;

#[doc(hidden)] // internal
pub mod _internal {
    pub use crate::calendar::CalMarkers;
    pub use crate::calendar::FullDataCalMarkers;
    pub use crate::calendar::NoDataCalMarkers;
}

use _internal::CalMarkers;
use _internal::FullDataCalMarkers as FullData;

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

size_test!(
    TypedNeoDateFormatter<icu_calendar::Gregorian>,
    typed_neo_date_formatter_size,
    456
);

/// [`TypedNeoDateFormatter`] can format dates from a calendar selected at compile time.
///
/// For the difference between this and [`DateFormatter`](crate::DateFormatter), please
/// read the [crate root docs][crate].
///
#[doc = typed_neo_date_formatter_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoDateFormatter<C: CldrCalendar> {
    selection: DatePatternSelectionData,
    names: RawDateTimeNames<DateMarker>,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar> TypedNeoDateFormatter<C> {
    /// Creates a [`TypedNeoDateFormatter`] for a date length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoDateFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = TypedNeoDateFormatter::<Gregorian>::try_new_with_length(
    ///     &locale!("es-MX").into(),
    ///     length::Date::Full,
    /// )
    /// .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&Date::try_new_gregorian_date(2023, 12, 20).unwrap()),
    ///     "miércoles, 20 de diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_length(locale: &DataLocale, length: length::Date) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Calendar-specific date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>,
    {
        Self::try_new_with_length_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_length,
        try_new_with_length_with_any_provider,
        try_new_with_length_with_buffer_provider,
        try_new_with_length_internal,
        length: length::Date
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_length)]
    pub fn try_new_with_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator keys
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_with_length_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            length,
        )
    }

    fn try_new_with_length_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader,
    {
        let selection = DatePatternSelectionData::try_new_with_length(
            &C::DatePatternV1Marker::bind(provider),
            locale,
            length,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &C::YearNamesV1Marker::bind(provider),  // year
            &C::MonthNamesV1Marker::bind(provider), // month
            &WeekdayNamesV1Marker::bind(provider),  // weekday
            &PhantomProvider,                       // day period
            Some(loader),                           // fixed decimal formatter
            Some(loader),                           // week calculator
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self {
            selection,
            names,
            _calendar: PhantomData,
        })
    }

    /// Creates a [`TypedNeoDateFormatter`] for a date skeleton.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoDateFormatter;
    /// use icu::datetime::neo_skeleton::{NeoSkeletonLength, YearMonthMarker};
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = TypedNeoDateFormatter::<Gregorian>::try_new_with_skeleton::<YearMonthMarker>(
    ///     &locale!("es-MX").into(),
    ///     NeoSkeletonLength::Long
    /// )
    /// .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&Date::try_new_gregorian_date(2023, 12, 20).unwrap()),
    ///     "diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_skeleton<S>(
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        S: ?Sized + TypedNeoSkeletonData<C> + NeoSkeletonComponents,
        crate::provider::Baked: Sized
            // Date formatting keys
            + DataProvider<S::DateSkeletonPatternsV1Marker>
            + DataProvider<S::YearNamesV1Marker>
            + DataProvider<S::MonthNamesV1Marker>
            + DataProvider<S::WeekdayNamesV1Marker>,
    {
        Self::try_new_with_skeleton_internal::<S, _, _>(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        S: TypedNeoSkeletonData<C> | NeoSkeletonComponents,
        try_new_with_skeleton,
        try_new_with_skeleton_with_any_provider,
        try_new_with_skeleton_with_buffer_provider,
        try_new_with_skeleton_internal,
        length: NeoSkeletonLength
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_length)]
    pub fn try_new_with_skeleton_unstable<S, P>(
        provider: &P,
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        S: ?Sized + TypedNeoSkeletonData<C> + NeoSkeletonComponents,
        P: ?Sized
            // Date formatting keys
            + DataProvider<S::DateSkeletonPatternsV1Marker>
            + DataProvider<S::YearNamesV1Marker>
            + DataProvider<S::MonthNamesV1Marker>
            + DataProvider<S::WeekdayNamesV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator keys
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_with_skeleton_internal::<S, _, _>(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            length,
        )
    }

    fn try_new_with_skeleton_internal<S, P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        S: ?Sized + TypedNeoSkeletonData<C> + NeoSkeletonComponents,
        P: ?Sized
            // Date formatting keys
            + DataProvider<S::DateSkeletonPatternsV1Marker>
            + DataProvider<S::YearNamesV1Marker>
            + DataProvider<S::MonthNamesV1Marker>
            + DataProvider<S::WeekdayNamesV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader,
    {
        let selection = DatePatternSelectionData::try_new_with_skeleton(
            &S::DateSkeletonPatternsV1Marker::bind(provider),
            locale,
            length,
            S::COMPONENTS,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &S::YearNamesV1Marker::bind(provider),    // year
            &S::MonthNamesV1Marker::bind(provider),   // month
            &S::WeekdayNamesV1Marker::bind(provider), // weekday
            &PhantomProvider,                         // day period
            Some(loader),                             // fixed decimal formatter
            Some(loader),                             // week calculator
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self {
            selection,
            names,
            _calendar: PhantomData,
        })
    }

    /// Formats a date.
    ///
    /// For an example, see [`TypedNeoDateFormatter`].
    pub fn format<T>(&self, date: &T) -> FormattedNeoDate
    where
        T: DateInput<Calendar = C>,
    {
        let datetime = ExtractedDateTimeInput::extract_from_date(date);
        FormattedNeoDate {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        }
    }
}

mod private {
    pub trait Sealed {}
}

/// A collection of types and constants for specific variants of [`TypedNeoFormatter`].
///
/// Individual fields can be [`NeverMarker`] if they are not needed for the specific variant.
pub trait TypedNeoFormatterMarker<C: CldrCalendar>: private::Sealed {
    /// Components in the neo skeleton.
    const COMPONENTS: NeoComponents;
    /// Fields for [`TypedDateTimeNames`].
    type DateTimeNamesMarker: DateTimeNamesMarker;
    /// Marker for loading year names.
    type YearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;
    /// Marker for loading month names.
    type MonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;
    /// Marker for loading date skeleton patterns.
    type DateSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading day period names.
    type DayPeriodNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading time skeleton patterns.
    type TimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading the date/time glue pattern.
    type DateTimePatternV1Marker: KeyedDataMarker<Yokeable = DateTimePatternV1<'static>>;
    // TODO: Add WeekCalculator and FixedDecimalFormatter optional bindings here
}

/// A collection of types and constants for specific variants of [`NeoFormatter`].
///
/// Individual fields can be [`NeverMarker`] if they are not needed for the specific variant.
///
/// The cross-calendar fields should be either [`FullDataCalMarkers`] or [`NoDataCalMarkers`].
///
/// [`FullDataCalMarkers`]: _internal::FullDataCalMarkers
/// [`NoDataCalMarkers`]: _internal::NoDataCalMarkers
pub trait NeoFormatterMarker {
    /// Components in the neo skeleton.
    const COMPONENTS: NeoComponents;
    /// Fields for [`TypedDateTimeNames`].
    type DateTimeNamesMarker: DateTimeNamesMarker;
    /// Cross-calendar data markers for year names.
    type Year: CalMarkers<YearNamesV1Marker>;
    /// Cross-calendar data markers for month names.
    type Month: CalMarkers<MonthNamesV1Marker>;
    /// Cross-calendar data markers for date skeleta.
    type Skel: CalMarkers<SkeletaV1Marker>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading day period names.
    type DayPeriodNamesV1Marker: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>;
    /// Marker for loading time skeleton patterns.
    type TimeSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;
    /// Marker for loading the date/time glue pattern.
    type DateTimePatternV1Marker: KeyedDataMarker<Yokeable = DateTimePatternV1<'static>>;
    // TODO: Add WeekCalculator, FixedDecimalFormatter, and AnyCalendar optional bindings here
}

/// Marker for a Year/Month/Day format, like "January 1, 2000"
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum NeoYearMonthDayMarker {}

impl private::Sealed for NeoYearMonthDayMarker {}

impl<C: CldrCalendar> TypedNeoFormatterMarker<C> for NeoYearMonthDayMarker {
    const COMPONENTS: NeoComponents =
        NeoComponents::Date(NeoDateComponents::Day(NeoDayComponents::YearMonthDay));
    type DateTimeNamesMarker = DateMarker;

    // Data to include
    type YearNamesV1Marker = C::YearNamesV1Marker;
    type MonthNamesV1Marker = C::MonthNamesV1Marker;
    type DateSkeletonPatternsV1Marker = C::SkeletaV1Marker;
    type WeekdayNamesV1Marker = WeekdayNamesV1Marker;

    // Data to exclude
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

impl NeoFormatterMarker for NeoYearMonthDayMarker {
    const COMPONENTS: NeoComponents =
        NeoComponents::Date(NeoDateComponents::Day(NeoDayComponents::YearMonthDay));
    type DateTimeNamesMarker = DateMarker;

    // Data to include
    type WeekdayNamesV1Marker = WeekdayNamesV1Marker;
    type Year = FullData;
    type Month = FullData;
    type Skel = FullData;

    // Data to exclude
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateTimePatternV1Marker = NeverMarker<DateTimePatternV1<'static>>;
}

size_test!(TypedNeoFormatter<icu_calendar::Gregorian, NeoYearMonthDayMarker>, typed_neo_year_month_day_formatter_size, 456);

/// [`TypedNeoFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at compile time.
///
/// For more details, please read the [crate root docs][crate].
///
#[doc = typed_neo_year_month_day_formatter_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoFormatter<C: CldrCalendar, R: TypedNeoFormatterMarker<C>> {
    selection: DatePatternSelectionData,
    names: RawDateTimeNames<R::DateTimeNamesMarker>,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar, R: TypedNeoFormatterMarker<C>> TypedNeoFormatter<C, R> {
    /// Creates a [`TypedNeoFormatter`] for a date skeleton.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoFormatter;
    /// use icu::datetime::neo::NeoYearMonthDayMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = TypedNeoFormatter::<Gregorian, NeoYearMonthDayMarker>::try_new(
    ///     &locale!("es-MX").into(),
    ///     NeoSkeletonLength::Long
    /// )
    /// .unwrap();
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
            // Date formatting keys
            + DataProvider<R::YearNamesV1Marker>
            + DataProvider<R::MonthNamesV1Marker>
            + DataProvider<R::DateSkeletonPatternsV1Marker>
            + DataProvider<R::WeekdayNamesV1Marker>
            + DataProvider<R::DayPeriodNamesV1Marker>
            + DataProvider<R::TimeSkeletonPatternsV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new,
        try_new_with_any_provider,
        try_new_with_buffer_provider,
        try_new_internal,
        length: NeoSkeletonLength
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_with_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<R::YearNamesV1Marker>
            + DataProvider<R::MonthNamesV1Marker>
            + DataProvider<R::DateSkeletonPatternsV1Marker>
            + DataProvider<R::WeekdayNamesV1Marker>
            + DataProvider<R::DayPeriodNamesV1Marker>
            + DataProvider<R::TimeSkeletonPatternsV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator keys
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_internal(provider, &ExternalLoaderUnstable(provider), locale, length)
    }

    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<R::YearNamesV1Marker>
            + DataProvider<R::MonthNamesV1Marker>
            + DataProvider<R::DateSkeletonPatternsV1Marker>
            + DataProvider<R::WeekdayNamesV1Marker>
            + DataProvider<R::DayPeriodNamesV1Marker>
            + DataProvider<R::TimeSkeletonPatternsV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader,
    {
        let selection = DatePatternSelectionData::try_new_with_skeleton(
            &R::DateSkeletonPatternsV1Marker::bind(provider),
            locale,
            length,
            R::COMPONENTS,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &R::YearNamesV1Marker::bind(provider),      // year
            &R::MonthNamesV1Marker::bind(provider),     // month
            &R::WeekdayNamesV1Marker::bind(provider),   // weekday
            &R::DayPeriodNamesV1Marker::bind(provider), // day period
            Some(loader),                               // fixed decimal formatter
            Some(loader),                               // week calculator
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self {
            selection,
            names,
            _calendar: PhantomData,
        })
    }

    /// Formats a date.
    ///
    /// For an example, see [`TypedNeoDateFormatter`].
    pub fn format<T>(&self, date: &T) -> FormattedNeoDate
    where
        T: DateInput<Calendar = C>,
    {
        let datetime = ExtractedDateTimeInput::extract_from_date(date);
        FormattedNeoDate {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        }
    }
}

size_test!(
    NeoFormatter<NeoYearMonthDayMarker>,
    neo_year_month_day_formatter_size,
    512
);

/// [`NeoFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at runtime.
///
/// For more details, please read the [crate root docs][crate].
///
#[doc = neo_year_month_day_formatter_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct NeoFormatter<R: NeoFormatterMarker> {
    selection: DatePatternSelectionData,
    names: RawDateTimeNames<R::DateTimeNamesMarker>,
    calendar: AnyCalendar,
}

impl<R: NeoFormatterMarker> NeoFormatter<R> {
    /// Construct a new [`NeoFormatter`] from compiled data.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{any_calendar::AnyCalendar, Date};
    /// use icu::datetime::neo::NeoFormatter;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::neo::NeoYearMonthDayMarker;
    /// use icu::locid::locale;
    /// use std::str::FromStr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let length = NeoSkeletonLength::Medium;
    /// let locale = locale!("en-u-ca-hebrew");
    ///
    /// let df = NeoFormatter::<NeoYearMonthDayMarker>::try_new(&locale.into(), length)
    ///     .expect("Failed to create TypedDateFormatter instance.");
    ///
    /// let datetime =
    ///     Date::try_new_iso_date(2024, 5, 8).expect("Failed to construct Date.");
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_try_writeable_eq!(
    ///     df.format(&any_datetime).expect("Calendars should match"),
    ///     "Nisan 30, 5784"
    /// );
    /// ```
    ///
    /// [`AnyCalendarKind`]: icu_calendar::AnyCalendarKind
    #[inline(never)]
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale, length: NeoSkeletonLength) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Date formatting keys
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<R::WeekdayNamesV1Marker>
            + DataProvider<R::DayPeriodNamesV1Marker>
            + DataProvider<R::TimeSkeletonPatternsV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
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
            // Date formatting keys
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<R::WeekdayNamesV1Marker>
            + DataProvider<R::DayPeriodNamesV1Marker>
            + DataProvider<R::TimeSkeletonPatternsV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>
            // AnyCalendar constructor keys
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<IslamicObservationalCacheV1Marker>
            + DataProvider<IslamicUmmAlQuraCacheV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator keys
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_internal(provider, &ExternalLoaderUnstable(provider), locale, length)
    }

    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<R::Year as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<R::Month as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Buddhist>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Chinese>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Coptic>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Dangi>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Ethiopian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Gregorian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Hebrew>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Indian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicCivil>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicObservational>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicTabular>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Japanese>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::JapaneseExtended>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Persian>
            + DataProvider<<R::Skel as CalMarkers<SkeletaV1Marker>>::Roc>
            + DataProvider<R::WeekdayNamesV1Marker>
            + DataProvider<R::DayPeriodNamesV1Marker>
            + DataProvider<R::TimeSkeletonPatternsV1Marker>
            + DataProvider<R::DateTimePatternV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader + AnyCalendarLoader,
    {
        let calendar = AnyCalendarLoader::load(loader, locale).map_err(LoadError::Data)?;
        let kind = calendar.kind();
        let selection = DatePatternSelectionData::try_new_with_skeleton(
            &AnyCalendarProvider::<R::Skel, _>::new(provider, kind),
            locale,
            length,
            R::COMPONENTS,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &AnyCalendarProvider::<R::Year, _>::new(provider, kind), // year
            &AnyCalendarProvider::<R::Month, _>::new(provider, kind), // month
            &R::WeekdayNamesV1Marker::bind(provider),                // weekday
            &R::DayPeriodNamesV1Marker::bind(provider),              // day period
            Some(loader),                                            // fixed decimal formatter
            Some(loader),                                            // week calculator
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self {
            selection,
            names,
            calendar,
        })
    }

    /// Formats a date.
    ///
    /// If the date is in neither ISO-8601 nor the same calendar system as the formatter,
    /// an error is returned.
    ///
    /// For an example, see [`NeoDateFormatter`].
    pub fn format<T>(&self, date: &T) -> Result<FormattedNeoDate, crate::MismatchedCalendarError>
    where
        T: DateInput<Calendar = AnyCalendar>,
    {
        let datetime =
            if let Some(converted) = crate::calendar::convert_if_necessary(&self.calendar, date)? {
                ExtractedDateTimeInput::extract_from_date(&converted)
            } else {
                ExtractedDateTimeInput::extract_from_date(date)
            };
        Ok(FormattedNeoDate {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        })
    }
}

size_test!(NeoDateFormatter, neo_date_formatter_size, 512);

/// [`NeoDateFormatter`] is a formatter capable of formatting dates from any calendar, selected
/// at runtime. For the difference between this and [`TypedNeoDateFormatter`], please read the
/// [crate root docs][crate].
///
#[doc = neo_date_formatter_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct NeoDateFormatter {
    selection: DatePatternSelectionData,
    names: RawDateTimeNames<DateMarker>,
    calendar: AnyCalendar,
}

impl NeoDateFormatter {
    /// Construct a new [`NeoDateFormatter`] from compiled data.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{any_calendar::AnyCalendar, Date};
    /// use icu::datetime::{options::length, neo::NeoDateFormatter};
    /// use icu::locid::locale;
    /// use icu_provider::any::DynamicDataProviderAnyMarkerWrap;
    /// use std::str::FromStr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let length = length::Date::Medium;
    /// let locale = locale!("en-u-ca-gregory");
    ///
    /// let df = NeoDateFormatter::try_new_with_length(&locale.into(), length)
    ///     .expect("Failed to create TypedDateFormatter instance.");
    ///
    /// let datetime =
    ///     Date::try_new_iso_date(2020, 9, 1).expect("Failed to construct Date.");
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_try_writeable_eq!(
    ///     df.format(&any_datetime).expect("Calendars should match"),
    ///     "Sep 1, 2020"
    /// );
    /// ```
    ///
    /// [`AnyCalendarKind`]: icu_calendar::AnyCalendarKind
    #[inline(never)]
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_length(
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError> {
        Self::try_new_with_length_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_length,
        try_new_with_length_with_any_provider,
        try_new_with_length_with_buffer_provider,
        try_new_with_length_internal,
        length: length::Date
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_length)]
    pub fn try_new_with_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Roc>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>
            // AnyCalendar constructor keys
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<IslamicObservationalCacheV1Marker>
            + DataProvider<IslamicUmmAlQuraCacheV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator keys
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_with_length_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            length,
        )
    }

    fn try_new_with_length_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Roc>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader + AnyCalendarLoader,
    {
        let calendar = AnyCalendarLoader::load(loader, locale).map_err(LoadError::Data)?;
        let kind = calendar.kind();
        let selection = DatePatternSelectionData::try_new_with_length(
            &AnyCalendarProvider::<FullData, _>::new(provider, kind),
            locale,
            length,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &AnyCalendarProvider::<FullData, _>::new(provider, kind), // year
            &AnyCalendarProvider::<FullData, _>::new(provider, kind), // month
            &WeekdayNamesV1Marker::bind(provider),                    // weekday
            &PhantomProvider,                                         // day period
            Some(loader),                                             // fixed decimal formatter
            Some(loader),                                             // week calculator
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self {
            selection,
            names,
            calendar,
        })
    }

    /// Formats a date.
    ///
    /// If the date is in neither ISO-8601 nor the same calendar system as the formatter,
    /// an error is returned.
    ///
    /// For an example, see [`NeoDateFormatter`].
    pub fn format<T>(&self, date: &T) -> Result<FormattedNeoDate, crate::MismatchedCalendarError>
    where
        T: DateInput<Calendar = AnyCalendar>,
    {
        let datetime =
            if let Some(converted) = crate::calendar::convert_if_necessary(&self.calendar, date)? {
                ExtractedDateTimeInput::extract_from_date(&converted)
            } else {
                ExtractedDateTimeInput::extract_from_date(date)
            };
        Ok(FormattedNeoDate {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        })
    }
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct FormattedNeoDate<'a> {
    pattern: DatePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a> TryWriteable for FormattedNeoDate<'a> {
    type Error = DateTimeWriteError;

    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        try_write_pattern(
            self.pattern.as_borrowed(),
            &self.datetime,
            Some(&self.names),
            Some(&self.names),
            self.names.week_calculator,
            self.names.fixed_decimal_formatter,
            sink,
        )
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'a> FormattedNeoDate<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}

size_test!(NeoTimeFormatter, neo_time_formatter_size, 312);

/// [`NeoTimeFormatter`] can format times of day.
/// It supports both 12-hour and 24-hour formats.
///
#[doc = neo_time_formatter_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct NeoTimeFormatter {
    selection: TimePatternSelectionData,
    names: RawDateTimeNames<TimeMarker>,
}

impl NeoTimeFormatter {
    /// Creates a [`NeoTimeFormatter`] for a time length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Time;
    /// use icu::datetime::neo::NeoTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = NeoTimeFormatter::try_new_with_length(
    ///     &locale!("es-MX").into(),
    ///     length::Time::Medium,
    /// )
    /// .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&Time::try_new(14, 48, 58, 0).unwrap()),
    ///     "2:48:58 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_length(
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, LoadError> {
        Self::try_new_with_length_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_length,
        try_new_with_length_with_any_provider,
        try_new_with_length_with_buffer_provider,
        try_new_with_length_internal,
        length: length::Time
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_length)]
    pub fn try_new_with_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>,
    {
        Self::try_new_with_length_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            length,
        )
    }

    fn try_new_with_length_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>,
        L: FixedDecimalFormatterLoader,
    {
        let selection = TimePatternSelectionData::try_new_with_length(provider, locale, length)
            .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &PhantomProvider,                        // year
            &PhantomProvider,                        // month
            &PhantomProvider,                        // weekday
            &DayPeriodNamesV1Marker::bind(provider), // day period
            Some(loader),                            // fixed decimal formatter
            None::<&PhantomLoader>,                  // week calculator
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self { selection, names })
    }

    /// Creates a [`NeoTimeFormatter`] for a time skeleton.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Time;
    /// use icu::datetime::neo::NeoTimeFormatter;
    /// use icu::datetime::neo_skeleton::{NeoSkeletonLength, HourMinuteMarker};
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = NeoTimeFormatter::try_new_with_skeleton::<HourMinuteMarker>(
    ///     &locale!("es-MX").into(),
    ///     NeoSkeletonLength::Medium
    /// )
    /// .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&Time::try_new(14, 48, 58, 0).unwrap()),
    ///     "2:48 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_skeleton<S>(
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        S: ?Sized + NeoSkeletonCommonData + NeoSkeletonComponents,
        crate::provider::Baked: Sized
            // Time formatting keys
            + DataProvider<S::TimeSkeletonPatternsV1Marker>
            + DataProvider<S::DayPeriodNamesV1Marker>,
    {
        Self::try_new_with_skeleton_internal::<S, _, _>(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        S: NeoSkeletonCommonData | NeoSkeletonComponents,
        try_new_with_skeleton,
        try_new_with_skeleton_with_any_provider,
        try_new_with_skeleton_with_buffer_provider,
        try_new_with_skeleton_internal,
        length: NeoSkeletonLength
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_length)]
    pub fn try_new_with_skeleton_unstable<S, P>(
        provider: &P,
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        S: ?Sized + NeoSkeletonCommonData + NeoSkeletonComponents,
        P: ?Sized
            // Time formatting keys
            + DataProvider<S::TimeSkeletonPatternsV1Marker>
            + DataProvider<S::DayPeriodNamesV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>,
    {
        Self::try_new_with_skeleton_internal::<S, _, _>(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            length,
        )
    }

    fn try_new_with_skeleton_internal<S, P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: NeoSkeletonLength,
    ) -> Result<Self, LoadError>
    where
        S: ?Sized + NeoSkeletonCommonData + NeoSkeletonComponents,
        P: ?Sized
            // Date formatting keys
            + DataProvider<S::TimeSkeletonPatternsV1Marker>
            + DataProvider<S::DayPeriodNamesV1Marker>,
        L: FixedDecimalFormatterLoader,
    {
        let selection =
            TimePatternSelectionData::try_new_with_skeleton::<S::TimeSkeletonPatternsV1Marker>(
                provider,
                locale,
                length,
                <S as NeoSkeletonComponents>::COMPONENTS,
            )
            .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &PhantomProvider,                           // year
            &PhantomProvider,                           // month
            &PhantomProvider,                           // weekday
            &S::DayPeriodNamesV1Marker::bind(provider), // day period
            Some(loader),                               // fixed decimal formatter
            None::<&PhantomLoader>,                     // week calculator
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self { selection, names })
    }

    /// Formats a time of day.
    ///
    /// For an example, see [`NeoTimeFormatter`].
    pub fn format<T>(&self, time: &T) -> FormattedNeoTime
    where
        T: IsoTimeInput,
    {
        let datetime = ExtractedDateTimeInput::extract_from_time(time);
        FormattedNeoTime {
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
pub struct FormattedNeoTime<'a> {
    pattern: TimePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a> TryWriteable for FormattedNeoTime<'a> {
    type Error = DateTimeWriteError;

    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        try_write_pattern(
            self.pattern.as_borrowed(),
            &self.datetime,
            Some(&self.names),
            Some(&self.names),
            self.names.week_calculator,
            self.names.fixed_decimal_formatter,
            sink,
        )
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'a> FormattedNeoTime<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}

size_test!(
    TypedNeoDateTimeFormatter<icu_calendar::Gregorian>,
    typed_neo_date_time_formatter_size,
    576
);

/// [`TypedNeoDateTimeFormatter`] can format dates with times of day. The dates must be in
/// a calendar system determined at compile time.
///
/// For the difference between this and [`DateTimeFormatter`](crate::DateTimeFormatter), please
/// read the [crate root docs][crate].
///
#[doc = typed_neo_date_time_formatter_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoDateTimeFormatter<C: CldrCalendar> {
    selection: DateTimePatternSelectionData,
    names: RawDateTimeNames<DateTimeMarker>,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar> TypedNeoDateTimeFormatter<C> {
    /// Creates a [`TypedNeoDateTimeFormatter`] for a date length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter =
    ///     TypedNeoDateTimeFormatter::<Gregorian>::try_new_with_date_length(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///     )
    ///     .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_gregorian_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///     ),
    ///     "miércoles, 20 de diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_date_length(
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Calendar-specific date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>,
    {
        Self::try_new_with_date_length_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_date_length,
        try_new_with_date_length_with_any_provider,
        try_new_with_date_length_with_buffer_provider,
        try_new_with_date_length_internal,
        length: length::Date
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_date_length)]
    pub fn try_new_with_date_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator keys
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_with_date_length_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            length,
        )
    }

    fn try_new_with_date_length_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader,
    {
        let date_formatter = TypedNeoDateFormatter::<C>::try_new_with_length_internal(
            provider, loader, locale, length,
        )?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Date(date_formatter.selection),
            names: date_formatter.names.into(),
            _calendar: PhantomData,
        })
    }

    /// Creates a [`TypedNeoDateTimeFormatter`] for a time length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter =
    ///     TypedNeoDateTimeFormatter::<Gregorian>::try_new_with_time_length(
    ///         &locale!("es-MX").into(),
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_gregorian_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///     ),
    ///     "2:48:58 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_time_length(
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, LoadError> {
        Self::try_new_with_time_length_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_time_length,
        try_new_with_time_length_with_any_provider,
        try_new_with_time_length_with_buffer_provider,
        try_new_with_time_length_internal,
        length: length::Time
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_time_length)]
    pub fn try_new_with_time_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>,
    {
        Self::try_new_with_time_length_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            length,
        )
    }

    fn try_new_with_time_length_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>,
        L: FixedDecimalFormatterLoader,
    {
        let time_formatter =
            NeoTimeFormatter::try_new_with_length_internal(provider, loader, locale, length)?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Time(time_formatter.selection),
            names: time_formatter.names.into(),
            _calendar: PhantomData,
        })
    }

    /// Creates a [`TypedNeoDateTimeFormatter`] for date and time lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter =
    ///     TypedNeoDateTimeFormatter::<Gregorian>::try_new_with_lengths(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_gregorian_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///     ),
    ///     "miércoles, 20 de diciembre de 2023, 2:48:58 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_lengths(
        locale: &DataLocale,
        date_length: length::Date,
        time_length: length::Time,
    ) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Calendar-specific date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>,
    {
        Self::try_new_with_lengths_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            date_length,
            time_length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_lengths,
        try_new_with_lengths_with_any_provider,
        try_new_with_lengths_with_buffer_provider,
        try_new_with_lengths_internal,
        date_length: length::Date,
        time_length: length::Time
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_lengths)]
    pub fn try_new_with_lengths_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        date_length: length::Date,
        time_length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            // DateTime glue key
            + DataProvider<DateTimePatternV1Marker>
            // FixedDecimalFormatter key
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator key
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_with_lengths_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            date_length,
            time_length,
        )
    }

    fn try_new_with_lengths_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        date_length: length::Date,
        time_length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            // DateTime glue key
            + DataProvider<DateTimePatternV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader,
    {
        let selection = DateTimeGluePatternSelectionData::try_new_with_lengths(
            &C::DatePatternV1Marker::bind(provider),
            provider,
            locale,
            date_length,
            time_length,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &C::YearNamesV1Marker::bind(provider),   // year
            &C::MonthNamesV1Marker::bind(provider),  // month
            &WeekdayNamesV1Marker::bind(provider),   // weekday
            &DayPeriodNamesV1Marker::bind(provider), // day period
            Some(loader),                            // fixed decimal formatter
            Some(loader),                            // week calculator
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self {
            selection: DateTimePatternSelectionData::DateTimeGlue(selection),
            names,
            _calendar: PhantomData,
        })
    }

    /// Creates a [`TypedNeoDateTimeFormatter`] from [`DateTimeFormatterOptions`].
    ///
    /// Experimental because [`DateTimeFormatterOptions`] might go away or be changed in neo.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
    /// of the icu meta-crate. Use with caution.
    /// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
    ///
    /// [`DateTimeFormatterOptions`]: crate::DateTimeFormatterOptions
    /// </div>
    #[cfg(all(feature = "compiled_data", feature = "experimental"))]
    pub fn try_new(
        locale: &DataLocale,
        options: crate::DateTimeFormatterOptions,
    ) -> Result<Self, LoadError>
    where
        crate::provider::Baked: Sized
            // Calendar-specific date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>,
    {
        use crate::DateTimeFormatterOptions;
        match options {
            DateTimeFormatterOptions::Length(length::Bag {
                date: Some(date),
                time: Some(time),
            }) => Self::try_new_with_lengths(locale, date, time),
            DateTimeFormatterOptions::Length(length::Bag {
                date: Some(date),
                time: None,
            }) => Self::try_new_with_date_length(locale, date),
            DateTimeFormatterOptions::Length(length::Bag {
                date: None,
                time: Some(time),
            }) => Self::try_new_with_time_length(locale, time),
            _ => todo!(), // Err(LoadError::UnsupportedOptions),
        }
    }

    /// Formats a date and time of day.
    ///
    /// For an example, see [`TypedNeoDateTimeFormatter`].
    pub fn format<T>(&self, datetime: &T) -> FormattedNeoDateTime
    where
        T: DateTimeInput<Calendar = C>,
    {
        let datetime = ExtractedDateTimeInput::extract_from(datetime);
        FormattedNeoDateTime {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        }
    }
}

size_test!(NeoDateTimeFormatter, neo_date_time_formatter_size, 632);

/// [`NeoDateTimeFormatter`] is a formatter capable of formatting dates from any calendar, selected
/// at runtime. For the difference between this and [`TypedNeoDateFormatter`], please read the
/// [crate root docs][crate].
///
#[doc = neo_date_time_formatter_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct NeoDateTimeFormatter {
    selection: DateTimePatternSelectionData,
    names: RawDateTimeNames<DateTimeMarker>,
    calendar: AnyCalendar,
}

impl NeoDateTimeFormatter {
    /// Constructs a [`NeoDateTimeFormatter`] for a date length from compiled data.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::neo::NeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter =
    ///     NeoDateTimeFormatter::try_new_with_date_length(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///     )
    ///     .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_iso_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///             .to_any()
    ///     )
    ///     .unwrap(),
    ///     "miércoles, 20 de diciembre de 2023"
    /// );
    /// ```
    ///
    /// [`AnyCalendarKind`]: icu_calendar::AnyCalendarKind
    #[inline(never)]
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_date_length(
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError> {
        Self::try_new_with_date_length_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_date_length,
        try_new_with_date_length_with_any_provider,
        try_new_with_date_length_with_buffer_provider,
        try_new_with_date_length_internal,
        length: length::Date
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_date_length)]
    pub fn try_new_with_date_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Roc>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>
            // AnyCalendar constructor keys
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<IslamicObservationalCacheV1Marker>
            + DataProvider<IslamicUmmAlQuraCacheV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator keys
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_with_date_length_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            length,
        )
    }

    fn try_new_with_date_length_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Roc>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader + AnyCalendarLoader,
    {
        let date_formatter =
            NeoDateFormatter::try_new_with_length_internal(provider, loader, locale, length)?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Date(date_formatter.selection),
            names: date_formatter.names.into(),
            calendar: date_formatter.calendar,
        })
    }

    /// Constructs a [`NeoDateTimeFormatter`] for a time length from compiled data.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::neo::NeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter =
    ///     NeoDateTimeFormatter::try_new_with_time_length(
    ///         &locale!("es-MX").into(),
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_iso_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///             .to_any()
    ///     )
    ///     .unwrap(),
    ///     "2:48:58 p.m."
    /// );
    /// ```
    ///
    /// [`AnyCalendarKind`]: icu_calendar::AnyCalendarKind
    #[inline(never)]
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_time_length(
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, LoadError> {
        Self::try_new_with_time_length_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_time_length,
        try_new_with_time_length_with_any_provider,
        try_new_with_time_length_with_buffer_provider,
        try_new_with_time_length_internal,
        length: length::Time
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_time_length)]
    pub fn try_new_with_time_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            // AnyCalendar constructor keys
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<IslamicObservationalCacheV1Marker>
            + DataProvider<IslamicUmmAlQuraCacheV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            // FixedDecimalFormatter keys
            + DataProvider<DecimalSymbolsV1Marker>,
    {
        Self::try_new_with_time_length_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            length,
        )
    }

    fn try_new_with_time_length_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>,
        L: FixedDecimalFormatterLoader + AnyCalendarLoader,
    {
        // Need to compute the calendar ourselves since it is not in NeoTimeFormatter
        let calendar = AnyCalendarLoader::load(loader, locale).map_err(LoadError::Data)?;
        let time_formatter =
            NeoTimeFormatter::try_new_with_length_internal(provider, loader, locale, length)?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Time(time_formatter.selection),
            names: time_formatter.names.into(),
            calendar,
        })
    }

    /// Creates a [`NeoDateTimeFormatter`] for date and time lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::neo::NeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter =
    ///     NeoDateTimeFormatter::try_new_with_lengths(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_iso_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///             .to_any()
    ///     )
    ///     .unwrap(),
    ///     "miércoles, 20 de diciembre de 2023, 2:48:58 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_lengths(
        locale: &DataLocale,
        date_length: length::Date,
        time_length: length::Time,
    ) -> Result<Self, LoadError> {
        Self::try_new_with_lengths_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            date_length,
            time_length,
        )
    }

    gen_any_buffer_constructors_with_external_loader!(
        try_new_with_lengths,
        try_new_with_lengths_with_any_provider,
        try_new_with_lengths_with_buffer_provider,
        try_new_with_lengths_internal,
        date_length: length::Date,
        time_length: length::Time
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_lengths)]
    pub fn try_new_with_lengths_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        date_length: length::Date,
        time_length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Roc>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            // DateTime glue key
            + DataProvider<DateTimePatternV1Marker>
            // AnyCalendar constructor keys
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<IslamicObservationalCacheV1Marker>
            + DataProvider<IslamicUmmAlQuraCacheV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            // FixedDecimalFormatter key
            + DataProvider<DecimalSymbolsV1Marker>
            // WeekCalculator key
            + DataProvider<WeekDataV2Marker>,
    {
        Self::try_new_with_lengths_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            date_length,
            time_length,
        )
    }

    fn try_new_with_lengths_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        date_length: length::Date,
        time_length: length::Time,
    ) -> Result<Self, LoadError>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<YearNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<MonthNamesV1Marker>>::Roc>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Buddhist>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Chinese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Coptic>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Dangi>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Ethiopian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::EthiopianAmeteAlem>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Gregorian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Hebrew>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Indian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicCivil>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicObservational>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicTabular>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::IslamicUmmAlQura>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Japanese>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::JapaneseExtended>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Persian>
            + DataProvider<<FullData as CalMarkers<DatePatternV1Marker>>::Roc>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            // DateTime glue key
            + DataProvider<DateTimePatternV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader + AnyCalendarLoader,
    {
        let calendar = AnyCalendarLoader::load(loader, locale).map_err(LoadError::Data)?;
        let kind = calendar.kind();
        let selection = DateTimeGluePatternSelectionData::try_new_with_lengths(
            &AnyCalendarProvider::<FullData, _>::new(provider, kind),
            provider,
            locale,
            date_length,
            time_length,
        )
        .map_err(LoadError::Data)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern(
            &AnyCalendarProvider::<FullData, _>::new(provider, kind), // year
            &AnyCalendarProvider::<FullData, _>::new(provider, kind), // month
            &WeekdayNamesV1Marker::bind(provider),                    // weekday
            &DayPeriodNamesV1Marker::bind(provider),                  // day period
            Some(loader),                                             // fixed decimal formatter
            Some(loader),                                             // week calculator
            locale,
            selection.pattern_items_for_data_loading(),
        )?;
        Ok(Self {
            selection: DateTimePatternSelectionData::DateTimeGlue(selection),
            names,
            calendar,
        })
    }

    /// Formats a date and time of day.
    ///
    /// If the date is in neither ISO-8601 nor the same calendar system as the formatter,
    /// an error is returned.
    ///
    /// For an example, see [`NeoDateTimeFormatter`].
    pub fn format<T>(
        &self,
        datetime: &T,
    ) -> Result<FormattedNeoDateTime, crate::MismatchedCalendarError>
    where
        T: DateTimeInput<Calendar = AnyCalendar>,
    {
        let datetime = if let Some(converted) =
            crate::calendar::convert_datetime_if_necessary(&self.calendar, datetime)?
        {
            ExtractedDateTimeInput::extract_from(&converted)
        } else {
            ExtractedDateTimeInput::extract_from(datetime)
        };
        Ok(FormattedNeoDateTime {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        })
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
        {
            let mut r = Ok(());
            let mut iter = self.pattern.iter_items().peekable();
            let metadata = self.pattern.metadata();
            while let Some(item) = iter.next() {
                match item {
                    crate::pattern::PatternItem::Literal(ch) => sink.write_char(ch)?,
                    crate::pattern::PatternItem::Field(field) => {
                        r = r.and(try_write_field(
                            field,
                            &mut iter,
                            metadata,
                            &self.datetime,
                            Some(&self.names),
                            Some(&self.names),
                            self.names.week_calculator,
                            self.names.fixed_decimal_formatter,
                            sink,
                        )?)
                    }
                }
            }
            Ok(r)
        }
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'a> FormattedNeoDateTime<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}
