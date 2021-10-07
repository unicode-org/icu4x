// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`DateTimeFormat`].

use crate::{
    format::datetime,
    options::DateTimeFormatOptions,
    provider::gregory::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker},
};
use alloc::string::String;
use icu_locid::Locale;
use icu_provider::prelude::*;

use crate::{
    date::DateTimeInput, pattern::reference::Pattern, provider, DateTimeFormatError,
    FormattedDateTime,
};

/// [`DateTimeFormat`] is the main structure of the [`icu_datetime`] component.
/// When constructed, it uses data from the [`DataProvider`], selected [`Locale`] and provided options to
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
/// use icu::locid::Locale;
/// use icu::locid::macros::langid;
/// use icu::datetime::{DateTimeFormat, options::length};
/// use icu::calendar::DateTime;
/// use icu_provider::inv::InvariantDataProvider;
///
/// let locale: Locale = langid!("en").into();
///
/// let provider = InvariantDataProvider;
///
/// let options = length::Bag {
///     date: Some(length::Date::Medium),
///     time: Some(length::Time::Short),
///     ..Default::default()
/// };
/// let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
///     .expect("Failed to create DateTimeFormat instance.");
///
///
/// let datetime = DateTime::new_gregorian_datetime_from_integers(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// let value = dtf.format_to_string(&datetime);
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`. In the future this will become even more pronounced
/// when we introduce asynchronous [`DataProvider`] and corresponding asynchronous constructor.
pub struct DateTimeFormat<'data> {
    pub(super) locale: Locale,
    pub(super) pattern: Pattern,
    pub(super) symbols: Option<DataPayload<'data, DateSymbolsV1Marker>>,
}

impl<'data> DateTimeFormat<'data> {
    /// Constructor that takes a selected [`Locale`], reference to a [`DataProvider`] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::macros::langid;
    /// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let locale: Locale = langid!("en").into();
    ///
    /// let provider = InvariantDataProvider;
    ///
    /// let options = DateTimeFormatOptions::default();
    ///
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options);
    ///
    /// assert_eq!(dtf.is_ok(), true);
    /// ```
    pub fn try_new<T: Into<Locale>, D>(
        locale: T,
        data_provider: &D,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        D: DataProvider<'data, DateSymbolsV1Marker>
            + DataProvider<'data, DatePatternsV1Marker>
            + DataProvider<'data, DateSkeletonPatternsV1Marker>,
    {
        let locale = locale.into();

        let pattern = provider::date_time::pattern_for_options(data_provider, &locale, options)?
            .unwrap_or_default();

        let requires_data = datetime::analyze_pattern(&pattern, false)
            .map_err(|field| DateTimeFormatError::UnsupportedField(field.symbol))?;

        let symbols_data = if requires_data {
            Some(
                data_provider
                    .load_payload(&DataRequest {
                        resource_path: ResourcePath {
                            key: provider::key::GREGORY_DATE_SYMBOLS_V1,
                            options: ResourceOptions {
                                variant: None,
                                langid: Some(locale.clone().into()),
                            },
                        },
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        Ok(Self::new(locale, pattern, symbols_data))
    }

    /// Creates a new [`DateTimeFormat`] regardless of whether there are time-zone symbols in the pattern.
    ///
    /// By contrast, the public [`DateTimeFormat::try_new()`] function will return an error if there are
    /// time-zone symbols in the pattern.
    ///
    /// This function is only `pub(super)` (not `pub`) because it is needed by [`ZonedDateTimeFormat`]
    /// to create a [`DateTimeFormat`] for use internally. [`ZonedDateTimeFormat`] maintains
    /// the invariant that [`DateTimeFormat`] will not be used to format the time zone.
    ///
    /// Creating a [`DateTimeFormat`] with time-zone symbols should always be an error
    /// in public contexts.
    ///
    /// [`ZonedDateTimeFormat`]: crate::zoned_datetime::ZonedDateTimeFormat
    pub(super) fn new<T: Into<Locale>>(
        locale: T,
        pattern: Pattern,
        symbols: Option<DataPayload<'data, DateSymbolsV1Marker>>,
    ) -> Self {
        let locale = locale.into();

        Self {
            locale,
            pattern,
            symbols,
        }
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::macros::langid;
    /// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu::calendar::DateTime;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options)
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
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l, T>
    where
        T: DateTimeInput,
    {
        FormattedDateTime {
            pattern: &self.pattern,
            symbols: self.symbols.as_ref().map(|s| s.get()),
            datetime: value,
            locale: &self.locale,
        }
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::macros::langid;
    /// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu::calendar::DateTime;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
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
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput,
    ) -> core::fmt::Result {
        datetime::write_pattern(
            &self.pattern,
            self.symbols.as_ref().map(|s| s.get()),
            value,
            &self.locale,
            w,
        )
        .map_err(|_| core::fmt::Error)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::macros::langid;
    /// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu::calendar::DateTime;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime_from_integers(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let _ = dtf.format_to_string(&datetime);
    /// ```
    pub fn format_to_string(&self, value: &impl DateTimeInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}
