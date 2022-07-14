// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use core::marker::PhantomData;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_locid::{extensions_unicode_key as key, Locale};
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;

use crate::{
    date::ZonedDateTimeInput,
    format::zoned_datetime::FormattedZonedDateTime,
    options::DateTimeFormatterOptions,
    provider::{
        self,
        calendar::{
            DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
            TimePatternsV1Marker, TimeSymbolsV1Marker,
        },
        week_data::WeekDataV1Marker,
    },
    raw,
    time_zone::TimeZoneFormatterOptions,
    CldrCalendar, DateTimeFormatterError,
};

/// The composition of [`DateTimeFormatter`](crate::DateTimeFormatter) and [`TimeZoneFormatter`](crate::TimeZoneFormatter).
///
/// [`ZonedDateTimeFormatter`] uses data from the [data provider]s, the selected [`Locale`], and the
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
/// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
/// use icu::datetime::{options::length, ZonedDateTimeFormatter};
/// use icu::locid::locale;
/// use icu_datetime::TimeZoneFormatterOptions;
/// use icu_provider::inv::InvariantDataProvider;
///
/// let date_provider = InvariantDataProvider;
/// let zone_provider = InvariantDataProvider;
/// let plural_provider = InvariantDataProvider;
/// let decimal_provider = InvariantDataProvider;
///
/// let options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
/// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
///     locale!("en"),
///     &date_provider,
///     &zone_provider,
///     &plural_provider,
///     &decimal_provider,
///     &options.into(),
///     &TimeZoneFormatterOptions::default(),
/// )
/// .expect("Failed to create DateTimeFormatter instance.");
///
/// let zoned_datetime: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
///     .parse()
///     .expect("Failed to parse zoned datetime");
///
/// let value = zdtf.format_to_string(&zoned_datetime);
/// ```
pub struct ZonedDateTimeFormatter<C>(raw::ZonedDateTimeFormatter, PhantomData<C>);

impl<C: CldrCalendar> ZonedDateTimeFormatter<C> {
    /// Constructor that takes a selected [`Locale`], a reference to a [data provider] for
    /// dates, a [data provider] for time zones, and a list of [`DateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu::datetime::{DateTimeFormatterOptions, ZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu_datetime::TimeZoneFormatterOptions;
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let date_provider = InvariantDataProvider;
    /// let zone_provider = InvariantDataProvider;
    /// let plural_provider = InvariantDataProvider;
    /// let decimal_provider = InvariantDataProvider;
    ///
    /// let options = DateTimeFormatterOptions::default();
    ///
    /// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     locale!("en"),
    ///     &date_provider,
    ///     &zone_provider,
    ///     &plural_provider,
    ///     &decimal_provider,
    ///     &options,
    ///     &TimeZoneFormatterOptions::default(),
    /// );
    ///
    /// assert_eq!(zdtf.is_ok(), true);
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new<L, DP, ZP, PP, DEP>(
        locale: L,
        date_provider: &DP,
        zone_provider: &ZP,
        plural_provider: &PP,
        decimal_provider: &DEP,
        date_time_format_options: &DateTimeFormatterOptions,
        time_zone_format_options: &TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        L: Into<Locale>,
        DP: ResourceProvider<DateSymbolsV1Marker>
            + ResourceProvider<TimeSymbolsV1Marker>
            + ResourceProvider<DatePatternsV1Marker>
            + ResourceProvider<TimePatternsV1Marker>
            + ResourceProvider<DateSkeletonPatternsV1Marker>
            + ResourceProvider<WeekDataV1Marker>
            + ?Sized,
        ZP: ResourceProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + ResourceProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesShortV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesShortV1Marker>
            + ?Sized,
        PP: ResourceProvider<OrdinalV1Marker> + ?Sized,
        DEP: ResourceProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        let mut locale = locale.into();
        // TODO(#419): Resolve the locale calendar with the API calendar.
        locale
            .extensions
            .unicode
            .keywords
            .set(key!("ca"), C::BCP_47_IDENTIFIER);
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

    /// Takes a [`ZonedDateTimeInput`] implementer and returns an instance of a [`FormattedZonedDateTime`]
    /// that contains all information necessary to display a formatted zoned datetime and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu::datetime::ZonedDateTimeFormatter;
    /// use icu_datetime::TimeZoneFormatterOptions;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale = icu::locid::locale!("en");
    /// # let date_provider = InvariantDataProvider;
    /// # let zone_provider = InvariantDataProvider;
    /// # let plural_provider = InvariantDataProvider;
    /// # let decimal_provider = InvariantDataProvider;
    /// # let options = icu::datetime::DateTimeFormatterOptions::default();
    /// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     locale,
    ///     &date_provider,
    ///     &zone_provider,
    ///     &plural_provider,
    ///     &decimal_provider,
    ///     &options,
    ///     &TimeZoneFormatterOptions::default(),
    /// )
    /// .expect("Failed to create ZonedDateTimeFormatter instance.");
    ///
    /// let zoned_datetime: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
    ///     .parse()
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let formatted_date = zdtf.format(&zoned_datetime);
    ///
    /// let _ = format!("Date: {}", formatted_date);
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but [`FormattedZonedDateTime`] will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedZonedDateTime<'l>
    where
        T: ZonedDateTimeInput,
    {
        self.0.format(value)
    }

    /// Takes a mutable reference to anything that implements the [`Write`](std::fmt::Write) trait
    /// and a [`ZonedDateTimeInput`] implementer, then populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu::datetime::ZonedDateTimeFormatter;
    /// use icu_datetime::TimeZoneFormatterOptions;
    /// # use icu_provider::inv::InvariantDataProvider;
    /// # let locale = icu::locid::locale!("en");
    /// # let date_provider = InvariantDataProvider;
    /// # let zone_provider = InvariantDataProvider;
    /// # let plural_provider = InvariantDataProvider;
    /// # let decimal_provider = InvariantDataProvider;
    /// # let options = icu::datetime::DateTimeFormatterOptions::default();
    /// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     locale,
    ///     &date_provider,
    ///     &zone_provider,
    ///     &plural_provider,
    ///     &decimal_provider,
    ///     &options.into(),
    ///     &TimeZoneFormatterOptions::default(),
    /// )
    /// .expect("Failed to create ZonedDateTimeFormatter instance.");
    ///
    /// let zoned_datetime: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
    ///     .parse()
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let mut buffer = String::new();
    /// zdtf.format_to_write(&mut buffer, &zoned_datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// let _ = format!("Date: {}", buffer);
    /// ```
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl ZonedDateTimeInput,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, value)
    }

    /// Takes a [`ZonedDateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu::datetime::ZonedDateTimeFormatter;
    /// use icu_datetime::TimeZoneFormatterOptions;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale = icu::locid::locale!("en");
    /// # let date_provider = InvariantDataProvider;
    /// # let zone_provider = InvariantDataProvider;
    /// # let plural_provider = InvariantDataProvider;
    /// # let decimal_provider = InvariantDataProvider;
    /// # let options = icu::datetime::DateTimeFormatterOptions::default();
    /// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     locale,
    ///     &date_provider,
    ///     &zone_provider,
    ///     &plural_provider,
    ///     &decimal_provider,
    ///     &options.into(),
    ///     &TimeZoneFormatterOptions::default(),
    /// )
    /// .expect("Failed to create ZonedDateTimeFormatter instance.");
    ///
    /// let zoned_datetime: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
    ///     .parse()
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let _ = zdtf.format_to_string(&zoned_datetime);
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl ZonedDateTimeInput) -> String {
        self.0.format_to_string(value)
    }
}
