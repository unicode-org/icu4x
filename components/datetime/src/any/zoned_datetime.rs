// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{calendar, options::DateTimeFormatterOptions, raw};
use alloc::string::String;

use icu_provider::prelude::*;

use crate::input::{DateTimeInput, ExtractedDateTimeInput, TimeZoneInput};
use crate::provider::{self, calendar::*, date_time::PatternSelector};
use crate::time_zone::TimeZoneFormatterOptions;
use crate::{DateTimeError, FormattedZonedDateTime};
use icu_calendar::any_calendar::{AnyCalendar, AnyCalendarKind};
use icu_calendar::provider::{
    JapaneseErasV1Marker, JapaneseExtendedErasV1Marker, WeekDataV1Marker,
};
use icu_calendar::{types::Time, DateTime};
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;
use writeable::Writeable;

/// [`ZonedDateTimeFormatter`] is a formatter capable of formatting
/// date/times with time zones from any calendar, selected at runtime. For the difference between this and [`TypedZonedDateTimeFormatter`](crate::TypedZonedDateTimeFormatter),
/// please read the [crate root docs][crate].
///
/// This is equivalently the composition of
/// [`DateTimeFormatter`](crate::DateTimeFormatter) and [`TimeZoneFormatter`].
///
/// [`ZonedDateTimeFormatter`] uses data from the [data provider]s, the selected [`DataLocale`], and the
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
/// Using a GMT time zone:
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length, ZonedDateTimeFormatter};
/// use icu::locid::locale;
/// use icu::timezone::CustomTimeZone;
/// use writeable::assert_writeable_eq;
///
/// let options = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Long,
/// );
/// let zdtf = ZonedDateTimeFormatter::try_new_unstable(
///     &icu_testdata::unstable(),
///     &locale!("en").into(),
///     options.into(),
///     Default::default(),
/// )
/// .expect("Failed to create ZonedDateTimeFormatter instance.");
///
/// let datetime = DateTime::try_new_iso_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
/// let any_datetime = datetime.to_any();
///
/// let time_zone = CustomTimeZone::utc();
///
/// assert_writeable_eq!(
///     zdtf.format(&any_datetime, &time_zone)
///         .expect("Calendars should match"),
///     "Sep 1, 2020, 12:34:28 PM GMT"
/// );
/// ```
///
/// Using a non-GMT time zone, specified by id:
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length, ZonedDateTimeFormatter};
/// use icu::locid::locale;
/// use icu::timezone::{CustomTimeZone, GmtOffset, MetazoneCalculator, ZoneVariant};
/// use tinystr::TinyAsciiStr;
/// use writeable::assert_writeable_eq;
///
/// let options = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Full,
/// );
/// let zdtf = ZonedDateTimeFormatter::try_new_unstable(
///     &icu_testdata::unstable(),
///     &locale!("en").into(),
///     options.into(),
///     Default::default(),
/// )
/// .expect("Failed to create ZonedDateTimeFormatter instance.");
///
/// // Create a DateTime at September 1, 2020 at 12:34:28 PM
/// let datetime = DateTime::try_new_iso_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
/// let any_datetime = datetime.to_any();
///
/// // Create a time zone for America/Chicago at GMT-6:
/// let mut time_zone = CustomTimeZone::new_empty();
/// time_zone.gmt_offset = "-06:00".parse::<GmtOffset>().ok();
/// time_zone.time_zone_id = "uschi".parse::<TinyAsciiStr<8>>().ok().map(Into::into);
/// time_zone.zone_variant = Some(ZoneVariant::daylight());
///
/// // Compute the metazone during `datetime` (September 1, 2020 at 12:34:28 PM):
/// let mzc = MetazoneCalculator::try_new_unstable(&icu_testdata::unstable()).unwrap();
/// time_zone.maybe_calculate_metazone(&mzc, &datetime);
///
/// assert_writeable_eq!(
///     zdtf
///       .format(&any_datetime, &time_zone)
///       .expect("Calendars should match"),
///     "Sep 1, 2020, 12:34:28 PM Central Daylight Time");
/// ```
///
/// [`TimeZoneFormatter`]: crate::time_zone::TimeZoneFormatter
#[derive(Debug)]
pub struct ZonedDateTimeFormatter(raw::ZonedDateTimeFormatter, AnyCalendar);

impl ZonedDateTimeFormatter {
    /// Constructor that takes a selected [`DataLocale`], a reference to a [data provider] for
    /// dates, a [data provider] for time zones, a [data provider] for calendars, and a list of [`DateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// This method is **unstable**, more bounds may be added in the future as calendar support is added. It is
    /// preferable to use a provider that implements `DataProvider<D>` for all `D`, and ensure data is loaded as
    /// appropriate. The [`Self::try_new_with_buffer_provider()`], [`Self::try_new_with_any_provider()`] constructors
    /// may also be used if compile stability is desired.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
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
    /// use icu::datetime::options::components;
    /// use icu::datetime::{DateTimeFormatterOptions, ZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu::timezone::CustomTimeZone;
    /// use icu_provider::AsDeserializingBufferProvider;
    /// use std::str::FromStr;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut options = components::Bag::default();
    /// options.year = Some(components::Year::Numeric);
    /// options.month = Some(components::Month::Long);
    /// options.hour = Some(components::Numeric::Numeric);
    /// options.minute = Some(components::Numeric::Numeric);
    /// options.time_zone_name = Some(components::TimeZoneName::GmtOffset);
    ///
    /// let zdtf = ZonedDateTimeFormatter::try_new_experimental_unstable(
    ///     &icu_testdata::buffer().as_deserializing(),
    ///     &locale!("en-u-ca-gregory").into(),
    ///     options.into(),
    ///     Default::default(),
    /// )
    /// .expect("Construction should succeed");
    ///
    /// let datetime =
    ///     DateTime::try_new_iso_datetime(2021, 04, 08, 16, 12, 37).unwrap();
    /// let time_zone = CustomTimeZone::from_str("-07:00").unwrap();
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_writeable_eq!(
    ///     zdtf.format(&any_datetime, &time_zone).unwrap(),
    ///     "April 2021, 16:12 GMT-07:00"
    /// );
    /// ```
    ///
    /// [data provider]: icu_provider
    #[cfg(feature = "experimental")]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn try_new_experimental_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        date_time_format_options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: DataProvider<TimeSymbolsV1Marker>
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
            + DataProvider<GregorianDateLengthsV1Marker>
            + DataProvider<BuddhistDateLengthsV1Marker>
            + DataProvider<JapaneseDateLengthsV1Marker>
            + DataProvider<JapaneseExtendedDateLengthsV1Marker>
            + DataProvider<CopticDateLengthsV1Marker>
            + DataProvider<IndianDateLengthsV1Marker>
            + DataProvider<EthiopianDateLengthsV1Marker>
            + DataProvider<GregorianDateSymbolsV1Marker>
            + DataProvider<BuddhistDateSymbolsV1Marker>
            + DataProvider<JapaneseDateSymbolsV1Marker>
            + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
            + DataProvider<CopticDateSymbolsV1Marker>
            + DataProvider<IndianDateSymbolsV1Marker>
            + DataProvider<EthiopianDateSymbolsV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            + ?Sized,
    {
        let calendar = AnyCalendar::try_new_for_locale_unstable(provider, locale)?;
        let kind = calendar.kind();

        let patterns = PatternSelector::for_options_experimental(
            provider,
            calendar::load_lengths_for_any_calendar_kind(provider, locale, kind)?,
            locale,
            &kind.as_bcp47_value(),
            &date_time_format_options,
        )?;

        Ok(Self(
            raw::ZonedDateTimeFormatter::try_new(
                provider,
                patterns,
                || calendar::load_symbols_for_any_calendar_kind(provider, locale, kind),
                locale,
                time_zone_format_options,
            )?,
            calendar,
        ))
    }

    /// Constructor that takes a selected [`DataLocale`], a reference to a [data provider] for
    /// dates, a [data provider] for time zones, a [data provider] for calendars, and a list of [`DateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// This method is **unstable**, more bounds may be added in the future as calendar support is added. It is
    /// preferable to use a provider that implements `DataProvider<D>` for all `D`, and ensure data is loaded as
    /// appropriate. The [`Self::try_new_with_buffer_provider()`], [`Self::try_new_with_any_provider()`] constructors
    /// may also be used if compile stability is desired.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::options::length;
    /// use icu::datetime::time_zone::TimeZoneFormatterOptions;
    /// use icu::datetime::{DateTimeFormatterOptions, ZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu::timezone::CustomTimeZone;
    /// use std::str::FromStr;
    /// use writeable::assert_writeable_eq;
    ///
    /// let options = length::Bag::from_date_time_style(
    ///     length::Date::Medium,
    ///     length::Time::Long,
    /// );
    /// let locale = locale!("en-u-ca-gregory");
    ///
    /// let zdtf = ZonedDateTimeFormatter::try_new_unstable(
    ///     &icu_testdata::unstable(),
    ///     &locale.into(),
    ///     options.into(),
    ///     TimeZoneFormatterOptions::default(),
    /// )
    /// .expect("Construction should succeed");
    ///
    /// let datetime =
    ///     DateTime::try_new_iso_datetime(2021, 04, 08, 16, 12, 37).unwrap();
    /// let time_zone = CustomTimeZone::from_str("-07:00").unwrap();
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_writeable_eq!(
    ///     zdtf.format(&any_datetime, &time_zone).unwrap(),
    ///     "Apr 8, 2021, 4:12:37 PM GMT-07:00"
    /// );
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        date_time_format_options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: DataProvider<TimeSymbolsV1Marker>
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
            + DataProvider<GregorianDateLengthsV1Marker>
            + DataProvider<BuddhistDateLengthsV1Marker>
            + DataProvider<JapaneseDateLengthsV1Marker>
            + DataProvider<JapaneseExtendedDateLengthsV1Marker>
            + DataProvider<CopticDateLengthsV1Marker>
            + DataProvider<IndianDateLengthsV1Marker>
            + DataProvider<EthiopianDateLengthsV1Marker>
            + DataProvider<GregorianDateSymbolsV1Marker>
            + DataProvider<BuddhistDateSymbolsV1Marker>
            + DataProvider<JapaneseDateSymbolsV1Marker>
            + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
            + DataProvider<CopticDateSymbolsV1Marker>
            + DataProvider<IndianDateSymbolsV1Marker>
            + DataProvider<EthiopianDateSymbolsV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            + ?Sized,
    {
        let calendar = AnyCalendar::try_new_for_locale_unstable(provider, locale)?;
        let kind = calendar.kind();

        let patterns = PatternSelector::for_options(
            provider,
            calendar::load_lengths_for_any_calendar_kind(provider, locale, kind)?,
            locale,
            &date_time_format_options,
        )?;

        Ok(Self(
            raw::ZonedDateTimeFormatter::try_new(
                provider,
                patterns,
                || calendar::load_symbols_for_any_calendar_kind(provider, locale, kind),
                locale,
                time_zone_format_options,
            )?,
            calendar,
        ))
    }

    /// Construct a new [`ZonedDateTimeFormatter`] from a data provider that implements
    /// [`AnyProvider`].
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// The provider must be able to provide data for the following keys: `datetime/symbols@1`, `datetime/timelengths@1`,
    /// `datetime/timelengths@1`, `datetime/symbols@1`, `datetime/skeletons@1`, `datetime/week_data@1`, `plurals/ordinals@1`,
    /// `time_zone/formats@1`, `time_zone/exemplar_cities@1`, `time_zone/generic_long@1`, `time_zone/generic_short@1`,
    /// `time_zone/specific_long@1`, `time_zone/specific_short@1`, `time_zone/metazone_period@1`.
    ///
    /// Furthermore, based on the type of calendar used, one of the following data keys may be necessary:
    ///
    /// - `u-ca-japanese` (Japanese calendar): `calendar/japanese@1`
    ///
    /// Test will currently fail due to <https://github.com/unicode-org/icu4x/issues/2188>,
    /// since these functions currently *must* be given a fallback-enabled provider and
    /// we do not have one in `icu_testdata`
    ///
    /// ```ignore
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::options::length;
    /// use icu::datetime::{DateTimeFormatterOptions, ZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu::timezone::CustomTimeZone;
    /// use std::str::FromStr;
    /// use writeable::assert_writeable_eq;
    ///
    /// let options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Long).into();
    /// let locale = locale!("en-u-ca-gregory");
    ///
    /// let zdtf = ZonedDateTimeFormatter::try_new_with_any_provider(
    ///     &icu_testdata::any(),
    ///     &locale.into(),
    ///     options,
    ///     Default::default(),
    /// )
    /// .expect("Construction should succeed");
    ///
    /// let datetime = DateTime::try_new_iso_datetime(2021, 04, 08, 16, 12, 37).unwrap();
    /// let time_zone = CustomTimeZone::from_str("-07:00").unwrap();
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_writeable_eq!(
    ///     zdtf.format(&any_datetime, &time_zone).unwrap(),
    ///     "Apr 8, 2021, 4:12:37 PM GMT-07:00"
    /// );
    /// ```
    #[inline]
    pub fn try_new_with_any_provider<P>(
        data_provider: &P,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: AnyProvider,
    {
        let downcasting = data_provider.as_downcasting();
        Self::try_new_unstable(&downcasting, locale, options, time_zone_format_options)
    }

    /// Construct a new [`ZonedDateTimeFormatter`] from a data provider that implements
    /// [`BufferProvider`].
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// The provider must be able to provide data for the following keys: `datetime/symbols@1`, `datetime/timelengths@1`,
    /// `datetime/timelengths@1`, `datetime/symbols@1`, `datetime/skeletons@1`, `datetime/week_data@1`, `plurals/ordinals@1`,
    /// `time_zone/formats@1`, `time_zone/exemplar_cities@1`, `time_zone/generic_long@1`, `time_zone/generic_short@1`,
    /// `time_zone/specific_long@1`, `time_zone/specific_short@1`, `time_zone/metazone_period@1`.
    ///
    /// Furthermore, based on the type of calendar used, one of the following data keys may be necessary:
    ///
    /// - `u-ca-japanese` (Japanese calendar): `calendar/japanese@1`
    ///
    /// ```rust
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::options::length;
    /// use icu::datetime::{DateTimeFormatterOptions, ZonedDateTimeFormatter};
    /// use icu::locid::locale;
    /// use icu::timezone::CustomTimeZone;
    /// use std::str::FromStr;
    /// use writeable::assert_writeable_eq;
    ///
    /// let options = length::Bag::from_date_time_style(
    ///     length::Date::Medium,
    ///     length::Time::Long,
    /// )
    /// .into();
    /// let locale = locale!("en");
    ///
    /// let zdtf = ZonedDateTimeFormatter::try_new_with_buffer_provider(
    ///     &icu_testdata::buffer(),
    ///     &locale.into(),
    ///     options,
    ///     Default::default(),
    /// )
    /// .expect("Construction should succeed");
    ///
    /// let datetime =
    ///     DateTime::try_new_iso_datetime(2021, 04, 08, 16, 12, 37).unwrap();
    /// let time_zone = CustomTimeZone::from_str("-07:00").unwrap();
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_writeable_eq!(
    ///     zdtf.format(&any_datetime, &time_zone).unwrap(),
    ///     "Apr 8, 2021, 4:12:37 PM GMT-07:00"
    /// );
    /// ```
    #[inline]
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider<P>(
        data_provider: &P,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: BufferProvider,
    {
        let deserializing = data_provider.as_deserializing();
        Self::try_new_unstable(&deserializing, locale, options, time_zone_format_options)
    }

    /// Takes a [`DateTimeInput`] and a [`TimeZoneInput`] and returns an instance of a [`FormattedZonedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format<'l>(
        &'l self,
        date: &impl DateTimeInput<Calendar = AnyCalendar>,
        time_zone: &impl TimeZoneInput,
    ) -> Result<FormattedZonedDateTime<'l>, DateTimeError> {
        if let Some(converted) = self.convert_if_necessary(date)? {
            Ok(self.0.format(&converted, time_zone))
        } else {
            Ok(self.0.format(date, time_zone))
        }
    }

    /// Takes a [`DateTimeInput`] and a [`TimeZoneInput`] and returns it formatted as a string.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format_to_string(
        &self,
        date: &impl DateTimeInput<Calendar = AnyCalendar>,
        time_zone: &impl TimeZoneInput,
    ) -> Result<String, DateTimeError> {
        Ok(self.format(date, time_zone)?.write_to_string().into_owned())
    }

    /// Converts a date to the correct calendar if necessary
    ///
    /// Returns Err if the date is not ISO or compatible with the current calendar, returns Ok(None)
    /// if the date is compatible with the current calendar and doesn't need conversion
    fn convert_if_necessary(
        &self,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<Option<ExtractedDateTimeInput>, DateTimeError> {
        let this_calendar = self.1.kind();
        let date_calendar = value.any_calendar_kind();
        if Some(this_calendar) != date_calendar {
            if date_calendar != Some(AnyCalendarKind::Iso) {
                return Err(DateTimeError::MismatchedAnyCalendar(
                    this_calendar,
                    date_calendar,
                ));
            }
            let date = value.to_iso();
            let time = Time::new(
                value.hour().unwrap_or_default(),
                value.minute().unwrap_or_default(),
                value.second().unwrap_or_default(),
                value.nanosecond().unwrap_or_default(),
            );
            let datetime = DateTime::new(date, time).to_any();
            let converted = self.1.convert_any_datetime(&datetime);
            // FIXME(#2145) this is very hacky, can be improved after we improve ZonedDateTimeInput
            let converted = ExtractedDateTimeInput::extract_from(&converted);
            Ok(Some(converted))
        } else {
            Ok(None)
        }
    }
}
