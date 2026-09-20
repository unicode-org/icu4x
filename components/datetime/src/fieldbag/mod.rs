// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Field-specific formatting options for better interop with ECMA-402 and ICU4C.

pub mod field;

use crate::{
    DateTimeFormatterPreferences,
    fieldsets::builder::{DateFields, FieldSetBuilder, ZoneStyle},
    options::{Alignment, Length, TimePrecision, YearStyle},
};
use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;

/// An unordered bag of datetime fields in a pattern or pattern skeleton.
///
/// See the [module-level docs](crate::fieldbag) for more information.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub struct DateTimeFieldBag {
    /// The length of the era field.
    pub era: Option<field::Era>,
    /// The length of the year field.
    pub year: Option<field::Year>,
    /// The length of the month field.
    pub month: Option<field::Month>,
    /// The length of the day-of-month field.
    pub day: Option<field::Day>,
    /// The length of the day-of-week field.
    pub weekday: Option<field::Weekday>,
    /// The length of the day-period field.
    pub day_period: Option<field::DayPeriod>,
    /// The type of hour field.
    pub hour_type: Option<field::HourType>,
    /// The length of the hour field.
    pub hour: Option<field::Hour>,
    /// The length of the minute field.
    pub minute: Option<field::Minute>,
    /// The length of the second field.
    pub second: Option<field::Second>,
    /// The length of the fractional second field.
    pub subsecond: Option<field::Subsecond>,
    /// The length and style of the time zone field.
    pub time_zone_name: Option<field::TimeZoneName>,
}

impl DateTimeFieldBag {
    /// Converts this [`DateTimeFieldBag`] into a [`FieldSetBuilder`].
    pub fn to_field_set_builder(&self) -> FieldSetBuilder {
        FieldSetBuilder::from_field_bag(self)
    }
}

/// Preferences associated with datetime fields, such as hour cycle.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub struct DateTimeFieldPreferences {
    /// The preferred hour cycle.
    pub hour_cycle: Option<HourCycle>,
}

/// A combination of a [`DateTimeFieldBag`] and associated [`DateTimeFieldPreferences`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
#[allow(clippy::exhaustive_structs)]
pub struct DateTimeFieldBagWithPreferences {
    /// The field options bag.
    pub bag: DateTimeFieldBag,
    /// The preferences associated with the field options.
    pub preferences: DateTimeFieldPreferences,
}

/// An error that occurred while parsing a UTS 35 skeleton string into a [`DateTimeFieldBagWithPreferences`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, displaydoc::Display)]
#[non_exhaustive]
pub enum DateTimeFieldBagParseError {
    /// Invalid or unsupported skeleton symbol: '{0}'.
    #[displaydoc("Invalid skeleton symbol: '{0}'")]
    InvalidSymbol(char),
    /// Invalid length {1} for symbol '{0}'.
    #[displaydoc("Invalid length {1} for symbol '{0}'")]
    InvalidLength(char, usize),
    /// Duplicate symbol in skeleton: '{0}'.
    #[displaydoc("Duplicate symbol in skeleton: '{0}'")]
    DuplicateSymbol(char),
}

impl core::error::Error for DateTimeFieldBagParseError {}

impl DateTimeFieldBag {
    fn put_field(&mut self, ch: char, count: usize) -> Result<(), ()> {
        enum Resolution {
            Ok,
            DuplicateSymbol,
            InvalidLength,
        }
        let resolution = match ch {
            'G' => 'block: {
                if self.era.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.era = Some(match count {
                    1..=3 => field::Era::Short,
                    4 => field::Era::Long,
                    5 => field::Era::Narrow,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'y' => 'block: {
                if self.year.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.year = Some(match count {
                    1 => field::Year::Numeric,
                    2 => field::Year::TwoDigit,
                    // TODO: Add 3-digit, 4-digit, ... ?
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'U' => 'block: {
                if self.year.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.year = Some(match count {
                    1..=3 => field::Year::CyclicAbbreviated,
                    4 => field::Year::CyclicLong,
                    5 => field::Year::CyclicNarrow,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'M' => 'block: {
                if self.month.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.month = Some(match count {
                    1 => field::Month::Numeric,
                    2 => field::Month::TwoDigit,
                    3 => field::Month::Short,
                    4 => field::Month::Long,
                    5 => field::Month::Narrow,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'd' => 'block: {
                if self.day.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.day = Some(match count {
                    1 => field::Day::Numeric,
                    2 => field::Day::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'E' => 'block: {
                if self.weekday.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.weekday = Some(match count {
                    1..=3 => field::Weekday::Abbreviated,
                    4 => field::Weekday::Long,
                    5 => field::Weekday::Narrow,
                    6 => field::Weekday::Short,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'j' => 'block: {
                if self.hour.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                if self.hour_type.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.hour = Some(match count {
                    1 | 3 | 5 => field::Hour::Numeric,
                    2 | 4 | 6 => field::Hour::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                self.hour_type = Some(field::HourType::AutoNumeric);
                Resolution::Ok
            }
            'J' => 'block: {
                if self.hour.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                if self.hour_type.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.hour = Some(match count {
                    1 => field::Hour::Numeric,
                    2 => field::Hour::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                self.hour_type = Some(field::HourType::AutoNumericNoDayPeriod);
                Resolution::Ok
            }
            'C' => 'block: {
                if self.hour.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                if self.hour_type.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.hour = Some(match count {
                    1 | 3 | 5 => field::Hour::Numeric,
                    2 | 4 | 6 => field::Hour::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                self.hour_type = Some(field::HourType::Auto);
                Resolution::Ok
            }
            'h' => 'block: {
                if self.hour.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.hour = Some(match count {
                    1 => field::Hour::Numeric,
                    2 => field::Hour::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                preferences.hour_cycle = Some(HourCycle::H12);
            }
            'K' => 'block: {
                if self.hour.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.hour = Some(match count {
                    1 => field::Hour::Numeric,
                    2 => field::Hour::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                preferences.hour_cycle = Some(HourCycle::H11);
            }
            'H' => 'block: {
                if self.hour.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.hour = Some(match count {
                    1 => field::Hour::Numeric,
                    2 => field::Hour::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                preferences.hour_cycle = Some(HourCycle::H23);
            }
            'k' => 'block: {
                if self.hour.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.hour = Some(match count {
                    1 => field::Hour::Numeric,
                    2 => field::Hour::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                // Note: ICU4X maps 'k' (1-24) to H23 because H24 is not supported.
                preferences.hour_cycle = Some(HourCycle::H23);
            }
            'a' | 'b' | 'B' => 'block: {
                if self.day_period.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.day_period = Some(match count {
                    1..=3 => field::DayPeriod::Short,
                    4 => field::DayPeriod::Long,
                    5 => field::DayPeriod::Narrow,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'm' => 'block: {
                if self.minute.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.minute = Some(match count {
                    1 => field::Minute::Numeric,
                    2 => field::Minute::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            's' => 'block: {
                if self.second.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.second = Some(match count {
                    1 => field::Second::Numeric,
                    2 => field::Second::TwoDigit,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'S' => 'block: {
                if self.subsecond.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                let Some(subsecond) = field::Subsecond::try_from_int(count as u8) else {
                    break 'block Resolution::InvalidLength,
                };
                self.subsecond = Some(subsecond);
            }
            'z' => 'block: {
                if self.time_zone_name.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.time_zone_name = Some(match count {
                    1..=3 => field::TimeZoneName::ShortSpecific,
                    4 => field::TimeZoneName::LongSpecific,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'O' => 'block: {
                if self.time_zone_name.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.time_zone_name = Some(match count {
                    1 => field::TimeZoneName::ShortOffset,
                    4 => field::TimeZoneName::LongOffset,
                    _ => break 'block Resolution::InvalidLength,
                });
                Resolution::Ok
            }
            'v' => 'block: {
                if self.time_zone_name.is_some() {
                    break 'block Resolution::DuplicateSymbol;
                }
                self.time_zone_name = Some(match count {
                    1 => field::TimeZoneName::ShortGeneric,
                    4 => field::TimeZoneName::LongGeneric,
                    _ => break 'block Resolution::InvalidLength,
                });
            }
            _ => break 'block Resolution::InvalidSymbol;
        }
    }
}

impl DateTimeFieldBagWithPreferences {
    /// Parses a UTS 35 skeleton string into a [`DateTimeFieldBagWithPreferences`].
    pub fn try_from_skeleton(
        skeleton: &str,
    ) -> Result<DateTimeFieldBagWithPreferences, DateTimeFieldBagParseError> {
        let mut bag = DateTimeFieldBag::default();
        let mut preferences = DateTimeFieldPreferences::default();

        let mut chars = skeleton.chars().peekable();
        while let Some(ch) = chars.next() {
            let mut count = 1;
            while chars.peek() == Some(&ch) {
                count += 1;
                chars.next();
            }

        }

        Ok(DateTimeFieldBagWithPreferences { bag, preferences })
    }
}

impl core::str::FromStr for DateTimeFieldBagWithPreferences {
    type Err = DateTimeFieldBagParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_skeleton(s)
    }
}

impl FieldSetBuilder {
    /// Creates a [`FieldSetBuilder`] configured with the options in [`DateTimeFieldBag`].
    pub fn from_field_bag(bag: &DateTimeFieldBag) -> Self {
        let mut builder = FieldSetBuilder::new();

        let has_y = bag.year.is_some() || bag.era.is_some();
        let has_m = bag.month.is_some();
        let has_d = bag.day.is_some();
        let has_e = bag.weekday.is_some();

        builder.date_fields = match (has_y, has_m, has_d, has_e) {
            // Exact matches
            (true, true, true, true) => Some(DateFields::YMDE),
            (true, true, true, false) => Some(DateFields::YMD),
            (false, true, true, true) => Some(DateFields::MDE),
            (false, true, true, false) => Some(DateFields::MD),
            (false, false, true, true) => Some(DateFields::DE),
            (false, false, true, false) => Some(DateFields::D),
            (false, false, false, true) => Some(DateFields::E),
            (true, true, false, false) => Some(DateFields::YM),
            (false, true, false, false) => Some(DateFields::M),
            (true, false, false, false) => Some(DateFields::Y),
            (false, false, false, false) => None,

            // Fields that are filled in
            (true, true, false, true) => Some(DateFields::YMDE),
            (false, true, false, true) => Some(DateFields::MDE),
            (true, false, true, false) => Some(DateFields::YMD),
            (true, false, true, true) => Some(DateFields::YMDE),
            (true, false, false, true) => Some(DateFields::YMDE),
        };

        if bag.era.is_some() {
            builder.year_style = Some(YearStyle::WithEra);
        } else if let Some(year) = bag.year {
            builder.year_style = match year {
                field::Year::Numeric => Some(YearStyle::Full),
                field::Year::TwoDigit => Some(YearStyle::Auto),
                _ => None,
            };
        }

        let is_long = matches!(bag.year, Some(field::Year::CyclicLong))
            || matches!(bag.month, Some(field::Month::Long))
            || matches!(bag.weekday, Some(field::Weekday::Long))
            || matches!(bag.era, Some(field::Era::Long))
            || matches!(bag.day_period, Some(field::DayPeriod::Long));
        let is_short_name = matches!(
            bag.year,
            Some(field::Year::CyclicAbbreviated | field::Year::CyclicNarrow)
        ) || matches!(
            bag.month,
            Some(field::Month::Short | field::Month::Narrow)
        ) || matches!(
            bag.weekday,
            Some(field::Weekday::Abbreviated | field::Weekday::Short | field::Weekday::Narrow)
        ) || matches!(bag.era, Some(field::Era::Short | field::Era::Narrow))
            || matches!(
                bag.day_period,
                Some(field::DayPeriod::Short | field::DayPeriod::Narrow)
            );
        let is_numeric_month = matches!(
            bag.month,
            Some(field::Month::Numeric | field::Month::TwoDigit)
        );

        if is_long {
            builder.length = Some(Length::Long);
        } else if is_short_name {
            builder.length = Some(Length::Medium);
        } else if is_numeric_month {
            builder.length = Some(Length::Short);
        }

        let has_two_digit = matches!(bag.year, Some(field::Year::TwoDigit))
            || matches!(bag.month, Some(field::Month::TwoDigit))
            || matches!(bag.day, Some(field::Day::TwoDigit))
            || matches!(bag.hour, Some(field::Hour::TwoDigit))
            || matches!(bag.minute, Some(field::Minute::TwoDigit))
            || matches!(bag.second, Some(field::Second::TwoDigit));
        if has_two_digit {
            builder.alignment = Some(Alignment::Column);
        }

        if let Some(subsecond) = bag.subsecond {
            builder.time_precision = Some(TimePrecision::Subsecond(subsecond));
        } else if bag.second.is_some() {
            builder.time_precision = Some(TimePrecision::Second);
        } else if bag.minute.is_some() {
            builder.time_precision = Some(TimePrecision::Minute);
        } else if bag.hour.is_some() || bag.day_period.is_some() {
            builder.time_precision = Some(TimePrecision::Hour);
        }

        if let Some(zone) = bag.time_zone_name {
            builder.zone_style = Some(match zone {
                field::TimeZoneName::ShortSpecific => ZoneStyle::SpecificShort,
                field::TimeZoneName::LongSpecific => ZoneStyle::SpecificLong,
                field::TimeZoneName::ShortOffset => ZoneStyle::LocalizedOffsetShort,
                field::TimeZoneName::LongOffset => ZoneStyle::LocalizedOffsetLong,
                field::TimeZoneName::ShortGeneric => ZoneStyle::GenericShort,
                field::TimeZoneName::LongGeneric => ZoneStyle::GenericLong,
            });
        }

        builder
    }
}

impl From<&DateTimeFieldBag> for FieldSetBuilder {
    fn from(bag: &DateTimeFieldBag) -> Self {
        Self::from_field_bag(bag)
    }
}

impl From<DateTimeFieldBag> for FieldSetBuilder {
    fn from(bag: DateTimeFieldBag) -> Self {
        Self::from_field_bag(&bag)
    }
}

impl DateTimeFormatterPreferences {
    /// Merges preferences from [`DateTimeFieldPreferences`] into `self`.
    pub fn merge_field_preferences(mut self, field_preferences: DateTimeFieldPreferences) -> Self {
        if let Some(hour_cycle) = field_preferences.hour_cycle {
            self.hour_cycle = Some(hour_cycle);
        }
        self
    }
}

impl From<DateTimeFieldPreferences> for DateTimeFormatterPreferences {
    fn from(field_preferences: DateTimeFieldPreferences) -> Self {
        Self::default().merge_field_preferences(field_preferences)
    }
}
