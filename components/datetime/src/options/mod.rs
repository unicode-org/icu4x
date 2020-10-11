// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! `DateTimeFormatOptions` is a bag of options which, together with `LanguageIdentifier`,
//! define how dates will be formatted be a `DateTimeFormat` instance.
//!
//! Each variant of the bag is a combination of settings definiting how to format
//! the date, with an optional `Preferences` which represent user preferences and
//! may alter how the selected pattern is formatted.
//!
//! # Examples
//!
//! ```
//! use icu_datetime::{DateTimeFormatOptions, options::style};
//!
//! let options = DateTimeFormatOptions::Style(
//!     style::Bag {
//!          date: Some(style::Date::Medium),
//!          time: Some(style::Time::Short),
//!         ..Default::default()
//!     }
//! );
//! ```
//!
//! At the moment only the `Style` bag works, and we plan to extend that to support
//! `ECMA 402` like components bag later.
pub mod components;
pub mod preferences;
pub mod style;
/// `DateTimeFormatOptions` is a bag of options which, together with `LanguageIdentifier`,
/// define how dates will be formatted be a `DateTimeFormat` instance.
///
/// Each variant of the bag is a combination of settings definiting how to format
/// the date, with an optional `Preferences` which represent user preferences and
/// may alter how the selected pattern is formatted.
///
/// # Examples
///
/// ```
/// use icu_datetime::{DateTimeFormatOptions, options::style};
///
/// let options = DateTimeFormatOptions::Style(
///     style::Bag {
///          date: Some(style::Date::Medium),
///          time: Some(style::Time::Short),
///         ..Default::default()
///     }
/// );
/// ```
///
/// At the moment only the `Style` bag works, and we plan to extend that to support
/// `ECMA 402` like components bag later.
#[derive(Debug)]
pub enum DateTimeFormatOptions {
    /// Bag of styles for date and time
    Style(style::Bag),
    /// Bag of components describing which fields and how should be displayed
    Components(components::Bag),
}

impl Default for DateTimeFormatOptions {
    fn default() -> Self {
        Self::Style(style::Bag::default())
    }
}

impl From<style::Bag> for DateTimeFormatOptions {
    fn from(input: style::Bag) -> Self {
        Self::Style(input)
    }
}

impl From<components::Bag> for DateTimeFormatOptions {
    fn from(input: components::Bag) -> Self {
        Self::Components(input)
    }
}
