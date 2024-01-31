// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use crate::calendar::AnyCalendarProvider;
use crate::external_loaders::*;
use crate::format::neo::*;
use crate::input::ExtractedDateTimeInput;
use crate::input::{DateInput, DateTimeInput, IsoTimeInput};
use crate::neo_pattern::DateTimePattern;
use crate::options::length;
use crate::provider::neo::*;
use crate::raw::neo::*;
use crate::CldrCalendar;
use crate::Error;
use core::fmt;
use core::marker::PhantomData;
use icu_calendar::provider::{
    ChineseCacheV1Marker, DangiCacheV1Marker, JapaneseErasV1Marker, JapaneseExtendedErasV1Marker,
    WeekDataV2Marker,
};
use icu_calendar::AnyCalendar;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_provider::prelude::*;
use writeable::Writeable;

/// Helper macro for generating any/buffer constructors in this file.
macro_rules! gen_any_buffer_constructors_with_external_loader {
    ($compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident, $($arg:ident: $ty:path),+) => {
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::$compiled_fn)]
        pub fn $any_fn<P>(
            provider: &P,
            locale: &DataLocale,
            $($arg: $ty),+
        ) -> Result<Self, Error>
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
        ) -> Result<Self, Error>
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
}

size_test!(
    TypedNeoDateFormatter<icu_calendar::Gregorian>,
    typed_neo_date_formatter_size,
    pinned = 512,
    nightly = 496
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
    names: RawDateTimeNames,
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
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = TypedNeoDateFormatter::<Gregorian>::try_new_with_length(
    ///     &locale!("es-MX").into(),
    ///     length::Date::Full,
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Date::try_new_gregorian_date(2023, 12, 20).unwrap()),
    ///     "miércoles, 20 de diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_length(locale: &DataLocale, length: length::Date) -> Result<Self, Error>
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
    ) -> Result<Self, Error>
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // Date formatting keys
            + DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader,
    {
        let selection = DatePatternSelectionData::try_new_with_length::<C::DatePatternV1Marker>(
            provider, locale, length,
        )?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern::<C::YearNamesV1Marker, C::MonthNamesV1Marker>(
            Some(provider),           // year
            Some(provider),           // month
            Some(provider),           // weekday
            None::<&PhantomProvider>, // day period
            Some(loader),             // fixed decimal formatter
            Some(loader),             // week calculator
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
    NeoDateFormatter,
    neo_date_formatter_size,
    pinned = 568,
    nightly = 552
);

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
    names: RawDateTimeNames,
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
    /// use writeable::assert_writeable_eq;
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
    /// assert_writeable_eq!(
    ///     df.format(&any_datetime).expect("Calendars should match"),
    ///     "Sep 1, 2020"
    /// );
    /// ```
    ///
    /// [`AnyCalendarKind`]: icu_calendar::AnyCalendarKind
    #[inline(never)]
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_length(locale: &DataLocale, length: length::Date) -> Result<Self, Error> {
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<BuddhistDatePatternV1Marker>
            + DataProvider<BuddhistYearNamesV1Marker>
            + DataProvider<BuddhistMonthNamesV1Marker>
            + DataProvider<ChineseDatePatternV1Marker>
            + DataProvider<ChineseYearNamesV1Marker>
            + DataProvider<ChineseMonthNamesV1Marker>
            + DataProvider<CopticDatePatternV1Marker>
            + DataProvider<CopticYearNamesV1Marker>
            + DataProvider<CopticMonthNamesV1Marker>
            + DataProvider<DangiDatePatternV1Marker>
            + DataProvider<DangiYearNamesV1Marker>
            + DataProvider<DangiMonthNamesV1Marker>
            + DataProvider<EthiopianDatePatternV1Marker>
            + DataProvider<EthiopianYearNamesV1Marker>
            + DataProvider<EthiopianMonthNamesV1Marker>
            + DataProvider<GregorianDatePatternV1Marker>
            + DataProvider<GregorianYearNamesV1Marker>
            + DataProvider<GregorianMonthNamesV1Marker>
            + DataProvider<HebrewDatePatternV1Marker>
            + DataProvider<HebrewYearNamesV1Marker>
            + DataProvider<HebrewMonthNamesV1Marker>
            + DataProvider<IndianDatePatternV1Marker>
            + DataProvider<IndianYearNamesV1Marker>
            + DataProvider<IndianMonthNamesV1Marker>
            + DataProvider<IslamicDatePatternV1Marker>
            + DataProvider<IslamicYearNamesV1Marker>
            + DataProvider<IslamicMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<JapaneseExtendedDatePatternV1Marker>
            + DataProvider<JapaneseExtendedYearNamesV1Marker>
            + DataProvider<JapaneseExtendedMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<PersianDatePatternV1Marker>
            + DataProvider<PersianYearNamesV1Marker>
            + DataProvider<PersianMonthNamesV1Marker>
            + DataProvider<RocDatePatternV1Marker>
            + DataProvider<RocYearNamesV1Marker>
            + DataProvider<RocMonthNamesV1Marker>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>
            // AnyCalendar constructor keys
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<BuddhistDatePatternV1Marker>
            + DataProvider<BuddhistYearNamesV1Marker>
            + DataProvider<BuddhistMonthNamesV1Marker>
            + DataProvider<ChineseDatePatternV1Marker>
            + DataProvider<ChineseYearNamesV1Marker>
            + DataProvider<ChineseMonthNamesV1Marker>
            + DataProvider<CopticDatePatternV1Marker>
            + DataProvider<CopticYearNamesV1Marker>
            + DataProvider<CopticMonthNamesV1Marker>
            + DataProvider<DangiDatePatternV1Marker>
            + DataProvider<DangiYearNamesV1Marker>
            + DataProvider<DangiMonthNamesV1Marker>
            + DataProvider<EthiopianDatePatternV1Marker>
            + DataProvider<EthiopianYearNamesV1Marker>
            + DataProvider<EthiopianMonthNamesV1Marker>
            + DataProvider<GregorianDatePatternV1Marker>
            + DataProvider<GregorianYearNamesV1Marker>
            + DataProvider<GregorianMonthNamesV1Marker>
            + DataProvider<HebrewDatePatternV1Marker>
            + DataProvider<HebrewYearNamesV1Marker>
            + DataProvider<HebrewMonthNamesV1Marker>
            + DataProvider<IndianDatePatternV1Marker>
            + DataProvider<IndianYearNamesV1Marker>
            + DataProvider<IndianMonthNamesV1Marker>
            + DataProvider<IslamicDatePatternV1Marker>
            + DataProvider<IslamicYearNamesV1Marker>
            + DataProvider<IslamicMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<JapaneseExtendedDatePatternV1Marker>
            + DataProvider<JapaneseExtendedYearNamesV1Marker>
            + DataProvider<JapaneseExtendedMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<PersianDatePatternV1Marker>
            + DataProvider<PersianYearNamesV1Marker>
            + DataProvider<PersianMonthNamesV1Marker>
            + DataProvider<RocDatePatternV1Marker>
            + DataProvider<RocYearNamesV1Marker>
            + DataProvider<RocMonthNamesV1Marker>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader + AnyCalendarLoader,
    {
        let calendar = AnyCalendarLoader::load(loader, locale)?;
        let kind = calendar.kind();
        let any_calendar_provider = AnyCalendarProvider { provider, kind };
        let selection = DatePatternSelectionData::try_new_with_length::<ErasedDatePatternV1Marker>(
            &any_calendar_provider,
            locale,
            length,
        )?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern::<ErasedYearNamesV1Marker, ErasedMonthNamesV1Marker>(
            Some(&any_calendar_provider), // year
            Some(&any_calendar_provider), // month
            Some(provider),               // weekday
            None::<&PhantomProvider>,     // day period
            Some(loader),                 // fixed decimal formatter
            Some(loader),                 // week calculator
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

impl<'a> Writeable for FormattedNeoDate<'a> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        DateTimeWriter {
            datetime: &self.datetime,
            names: self.names,
            pattern_items: self.pattern.iter_items(),
            pattern_metadata: self.pattern.metadata(),
        }
        .write_to(sink)
    }

    // TODO(#489): Implement writeable_length_hint
}

writeable::impl_display_with_writeable!(FormattedNeoDate<'_>);

impl<'a> FormattedNeoDate<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}

size_test!(
    NeoTimeFormatter,
    neo_time_formatter_size,
    pinned = 464,
    nightly = 448
);

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
    names: RawDateTimeNames,
}

impl NeoTimeFormatter {
    /// Creates a [`NeoTimeFormatter`] for a time length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::types::Time;
    /// use icu::datetime::neo::NeoTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = NeoTimeFormatter::try_new_with_length(
    ///     &locale!("es-MX").into(),
    ///     length::Time::Medium,
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Time::try_new(14, 48, 58, 0).unwrap()),
    ///     "2:48:58 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_length(locale: &DataLocale, length: length::Time) -> Result<Self, Error> {
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
    ) -> Result<Self, Error>
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>,
        L: FixedDecimalFormatterLoader,
    {
        let selection = TimePatternSelectionData::try_new_with_length(provider, locale, length)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        // NOTE: The Gregorian types below are placeholders only. They are not actually linked.
        names.load_for_pattern::<GregorianYearNamesV1Marker, GregorianMonthNamesV1Marker>(
            None::<&PhantomProvider>, // year
            None::<&PhantomProvider>, // month
            None::<&PhantomProvider>, // weekday
            Some(provider),           // day period
            Some(loader),             // fixed decimal formatter
            None::<&PhantomLoader>,   // week calculator
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

impl<'a> Writeable for FormattedNeoTime<'a> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        DateTimeWriter {
            datetime: &self.datetime,
            names: self.names,
            pattern_items: self.pattern.iter_items(),
            pattern_metadata: self.pattern.metadata(),
        }
        .write_to(sink)
    }

    // TODO(#489): Implement writeable_length_hint
}

writeable::impl_display_with_writeable!(FormattedNeoTime<'_>);

impl<'a> FormattedNeoTime<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}

size_test!(
    TypedNeoDateTimeFormatter<icu_calendar::Gregorian>,
    typed_neo_date_time_formatter_size,
    pinned = 584,
    nightly = 568
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
    names: RawDateTimeNames,
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
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     TypedNeoDateTimeFormatter::<Gregorian>::try_new_with_date_length(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
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
    ) -> Result<Self, Error>
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
    ) -> Result<Self, Error>
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
    ) -> Result<Self, Error>
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
            names: date_formatter.names,
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
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     TypedNeoDateTimeFormatter::<Gregorian>::try_new_with_time_length(
    ///         &locale!("es-MX").into(),
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
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
    ) -> Result<Self, Error> {
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
    ) -> Result<Self, Error>
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
    ) -> Result<Self, Error>
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
            names: time_formatter.names,
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
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     TypedNeoDateTimeFormatter::<Gregorian>::try_new_with_lengths(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
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
    ) -> Result<Self, Error>
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
    ) -> Result<Self, Error>
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
    ) -> Result<Self, Error>
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
        let selection = DateTimeGluePatternSelectionData::try_new_with_lengths::<
            C::DatePatternV1Marker,
            _,
        >(provider, provider, locale, date_length, time_length)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern::<C::YearNamesV1Marker, C::MonthNamesV1Marker>(
            Some(provider), // year
            Some(provider), // month
            Some(provider), // weekday
            Some(provider), // day period
            Some(loader),   // fixed decimal formatter
            Some(loader),   // week calculator
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
    ) -> Result<Self, Error>
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
            _ => Err(Error::UnsupportedOptions),
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

size_test!(
    NeoDateTimeFormatter,
    neo_date_time_formatter_size,
    pinned = 640,
    nightly = 624
);

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
    names: RawDateTimeNames,
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
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     NeoDateTimeFormatter::try_new_with_date_length(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
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
    ) -> Result<Self, Error> {
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<BuddhistDatePatternV1Marker>
            + DataProvider<BuddhistYearNamesV1Marker>
            + DataProvider<BuddhistMonthNamesV1Marker>
            + DataProvider<ChineseDatePatternV1Marker>
            + DataProvider<ChineseYearNamesV1Marker>
            + DataProvider<ChineseMonthNamesV1Marker>
            + DataProvider<CopticDatePatternV1Marker>
            + DataProvider<CopticYearNamesV1Marker>
            + DataProvider<CopticMonthNamesV1Marker>
            + DataProvider<DangiDatePatternV1Marker>
            + DataProvider<DangiYearNamesV1Marker>
            + DataProvider<DangiMonthNamesV1Marker>
            + DataProvider<EthiopianDatePatternV1Marker>
            + DataProvider<EthiopianYearNamesV1Marker>
            + DataProvider<EthiopianMonthNamesV1Marker>
            + DataProvider<GregorianDatePatternV1Marker>
            + DataProvider<GregorianYearNamesV1Marker>
            + DataProvider<GregorianMonthNamesV1Marker>
            + DataProvider<HebrewDatePatternV1Marker>
            + DataProvider<HebrewYearNamesV1Marker>
            + DataProvider<HebrewMonthNamesV1Marker>
            + DataProvider<IndianDatePatternV1Marker>
            + DataProvider<IndianYearNamesV1Marker>
            + DataProvider<IndianMonthNamesV1Marker>
            + DataProvider<IslamicDatePatternV1Marker>
            + DataProvider<IslamicYearNamesV1Marker>
            + DataProvider<IslamicMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<JapaneseExtendedDatePatternV1Marker>
            + DataProvider<JapaneseExtendedYearNamesV1Marker>
            + DataProvider<JapaneseExtendedMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<PersianDatePatternV1Marker>
            + DataProvider<PersianYearNamesV1Marker>
            + DataProvider<PersianMonthNamesV1Marker>
            + DataProvider<RocDatePatternV1Marker>
            + DataProvider<RocYearNamesV1Marker>
            + DataProvider<RocMonthNamesV1Marker>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>
            // AnyCalendar constructor keys
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<BuddhistDatePatternV1Marker>
            + DataProvider<BuddhistYearNamesV1Marker>
            + DataProvider<BuddhistMonthNamesV1Marker>
            + DataProvider<ChineseDatePatternV1Marker>
            + DataProvider<ChineseYearNamesV1Marker>
            + DataProvider<ChineseMonthNamesV1Marker>
            + DataProvider<CopticDatePatternV1Marker>
            + DataProvider<CopticYearNamesV1Marker>
            + DataProvider<CopticMonthNamesV1Marker>
            + DataProvider<DangiDatePatternV1Marker>
            + DataProvider<DangiYearNamesV1Marker>
            + DataProvider<DangiMonthNamesV1Marker>
            + DataProvider<EthiopianDatePatternV1Marker>
            + DataProvider<EthiopianYearNamesV1Marker>
            + DataProvider<EthiopianMonthNamesV1Marker>
            + DataProvider<GregorianDatePatternV1Marker>
            + DataProvider<GregorianYearNamesV1Marker>
            + DataProvider<GregorianMonthNamesV1Marker>
            + DataProvider<HebrewDatePatternV1Marker>
            + DataProvider<HebrewYearNamesV1Marker>
            + DataProvider<HebrewMonthNamesV1Marker>
            + DataProvider<IndianDatePatternV1Marker>
            + DataProvider<IndianYearNamesV1Marker>
            + DataProvider<IndianMonthNamesV1Marker>
            + DataProvider<IslamicDatePatternV1Marker>
            + DataProvider<IslamicYearNamesV1Marker>
            + DataProvider<IslamicMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<JapaneseExtendedDatePatternV1Marker>
            + DataProvider<JapaneseExtendedYearNamesV1Marker>
            + DataProvider<JapaneseExtendedMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<PersianDatePatternV1Marker>
            + DataProvider<PersianYearNamesV1Marker>
            + DataProvider<PersianMonthNamesV1Marker>
            + DataProvider<RocDatePatternV1Marker>
            + DataProvider<RocYearNamesV1Marker>
            + DataProvider<RocMonthNamesV1Marker>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader + AnyCalendarLoader,
    {
        let date_formatter =
            NeoDateFormatter::try_new_with_length_internal(provider, loader, locale, length)?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Date(date_formatter.selection),
            names: date_formatter.names,
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
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     NeoDateTimeFormatter::try_new_with_time_length(
    ///         &locale!("es-MX").into(),
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
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
    ) -> Result<Self, Error> {
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            // AnyCalendar constructor keys
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<DangiCacheV1Marker>
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>,
        L: FixedDecimalFormatterLoader + AnyCalendarLoader,
    {
        // Need to compute the calendar ourselves since it is not in NeoTimeFormatter
        let calendar = AnyCalendarLoader::load(loader, locale)?;
        let time_formatter =
            NeoTimeFormatter::try_new_with_length_internal(provider, loader, locale, length)?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Time(time_formatter.selection),
            names: time_formatter.names,
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
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     NeoDateTimeFormatter::try_new_with_lengths(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
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
    ) -> Result<Self, Error> {
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<BuddhistDatePatternV1Marker>
            + DataProvider<BuddhistYearNamesV1Marker>
            + DataProvider<BuddhistMonthNamesV1Marker>
            + DataProvider<ChineseDatePatternV1Marker>
            + DataProvider<ChineseYearNamesV1Marker>
            + DataProvider<ChineseMonthNamesV1Marker>
            + DataProvider<CopticDatePatternV1Marker>
            + DataProvider<CopticYearNamesV1Marker>
            + DataProvider<CopticMonthNamesV1Marker>
            + DataProvider<DangiDatePatternV1Marker>
            + DataProvider<DangiYearNamesV1Marker>
            + DataProvider<DangiMonthNamesV1Marker>
            + DataProvider<EthiopianDatePatternV1Marker>
            + DataProvider<EthiopianYearNamesV1Marker>
            + DataProvider<EthiopianMonthNamesV1Marker>
            + DataProvider<GregorianDatePatternV1Marker>
            + DataProvider<GregorianYearNamesV1Marker>
            + DataProvider<GregorianMonthNamesV1Marker>
            + DataProvider<HebrewDatePatternV1Marker>
            + DataProvider<HebrewYearNamesV1Marker>
            + DataProvider<HebrewMonthNamesV1Marker>
            + DataProvider<IndianDatePatternV1Marker>
            + DataProvider<IndianYearNamesV1Marker>
            + DataProvider<IndianMonthNamesV1Marker>
            + DataProvider<IslamicDatePatternV1Marker>
            + DataProvider<IslamicYearNamesV1Marker>
            + DataProvider<IslamicMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<JapaneseExtendedDatePatternV1Marker>
            + DataProvider<JapaneseExtendedYearNamesV1Marker>
            + DataProvider<JapaneseExtendedMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<PersianDatePatternV1Marker>
            + DataProvider<PersianYearNamesV1Marker>
            + DataProvider<PersianMonthNamesV1Marker>
            + DataProvider<RocDatePatternV1Marker>
            + DataProvider<RocYearNamesV1Marker>
            + DataProvider<RocMonthNamesV1Marker>
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
    ) -> Result<Self, Error>
    where
        P: ?Sized
            // DatePattern, YearNames, and MonthNames keys
            + DataProvider<BuddhistDatePatternV1Marker>
            + DataProvider<BuddhistYearNamesV1Marker>
            + DataProvider<BuddhistMonthNamesV1Marker>
            + DataProvider<ChineseDatePatternV1Marker>
            + DataProvider<ChineseYearNamesV1Marker>
            + DataProvider<ChineseMonthNamesV1Marker>
            + DataProvider<CopticDatePatternV1Marker>
            + DataProvider<CopticYearNamesV1Marker>
            + DataProvider<CopticMonthNamesV1Marker>
            + DataProvider<DangiDatePatternV1Marker>
            + DataProvider<DangiYearNamesV1Marker>
            + DataProvider<DangiMonthNamesV1Marker>
            + DataProvider<EthiopianDatePatternV1Marker>
            + DataProvider<EthiopianYearNamesV1Marker>
            + DataProvider<EthiopianMonthNamesV1Marker>
            + DataProvider<GregorianDatePatternV1Marker>
            + DataProvider<GregorianYearNamesV1Marker>
            + DataProvider<GregorianMonthNamesV1Marker>
            + DataProvider<HebrewDatePatternV1Marker>
            + DataProvider<HebrewYearNamesV1Marker>
            + DataProvider<HebrewMonthNamesV1Marker>
            + DataProvider<IndianDatePatternV1Marker>
            + DataProvider<IndianYearNamesV1Marker>
            + DataProvider<IndianMonthNamesV1Marker>
            + DataProvider<IslamicDatePatternV1Marker>
            + DataProvider<IslamicYearNamesV1Marker>
            + DataProvider<IslamicMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<JapaneseExtendedDatePatternV1Marker>
            + DataProvider<JapaneseExtendedYearNamesV1Marker>
            + DataProvider<JapaneseExtendedMonthNamesV1Marker>
            + DataProvider<JapaneseDatePatternV1Marker>
            + DataProvider<JapaneseYearNamesV1Marker>
            + DataProvider<JapaneseMonthNamesV1Marker>
            + DataProvider<PersianDatePatternV1Marker>
            + DataProvider<PersianYearNamesV1Marker>
            + DataProvider<PersianMonthNamesV1Marker>
            + DataProvider<RocDatePatternV1Marker>
            + DataProvider<RocYearNamesV1Marker>
            + DataProvider<RocMonthNamesV1Marker>
            // Other date formatting keys
            + DataProvider<WeekdayNamesV1Marker>
            // Time formatting keys
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            // DateTime glue key
            + DataProvider<DateTimePatternV1Marker>,
        L: FixedDecimalFormatterLoader + WeekCalculatorLoader + AnyCalendarLoader,
    {
        let calendar = AnyCalendarLoader::load(loader, locale)?;
        let kind = calendar.kind();
        let any_calendar_provider = AnyCalendarProvider { provider, kind };
        let selection =
            DateTimeGluePatternSelectionData::try_new_with_lengths::<ErasedDatePatternV1Marker, _>(
                &any_calendar_provider,
                provider,
                locale,
                date_length,
                time_length,
            )?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern::<ErasedYearNamesV1Marker, ErasedMonthNamesV1Marker>(
            Some(&any_calendar_provider), // year
            Some(&any_calendar_provider), // month
            Some(provider),               // weekday
            Some(provider),               // day period
            Some(loader),                 // fixed decimal formatter
            Some(loader),                 // week calculator
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
        date: &T,
    ) -> Result<FormattedNeoDateTime, crate::MismatchedCalendarError>
    where
        T: DateTimeInput<Calendar = AnyCalendar>,
    {
        let datetime =
            if let Some(converted) = crate::calendar::convert_if_necessary(&self.calendar, date)? {
                ExtractedDateTimeInput::extract_from_date(&converted)
            } else {
                ExtractedDateTimeInput::extract_from_date(date)
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

impl<'a> Writeable for FormattedNeoDateTime<'a> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        DateTimeWriter {
            datetime: &self.datetime,
            names: self.names,
            pattern_items: self.pattern.iter_items(),
            pattern_metadata: self.pattern.metadata(),
        }
        .write_to(sink)
    }

    // TODO(#489): Implement writeable_length_hint
}

writeable::impl_display_with_writeable!(FormattedNeoDateTime<'_>);

impl<'a> FormattedNeoDateTime<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}
