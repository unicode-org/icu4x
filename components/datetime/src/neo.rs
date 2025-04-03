// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use crate::error::DateTimeFormatterLoadError;
use crate::fieldsets::builder::FieldSetBuilder;
use crate::fieldsets::enums::CompositeFieldSet;
use crate::format::datetime::try_write_pattern_items;
use crate::format::DateTimeInputUnchecked;
use crate::pattern::*;
use crate::preferences::{CalendarAlgorithm, HourCycle, NumberingSystem};
use crate::raw::neo::*;
use crate::scaffold::*;
use crate::scaffold::{
    AllInputMarkers, ConvertCalendar, DateDataMarkers, DateInputMarkers, DateTimeMarkers, GetField,
    InFixedCalendar, InSameCalendar, TimeMarkers, TypedDateDataMarkers, ZoneMarkers,
};
use crate::size_test_macro::size_test;
use crate::MismatchedCalendarError;
use crate::{external_loaders::*, DateTimeWriteError};
use core::fmt;
use core::marker::PhantomData;
use icu_calendar::{preferences::CalendarPreferences, AnyCalendar, IntoAnyCalendar};
use icu_decimal::DecimalFormatterPreferences;
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_provider::prelude::*;
use writeable::{impl_display_with_writeable, TryWriteable, Writeable};

define_preferences!(
    /// The user locale preferences for datetime formatting.
    ///
    /// # Examples
    ///
    /// Two ways to build a preferences bag with a custom hour cycle and calendar system:
    ///
    /// ```
    /// use icu::datetime::DateTimeFormatterPreferences;
    /// use icu::locale::Locale;
    /// use icu::locale::preferences::extensions::unicode::keywords::CalendarAlgorithm;
    /// use icu::locale::preferences::extensions::unicode::keywords::HourCycle;
    /// use icu::locale::subtags::Language;
    ///
    /// let prefs1: DateTimeFormatterPreferences = Locale::try_from_str("fr-u-ca-buddhist-hc-h12").unwrap().into();
    ///
    /// let locale = Locale::try_from_str("fr").unwrap();
    /// let mut prefs2 = DateTimeFormatterPreferences::default();
    /// prefs2.locale_preferences = (&locale).into();
    /// prefs2.hour_cycle = Some(HourCycle::H12);
    /// prefs2.calendar_algorithm = Some(CalendarAlgorithm::Buddhist);
    ///
    /// assert_eq!(prefs1, prefs2);
    /// ```
    [Copy]
    DateTimeFormatterPreferences,
    {
        /// The user's preferred numbering system.
        ///
        /// Corresponds to the `-u-nu` in Unicode Locale Identifier.
        ///
        /// To get the resolved numbering system, you can inspect the data provider.
        /// See the [`icu_decimal::provider`] module for an example.
        numbering_system: NumberingSystem,
        /// The user's preferred hour cycle.
        ///
        /// Corresponds to the `-u-hc` in Unicode Locale Identifier.
        ///
        /// To get the resolved hour cycle, you can inspect the formatting pattern.
        /// See [`DateTimePattern`](crate::pattern::DateTimePattern) for an example.
        hour_cycle: HourCycle,
        /// The user's preferred calendar system
        ///
        /// Corresponds to the `-u-ca` in Unicode Locale Identifier.
        ///
        /// To get the resolved calendar system, use [`DateTimeFormatter::calendar_kind()`].
        calendar_algorithm: CalendarAlgorithm
    }
);

prefs_convert!(DateTimeFormatterPreferences, DecimalFormatterPreferences, {
    numbering_system
});

prefs_convert!(DateTimeFormatterPreferences, CalendarPreferences, {
    calendar_algorithm
});

/// Helper macro for generating any/buffer constructors in this file.
macro_rules! gen_buffer_constructors_with_external_loader {
    (@runtime_fset, $fset:ident, $compiled_fn:ident $buffer_fn:ident, $internal_fn:ident) => {
        #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER, Self::$compiled_fn)]
        #[cfg(feature = "serde")]
        pub fn $buffer_fn<P>(
            provider: &P,
            prefs: DateTimeFormatterPreferences,
            field_set: $fset,
        ) -> Result<Self, DateTimeFormatterLoadError>
        where
            P: BufferProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_deserializing(),
                &ExternalLoaderBuffer(provider),
                prefs,
                field_set.get_field(),
            )
        }
    };
    (@compiletime_fset, $fset:ident, $compiled_fn:ident, $buffer_fn:ident, $internal_fn:ident) => {
        #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER, Self::$compiled_fn)]
        #[cfg(feature = "serde")]
        pub fn $buffer_fn<P>(
            provider: &P,
            prefs: DateTimeFormatterPreferences,
            field_set: $fset,
        ) -> Result<Self, DateTimeFormatterLoadError>
        where
            P: BufferProvider + ?Sized,
        {
            Self::$internal_fn(
                &provider.as_deserializing(),
                &ExternalLoaderBuffer(provider),
                prefs,
                field_set.get_field(),
            )
        }
    };
}

size_test!(FixedCalendarDateTimeFormatter<icu_calendar::Gregorian, crate::fieldsets::YMD>, typed_neo_year_month_day_formatter_size, 328);

/// [`FixedCalendarDateTimeFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at compile time.
///
/// For more details, please read the [crate root docs][crate].
#[doc = typed_neo_year_month_day_formatter_size!()]
#[derive(Debug, Clone)]
pub struct FixedCalendarDateTimeFormatter<C: CldrCalendar, FSet: DateTimeNamesMarker> {
    selection: DateTimeZonePatternSelectionData,
    pub(crate) names: RawDateTimeNames<FSet>,
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
    /// This ignores the `calendar_kind` preference and instead uses the static calendar type,
    /// and supports calendars that are not expressible as preferences, such as [`JapaneseExtended`](icu_calendar::cal::JapaneseExtended).
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
    /// use icu::calendar::cal::JapaneseExtended;
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::datetime::input::Date;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     locale!("es-MX").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Date::try_new_iso(2023, 12, 20).unwrap().to_calendar(JapaneseExtended::new())),
    ///     "20 de diciembre de 5 Reiwa"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        prefs: DateTimeFormatterPreferences,
        field_set: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        crate::provider::Baked: AllFixedCalendarFormattingDataMarkers<C, FSet>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            prefs,
            field_set.get_field(),
        )
    }

    gen_buffer_constructors_with_external_loader!(
        @compiletime_fset,
        FSet,
        try_new,
        try_new_with_buffer_provider,
        try_new_internal
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        field_set: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: ?Sized
            + AllFixedCalendarFormattingDataMarkers<C, FSet>
            + AllFixedCalendarExternalDataMarkers,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            prefs,
            field_set.get_field(),
        )
    }
}

impl<C: CldrCalendar, FSet: DateTimeMarkers> FixedCalendarDateTimeFormatter<C, FSet>
where
    FSet::D: TypedDateDataMarkers<C>,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        prefs: DateTimeFormatterPreferences,
        field_set: CompositeFieldSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: ?Sized + AllFixedCalendarFormattingDataMarkers<C, FSet>,
        L: DecimalFormatterLoader,
    {
        let names = RawDateTimeNames::new_without_number_formatting();
        Self::try_new_internal_with_names(provider, provider, loader, prefs, field_set, names)
            .map_err(|e| e.0)
    }

    #[allow(clippy::result_large_err)] // returning ownership of an argument to the caller
    pub(crate) fn try_new_internal_with_names<P0, P1, L>(
        provider_p: &P0,
        provider: &P1,
        loader: &L,
        prefs: DateTimeFormatterPreferences,
        field_set: CompositeFieldSet,
        mut names: RawDateTimeNames<FSet>,
    ) -> Result<Self, (DateTimeFormatterLoadError, RawDateTimeNames<FSet>)>
    where
        P0: ?Sized + AllFixedCalendarPatternDataMarkers<C, FSet>,
        P1: ?Sized + AllFixedCalendarFormattingDataMarkers<C, FSet>,
        L: DecimalFormatterLoader,
    {
        let selection = DateTimeZonePatternSelectionData::try_new_with_skeleton(
            &<FSet::D as TypedDateDataMarkers<C>>::DateSkeletonPatternsV1::bind(provider_p),
            &<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1::bind(provider_p),
            &FSet::GluePatternV1::bind(provider_p),
            prefs,
            field_set,
        );
        let selection = match selection {
            Ok(selection) => selection,
            Err(e) => return Err((DateTimeFormatterLoadError::Data(e), names)),
        };
        let result = names.load_for_pattern(
            &<FSet::D as TypedDateDataMarkers<C>>::YearNamesV1::bind(provider),
            &<FSet::D as TypedDateDataMarkers<C>>::MonthNamesV1::bind(provider),
            &<FSet::D as TypedDateDataMarkers<C>>::WeekdayNamesV1::bind(provider),
            &<FSet::T as TimeMarkers>::DayPeriodNamesV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::EssentialsV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::LocationsV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::LocationsRootV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::ExemplarCitiesV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::ExemplarCitiesRootV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::GenericLongV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::GenericShortV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::StandardLongV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::SpecificLongV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::SpecificShortV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::MetazonePeriodV1::bind(provider),
            loader, // fixed decimal formatter
            prefs,
            selection.pattern_items_for_data_loading(),
        );
        match result {
            Ok(()) => (),
            Err(e) => return Err((DateTimeFormatterLoadError::Names(e), names)),
        };
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
    /// use icu::datetime::input::Date;
    /// use icu::datetime::input::cal::Buddhist;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::locale::locale;
    ///
    /// let formatter =
    ///     FixedCalendarDateTimeFormatter::<Buddhist, _>::try_new(
    ///         locale!("es-MX").into(),
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
    /// ```compile_fail,E0277
    /// use icu::datetime::input::Time;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::locale::locale;
    ///
    /// let formatter =
    ///     FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
    ///         locale!("es-MX").into(),
    ///         YMD::long(),
    ///     )
    ///     .unwrap();
    ///
    /// // error[E0277]: the trait bound `Time: AllInputMarkers<fieldsets::YMD>` is not satisfied
    /// formatter.format(&Time::midnight());
    /// ```
    pub fn format<I>(&self, input: &I) -> FormattedDateTime
    where
        I: ?Sized + InFixedCalendar<C> + AllInputMarkers<FSet>,
    {
        let input =
            DateTimeInputUnchecked::extract_from_neo_input::<FSet::D, FSet::T, FSet::Z, I>(input);
        FormattedDateTime {
            pattern: self.selection.select(&input),
            input,
            names: self.names.as_borrowed(),
        }
    }
}

impl<C: CldrCalendar, FSet: DateTimeNamesMarker> FixedCalendarDateTimeFormatter<C, FSet> {
    /// Formats a datetime without enforcing either the field set or the calendar.
    ///
    /// This function is useful when the caller knows something about the field set that the
    /// type system is unaware of. For example, if the formatter is represented with a
    /// [dynamic field set](crate::fieldsets::enums), the caller may be able to provide a
    /// narrower type for formatting.
    ///
    /// ❗ The caller must ensure that:
    ///
    /// 1. The calendar of the input matches the calendar of the formatter
    /// 2. The fields of the input are a superset of the fields of the formatter
    ///
    /// Returns a [`FormattedDateTimeUnchecked`] to surface errors when they occur,
    /// but not every invariant will result in an error. Use with caution!
    ///
    /// # Examples
    ///
    /// In the following example, we know that the formatter's field set is [`YMD`], but the
    /// type system thinks we are a [`CompositeFieldSet`], which requires a [`ZonedDateTime`]
    /// as input. However, since [`Date`] contains all the fields required by [`YMD`], we can
    /// successfully pass it into [`format_unchecked`].
    ///
    /// ```
    /// use icu::calendar::cal::Buddhist;
    /// use icu::datetime::fieldsets::{T, YMD};
    /// use icu::datetime::fieldsets::enums::CompositeFieldSet;
    /// use icu::datetime::input::{Date, Time};
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::DateTimeWriteError;
    /// use icu::datetime::DateTimeInputUnchecked;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::<Buddhist, _>::try_new(
    ///     locale!("th").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap()
    /// .cast_into_fset::<CompositeFieldSet>();
    ///
    /// // Create a date and convert it to the correct calendar:
    /// let date = Date::try_new_iso(2025, 3, 7).unwrap().to_calendar(Buddhist);
    ///
    /// // Extract the fields and use it with format_unchecked:
    /// let mut input = DateTimeInputUnchecked::default();
    /// input.set_date_fields(date);
    /// let result = formatter.format_unchecked(input);
    ///
    /// assert_try_writeable_eq!(result, "7 มีนาคม 2568");
    ///
    /// // If we don't give all needed fields, we will get an error!
    /// let mut input = DateTimeInputUnchecked::default();
    /// let result = formatter.format_unchecked(input);
    /// assert_try_writeable_eq!(
    ///     result,
    ///     "{d} {M} {G} {y}",
    ///     Err(DateTimeWriteError::MissingInputField("day_of_month"))
    /// );
    /// ```
    ///
    /// [`Date`]: crate::input::Date
    /// [`ZonedDateTime`]: crate::input::ZonedDateTime
    /// [`YMD`]: crate::fieldsets::YMD
    /// [`format_unchecked`]: Self::format_unchecked
    pub fn format_unchecked(&self, datetime: DateTimeInputUnchecked) -> FormattedDateTimeUnchecked {
        FormattedDateTimeUnchecked {
            pattern: self.selection.select(&datetime),
            input: datetime,
            names: self.names.as_borrowed(),
        }
    }
}

size_test!(
    DateTimeFormatter<crate::fieldsets::YMD>,
    neo_year_month_day_formatter_size,
    384
);

/// [`DateTimeFormatter`] is a formatter capable of formatting dates and/or times from
/// a calendar selected at runtime.
///
/// For more details, please read the [crate root docs][crate].
#[doc = neo_year_month_day_formatter_size!()]
#[derive(Debug, Clone)]
pub struct DateTimeFormatter<FSet: DateTimeNamesMarker> {
    selection: DateTimeZonePatternSelectionData,
    pub(crate) names: RawDateTimeNames<FSet>,
    pub(crate) calendar: UntaggedFormattableAnyCalendar,
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
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::datetime::input::Date;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("en-u-ca-hebrew").into(),
    ///     YMD::medium(),
    /// )
    /// .unwrap();
    ///
    /// let date = Date::try_new_iso(2024, 5, 8).unwrap();
    ///
    /// assert_writeable_eq!(formatter.format(&date), "30 Nisan 5784");
    /// ```
    ///
    /// [`AnyCalendarKind`]: icu_calendar::AnyCalendarKind
    #[inline(never)]
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        prefs: DateTimeFormatterPreferences,
        field_set: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        crate::provider::Baked: AllAnyCalendarFormattingDataMarkers<FSet>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            prefs,
            field_set.get_field(),
        )
    }

    gen_buffer_constructors_with_external_loader!(
        @compiletime_fset,
        FSet,
        try_new,
        try_new_with_buffer_provider,
        try_new_internal
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        field_set: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: ?Sized + AllAnyCalendarFormattingDataMarkers<FSet> + AllAnyCalendarExternalDataMarkers,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            prefs,
            field_set.get_field(),
        )
    }
}

impl<FSet: DateTimeMarkers> DateTimeFormatter<FSet>
where
    FSet::D: DateDataMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    fn try_new_internal<P, L>(
        provider: &P,
        loader: &L,
        prefs: DateTimeFormatterPreferences,
        field_set: CompositeFieldSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: ?Sized + AllAnyCalendarFormattingDataMarkers<FSet>,
        L: DecimalFormatterLoader + FormattableAnyCalendarLoader,
    {
        let kind = FormattableAnyCalendarKind::from_preferences(prefs);
        let calendar = FormattableAnyCalendarLoader::load(loader, kind)?;
        let names = RawDateTimeNames::new_without_number_formatting();
        Self::try_new_internal_with_calendar_and_names(
            provider, provider, loader, prefs, field_set, calendar, names,
        )
        .map_err(|e| e.0)
    }

    #[allow(clippy::result_large_err)] // returning ownership of an argument to the caller
    pub(crate) fn try_new_internal_with_calendar_and_names<P0, P1, L>(
        provider_p: &P0,
        provider: &P1,
        loader: &L,
        prefs: DateTimeFormatterPreferences,
        field_set: CompositeFieldSet,
        calendar: FormattableAnyCalendar,
        mut names: RawDateTimeNames<FSet>,
    ) -> Result<
        Self,
        (
            DateTimeFormatterLoadError,
            (FormattableAnyCalendar, RawDateTimeNames<FSet>),
        ),
    >
    where
        P0: ?Sized + AllAnyCalendarPatternDataMarkers<FSet>,
        P1: ?Sized + AllAnyCalendarFormattingDataMarkers<FSet>,
        L: DecimalFormatterLoader,
    {
        let selection = DateTimeZonePatternSelectionData::try_new_with_skeleton(
            &FormattableAnyCalendarNamesLoader::<<FSet::D as DateDataMarkers>::Skel, _>::new(
                provider_p,
                calendar.kind(),
            ),
            &<FSet::T as TimeMarkers>::TimeSkeletonPatternsV1::bind(provider_p),
            &FSet::GluePatternV1::bind(provider_p),
            prefs,
            field_set,
        );
        let selection = match selection {
            Ok(selection) => selection,
            Err(e) => return Err((DateTimeFormatterLoadError::Data(e), (calendar, names))),
        };
        let result = names.load_for_pattern(
            &FormattableAnyCalendarNamesLoader::<<FSet::D as DateDataMarkers>::Year, _>::new(
                provider,
                calendar.kind(),
            ),
            &FormattableAnyCalendarNamesLoader::<<FSet::D as DateDataMarkers>::Month, _>::new(
                provider,
                calendar.kind(),
            ),
            &<FSet::D as DateDataMarkers>::WeekdayNamesV1::bind(provider),
            &<FSet::T as TimeMarkers>::DayPeriodNamesV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::EssentialsV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::LocationsV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::LocationsRootV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::ExemplarCitiesRootV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::ExemplarCitiesV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::GenericLongV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::GenericShortV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::StandardLongV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::SpecificLongV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::SpecificShortV1::bind(provider),
            &<FSet::Z as ZoneMarkers>::MetazonePeriodV1::bind(provider),
            loader, // fixed decimal formatter
            prefs,
            selection.pattern_items_for_data_loading(),
        );
        match result {
            Ok(()) => (),
            Err(e) => return Err((DateTimeFormatterLoadError::Names(e), (calendar, names))),
        };
        Ok(Self {
            selection,
            names,
            calendar: calendar.into_untagged(),
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
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::datetime::input::Date;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::MismatchedCalendarError;
    /// use icu::locale::locale;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("en-u-ca-hebrew").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap();
    ///
    /// let date = Date::try_new_gregorian(2023, 12, 20).unwrap();
    ///
    /// assert!(matches!(
    ///     formatter.format_same_calendar(&date),
    ///     Err(MismatchedCalendarError { .. })
    /// ));
    /// ```
    ///
    /// A time cannot be passed into the formatter when a date is expected:
    ///
    /// ```compile_fail,E0277
    /// use icu::datetime::input::Time;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::locale::locale;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("es-MX").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap();
    ///
    /// // error[E0277]: the trait bound `Time: AllInputMarkers<fieldsets::YMD>` is not satisfied
    /// formatter.format_same_calendar(&Time::midnight());
    /// ```
    pub fn format_same_calendar<I>(
        &self,
        datetime: &I,
    ) -> Result<FormattedDateTime, crate::MismatchedCalendarError>
    where
        I: ?Sized + InSameCalendar + AllInputMarkers<FSet>,
    {
        datetime.check_any_calendar_kind(self.calendar.any_calendar().kind())?;
        let datetime = DateTimeInputUnchecked::extract_from_neo_input::<FSet::D, FSet::T, FSet::Z, I>(
            datetime,
        );
        Ok(FormattedDateTime {
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
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::datetime::input::Date;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::MismatchedCalendarError;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("en-u-ca-hebrew").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap();
    ///
    /// let date = Date::try_new_roc(113, 5, 8).unwrap();
    ///
    /// assert_writeable_eq!(formatter.format(&date), "30 Nisan 5784");
    /// ```
    ///
    /// A time cannot be passed into the formatter when a date is expected:
    ///
    /// ```compile_fail,E0277
    /// use icu::datetime::input::Time;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::locale::locale;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("es-MX").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap();
    ///
    /// // error[E0277]: the trait bound `Time: AllInputMarkers<fieldsets::YMD>` is not satisfied
    /// formatter.format(&Time::midnight());
    /// ```
    pub fn format<'a, I>(&'a self, datetime: &I) -> FormattedDateTime<'a>
    where
        I: ?Sized + ConvertCalendar,
        I::Converted<'a>: Sized + AllInputMarkers<FSet>,
    {
        let datetime = datetime.to_calendar(self.calendar.any_calendar());
        let datetime = DateTimeInputUnchecked::extract_from_neo_input::<
            FSet::D,
            FSet::T,
            FSet::Z,
            I::Converted<'a>,
        >(&datetime);
        FormattedDateTime {
            pattern: self.selection.select(&datetime),
            input: datetime,
            names: self.names.as_borrowed(),
        }
    }
}

impl<FSet: DateTimeNamesMarker> DateTimeFormatter<FSet> {
    /// Formats a datetime without enforcing either the field set or the calendar.
    ///
    /// This function is useful when the caller knows something about the field set that the
    /// type system is unaware of. For example, if the formatter is represented with a
    /// [dynamic field set](crate::fieldsets::enums), the caller may be able to provide a
    /// narrower type for formatting.
    ///
    /// ❗ The caller must ensure that:
    ///
    /// 1. The calendar of the input matches the calendar of the formatter
    /// 2. The fields of the input are a superset of the fields of the formatter
    ///
    /// Returns a [`FormattedDateTimeUnchecked`] to surface errors when they occur,
    /// but not every invariant will result in an error. Use with caution!
    ///
    /// # Examples
    ///
    /// In the following example, we know that the formatter's field set is [`YMD`], but the
    /// type system thinks we are a [`CompositeFieldSet`], which requires a [`ZonedDateTime`]
    /// as input. However, since [`Date`] contains all the fields required by [`YMD`], we can
    /// successfully pass it into [`format_unchecked`].
    ///
    /// ```
    /// use icu::datetime::fieldsets::{T, YMD};
    /// use icu::datetime::fieldsets::enums::CompositeFieldSet;
    /// use icu::datetime::input::{Date, Time};
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::DateTimeWriteError;
    /// use icu::datetime::DateTimeInputUnchecked;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("th-TH").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap()
    /// .cast_into_fset::<CompositeFieldSet>();
    ///
    /// // Create a date and convert it to the correct calendar:
    /// let date = Date::try_new_iso(2025, 3, 7).unwrap().to_calendar(formatter.calendar());
    ///
    /// // Extract the fields and use it with format_unchecked:
    /// let mut input = DateTimeInputUnchecked::default();
    /// input.set_date_fields(date);
    /// let result = formatter.format_unchecked(input);
    ///
    /// assert_try_writeable_eq!(result, "7 มีนาคม 2568");
    ///
    /// // If we don't give all needed fields, we will get an error!
    /// let mut input = DateTimeInputUnchecked::default();
    /// let result = formatter.format_unchecked(input);
    /// assert_try_writeable_eq!(
    ///     result,
    ///     "{d} {M} {G} {y}",
    ///     Err(DateTimeWriteError::MissingInputField("day_of_month"))
    /// );
    /// ```
    ///
    /// [`Date`]: crate::input::Date
    /// [`ZonedDateTime`]: crate::input::ZonedDateTime
    /// [`YMD`]: crate::fieldsets::YMD
    /// [`format_unchecked`]: Self::format_unchecked
    pub fn format_unchecked(&self, datetime: DateTimeInputUnchecked) -> FormattedDateTimeUnchecked {
        FormattedDateTimeUnchecked {
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
    /// [`DateTimeFormatter`] does not necesarily support all calendars that are supported by
    /// [`FixedCalendarDateTimeFormatter`], which is why this function can fail.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::cal::Hebrew;
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::datetime::input::Date;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     locale!("en").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap()
    /// .into_formatter(Hebrew::new());
    ///
    /// let date = Date::try_new_iso(2024, 10, 14).unwrap();
    ///
    /// assert_writeable_eq!(formatter.format(&date), "12 Tishri 5785");
    /// ```
    pub fn into_formatter(self, calendar: C) -> DateTimeFormatter<FSet>
    where
        C: IntoFormattableAnyCalendar,
    {
        DateTimeFormatter {
            selection: self.selection,
            names: self.names,
            calendar: FormattableAnyCalendar::from_calendar(calendar).into_untagged(),
        }
    }

    /// Maps a [`FixedCalendarDateTimeFormatter`] of a specific `FSet` to a more general `FSet`.
    ///
    /// For example, this can transform a formatter for [`YMD`] to one for [`DateFieldSet`].
    ///
    /// [`YMD`]: crate::fieldsets::YMD
    /// [`DateFieldSet`]: crate::fieldsets::enums::DateFieldSet
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fieldsets::{enums::DateFieldSet, YMD};
    /// use icu::datetime::input::Date;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let specific_formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     locale!("fr").into(),
    ///     YMD::medium(),
    /// )
    /// .unwrap();
    ///
    /// // Test that the specific formatter works:
    /// let date = Date::try_new_gregorian(2024, 12, 20).unwrap();
    /// assert_writeable_eq!(specific_formatter.format(&date), "20 déc. 2024");
    ///
    /// // Make a more general formatter:
    /// let general_formatter = specific_formatter.cast_into_fset::<DateFieldSet>();
    ///
    /// // Test that it still works:
    /// assert_writeable_eq!(general_formatter.format(&date), "20 déc. 2024");
    /// ```
    pub fn cast_into_fset<FSet2: DateTimeNamesFrom<FSet>>(
        self,
    ) -> FixedCalendarDateTimeFormatter<C, FSet2> {
        FixedCalendarDateTimeFormatter {
            selection: self.selection,
            names: self.names.cast_into_fset(),
            _calendar: PhantomData,
        }
    }

    /// Gets a [`FieldSetBuilder`] corresponding to the fields and options configured in this
    /// formatter. The builder can be used to recreate the formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::YMDT;
    /// use icu::datetime::input::*;
    /// use icu::datetime::options::*;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// // Create a simple YMDT formatter:
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     locale!("und").into(),
    ///     YMDT::long().hm().with_alignment(Alignment::Column)
    /// )
    /// .unwrap();
    ///
    /// // Get the builder corresponding to it:
    /// let builder = formatter.to_field_set_builder();
    ///
    /// // Check that the builder is what we expect:
    /// let mut equivalent_builder = FieldSetBuilder::default();
    /// equivalent_builder.length = Some(Length::Long);
    /// equivalent_builder.date_fields = Some(DateFields::YMD);
    /// equivalent_builder.time_precision = Some(TimePrecision::Minute);
    /// equivalent_builder.alignment = Some(Alignment::Column);
    /// equivalent_builder.year_style = None;
    /// assert_eq!(
    ///     builder,
    ///     equivalent_builder,
    /// );
    ///
    /// // Check that it creates a formatter with equivalent behavior:
    /// let built_formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     locale!("und").into(),
    ///     builder.build_composite_datetime().unwrap(),
    /// )
    /// .unwrap();
    /// let datetime = DateTime {
    ///     date: Date::try_new_gregorian(2025, 3, 6).unwrap(),
    ///     time: Time::try_new(16, 41, 0, 0).unwrap(),
    /// };
    /// assert_eq!(
    ///     formatter.format(&datetime).to_string(),
    ///     built_formatter.format(&datetime).to_string(),
    /// );
    /// ```
    pub fn to_field_set_builder(&self) -> FieldSetBuilder {
        self.selection.to_builder()
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
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::datetime::input::Date;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("en-u-ca-hebrew").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap()
    /// .try_into_typed_formatter::<Hebrew>()
    /// .unwrap();
    ///
    /// let date = Date::try_new_hebrew(5785, 1, 12).unwrap();
    ///
    /// assert_writeable_eq!(formatter.format(&date), "12 Tishri 5785");
    /// ```
    ///
    /// An error occurs if the calendars don't match:
    ///
    /// ```
    /// use icu::calendar::cal::Hebrew;
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::datetime::input::Date;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::MismatchedCalendarError;
    /// use icu::locale::locale;
    ///
    /// let result = DateTimeFormatter::try_new(
    ///     locale!("en-u-ca-buddhist").into(),
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
        if let Err(cal) = C::from_any(self.calendar.take_any_calendar()) {
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

    /// Maps a [`DateTimeFormatter`] of a specific `FSet` to a more general `FSet`.
    ///
    /// For example, this can transform a formatter for [`YMD`] to one for [`DateFieldSet`].
    ///
    /// [`YMD`]: crate::fieldsets::YMD
    /// [`DateFieldSet`]: crate::fieldsets::enums::DateFieldSet
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fieldsets::{enums::DateFieldSet, YMD};
    /// use icu::datetime::input::Date;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let specific_formatter =
    ///     DateTimeFormatter::try_new(locale!("fr").into(), YMD::medium())
    ///         .unwrap();
    ///
    /// // Test that the specific formatter works:
    /// let date = Date::try_new_gregorian(2024, 12, 20).unwrap();
    /// assert_writeable_eq!(specific_formatter.format(&date), "20 déc. 2024");
    ///
    /// // Make a more general formatter:
    /// let general_formatter = specific_formatter.cast_into_fset::<DateFieldSet>();
    ///
    /// // Test that it still works:
    /// assert_writeable_eq!(general_formatter.format(&date), "20 déc. 2024");
    /// ```
    pub fn cast_into_fset<FSet2: DateTimeNamesFrom<FSet>>(self) -> DateTimeFormatter<FSet2> {
        DateTimeFormatter {
            selection: self.selection,
            names: self.names.cast_into_fset(),
            calendar: self.calendar,
        }
    }

    /// Returns the calendar used in this formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::AnyCalendarKind;
    /// use icu::datetime::fieldsets::YMD;
    /// use icu::datetime::input::Date;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     DateTimeFormatter::try_new(locale!("th-TH").into(), YMD::long()).unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Date::try_new_iso(2024, 12, 16).unwrap()),
    ///     "16 ธันวาคม 2567"
    /// );
    ///
    /// assert_eq!(formatter.calendar().kind(), AnyCalendarKind::Buddhist);
    /// ```
    pub fn calendar(&self) -> icu_calendar::Ref<AnyCalendar> {
        icu_calendar::Ref(self.calendar.any_calendar())
    }

    /// Gets a [`FieldSetBuilder`] corresponding to the fields and options configured in this
    /// formatter. The builder can be used to recreate the formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::YMDT;
    /// use icu::datetime::input::*;
    /// use icu::datetime::options::*;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// // Create a simple YMDT formatter:
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("und").into(),
    ///     YMDT::long().with_alignment(Alignment::Column)
    /// )
    /// .unwrap();
    ///
    /// // Get the builder corresponding to it:
    /// let builder = formatter.to_field_set_builder();
    ///
    /// // Check that the builder is what we expect:
    /// let mut equivalent_builder = FieldSetBuilder::default();
    /// equivalent_builder.length = Some(Length::Long);
    /// equivalent_builder.date_fields = Some(DateFields::YMD);
    /// equivalent_builder.time_precision = Some(TimePrecision::Second); // set automatically
    /// equivalent_builder.alignment = Some(Alignment::Column);
    /// equivalent_builder.year_style = None;
    /// assert_eq!(
    ///     builder,
    ///     equivalent_builder,
    /// );
    ///
    /// // Check that it creates a formatter with equivalent behavior:
    /// let built_formatter = DateTimeFormatter::try_new(
    ///     locale!("und").into(),
    ///     builder.build_composite_datetime().unwrap(),
    /// )
    /// .unwrap();
    /// let datetime = DateTime {
    ///     date: Date::try_new_iso(2025, 3, 6).unwrap(),
    ///     time: Time::try_new(16, 41, 0, 0).unwrap(),
    /// };
    /// assert_eq!(
    ///     formatter.format(&datetime).to_string(),
    ///     built_formatter.format(&datetime).to_string(),
    /// );
    /// ```
    pub fn to_field_set_builder(&self) -> FieldSetBuilder {
        self.selection.to_builder()
    }
}

/// A formatter optimized for time and time zone formatting, when a calendar is not needed.
///
/// # Examples
///
/// A [`NoCalendarFormatter`] cannot be constructed with a fieldset that involves dates:
///
/// ```
/// use icu::datetime::fieldsets::Y;
/// use icu::datetime::NoCalendarFormatter;
/// use icu::locale::locale;
///
/// assert!(
///     NoCalendarFormatter::try_new(locale!("und").into(), Y::medium())
///         .is_err()
/// );
/// ```
///
/// Furthermore, it is a compile error in the format function:
///
/// ```compile_fail,E0271
/// use icu::datetime::NoCalendarFormatter;
/// use icu::datetime::fieldsets::Y;
/// use icu::locale::locale;
///
/// let date: icu::calendar::Date<icu::calendar::Gregorian> = unimplemented!();
/// let formatter = NoCalendarFormatter::try_new(locale!("und").into(), Y::medium()).unwrap();
///
/// // error[E0271]: type mismatch resolving `<Gregorian as AsCalendar>::Calendar == ()`
/// formatter.format(&date);
/// ```
pub type NoCalendarFormatter<FSet> = FixedCalendarDateTimeFormatter<(), FSet>;

/// An intermediate type during a datetime formatting operation.
///
/// Not intended to be stored: convert to a string first.
#[derive(Debug)]
pub struct FormattedDateTime<'a> {
    pattern: DateTimeZonePatternDataBorrowed<'a>,
    input: DateTimeInputUnchecked,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl Writeable for FormattedDateTime<'_> {
    fn write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<(), fmt::Error> {
        let result = try_write_pattern_items(
            self.pattern.metadata(),
            self.pattern.iter_items(),
            &self.input,
            &self.names,
            self.names.decimal_formatter,
            sink,
        );
        // A DateTimeWriteError should not occur in normal usage because DateTimeFormatter
        // guarantees that all names for the pattern have been loaded and that the input type
        // is compatible with the pattern. However, this code path might be reachable with
        // invalid data. In that case, debug-panic and return the fallback string.
        match result {
            Ok(Ok(())) => Ok(()),
            Err(fmt::Error) => Err(fmt::Error),
            Ok(Err(e)) => {
                debug_assert!(false, "unexpected error in FormattedDateTime: {e:?}");
                Ok(())
            }
        }
    }

    // TODO(#489): Implement writeable_length_hint
}

impl_display_with_writeable!(FormattedDateTime<'_>);

impl FormattedDateTime<'_> {
    /// Gets the pattern used in this formatted value.
    ///
    /// From the pattern, one can check the properties of the included components, such as
    /// the hour cycle being used for formatting. See [`DateTimePattern`].
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}

/// An intermediate type during a datetime formatting operation with dynamic input.
///
/// Unlike [`FormattedDateTime`], converting this to a string could fail.
///
/// Not intended to be stored: convert to a string first.
#[derive(Debug)]
pub struct FormattedDateTimeUnchecked<'a> {
    pattern: DateTimeZonePatternDataBorrowed<'a>,
    input: DateTimeInputUnchecked,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl TryWriteable for FormattedDateTimeUnchecked<'_> {
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
            self.names.decimal_formatter,
            sink,
        )
    }

    // TODO(#489): Implement writeable_length_hint
}

impl FormattedDateTimeUnchecked<'_> {
    /// Gets the pattern used in this formatted value.
    ///
    /// From the pattern, one can check the properties of the included components, such as
    /// the hour cycle being used for formatting. See [`DateTimePattern`].
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}
