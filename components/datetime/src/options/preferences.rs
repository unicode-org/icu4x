// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Experimental\] Types to hold user preferences to configure a DateTimeFormatter.
//!
//! ✨ *Enabled with the `experimental` Cargo feature.*
//!
//! Preferences is a bag of options to be associated with either [`length::Bag`] or [`components::Bag`]
//! which provides information on user preferences that can affect the result of the formatting.
//!
//! [`length::Bag`]: crate::options::length::Bag
//! [`components::Bag`]: crate::options::components::Bag
//!
//! <div class="stab unstable">
//! 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
//! of the icu meta-crate. Use with caution.
//! <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
//! </div>
//!
//! # Unicode Extensions
//! User preferences will often be stored as part of the [`Locale`] identified as `Unicode Extensions`, but
//! for scenarios where the application stores information about user preferences they can be also provided via
//! this bag (and if they are, they will take precedence over unicode extensions from the locale).
//!
//! [`Locale`]: icu_locale_core::Locale
//!
//! # Examples
//!
//! ```
//! use icu::datetime::options::preferences;
//!
//! let prefs = preferences::Bag::from_hour_cycle(preferences::HourCycle::H23);
//! ```
use crate::fields;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use icu_locale_core::{
    extensions::unicode::key,
    subtags::{subtag, Subtag},
};
use icu_provider::DataLocale;

/// Stores user preferences which may affect the result of date and time formatting.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
///
/// # Examples
///
/// ```
/// use icu::datetime::options::preferences;
///
/// let prefs = preferences::Bag::from_hour_cycle(preferences::HourCycle::H23);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Bag {
    /// The hour cycle can be adjusts according to user preferences, for instance at the OS-level.
    /// That preference can be applied here to change the hour cycle from the default for the
    /// given locale.
    #[cfg_attr(feature = "serde", serde(rename = "hourCycle"))]
    pub hour_cycle: Option<HourCycle>,
}

impl Bag {
    /// Construct a [`Bag`] with a given [`HourCycle`]
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    pub fn from_hour_cycle(h: HourCycle) -> Self {
        Self {
            hour_cycle: Some(h),
        }
    }

    /// Construct a [`Bag`] from a given [`DataLocale`]
    pub(crate) fn from_data_locale(data_locale: &DataLocale) -> Self {
        const H11: Subtag = subtag!("h11");
        const H12: Subtag = subtag!("h12");
        const H23: Subtag = subtag!("h23");
        const H24: Subtag = subtag!("h24");
        let hour_cycle = match data_locale
            .get_unicode_ext(&key!("hc"))
            .and_then(|v| v.into_single_subtag())
        {
            Some(H11) => Some(HourCycle::H11),
            Some(H12) => Some(HourCycle::H12),
            Some(H23) => Some(HourCycle::H23),
            Some(H24) => Some(HourCycle::H24),
            _ => None,
        };
        Self { hour_cycle }
    }
}

/// A user preference for adjusting how the hour component is displayed.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub enum HourCycle {
    /// Hour is formatted to be in range 1-24 where midnight is 24:00.
    ///
    /// # Examples
    ///
    /// ```
    /// "24:12";
    /// "8:23";
    /// "19:00";
    /// "23:21";
    /// ```
    #[cfg_attr(feature = "serde", serde(rename = "h24"))]
    H24,
    /// Hour is formatted to be in range 0-23 where midnight is 00:00.
    ///
    /// # Examples
    ///
    /// ```
    /// "0:12";
    /// "8:23";
    /// "19:00";
    /// "23:21";
    /// ```
    #[cfg_attr(feature = "serde", serde(rename = "h23"))]
    H23,
    /// Hour is formatted to be in range 1-12 where midnight is 12:00.
    ///
    /// # Examples
    ///
    /// ```
    /// "1:12";
    /// "8:23";
    /// "7:00";
    /// "11:21";
    /// ```
    #[cfg_attr(feature = "serde", serde(rename = "h12"))]
    H12,
    /// Hour is formatted to be in range 0-11 where midnight is 00:00.
    ///
    /// # Examples
    ///
    /// ```
    /// "0:12";
    /// "8:23";
    /// "7:00";
    /// "11:21";
    /// ```
    #[cfg_attr(feature = "serde", serde(rename = "h11"))]
    H11,
}

impl HourCycle {
    /// Convert the HourCycle preference to a field.
    pub fn field(self) -> fields::Hour {
        match self {
            Self::H11 => fields::Hour::H11,
            Self::H12 => fields::Hour::H12,
            Self::H23 => fields::Hour::H23,
            Self::H24 => fields::Hour::H24,
        }
    }
}
