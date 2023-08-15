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
/// let zdtf = ZonedDateTimeFormatter::try_new(
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
/// let zdtf = ZonedDateTimeFormatter::try_new(
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
/// let mzc = MetazoneCalculator::new();
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
    /// Constructor that takes a selected [`DataLocale`] and a list of [`DateTimeFormatterOptions`] and uses compiled data.
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
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
    /// let zdtf = ZonedDateTimeFormatter::try_new_experimental(
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
    #[cfg(feature = "experimental")]
    #[cfg(feature = "compiled_data")]
    pub fn try_new_experimental(
        locale: &DataLocale,
        date_time_format_options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError> {
        let calendar = AnyCalendar::new_for_locale(locale);
        let kind = calendar.kind();

        let patterns = PatternSelector::for_options_experimental(
            &crate::provider::Baked,
            calendar::load_lengths_for_any_calendar_kind(&crate::provider::Baked, locale, kind)?,
            locale,
            &kind.as_bcp47_value(),
            &date_time_format_options,
        )?;

        Ok(Self(
            raw::ZonedDateTimeFormatter::try_new(
                patterns,
                || {
                    calendar::load_symbols_for_any_calendar_kind(
                        &crate::provider::Baked,
                        locale,
                        kind,
                    )
                },
                locale,
                time_zone_format_options,
            )?,
            calendar,
        ))
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_experimental)]
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
            + DataProvider<BuddhistDateLengthsV1Marker>
            + DataProvider<BuddhistDateSymbolsV1Marker>
            + DataProvider<ChineseDateLengthsV1Marker>
            + DataProvider<ChineseDateSymbolsV1Marker>
            + DataProvider<CopticDateLengthsV1Marker>
            + DataProvider<CopticDateSymbolsV1Marker>
            + DataProvider<DangiDateLengthsV1Marker>
            + DataProvider<DangiDateSymbolsV1Marker>
            + DataProvider<EthiopianDateLengthsV1Marker>
            + DataProvider<EthiopianDateSymbolsV1Marker>
            + DataProvider<GregorianDateLengthsV1Marker>
            + DataProvider<GregorianDateSymbolsV1Marker>
            + DataProvider<HebrewDateLengthsV1Marker>
            + DataProvider<HebrewDateSymbolsV1Marker>
            + DataProvider<IndianDateLengthsV1Marker>
            + DataProvider<IndianDateSymbolsV1Marker>
            + DataProvider<IslamicDateLengthsV1Marker>
            + DataProvider<IslamicDateSymbolsV1Marker>
            + DataProvider<JapaneseDateLengthsV1Marker>
            + DataProvider<JapaneseDateSymbolsV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedDateLengthsV1Marker>
            + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            + DataProvider<PersianDateLengthsV1Marker>
            + DataProvider<PersianDateSymbolsV1Marker>
            + DataProvider<RocDateLengthsV1Marker>
            + DataProvider<RocDateSymbolsV1Marker>
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
            raw::ZonedDateTimeFormatter::try_new_unstable(
                provider,
                patterns,
                || calendar::load_symbols_for_any_calendar_kind(provider, locale, kind),
                locale,
                time_zone_format_options,
            )?,
            calendar,
        ))
    }

    /// Constructor that takes a selected [`DataLocale`] and a list of [`DateTimeFormatterOptions`] and uses compiled data.
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
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
    /// let zdtf = ZonedDateTimeFormatter::try_new(
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
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        locale: &DataLocale,
        date_time_format_options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError> {
        let calendar = AnyCalendar::new_for_locale(locale);
        let kind = calendar.kind();

        let patterns = PatternSelector::for_options(
            &crate::provider::Baked,
            calendar::load_lengths_for_any_calendar_kind(&crate::provider::Baked, locale, kind)?,
            locale,
            &date_time_format_options,
        )?;

        Ok(Self(
            raw::ZonedDateTimeFormatter::try_new(
                patterns,
                || {
                    calendar::load_symbols_for_any_calendar_kind(
                        &crate::provider::Baked,
                        locale,
                        kind,
                    )
                },
                locale,
                time_zone_format_options,
            )?,
            calendar,
        ))
    }

    #[inline]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
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
            + DataProvider<BuddhistDateLengthsV1Marker>
            + DataProvider<BuddhistDateSymbolsV1Marker>
            + DataProvider<ChineseDateLengthsV1Marker>
            + DataProvider<ChineseDateSymbolsV1Marker>
            + DataProvider<CopticDateLengthsV1Marker>
            + DataProvider<CopticDateSymbolsV1Marker>
            + DataProvider<DangiDateLengthsV1Marker>
            + DataProvider<DangiDateSymbolsV1Marker>
            + DataProvider<EthiopianDateLengthsV1Marker>
            + DataProvider<EthiopianDateSymbolsV1Marker>
            + DataProvider<GregorianDateLengthsV1Marker>
            + DataProvider<GregorianDateSymbolsV1Marker>
            + DataProvider<HebrewDateLengthsV1Marker>
            + DataProvider<HebrewDateSymbolsV1Marker>
            + DataProvider<IndianDateLengthsV1Marker>
            + DataProvider<IndianDateSymbolsV1Marker>
            + DataProvider<IslamicDateLengthsV1Marker>
            + DataProvider<IslamicDateSymbolsV1Marker>
            + DataProvider<JapaneseDateLengthsV1Marker>
            + DataProvider<JapaneseDateSymbolsV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedDateLengthsV1Marker>
            + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            + DataProvider<PersianDateLengthsV1Marker>
            + DataProvider<PersianDateSymbolsV1Marker>
            + DataProvider<RocDateLengthsV1Marker>
            + DataProvider<RocDateSymbolsV1Marker>
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
            raw::ZonedDateTimeFormatter::try_new_unstable(
                provider,
                patterns,
                || calendar::load_symbols_for_any_calendar_kind(provider, locale, kind),
                locale,
                time_zone_format_options,
            )?,
            calendar,
        ))
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::try_new)]
    #[inline]
    pub fn try_new_with_any_provider(
        provider: &impl AnyProvider,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError> {
        let downcasting = provider.as_downcasting();
        Self::try_new_unstable(&downcasting, locale, options, time_zone_format_options)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::try_new)]
    #[inline]
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider(
        provider: &impl BufferProvider,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError> {
        let deserializing = provider.as_deserializing();
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

#[test]
#[cfg(feature = "serde")]
fn buffer_constructor() {
    #![allow(clippy::zero_prefixed_literal)]
    use icu::calendar::DateTime;
    use icu::datetime::options::length;
    use icu::datetime::ZonedDateTimeFormatter;
    use icu::locid::locale;
    use icu::timezone::CustomTimeZone;
    use std::str::FromStr;
    use writeable::assert_writeable_eq;

    let provider = icu_provider_blob::BlobDataProvider::try_new_from_static_blob(include_bytes!(
        "../../tests/data/blob.postcard"
    ))
    .unwrap();

    let zdtf = ZonedDateTimeFormatter::try_new_with_buffer_provider(
        &provider,
        &locale!("en").into(),
        length::Bag::from_date_time_style(length::Date::Medium, length::Time::Long).into(),
        Default::default(),
    )
    .unwrap();

    assert_writeable_eq!(
        zdtf.format(
            &DateTime::try_new_iso_datetime(2021, 04, 08, 16, 12, 37)
                .unwrap()
                .to_any(),
            &CustomTimeZone::from_str("-07:00").unwrap()
        )
        .unwrap(),
        "Apr 8, 2021, 4:12:37 PM GMT-07:00"
    );
}
