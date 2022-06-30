// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains the untyped [`AnyCalendar`]-based `DateTimeFormat` APIs that are
//! capable of formatting dates from any calendar

use crate::{
    options::{components, DateTimeFormatOptions},
    raw,
};
use alloc::string::String;

use icu_locid::Locale;

use icu_provider::prelude::*;

use crate::provider::{
    calendar::{
        DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
        TimePatternsV1Marker, TimeSymbolsV1Marker,
    },
    week_data::WeekDataV1Marker,
};
use crate::{date::DateTimeInput, DateTimeFormatError, FormattedDateTime};
use icu_calendar::any_calendar::{AnyCalendar, AnyCalendarKind};
use icu_calendar::provider::JapaneseErasV1Marker;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;

/// [`AnyDateTimeFormat`] is a [`DateTimeFormat`](crate::DateTimeFormat) capable of formatting
/// dates from any calendar, selected at runtime.
///
/// When constructed, it uses data from the [data provider], selected [`Locale`] and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`DateTimeFormat`](crate::DateTimeFormat), and then fast formatting of [`DateTime`](icu_calendar::DateTime) data using the instance.
///
/// [`icu_datetime`]: crate
/// [`DateTimeFormat`]: crate::datetime::DateTimeFormat
///
/// # Examples
///
/// ```
/// use icu::calendar::{any_calendar::AnyCalendar, DateTime, Gregorian};
/// use icu::datetime::{options::length, any::AnyDateTimeFormat};
/// use icu::locid::Locale;
/// use icu_provider::any::DynProviderAnyMarkerWrap;
/// use std::str::FromStr;
///
/// let provider = icu_testdata::get_provider();
///
/// let mut options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
///
/// let dtf = AnyDateTimeFormat::try_new_with_buffer_provider(Locale::from_str("en-u-ca-gregory").unwrap(), &provider, &options.into())
///     .expect("Failed to create DateTimeFormat instance.");
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
pub struct AnyDateTimeFormat(pub(super) raw::DateTimeFormat, AnyCalendar);

impl AnyDateTimeFormat {
    /// Construct a new [`AnyDateTimeFormat`] from a data provider that implements
    /// [`AnyProvider`].
    ///
    /// The provider must be able to provide data for the following keys: `datetime/symbols@1`, `datetime/timelengths@1`,
    /// `datetime/timelengths@1`, `datetime/symbols@1`, `datetime/skeletons@1`, `datetime/week_data@1`, and `plurals/ordinals@1`.

    ///
    /// Furthermore, based on the type of calendar used, one of the following data keys may be necessary:
    ///
    /// - `u-ca-japanese` (Japanese calendar): `calendar/japanese@1`
    #[inline]
    pub fn try_new_with_any_provider<T: Into<Locale>, P>(
        locale: T,
        data_provider: &P,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        P: AnyProvider,
    {
        let downcasting = data_provider.as_downcasting();
        Self::try_new_unstable(locale, &downcasting, options)
    }

    /// Construct a new [`AnyDateTimeFormat`] from a data provider that implements
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
    /// use icu::datetime::{options::length, any::AnyDateTimeFormat};
    /// use icu::locid::Locale;
    /// use icu_provider::any::DynProviderAnyMarkerWrap;
    /// use std::str::FromStr;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let mut options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
    ///
    /// let dtf = AnyDateTimeFormat::try_new_with_buffer_provider(Locale::from_str("en-u-ca-gregory").unwrap(), &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
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
    pub fn try_new_with_buffer_provider<T: Into<Locale>, P>(
        locale: T,
        data_provider: &P,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        P: BufferProvider,
    {
        let deserializing = data_provider.as_deserializing();
        Self::try_new_unstable(locale, &deserializing, options)
    }

    /// Construct a new [`AnyDateTimeFormat`] from a data provider that can provide all of the requested data.
    ///
    /// This method is **unstable**, more bounds may be added in the future as calendar support is added. It is
    /// preferable to use a provider that implements `ResourceProvider<D>` for all `D`, and ensure data is loaded as
    /// appropriate. The [`Self::try_new_with_buffer_provider()`], [`Self::try_new_with_any_provider()`] constructors
    /// may also be used if compile stability is desired.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{any_calendar::AnyCalendar, DateTime, Gregorian};
    /// use icu::datetime::{options::length, any::AnyDateTimeFormat};
    /// use icu::locid::Locale;
    /// use icu_provider::any::DynProviderAnyMarkerWrap;
    /// use std::str::FromStr;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let mut options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
    ///
    /// let dtf = AnyDateTimeFormat::try_new_unstable(Locale::from_str("en-u-ca-gregory").unwrap(), &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    /// let any_datetime = datetime.to_any();
    ///
    /// let value = dtf.format_to_string(&any_datetime).expect("calendars should match");
    /// assert_eq!(value, "Sep 1, 2020, 12:34 PM");
    /// ```
    #[inline(never)]
    pub fn try_new_unstable<T, P>(
        locale: T,
        data_provider: &P,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        T: Into<Locale>,
        P: ResourceProvider<DateSymbolsV1Marker>
            + ResourceProvider<TimeSymbolsV1Marker>
            + ResourceProvider<DatePatternsV1Marker>
            + ResourceProvider<TimePatternsV1Marker>
            + ResourceProvider<DateSkeletonPatternsV1Marker>
            + ResourceProvider<OrdinalV1Marker>
            + ResourceProvider<WeekDataV1Marker>
            + ResourceProvider<DecimalSymbolsV1Marker>
            + ResourceProvider<JapaneseErasV1Marker>
            + ?Sized,
    {
        let locale = locale.into();

        // TODO (#2038), DO NOT SHIP 1.0 without fixing this
        let kind = AnyCalendarKind::from_locale(&locale).unwrap_or(AnyCalendarKind::Gregorian);

        let calendar = AnyCalendar::try_new_unstable(kind, data_provider)?;

        Ok(Self(
            raw::DateTimeFormat::try_new(locale, data_provider, options)?,
            calendar,
        ))
    }
    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary.
    #[inline]
    pub fn format<'l, T>(
        &'l self,
        value: &'l T,
    ) -> Result<FormattedDateTime<'l>, DateTimeFormatError>
    where
        T: DateTimeInput<Calendar = AnyCalendar>,
    {
        self.check_calendars(value)?;
        Ok(self.0.format(value))
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary.
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<(), DateTimeFormatError> {
        self.check_calendars(value)?;
        self.0.format_to_write(w, value)?;
        Ok(())
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary.
    #[inline]
    pub fn format_to_string(
        &self,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<String, DateTimeFormatError> {
        self.check_calendars(value)?;
        Ok(self.0.format_to_string(value))
    }

    /// Returns a [`components::Bag`] that represents the resolved components for the
    /// options that were provided to the [`DateTimeFormat`](crate::DateTimeFormat). The developer may request
    /// a certain set of options for a [`DateTimeFormat`](crate::DateTimeFormat) but the locale and resolution
    /// algorithm may change certain details of what actually gets resolved.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::{components, length},
    ///     any::AnyDateTimeFormat, DateTimeFormatOptions,
    /// };
    /// use icu::locid::Locale;
    /// use std::str::FromStr;
    ///
    /// let options = length::Bag::from_date_style(length::Date::Medium).into();
    ///
    /// let provider = icu_testdata::get_provider();
    /// let dtf = AnyDateTimeFormat::try_new_with_buffer_provider(Locale::from_str("en-u-ca-gregory").unwrap(),
    ///                                                           &provider, &options)
    ///     .expect("Failed to create DateTimeFormat instance.");
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

    /// Checks if a date is constructed with the same calendar
    fn convert_if_necessary(
        &self,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Option<DateTime<AnyCalendar>> {
        let this_calendar = self.1.kind();
        let date_calendar = value.any_calendar_kind();
        if Some(this_calendar) != date_calendar {
            return Err(DateTimeFormatError::MismatchedAnyCalendar(
                this_calendar,
                date_calendar,
            ));
        }

        None
    }
}
