// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_datetime` is one of the [`ICU4X`] components.
//!
//! This API provides necessary functionality for formatting date and time to user readable textual representation.
//!
//! [`DateTimeFormat`] is the main structure of the component. It accepts a set of arguments which
//! allow it to collect necessary data from the [`DataProvider`], and once instantiated, can be
//! used to quickly format any date and time provided.
//!
//! # Examples
//!
//! ```
//! # #[cfg(feature = "provider_serde")] {
//! use icu_locid::Locale;
//! use icu_locid_macros::langid;
//! use icu_datetime::{DateTimeFormat, DateTimeFormatOptions, mock::datetime::MockDateTime, options::length};
//!
//! let provider = icu_testdata::get_provider();
//!
//! let locale: Locale = langid!("en").into();
//!
//! // See the next code example for a more ergonomic example with .into().
//! let options = DateTimeFormatOptions::Length(length::Bag {
//!     date: Some(length::Date::Medium),
//!     time: Some(length::Time::Short),
//!     ..Default::default()
//! });
//!
//! let dtf = DateTimeFormat::try_new(locale, &provider, &options)
//!     .expect("Failed to create DateTimeFormat instance.");
//!
//!
//! let date: MockDateTime = "2020-09-12T12:35:00".parse()
//!     .expect("Failed to parse date.");
//!
//! let formatted_date = dtf.format(&date);
//! assert_eq!(formatted_date.to_string(), "Sep 12, 2020, 12:35 PM");
//! # } // feature = "provider_serde"
//! ```
//!
//! The options can be created more ergonomically using the `Into` trait to automatically
//! convert a [`options::length::Bag`] into a [`DateTimeFormatOptions::Length`].
//!
//! ```
//! # #[cfg(feature = "provider_serde")] {
//! # use icu_locid::Locale;
//! # use icu_locid_macros::langid;
//! # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions, mock::datetime::MockDateTime, options::length};
//! # let provider = icu_testdata::get_provider();
//! # let locale: Locale = langid!("en").into();
//! let options = length::Bag {
//!     date: Some(length::Date::Medium),
//!     time: Some(length::Time::Short),
//!     ..Default::default()
//! }.into();
//!
//! let dtf = DateTimeFormat::try_new(locale, &provider, &options);
//! # } // feature = "provider_serde"
//! ```
//!
//! At the moment, the crate provides only options using the [`Length`] bag, but in the future,
//! we expect to add more ways to customize the output, like skeletons, and components.
//!
//! *Notice:* Rust at the moment does not have a canonical way to represent date and time. We are introducing
//! [`MockDateTime`] as an example of the data necessary for ICU [`DateTimeFormat`] to work, and
//! [we hope to work with the community](https://github.com/unicode-org/icu4x/blob/main/docs/research/date_time.md)
//! to develop core date and time APIs that will work as an input for this component.
//!
//! [`DataProvider`]: icu_provider::DataProvider
//! [`ICU4X`]: ../icu/index.html
//! [`Length`]: options::length
//! [`MockDateTime`]: mock::datetime::MockDateTime
mod arithmetic;
pub mod date;
mod error;
mod fields;
mod format;
pub mod mock;
pub mod options;
#[doc(hidden)]
pub mod pattern;
pub mod provider;
pub mod skeleton;

use crate::provider::helpers::DateTimePatterns;
use date::{DateTimeInput, TimeZoneInput, ZonedDateTimeInput};
pub use error::DateTimeFormatError;
use fields::{FieldSymbol, TimeZone};
pub use format::datetime::FormattedDateTime;
use format::{
    datetime::write_pattern,
    timezone::{self, FormattedTimeZone},
    zoned_datetime::{self, FormattedZonedDateTime},
};
use icu_locid::Locale;
use icu_provider::prelude::*;
#[doc(inline)]
pub use options::DateTimeFormatOptions;
use pattern::{Pattern, PatternItem};
use provider::timezones::{
    ExemplarCitiesV1, MetaZoneGenericNamesLongV1, MetaZoneGenericNamesShortV1,
    MetaZoneSpecificNamesLongV1, MetaZoneSpecificNamesShortV1, TimeZoneFormatsV1,
};
use std::borrow::Cow;

/// `DateTimeFormat` is the main structure of the `icu_datetime` component.
/// When constructed, it uses data from the `DataProvider`, selected `Locale` and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of `DateTimeFormat`, and then fast formatting of `DateTime` data using the instance.
///
/// # Examples
///
/// ```
/// use icu_locid::Locale;
/// use icu_locid_macros::langid;
/// use icu_datetime::{DateTimeFormat, options::length};
/// use icu_datetime::mock::datetime::MockDateTime;
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
/// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// let value = dtf.format_to_string(&date_time);
/// ```
///
/// This model replicates that of `ICU` and `ECMA402` and in the future will get even more pronounce when we introduce
/// asynchronous `DataProvider` and corresponding asynchronous constructor.
pub struct DateTimeFormat<'d> {
    locale: Locale,
    pattern: Pattern,
    symbols: Cow<'d, provider::gregory::DateSymbolsV1>,
}

impl<'d> DateTimeFormat<'d> {
    /// `DateTimeFormat` constructor which takes a selected `Locale`, reference to a `DataProvider` and
    /// a list of options and collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::Locale;
    /// use icu_locid_macros::langid;
    /// use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu_datetime::mock::datetime::MockDateTime;
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
    pub fn try_new<T: Into<Locale>, D: DataProvider<'d, provider::gregory::DatesV1> + ?Sized>(
        locale: T,
        data_provider: &D,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError> {
        let locale = locale.into();
        let data = data_provider
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

        let symbols = match data {
            Cow::Borrowed(data) => Cow::Borrowed(&data.symbols),
            Cow::Owned(data) => Cow::Owned(data.symbols),
        };

        Ok(Self {
            locale,
            pattern,
            symbols,
        })
    }

    /// `format` takes a `DateTime` value and returns an instance of a `FormattedDateTime` object
    /// which contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu_locid::Locale;
    /// # use icu_locid_macros::langid;
    /// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// # use icu_datetime::mock::datetime::MockDateTime;
    /// # use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options)
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let formatted_date = dtf.format(&date_time);
    ///
    /// let _ = format!("Date: {}", formatted_date);
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but `FormattedDateTime` will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    pub fn format<'s, T>(&'s self, value: &'s T) -> FormattedDateTime<'s, T>
    where
        T: DateTimeInput,
    {
        FormattedDateTime {
            pattern: &self.pattern,
            symbols: &self.symbols,
            date_time: value,
            locale: &self.locale,
        }
    }

    /// `format_to_write` takes a mutable reference to anything that implements `Write` trait
    /// and a `DateTime` value and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu_locid::Locale;
    /// # use icu_locid_macros::langid;
    /// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// # use icu_datetime::mock::datetime::MockDateTime;
    /// # use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// dtf.format_to_write(&mut buffer, &date_time)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// let _ = format!("Date: {}", buffer);
    /// ```
    pub fn format_to_write(
        &self,
        w: &mut impl std::fmt::Write,
        value: &impl DateTimeInput,
    ) -> std::fmt::Result {
        write_pattern(&self.pattern, &self.symbols, value, &self.locale, w)
            .map_err(|_| std::fmt::Error)
    }

    /// `format_to_string` takes a `DateTime` value and returns it formatted
    /// as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu_locid::Locale;
    /// # use icu_locid_macros::langid;
    /// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// # use icu_datetime::mock::datetime::MockDateTime;
    /// # use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let _ = dtf.format_to_string(&date_time);
    /// ```
    pub fn format_to_string(&self, value: &impl DateTimeInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}

pub struct TimeZoneFormat<'d> {
    pattern: Pattern,
    zone_formats: Cow<'d, provider::timezones::TimeZoneFormatsV1<'d>>,
    exemplar_cities: Option<Cow<'d, provider::timezones::ExemplarCitiesV1<'d>>>,
    mz_generic_long: Option<Cow<'d, provider::timezones::MetaZoneGenericNamesLongV1<'d>>>,
    mz_generic_short: Option<Cow<'d, provider::timezones::MetaZoneGenericNamesShortV1<'d>>>,
    mz_specific_long: Option<Cow<'d, provider::timezones::MetaZoneSpecificNamesLongV1<'d>>>,
    mz_specific_short: Option<Cow<'d, provider::timezones::MetaZoneSpecificNamesShortV1<'d>>>,
}

impl<'d> TimeZoneFormat<'d> {
    pub fn try_new<L, ZP>(
        locale: L,
        pattern: Pattern,
        zone_provider: &ZP,
    ) -> Result<Self, DateTimeFormatError>
    where
        L: Into<Locale>,
        ZP: DataProvider<'d, provider::timezones::TimeZoneFormatsV1<'d>>
            + DataProvider<'d, provider::timezones::ExemplarCitiesV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneGenericNamesLongV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneGenericNamesShortV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneSpecificNamesLongV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneSpecificNamesShortV1<'d>>
            + ?Sized,
    {
        let locale = locale.into();

        let zone_formats: Cow<TimeZoneFormatsV1> = zone_provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: provider::key::TIMEZONE_FORMATS_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(locale.clone().into()),
                    },
                },
            })?
            .payload
            .take()?;

        let zone_symbols = pattern
            .items()
            .iter()
            .filter_map(|item| match item {
                PatternItem::Field(field) => Some(field),
                _ => None,
            })
            .map(|field| field.symbol)
            .filter_map(|symbol| match symbol {
                FieldSymbol::TimeZone(zone) => Some(zone),
                _ => None,
            });

        let exemplar_cities: Option<Cow<ExemplarCitiesV1>> = None;
        let mz_generic_long: Option<Cow<MetaZoneGenericNamesLongV1>> = None;
        let mz_generic_short: Option<Cow<MetaZoneGenericNamesShortV1>> = None;
        let mut mz_specific_long: Option<Cow<MetaZoneSpecificNamesLongV1>> = None;
        let mut mz_specific_short: Option<Cow<MetaZoneSpecificNamesShortV1>> = None;

        for zone_symbol in zone_symbols {
            match zone_symbol {
                TimeZone::LowerZ => {
                    mz_specific_long = Some(
                        zone_provider
                            .load_payload(&DataRequest {
                                resource_path: ResourcePath {
                                    key: provider::key::TIMEZONE_SPECIFIC_NAMES_LONG_V1,
                                    options: ResourceOptions {
                                        variant: None,
                                        langid: Some(locale.clone().into()),
                                    },
                                },
                            })?
                            .payload
                            .take()?,
                    );
                    mz_specific_short = Some(
                        zone_provider
                            .load_payload(&DataRequest {
                                resource_path: ResourcePath {
                                    key: provider::key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1,
                                    options: ResourceOptions {
                                        variant: None,
                                        langid: Some(locale.clone().into()),
                                    },
                                },
                            })?
                            .payload
                            .take()?,
                    );
                }
                TimeZone::UpperZ => todo!(),
                TimeZone::LowerO => todo!(),
                TimeZone::UpperO => todo!(),
                TimeZone::LowerV => todo!(),
                TimeZone::UpperV => todo!(),
                TimeZone::LowerX => todo!(),
                TimeZone::UpperX => todo!(),
            }
        }

        Ok(Self {
            pattern,
            zone_formats,
            exemplar_cities,
            mz_generic_long,
            mz_generic_short,
            mz_specific_long,
            mz_specific_short,
        })
    }

    pub fn format<'s: 'd, T>(&'s self, value: &'s T) -> FormattedTimeZone<'s, T>
    where
        T: TimeZoneInput,
    {
        FormattedTimeZone {
            time_zone_format: self,
            time_zone: value,
        }
    }

    pub fn format_to_write(
        &self,
        w: &mut impl std::fmt::Write,
        value: &impl TimeZoneInput,
    ) -> std::fmt::Result {
        timezone::write_pattern(self, value, w).map_err(|_| std::fmt::Error)
    }

    pub fn format_to_string(&self, value: &impl TimeZoneInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }

    pub fn short_specific_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<str>> {
        self.mz_specific_short
            .as_ref()
            .and_then(|metazones| {
                time_zone
                    .metazone_id()
                    .and_then(|mz| metazones.get::<str>(&mz))
            })
            .and_then(|specific_names| {
                time_zone
                    .time_variant()
                    .and_then(|variant| specific_names.get::<str>(&variant))
            })
            .map(Clone::clone)
    }

    pub fn long_specific_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<str>> {
        self.mz_specific_long
            .as_ref()
            .and_then(|metazones| {
                time_zone
                    .metazone_id()
                    .and_then(|mz| metazones.get::<str>(&mz))
            })
            .and_then(|specific_names| {
                time_zone
                    .time_variant()
                    .and_then(|variant| specific_names.get::<str>(&variant))
            })
            .map(Clone::clone)
    }

    pub fn localized_gmt_format(&self, time_zone: &impl TimeZoneInput) -> Cow<str> {
        let gmt_offset = time_zone.gmt_offset();
        if gmt_offset.is_zero() {
            self.zone_formats.gmt_zero_format.clone()
        } else {
            Cow::Owned(
                self.zone_formats
                    .gmt_format
                    .replace(
                        "{0}",
                        if gmt_offset.is_positive() {
                            &self.zone_formats.hour_format.0
                        } else {
                            &self.zone_formats.hour_format.1
                        },
                    )
                    // support all combos of "(HH|H):mm" by replacing longest patterns first.
                    .replace("HH", &format!("{:02}", gmt_offset.hours()))
                    .replace("mm", &format!("{:02}", gmt_offset.minutes()))
                    .replace("H", &gmt_offset.hours().to_string()),
            )
        }
    }
}

pub struct ZonedDateTimeFormat<'d> {
    date_time_format: DateTimeFormat<'d>,
    time_zone_format: TimeZoneFormat<'d>,
}

impl<'d> ZonedDateTimeFormat<'d> {
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
        let date_time_format = DateTimeFormat::try_new(locale, date_provider, options)?;
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

    pub fn format<'s: 'd, T>(&'s self, value: &'s T) -> FormattedZonedDateTime<'s, T>
    where
        T: ZonedDateTimeInput,
    {
        FormattedZonedDateTime {
            zoned_date_time_format: self,
            zoned_datetime: value,
        }
    }

    pub fn format_to_write(
        &self,
        w: &mut impl std::fmt::Write,
        value: &impl ZonedDateTimeInput,
    ) -> std::fmt::Result {
        zoned_datetime::write_pattern(self, value, w).map_err(|_| std::fmt::Error)
    }

    pub fn format_to_string(&self, value: &impl ZonedDateTimeInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}
