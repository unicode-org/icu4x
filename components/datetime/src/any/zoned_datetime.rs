// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{options::DateTimeFormatterOptions, raw};
use alloc::string::String;

use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value, Locale};

use icu_provider::prelude::*;

use crate::date::{ExtractedDateTimeInput, ExtractedZonedDateTimeInput, ZonedDateTimeInput};
use crate::provider::{
    self,
    calendar::{
        DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
        TimePatternsV1Marker, TimeSymbolsV1Marker,
    },
    week_data::WeekDataV1Marker,
};
use crate::time_zone::TimeZoneFormatterOptions;
use crate::{DateTimeFormatterError, FormattedZonedDateTime};
use icu_calendar::any_calendar::{AnyCalendar, AnyCalendarKind};
use icu_calendar::provider::JapaneseErasV1Marker;
use icu_calendar::{types::Time, DateTime};
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;

/// [`ZonedAnyDateTimeFormatter`] is a [`ZonedDateTimeFormatter`](crate::ZonedDateTimeFormatter) capable of formatting
/// dates from any calendar, selected at runtime.
///
/// This is equivalently the composition of
/// [`AnyDateTimeFormatter`](crate::any::AnyDateTimeFormatter) and [`TimeZoneFormatter`](crate::TimeZoneFormatter).
///
/// [`ZonedAnyDateTimeFormatter`] uses data from the [data provider]s, the selected [`Locale`], and the
/// provided pattern to collect all data necessary to format a datetime with time zones into that locale.
///
/// The various pattern symbols specified in UTS-35 require different sets of data for formatting.
/// As such, `TimeZoneFormatter` will pull in only the resources it needs to format that pattern
/// that is derived from the provided [`DateTimeFormatterOptions`].
///
/// For that reason, one should think of the process of formatting a zoned datetime in two steps:
/// first, a computationally heavy construction of [`ZonedAnyDateTimeFormatter`], and then fast formatting
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
///
/// let provider = icu_testdata::get_provider();
///
/// let options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
/// let zdtf = ZonedDateTimeFormatter::<Gregorian>::try_new(
///     locale!("en"),
///     &provider,
///     &provider,
///     &provider,
///     &provider,
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
pub struct ZonedAnyDateTimeFormatter(raw::ZonedDateTimeFormatter, AnyCalendar);

impl ZonedAnyDateTimeFormatter {
    /// Constructor that takes a selected [`Locale`], a reference to a [data provider] for
    /// dates, a [data provider] for time zones, a [data provider] for calendars, and a list of [`DateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// This method is **unstable**, more bounds may be added in the future as calendar support is added. It is
    /// preferable to use a provider that implements `ResourceProvider<D>` for all `D`, and ensure data is loaded as
    /// appropriate. The [`Self::try_new_with_buffer_provider()`], [`Self::try_new_with_any_provider()`] constructors
    /// may also be used if compile stability is desired.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu::datetime::{DateTimeFormatterOptions, any::ZonedAnyDateTimeFormatter};
    /// use icu::locid::Locale;
    /// use icu::datetime::TimeZoneFormatterOptions;
    /// use std::str::FromStr;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let options = DateTimeFormatterOptions::default();
    /// let locale = Locale::from_str("en-u-ca-gregory").unwrap();
    ///
    /// let zdtf = ZonedAnyDateTimeFormatter::try_new_unstable(
    ///     locale,
    ///     &provider,
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
    #[allow(clippy::too_many_arguments)]
    pub fn try_new_unstable<L, DP, ZP, PP, DEP, CEP>(
        locale: L,
        date_provider: &DP,
        zone_provider: &ZP,
        plural_provider: &PP,
        decimal_provider: &DEP,
        calendar_provider: &CEP,
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
        CEP: ResourceProvider<JapaneseErasV1Marker> + ?Sized,
    {
        let mut locale = locale.into();

        // TODO (#2038), DO NOT SHIP 1.0 without fixing this
        let kind = if let Some(kind) = AnyCalendarKind::from_locale(&locale) {
            kind
        } else {
            locale
                .extensions
                .unicode
                .keywords
                .set(key!("ca"), value!("gregory"));
            AnyCalendarKind::Gregorian
        };

        // We share data under ethiopic
        if kind == AnyCalendarKind::Ethioaa {
            locale
                .extensions
                .unicode
                .keywords
                .set(key!("ca"), value!("ethiopic"));
        }
        let calendar = AnyCalendar::try_new_unstable(kind, calendar_provider)?;

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
            calendar,
        ))
    }

    /// Construct a new [`ZonedAnyDateTimeFormatter`] from a data provider that implements
    /// [`AnyProvider`].
    ///
    /// The provider must be able to provide data for the following keys: `datetime/symbols@1`, `datetime/timelengths@1`,
    /// `datetime/timelengths@1`, `datetime/symbols@1`, `datetime/skeletons@1`, `datetime/week_data@1`, `plurals/ordinals@1`,
    /// `time_zone/formats@1`, `time_zone/exemplar_cities@1`, `time_zone/generic_long@1`, `time_zone/generic_short@1`,
    /// `time_zone/specific_long@1`, `time_zone/specific_short@1`, `time_zone/metazone_period@1`.
    ///
    /// Furthermore, based on the type of calendar used, one of the following data keys may be necessary:
    ///
    /// - `u-ca-japanese` (Japanese calendar): `calendar/japanese@1`
    #[inline]
    pub fn try_new_with_any_provider<T: Into<Locale>, P>(
        locale: T,
        data_provider: &P,
        options: &DateTimeFormatterOptions,
        time_zone_format_options: &TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        P: AnyProvider,
    {
        let downcasting = data_provider.as_downcasting();
        Self::try_new_unstable(
            locale,
            &downcasting,
            &downcasting,
            &downcasting,
            &downcasting,
            &downcasting,
            options,
            time_zone_format_options,
        )
    }

    /// Construct a new [`ZonedAnyDateTimeFormatter`] from a data provider that implements
    /// [`BufferProvider`].
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
    /// Test TBD: <https://github.com/unicode-org/icu4x/issues/2145>
    #[inline]
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider<T: Into<Locale>, P>(
        locale: T,
        data_provider: &P,
        options: &DateTimeFormatterOptions,
        time_zone_format_options: &TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        P: BufferProvider,
    {
        let deserializing = data_provider.as_deserializing();
        Self::try_new_unstable(
            locale,
            &deserializing,
            &deserializing,
            &deserializing,
            &deserializing,
            &deserializing,
            options,
            time_zone_format_options,
        )
    }

    /// Takes a [`ZonedDateTimeInput`] implementer and returns an instance of a [`FormattedZonedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format<'l, T>(
        &'l self,
        value: &T,
    ) -> Result<FormattedZonedDateTime<'l>, DateTimeFormatterError>
    where
        T: ZonedDateTimeInput<Calendar = AnyCalendar>,
    {
        if let Some(converted) = self.convert_if_necessary(value)? {
            Ok(self.0.format(&converted))
        } else {
            Ok(self.0.format(value))
        }
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`ZonedDateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl ZonedDateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<(), DateTimeFormatterError> {
        if let Some(converted) = self.convert_if_necessary(value)? {
            self.0.format_to_write(w, &converted)?;
        } else {
            self.0.format_to_write(w, value)?;
        }
        Ok(())
    }

    /// Takes a [`ZonedDateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format_to_string(
        &self,
        value: &impl ZonedDateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<String, DateTimeFormatterError> {
        if let Some(converted) = self.convert_if_necessary(value)? {
            Ok(self.0.format_to_string(&converted))
        } else {
            Ok(self.0.format_to_string(value))
        }
    }

    /// Converts a date to the correct calendar if necessary
    ///
    /// Returns Err if the date is not ISO or compatible with the current calendar, returns Ok(None)
    /// if the date is compatible with the current calendar and doesn't need conversion
    fn convert_if_necessary(
        &self,
        value: &impl ZonedDateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<Option<ExtractedZonedDateTimeInput>, DateTimeFormatterError> {
        let this_calendar = self.1.kind();
        let date_calendar = value.any_calendar_kind();
        if Some(this_calendar) != date_calendar {
            if date_calendar != Some(AnyCalendarKind::Iso) {
                return Err(DateTimeFormatterError::MismatchedAnyCalendar(
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
            let mut extracted = ExtractedZonedDateTimeInput::extract_from(value);
            extracted.date_time_input = converted;
            Ok(Some(extracted))
        } else {
            Ok(None)
        }
    }
}
