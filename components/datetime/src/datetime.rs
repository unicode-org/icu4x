// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`DateTimeFormat`].

use crate::{
    options::{components, length, preferences, DateTimeFormatOptions},
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
use icu_locid::{extensions_unicode_key as key, Locale};
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;

use crate::{date::DateTimeInput, CldrCalendar, DateTimeFormatError, FormattedDateTime};

/// [`TimeFormat`] is a structure of the [`icu_datetime`] component that provides time formatting only.
/// When constructed, it uses data from the [data provider], selected [`Locale`] and provided preferences to
/// collect all data necessary to format any time into that locale.
///
/// For that reason, one should think of the process of formatting a time in two steps - first, a computational
/// heavy construction of [`TimeFormat`], and then fast formatting of [`DateTimeInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`DateTimeFormat`]: crate::datetime::TimeFormat
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length::Time, TimeFormat};
/// use icu::locid::locale;
///
/// let provider = icu_testdata::get_provider();
///
/// let tf = TimeFormat::<Gregorian>::try_new(
///     locale!("en"),
///     &provider,
///     Time::Short,
///     None,
/// )
/// .expect("Failed to create DateTimeFormat instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// println!("{}", tf.format_to_string(&datetime));
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct TimeFormat<C>(pub(super) raw::TimeFormat, PhantomData<C>);

impl<C: CldrCalendar> TimeFormat<C> {
    /// Constructor that takes a selected [`Locale`], reference to a [data provider] and
    /// a list of preferences, then collects all data necessary to format date and time values into the given locale,
    /// using the short style.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{options::length::Time, TimeFormat};
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// TimeFormat::<Gregorian>::try_new(
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
    pub fn try_new<T: Into<Locale>, D>(
        locale: T,
        data_provider: &D,
        length: length::Time,
        preferences: Option<preferences::Bag>,
    ) -> Result<Self, DateTimeFormatError>
    where
        D: ResourceProvider<TimePatternsV1Marker>
            + ResourceProvider<TimeSymbolsV1Marker>
            + ResourceProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        let mut locale = locale.into();
        // TODO(#419): Resolve the locale calendar with the API calendar.
        locale
            .extensions
            .unicode
            .keywords
            .set(key!("ca"), C::BCP_47_IDENTIFIER);

        Ok(Self(
            raw::TimeFormat::try_new(locale, data_provider, length, preferences)?,
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
    /// use icu::datetime::{options::length::Time, TimeFormat};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let tf =
    ///     TimeFormat::<Gregorian>::try_new(locale, &provider, Time::Short, None)
    ///         .expect("Failed to create TimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let formatted = tf.format(&datetime);
    ///
    /// println!("Time: {}", formatted);
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
    /// use icu::datetime::{options::length::Time, TimeFormat};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let tf =
    ///     TimeFormat::<Gregorian>::try_new(locale, &provider, Time::Short, None)
    ///         .expect("Failed to create TimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// tf.format_to_write(&mut buffer, &datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// println!("Time: {}", buffer);
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
    /// use icu::datetime::{options::length::Time, TimeFormat};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let tf =
    ///     TimeFormat::<Gregorian>::try_new(locale, &provider, Time::Short, None)
    ///         .expect("Failed to create TimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// println!("{}", tf.format_to_string(&datetime));
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput) -> String {
        self.0.format_to_string(value)
    }
}

/// [`DateFormat`] is a structure of the [`icu_datetime`] component that provides date formatting only.
/// When constructed, it uses data from the [data provider], selected [`Locale`] and provided preferences to
/// collect all data necessary to format any date into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`DateFormat`], and then fast formatting of [`DateTimeInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`DateTimeFormat`]: crate::datetime::DateFormat
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length::Date, DateFormat};
/// use icu::locid::locale;
///
/// let provider = icu_testdata::get_provider();
///
/// let df = DateFormat::<Gregorian>::try_new(locale!("en"), &provider, Date::Full)
///     .expect("Failed to create DateFormat instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// println!("{}", df.format_to_string(&datetime));
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct DateFormat<C>(pub(super) raw::DateFormat, PhantomData<C>);

impl<C: CldrCalendar> DateFormat<C> {
    /// Constructor that takes a selected [`Locale`], reference to a [data provider] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{options::length::Date, DateFormat};
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// DateFormat::<Gregorian>::try_new(locale!("en"), &provider, Date::Full)
    ///     .unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new<T: Into<Locale>, D>(
        locale: T,
        data_provider: &D,
        length: length::Date,
    ) -> Result<Self, DateTimeFormatError>
    where
        D: ResourceProvider<DateSymbolsV1Marker>
            + ResourceProvider<DatePatternsV1Marker>
            + ResourceProvider<DecimalSymbolsV1Marker>
            + ResourceProvider<OrdinalV1Marker>
            + ResourceProvider<WeekDataV1Marker>
            + ?Sized,
    {
        let mut locale = locale.into();
        // TODO(#419): Resolve the locale calendar with the API calendar.
        locale
            .extensions
            .unicode
            .keywords
            .set(key!("ca"), C::BCP_47_IDENTIFIER);

        Ok(Self(
            raw::DateFormat::try_new(locale, data_provider, length)?,
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
    /// use icu::datetime::{options::length::Date, DateFormat};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let df = DateFormat::<Gregorian>::try_new(locale, &provider, Date::Full)
    ///     .expect("Failed to create DateFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let formatted = df.format(&datetime);
    ///
    /// println!("Time: {}", formatted);
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
    /// use icu::datetime::{options::length::Date, DateFormat};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let df = DateFormat::<Gregorian>::try_new(locale, &provider, Date::Short)
    ///     .expect("Failed to create DateFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// df.format_to_write(&mut buffer, &datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// println!("Time: {}", buffer);
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
    /// use icu::datetime::{options::length::Date, DateFormat};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let df = DateFormat::<Gregorian>::try_new(locale, &provider, Date::Short)
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// println!("{}", df.format_to_string(&datetime));
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput<Calendar = C>) -> String {
        self.0.format_to_string(value)
    }
}

/// [`DateTimeFormat`] is the main structure of the [`icu_datetime`] component.
/// When constructed, it uses data from the [data provider], selected [`Locale`] and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`DateTimeFormat`], and then fast formatting of [`DateTimeInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`DateTimeFormat`]: crate::datetime::DateTimeFormat
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length, DateTimeFormat};
/// use icu::locid::locale;
///
/// let provider = icu_testdata::get_provider();
///
/// let mut options = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Short,
/// );
///
/// let dtf = DateTimeFormat::<Gregorian>::try_new(
///     locale!("en"),
///     &provider,
///     &options.into(),
/// )
/// .expect("Failed to create DateTimeFormat instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// println!("{}", dtf.format_to_string(&datetime));
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct DateTimeFormat<C>(pub(super) raw::DateTimeFormat, PhantomData<C>);

impl<C: CldrCalendar> DateTimeFormat<C> {
    /// Constructor that takes a [`TimeFormat`] and [`DateFormat`] and combines them into a [`DateTimeFormat`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::length, DateFormat, DateTimeFormat, TimeFormat,
    /// };
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let tf = TimeFormat::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &provider,
    ///     length::Time::Short,
    ///     None,
    /// )
    /// .expect("Failed to create TimeFormat instance.");
    /// let df = DateFormat::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &provider,
    ///     length::Date::Short,
    /// )
    /// .expect("Failed to create DateFormat instance.");
    ///
    /// DateTimeFormat::<Gregorian>::try_from_date_and_time(df, tf).unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_from_date_and_time(
        date: DateFormat<C>,
        time: TimeFormat<C>,
    ) -> Result<Self, DateTimeFormatError>
where {
        Ok(Self(
            raw::DateTimeFormat::try_from_date_and_time(date.0, time.0)?,
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
    /// use icu::datetime::{options::length, DateTimeFormat};
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let options = length::Bag::from_time_style(length::Time::Medium);
    ///
    /// DateTimeFormat::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &provider,
    ///     &options.into(),
    /// )
    /// .unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new<T: Into<Locale>, D>(
        locale: T,
        data_provider: &D,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        D: ResourceProvider<DateSymbolsV1Marker>
            + ResourceProvider<TimeSymbolsV1Marker>
            + ResourceProvider<DatePatternsV1Marker>
            + ResourceProvider<TimePatternsV1Marker>
            + ResourceProvider<DateSkeletonPatternsV1Marker>
            + ResourceProvider<DecimalSymbolsV1Marker>
            + ResourceProvider<OrdinalV1Marker>
            + ResourceProvider<WeekDataV1Marker>
            + ?Sized,
    {
        let mut locale = locale.into();
        // TODO(#419): Resolve the locale calendar with the API calendar.
        locale
            .extensions
            .unicode
            .keywords
            .set(key!("ca"), C::BCP_47_IDENTIFIER);
        Ok(Self(
            raw::DateTimeFormat::try_new(locale, data_provider, options)?,
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
    /// use icu::datetime::DateTimeFormat;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = DateTimeFormat::<Gregorian>::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let formatted_date = dtf.format(&datetime);
    ///
    /// println!("Date: {}", formatted_date);
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
    /// use icu::datetime::DateTimeFormat;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = DateTimeFormat::<Gregorian>::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// dtf.format_to_write(&mut buffer, &datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// println!("Date: {}", buffer);
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
    /// use icu::datetime::DateTimeFormat;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = DateTimeFormat::<Gregorian>::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// println!("{}", dtf.format_to_string(&datetime));
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput<Calendar = C>) -> String {
        self.0.format_to_string(value)
    }

    /// Returns a [`components::Bag`] that represents the resolved components for the
    /// options that were provided to the [`DateTimeFormat`]. The developer may request
    /// a certain set of options for a [`DateTimeFormat`] but the locale and resolution
    /// algorithm may change certain details of what actually gets resolved.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::{components, length},
    ///     DateTimeFormat, DateTimeFormatOptions,
    /// };
    /// use icu::locid::locale;
    ///
    /// let options = length::Bag::from_date_style(length::Date::Medium).into();
    ///
    /// let provider = icu_testdata::get_provider();
    /// let dtf = DateTimeFormat::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &provider,
    ///     &options,
    /// )
    /// .expect("Failed to create DateTimeFormat instance.");
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
