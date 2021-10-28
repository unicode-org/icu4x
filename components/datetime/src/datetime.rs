// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`DateTimeFormat`].

use crate::{
    format::datetime,
    options::DateTimeFormatOptions,
    provider::gregory::patterns::PatternPluralsFromPatternsV1Marker,
    provider::gregory::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker},
};
use alloc::string::String;
use icu_locid::Locale;
use icu_plurals::{provider::PluralRulesV1Marker, PluralRuleType, PluralRules};
use icu_provider::prelude::*;

use crate::{
    date::DateTimeInput, pattern::runtime::PatternPlurals, provider, DateTimeFormatError,
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
    pub(super) patterns: DataPayload<'data, PatternPluralsFromPatternsV1Marker>,
    pub(super) symbols: Option<DataPayload<'data, DateSymbolsV1Marker>>,
    pub(super) ordinal_rules: Option<PluralRules<'data>>,
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
            + DataProvider<'data, DateSkeletonPatternsV1Marker>
            + DataProvider<'data, PluralRulesV1Marker>,
    {
        let locale = locale.into();

        let patterns =
            provider::date_time::PatternSelector::for_options(data_provider, &locale, options)?;

        let requires_data = datetime::analyze_patterns(&patterns.get().0, false)
            .map_err(|field| DateTimeFormatError::UnsupportedField(field.symbol))?;

        let langid: icu_locid::LanguageIdentifier = locale.clone().into();

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new(
                locale.clone(),
                data_provider,
                PluralRuleType::Ordinal,
            )?)
        } else {
            None
        };

        let symbols_data = if requires_data {
            Some(
                data_provider
                    .load_payload(&DataRequest {
                        resource_path: ResourcePath {
                            key: provider::key::GREGORY_DATE_SYMBOLS_V1,
                            options: ResourceOptions {
                                variant: None,
                                langid: Some(langid),
                            },
                        },
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        Ok(Self::new(locale, patterns, symbols_data, ordinal_rules))
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
        patterns: DataPayload<'data, PatternPluralsFromPatternsV1Marker>,
        symbols: Option<DataPayload<'data, DateSymbolsV1Marker>>,
        ordinal_rules: Option<PluralRules<'data>>,
    ) -> Self {
        let locale = locale.into();

        Self {
            locale,
            patterns,
            symbols,
            ordinal_rules,
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
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l, 'data, T>
    where
        T: DateTimeInput,
    {
        FormattedDateTime {
            patterns: &self.patterns,
            symbols: self.symbols.as_ref().map(|s| s.get()),
            datetime: value,
            locale: &self.locale,
            ordinal_rules: self.ordinal_rules.as_ref(),
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
        datetime::write_pattern_plurals(
            &self.patterns.get().0,
            self.symbols.as_ref().map(|s| s.get()),
            value,
            self.ordinal_rules.as_ref(),
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
