// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Preferences is a bag of options to be associated with either `Style` or `Components` bag which provides
//! information on user preferences that can affect the result of the formatting.
//!
//! # Unicode Extensions
//! User preferences will often be stored as part of the `Locale` identified as `Unicode Extensions`, but
//! for scenarios where the application stores information about user preferences they can be also provided via
//! this bag (and if they are, they will take precedence over unicode extensions from the locale).
//!
//! # Examples
//!
//! ```
//! use icu_datetime::options::preferences;
//!
//! let prefs = preferences::Bag {
//!     hour_cycle: Some(preferences::HourCycle::H23)
//! };
//! ```
use crate::fields;

/// Bag of preferences stores user preferences which may affect the result of date and time formatting.
///
/// # Examples
///
/// ```
/// use icu_datetime::options::preferences;
///
/// let prefs = preferences::Bag {
///     hour_cycle: Some(preferences::HourCycle::H23)
/// };
/// ```
#[derive(Debug)]
pub struct Bag {
    pub hour_cycle: Option<HourCycle>,
}

/// User Preference for adjusting how hour component is displayed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HourCycle {
    /// Hour is formatted to be in range 1-24
    ///
    /// # Examples
    ///
    /// ```
    /// "24:12";
    /// "8:23";
    /// "19:00";
    /// "23:21";
    /// ```
    H24,
    /// Hour is formatted to be in range 0-23
    ///
    /// # Examples
    ///
    /// ```
    /// "0:12";
    /// "8:23";
    /// "19:00";
    /// "23:21";
    /// ```
    H23,
    /// Hour is formatted to be in range 1-12
    ///
    /// # Examples
    ///
    /// ```
    /// "1:12";
    /// "8:23";
    /// "7:00";
    /// "11:21";
    /// ```
    H12,
    /// Hour is formatted to be in range 0-11
    ///
    /// # Examples
    ///
    /// ```
    /// "0:12";
    /// "8:23";
    /// "7:00";
    /// "11:21";
    /// ```
    H11,
}

impl HourCycle {
    pub fn field(&self) -> fields::Hour {
        match self {
            Self::H11 => fields::Hour::H11,
            Self::H12 => fields::Hour::H12,
            Self::H23 => fields::Hour::H23,
            Self::H24 => fields::Hour::H24,
        }
    }
}
