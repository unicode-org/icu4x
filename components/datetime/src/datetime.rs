// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`DateTimeFormat`].

use crate::{
    options::{components, DateTimeFormatOptions},
    provider::calendar::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker},
    provider::week_data::WeekDataV1Marker,
    raw,
};
use alloc::string::String;
use core::marker::PhantomData;
use icu_locid::{unicode_ext_key, Locale};
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;

use crate::{date::DateTimeInput, CldrCalendar, DateTimeFormatError, FormattedDateTime};

/// [`DateTimeFormat`] is the main structure of the [`icu_datetime`] component.
/// When constructed, it uses data from the [data provider], selected [`Locale`] and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`DateTimeFormat`], and then fast formatting of [`DateTimeInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`DateTimeFormat`]: crate::datetime::DateTimeFormat
///
/// # Examples
///
/// ```
/// use icu::locid::locale;
/// use icu::datetime::{DateTimeFormat, options::length};
/// use icu::calendar::{DateTime, Gregorian};
/// use icu_provider::inv::InvariantDataProvider;
///
/// let provider = InvariantDataProvider;
///
/// let options = length::Bag {
///     date: Some(length::Date::Medium),
///     time: Some(length::Time::Short),
///     ..Default::default()
/// };
/// let dtf = DateTimeFormat::<Gregorian>::try_new(locale!("en"), &provider, &options.into())
///     .expect("Failed to create DateTimeFormat instance.");
///
///
/// let datetime = DateTime::new_gregorian_datetime_from_integers(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// let value = dtf.format_to_string(&datetime);
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct DateTimeFormat<C>(pub(super) raw::DateTimeFormat, PhantomData<C>);

impl<C: CldrCalendar> DateTimeFormat<C> {
    /// Constructor that takes a selected [`Locale`], reference to a [data provider] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let provider = InvariantDataProvider;
    ///
    /// let options = DateTimeFormatOptions::default();
    ///
    /// let dtf = DateTimeFormat::<Gregorian>::try_new(locale!("en"), &provider, &options);
    ///
    /// assert_eq!(dtf.is_ok(), true);
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new<T: Into<Locale>, D>(
        locale: T,
        data_provider: &D,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        D: ResourceProvider<DateSymbolsV1Marker>
            + ResourceProvider<DatePatternsV1Marker>
            + ResourceProvider<DateSkeletonPatternsV1Marker>
            + ResourceProvider<OrdinalV1Marker>
            + ResourceProvider<WeekDataV1Marker>,
    {
        let mut locale = locale.into();
        // TODO(#419): Resolve the locale calendar with the API calendar.
        locale
            .extensions
            .unicode
            .keywords
            .set(unicode_ext_key!("ca"), C::IDENTIFIER);
        Ok(Self(
            raw::DateTimeFormat::try_new(locale, data_provider, options)?,
            PhantomData,
        ))
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::<Gregorian>::try_new(locale, &provider, &options)
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime_from_integers(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let formatted_date = dtf.format(&datetime);
    ///
    /// let _ = format!("Date: {}", formatted_date);
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but [`FormattedDateTime`] will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l, T>
    where
        T: DateTimeInput<Calendar = C>,
    {
        self.0.format(value)
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::<Gregorian>::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime_from_integers(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// dtf.format_to_write(&mut buffer, &datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// let _ = format!("Date: {}", buffer);
    /// ```
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput<Calendar = C>,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, value)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::<Gregorian>::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime_from_integers(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let _ = dtf.format_to_string(&datetime);
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput<Calendar = C>) -> String {
        self.0.format_to_string(value)
    }

    /// Returns a [`components::Bag`] that represents the resolved components for the
    /// options that were provided to the [`DateTimeFormat`]. The developer may request
    /// a certain set of options for a [`DateTimeFormat`] but the locale and resolution
    /// algorithm may change certain details of what actually gets resolved.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::{components, length},
    ///     DateTimeFormat, DateTimeFormatOptions,
    /// };
    /// use icu::locid::locale;
    ///
    /// let options = DateTimeFormatOptions::Length(length::Bag {
    ///     date: Some(length::Date::Medium),
    ///     time: None,
    ///     preferences: None,
    /// });
    ///
    /// let provider = icu_testdata::get_provider();
    /// let dtf = DateTimeFormat::<Gregorian>::try_new(locale!("en"), &provider, &options)
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// assert_eq!(
    ///     dtf.resolve_components(),
    ///     components::Bag {
    ///         year: Some(components::Year::Numeric),
    ///         month: Some(components::Month::Short),
    ///         day: Some(components::Day::NumericDayOfMonth),
    ///         ..Default::default()
    ///     }
    /// );
    /// ```
    pub fn resolve_components(&self) -> components::Bag {
        self.0.resolve_components()
    }
}
