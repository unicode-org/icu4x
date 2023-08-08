// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use core::marker::PhantomData;
use icu_calendar::provider::WeekDataV1Marker;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;
use writeable::Writeable;

use crate::{
    calendar,
    format::zoned_datetime::FormattedZonedDateTime,
    input::{DateTimeInput, TimeZoneInput},
    options::DateTimeFormatterOptions,
    provider::{
        self,
        calendar::{TimeLengthsV1Marker, TimeSymbolsV1Marker},
        date_time::PatternSelector,
    },
    raw,
    time_zone::TimeZoneFormatterOptions,
    CldrCalendar, DateTimeError,
};

/// The composition of [`TypedDateTimeFormatter`](crate::TypedDateTimeFormatter) and [`TimeZoneFormatter`].
///
/// [`TypedZonedDateTimeFormatter`] is a formatter capable of formatting
/// date/times with time zones from a calendar selected at compile time. For the difference between this
/// and [`DateTimeFormatter`](crate::DateTimeFormatter), please read the [crate root docs][crate].
///
/// [`TypedZonedDateTimeFormatter`] uses data from the [data provider]s, the selected locale, and the
/// provided pattern to collect all data necessary to format a datetime with time zones into that locale.
///
/// The various pattern symbols specified in UTS-35 require different sets of data for formatting.
/// As such, [`TimeZoneFormatter`] will pull in only the resources it needs to format that pattern
/// that is derived from the provided [`DateTimeFormatterOptions`].
///
/// For that reason, one should think of the process of formatting a zoned datetime in two steps:
/// first, a computationally heavy construction of [`TypedZonedDateTimeFormatter`], and then fast formatting
/// of the data using the instance.
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::time_zone::TimeZoneFormatterOptions;
/// use icu::datetime::{options::length, TypedZonedDateTimeFormatter};
/// use icu::locid::locale;
/// use icu::timezone::CustomTimeZone;
/// use std::str::FromStr;
/// use writeable::assert_writeable_eq;
///
/// let options = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Long,
/// );
/// let zdtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
///     &locale!("en").into(),
///     options.into(),
///     TimeZoneFormatterOptions::default(),
/// )
/// .expect("Failed to create TypedDateTimeFormatter instance.");
///
/// let datetime =
///     DateTime::try_new_gregorian_datetime(2020, 9, 12, 12, 34, 28).unwrap();
/// let time_zone = CustomTimeZone::from_str("-07:00").unwrap();
///
/// let formatted_date = zdtf.format(&datetime, &time_zone);
///
/// assert_writeable_eq!(formatted_date, "Sep 12, 2020, 12:34:28 PM GMT-07:00");
/// ```
///
/// [`TimeZoneFormatter`]: crate::time_zone::TimeZoneFormatter
#[derive(Debug)]
pub struct TypedZonedDateTimeFormatter<C>(raw::ZonedDateTimeFormatter, PhantomData<C>);

impl<C: CldrCalendar> TypedZonedDateTimeFormatter<C> {
    /// Constructor that takes a selected locale and a list of [`DateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
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
    /// use icu::calendar::{Gregorian, DateTime};
    /// use icu::datetime::{options::components, TypedZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu::datetime::time_zone::TimeZoneFormatterOptions;
    /// use icu::timezone::CustomTimeZone;
    /// use icu_provider::AsDeserializingBufferProvider;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut options = components::Bag::default();
    /// options.year = Some(components::Year::Numeric);
    /// options.month = Some(components::Month::Long);
    /// options.hour = Some(components::Numeric::Numeric);
    /// options.minute = Some(components::Numeric::Numeric);
    /// options.time_zone_name = Some(components::TimeZoneName::GmtOffset);
    ///
    /// let zdtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new_experimental(
    ///     &locale!("en").into(),
    ///     options.into(),
    ///     TimeZoneFormatterOptions::default(),
    /// ).unwrap();
    ///
    /// let datetime = DateTime::try_new_gregorian_datetime(2022, 8, 31, 1, 2, 3).unwrap();
    ///
    /// assert_writeable_eq!(
    ///     zdtf.format(&datetime, &CustomTimeZone::utc()),
    ///     "August 2022, 01:02 GMT",
    /// );
    /// ```
    ///
    /// [data provider]: icu_provider
    #[cfg(feature = "experimental")]
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub fn try_new_experimental(
        locale: &DataLocale,
        date_time_format_options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        crate::provider::Baked:
            DataProvider<C::DateLengthsV1Marker> + DataProvider<C::DateSymbolsV1Marker>,
    {
        let patterns = PatternSelector::for_options_experimental(
            &crate::provider::Baked,
            calendar::load_lengths_for_cldr_calendar::<C, _>(&crate::provider::Baked, locale)?,
            locale,
            &C::DEFAULT_BCP_47_IDENTIFIER,
            &date_time_format_options,
        )?;
        Ok(Self(
            raw::ZonedDateTimeFormatter::try_new(
                patterns,
                || {
                    calendar::load_symbols_for_cldr_calendar::<C, _>(
                        &crate::provider::Baked,
                        locale,
                    )
                },
                locale,
                time_zone_format_options,
            )?,
            PhantomData,
        ))
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_experimental)]
    #[cfg(feature = "experimental")]
    #[inline]
    pub fn try_new_experimental_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        date_time_format_options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>
            + DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<crate::provider::calendar::DateSkeletonPatternsV1Marker>
            + DataProvider<WeekDataV1Marker>
            + DataProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + DataProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + DataProvider<provider::time_zones::MetazoneGenericNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetazoneGenericNamesShortV1Marker>
            + DataProvider<provider::time_zones::MetazoneSpecificNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetazoneSpecificNamesShortV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        let patterns = PatternSelector::for_options_experimental(
            provider,
            calendar::load_lengths_for_cldr_calendar::<C, _>(provider, locale)?,
            locale,
            &C::DEFAULT_BCP_47_IDENTIFIER,
            &date_time_format_options,
        )?;
        Ok(Self(
            raw::ZonedDateTimeFormatter::try_new_unstable(
                provider,
                patterns,
                || calendar::load_symbols_for_cldr_calendar::<C, _>(provider, locale),
                locale,
                time_zone_format_options,
            )?,
            PhantomData,
        ))
    }

    /// Constructor that takes a selected locale and a list of [`DateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::time_zone::TimeZoneFormatterOptions;
    /// use icu::datetime::{options::length, TypedZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu::timezone::CustomTimeZone;
    /// use writeable::assert_writeable_eq;
    ///
    /// let options = length::Bag::from_date_time_style(
    ///     length::Date::Medium,
    ///     length::Time::Long,
    /// );
    ///
    /// let zdtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     &locale!("en").into(),
    ///     options.into(),
    ///     TimeZoneFormatterOptions::default(),
    /// )
    /// .unwrap();
    ///
    /// let datetime =
    ///     DateTime::try_new_gregorian_datetime(2022, 8, 31, 1, 2, 3).unwrap();
    ///
    /// assert_writeable_eq!(
    ///     zdtf.format(&datetime, &CustomTimeZone::utc()),
    ///     "Aug 31, 2022, 1:02:03 AM GMT",
    /// );
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        locale: &DataLocale,
        date_time_format_options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        crate::provider::Baked:
            DataProvider<C::DateLengthsV1Marker> + DataProvider<C::DateSymbolsV1Marker>,
    {
        let patterns = PatternSelector::for_options(
            &crate::provider::Baked,
            calendar::load_lengths_for_cldr_calendar::<C, _>(&crate::provider::Baked, locale)?,
            locale,
            &date_time_format_options,
        )?;
        Ok(Self(
            raw::ZonedDateTimeFormatter::try_new(
                patterns,
                || {
                    calendar::load_symbols_for_cldr_calendar::<C, _>(
                        &crate::provider::Baked,
                        locale,
                    )
                },
                locale,
                time_zone_format_options,
            )?,
            PhantomData,
        ))
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        date_time_format_options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
        error: DateTimeError,
        #[cfg(skip)]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    #[inline]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        date_time_format_options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>
            + DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<WeekDataV1Marker>
            + DataProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + DataProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + DataProvider<provider::time_zones::MetazoneGenericNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetazoneGenericNamesShortV1Marker>
            + DataProvider<provider::time_zones::MetazoneSpecificNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetazoneSpecificNamesShortV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        let patterns = PatternSelector::for_options(
            provider,
            calendar::load_lengths_for_cldr_calendar::<C, _>(provider, locale)?,
            locale,
            &date_time_format_options,
        )?;
        Ok(Self(
            raw::ZonedDateTimeFormatter::try_new_unstable(
                provider,
                patterns,
                || calendar::load_symbols_for_cldr_calendar::<C, _>(provider, locale),
                locale,
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
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::length, TypedZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu::timezone::CustomTimeZone;
    /// use std::str::FromStr;
    /// use writeable::assert_writeable_eq;
    ///
    /// let options = length::Bag::from_date_time_style(
    ///     length::Date::Medium,
    ///     length::Time::Long,
    /// );
    ///
    /// let zdtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     &locale!("en").into(),
    ///     options.into(),
    ///     Default::default(),
    /// )
    /// .expect("Failed to create TypedZonedDateTimeFormatter instance.");
    ///
    /// let datetime =
    ///     DateTime::try_new_gregorian_datetime(2020, 9, 12, 12, 34, 28).unwrap();
    /// let time_zone = CustomTimeZone::from_str("-07:00").unwrap();
    ///
    /// let formatted_date = zdtf.format(&datetime, &time_zone);
    ///
    /// assert_writeable_eq!(formatted_date, "Sep 12, 2020, 12:34:28 PM GMT-07:00");
    /// ```
    #[inline]
    pub fn format<'l>(
        &'l self,
        date: &impl DateTimeInput<Calendar = C>,
        time_zone: &impl TimeZoneInput,
    ) -> FormattedZonedDateTime<'l> {
        self.0.format(date, time_zone)
    }

    /// Takes a [`DateTimeInput`] and a [`TimeZoneInput`] and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::{options::length, TypedZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu::timezone::CustomTimeZone;
    /// use std::str::FromStr;
    ///
    /// let options = length::Bag::from_date_time_style(
    ///     length::Date::Medium,
    ///     length::Time::Long,
    /// );
    ///
    /// let zdtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
    ///     &locale!("en").into(),
    ///     options.into(),
    ///     Default::default(),
    /// )
    /// .expect("Failed to create TypedZonedDateTimeFormatter instance.");
    ///
    /// let datetime =
    ///     DateTime::try_new_gregorian_datetime(2020, 9, 12, 12, 34, 28).unwrap();
    /// let time_zone = CustomTimeZone::from_str("-07:00").unwrap();
    ///
    /// let formatted_string = zdtf.format_to_string(&datetime, &time_zone);
    ///
    /// assert_eq!(formatted_string, "Sep 12, 2020, 12:34:28 PM GMT-07:00");
    /// ```
    #[inline]
    pub fn format_to_string(
        &self,
        date: &impl DateTimeInput<Calendar = C>,
        time_zone: &impl TimeZoneInput,
    ) -> String {
        self.format(date, time_zone).write_to_string().into_owned()
    }
}
