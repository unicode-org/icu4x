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
use crate::fields::{self, Field, FieldLength, FieldSymbol};

use super::preferences;
#[cfg(all(not(feature = "serialize_none"), feature = "serde"))]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Serialize, Deserialize)
)]
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

    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(skip_serializing, skip_deserializing)
    )]
    pub preferences: Option<preferences::Bag>,
}

impl Bag {
    /// Converts the components::Bag into a Vec<Field>. The fields will be ordered in from most
    /// significant field to least significant. This is the order the fields are listed in
    /// the UTS 35 table - https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
    pub fn to_vec_fields(&self) -> Vec<Field> {
        let mut fields = Vec::new();
        if let Some(_era) = self.era {
            // FieldSymbol::Era is needed.
            unimplemented!()
        }

        if let Some(year) = self.year {
            // Unimplemented year fields:
            // Y - Week of Year
            // u - Extended year
            // U - Cyclic year name
            // r - Related Gregorian year
            fields.push(Field {
                symbol: FieldSymbol::Year(fields::Year::Calendar),
                length: match year {
                    // Calendar year (numeric).
                    // y       2, 20, 201, 2017, 20173
                    // yy      02, 20, 01, 17, 73
                    // yyy     002, 020, 201, 2017, 20173    (not implemented)
                    // yyyy    0002, 0020, 0201, 2017, 20173 (not implemented)
                    // yyyyy+  ...                           (not implemented)
                    Numeric::Numeric => FieldLength::One,
                    Numeric::TwoDigit => FieldLength::TwoDigit,
                },
            });
        }

        // Unimplemented quarter fields:
        // Q - Quarter number/name
        // q - Stand-alone quarter

        if let Some(month) = self.month {
            fields.push(Field {
                symbol: if self.day.is_some() {
                    FieldSymbol::Month(fields::Month::Format)
                } else {
                    FieldSymbol::Month(fields::Month::StandAlone)
                },
                length: match month {
                    // (intended to be used in conjunction with ‘d’ for day number).
                    // M      9, 12      Numeric: minimum digits
                    // MM     09, 12     Numeric: 2 digits, zero pad if needed
                    // MMM    Sep        Abbreviated
                    // MMMM   September  Wide
                    // MMMMM  S          Narrow
                    Month::Numeric => FieldLength::One,
                    Month::TwoDigit => FieldLength::TwoDigit,
                    Month::Long => FieldLength::Wide,
                    Month::Short => FieldLength::Abbreviated,
                    Month::Narrow => FieldLength::Narrow,
                },
            });
        }

        // Unimplemented week fields:
        // w - Week of year
        // W - Week of month

        if let Some(day) = self.day {
            // Unimplemented day fields:
            // D - Day of year
            // F - Day of week in month
            // g - Modified Julian day.
            fields.push(Field {
                symbol: FieldSymbol::Day(fields::Day::DayOfMonth),
                length: match day {
                    // Day of month (numeric).
                    // d    1 	  Numeric: minimum digits
                    // dd   01 	  Numeric: 2 digits, zero pad if needed
                    Numeric::Numeric => FieldLength::One,
                    Numeric::TwoDigit => FieldLength::TwoDigit,
                },
            });
        }

        if let Some(weekday) = self.weekday {
            // Unimplemented fields
            // e - Local day of week.
            // c - Stand-alone local day of week.
            fields.push(Field {
                symbol: FieldSymbol::Weekday(fields::Weekday::Format),
                length: match weekday {
                    // Day of week name, format style.
                    //
                    // E..EEE   Tue      Abbreviated
                    // EEEE     Tuesday  Wide
                    // EEEEE    T 	     Narrow
                    // EEEEEE   Tu       Short
                    Text::Long => FieldLength::Wide,
                    Text::Short => FieldLength::One,
                    Text::Narrow => FieldLength::Narrow,
                },
            });
        }

        // Unimplemented period fields:
        // a - AM, PM
        // b - am, pm, noon, midnight
        // c - flexible day periods

        if let Some(hour) = self.hour {
            // Unimplemented fields
            // e - Local day of week.
            // c - Stand-alone local day of week.

            // fields::Hour::H11
            // fields::Hour::H12
            // fields::Hour::H23
            // fields::Hour::H24

            // When used in skeleton data or in a skeleton passed in an API for flexible date
            // pattern generation, it should match the 12-hour-cycle format preferred by the
            // locale (h or K); it should not match a 24-hour-cycle format (H or k).
            fields.push(Field {
                symbol: FieldSymbol::Hour(match self.preferences {
                    Some(preferences::Bag {
                        hour_cycle: Some(hour_cycle),
                    }) => match hour_cycle {
                        // k - symbol
                        preferences::HourCycle::H24 => fields::Hour::H24,
                        // H - symbol
                        preferences::HourCycle::H23 => fields::Hour::H23,
                        // h - symbol
                        preferences::HourCycle::H12 => fields::Hour::H12,
                        // K - symbol
                        preferences::HourCycle::H11 => fields::Hour::H11,
                    },
                    // No preference
                    // TODO - What should this default to?
                    _ => fields::Hour::H24,
                }),
                length: match hour {
                    // Example for h: (note that this is the same for k, K, and H)
                    // h     1, 12  Numeric: minimum digits
                    // hh   01, 12  Numeric: 2 digits, zero pad if needed
                    Numeric::Numeric => FieldLength::One,
                    Numeric::TwoDigit => FieldLength::TwoDigit,
                },
            });
        }

        if let Some(minute) = self.minute {
            // m   8, 59    Numeric: minimum digits
            // mm  08, 59   Numeric: 2 digits, zero pad if needed
            fields.push(Field {
                symbol: FieldSymbol::Minute,
                length: match minute {
                    Numeric::Numeric => FieldLength::One,
                    Numeric::TwoDigit => FieldLength::TwoDigit,
                },
            });
        }

        if let Some(second) = self.second {
            // s    8, 12    Numeric: minimum digits
            // ss  08, 12    Numeric: 2 digits, zero pad if needed
            fields.push(Field {
                symbol: FieldSymbol::Second(fields::Second::Second),
                length: match second {
                    Numeric::Numeric => FieldLength::One,
                    Numeric::TwoDigit => FieldLength::TwoDigit,
                },
            });
            // S - Not used in skeletons.
            // A - Milliseconds in day. Not used in skeletons.
        }

        // TODO - Implement:
        // if self.time_zone_name.is_some() {
        //     unimplemented!();
        // }

        fields
    }
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
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum Numeric {
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "numeric")
    )]
    Numeric,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "two-digit")
    )]
    TwoDigit,
}

impl Numeric {
    pub fn matches_field_length(&self, length: FieldLength) -> bool {
        length
            == match self {
                Numeric::Numeric => FieldLength::One,
                Numeric::TwoDigit => FieldLength::TwoDigit,
            }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum Text {
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "long")
    )]
    Long,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "short")
    )]
    Short,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "narrow")
    )]
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum Month {
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "numeric")
    )]
    Numeric,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "two-digit")
    )]
    TwoDigit,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "long")
    )]
    Long,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "short")
    )]
    Short,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "narrow")
    )]
    Narrow,
}

impl Month {
    pub fn matches_field_length(&self, length: FieldLength) -> bool {
        length
            == match self {
                Month::Numeric => FieldLength::One,
                Month::TwoDigit => FieldLength::TwoDigit,
                Month::Short => FieldLength::Abbreviated,
                Month::Long => FieldLength::Wide,
                Month::Narrow => FieldLength::Narrow,
            }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    all(not(feature = "serialize_none"), feature = "serde"),
    derive(Serialize, Deserialize)
)]
pub enum TimeZoneName {
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "long")
    )]
    Long,
    #[cfg_attr(
        all(not(feature = "serialize_none"), feature = "serde"),
        serde(rename = "short")
    )]
    Short,
}

#[cfg(test)]
mod test {
    use super::*;

    // Shorten these for terser tests.
    type FS = FieldSymbol;
    type FL = FieldLength;

    #[test]
    fn test_component_bag_to_vec_field() {
        let bag = Bag::default();
        assert_eq!(
            bag.to_vec_fields(),
            vec![
                (FS::Year(fields::Year::Calendar), FL::One).into(),
                (FS::Month(fields::Month::Format), FL::Wide).into(),
                (FS::Day(fields::Day::DayOfMonth), FL::One).into(),
                (FS::Hour(fields::Hour::H24), FL::One).into(),
                (FS::Minute, FL::One).into(),
                (FS::Second(fields::Second::Second), FL::One).into(),
            ]
        );
    }

    #[test]
    fn test_component_bag_to_vec_field2() {
        let bag = Bag {
            era: None,
            year: Some(Numeric::Numeric),
            month: Some(Month::TwoDigit),
            day: Some(Numeric::Numeric),
            weekday: None,
            hour: None,
            minute: None,
            second: None,
            time_zone_name: None,
            preferences: None,
        };
        assert_eq!(
            bag.to_vec_fields(),
            vec![
                (FS::Year(fields::Year::Calendar), FL::One).into(),
                (FS::Month(fields::Month::Format), FL::TwoDigit).into(),
                (FS::Day(fields::Day::DayOfMonth), FL::One).into(),
            ]
        );
    }
}
