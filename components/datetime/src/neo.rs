// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use crate::dynamic::CompositeFieldSet;
use crate::external_loaders::*;
use crate::format::datetime::try_write_pattern_items;
use crate::format::neo::*;
use crate::input::ExtractedInput;
use crate::neo_pattern::DateTimePattern;
use crate::options::preferences::HourCycle;
use crate::raw::neo::*;
use crate::scaffold::*;
use crate::scaffold::{
    AllInputMarkers, ConvertCalendar, DateDataMarkers, DateInputMarkers, DateTimeMarkers, GetField,
    IsAnyCalendarKind, IsInCalendar, TimeMarkers, TypedDateDataMarkers, ZoneMarkers,
};
use crate::DateTimeWriteError;
use crate::MismatchedCalendarError;
use core::fmt;
use core::marker::PhantomData;
use icu_calendar::any_calendar::IntoAnyCalendar;
use icu_calendar::AnyCalendar;
use icu_provider::prelude::*;
use writeable::TryWriteable;

/// Helper macro for generating any/buffer constructors in this file.
macro_rules! gen_any_buffer_constructors_with_external_loader {
    (@runtime_fset, $fset:ident, $compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident) => {
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::$compiled_fn)]
        pub fn $any_fn<P>(
            provider: &P,
            locale: &DataLocale,
            field_set: $fset,
        ) -> Result<Self, PatternLoadError>
        where
            P: AnyProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_downcasting(),
                &ExternalLoaderAny(provider),
                locale,
                field_set.get_field(),
            )
        }
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::$compiled_fn)]
        #[cfg(feature = "serde")]
        pub fn $buffer_fn<P>(
            provider: &P,
            locale: &DataLocale,
            field_set: $fset,
        ) -> Result<Self, PatternLoadError>
        where
            P: BufferProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_deserializing(),
                &ExternalLoaderBuffer(provider),
                locale,
                field_set.get_field(),
            )
        }
    };
    (@compiletime_fset, $fset:ident, $compiled_fn:ident, $any_fn:ident, $buffer_fn:ident, $internal_fn:ident) => {
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::$compiled_fn)]
        pub fn $any_fn<P>(
            provider: &P,
            locale: &DataLocale,
            field_set: $fset,
        ) -> Result<Self, PatternLoadError>
        where
            P: AnyProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_downcasting(),
                &ExternalLoaderAny(provider),
                locale,
                field_set.get_field(),
            )
        }
        #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::$compiled_fn)]
        #[cfg(feature = "serde")]
        pub fn $buffer_fn<P>(
            provider: &P,
            locale: &DataLocale,
            field_set: $fset,
        ) -> Result<Self, PatternLoadError>
        where
            P: BufferProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_deserializing(),
                &ExternalLoaderBuffer(provider),
                locale,
                field_set.get_field(),
            )
        }
    };
}

// impl RawNeoOptions {
//     pub(crate) fn from_field_set_and_locale<FSet>(field_set: &FSet, locale: &DataLocale) -> Self
//     where
//         FSet: DateTimeMarkers,
//         FSet: GetField<FSet::LengthOption>,
//         FSet: GetField<FSet::AlignmentOption>,
//         FSet: GetField<FSet::YearStyleOption>,
//         FSet: GetField<FSet::TimePrecisionOption>,
//     {
//         // TODO: Return an error if there are more options than field set
//         let hour_cycle = locale
//             .get_unicode_ext(&icu_locale_core::extensions::unicode::key!("hc"))
//             .as_ref()
//             .and_then(HourCycle::from_locale_value);
//         Self {
//             length: match GetField::<FSet::LengthOption>::get_field(field_set).into_option() {
//                 Some(length) => length,
//                 None => {
//                     debug_assert!(false, "unreachable");
//                     NeoSkeletonLength::Medium
//                 }
//             },
//             alignment: GetField::<FSet::AlignmentOption>::get_field(field_set).into_option(),
//             year_style: GetField::<FSet::YearStyleOption>::get_field(field_set).into_option(),
//             time_precision: GetField::<FSet::TimePrecisionOption>::get_field(field_set)
//                 .into_option(),
//             hour_cycle,
//         }
//     }
// }

size_test!(FixedCalendarDateTimeFormatter<icu_calendar::Gregorian, crate::fieldset::YMD>, typed_neo_year_month_day_formatter_size, 456);

/// [`FixedCalendarDateTimeFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at compile time.
///
/// For more details, please read the [crate root docs][crate].
///
#[doc = typed_neo_year_month_day_formatter_size!()]
#[derive(Debug)]
pub struct FixedCalendarDateTimeFormatter<C: CldrCalendar, FSet: DateTimeNamesMarker> {
    selection: DateTimeZonePatternSelectionData,
    names: RawDateTimeNames<FSet>,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar, FSet: DateTimeMarkers> FixedCalendarDateTimeFormatter<C, FSet>
where
    FSet::D: TypedDateDataMarkers<C>,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<CompositeFieldSet>,
{
    /// Creates a new [`FixedCalendarDateTimeFormatter`] from compiled data with
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
    /// use icu::datetime::fieldset::YMD;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     &locale!("es-MX").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&Date::try_new_gregorian(2023, 12, 20).unwrap()),
    ///     "20 de diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale, field_set: FSet) -> Result<Self, PatternLoadError>
    where
        crate::provider::Baked: AllFixedCalendarFormattingDataMarkers<C, FSet>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            field_set.get_field(),
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
    ) -> Result<Self, PatternLoadError>
    where
        P: ?Sized
            + AllFixedCalendarFormattingDataMarkers<C, FSet>
            + AllFixedCalendarExternalDataMarkers,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            field_set.get_field(),
        )
    }
}

/*
impl<C: CldrCalendar, FSet: DateTimeMarkers>
    FixedCalendarDateTimeFormatter<C, FSet>
where
    FSet::D: TypedDateDataMarkers<C>,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<CompositeFieldSet>,
    FSet: GetField<FSet::LengthOption>,
    FSet: GetField<FSet::AlignmentOption>,
    FSet: GetField<FSet::YearStyleOption>,
    FSet: GetField<FSet::TimePrecisionOption>,
{
    /// Creates a new [`FixedCalendarDateTimeFormatter`] from compiled data with
    /// datetime components specified at runtime.
    ///
    /// If you know the datetime components at build time, use
    /// [`FixedCalendarDateTimeFormatter::try_new`] for smaller data size and memory use.
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
    /// use icu::datetime::neo_skeleton::NeoDateComponents;
    /// use icu::datetime::neo_skeleton::NeoDateSkeleton;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt =
    ///     FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new_with_skeleton(
    ///         &locale!("es-MX").into(),
    ///         NeoDateComponents::DayWeekday.medium(),
    ///     )
    ///     .unwrap();
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
    /// use icu::datetime::neo_skeleton::NeoCalendarPeriodComponents;
    /// use icu::datetime::neo_skeleton::NeoCalendarPeriodSkeleton;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt =
    ///     FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new_with_skeleton(
    ///         &locale!("es-MX").into(),
    ///         NeoCalendarPeriodComponents::YearMonth.medium(),
    ///     )
    ///     .unwrap();
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
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::datetime::neo_skeleton::TimePrecision;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt =
    ///     FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new_with_skeleton(
    ///         &locale!("es-MX").into(),
    ///         {
    ///             let mut skeleton = NeoTimeComponents::Time.medium();
    ///             skeleton.time_precision = Some(TimePrecision::HourExact);
    ///             skeleton
    ///         }
    ///     )
    ///     .unwrap();
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
    /// use icu::datetime::neo_skeleton::NeoDateComponents;
    /// use icu::datetime::neo_skeleton::NeoDateTimeComponents;
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt =
    ///     FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new_with_skeleton(
    ///         &locale!("es-MX").into(),
    ///         NeoDateTimeComponents::DateTime(
    ///             NeoDateComponents::Weekday,
    ///             NeoTimeComponents::Time,
    ///         )
    ///         .long()
    ///         .hm(),
    ///     )
    ///     .unwrap();
    /// let dt = DateTime::try_new_gregorian(2024, 1, 10, 16, 20, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(fmt.format(&dt), "miércoles 4:20 p.m.");
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_skeleton(
        locale: &DataLocale,
        skeleton: FSet,
    ) -> Result<Self, PatternLoadError>
    where
        crate::provider::Baked: AllFixedCalendarFormattingDataMarkers<C, FSet>,
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
    ) -> Result<Self, PatternLoadError>
    where
        P: ?Sized
            + AllFixedCalendarFormattingDataMarkers<C, FSet>
            + AllFixedCalendarExternalDataMarkers,
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
*/

impl<C: CldrCalendar, FSet: DateTimeMarkers> FixedCalendarDateTimeFormatter<C, FSet>
where
    FSet::D: TypedDateDataMarkers<C>,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        field_set: CompositeFieldSet,
    ) -> Result<Self, PatternLoadError>
    where
        P: ?Sized + AllFixedCalendarFormattingDataMarkers<C, FSet>,
        L: FixedDecimalFormatterLoader,
    {
        // TODO: Remove this when NeoOptions is gone
        let prefs = RawPreferences {
            hour_cycle: locale
                .get_unicode_ext(&icu_locale_core::extensions::unicode::key!("hc"))
                .as_ref()
                .and_then(HourCycle::from_locale_value),
        };
        // END TODO

        let selection = DateTimeZonePatternSelectionData::try_new_with_skeleton(
            &<FSet::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1Marker::bind(provider),
            &<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker::bind(provider),
            &FSet::GluePatternV1Marker::bind(provider),
            locale,
            field_set,
            prefs,
        )
        .map_err(PatternLoadError::Data)?;
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

impl<C: CldrCalendar, FSet: DateTimeMarkers> FixedCalendarDateTimeFormatter<C, FSet>
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
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::YMD;
    /// use icu::locale::locale;
    ///
    /// let formatter =
    ///     FixedCalendarDateTimeFormatter::<Buddhist, _>::try_new(
    ///         &locale!("es-MX").into(),
    ///         YMD::long(),
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
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::YMD;
    /// use icu::locale::locale;
    ///
    /// let formatter =
    ///     FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
    ///         &locale!("es-MX").into(),
    ///         YMD::long(),
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
    DateTimeFormatter<crate::fieldset::YMD>,
    neo_year_month_day_formatter_size,
    512
);

/// [`DateTimeFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at runtime.
///
/// For more details, please read the [crate root docs][crate].
///
#[doc = neo_year_month_day_formatter_size!()]
#[derive(Debug)]
pub struct DateTimeFormatter<FSet: DateTimeNamesMarker> {
    selection: DateTimeZonePatternSelectionData,
    names: RawDateTimeNames<FSet>,
    calendar: AnyCalendar,
}

impl<FSet: DateTimeMarkers> DateTimeFormatter<FSet>
where
    FSet::D: DateDataMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<CompositeFieldSet>,
{
    /// Creates a new [`DateTimeFormatter`] from compiled data with
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
    /// use icu::datetime::fieldset::YMD;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use std::str::FromStr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let locale = locale!("en-u-ca-hebrew");
    ///
    /// let formatter =
    ///     DateTimeFormatter::try_new(&locale.into(), YMD::medium()).unwrap();
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
    pub fn try_new(locale: &DataLocale, field_set: FSet) -> Result<Self, PatternLoadError>
    where
        crate::provider::Baked: AllAnyCalendarFormattingDataMarkers<FSet>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            locale,
            field_set.get_field(),
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
    ) -> Result<Self, PatternLoadError>
    where
        P: ?Sized + AllAnyCalendarFormattingDataMarkers<FSet> + AllAnyCalendarExternalDataMarkers,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            locale,
            field_set.get_field(),
        )
    }
}

/*
impl<FSet: DateTimeMarkers + IsRuntimeComponents> DateTimeFormatter<FSet>
where
    FSet::D: DateDataMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<FSet::LengthOption>,
    FSet: GetField<FSet::AlignmentOption>,
    FSet: GetField<FSet::YearStyleOption>,
    FSet: GetField<FSet::TimePrecisionOption>,
{
    /// Creates a new [`DateTimeFormatter`] from compiled data with
    /// datetime components specified at runtime.
    ///
    /// If you know the datetime components at build time, use
    /// [`DateTimeFormatter::try_new`] for smaller data size and memory use.
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
    /// use icu::datetime::neo_skeleton::NeoDateComponents;
    /// use icu::datetime::neo_skeleton::NeoDateSkeleton;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = DateTimeFormatter::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoDateComponents::DayWeekday.medium(),
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
    /// use icu::datetime::neo_skeleton::NeoCalendarPeriodComponents;
    /// use icu::datetime::neo_skeleton::NeoCalendarPeriodSkeleton;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = DateTimeFormatter::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoCalendarPeriodComponents::YearMonth.medium(),
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
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::datetime::neo_skeleton::TimePrecision;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = DateTimeFormatter::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     {
    ///         let mut skeleton = NeoTimeComponents::Time.medium();
    ///         skeleton.time_precision = Some(TimePrecision::HourExact);
    ///         skeleton
    ///     }
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
    /// use icu::datetime::neo_skeleton::NeoDateComponents;
    /// use icu::datetime::neo_skeleton::NeoDateTimeComponents;
    /// use icu::datetime::neo_skeleton::NeoDateTimeSkeleton;
    /// use icu::datetime::neo_skeleton::NeoTimeComponents;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = DateTimeFormatter::try_new_with_skeleton(
    ///     &locale!("es-MX").into(),
    ///     NeoDateTimeComponents::DateTime(
    ///         NeoDateComponents::Weekday,
    ///         NeoTimeComponents::Time,
    ///     )
    ///     .long()
    ///     .hm(),
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
    pub fn try_new_with_skeleton(
        locale: &DataLocale,
        skeleton: FSet,
    ) -> Result<Self, PatternLoadError>
    where
        crate::provider::Baked: AllAnyCalendarFormattingDataMarkers<FSet>,
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
    ) -> Result<Self, PatternLoadError>
    where
        P: ?Sized + AllAnyCalendarFormattingDataMarkers<FSet> + AllAnyCalendarExternalDataMarkers,
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
*/

impl<FSet: DateTimeMarkers> DateTimeFormatter<FSet>
where
    FSet::D: DateDataMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        locale: &DataLocale,
        field_set: CompositeFieldSet,
    ) -> Result<Self, PatternLoadError>
    where
        P: ?Sized + AllAnyCalendarFormattingDataMarkers<FSet>,
        L: FixedDecimalFormatterLoader + AnyCalendarLoader,
    {
        // TODO: Remove this when NeoOptions is gone
        let prefs = RawPreferences {
            hour_cycle: locale
                .get_unicode_ext(&icu_locale_core::extensions::unicode::key!("hc"))
                .as_ref()
                .and_then(HourCycle::from_locale_value),
        };
        // END TODO

        let calendar = AnyCalendarLoader::load(loader, locale).map_err(PatternLoadError::Data)?;
        let kind = calendar.kind();
        let selection = DateTimeZonePatternSelectionData::try_new_with_skeleton(
            &AnyCalendarProvider::<<FSet::D as DateDataMarkers>::Skel, _>::new(provider, kind),
            &<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1Marker::bind(provider),
            &FSet::GluePatternV1Marker::bind(provider),
            locale,
            field_set,
            prefs,
        )
        .map_err(PatternLoadError::Data)?;
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

impl<FSet: DateTimeMarkers> DateTimeFormatter<FSet>
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
    /// use icu::datetime::fieldset::YMD;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::MismatchedCalendarError;
    /// use icu::locale::locale;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     &locale!("en-u-ca-hebrew").into(),
    ///     YMD::long(),
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
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::fieldset::YMD;
    /// use icu::locale::locale;
    ///
    /// let formatter = DateTimeFormatter::try_new(
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
        datetime.check_any_calendar_kind(self.calendar.kind())?;
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
    /// use icu::datetime::fieldset::YMD;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::MismatchedCalendarError;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     &locale!("en-u-ca-hebrew").into(),
    ///     YMD::long(),
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
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::fieldset::YMD;
    /// use icu::locale::locale;
    ///
    /// let formatter = DateTimeFormatter::try_new(
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

impl<C: CldrCalendar, FSet: DateTimeMarkers> FixedCalendarDateTimeFormatter<C, FSet> {
    /// Make this [`FixedCalendarDateTimeFormatter`] adopt a calendar so it can format any date.
    ///
    /// This is useful if you need a [`DateTimeFormatter`] but know the calendar system ahead of time,
    /// so that you do not need to link extra data you aren't using.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::cal::Hebrew;
    /// use icu::calendar::Date;
    /// use icu::datetime::fieldset::YMD;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     &locale!("en").into(),
    ///     YMD::long(),
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
    pub fn into_formatter(self, calendar: C) -> DateTimeFormatter<FSet>
    where
        C: IntoAnyCalendar,
    {
        DateTimeFormatter {
            selection: self.selection,
            names: self.names,
            calendar: calendar.to_any(),
        }
    }
}

impl<FSet: DateTimeMarkers> DateTimeFormatter<FSet> {
    /// Attempt to convert this [`DateTimeFormatter`] into one with a specific calendar.
    ///
    /// Returns an error if the type parameter does not match the inner calendar.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::cal::Hebrew;
    /// use icu::calendar::Date;
    /// use icu::datetime::fieldset::YMD;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     &locale!("en-u-ca-hebrew").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap()
    /// .try_into_typed_formatter::<Hebrew>()
    /// .unwrap();
    ///
    /// let date = Date::try_new_hebrew(5785, 1, 12).unwrap();
    ///
    /// assert_try_writeable_eq!(formatter.format(&date), "12 Tishri 5785");
    /// ```
    ///
    /// An error occurs if the calendars don't match:
    ///
    /// ```
    /// use icu::calendar::cal::Hebrew;
    /// use icu::calendar::Date;
    /// use icu::datetime::fieldset::YMD;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::MismatchedCalendarError;
    /// use icu::locale::locale;
    ///
    /// let result = DateTimeFormatter::try_new(
    ///     &locale!("en-u-ca-buddhist").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap()
    /// .try_into_typed_formatter::<Hebrew>();
    ///
    /// assert!(matches!(result, Err(MismatchedCalendarError { .. })));
    /// ```
    pub fn try_into_typed_formatter<C>(
        self,
    ) -> Result<FixedCalendarDateTimeFormatter<C, FSet>, MismatchedCalendarError>
    where
        C: CldrCalendar + IntoAnyCalendar,
    {
        if let Err(cal) = C::from_any(self.calendar) {
            return Err(MismatchedCalendarError {
                this_kind: cal.kind(),
                date_kind: None,
            });
        }
        Ok(FixedCalendarDateTimeFormatter {
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
