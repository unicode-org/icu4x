// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Components is a model of encoding information on how to format date and time by specifying a list of components
//! the user wants to be visible in the formatted string and how each field should be displayed.
//!
//! This model closely corresponds to `ECMA402` API and allows for high level of customization compared to `Style` model.
//!
//! Additionally, the bag contains an optional set of `Preferences` which represent user preferred adjustments
//! that can be applied onto the pattern right before formatting.
//!
//! # Pattern Selection
//!
//! It is important to understand that the components bag is a human-friendly way to describe a skeleton, not a pattern.
//! That means that the components and their styles provided by the user will be matched against available patterns for
//! a given locale and the closest available pattern will be used for formatting.
//!
//! That means, that it is possible that if the user asks for a combination of fields or lengths that `CLDR` has no
//! data associated with, the selected pattern may be different than the selection in the `Components` bag.
//! Such scenarios should be rare.
//!
//! # Examples
//!
//! ```
//! use icu_datetime::options::components;
//!
//! let options = components::Bag {
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
//! ```
//!
//! *Note*: The exact result returned from [`DateTimeFormat`] is a subject to change over
//! time. Formatted result should be treated as opaque and displayed to the user as-is,
//! and it is strongly recommended to never write tests that expect a particular formatted output.
use super::preferences;

#[derive(Debug)]
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
pub enum Numeric {
    Numeric,
    TwoDigit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Text {
    Long,
    Short,
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Month {
    Numeric,
    TwoDigit,
    Long,
    Short,
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeZoneName {
    Long,
    Short,
}
