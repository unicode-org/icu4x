// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`DateTimeFormatter`].

use crate::{
    options::{components, length, preferences, DateTimeFormatterOptions},
    provider::calendar::{
        DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
        TimePatternsV1Marker, TimeSymbolsV1Marker,
    },
    provider::week_data::WeekDataV1Marker,
    raw,
};
use alloc::string::String;
use core::marker::PhantomData;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;

use crate::{
    calendar, date::DateTimeInput, CldrCalendar, DateTimeFormatterError, FormattedDateTime,
};

/// [`TimeFormatter`] is a structure of the [`icu_datetime`] component that provides time formatting only.
/// When constructed, it uses data from the [data provider], selected [`Locale`] and provided preferences to
/// collect all data necessary to format any time into that locale.
///
/// For that reason, one should think of the process of formatting a time in two steps - first, a computational
/// heavy construction of [`TimeFormatter`], and then fast formatting of [`DateTimeInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`DateTimeFormatter`]: crate::datetime::TimeFormatter
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length::Time, TimeFormatter};
/// use icu::locid::locale;
/// use writeable::assert_writeable_eq;
///
/// let provider = icu_testdata::get_provider();
///
/// let tf = TimeFormatter::<Gregorian>::try_new(
///     locale!("en"),
///     &provider,
///     Time::Short,
///     None,
/// )
/// .expect("Failed to create TimeFormatter instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// assert_writeable_eq!(tf.format(&datetime), "12:34 PM");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct TimeFormatter<C>(pub(super) raw::TimeFormatter, PhantomData<C>);

impl<C: CldrCalendar> TimeFormatter<C> {
    /// Constructor that takes a selected [`Locale`], reference to a [data provider] and
    /// a list of preferences, then collects all data necessary to format date and time values into the given locale,
    /// using the short style.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// TimeFormatter::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &provider,
    ///     Time::Short,
    ///     None,
    /// )
    /// .unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new<D>(
        locale: &DataLocale,
        data_provider: &D,
        length: length::Time,
        preferences: Option<preferences::Bag>,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<TimePatternsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        // TODO(#2188): Avoid cloning the DataLocale by passing the calendar
        // separately into the raw formatter.
        let mut locale = locale.clone();

        calendar::potentially_fixup_calendar::<C>(&mut locale)?;
        Ok(Self(
            raw::TimeFormatter::try_new(locale, data_provider, length, preferences)?,
            PhantomData,
        ))
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// use writeable::assert_writeable_eq;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let tf =
    ///     TimeFormatter::<Gregorian>::try_new(locale, &provider, Time::Short, None)
    ///         .expect("Failed to create TimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_writeable_eq!(tf.format(&datetime), "12:34 PM");
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but [`FormattedDateTime`] will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l>
    where
        T: DateTimeInput,
    {
        self.0.format(value)
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let tf =
    ///     TimeFormatter::<Gregorian>::try_new(locale, &provider, Time::Short, None)
    ///         .expect("Failed to create TimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// tf.format_to_write(&mut buffer, &datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// assert_eq!(buffer, "12:34 PM");
    /// ```
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, value)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let tf =
    ///     TimeFormatter::<Gregorian>::try_new(locale, &provider, Time::Short, None)
    ///         .expect("Failed to create TimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_eq!(tf.format_to_string(&datetime), "12:34 PM");
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput) -> String {
        self.0.format_to_string(value)
    }
}

/// [`DateFormatter`] is a structure of the [`icu_datetime`] component that provides date formatting only.
/// When constructed, it uses data from the [data provider], selected [`Locale`] and provided preferences to
/// collect all data necessary to format any date into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`DateFormatter`], and then fast formatting of [`DateTimeInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`DateTimeFormatter`]: crate::datetime::DateFormatter
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length::Date, DateFormatter};
/// use icu::locid::locale;
///
/// let provider = icu_testdata::get_provider();
///
/// let df = DateFormatter::<Gregorian>::try_new(locale!("en"), &provider, Date::Full)
///     .expect("Failed to create DateFormatter instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// assert_eq!(df.format_to_string(&datetime), "Tuesday, September 1, 2020");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct DateFormatter<C>(pub(super) raw::DateFormatter, PhantomData<C>);

impl<C: CldrCalendar> DateFormatter<C> {
    /// Constructor that takes a selected [`Locale`], reference to a [data provider] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{options::length::Date, DateFormatter};
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// DateFormatter::<Gregorian>::try_new(locale!("en"), &provider, Date::Full)
    ///     .unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new<D>(
        locale: &DataLocale,
        data_provider: &D,
        length: length::Date,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<DateSymbolsV1Marker>
            + DataProvider<DatePatternsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        // TODO(#2188): Avoid cloning the DataLocale by passing the calendar
        // separately into the raw formatter.
        let mut locale = locale.clone();

        calendar::potentially_fixup_calendar::<C>(&mut locale)?;
        Ok(Self(
            raw::DateFormatter::try_new(locale, data_provider, length)?,
            PhantomData,
        ))
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::length::Date, DateFormatter};
    /// use writeable::assert_writeable_eq;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let df = DateFormatter::<Gregorian>::try_new(locale, &provider, Date::Full)
    ///     .expect("Failed to create DateFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_writeable_eq!(df.format(&datetime), "Tuesday, September 1, 2020");
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but [`FormattedDateTime`] will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l>
    where
        T: DateTimeInput<Calendar = C>,
    {
        self.0.format(value)
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::length::Date, DateFormatter};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let df = DateFormatter::<Gregorian>::try_new(locale, &provider, Date::Short)
    ///     .expect("Failed to create DateFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// df.format_to_write(&mut buffer, &datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// assert_eq!(buffer, "9/1/20");
    /// ```
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput<Calendar = C>,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, value)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::length::Date, DateFormatter};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let df = DateFormatter::<Gregorian>::try_new(locale, &provider, Date::Short)
    ///     .expect("Failed to create DateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_eq!(df.format_to_string(&datetime), "9/1/20");
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput<Calendar = C>) -> String {
        self.0.format_to_string(value)
    }
}

/// [`DateTimeFormatter`] is the main structure of the [`icu_datetime`] component.
/// When constructed, it uses data from the [data provider], selected [`Locale`] and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`DateTimeFormatter`], and then fast formatting of [`DateTimeInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`DateTimeFormatter`]: crate::datetime::DateTimeFormatter
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length, DateTimeFormatter};
/// use icu::locid::locale;
///
/// let provider = icu_testdata::get_provider();
///
/// let mut options = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Short,
/// );
///
/// let dtf = DateTimeFormatter::<Gregorian>::try_new(
///     locale!("en"),
///     &provider,
///     &options.into(),
/// )
/// .expect("Failed to create DateTimeFormatter instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// assert_eq!(dtf.format_to_string(&datetime), "Sep 1, 2020, 12:34 PM");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct DateTimeFormatter<C>(pub(super) raw::DateTimeFormatter, PhantomData<C>);

impl<C: CldrCalendar> DateTimeFormatter<C> {
    /// Constructor that takes a [`TimeFormatter`] and [`DateFormatter`] and combines them into a [`DateTimeFormatter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::length, DateFormatter, DateTimeFormatter, TimeFormatter,
    /// };
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let tf = TimeFormatter::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &provider,
    ///     length::Time::Short,
    ///     None,
    /// )
    /// .expect("Failed to create TimeFormatter instance.");
    /// let df = DateFormatter::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &provider,
    ///     length::Date::Short,
    /// )
    /// .expect("Failed to create DateFormatter instance.");
    ///
    /// DateTimeFormatter::<Gregorian>::try_from_date_and_time(df, tf).unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_from_date_and_time(
        date: DateFormatter<C>,
        time: TimeFormatter<C>,
    ) -> Result<Self, DateTimeFormatterError>
where {
        Ok(Self(
            raw::DateTimeFormatter::try_from_date_and_time(date.0, time.0)?,
            PhantomData,
        ))
    }

    /// Constructor that takes a selected [`Locale`], reference to a [data provider] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{options::length, DateTimeFormatter};
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let options = length::Bag::from_time_style(length::Time::Medium);
    ///
    /// DateTimeFormatter::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &provider,
    ///     &options.into(),
    /// )
    /// .unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new<D>(
        locale: &DataLocale,
        data_provider: &D,
        options: &DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<DateSymbolsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DatePatternsV1Marker>
            + DataProvider<TimePatternsV1Marker>
            + DataProvider<DateSkeletonPatternsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        // TODO(#2188): Avoid cloning the DataLocale by passing the calendar
        // separately into the raw formatter.
        let mut locale = locale.clone();

        calendar::potentially_fixup_calendar::<C>(&mut locale)?;
        Ok(Self(
            raw::DateTimeFormatter::try_new(locale, data_provider, options)?,
            PhantomData,
        ))
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::DateTimeFormatter;
    /// use writeable::assert_writeable_eq;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = DateTimeFormatter::<Gregorian>::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_writeable_eq!(dtf.format(&datetime), "12:34:28 PM");
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but [`FormattedDateTime`] will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: DateTimeInput<Calendar = C>,
    {
        self.0.format(value)
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::DateTimeFormatter;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = DateTimeFormatter::<Gregorian>::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// dtf.format_to_write(&mut buffer, &datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// assert_eq!(buffer, "12:34:28 PM");
    /// ```
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput<Calendar = C>,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, value)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::DateTimeFormatter;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = DateTimeFormatter::<Gregorian>::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_eq!(dtf.format_to_string(&datetime), "12:34:28 PM");
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput<Calendar = C>) -> String {
        self.0.format_to_string(value)
    }

    /// Returns a [`components::Bag`] that represents the resolved components for the
    /// options that were provided to the [`DateTimeFormatter`]. The developer may request
    /// a certain set of options for a [`DateTimeFormatter`] but the locale and resolution
    /// algorithm may change certain details of what actually gets resolved.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::{components, length},
    ///     DateTimeFormatter, DateTimeFormatterOptions,
    /// };
    /// use icu::locid::locale;
    ///
    /// let options = length::Bag::from_date_style(length::Date::Medium).into();
    ///
    /// let provider = icu_testdata::get_provider();
    /// let dtf = DateTimeFormatter::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &provider,
    ///     &options,
    /// )
    /// .expect("Failed to create DateTimeFormatter instance.");
    ///
    /// let mut expected_components_bag = components::Bag::default();
    /// expected_components_bag.year = Some(components::Year::Numeric);
    /// expected_components_bag.month = Some(components::Month::Short);
    /// expected_components_bag.day = Some(components::Day::NumericDayOfMonth);
    ///
    /// assert_eq!(dtf.resolve_components(), expected_components_bag);
    /// ```
    pub fn resolve_components(&self) -> components::Bag {
        self.0.resolve_components()
    }
}
