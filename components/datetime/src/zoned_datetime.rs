// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use core::marker::PhantomData;
use icu_locid::{unicode_ext_key, Locale};
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;

use crate::{
    date::ZonedDateTimeInput,
    format::zoned_datetime::FormattedZonedDateTime,
    options::DateTimeFormatOptions,
    provider::{
        self,
        calendar::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker},
        week_data::WeekDataV1Marker,
    },
    raw,
    time_zone::TimeZoneFormatOptions,
    CldrCalendar, DateTimeFormatError,
};

/// The composition of [`DateTimeFormat`](crate::DateTimeFormat) and [`TimeZoneFormat`](crate::TimeZoneFormat).
///
/// [`ZonedDateTimeFormat`] uses data from the [data provider]s, the selected [`Locale`], and the
/// provided pattern to collect all data necessary to format a datetime with time zones into that locale.
///
/// The various pattern symbols specified in UTS-35 require different sets of data for formatting.
/// As such, `TimeZoneFormat` will pull in only the resources it needs to format that pattern
/// that is derived from the provided [`DateTimeFormatOptions`].
///
/// For that reason, one should think of the process of formatting a zoned datetime in two steps:
/// first, a computationally heavy construction of [`ZonedDateTimeFormat`], and then fast formatting
/// of the data using the instance.
///
/// # Examples
///
/// ```
/// use icu::locid::locale;
/// use icu::calendar::Gregorian;
/// use icu::datetime::{ZonedDateTimeFormat, options::length};
/// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
/// use icu_datetime::TimeZoneFormatOptions;
/// use icu_provider::inv::InvariantDataProvider;
///
/// let date_provider = InvariantDataProvider;
/// let zone_provider = InvariantDataProvider;
/// let plural_provider = InvariantDataProvider;
///
/// let options = length::Bag {
///     date: Some(length::Date::Medium),
///     time: Some(length::Time::Short),
///     ..Default::default()
/// };
/// let zdtf = ZonedDateTimeFormat::<Gregorian>::try_new(locale!("en"), &date_provider, &zone_provider, &plural_provider, &options.into(), &TimeZoneFormatOptions::default())
///     .expect("Failed to create DateTimeFormat instance.");
///
///
/// let zoned_datetime: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
///     .parse()
///     .expect("Failed to parse zoned datetime");
///
/// let value = zdtf.format_to_string(&zoned_datetime);
/// ```
pub struct ZonedDateTimeFormat<C>(raw::ZonedDateTimeFormat, PhantomData<C>);

impl<C: CldrCalendar> ZonedDateTimeFormat<C> {
    /// Constructor that takes a selected [`Locale`], a reference to a [data provider] for
    /// dates, a [data provider] for time zones, and a list of [`DateTimeFormatOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{ZonedDateTimeFormat, DateTimeFormatOptions};
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu_datetime::TimeZoneFormatOptions;
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let date_provider = InvariantDataProvider;
    /// let zone_provider = InvariantDataProvider;
    /// let plural_provider = InvariantDataProvider;
    ///
    /// let options = DateTimeFormatOptions::default();
    ///
    /// let zdtf = ZonedDateTimeFormat::<Gregorian>::try_new(locale!("en"), &date_provider, &zone_provider, &plural_provider, &options, &TimeZoneFormatOptions::default());
    ///
    /// assert_eq!(zdtf.is_ok(), true);
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new<L, DP, ZP, PP>(
        locale: L,
        date_provider: &DP,
        zone_provider: &ZP,
        plural_provider: &PP,
        date_time_format_options: &DateTimeFormatOptions,
        time_zone_format_options: &TimeZoneFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        L: Into<Locale>,
        DP: ResourceProvider<DateSymbolsV1Marker>
            + ResourceProvider<DatePatternsV1Marker>
            + ResourceProvider<DateSkeletonPatternsV1Marker>
            + ResourceProvider<WeekDataV1Marker>,
        ZP: ResourceProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + ResourceProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesShortV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesShortV1Marker>
            + ?Sized,
        PP: ResourceProvider<OrdinalV1Marker>,
    {
        let mut locale = locale.into();
        // TODO(#419): Resolve the locale calendar with the API calendar.
        locale
            .extensions
            .unicode
            .keywords
            .set(unicode_ext_key!("ca"), C::IDENTIFIER);
        Ok(Self(
            raw::ZonedDateTimeFormat::try_new(
                locale,
                date_provider,
                zone_provider,
                plural_provider,
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
    /// use icu::datetime::ZonedDateTimeFormat;
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu_datetime::TimeZoneFormatOptions;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale = icu::locid::locale!("en");
    /// # let date_provider = InvariantDataProvider;
    /// # let zone_provider = InvariantDataProvider;
    /// # let plural_provider = InvariantDataProvider;
    /// # let options = icu::datetime::DateTimeFormatOptions::default();
    /// let zdtf = ZonedDateTimeFormat::<Gregorian>::try_new(locale, &date_provider, &zone_provider, &plural_provider, &options, &TimeZoneFormatOptions::default())
    ///     .expect("Failed to create ZonedDateTimeFormat instance.");
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
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedZonedDateTime<'l, T>
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
    /// use icu::datetime::ZonedDateTimeFormat;
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu_datetime::TimeZoneFormatOptions;
    /// # use icu_provider::inv::InvariantDataProvider;
    /// # let locale = icu::locid::locale!("en");
    /// # let date_provider = InvariantDataProvider;
    /// # let zone_provider = InvariantDataProvider;
    /// # let plural_provider = InvariantDataProvider;
    /// # let options = icu::datetime::DateTimeFormatOptions::default();
    /// let zdtf = ZonedDateTimeFormat::<Gregorian>::try_new(locale, &date_provider, &zone_provider, &plural_provider, &options.into(), &TimeZoneFormatOptions::default())
    ///     .expect("Failed to create ZonedDateTimeFormat instance.");
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
    /// use icu::datetime::ZonedDateTimeFormat;
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu_datetime::TimeZoneFormatOptions;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale = icu::locid::locale!("en");
    /// # let date_provider = InvariantDataProvider;
    /// # let zone_provider = InvariantDataProvider;
    /// # let plural_provider = InvariantDataProvider;
    /// # let options = icu::datetime::DateTimeFormatOptions::default();
    /// let zdtf = ZonedDateTimeFormat::<Gregorian>::try_new(locale, &date_provider, &zone_provider, &plural_provider, &options.into(), &TimeZoneFormatOptions::default())
    ///     .expect("Failed to create ZonedDateTimeFormat instance.");
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
