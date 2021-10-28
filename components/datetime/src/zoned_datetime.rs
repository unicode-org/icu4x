// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use icu_locid::{LanguageIdentifier, Locale};
use icu_plurals::{provider::PluralRulesV1Marker, PluralRuleType, PluralRules};
use icu_provider::{DataProvider, DataRequest, ResourceOptions, ResourcePath};

use crate::{
    date::ZonedDateTimeInput,
    datetime::DateTimeFormat,
    format::{
        datetime,
        zoned_datetime::{self, FormattedZonedDateTime},
    },
    options::DateTimeFormatOptions,
    pattern::runtime::PatternPlurals,
    provider::{
        self,
        gregory::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker},
    },
    time_zone::TimeZoneFormat,
    DateTimeFormatError,
};

// TODO(#622) link [`TimeZoneFormat`] once it is public.
/// The composition of [`DateTimeFormat`] and `TimeZoneFormat`.
///
/// [`ZonedDateTimeFormat`] uses data from the [`DataProvider`]s, the selected [`Locale`], and the
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
/// use icu::locid::Locale;
/// use icu::locid::macros::langid;
/// use icu::datetime::{ZonedDateTimeFormat, options::length};
/// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
/// use icu_provider::inv::InvariantDataProvider;
///
/// let locale: Locale = langid!("en").into();
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
/// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &plural_provider, &options.into())
///     .expect("Failed to create DateTimeFormat instance.");
///
///
/// let zoned_datetime: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
///     .parse()
///     .expect("Failed to parse zoned datetime");
///
/// let value = zdtf.format_to_string(&zoned_datetime);
/// ```
pub struct ZonedDateTimeFormat<'data> {
    pub(super) datetime_format: DateTimeFormat<'data>,
    pub(super) time_zone_format: TimeZoneFormat<'data>,
}

impl<'data> ZonedDateTimeFormat<'data> {
    /// Constructor that takes a selected [`Locale`], a reference to a [`DataProvider`] for
    /// dates, a [`DataProvider`] for time zones, and a list of [`DateTimeFormatOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::macros::langid;
    /// use icu::datetime::{ZonedDateTimeFormat, DateTimeFormatOptions};
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let locale: Locale = langid!("en").into();
    ///
    /// let date_provider = InvariantDataProvider;
    /// let zone_provider = InvariantDataProvider;
    /// let plural_provider = InvariantDataProvider;
    ///
    /// let options = DateTimeFormatOptions::default();
    ///
    /// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &plural_provider, &options);
    ///
    /// assert_eq!(zdtf.is_ok(), true);
    /// ```
    pub fn try_new<L, DP, ZP, PP>(
        locale: L,
        date_provider: &DP,
        zone_provider: &ZP,
        plural_provider: &PP,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        L: Into<Locale>,
        DP: DataProvider<'data, DateSymbolsV1Marker>
            + DataProvider<'data, DatePatternsV1Marker>
            + DataProvider<'data, DateSkeletonPatternsV1Marker>,
        ZP: DataProvider<'data, provider::time_zones::TimeZoneFormatsV1Marker>
            + DataProvider<'data, provider::time_zones::ExemplarCitiesV1Marker>
            + DataProvider<'data, provider::time_zones::MetaZoneGenericNamesLongV1Marker>
            + DataProvider<'data, provider::time_zones::MetaZoneGenericNamesShortV1Marker>
            + DataProvider<'data, provider::time_zones::MetaZoneSpecificNamesLongV1Marker>
            + DataProvider<'data, provider::time_zones::MetaZoneSpecificNamesShortV1Marker>
            + ?Sized,
        PP: DataProvider<'data, PluralRulesV1Marker>,
    {
        let locale = locale.into();
        let langid: LanguageIdentifier = locale.clone().into();

        let patterns =
            provider::date_time::PatternSelector::for_options(date_provider, &locale, options)?;

        let requires_data = datetime::analyze_patterns(&patterns.get().0, true)
            .map_err(|field| DateTimeFormatError::UnsupportedField(field.symbol))?;

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new(
                locale.clone(),
                plural_provider,
                PluralRuleType::Ordinal,
            )?)
        } else {
            None
        };

        let symbols_data = if requires_data {
            Some(
                date_provider
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

        let datetime_format = DateTimeFormat::new(locale, patterns, symbols_data, ordinal_rules);
        let time_zone_format = TimeZoneFormat::try_new(
            datetime_format.locale.clone(),
            datetime_format
                // Only dates have plural variants so we can use any of the patterns for the time segment.
                .patterns
                .clone(),
            zone_provider,
        )?;

        Ok(Self {
            datetime_format,
            time_zone_format,
        })
    }

    /// Takes a [`ZonedDateTimeInput`] implementer and returns an instance of a [`FormattedZonedDateTime`]
    /// that contains all information necessary to display a formatted zoned datetime and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::macros::langid;
    /// use icu::datetime::{ZonedDateTimeFormat, DateTimeFormatOptions};
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let date_provider = InvariantDataProvider;
    /// # let zone_provider = InvariantDataProvider;
    /// # let plural_provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &plural_provider, &options)
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
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedZonedDateTime<'l, 'data, T>
    where
        T: ZonedDateTimeInput,
    {
        FormattedZonedDateTime {
            zoned_datetime_format: self,
            zoned_datetime: value,
        }
    }

    /// Takes a mutable reference to anything that implements the [`Write`](std::fmt::Write) trait
    /// and a [`ZonedDateTimeInput`] implementer, then populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::macros::langid;
    /// use icu::datetime::{ZonedDateTimeFormat, DateTimeFormatOptions};
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let date_provider = InvariantDataProvider;
    /// # let zone_provider = InvariantDataProvider;
    /// # let plural_provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &plural_provider, &options.into())
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
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl ZonedDateTimeInput,
    ) -> core::fmt::Result {
        zoned_datetime::write_pattern(self, value, w).map_err(|_| core::fmt::Error)
    }

    /// Takes a [`ZonedDateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use icu::locid::macros::langid;
    /// use icu::datetime::{ZonedDateTimeFormat, DateTimeFormatOptions};
    /// use icu::datetime::mock::zoned_datetime::MockZonedDateTime;
    /// use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let date_provider = InvariantDataProvider;
    /// # let zone_provider = InvariantDataProvider;
    /// # let plural_provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &plural_provider, &options.into())
    ///     .expect("Failed to create ZonedDateTimeFormat instance.");
    ///
    /// let zoned_datetime: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
    ///     .parse()
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let _ = zdtf.format_to_string(&zoned_datetime);
    /// ```
    pub fn format_to_string(&self, value: &impl ZonedDateTimeInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}
