// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! [`DateTimeFormatOptions`] is a bag of options which, together with [`Locale`],
//! defines how dates will be formatted with a [`DateTimeFormat`] instance.
//!
//! Each variant of the bag is a combination of settings defining how to format
//! the date, with an optional `Preferences` which represent user preferences and
//! may alter how the selected pattern is formatted.
//!
//! [`Locale`]: icu_locid::Locale
//! [`DateTimeFormat`]: crate::DateTimeFormat
//!
//! # Examples
//!
//! ```
//! use icu::datetime::{DateTimeFormatOptions, options::length};
//!
//! let bag = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
//! ```
//!
//! At the moment only the [`length::Bag`] works, and we plan to extend that to support
//! `ECMA402`-like components bag later.

pub mod components;
pub mod length;
pub mod preferences;
/// A bag of options which, together with [`Locale`](icu_locid::Locale), defines how
/// dates will be formatted with a [`DateTimeFormat`](crate::DateTimeFormat) instance.
///
/// Each variant of the bag is a combination of settings defining how to format
/// the date, with an optional `Preferences` which represent user preferences and
/// may alter how the selected pattern is formatted.
///
/// # Examples
///
/// ```
/// use icu::datetime::{DateTimeFormatOptions, options::length};
///
/// let bag = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
/// ```
///
/// At the moment only the [`length::Bag`] works, and we plan to extend that to support
/// `ECMA402` like components bag later.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DateTimeFormatOptions {
    /// Bag of lengths for date and time.
    Length(length::Bag),
    /// Bag of components describing which fields and how should be displayed.
    Components(components::Bag),
}

impl Default for DateTimeFormatOptions {
    fn default() -> Self {
        Self::Length(length::Bag::default())
    }
}

impl From<length::Bag> for DateTimeFormatOptions {
    fn from(input: length::Bag) -> Self {
        Self::Length(input)
    }
}

impl From<components::Bag> for DateTimeFormatOptions {
    fn from(input: components::Bag) -> Self {
        Self::Components(input)
    }
}
