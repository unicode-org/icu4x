// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Length is a model of encoding information on how to format date and time by specifying the preferred length
//! of date and time fields.
//!
//! If either of the fields is omitted, the value will be formatted according to the pattern associated with the
//! preferred length of the present field in a given locale.
//!
//! If both fields are present, both parts of the value will be formatted and an additional connector pattern
//! will be used to construct a full result.
//! The type of the connector is determined by the length of the [`Date`] field.
//!
//! Additionally, the bag contains an optional set of `Preferences` which represent user preferred adjustments
//! that can be applied onto the pattern right before formatting.
//!
//! # Examples
//!
//! ```
//! use icu::datetime::options::length;
//! use icu::datetime::DateTimeFormatterOptions;
//!
//! let bag = length::Bag::from_date_time_style(
//!     length::Date::Medium, // "medium" date connector will be used
//!     length::Time::Short,
//! );
//!
//! let options = DateTimeFormatterOptions::Length(bag);
//! ```
//!
//! Or the options can be inferred through the [`Into`] trait.
//!
//! ```
//! use icu::datetime::options::length;
//! use icu::datetime::DateTimeFormatterOptions;
//! let options: DateTimeFormatterOptions = length::Bag::default().into();
//! ```
//!
//! *Note*: The exact result returned from [`TypedDateTimeFormatter`](crate::TypedDateTimeFormatter) is a subject to change over
//! time. Formatted result should be treated as opaque and displayed to the user as-is,
//! and it is strongly recommended to never write tests that expect a particular formatted output.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A structure to represent the set of lengths in which the [`DateTimeInput`] implementer should be formatted to.
///
/// [`DateTimeInput`]: crate::input::DateTimeInput
///
/// The available lengths correspond to [`UTS #35: Unicode LDML 4. Dates`], section 2.4 [`Element dateFormats`].
///
/// # Examples
///
/// ```
/// use icu::datetime::options::length;
/// use icu::datetime::DateTimeFormatterOptions;
///
/// let bag = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Short,
/// );
///
/// let options = DateTimeFormatterOptions::Length(bag);
/// ```
///
/// Or the options can be inferred through the [`Into`] trait.
///
/// ```
/// use icu::datetime::options::length;
/// use icu::datetime::DateTimeFormatterOptions;
/// let options: DateTimeFormatterOptions = length::Bag::default().into();
/// ```
///
/// [`UTS #35: Unicode LDML 4. Dates`]: https://unicode.org/reports/tr35/tr35-dates.html
/// [`Element dateFormats`]: https://unicode.org/reports/tr35/tr35-dates.html#dateFormats
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Bag {
    /// Configure the date part of the datetime.
    pub date: Option<Date>,
    /// Configure the time part of the datetime.
    pub time: Option<Time>,
}

impl Default for Bag {
    /// Constructs a Bag with medium date and time options
    fn default() -> Self {
        Self {
            date: Some(Date::Medium),
            time: Some(Time::Medium),
        }
    }
}

impl Bag {
    /// Constructs a Bag with all fields set to None
    ///
    /// Note that the [`Default`] implementation returns medium date and time options
    pub fn empty() -> Self {
        Self {
            date: None,
            time: None,
        }
    }

    /// Constructs a Bag given a date and time field
    pub fn from_date_time_style(date: Date, time: Time) -> Self {
        Self {
            date: Some(date),
            time: Some(time),
        }
    }

    /// Constructs a Bag given a date field (time set to None)
    pub fn from_date_style(date: Date) -> Self {
        Self {
            date: Some(date),
            time: None,
        }
    }

    /// Constructs a Bag given a time field (date set to None)
    pub fn from_time_style(time: Time) -> Self {
        Self {
            date: None,
            time: Some(time),
        }
    }
}
/// Represents different lengths a [`DateTimeInput`] implementer can be formatted into.
/// Each length has associated best pattern for it for a given locale.
///
/// [`DateTimeInput`]: crate::input::DateTimeInput
///
/// # Examples
///
/// ```
/// use icu::datetime::options::length;
///
/// let bag = length::Bag::from_date_style(length::Date::Long);
/// ```
///
/// The available lengths correspond to [`UTS #35: Unicode LDML 4. Dates`], section 2.4 [`Element dateFormats`].
///
/// *Note*: The exact result returned from [`TypedDateTimeFormatter`] is a subject to change over
/// time. Formatted result should be treated as opaque and displayed to the user as-is,
/// and it is strongly recommended to never write tests that expect a particular formatted output.
///
/// [`UTS #35: Unicode LDML 4. Dates`]: https://unicode.org/reports/tr35/tr35-dates.html
/// [`Element dateFormats`]: https://unicode.org/reports/tr35/tr35-dates.html#dateFormats
/// [`TypedDateTimeFormatter`]: super::super::TypedDateTimeFormatter
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum Date {
    /// Full length, usually with weekday name.
    ///
    /// # Examples
    ///
    /// * Tuesday, January 21, 2020 (`en-US`)
    /// * wtorek, 21 stycznia, 2020 (`pl`)
    /// * الثلاثاء، ٢١ يناير ٢٠٢٠ (`ar`)
    /// * вторник, 21 января 2020 г. (`ru`)
    /// * 2020年1月21日火曜日 (`ja`)
    #[cfg_attr(feature = "serde", serde(rename = "full"))]
    Full,
    /// Long length, with wide month name.
    ///
    /// # Examples
    ///
    /// * September 10, 2020 (`en-US`)
    /// * 10 września 2020 (`pl`)
    /// * ١٠ سبتمبر ٢٠٢٠ (`ar`)
    /// * 10 сентября 2020 г. (`ru`)
    /// * 2020年9月10日 (`ja`)
    #[cfg_attr(feature = "serde", serde(rename = "long"))]
    Long,
    /// Medium length.
    ///
    /// # Examples
    ///
    /// * Feb 20, 2020 (`en-US`)
    /// * 20 lut 2020 (`pl`)
    /// * ٢٠‏/٠٢‏/٢٠٢٠ (`ar`)
    /// * 20 февр. 2020 г. (`ru`)
    /// * 2020/02/20 (`ja`)
    #[cfg_attr(feature = "serde", serde(rename = "medium"))]
    Medium,
    /// Short length, usually with numeric month.
    ///
    /// # Examples
    ///
    /// * 1/30/20 (`en-US`)
    /// * 30.01.2020 (`pl`)
    /// * ٣٠‏/١‏/٢٠٢٠ (`ar`)
    /// * 30.01.2020 (`ru`)
    /// * 2020/01/30 (`ja`)
    #[cfg_attr(feature = "serde", serde(rename = "short"))]
    Short,
}

/// Represents different length lengths a [`DateTimeInput`] implementer can be formatted into.
/// Each length has associated best pattern for it for a given locale.
///
/// [`DateTimeInput`]: crate::input::DateTimeInput
///
/// # Examples
///
/// ```
/// use icu::datetime::options::length;
///
/// let bag = length::Bag::from_time_style(length::Time::Medium);
/// ```
///
/// The available lengths correspond to [`UTS #35: Unicode LDML 4. Dates`], section 2.4 [`Element timeFormats`].
///
/// *Note*: The exact result returned from [`TypedDateTimeFormatter`] is a subject to change over
/// time. Formatted result should be treated as opaque and displayed to the user as-is,
/// and it is strongly recommended to never write tests that expect a particular formatted output.
///
/// [`UTS #35: Unicode LDML 4. Dates`]: https://unicode.org/reports/tr35/tr35-dates.html
/// [`Element dateFormats`]: https://unicode.org/reports/tr35/tr35-dates.html#timeFormats
/// [`TypedDateTimeFormatter`]: super::super::TypedDateTimeFormatter
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum Time {
    /// Full length, with spelled out time zone name.
    ///
    /// # Examples
    ///
    /// * 8:25:07 AM Pacific Standard Time (`en-US`)
    /// * 08:25:07 czas pacyficzny standardowy (`pl`)
    /// * ٨:٢٥:٠٧ ص توقيت المحيط الهادي الرسمي (`ar`)
    /// * 08:25:07 Тихоокеанское стандартное время (`ru`)
    /// * 8時25分07秒 アメリカ太平洋標準時 (`ja`)
    #[cfg_attr(feature = "serde", serde(rename = "full"))]
    Full,
    /// Full length, usually with short time-zone code.
    ///
    /// # Examples
    ///
    /// * 8:25:07 AM PST (`en-US`)
    /// * 08:25:07 GMT-8 (`pl`)
    /// * ٨:٢٥:٠٧ ص غرينتش-٨ (`ar`)
    /// * 08:25:07 GMT-8 (`ru`)
    /// * 8:25:07 GMT-8 (`ja`)
    #[cfg_attr(feature = "serde", serde(rename = "long"))]
    Long,
    /// Full length, usually with seconds.
    ///
    /// # Examples
    ///
    /// * 8:25:07 AM (`en-US`)
    /// * 08:25:07 (`pl`)
    /// * ٨:٢٥:٠٧ ص (`ar`)
    /// * 08:25:07 (`ru`)
    /// * 8:25:07 (`ja`)
    #[cfg_attr(feature = "serde", serde(rename = "medium"))]
    Medium,
    /// Full length, usually without seconds.
    ///
    /// # Examples
    ///
    /// * 8:25 AM (`en-US`)
    /// * 08:25 (`pl`)
    /// * ٨:٢٥ ص (`ar`)
    /// * 08:25 (`ru`)
    /// * 8:25 (`ja`)
    #[cfg_attr(feature = "serde", serde(rename = "short"))]
    Short,
}
