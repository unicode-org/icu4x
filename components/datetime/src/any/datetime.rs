// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    options::{components, DateTimeFormatterOptions},
    raw,
};
use alloc::string::String;

use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value};

use icu_provider::prelude::*;

use crate::provider::{
    calendar::{
        DateLengthsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
        TimeLengthsV1Marker, TimeSymbolsV1Marker,
    },
    week_data::WeekDataV1Marker,
};
use crate::{input::DateTimeInput, DateTimeFormatterError, FormattedDateTime};
use icu_calendar::any_calendar::{AnyCalendar, AnyCalendarKind};
use icu_calendar::provider::JapaneseErasV1Marker;
use icu_calendar::{types::Time, DateTime};
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::DataLocale;

/// [`AnyDateTimeFormatter`] is a [`DateTimeFormatter`](crate::DateTimeFormatter) capable of formatting
/// dates from any calendar, selected at runtime.
///
/// When constructed, it uses data from the [data provider], selected locale and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`AnyDateTimeFormatter`], and then fast formatting of [`DateTime`](icu_calendar::DateTime) data using the instance.
///
/// [`icu_datetime`]: crate
///
/// # Examples
///
/// ```
/// use icu::calendar::{any_calendar::AnyCalendar, DateTime, Gregorian};
/// use icu::datetime::{options::length, any::AnyDateTimeFormatter};
/// use icu::locid::Locale;
/// use std::str::FromStr;
///
/// let provider = icu_testdata::get_provider();
///
/// let mut options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
///
/// let dtf = AnyDateTimeFormatter::try_new_with_buffer_provider(&provider, &Locale::from_str("en-u-ca-gregory").unwrap().into(), &options.into())
///     .expect("Failed to create DateTimeFormatter instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
/// let any_datetime = datetime.to_any();
///
/// let value = dtf.format_to_string(&any_datetime).expect("calendars should match");
/// assert_eq!(value, "Sep 1, 2020, 12:34 PM");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct AnyDateTimeFormatter(pub(crate) raw::DateTimeFormatter, AnyCalendar);

impl AnyDateTimeFormatter {
    /// Construct a new [`AnyDateTimeFormatter`] from a data provider that implements
    /// [`AnyProvider`].
    ///
    /// The provider must be able to provide data for the following keys: `datetime/symbols@1`, `datetime/timelengths@1`,
    /// `datetime/timelengths@1`, `datetime/symbols@1`, `datetime/skeletons@1`, `datetime/week_data@1`, and `plurals/ordinals@1`.

    ///
    /// Furthermore, based on the type of calendar used, one of the following data keys may be necessary:
    ///
    /// - `u-ca-japanese` (Japanese calendar): `calendar/japanese@1`
    #[inline]
    pub fn try_new_with_any_provider<P>(
        data_provider: &P,
        locale: &DataLocale,
        options: &DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        P: AnyProvider,
    {
        let downcasting = data_provider.as_downcasting();
        Self::try_new_unstable(&downcasting, locale, options)
    }

    /// Construct a new [`AnyDateTimeFormatter`] from a data provider that implements
    /// [`BufferProvider`].
    ///
    /// The provider must be able to provide data for the following keys: `datetime/symbols@1`, `datetime/datelengths@1`,
    /// `datetime/timelengths@1`, `datetime/symbols@1`, `datetime/skeletons@1`, `datetime/week_data@1`, and `plurals/ordinals@1`.
    ///
    /// Furthermore, based on the type of calendar used, one of the following data keys may be necessary:
    ///
    /// - `u-ca-japanese` (Japanese calendar): `calendar/japanese@1`
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{any_calendar::AnyCalendar, DateTime, Gregorian};
    /// use icu::datetime::{options::length, any::AnyDateTimeFormatter};
    /// use icu::locid::Locale;
    /// use icu_provider::any::DynamicDataProviderAnyMarkerWrap;
    /// use std::str::FromStr;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let mut options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
    /// let locale = Locale::from_str("en-u-ca-gregory").unwrap();
    ///
    /// let dtf = AnyDateTimeFormatter::try_new_with_buffer_provider(&provider, &locale.into(), &options.into())
    ///     .expect("Failed to create DateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    /// let any_datetime = datetime.to_any();
    ///
    /// let value = dtf.format_to_string(&any_datetime).expect("calendars should match");
    /// assert_eq!(value, "Sep 1, 2020, 12:34 PM");
    /// ```
    #[inline]
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider<P>(
        data_provider: &P,
        locale: &DataLocale,
        options: &DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        P: BufferProvider,
    {
        let deserializing = data_provider.as_deserializing();
        Self::try_new_unstable(&deserializing, locale, options)
    }

    /// Construct a new [`AnyDateTimeFormatter`] from a data provider that can provide all of the requested data.
    ///
    /// This method is **unstable**, more bounds may be added in the future as calendar support is added. It is
    /// preferable to use a provider that implements `DataProvider<D>` for all `D`, and ensure data is loaded as
    /// appropriate. The [`Self::try_new_with_buffer_provider()`], [`Self::try_new_with_any_provider()`] constructors
    /// may also be used if compile stability is desired.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{any_calendar::AnyCalendar, DateTime, Gregorian};
    /// use icu::datetime::{options::length, any::AnyDateTimeFormatter};
    /// use icu::locid::Locale;
    /// use icu_provider::any::DynamicDataProviderAnyMarkerWrap;
    /// use std::str::FromStr;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let mut options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
    /// let locale = Locale::from_str("en-u-ca-gregory").unwrap();
    ///
    /// let dtf = AnyDateTimeFormatter::try_new_unstable(&provider, &locale.into(), &options.into())
    ///     .expect("Failed to create DateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    /// let any_datetime = datetime.to_any();
    ///
    /// let value = dtf.format_to_string(&any_datetime).expect("calendars should match");
    /// assert_eq!(value, "Sep 1, 2020, 12:34 PM");
    /// ```
    #[inline(never)]
    pub fn try_new_unstable<P>(
        data_provider: &P,
        locale: &DataLocale,
        options: &DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        P: DataProvider<DateSymbolsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DateLengthsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<DateSkeletonPatternsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + ?Sized,
    {
        // TODO(#2188): Avoid cloning the DataLocale by passing the calendar
        // separately into the raw formatter.
        let mut locale = locale.clone();

        // TODO (#2038), DO NOT SHIP 1.0 without fixing this
        let kind = if let Some(kind) = AnyCalendarKind::from_data_locale(&locale) {
            kind
        } else {
            locale.set_unicode_ext(key!("ca"), value!("gregory"));
            AnyCalendarKind::Gregorian
        };

        // We share data under ethiopic
        if kind == AnyCalendarKind::Ethioaa {
            locale.set_unicode_ext(key!("ca"), value!("ethiopic"));
        }

        let calendar = AnyCalendar::try_new_unstable(kind, data_provider)?;

        Ok(Self(
            raw::DateTimeFormatter::try_new(data_provider, locale, options)?,
            calendar,
        ))
    }
    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format<'l, T>(
        &'l self,
        value: &T,
    ) -> Result<FormattedDateTime<'l>, DateTimeFormatterError>
    where
        T: DateTimeInput<Calendar = AnyCalendar>,
    {
        if let Some(converted) = self.convert_if_necessary(value)? {
            Ok(self.0.format(&converted))
        } else {
            Ok(self.0.format(value))
        }
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<(), DateTimeFormatterError> {
        if let Some(converted) = self.convert_if_necessary(value)? {
            self.0.format_to_write(w, &converted)?;
        } else {
            self.0.format_to_write(w, value)?;
        }
        Ok(())
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format_to_string(
        &self,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<String, DateTimeFormatterError> {
        if let Some(converted) = self.convert_if_necessary(value)? {
            Ok(self.0.format_to_string(&converted))
        } else {
            Ok(self.0.format_to_string(value))
        }
    }

    /// Returns a [`components::Bag`] that represents the resolved components for the
    /// options that were provided to the [`AnyDateTimeFormatter`]. The developer may request
    /// a certain set of options for a [`AnyDateTimeFormatter`] but the locale and resolution
    /// algorithm may change certain details of what actually gets resolved.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::{components, length},
    ///     any::AnyDateTimeFormatter, DateTimeFormatterOptions,
    /// };
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    ///
    /// let options = length::Bag::from_date_style(length::Date::Medium).into();
    ///
    /// let provider = icu_testdata::get_provider();
    /// let dtf = AnyDateTimeFormatter::try_new_with_buffer_provider(
    ///     &provider,
    ///     &Locale::from_str("en-u-ca-gregory").unwrap().into(),
    ///     &options
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

    /// Converts a date to the correct calendar if necessary
    ///
    /// Returns Err if the date is not ISO or compatible with the current calendar, returns Ok(None)
    /// if the date is compatible with the current calendar and doesn't need conversion
    fn convert_if_necessary<'a>(
        &'a self,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<Option<DateTime<icu_calendar::Ref<'a, AnyCalendar>>>, DateTimeFormatterError> {
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
            Ok(Some(converted))
        } else {
            Ok(None)
        }
    }
}
