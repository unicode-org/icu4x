// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # Implementation status
//!
//! This is currently only a partial implementation of the UTS 35 skeleton matching algorithm.
//!
//! | Algorithm step | Status |
//! |----------------|--------|
//! | Match skeleton fields according to a ranking             | Implemented |
//! | Adjust the matched pattern to have certain widths        | Not yet implemented. See [issue #584](https://github.com/unicode-org/icu4x/issues/584) |
//! | Match date and times separately, and them combine them   | Not yet implemented. See [issue #585](https://github.com/unicode-org/icu4x/issues/585) |
//! | Use appendItems to fill in a pattern with missing fields | Not yet, and may not be fully implemented. See [issue #586](https://github.com/unicode-org/icu4x/issues/586) |
//!
//! # Description
//!
//! A [`components::Bag`](struct.Bag.html) is a model of encoding information on how to format date
//! and time by specifying a list of components the user wants to be visible in the formatted string
//! and how each field should be displayed.
//!
//! This model closely corresponds to `ECMA402` API and allows for high level of customization
//! compared to `Length` model.
//!
//! Additionally, the bag contains an optional set of `Preferences` which represent user
//! preferred adjustments that can be applied onto the pattern right before formatting.
//!
//! ## Pattern Selection
//!
//! The [`components::Bag`](struct.Bag.html) is a way for the developer to describe which components
//! should be included in in a date time, and how they should be displayed. There is not a strict
//! guarantee in how the final date will be displayed to the end user. The user's preferences and
//! locale information can override the developer preferences.
//!
//! The fields in the [`components::Bag`](struct.Bag.html) are matched against available patterns in
//! the `CLDR` locale data. A best fit is found, and presented to the user. This means that in
//! certain situations, and component combinations, fields will not have a match, or the match will
//! have a different type of presentation for a given locale.
//!
//!
//! # Examples
//!
//! ```
//! use icu_datetime::DateTimeFormatOptions;
//! use icu_datetime::options::components;
//!
//! let bag = components::Bag {
//!     year: Some(components::Numeric::Numeric),
//!     month: Some(components::Month::Long),
//!     day: Some(components::Numeric::Numeric),
//!
//!     hour: Some(components::Numeric::TwoDigit),
//!     minute: Some(components::Numeric::TwoDigit),
//!
//!     preferences: None,
//!
//!     ..Default::default()
//! };
//!
//! // The options can be created manually.
//! let options = DateTimeFormatOptions::Components(bag);
//! ```
//!
//! Or the options can be inferred through the `.into()` trait.
//!
//! ```
//! # use icu_datetime::DateTimeFormatOptions;
//! # use icu_datetime::options::components;
//! let options: DateTimeFormatOptions = components::Bag::default().into();
//! ```
//!
//! *Note*: The exact result returned from [`DateTimeFormat`](crate::DateTimeFormat) is a subject to change over
//! time. Formatted result should be treated as opaque and displayed to the user as-is,
//! and it is strongly recommended to never write tests that expect a particular formatted output.
use super::preferences;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// See the [module-level](./index.html) docs for more information.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bag {
    pub era: Option<Text>,
    pub year: Option<Numeric>,
    pub month: Option<Month>,
    pub day: Option<Numeric>,
    pub weekday: Option<Text>,

    pub hour: Option<Numeric>,
    pub minute: Option<Numeric>,
    pub second: Option<Numeric>,

    pub time_zone_name: Option<TimeZoneName>,

    #[cfg_attr(feature = "serde", serde(skip_serializing, skip_deserializing))]
    pub preferences: Option<preferences::Bag>,
}

impl Default for Bag {
    fn default() -> Self {
        Self {
            era: None,
            year: Some(Numeric::Numeric),
            month: Some(Month::Long),
            day: Some(Numeric::Numeric),
            weekday: None,

            hour: Some(Numeric::Numeric),
            minute: Some(Numeric::Numeric),
            second: Some(Numeric::Numeric),

            time_zone_name: None,

            preferences: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Numeric {
    #[cfg_attr(feature = "serde", serde(rename = "numeric"))]
    Numeric,
    #[cfg_attr(feature = "serde", serde(rename = "two-digit"))]
    TwoDigit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Text {
    #[cfg_attr(feature = "serde", serde(rename = "long"))]
    Long,
    #[cfg_attr(feature = "serde", serde(rename = "short"))]
    Short,
    #[cfg_attr(feature = "serde", serde(rename = "narrow"))]
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Month {
    #[cfg_attr(feature = "serde", serde(rename = "numeric"))]
    Numeric,
    #[cfg_attr(feature = "serde", serde(rename = "two-digit"))]
    TwoDigit,
    #[cfg_attr(feature = "serde", serde(rename = "long"))]
    Long,
    #[cfg_attr(feature = "serde", serde(rename = "short"))]
    Short,
    #[cfg_attr(feature = "serde", serde(rename = "narrow"))]
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TimeZoneName {
    #[cfg_attr(feature = "serde", serde(rename = "long"))]
    Long,
    #[cfg_attr(feature = "serde", serde(rename = "short"))]
    Short,
}
