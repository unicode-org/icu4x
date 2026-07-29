// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::DateTimeFormatter;
use crate::DateTimeFormatterLoadError;
use crate::DateTimeFormatterPreferences;
use crate::FixedCalendarDateTimeFormatter;
#[cfg(feature = "compiled_data")]
use crate::external_loaders::ExternalLoaderCompiledData;
use crate::external_loaders::{
    DecimalFormatterLoader, ExternalLoaderUnstable, FormattableAnyCalendarLoader,
};
use crate::fieldsets::enums::CompositeFieldSet;
use crate::format::DateTimeInputUnchecked;
use crate::provider::range_patterns::DatetimePatternsRangeGlueV1;
use crate::range::formatter_impl::format_impl_shared;
use crate::range::write::FormattedDateRange;
use crate::raw::neo::DateTimeZoneRangePatternSelectionData;
use crate::scaffold::{
    AllAnyCalendarExternalDataMarkers, AllAnyCalendarFormattingDataMarkers,
    AllAnyCalendarRangePatternDataMarkers, AllFixedCalendarExternalDataMarkers,
    AllFixedCalendarFormattingDataMarkers, AllFixedCalendarRangePatternDataMarkers,
    AllInputMarkers, CldrCalendar, ConvertCalendar, DateDataMarkers, DateInputMarkers,
    DateTimeMarkers, DateTimeNamesMarker, FormattableAnyCalendarNamesLoader, GetField,
    InFixedCalendar, TimeMarkers, TypedDateDataMarkers, ZoneMarkers,
};
use icu_provider::prelude::*;

/// A formatter capable of formatting date/time ranges for any calendar.
///
/// This formatter is calendar-agnostic. It dynamically converts the input dates to the
/// calendar preferred by the locale before formatting.
///
/// # Examples
///
/// ```
/// use icu::calendar::Date;
/// use icu::datetime::input::{DateTime, Time};
/// use icu::datetime::fieldsets::YMD;
/// use icu::datetime::DateRangeFormatter;
/// use icu::locale::locale;
/// use writeable::assert_writeable_eq;
///
/// let fmt = DateRangeFormatter::try_new(
///     locale!("th-u-ca-buddhist").into(),
///     YMD::medium(),
/// )
/// .unwrap();
///
/// let start = DateTime {
///     date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
///     time: Time::try_new(9, 0, 0, 0).unwrap(),
/// };
/// let end = DateTime {
///     date: Date::try_new_gregorian(2023, 12, 23).unwrap(),
///     time: Time::try_new(17, 0, 0, 0).unwrap(),
/// };
///
/// // Gregorian input is dynamically converted to Buddhist (2023 -> 2566)
/// // Thai day-difference range has no spaces around en-dash:
/// assert_writeable_eq!(
///     fmt.format(&start, &end),
///     "22–23 ธ.ค. 2566"
/// );
/// ```
#[derive(Debug)]
pub struct DateRangeFormatter<FSet: DateTimeNamesMarker> {
    pub(crate) datetime_formatter: DateTimeFormatter<FSet>,
    pub(crate) range_selection: DateTimeZoneRangePatternSelectionData,
}

impl<FSet: DateTimeMarkers + Clone> DateRangeFormatter<FSet>
where
    FSet::D: DateDataMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<CompositeFieldSet>,
{
    pub(crate) fn try_new_internal<P>(
        provider: &P,
        external_loader: &(impl FormattableAnyCalendarLoader + DecimalFormatterLoader),
        prefs: DateTimeFormatterPreferences,
        field_set_with_options: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: ?Sized
            + AllAnyCalendarFormattingDataMarkers<FSet>
            + AllAnyCalendarRangePatternDataMarkers<FSet>,
    {
        let field_set_with_options_clone = field_set_with_options.clone();
        let datetime_formatter = DateTimeFormatter::try_new_internal(
            provider,
            external_loader,
            prefs,
            field_set_with_options_clone.get_field(),
        )?;

        let range_selection = DateTimeZoneRangePatternSelectionData::try_new_with_skeleton(
            &FormattableAnyCalendarNamesLoader::<<FSet::D as DateDataMarkers>::RangeSkel, _>::new(
                provider,
                &datetime_formatter.calendar,
            ),
            &<FSet::T as TimeMarkers>::TimeRangeSkeletonPatternsV1::bind(provider),
            &DatetimePatternsRangeGlueV1::bind(provider),
            prefs,
            field_set_with_options.get_field(),
        )
        .map_err(DateTimeFormatterLoadError::Data)?;

        Ok(Self {
            datetime_formatter,
            range_selection,
        })
    }

    /// Constructor taking a data provider.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    pub fn try_new_unstable<P>(
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        field_set_with_options: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: ?Sized
            + AllAnyCalendarFormattingDataMarkers<FSet>
            + AllAnyCalendarExternalDataMarkers
            + AllAnyCalendarRangePatternDataMarkers<FSet>,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            prefs,
            field_set_with_options,
        )
    }

    #[cfg(feature = "compiled_data")]
    /// Constructor using compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    pub fn try_new(
        prefs: DateTimeFormatterPreferences,
        field_set_with_options: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        crate::provider::Baked:
            AllAnyCalendarFormattingDataMarkers<FSet> + AllAnyCalendarRangePatternDataMarkers<FSet>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            prefs,
            field_set_with_options,
        )
    }

    #[cfg(feature = "serde")]
    /// Constructor taking a buffer provider.
    ///
    /// ✨ *Enabled with the `serde` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    pub fn try_new_with_buffer_provider<P>(
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        field_set_with_options: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: BufferProvider + ?Sized,
    {
        use crate::provider::compat::CompatProvider;
        let deser_provider = provider.as_deserializing();
        let compat_provider = CompatProvider(&deser_provider, provider);
        Self::try_new_unstable(&compat_provider, prefs, field_set_with_options)
    }
}

impl<FSet: DateTimeMarkers + DateTimeNamesMarker> DateRangeFormatter<FSet>
where
    FSet::D: DateInputMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    /// Formats a date/time range.
    ///
    /// This method converts the input dates dynamically to the calendar preferred by the
    /// locale before formatting them as a range.
    pub fn format<'a, I>(&'a self, start: &I, end: &I) -> FormattedDateRange<'a>
    where
        I: ?Sized + ConvertCalendar,
        I::Converted<'a>: Sized + AllInputMarkers<FSet>,
    {
        let start_cal = start.to_calendar(self.datetime_formatter.calendar.any_calendar());
        let end_cal = end.to_calendar(self.datetime_formatter.calendar.any_calendar());

        let start_input = DateTimeInputUnchecked::extract_from_neo_input::<
            FSet::D,
            FSet::T,
            FSet::Z,
            I::Converted<'a>,
        >(&start_cal);
        let end_input = DateTimeInputUnchecked::extract_from_neo_input::<
            FSet::D,
            FSet::T,
            FSet::Z,
            I::Converted<'a>,
        >(&end_cal);

        format_impl_shared(
            &self.datetime_formatter,
            &self.range_selection,
            &start_input,
            &end_input,
        )
    }
}

/// A formatter capable of formatting date/time ranges for a specific calendar.
///
/// This formatter is statically typed to a specific calendar `C`. It is highly optimized
/// and avoids the overhead of dynamic calendar conversion, but can only format dates
/// that are in the calendar `C` (or can be statically converted to it).
///
/// # Examples
///
/// ```
/// use icu::calendar::Date;
/// use icu::datetime::input::{DateTime, Time};
/// use icu::datetime::fieldsets::YMD;
/// use icu::datetime::FixedCalendarDateRangeFormatter;
/// use icu::locale::locale;
/// use writeable::assert_writeable_eq;
///
/// let fmt = FixedCalendarDateRangeFormatter::try_new(
///     locale!("en").into(),
///     YMD::medium(),
/// )
/// .unwrap();
///
/// let start = DateTime {
///     date: Date::try_new_gregorian(2023, 12, 22).unwrap(),
///     time: Time::try_new(9, 0, 0, 0).unwrap(),
/// };
/// let end = DateTime {
///     date: Date::try_new_gregorian(2023, 12, 23).unwrap(),
///     time: Time::try_new(17, 0, 0, 0).unwrap(),
/// };
///
/// // English medium YMD day-difference range uses thin spaces (\u{2009}) around en-dash:
/// assert_writeable_eq!(
///     fmt.format(&start, &end),
///     "Dec 22\u{2009}–\u{2009}23, 2023"
/// );
/// ```
#[derive(Debug)]
pub struct FixedCalendarDateRangeFormatter<C: CldrCalendar, FSet: DateTimeNamesMarker> {
    pub(crate) datetime_formatter: FixedCalendarDateTimeFormatter<C, FSet>,
    pub(crate) range_selection: DateTimeZoneRangePatternSelectionData,
}

impl<C: CldrCalendar, FSet: DateTimeMarkers + Clone> FixedCalendarDateRangeFormatter<C, FSet>
where
    FSet::D: TypedDateDataMarkers<C>,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
    FSet: GetField<CompositeFieldSet>,
{
    pub(crate) fn try_new_internal<P>(
        provider: &P,
        external_loader: &impl DecimalFormatterLoader,
        prefs: DateTimeFormatterPreferences,
        field_set_with_options: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: ?Sized
            + AllFixedCalendarFormattingDataMarkers<C, FSet>
            + AllFixedCalendarRangePatternDataMarkers<C, FSet>,
    {
        let field_set_with_options_clone = field_set_with_options.clone();
        let datetime_formatter = FixedCalendarDateTimeFormatter::try_new_internal(
            provider,
            external_loader,
            prefs,
            field_set_with_options_clone.get_field(),
        )?;

        let range_selection = DateTimeZoneRangePatternSelectionData::try_new_with_skeleton(
            &<FSet::D as TypedDateDataMarkers<C>>::DateRangeSkeletonPatternsV1::bind(provider),
            &<FSet::T as TimeMarkers>::TimeRangeSkeletonPatternsV1::bind(provider),
            &DatetimePatternsRangeGlueV1::bind(provider),
            prefs,
            field_set_with_options.get_field(),
        )
        .map_err(DateTimeFormatterLoadError::Data)?;

        Ok(Self {
            datetime_formatter,
            range_selection,
        })
    }

    /// Constructor taking a data provider.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    pub fn try_new_unstable<P>(
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        field_set_with_options: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: ?Sized
            + AllFixedCalendarFormattingDataMarkers<C, FSet>
            + AllFixedCalendarExternalDataMarkers
            + AllFixedCalendarRangePatternDataMarkers<C, FSet>,
    {
        Self::try_new_internal(
            provider,
            &ExternalLoaderUnstable(provider),
            prefs,
            field_set_with_options,
        )
    }

    #[cfg(feature = "compiled_data")]
    /// Constructor using compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    pub fn try_new(
        prefs: DateTimeFormatterPreferences,
        field_set_with_options: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        crate::provider::Baked: AllFixedCalendarFormattingDataMarkers<C, FSet>
            + AllFixedCalendarRangePatternDataMarkers<C, FSet>,
    {
        Self::try_new_internal(
            &crate::provider::Baked,
            &ExternalLoaderCompiledData,
            prefs,
            field_set_with_options,
        )
    }

    #[cfg(feature = "serde")]
    /// Constructor taking a buffer provider.
    ///
    /// ✨ *Enabled with the `serde` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    pub fn try_new_with_buffer_provider<P>(
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        field_set_with_options: FSet,
    ) -> Result<Self, DateTimeFormatterLoadError>
    where
        P: BufferProvider + ?Sized,
    {
        use crate::provider::compat::CompatProvider;
        let deser_provider = provider.as_deserializing();
        let compat_provider = CompatProvider(&deser_provider, provider);
        Self::try_new_unstable(&compat_provider, prefs, field_set_with_options)
    }
}

impl<C: CldrCalendar, FSet: DateTimeMarkers + DateTimeNamesMarker>
    FixedCalendarDateRangeFormatter<C, FSet>
where
    FSet::D: DateInputMarkers,
    FSet::T: TimeMarkers,
    FSet::Z: ZoneMarkers,
{
    /// Formats a date/time range.
    ///
    /// This method formats the input dates directly, enforcing that they must be in the
    /// statically typed calendar `C`.
    pub fn format<'a, I>(&'a self, start: &I, end: &I) -> FormattedDateRange<'a>
    where
        I: ?Sized + InFixedCalendar<C> + AllInputMarkers<FSet>,
    {
        let start_input =
            DateTimeInputUnchecked::extract_from_neo_input::<FSet::D, FSet::T, FSet::Z, I>(start);
        let end_input =
            DateTimeInputUnchecked::extract_from_neo_input::<FSet::D, FSet::T, FSet::Z, I>(end);

        format_impl_shared(
            &self.datetime_formatter,
            &self.range_selection,
            &start_input,
            &end_input,
        )
    }
}
