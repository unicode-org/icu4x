// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! `icu-datetime` is one of the [`ICU4X`] components.
//!
//! It is a core API for formatting date and time to user readable textual representation.
//!
//! [`DateTimeFormat`] is the main structure of the component. It accepts a set of arguments which
//! allow it to collect necessary data from the `DataProvider`, and once instantiated, can be
//! used to quickly format any date and time provided.
//!
//! # Examples
//!
//! ```
//! use icu_locale_macros::langid;
//! use icu_datetime::{DateTimeFormat, options::style};
//! use icu_datetime::MockDateTime;
//! use icu_data_provider::InvariantDataProvider;
//!
//! let lid = langid!("en");
//!
//! let provider = InvariantDataProvider;
//!
//! let options = style::Bag {
//!     date: Some(style::Date::Medium),
//!     time: Some(style::Time::Short),
//!     ..Default::default()
//! };
//! let dtf = DateTimeFormat::try_new(lid, &provider, &options.into())
//!     .expect("Failed to create DateTimeFormat instance.");
//!
//!
//! let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
//!     .expect("Failed to construct DateTime.");
//!
//! let value = dtf.format_to_string(&date_time);
//! ```
//!
//! At the moment, the crate provides only options using the `Style` bag, but in the future,
//! we expect to add more ways to customize the output, like skeletons, and components.
//!
//! *Notice:* Rust at the moment does not have a canonical way to represent date and time. We are introducing
//! `date::MockDateTime` as a reference example of the data necessary for ICU DateTimeFormat to work, and
//! we hope to work with the community to develop core date and time APIs that will work as an input for this component.
//!
//! [`DateTimeFormat`]: ./struct.DateTimeFormat.html
//! [`ICU4X`]: https://github.com/unicode-org/icu4x
pub mod date;
mod error;
mod fields;
mod format;
pub mod options;
#[doc(hidden)]
pub mod pattern;
mod provider;

pub use date::{DateTimeType, MockDateTime};
pub use error::DateTimeFormatError;
use format::write_pattern;
pub use format::FormattedDateTime;
use icu_data_provider::{icu_data_key, structs, DataEntry, DataProvider, DataRequest};
use icu_locale::LanguageIdentifier;
#[doc(inline)]
pub use options::DateTimeFormatOptions;
use pattern::Pattern;
use provider::DateTimeDates;
use std::borrow::Cow;

/// `DateTimeFormat` is the main structure of the `icu-datetime` component.
/// When constructed, it uses data from the `DataProvider`, selected `LanguageIdentifier` and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of `DateTimeFormat`, and then fast formatting of `DateTime` data using the instance.
///
/// # Examples
///
/// ```
/// use icu_locale_macros::langid;
/// use icu_datetime::{DateTimeFormat, options::style};
/// use icu_datetime::MockDateTime;
/// use icu_data_provider::InvariantDataProvider;
///
/// let lid = langid!("en");
///
/// let provider = InvariantDataProvider;
///
/// let options = style::Bag {
///     date: Some(style::Date::Medium),
///     time: Some(style::Time::Short),
///     ..Default::default()
/// };
/// let dtf = DateTimeFormat::try_new(lid, &provider, &options.into())
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
    _langid: LanguageIdentifier,
    pattern: Pattern,
    data: Cow<'d, structs::dates::gregory::DatesV1>,
}

impl<'d> DateTimeFormat<'d> {
    /// `DateTimeFormat` constructor which takes a selected `LanguageIdentifier`, reference to a `DataProvider` and
    /// a list of options and collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_macros::langid;
    /// use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu_datetime::MockDateTime;
    /// use icu_data_provider::InvariantDataProvider;
    ///
    /// let lid = langid!("en");
    ///
    /// let provider = InvariantDataProvider;
    ///
    /// let options = DateTimeFormatOptions::default();
    ///
    /// let dtf = DateTimeFormat::try_new(lid, &provider, &options);
    ///
    /// assert_eq!(dtf.is_ok(), true);
    /// ```
    pub fn try_new<D: DataProvider<'d>>(
        langid: LanguageIdentifier,
        data_provider: &D,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError> {
        let data_key = icu_data_key!(dates: gregory@1);
        let response = data_provider.load(&DataRequest {
            data_key,
            data_entry: DataEntry {
                variant: None,
                langid: langid.clone(),
            },
        })?;
        let data: Cow<structs::dates::gregory::DatesV1> = response.take_payload()?;

        let pattern = data.get_pattern_for_options(options)?.unwrap_or_default();

        Ok(Self {
            _langid: langid,
            pattern,
            data,
        })
    }

    /// `format` takes a `DateTime` value and returns an instance of a `FormattedDateTime` object
    /// which contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu_locale_macros::langid;
    /// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// # use icu_datetime::MockDateTime;
    /// # use icu_data_provider::InvariantDataProvider;
    /// # let lid = langid!("en");
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(lid, &provider, &options)
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
        T: DateTimeType,
    {
        FormattedDateTime {
            pattern: &self.pattern,
            data: &self.data,
            date_time: value,
        }
    }

    /// `format_to_write` takes a mutable reference to anything that implements `Write` trait
    /// and a `DateTime` value and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu_locale_macros::langid;
    /// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// # use icu_datetime::MockDateTime;
    /// # use icu_data_provider::InvariantDataProvider;
    /// # let lid = langid!("en");
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(lid, &provider, &options.into())
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
    pub fn format_to_write<T>(&self, w: &mut impl std::fmt::Write, value: &T) -> std::fmt::Result
    where
        T: DateTimeType,
    {
        write_pattern(&self.pattern, &self.data, value, w).map_err(|_| std::fmt::Error)
    }

    /// `format_to_string` takes a `DateTime` value and returns it formatted
    /// as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu_locale_macros::langid;
    /// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// # use icu_datetime::MockDateTime;
    /// # use icu_data_provider::InvariantDataProvider;
    /// # let lid = langid!("en");
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(lid, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let _ = dtf.format_to_string(&date_time);
    /// ```
    pub fn format_to_string<T>(&self, value: &T) -> String
    where
        T: DateTimeType,
    {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}
