// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use core::marker::PhantomData;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;

use crate::{
    calendar,
    input::{DateTimeInput, TimeZoneInput},
    format::zoned_datetime::FormattedZonedDateTime,
    options::DateTimeFormatterOptions,
    provider::{
        self,
        calendar::{
            DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
            TimeLengthsV1Marker, TimeSymbolsV1Marker,
        },
        week_data::WeekDataV1Marker,
    },
    raw,
    time_zone::TimeZoneFormatterOptions,
    CldrCalendar, DateTimeFormatterError,
};

/// The composition of [`DateTimeFormatter`](crate::DateTimeFormatter) and [`TimeZoneFormatter`](crate::TimeZoneFormatter).
///
/// [`ZonedDateTimeFormatter`] uses data from the [data provider]s, the selected locale, and the
/// provided pattern to collect all data necessary to format a datetime with time zones into that locale.
///
/// The various pattern symbols specified in UTS-35 require different sets of data for formatting.
/// As such, `TimeZoneFormatter` will pull in only the resources it needs to format that pattern
/// that is derived from the provided [`DateTimeFormatterOptions`].
///
/// For that reason, one should think of the process of formatting a zoned datetime in two steps:
/// first, a computationally heavy construction of [`ZonedDateTimeFormatter`], and then fast formatting
/// of the data using the instance.
///
/// # Examples
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::datetime::mock::parse_zoned_gregorian_from_str;
/// use icu::datetime::{options::length, ZonedDateTimeFormatter};
/// use icu::locid::locale;
/// use icu_datetime::TimeZoneFormatterOptions;
///
/// let provider = icu_testdata::get_provider();
///
/// let options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
/// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
///     &locale!("en").into(),
///     &provider,
///     &provider,
///     &provider,
///     &provider,
///     &options.into(),
///     &TimeZoneFormatterOptions::default(),
/// )
/// .expect("Failed to create DateTimeFormatter instance.");
///
/// let (datetime, time_zone) = parse_zoned_gregorian_from_str("2021-04-08T16:12:37.000-07:00")
///     .expect("Failed to parse zoned datetime");
///
/// let value = zdtf.format_to_string(&datetime, &time_zone);
/// ```
pub struct ZonedDateTimeFormatter<C>(raw::ZonedDateTimeFormatter, PhantomData<C>);

impl<C: CldrCalendar> ZonedDateTimeFormatter<C> {
    /// Constructor that takes a selected locale, a reference to a [data provider] for
    /// dates, a [data provider] for time zones, and a list of [`DateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{DateTimeFormatterOptions, ZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu_datetime::TimeZoneFormatterOptions;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let options = DateTimeFormatterOptions::default();
    ///
    /// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     &locale!("en").into(),
    ///     &provider,
    ///     &provider,
    ///     &provider,
    ///     &provider,
    ///     &options,
    ///     &TimeZoneFormatterOptions::default(),
    /// );
    ///
    /// assert_eq!(zdtf.is_ok(), true);
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new<DP, ZP, PP, DEP>(
        locale: &DataLocale,
        date_provider: &DP,
        zone_provider: &ZP,
        plural_provider: &PP,
        decimal_provider: &DEP,
        date_time_format_options: &DateTimeFormatterOptions,
        time_zone_format_options: &TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        DP: DataProvider<DateSymbolsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DatePatternsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<DateSkeletonPatternsV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
        ZP: DataProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + DataProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + DataProvider<provider::time_zones::MetaZoneGenericNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetaZoneGenericNamesShortV1Marker>
            + DataProvider<provider::time_zones::MetaZoneSpecificNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetaZoneSpecificNamesShortV1Marker>
            + ?Sized,
        PP: DataProvider<OrdinalV1Marker> + ?Sized,
        DEP: DataProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        // TODO(#2188): Avoid cloning the DataLocale by passing the calendar
        // separately into the raw formatter.
        let mut locale = locale.clone();

        calendar::potentially_fixup_calendar::<C>(&mut locale)?;
        Ok(Self(
            raw::ZonedDateTimeFormatter::try_new(
                locale,
                date_provider,
                zone_provider,
                plural_provider,
                decimal_provider,
                date_time_format_options,
                time_zone_format_options,
            )?,
            PhantomData,
        ))
    }

    /// Takes a [`DateTimeInput`] and a [`TimeZoneInput`] and returns an instance of a [`FormattedZonedDateTime`]
    /// that contains all information necessary to display a formatted zoned datetime and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::mock::parse_zoned_gregorian_from_str;
    /// use icu::datetime::ZonedDateTimeFormatter;
    /// use icu_datetime::TimeZoneFormatterOptions;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::DateTimeFormatterOptions::default();
    /// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     &locale.into(),
    ///     &provider,
    ///     &provider,
    ///     &provider,
    ///     &provider,
    ///     &options,
    ///     &TimeZoneFormatterOptions::default(),
    /// )
    /// .expect("Failed to create ZonedDateTimeFormatter instance.");
    ///
    /// let (datetime, time_zone) = parse_zoned_gregorian_from_str("2021-04-08T16:12:37.000-07:00")
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let formatted_date = zdtf.format(&datetime, &time_zone);
    ///
    /// let _ = format!("Date: {}", formatted_date);
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but [`FormattedZonedDateTime`] will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    #[inline]
    pub fn format<'l>(
        &'l self,
        date: &impl DateTimeInput<Calendar = C>,
        time_zone: &impl TimeZoneInput,
    ) -> FormattedZonedDateTime<'l> {
        self.0.format(date, time_zone)
    }

    /// Takes a mutable reference to anything that implements the [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] and a [`TimeZoneInput`], then populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::mock::parse_zoned_gregorian_from_str;
    /// use icu::datetime::ZonedDateTimeFormatter;
    /// use icu_datetime::TimeZoneFormatterOptions;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::DateTimeFormatterOptions::default();
    /// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     &locale.into(),
    ///     &provider,
    ///     &provider,
    ///     &provider,
    ///     &provider,
    ///     &options.into(),
    ///     &TimeZoneFormatterOptions::default(),
    /// )
    /// .expect("Failed to create ZonedDateTimeFormatter instance.");
    ///
    /// let (datetime, time_zone) = parse_zoned_gregorian_from_str("2021-04-08T16:12:37.000-07:00")
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let mut buffer = String::new();
    /// zdtf.format_to_write(&mut buffer, &datetime, &time_zone)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// let _ = format!("Date: {}", buffer);
    /// ```
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        date: &impl DateTimeInput<Calendar = C>,
        time_zone: &impl TimeZoneInput,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, date, time_zone)
    }

    /// Takes a [`DateTimeInput`] and a [`TimeZoneInput`] and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::mock::parse_zoned_gregorian_from_str;
    /// use icu::datetime::ZonedDateTimeFormatter;
    /// use icu_datetime::TimeZoneFormatterOptions;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::DateTimeFormatterOptions::default();
    /// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     &locale.into(),
    ///     &provider,
    ///     &provider,
    ///     &provider,
    ///     &provider,
    ///     &options.into(),
    ///     &TimeZoneFormatterOptions::default(),
    /// )
    /// .expect("Failed to create ZonedDateTimeFormatter instance.");
    ///
    /// let (datetime, time_zone) = parse_zoned_gregorian_from_str("2021-04-08T16:12:37.000-07:00")
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let _ = zdtf.format_to_string(&datetime, &time_zone);
    /// ```
    #[inline]
    pub fn format_to_string(
        &self,
        date: &impl DateTimeInput<Calendar = C>,
        time_zone: &impl TimeZoneInput,
    ) -> String {
        self.0.format_to_string(date, time_zone)
    }
}
