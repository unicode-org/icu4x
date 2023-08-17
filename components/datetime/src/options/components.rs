// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Experimental\] Options for constructing DateTimeFormatter objects by each component style.
//!
//! <div class="stab unstable">
//! 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
//! of the icu meta-crate. Use with caution.
//! <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
//! </div>
//!
//! # Implementation status
//!
//! This module is available by enabling the `"experimental"` Cargo feature.
//! It may change in breaking ways, including across minor releases.
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
//! use icu::datetime::options::components;
//! use icu::datetime::DateTimeFormatterOptions;
//!
//! let mut bag = components::Bag::default();
//! bag.year = Some(components::Year::Numeric);
//! bag.month = Some(components::Month::Long);
//! bag.day = Some(components::Day::NumericDayOfMonth);
//!
//! bag.hour = Some(components::Numeric::TwoDigit);
//! bag.minute = Some(components::Numeric::TwoDigit);
//!
//! // The options can be created manually.
//! let options = DateTimeFormatterOptions::Components(bag);
//! ```
//!
//! Or the options can be inferred through the `.into()` trait.
//!
//! ```
//! use icu::datetime::options::components;
//! use icu::datetime::DateTimeFormatterOptions;
//! let options: DateTimeFormatterOptions = components::Bag::default().into();
//! ```
//!
//! *Note*: The exact result returned from [`TypedDateTimeFormatter`](crate::TypedDateTimeFormatter) is a subject to change over
//! time. Formatted result should be treated as opaque and displayed to the user as-is,
//! and it is strongly recommended to never write tests that expect a particular formatted output.

use crate::{
    fields::{self, Field, FieldLength, FieldSymbol},
    pattern::{runtime::PatternPlurals, PatternItem},
};

#[cfg(feature = "experimental")]
use alloc::vec::Vec;

use super::preferences;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// See the [module-level](./index.html) docs for more information.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug, Clone, PartialEq, Default, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Bag {
    /// Include the era, such as "AD" or "CE".
    pub era: Option<Text>,
    /// Include the year, such as "1970" or "70".
    pub year: Option<Year>,
    /// Include the month, such as "April" or "Apr".
    pub month: Option<Month>,
    /// Include the week number, such as "51st" or "51" for week 51.
    pub week: Option<Week>,
    /// Include the day of the month/year, such as "07" or "7".
    pub day: Option<Day>,
    /// Include the weekday, such as "Wednesday" or "Wed".
    pub weekday: Option<Text>,

    /// Include the hour such as "2" or "14".
    pub hour: Option<Numeric>,
    /// Include the minute such as "3" or "03".
    pub minute: Option<Numeric>,
    /// Include the second such as "3" or "03".
    pub second: Option<Numeric>,
    /// Specify the number of fractional second digits such as 1 (".3") or 3 (".003").
    pub fractional_second: Option<u8>,

    /// Include the time zone, such as "GMT+05:00".
    pub time_zone_name: Option<TimeZoneName>,

    /// Adjust the preferences for the date, such as setting the hour cycle.
    pub preferences: Option<preferences::Bag>,
}

impl Bag {
    /// Creates an empty components bag
    ///
    /// Has the same behavior as the [`Default`] implementation on this type.
    pub fn empty() -> Self {
        Self::default()
    }

    #[allow(clippy::wrong_self_convention)]
    /// Converts the components::Bag into a Vec<Field>. The fields will be ordered in from most
    /// significant field to least significant. This is the order the fields are listed in
    /// the UTS 35 table - https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
    #[cfg(any(test, feature = "experimental"))] // only used in test and experimental code
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
            // TODO(#591) Unimplemented day fields:
            // D - Day of year
            // g - Modified Julian day.
            fields.push(Field {
                symbol: FieldSymbol::Day(match day {
                    Day::NumericDayOfMonth | Day::TwoDigitDayOfMonth => fields::Day::DayOfMonth,
                    Day::DayOfWeekInMonth => fields::Day::DayOfWeekInMonth,
                }),
                length: match day {
                    // d    1 	  Numeric day of month: minimum digits
                    // dd   01 	  Numeric day of month: 2 digits, zero pad if needed
                    // F    1  	  Numeric day of week in month: minimum digits
                    Day::NumericDayOfMonth | Day::DayOfWeekInMonth => FieldLength::One,
                    Day::TwoDigitDayOfMonth => FieldLength::TwoDigit,
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
            // A - Milliseconds in day. Not used in skeletons.
        }

        if let Some(precision) = self.fractional_second {
            // S - Fractional seconds.
            fields.push(Field {
                symbol: FieldSymbol::Second(fields::Second::FractionalSecond),
                length: FieldLength::Fixed(precision),
            });
        }

        if self.time_zone_name.is_some() {
            // Only the lower "v" field is used in skeletons.
            fields.push(Field {
                symbol: FieldSymbol::TimeZone(fields::TimeZone::LowerV),
                length: FieldLength::One,
            });
        }

        {
            #![allow(clippy::indexing_slicing)] // debug
            debug_assert!(
                fields.windows(2).all(|f| f[0] < f[1]),
                "The fields are sorted and unique."
            );
        }

        fields
    }
}

/// A numeric component for the `components::`[`Bag`]. It is used for the year, day, hour, minute,
/// and second.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Numeric {
    /// Display the numeric value. For instance in a year this would be "1970".
    Numeric,
    /// Display the two digit value. For instance in a year this would be "70".
    TwoDigit,
}

/// A text component for the `components::`[`Bag`]. It is used for the era and weekday.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Text {
    /// Display the long form of the text, such as "Wednesday" for the weekday.
    Long,
    /// Display the short form of the text, such as "Wed" for the weekday.
    Short,
    /// Display the narrow form of the text, such as "W" for the weekday.
    Narrow,
}

/// Options for displaying a Year for the `components::`[`Bag`].
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Year {
    /// The numeric value of the year, such as "2018" for 2018-12-31.
    Numeric,
    /// The two-digit value of the year, such as "18" for 2018-12-31.
    TwoDigit,
    /// The numeric value of the year in "week-of-year", such as "2019" in
    /// "week 01 of 2019" for the week of 2018-12-31 according to the ISO calendar.
    NumericWeekOf,
    /// The numeric value of the year in "week-of-year", such as "19" in
    /// "week 01 '19" for the week of 2018-12-31 according to the ISO calendar.
    TwoDigitWeekOf,
}

/// Options for displaying a Month for the `components::`[`Bag`].
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Month {
    /// The numeric value of the month, such as "4".
    Numeric,
    /// The two-digit value of the month, such as "04".
    TwoDigit,
    /// The long value of the month, such as "April".
    Long,
    /// The short value of the month, such as "Apr".
    Short,
    /// The narrow value of the month, such as "A".
    Narrow,
}

// Each enum variant is documented with the UTS 35 field information from:
// https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table

/// Options for displaying the current week number for the `components::`[`Bag`].
///
/// Week numbers are relative to either a month or year, e.g. 'week 3 of January' or 'week 40 of 2000'.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Week {
    /// The week of the month, such as the "3" in "week 3 of January".
    WeekOfMonth,
    /// The numeric value of the week of the year, such as the "8" in "week 8 of 2000".
    NumericWeekOfYear,
    /// The two-digit value of the week of the year, such as the "08" in "2000-W08".
    TwoDigitWeekOfYear,
}

/// Options for displaying the current day of the month or year.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum Day {
    /// The numeric value of the day of month, such as the "2" in July 2 1984.
    NumericDayOfMonth,
    /// The two digit value of the day of month, such as the "02" in 1984-07-02.
    TwoDigitDayOfMonth,
    /// The day of week in this month, such as the "2" in 2nd Wednesday of July.
    DayOfWeekInMonth,
}

/// Options for displaying a time zone for the `components::`[`Bag`].
///
/// Note that the initial implementation is focusing on only supporting ECMA-402 compatible
/// options.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "kebab-case")
)]
#[non_exhaustive]
pub enum TimeZoneName {
    // UTS-35 fields: z..zzz
    /// Short localized form, without the location. (e.g.: PST, GMT-8)
    ShortSpecific,

    // UTS-35 fields: zzzz
    // Per UTS-35: [long form] specific non-location (falling back to long localized GMT)
    /// Long localized form, without the location (e.g., Pacific Standard Time, Nordamerikanische Westküsten-Normalzeit)
    LongSpecific,

    // UTS-35 fields: O, OOOO
    // Per UTS-35: The long localized GMT format. This is equivalent to the "OOOO" specifier
    // Per UTS-35: Short localized GMT format (e.g., GMT-8)
    // This enum variant is combining the two types of fields, as the CLDR specifies the preferred
    // hour-format for the locale, and ICU4X uses the preferred one.
    //   e.g.
    //   https://github.com/unicode-org/cldr-json/blob/c23635f13946292e40077fd62aee6a8e122e7689/cldr-json/cldr-dates-full/main/es-MX/timeZoneNames.json#L13
    /// Localized GMT format, in the locale's preferred hour format. (e.g., GMT-0800),
    GmtOffset,

    // UTS-35 fields: v
    //   * falling back to generic location (See UTS 35 for more specific rules)
    //   * falling back to short localized GMT
    /// Short generic non-location format (e.g.: PT, Los Angeles, Zeit).
    ShortGeneric,

    // UTS-35 fields: vvvv
    //  * falling back to generic location (See UTS 35 for more specific rules)
    //  * falling back to long localized GMT
    /// Long generic non-location format (e.g.: Pacific Time, Nordamerikanische Westküstenzeit),
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

/// Get the resolved components for a TypedDateTimeFormatter, via the PatternPlurals. In the case of
/// plurals resolve off of the required `other` pattern.
impl<'data> From<&PatternPlurals<'data>> for Bag {
    fn from(other: &PatternPlurals) -> Self {
        let pattern = match other {
            PatternPlurals::SinglePattern(pattern) => pattern,
            PatternPlurals::MultipleVariants(plural_pattern) => &plural_pattern.other,
        };

        let mut bag: Bag = Default::default();

        // Transform the fields into components per:
        // https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
        for item in pattern.items.iter() {
            let field = match item {
                PatternItem::Field(ref field) => field,
                PatternItem::Literal(_) => continue,
            };
            match field.symbol {
                FieldSymbol::Era => {
                    bag.era = Some(match field.length {
                        FieldLength::One | FieldLength::TwoDigit | FieldLength::Abbreviated => {
                            Text::Short
                        }
                        FieldLength::Wide => Text::Long,
                        FieldLength::Narrow | FieldLength::Six | FieldLength::Fixed(_) => {
                            Text::Narrow
                        }
                    });
                }
                FieldSymbol::Year(year) => {
                    bag.year = Some(match year {
                        fields::Year::Calendar => match field.length {
                            FieldLength::TwoDigit => Year::TwoDigit,
                            _ => Year::Numeric,
                        },
                        fields::Year::WeekOf => match field.length {
                            FieldLength::TwoDigit => Year::TwoDigitWeekOf,
                            _ => Year::NumericWeekOf,
                        },
                        _ => todo!("TODO(#3762): Add support for U and r"),
                    });
                }
                FieldSymbol::Month(_) => {
                    // `Month::StandAlone` is only relevant in the pattern, so only differentiate
                    // on the field length.
                    bag.month = Some(match field.length {
                        FieldLength::One => Month::Numeric,
                        FieldLength::TwoDigit => Month::TwoDigit,
                        FieldLength::Abbreviated => Month::Short,
                        FieldLength::Wide => Month::Long,
                        FieldLength::Narrow | FieldLength::Six | FieldLength::Fixed(_) => {
                            Month::Narrow
                        }
                    });
                }
                FieldSymbol::Week(week) => {
                    bag.week = Some(match week {
                        fields::Week::WeekOfYear => match field.length {
                            FieldLength::TwoDigit => Week::TwoDigitWeekOfYear,
                            _ => Week::NumericWeekOfYear,
                        },
                        fields::Week::WeekOfMonth => Week::WeekOfMonth,
                    });
                }
                FieldSymbol::Day(day) => {
                    bag.day = Some(match day {
                        fields::Day::DayOfMonth => match field.length {
                            FieldLength::TwoDigit => Day::TwoDigitDayOfMonth,
                            _ => Day::NumericDayOfMonth,
                        },
                        fields::Day::DayOfYear => unimplemented!("fields::Day::DayOfYear #591"),
                        fields::Day::DayOfWeekInMonth => Day::DayOfWeekInMonth,
                        fields::Day::ModifiedJulianDay => {
                            unimplemented!("fields::Day::ModifiedJulianDay")
                        }
                    });
                }
                FieldSymbol::Weekday(weekday) => {
                    bag.weekday = Some(match weekday {
                        fields::Weekday::Format => match field.length {
                            FieldLength::One | FieldLength::TwoDigit | FieldLength::Abbreviated => {
                                Text::Short
                            }
                            FieldLength::Wide => Text::Long,
                            _ => Text::Narrow,
                        },
                        fields::Weekday::StandAlone => match field.length {
                            FieldLength::One | FieldLength::TwoDigit => {
                                // Stand-alone fields also support a numeric 1 digit per UTS-35, but there is
                                // no way to request it using the current system. As of 2021-12-06
                                // no skeletons resolve to patterns containing this symbol.
                                //
                                // All resolved patterns from cldr-json:
                                // https://github.com/gregtatum/cldr-json/blob/d4779f9611a4cc1e3e6a0a4597e92ead32d9f118/stand-alone-week.js
                                //     'ccc',
                                //     'ccc d. MMM',
                                //     'ccc d. MMMM',
                                //     'cccc d. MMMM y',
                                //     'd, ccc',
                                //     'cccနေ့',
                                //     'ccc, d MMM',
                                //     "ccc, d 'de' MMMM",
                                //     "ccc, d 'de' MMMM 'de' y",
                                //     'ccc, h:mm B',
                                //     'ccc, h:mm:ss B',
                                //     'ccc, d',
                                //     "ccc, dd.MM.y 'г'.",
                                //     'ccc, d.MM.y',
                                //     'ccc, MMM d. y'
                                unimplemented!("Numeric stand-alone fields are not supported.")
                            }
                            FieldLength::Abbreviated => Text::Short,
                            FieldLength::Wide => Text::Long,
                            FieldLength::Narrow | FieldLength::Six | FieldLength::Fixed(_) => {
                                Text::Narrow
                            }
                        },
                        fields::Weekday::Local => unimplemented!("fields::Weekday::Local"),
                    });
                }
                FieldSymbol::DayPeriod(_) => {
                    // Day period does not affect the resolved components.
                }
                FieldSymbol::Hour(hour) => {
                    bag.hour = Some(match field.length {
                        FieldLength::TwoDigit => Numeric::TwoDigit,
                        _ => Numeric::Numeric,
                    });
                    bag.preferences = Some(preferences::Bag {
                        hour_cycle: Some(match hour {
                            fields::Hour::H11 => preferences::HourCycle::H11,
                            fields::Hour::H12 => preferences::HourCycle::H12,
                            fields::Hour::H23 => preferences::HourCycle::H23,
                            fields::Hour::H24 => preferences::HourCycle::H24,
                        }),
                    });
                }
                FieldSymbol::Minute => {
                    bag.minute = Some(match field.length {
                        FieldLength::TwoDigit => Numeric::TwoDigit,
                        _ => Numeric::Numeric,
                    });
                }
                FieldSymbol::Second(second) => {
                    match second {
                        fields::Second::Second => {
                            bag.second = Some(match field.length {
                                FieldLength::TwoDigit => Numeric::TwoDigit,
                                _ => Numeric::Numeric,
                            });
                        }
                        fields::Second::FractionalSecond => {
                            if let FieldLength::Fixed(p) = field.length {
                                if p > 0 {
                                    bag.fractional_second = Some(p);
                                }
                            }
                        }
                        fields::Second::Millisecond => {
                            // fields::Second::Millisecond is not implemented (#1834)
                        }
                    }
                }
                FieldSymbol::TimeZone(time_zone_name) => {
                    bag.time_zone_name = Some(match time_zone_name {
                        fields::TimeZone::LowerZ => match field.length {
                            FieldLength::One => TimeZoneName::ShortSpecific,
                            _ => TimeZoneName::LongSpecific,
                        },
                        fields::TimeZone::LowerV => match field.length {
                            FieldLength::One => TimeZoneName::ShortGeneric,
                            _ => TimeZoneName::LongGeneric,
                        },
                        fields::TimeZone::UpperO => TimeZoneName::GmtOffset,
                        fields::TimeZone::UpperZ => unimplemented!("fields::TimeZone::UpperZ"),
                        fields::TimeZone::UpperV => unimplemented!("fields::TimeZone::UpperV"),
                        fields::TimeZone::LowerX => unimplemented!("fields::TimeZone::LowerX"),
                        fields::TimeZone::UpperX => unimplemented!("fields::TimeZone::UpperX"),
                    });
                }
            }
        }

        bag
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
            day: Some(Day::NumericDayOfMonth),

            hour: Some(Numeric::Numeric),
            minute: Some(Numeric::Numeric),
            second: Some(Numeric::Numeric),
            fractional_second: Some(3),

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
                (
                    Symbol::Second(fields::Second::FractionalSecond),
                    Length::Fixed(3)
                )
                    .into(),
            ]
        );
    }

    #[test]
    fn test_component_bag_to_vec_field2() {
        let bag = Bag {
            year: Some(Year::Numeric),
            month: Some(Month::TwoDigit),
            day: Some(Day::NumericDayOfMonth),
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
