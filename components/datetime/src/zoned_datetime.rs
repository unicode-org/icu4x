// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::Locale;
use icu_provider::{DataProvider, DataRequest, ResourceOptions, ResourcePath};

use crate::{
    date::ZonedDateTimeInput,
    datetime::DateTimeFormat,
    format::zoned_datetime::{self, FormattedZonedDateTime},
    options::DateTimeFormatOptions,
    provider::{self, helpers::DateTimePatterns},
    timezone::TimeZoneFormat,
    DateTimeFormatError,
};

/// `ZonedDateTimeFormat` is the composition of `DateTimeFormat` and `TimeZoneFormat`.
///
/// `ZonedDateTimeFormat` uses data from the `DataProvider`s, the selected `Locale`, and the provided
/// pattern to collect all data necessary to format a datetime with time zones into that locale.
///
/// The various pattern symbols specified in UTS-35 require different sets of data for formatting.
/// As such, `TimeZoneFormat` will pull in only the resources it needs to format that pattern
/// that is derived from the provided `DateTimeFormatOptions`.
///
/// For that reason, one should think of the process of formatting a zoned datetime in two steps:
/// first, a computationally heavy construction of `ZonedDateTimeFormat`, and then fast formatting
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
///
/// let options = length::Bag {
///     date: Some(length::Date::Medium),
///     time: Some(length::Time::Short),
///     ..Default::default()
/// };
/// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &options.into())
///     .expect("Failed to create DateTimeFormat instance.");
///
///
/// let zoned_date_time: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
///     .parse()
///     .expect("Failed to parse zoned datetime");
///
/// let value = zdtf.format_to_string(&zoned_date_time);
/// ```
pub struct ZonedDateTimeFormat<'d> {
    pub(super) date_time_format: DateTimeFormat<'d>,
    pub(super) time_zone_format: TimeZoneFormat<'d>,
}

impl<'d> ZonedDateTimeFormat<'d> {
    /// `ZonedDateTimeFormat` constructor which takes a selected `Locale`, reference to a `DataProvider` for
    /// dates, a `DataProvider` for time zones, and a list of options. It collects all data necessary to
    /// format zoned datetime values into the given locale.
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
    ///
    /// let options = DateTimeFormatOptions::default();
    ///
    /// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &options);
    ///
    /// assert_eq!(zdtf.is_ok(), true);
    /// ```
    pub fn try_new<L, DP, ZP>(
        locale: L,
        date_provider: &DP,
        zone_provider: &ZP,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        L: Into<Locale>,
        DP: DataProvider<'d, provider::gregory::DatesV1> + ?Sized,
        ZP: DataProvider<'d, provider::timezones::TimeZoneFormatsV1<'d>>
            + DataProvider<'d, provider::timezones::ExemplarCitiesV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneGenericNamesLongV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneGenericNamesShortV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneSpecificNamesLongV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneSpecificNamesShortV1<'d>>
            + ?Sized,
    {
        let locale = locale.into();
        let data = date_provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: provider::key::GREGORY_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(locale.clone().into()),
                    },
                },
            })?
            .payload
            .take()?;

        let pattern = data
            .patterns
            .get_pattern_for_options(options)?
            .unwrap_or_default();

        let date_time_format = DateTimeFormat::new(locale, pattern, data);
        let time_zone_format = TimeZoneFormat::try_new(
            date_time_format.locale.clone(),
            date_time_format.pattern.clone(),
            zone_provider,
        )?;

        Ok(Self {
            date_time_format,
            time_zone_format,
        })
    }

    /// `format` takes a `ZonedDateTime` value and returns an instance of a `FormattedZonedDateTime`,
    /// which contains all information necessary to display a formatted zoned datetime and operate on it.
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
    /// # let options = DateTimeFormatOptions::default();
    /// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &options)
    ///     .expect("Failed to create ZonedDateTimeFormat instance.");
    ///
    /// let zoned_date_time: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
    ///     .parse()
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let formatted_date = zdtf.format(&zoned_date_time);
    ///
    /// let _ = format!("Date: {}", formatted_date);
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but `FormattedZonedDateTime` will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    pub fn format<'l: 'd, T>(&'l self, value: &'l T) -> FormattedZonedDateTime<'l, T>
    where
        T: ZonedDateTimeInput,
    {
        FormattedZonedDateTime {
            zoned_date_time_format: self,
            zoned_datetime: value,
        }
    }

    /// `format_to_write` takes a mutable reference to anything that implements the `Write` trait
    /// and a `ZonedDateTime` value, then populates the buffer with a formatted value.
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
    /// # let options = DateTimeFormatOptions::default();
    /// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &options.into())
    ///     .expect("Failed to create ZonedDateTimeFormat instance.");
    ///
    /// let zoned_date_time: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
    ///     .parse()
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let mut buffer = String::new();
    /// zdtf.format_to_write(&mut buffer, &zoned_date_time)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// let _ = format!("Date: {}", buffer);
    /// ```
    pub fn format_to_write(
        &self,
        w: &mut impl std::fmt::Write,
        value: &impl ZonedDateTimeInput,
    ) -> std::fmt::Result {
        zoned_datetime::write_pattern(self, value, w).map_err(|_| std::fmt::Error)
    }

    /// `format_to_string` takes a `ZonedDateTime` value and returns it formatted
    /// as a string.
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
    /// # let options = DateTimeFormatOptions::default();
    /// let zdtf = ZonedDateTimeFormat::try_new(locale, &date_provider, &zone_provider, &options.into())
    ///     .expect("Failed to create ZonedDateTimeFormat instance.");
    ///
    /// let zoned_date_time: MockZonedDateTime = "2021-04-08T16:12:37.000-07:00"
    ///     .parse()
    ///     .expect("Failed to parse zoned datetime");
    ///
    /// let _ = zdtf.format_to_string(&zoned_date_time);
    /// ```
    pub fn format_to_string(&self, value: &impl ZonedDateTimeInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}
