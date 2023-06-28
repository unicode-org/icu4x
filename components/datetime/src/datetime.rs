// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`TypedDateTimeFormatter`].

use crate::{
    options::{length, preferences, DateTimeFormatterOptions},
    provider::calendar::{TimeLengthsV1Marker, TimeSymbolsV1Marker},
    provider::date_time::PatternSelector,
    raw,
};
use alloc::string::String;
use core::marker::PhantomData;
use icu_calendar::provider::WeekDataV1Marker;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;
use writeable::Writeable;

use crate::{
    calendar, input::DateInput, input::DateTimeInput, input::IsoTimeInput, CldrCalendar,
    DateTimeError, FormattedDateTime,
};

#[cfg(feature = "experimental")]
use crate::options::components;

/// [`TimeFormatter`] is a structure of the [`icu_datetime`] component that provides time formatting only.
/// When constructed, it uses data from the [data provider], selected locale and provided preferences to
/// collect all data necessary to format any time into that locale.
///
/// For that reason, one should think of the process of formatting a time in two steps - first, a computational
/// heavy construction of [`TimeFormatter`], and then fast formatting of [`DateTimeInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`TypedDateTimeFormatter`]: crate::datetime::TimeFormatter
///
/// # Examples
///
/// ```
/// use icu::calendar::DateTime;
/// use icu::datetime::{options::length::Time, TimeFormatter};
/// use icu::locid::locale;
/// use writeable::assert_writeable_eq;
///
/// let tf = TimeFormatter::try_new_with_length(
///     &locale!("en").into(),
///     Time::Short,
/// )
/// .expect("Failed to create TimeFormatter instance.");
///
/// let datetime = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// assert_writeable_eq!(tf.format(&datetime), "12:34 PM");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
#[derive(Debug)]
pub struct TimeFormatter(pub(super) raw::TimeFormatter);

impl TimeFormatter {
    /// Constructor that takes a selected locale and a list of preferences, then collects all
    /// compiled data necessary to format date and time values into the given locale,
    /// using the given style.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// use icu::locid::locale;
    ///
    /// TimeFormatter::try_new_with_length(
    ///     &locale!("en").into(),
    ///     Time::Short,
    /// )
    /// .unwrap();
    /// ```
    ///
    /// ✨ **Enabled with the `"data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "data")]
    pub fn try_new_with_length(
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, DateTimeError> {
        let preferences = Some(preferences::Bag::from_data_locale(locale));

        Ok(Self(raw::TimeFormatter::try_new(
            locale,
            length,
            preferences,
        )?))
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        length: length::Time,
        error: DateTimeError,
        #[cfg(skip)]
        functions: [
            try_new_with_length,
            try_new_with_length_with_any_provider,
            try_new_with_length_with_buffer_provider,
            try_new_with_length_unstable,
            Self,
        ]
    );

    #[inline]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_length)]
    pub fn try_new_with_length_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, DateTimeError>
    where
        D: DataProvider<TimeLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        let preferences = Some(preferences::Bag::from_data_locale(locale));

        Ok(Self(raw::TimeFormatter::try_new_unstable(
            data_provider,
            locale,
            length,
            preferences,
        )?))
    }

    /// Takes a [`IsoTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let tf = TimeFormatter::try_new_with_length(
    ///     &locale!("en").into(),
    ///     Time::Short,
    /// )
    /// .expect("Failed to create TimeFormatter instance.");
    ///
    /// let datetime = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_writeable_eq!(tf.format(&datetime), "12:34 PM");
    /// ```
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: IsoTimeInput,
    {
        self.0.format(value)
    }

    /// Takes a [`IsoTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// use icu::locid::locale;
    /// let tf = TimeFormatter::try_new_with_length(
    ///     &locale!("en").into(),
    ///     Time::Short,
    /// )
    /// .expect("Failed to create TimeFormatter instance.");
    ///
    /// let datetime = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_eq!(tf.format_to_string(&datetime), "12:34 PM");
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl IsoTimeInput) -> String {
        self.format(value).write_to_string().into_owned()
    }
}

/// [`TypedDateFormatter`] is a formatter capable of formatting
/// dates from a calendar selected at compile time. For the difference between this
/// and [`DateFormatter`](crate::DateFormatter), please read the [crate root docs][crate].
///
/// When constructed, it uses data from the [data provider], selected locale and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`TypedDateFormatter`], and then fast formatting of [`DateInput`] data using the instance.
///
/// [`icu_datetime`]: crate
///
/// # Examples
///
/// ```
/// use icu::calendar::{Date, Gregorian};
/// use icu::datetime::{options::length, TypedDateFormatter};
/// use icu::locid::locale;
/// use writeable::assert_writeable_eq;
///
/// let df = TypedDateFormatter::<Gregorian>::try_new_with_length(
///     &locale!("en").into(),
///     length::Date::Full,
/// )
/// .expect("Failed to create TypedDateFormatter instance.");
///
/// let date = Date::try_new_gregorian_date(2020, 9, 1)
///     .expect("Failed to construct Date.");
///
/// assert_writeable_eq!(df.format(&date), "Tuesday, September 1, 2020");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
#[derive(Debug)]
pub struct TypedDateFormatter<C>(pub(super) raw::DateFormatter, PhantomData<C>);

impl<C: CldrCalendar> TypedDateFormatter<C> {
    /// Constructor that takes a selected locale and a list of options, then collects all data
    /// necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{options::length, TypedDateFormatter};
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     TypedDateFormatter::<Gregorian>::try_new_with_length(
    ///         &locale!("en").into(),
    ///         length::Date::Full,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Date::try_new_gregorian_date(2022, 8, 29).unwrap()),
    ///     "Monday, August 29, 2022",
    /// );
    /// ```
    ///
    /// If the locale has a calendar keyword, the keyword is ignored in favor of the
    /// type parameter on the [`TypedDateFormatter`]. To obey the calendar keyword,
    /// use [`DateFormatter`] instead.
    ///
    /// ```
    /// use icu::calendar::indian::Indian;
    /// use icu::calendar::Date;
    /// use icu::datetime::{options::length, TypedDateFormatter};
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = TypedDateFormatter::<Indian>::try_new_with_length(
    ///     &locale!("en-u-ca-japanese").into(),
    ///     length::Date::Full,
    /// )
    /// .unwrap();
    ///
    /// // Indian format from type wins over locale keyword
    /// assert_writeable_eq!(
    ///     formatter.format(&Date::try_new_indian_date(1944, 6, 7).unwrap()),
    ///     "Monday, Bhadra 7, 1944 Saka",
    /// );
    /// ```
    ///
    /// [`DateFormatter`]: crate::DateFormatter
    ///
    /// ✨ **Enabled with the `"data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "data")]
    pub fn try_new_with_length(
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, DateTimeError>
    where
        crate::provider::Baked: icu_provider::DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>
            + icu_provider::DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>,
    {
        Ok(Self(
            raw::DateFormatter::try_new(
                calendar::load_lengths_for_cldr_calendar::<C, _>(&crate::provider::Baked, locale)?,
                || {
                    calendar::load_symbols_for_cldr_calendar::<C, _>(
                        &crate::provider::Baked,
                        locale,
                    )
                },
                locale,
                length,
            )?,
            PhantomData,
        ))
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        length: length::Date,
        error: DateTimeError,
        #[cfg(skip)]
        functions: [
            try_new_with_length,
            try_new_with_length_with_any_provider,
            try_new_with_length_with_buffer_provider,
            try_new_with_length_unstable,
            Self,
        ]
    );

    #[inline]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_length)]
    pub fn try_new_with_length_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, DateTimeError>
    where
        D: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>
            + DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        Ok(Self(
            raw::DateFormatter::try_new_unstable(
                data_provider,
                calendar::load_lengths_for_cldr_calendar::<C, _>(data_provider, locale)?,
                || calendar::load_symbols_for_cldr_calendar::<C, _>(data_provider, locale),
                locale,
                length,
            )?,
            PhantomData,
        ))
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{Date, Gregorian};
    /// use icu::datetime::{options::length, TypedDateFormatter};
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    /// let df = TypedDateFormatter::<Gregorian>::try_new_with_length(
    ///     &locale!("en").into(),
    ///     length::Date::Full,
    /// )
    /// .expect("Failed to create TypedDateFormatter instance.");
    ///
    /// let date = Date::try_new_gregorian_date(2020, 9, 1)
    ///     .expect("Failed to construct Date.");
    ///
    /// assert_writeable_eq!(df.format(&date), "Tuesday, September 1, 2020");
    /// ```
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: DateInput<Calendar = C>,
    {
        self.0.format(value)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{Date, Gregorian};
    /// use icu::datetime::{options::length, TypedDateFormatter};
    /// use icu::locid::locale;
    /// let df = TypedDateFormatter::<Gregorian>::try_new_with_length(
    ///     &locale!("en").into(),
    ///     length::Date::Short,
    /// )
    /// .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let date = Date::try_new_gregorian_date(2020, 9, 1)
    ///     .expect("Failed to construct Date.");
    ///
    /// assert_eq!(df.format_to_string(&date), "9/1/20");
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateInput<Calendar = C>) -> String {
        self.format(value).write_to_string().into_owned()
    }
}

/// [`TypedDateTimeFormatter`] is a formatter capable of formatting
/// date/times from a calendar selected at compile time. For the difference between this
///  and [`DateTimeFormatter`](crate::DateTimeFormatter), please read the [crate root docs][crate].
///
/// When constructed, it uses data from the [data provider], selected locale and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`TypedDateTimeFormatter`], and then fast formatting of [`DateInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`TypedDateTimeFormatter`]: crate::datetime::TypedDateTimeFormatter
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length, TypedDateTimeFormatter};
/// use icu::locid::locale;
/// use writeable::assert_writeable_eq;
///
/// let mut options = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Short,
/// );
///
/// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new(
///     &locale!("en").into(),
///     options.into(),
/// )
/// .expect("Failed to create TypedDateTimeFormatter instance.");
///
/// let datetime = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// assert_writeable_eq!(dtf.format(&datetime), "Sep 1, 2020, 12:34 PM");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
#[derive(Debug)]
pub struct TypedDateTimeFormatter<C>(pub(super) raw::DateTimeFormatter, PhantomData<C>);

impl<C: CldrCalendar> TypedDateTimeFormatter<C> {
    /// Constructor that takes a [`TimeFormatter`] and [`TypedDateFormatter`] and combines them into a [`TypedDateTimeFormatter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::length, TimeFormatter, TypedDateFormatter,
    ///     TypedDateTimeFormatter,
    /// };
    /// use icu::locid::locale;
    ///
    /// let tf = TimeFormatter::try_new_with_length(
    ///     &locale!("en").into(),
    ///     length::Time::Short,
    /// )
    /// .expect("Failed to create TimeFormatter instance.");
    /// let df = TypedDateFormatter::<Gregorian>::try_new_with_length(
    ///     &locale!("en").into(),
    ///     length::Date::Short,
    /// )
    /// .expect("Failed to create TypedDateFormatter instance.");
    ///
    /// TypedDateTimeFormatter::<Gregorian>::try_from_date_and_time(df, tf)
    ///     .unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_from_date_and_time(
        date: TypedDateFormatter<C>,
        time: TimeFormatter,
    ) -> Result<Self, DateTimeError>
where {
        Ok(Self(
            raw::DateTimeFormatter::try_from_date_and_time(date.0, time.0)?,
            PhantomData,
        ))
    }

    /// Constructor that takes a selected locale, then uses compiled data to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::length, TypedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let options = length::Bag::from_date_time_style(
    ///     length::Date::Medium,
    ///     length::Time::Medium,
    /// );
    ///
    /// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new(
    ///     &locale!("en").into(),
    ///     options.into(),
    /// )
    /// .unwrap();
    ///
    /// let datetime =
    ///     DateTime::try_new_gregorian_datetime(2022, 8, 31, 1, 2, 3).unwrap();
    ///
    /// assert_writeable_eq!(dtf.format(&datetime), "Aug 31, 2022, 1:02:03 AM");
    /// ```
    ///
    /// [data provider]: icu_provider
    ///
    /// ✨ **Enabled with the `"data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "data")]
    pub fn try_new(
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        crate::provider::Baked: DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>,
        crate::provider::Baked: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>,
    {
        let patterns = PatternSelector::for_options(
            &crate::provider::Baked,
            calendar::load_lengths_for_cldr_calendar::<C, _>(&crate::provider::Baked, locale)?,
            locale,
            &options,
        )?;
        Ok(Self(
            raw::DateTimeFormatter::try_new(
                patterns,
                || {
                    calendar::load_symbols_for_cldr_calendar::<C, _>(
                        &crate::provider::Baked,
                        locale,
                    )
                },
                locale,
            )?,
            PhantomData,
        ))
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: DateTimeFormatterOptions,
        error: DateTimeError,
        #[cfg(skip)]
        functions: [
            try_new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[inline]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        D: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>
            + DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        let patterns = PatternSelector::for_options(
            data_provider,
            calendar::load_lengths_for_cldr_calendar::<C, _>(data_provider, locale)?,
            locale,
            &options,
        )?;
        Ok(Self(
            raw::DateTimeFormatter::try_new_unstable(
                data_provider,
                patterns,
                || calendar::load_symbols_for_cldr_calendar::<C, _>(data_provider, locale),
                locale,
            )?,
            PhantomData,
        ))
    }

    /// Constructor that supports experimental options using compiled data.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
    /// of the icu meta-crate. Use with caution.
    /// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::components, TypedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu_provider::AsDeserializingBufferProvider;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut options = components::Bag::default();
    /// options.year = Some(components::Year::Numeric);
    /// options.month = Some(components::Month::Long);
    ///
    /// let dtf =
    ///     TypedDateTimeFormatter::<Gregorian>::try_new_experimental(
    ///         &locale!("en").into(),
    ///         options.into(),
    ///     )
    ///     .unwrap();
    ///
    /// let datetime =
    ///     DateTime::try_new_gregorian_datetime(2022, 8, 31, 1, 2, 3).unwrap();
    ///
    /// assert_writeable_eq!(dtf.format(&datetime), "August 2022");
    /// ```
    ///
    /// [data provider]: icu_provider
    ///
    /// ✨ **Enabled with the `"data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "experimental")]
    #[inline]
    #[cfg(feature = "data")]
    pub fn try_new_experimental(
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        crate::provider::Baked: DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>,
        crate::provider::Baked: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>,
    {
        let patterns = PatternSelector::for_options_experimental(
            &crate::provider::Baked,
            calendar::load_lengths_for_cldr_calendar::<C, _>(&crate::provider::Baked, locale)?,
            locale,
            &C::DEFAULT_BCP_47_IDENTIFIER,
            &options,
        )?;
        Ok(Self(
            raw::DateTimeFormatter::try_new(
                patterns,
                || {
                    calendar::load_symbols_for_cldr_calendar::<C, _>(
                        &crate::provider::Baked,
                        locale,
                    )
                },
                locale,
            )?,
            PhantomData,
        ))
    }

    /// Constructor that supports experimental options.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
    /// of the icu meta-crate. Use with caution.
    /// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::components, TypedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu_provider::AsDeserializingBufferProvider;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut options = components::Bag::default();
    /// options.year = Some(components::Year::Numeric);
    /// options.month = Some(components::Month::Long);
    ///
    /// let dtf =
    ///     TypedDateTimeFormatter::<Gregorian>::try_new_experimental(
    ///         &locale!("en").into(),
    ///         options.into(),
    ///     )
    ///     .unwrap();
    ///
    /// let datetime =
    ///     DateTime::try_new_gregorian_datetime(2022, 8, 31, 1, 2, 3).unwrap();
    ///
    /// assert_writeable_eq!(dtf.format(&datetime), "August 2022");
    /// ```
    ///
    /// [data provider]: icu_provider
    #[cfg(feature = "experimental")]
    #[inline]
    pub fn try_new_experimental_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        D: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>
            + DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<crate::provider::calendar::DateSkeletonPatternsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        let patterns = PatternSelector::for_options_experimental(
            data_provider,
            calendar::load_lengths_for_cldr_calendar::<C, _>(data_provider, locale)?,
            locale,
            &C::DEFAULT_BCP_47_IDENTIFIER,
            &options,
        )?;
        Ok(Self(
            raw::DateTimeFormatter::try_new_unstable(
                data_provider,
                patterns,
                || calendar::load_symbols_for_cldr_calendar::<C, _>(data_provider, locale),
                locale,
            )?,
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
    /// use icu::datetime::TypedDateTimeFormatter;
    /// use writeable::assert_writeable_eq;
    /// use icu::locid::locale;
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new(&locale!("en").into(), options.into())
    ///     .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_writeable_eq!(dtf.format(&datetime), "12:34:28 PM");
    /// ```
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: DateTimeInput<Calendar = C>,
    {
        self.0.format(value)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::TypedDateTimeFormatter;
    /// use icu::locid::locale;
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new(&locale!("en").into(), options.into())
    ///     .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_eq!(dtf.format_to_string(&datetime), "12:34:28 PM");
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput<Calendar = C>) -> String {
        self.format(value).write_to_string().into_owned()
    }

    /// Returns a [`components::Bag`] that represents the resolved components for the
    /// options that were provided to the [`TypedDateTimeFormatter`]. The developer may request
    /// a certain set of options for a [`TypedDateTimeFormatter`] but the locale and resolution
    /// algorithm may change certain details of what actually gets resolved.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::{components, length},
    ///     DateTimeFormatterOptions, TypedDateTimeFormatter,
    /// };
    /// use icu::locid::locale;
    ///
    /// let options = length::Bag::from_date_style(length::Date::Medium).into();
    /// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new(
    ///     &locale!("en").into(),
    ///     options,
    /// )
    /// .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let mut expected_components_bag = components::Bag::default();
    /// expected_components_bag.year = Some(components::Year::Numeric);
    /// expected_components_bag.month = Some(components::Month::Short);
    /// expected_components_bag.day = Some(components::Day::NumericDayOfMonth);
    ///
    /// assert_eq!(dtf.resolve_components(), expected_components_bag);
    /// ```
    #[cfg(feature = "experimental")]
    pub fn resolve_components(&self) -> components::Bag {
        self.0.resolve_components()
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;
    use icu_calendar::DateTime;
    use icu_calendar::Gregorian;

    #[test]
    fn works_with_default_options() {
        assert_eq!(
            TypedDateTimeFormatter::<Gregorian>::try_new(Default::default(), Default::default(),)
                .unwrap()
                .format_to_string(
                    &DateTime::try_new_gregorian_datetime(2022, 9, 20, 0, 0, 0).unwrap()
                ),
            "2022 M09 20 00:00:00"
        );
    }
}
