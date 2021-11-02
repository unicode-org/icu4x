// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # Implementation status
//!
//! This is currently only a partial implementation of the UTS-35 skeleton matching algorithm.
//!
//! | Algorithm step | Status |
//! |----------------|--------|
//! | Match skeleton fields according to a ranking             | Implemented |
//! | Adjust the matched pattern to have certain widths        | Implemented |
//! | Match date and times separately, and them combine them   | Implemented |
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
//! should be included in in a datetime, and how they should be displayed. There is not a strict
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
//! use icu::datetime::DateTimeFormatOptions;
//! use icu::datetime::options::components;
//!
//! let bag = components::Bag {
//!     year: Some(components::Year::Numeric),
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
//! use icu::datetime::DateTimeFormatOptions;
//! use icu::datetime::options::components;
//! let options: DateTimeFormatOptions = components::Bag::default().into();
//! ```
//!
//! *Note*: The exact result returned from [`DateTimeFormat`](crate::DateTimeFormat) is a subject to change over
//! time. Formatted result should be treated as opaque and displayed to the user as-is,
//! and it is strongly recommended to never write tests that expect a particular formatted output.
use crate::fields::{self, Field, FieldLength, FieldSymbol};

use alloc::vec::Vec;

use super::preferences;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// See the [module-level](./index.html) docs for more information.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bag {
    /// Include the era, such as "AD" or "CE".
    pub era: Option<Text>,
    /// Include the year, such as "1970" or "70".
    pub year: Option<Year>,
    /// Include the month, such as "April" or "Apr".
    pub month: Option<Month>,
    /// Include the week, such as "1st" or "1".
    #[doc(hidden)]
    // TODO(#488): make visible once fully supported.
    pub week: Option<Week>,
    /// Include the day, such as "07" or "7".
    pub day: Option<Numeric>,
    /// Include the weekday, such as "Wednesday" or "Wed".
    pub weekday: Option<Text>,

    /// Include the hour such as "2" or "14".
    pub hour: Option<Numeric>,
    /// Include the minute such as "3" or "03".
    pub minute: Option<Numeric>,
    /// Include the second such as "3" or "03".
    pub second: Option<Numeric>,

    /// Include the time zone, such as "GMT+05:00".
    pub time_zone_name: Option<TimeZoneName>,

    /// Adjust the preferences for the date, such as setting the hour cycle.
    pub preferences: Option<preferences::Bag>,
}

impl Bag {
    #[allow(clippy::wrong_self_convention)]
    /// Converts the components::Bag into a Vec<Field>. The fields will be ordered in from most
    /// significant field to least significant. This is the order the fields are listed in
    /// the UTS 35 table - https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
    pub(crate) fn to_vec_fields(&self) -> Vec<Field> {
        let mut fields = Vec::new();
        if let Some(era) = self.era {
            fields.push(Field {
                symbol: FieldSymbol::Era,
                length: match era {
                    // Era name, format length.
                    //
                    // G..GGG  AD           Abbreviated
                    // GGGG    Anno Domini  Wide
                    // GGGGG   A            Narrow
                    Text::Short => FieldLength::Abbreviated,
                    Text::Long => FieldLength::Wide,
                    Text::Narrow => FieldLength::Narrow,
                },
            })
        }

        if let Some(year) = self.year {
            // Unimplemented year fields:
            // u - Extended year
            // U - Cyclic year name
            // r - Related Gregorian year
            fields.push(Field {
                symbol: FieldSymbol::Year(match year {
                    Year::Numeric | Year::TwoDigit => fields::Year::Calendar,
                    Year::NumericWeekOf | Year::TwoDigitWeekOf => fields::Year::WeekOf,
                }),
                length: match year {
                    // Calendar year (numeric).
                    // y       2, 20, 201, 2017, 20173
                    // yy      02, 20, 01, 17, 73
                    // yyy     002, 020, 201, 2017, 20173    (not implemented)
                    // yyyy    0002, 0020, 0201, 2017, 20173 (not implemented)
                    // yyyyy+  ...                           (not implemented)
                    Year::Numeric | Year::NumericWeekOf => FieldLength::One,
                    Year::TwoDigit | Year::TwoDigitWeekOf => FieldLength::TwoDigit,
                },
            });
        }

        // TODO(#501) - Unimplemented quarter fields:
        // Q - Quarter number/name
        // q - Stand-alone quarter

        if let Some(month) = self.month {
            fields.push(Field {
                // Always choose Month::Format as Month::StandAlone is not used in skeletons.
                symbol: FieldSymbol::Month(fields::Month::Format),
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

        if let Some(week) = self.week {
            fields.push(Field {
                symbol: FieldSymbol::Week(match week {
                    Week::WeekOfMonth => fields::Week::WeekOfMonth,
                    Week::NumericWeekOfYear | Week::TwoDigitWeekOfYear => fields::Week::WeekOfYear,
                }),
                length: match week {
                    Week::WeekOfMonth | Week::NumericWeekOfYear => FieldLength::One,
                    Week::TwoDigitWeekOfYear => FieldLength::TwoDigit,
                },
            });
        }

        if let Some(day) = self.day {
            // TODO(#591,#592) Unimplemented day fields:
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
            // TODO(#593) Unimplemented fields
            // e - Local day of week.
            // c - Stand-alone local day of week.
            fields.push(Field {
                symbol: FieldSymbol::Weekday(fields::Weekday::Format),
                length: match weekday {
                    // Day of week name, format length.
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

        // The period fields are not included in skeletons:
        // a - AM, PM
        // b - am, pm, noon, midnight
        // c - flexible day periods

        if let Some(hour) = self.hour {
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
                        // Skeletons only contain the h12, not h11. The pattern that is matched
                        // is free to use h11 or h12.
                        preferences::HourCycle::H11 | preferences::HourCycle::H12 => {
                            // h - symbol
                            fields::Hour::H12
                        }
                        // Skeletons only contain the h23, not h24. The pattern that is matched
                        // is free to use h23 or h24.
                        preferences::HourCycle::H24 | preferences::HourCycle::H23 => {
                            // H - symbol
                            fields::Hour::H23
                        }
                    },
                    // TODO(#594) - This should default should be the locale default, which is
                    // region-based (h12 for US, h23 for GB, etc). This is in CLDR, but we need
                    // to load it as well as think about the best architecture for where that
                    // data loading code should reside.
                    _ => fields::Hour::H23,
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

        if self.time_zone_name.is_some() {
            // Only the lower "v" field is used in skeletons.
            fields.push(Field {
                symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerV),
                length: FieldLength::One,
            });
        }

        debug_assert!(
            fields.windows(2).all(|f| f[0] < f[1]),
            "The fields are sorted and unique."
        );

        fields
    }
}

/// A numeric component for the `components::`[`Bag`]. It is used for the year, day, hour, minute,
/// and second.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Numeric {
    /// Display the numeric value. For instance in a year this would be "1970".
    #[cfg_attr(feature = "serde", serde(rename = "numeric"))]
    Numeric,
    /// Display the two digit value. For instance in a year this would be "70".
    #[cfg_attr(feature = "serde", serde(rename = "two-digit"))]
    TwoDigit,
}

/// A text component for the `components::`[`Bag`]. It is used for the era and weekday.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Text {
    /// Display the long form of the text, such as "Wednesday" for the weekday.
    #[cfg_attr(feature = "serde", serde(rename = "long"))]
    Long,
    /// Display the short form of the text, such as "Wed" for the weekday.
    #[cfg_attr(feature = "serde", serde(rename = "short"))]
    Short,
    /// Display the narrow form of the text, such as "W" for the weekday.
    #[cfg_attr(feature = "serde", serde(rename = "narrow"))]
    Narrow,
}

/// Options for displaying a Year for the `components::`[`Bag`].
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Year {
    /// The numeric value of the year, such as "2018" for 2018-12-31.
    #[cfg_attr(feature = "serde", serde(rename = "numeric"))]
    Numeric,
    /// The two-digit value of the year, such as "18" for 2018-12-31.
    #[cfg_attr(feature = "serde", serde(rename = "two-digit"))]
    TwoDigit,
    /// The numeric value of the year in "week-of-year", such as "2019" for the
    /// week of 2018-12-31 according to the ISO calendar.
    NumericWeekOf,
    /// The two-digit value of the year in "week-of-year", such as "19" for the
    /// week of 2018-12-31 according to the ISO calendar.
    TwoDigitWeekOf,
}

/// Options for displaying a Month for the `components::`[`Bag`].
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Month {
    /// The numeric value of the month, such as "4".
    #[cfg_attr(feature = "serde", serde(rename = "numeric"))]
    Numeric,
    /// The two-digit value of the month, such as "04".
    #[cfg_attr(feature = "serde", serde(rename = "two-digit"))]
    TwoDigit,
    /// The two-digit value of the month, such as "April".
    #[cfg_attr(feature = "serde", serde(rename = "long"))]
    Long,
    /// The short value of the month, such as "Apr".
    #[cfg_attr(feature = "serde", serde(rename = "short"))]
    Short,
    /// The narrow value of the month, such as "A".
    #[cfg_attr(feature = "serde", serde(rename = "narrow"))]
    Narrow,
}

// Each enum variant is documented with the UTS 35 field information from:
// https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
//
/// Options for displaying the current week for the `components::`[`Bag`].
#[doc(hidden)]
// TODO(#488): make visible once fully supported.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Week {
    /// The week of the month, such as "3".
    WeekOfMonth,
    /// The numeric value of the week of the year, such as "8".
    NumericWeekOfYear,
    /// The two-digit value of the week of the year, such as "08".
    TwoDigitWeekOfYear,
}

/// Options for displaying a time zone for the `components::`[`Bag`].
///
/// Note that the initial implementation is focusing on only supporting ECMA-402 compatible
/// options.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TimeZoneName {
    // UTS-35 fields: z..zzz
    //
    /// Short localized form, without the location. (e.g.: PST, GMT-8)
    #[cfg_attr(feature = "serde", serde(rename = "shortSpecific"))]
    ShortSpecific,

    // UTS-35 fields: zzzz
    // Per UTS-35: [long form] specific non-location (falling back to long localized GMT)
    //
    /// Long localized form, without the location (e.g., Pacific Standard Time, Nordamerikanische Westküsten-Normalzeit)
    #[cfg_attr(feature = "serde", serde(rename = "longSpecific"))]
    LongSpecific,

    // UTS-35 fields: O, OOOO
    // Per UTS-35: The long localized GMT format. This is equivalent to the "OOOO" specifier
    // Per UTS-35: Short localized GMT format (e.g., GMT-8)
    // This enum variant is combining the two types of fields, as the CLDR specifices the preferred
    // hour-format for the locale, and ICU4X uses the preferred one.
    //   e.g.
    //   https://github.com/unicode-org/cldr-json/blob/c23635f13946292e40077fd62aee6a8e122e7689/cldr-json/cldr-dates-full/main/es-MX/timeZoneNames.json#L13
    //
    /// Localized GMT format, in the locale's preferred hour format. (e.g., GMT-0800),
    #[cfg_attr(feature = "serde", serde(rename = "gmtOffset"))]
    GmtOffset,

    // UTS-35 fields: v
    //   * falling back to generic location (See UTS 35 for more specific rules)
    //   * falling back to short localized GMT
    /// Short generic non-location format (e.g.: PT, Los Angeles, Zeit).
    #[cfg_attr(feature = "serde", serde(rename = "shortGeneric"))]
    ShortGeneric,

    // UTS-35 fields: vvvv
    //  * falling back to generic location (See UTS 35 for more specific rules)
    //  * falling back to long localized GMT
    /// Long generic non-location format (e.g.: Pacific Time, Nordamerikanische Westküstenzeit),
    #[cfg_attr(feature = "serde", serde(rename = "longGeneric"))]
    LongGeneric,
}

impl From<TimeZoneName> for Field {
    fn from(time_zone_name: TimeZoneName) -> Self {
        match time_zone_name {
            TimeZoneName::ShortSpecific => Field {
                symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerZ),
                length: FieldLength::One,
            },
            TimeZoneName::LongSpecific => Field {
                symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerZ),
                length: FieldLength::Wide,
            },
            TimeZoneName::GmtOffset => Field {
                symbol: FieldSymbol::TimeZone(fields::TimeZone::UpperO),
                length: FieldLength::Wide,
            },
            TimeZoneName::ShortGeneric => Field {
                symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerV),
                length: FieldLength::One,
            },
            TimeZoneName::LongGeneric => Field {
                symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerV),
                length: FieldLength::Wide,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Shorten these for terser tests.
    type Symbol = FieldSymbol;
    type Length = FieldLength;

    #[test]
    fn test_component_bag_to_vec_field() {
        let bag = Bag {
            year: Some(Year::Numeric),
            month: Some(Month::Long),
            week: Some(Week::WeekOfMonth),
            day: Some(Numeric::Numeric),

            hour: Some(Numeric::Numeric),
            minute: Some(Numeric::Numeric),
            second: Some(Numeric::Numeric),

            ..Default::default()
        };
        assert_eq!(
            bag.to_vec_fields(),
            vec![
                (Symbol::Year(fields::Year::Calendar), Length::One).into(),
                (Symbol::Month(fields::Month::Format), Length::Wide).into(),
                (Symbol::Week(fields::Week::WeekOfMonth), Length::One).into(),
                (Symbol::Day(fields::Day::DayOfMonth), Length::One).into(),
                (Symbol::Hour(fields::Hour::H23), Length::One).into(),
                (Symbol::Minute, Length::One).into(),
                (Symbol::Second(fields::Second::Second), Length::One).into(),
            ]
        );
    }

    #[test]
    fn test_component_bag_to_vec_field2() {
        let bag = Bag {
            year: Some(Year::Numeric),
            month: Some(Month::TwoDigit),
            day: Some(Numeric::Numeric),
            ..Default::default()
        };
        assert_eq!(
            bag.to_vec_fields(),
            vec![
                (Symbol::Year(fields::Year::Calendar), Length::One).into(),
                (Symbol::Month(fields::Month::Format), Length::TwoDigit).into(),
                (Symbol::Day(fields::Day::DayOfMonth), Length::One).into(),
            ]
        );
    }
}
