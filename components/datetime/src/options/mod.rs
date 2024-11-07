// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! [`DateTimeFormatterOptions`] is a bag of options which, together with [`Locale`],
//! defines how dates will be formatted.
//!
//! Each variant of the bag is a combination of settings defining how to format
//! the date, with an optional `Preferences` which represent user preferences and
//! may alter how the selected pattern is formatted.
//!
//! [`Locale`]: icu_locale_core::Locale
//! [`FixedCalendarDateTimeFormatter`]: crate::FixedCalendarDateTimeFormatter
//!
//! # Examples
//!
//! ```
//! use icu::datetime::options::length;
//! use icu::datetime::options::DateTimeFormatterOptions;
//!
//! let bag = length::Bag::from_date_time_style(
//!     length::Date::Medium,
//!     length::Time::Short,
//! );
//! ```
//!
//! At the moment only the [`length::Bag`] works, and we plan to extend that to support
//! `ECMA402`-like components bag later.

#[cfg(feature = "datagen")]
pub mod components;
#[cfg(feature = "datagen")]
pub mod length;

#[cfg(feature = "datagen")]
pub mod preferences;
#[cfg(not(feature = "datagen"))]
pub(crate) mod preferences;

/// A bag of options which, together with [`Locale`](icu_locale_core::Locale), defines how
/// dates will be formatted with a formatter instance.
///
/// Each variant of the bag is a combination of settings defining how to format
/// the date, with an optional `Preferences` which represent user preferences and
/// may alter how the selected pattern is formatted.
///
/// # Examples
///
/// ```
/// use icu::datetime::options::length;
///
/// let bag = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Short,
/// );
/// ```
///
/// At the moment only the [`length::Bag`] works, and we plan to extend that to support
/// `ECMA402` like components bag later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(feature = "datagen")]
#[non_exhaustive]
pub enum DateTimeFormatterOptions {
    /// Bag of lengths for date and time.
    Length(length::Bag),
    /// Bag of components describing which fields and how should be displayed.
    ///
    /// ✨ *Enabled with the `experimental` Cargo feature.*
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. It can be enabled with the `experimental` Cargo feature
    /// of the icu meta-crate. Use with caution.
    /// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
    /// </div>
    #[cfg(feature = "datagen")]
    Components(components::Bag),
}

#[cfg(feature = "datagen")]
impl Default for DateTimeFormatterOptions {
    fn default() -> Self {
        Self::Length(length::Bag::default())
    }
}

#[cfg(feature = "datagen")]
impl From<length::Bag> for DateTimeFormatterOptions {
    fn from(input: length::Bag) -> Self {
        Self::Length(input)
    }
}

#[cfg(feature = "datagen")]
impl From<components::Bag> for DateTimeFormatterOptions {
    fn from(input: components::Bag) -> Self {
        Self::Components(input)
    }
}
