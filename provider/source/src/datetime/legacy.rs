// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO(#5613): Even though these markers are no longer exported, we need them in order to export
//! semantic skeleton data markers. This should be refactored to skip the intermediate data struct.

use crate::cldr_serde;
use crate::cldr_serde::ca;
use crate::datetime::DatagenCalendar;
use crate::SourceDataProvider;
use alloc::borrow::Cow;
use icu::calendar::types::MonthCode;
use icu::datetime::preferences::HourCycle;
use icu::datetime::provider::fields::components;
use icu::datetime::provider::neo::*;
use icu::datetime::provider::pattern::runtime;
use icu::datetime::provider::skeleton::*;
use icu::locale::langid;
use std::collections::BTreeMap;
use tinystr::{tinystr, TinyStr4};
use zerovec::ZeroMap;

/// Data struct for date/time patterns broken down by pattern length.
pub struct LengthPatterns<'data> {
    /// A medium length date/time pattern.
    pub medium: runtime::Pattern<'data>,
    /// A short length date/time pattern.
    pub short: runtime::Pattern<'data>,
}

/// Pattern data for dates.
pub struct DateLengths<'data> {
    /// Date pattern data, broken down by pattern length.
    pub date: LengthPatterns<'data>,

    /// Patterns used to combine date and time length patterns into full date_time patterns.
    pub length_combinations: icu::datetime::provider::skeleton::GenericLengthPatterns<'data>,
}

/// Symbol data for the months, weekdays, and eras needed to format a date.
///
/// For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
pub struct DateSymbols<'data> {
    /// Symbol data for months.
    pub months: months::Contexts<'data>,
    /// Symbol data for weekdays.
    pub weekdays: weekdays::Contexts<'data>,
}

///Formatting symbols for [`Month`](crate::provider::fields::FieldSymbol::Month).
///
///For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
pub mod months {
    use super::*;
    ///Locale data for Month corresponding to the symbols.
    #[expect(clippy::large_enum_variant)]
    pub enum Symbols<'data> {
        /// Twelve symbols for a solar calendar
        ///
        /// This is an optimization to reduce data size.
        SolarTwelve([Cow<'data, str>; 12]),
        /// A calendar with an arbitrary number of months, potentially including leap months
        Other(ZeroMap<'data, MonthCode, str>),
    }

    ///Symbol data for the "format" style formatting of Month.
    ///
    ///The format style is used in contexts where it is different from the stand-alone form, ex: a case inflected form where the stand-alone form is the nominative case.
    pub struct FormatWidths<'data> {
        ///Wide length symbol for "format" style symbol for months.
        pub wide: Symbols<'data>,
    }

    ///Symbol data for the "stand-alone" style formatting of Month.
    ///
    ///The stand-alone style is used in contexts where the field is displayed by itself.
    pub struct StandAloneWidths<'data> {
        ///Abbreviated length symbol for "stand-alone" style symbol for months.
        pub abbreviated: Option<Symbols<'data>>,
        ///Narrow length symbol for "stand-alone" style symbol for months.
        pub narrow: Option<Symbols<'data>>,
        ///Short length symbol for "stand-alone" style symbol for months.
        pub short: Option<Symbols<'data>>,
        ///Wide length symbol for "stand-alone" style symbol for months.
        pub wide: Option<Symbols<'data>>,
    }

    ///The struct containing the symbol data for Month that contains the "format" style symbol data ([`FormatWidths`]) and "stand-alone" style symbol data ([`StandAloneWidths`]).
    pub struct Contexts<'data> {
        /// The symbol data for "format" style symbols.
        pub format: FormatWidths<'data>,
        /// The symbol data for "stand-alone" style symbols.
        pub stand_alone: Option<StandAloneWidths<'data>>,
    }
}

impl months::Symbols<'_> {
    /// Get the symbol for the given month code
    pub fn get(&self, code: MonthCode) -> Option<&str> {
        use tinystr::{tinystr, TinyStr4};
        match *self {
            Self::SolarTwelve(ref arr) => {
                // The tinystr macro doesn't work in match patterns
                // so we use consts first
                const CODE_1: TinyStr4 = tinystr!(4, "M01");
                const CODE_2: TinyStr4 = tinystr!(4, "M02");
                const CODE_3: TinyStr4 = tinystr!(4, "M03");
                const CODE_4: TinyStr4 = tinystr!(4, "M04");
                const CODE_5: TinyStr4 = tinystr!(4, "M05");
                const CODE_6: TinyStr4 = tinystr!(4, "M06");
                const CODE_7: TinyStr4 = tinystr!(4, "M07");
                const CODE_8: TinyStr4 = tinystr!(4, "M08");
                const CODE_9: TinyStr4 = tinystr!(4, "M09");
                const CODE_10: TinyStr4 = tinystr!(4, "M10");
                const CODE_11: TinyStr4 = tinystr!(4, "M11");
                const CODE_12: TinyStr4 = tinystr!(4, "M12");
                let idx = match code.0 {
                    CODE_1 => 0,
                    CODE_2 => 1,
                    CODE_3 => 2,
                    CODE_4 => 3,
                    CODE_5 => 4,
                    CODE_6 => 5,
                    CODE_7 => 6,
                    CODE_8 => 7,
                    CODE_9 => 8,
                    CODE_10 => 9,
                    CODE_11 => 10,
                    CODE_12 => 11,
                    _ => return None,
                };
                arr.get(idx).map(|x| &**x)
            }
            Self::Other(ref map) => map.get(&code),
        }
    }
}

impl Default for months::Symbols<'_> {
    fn default() -> Self {
        Self::Other(Default::default())
    }
}

///Formatting symbols for [`Weekday`](crate::provider::fields::FieldSymbol::Weekday).
///
///For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
pub mod weekdays {
    use super::*;
    ///Locale data for Weekday corresponding to the symbols.
    pub struct Symbols<'data>(pub [Cow<'data, str>; 7]);

    ///Symbol data for the "format" style formatting of Weekday.
    ///
    ///The format style is used in contexts where it is different from the stand-alone form, ex: a case inflected form where the stand-alone form is the nominative case.
    pub struct FormatWidths<'data> {
        ///Short length symbol for "format" style symbol for weekdays, if present.
        pub short: Option<Symbols<'data>>,
    }

    pub struct Contexts<'data> {
        /// The symbol data for "format" style symbols.
        pub format: FormatWidths<'data>,
        /// Whether or not there are "standalone" style symbols
        pub stand_alone: bool,
    }
}

///Formatting symbols for [`DayPeriod`](crate::provider::fields::FieldSymbol::DayPeriod).
///
///For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
pub mod day_periods {
    use super::*;
    ///Locale data for DayPeriod corresponding to the symbols.
    pub struct Symbols<'data> {
        /// Day period for AM (before noon).
        pub am: Cow<'data, str>,
        /// Day period for PM (after noon).
        pub pm: Cow<'data, str>,
        /// Day period for noon, in locales that support it.
        pub noon: Option<Cow<'data, str>>,
        /// Day period for midnight, in locales that support it.
        pub midnight: Option<Cow<'data, str>>,
    }
}

impl From<&cldr_serde::ca::LengthPatterns> for LengthPatterns<'_> {
    fn from(other: &cldr_serde::ca::LengthPatterns) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            medium: other
                .medium
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            short: other
                .short
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
        }
    }
}

impl From<&cldr_serde::ca::DateTimeFormats> for LengthPatterns<'_> {
    fn from(other: &cldr_serde::ca::DateTimeFormats) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            medium: other
                .medium
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            short: other
                .short
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
        }
    }
}

impl From<&cldr_serde::ca::Dates> for DateLengths<'_> {
    fn from(other: &cldr_serde::ca::Dates) -> Self {
        let length_combinations_v1 = GenericLengthPatterns::from(&other.datetime_formats_at_time);

        Self {
            date: (&other.date_skeletons).into(),
            length_combinations: length_combinations_v1,
        }
    }
}

impl<'a> From<&months::Symbols<'a>> for MonthNames<'a> {
    fn from(other: &months::Symbols<'a>) -> Self {
        match other {
            months::Symbols::SolarTwelve(cow_list) => {
                // Can't zero-copy convert a cow list to a VarZeroVec, so we need to allocate
                // a new VarZeroVec. Since VarZeroVec does not implement `from_iter`, first we
                // make a Vec of string references.
                let vec: alloc::vec::Vec<&str> = cow_list.iter().map(|x| &**x).collect();
                MonthNames::Linear((&vec).into())
            }
            months::Symbols::Other(zero_map) => {
                // Only calendar that uses this is hebrew, we can assume it is 12-month
                let mut vec = vec![""; 24];

                #[allow(deprecated)]
                for (k, v) in zero_map.iter() {
                    let Some((number, leap)) = MonthCode(*k).parsed() else {
                        debug_assert!(false, "Found unknown month code {k}");
                        continue;
                    };
                    let offset = if leap { 12 } else { 0 };
                    if let Some(entry) = vec.get_mut((number + offset - 1) as usize) {
                        *entry = v;
                    } else {
                        debug_assert!(false, "Found out of bounds hebrew month code {k}")
                    }
                }
                MonthNames::LeapLinear((&vec).into())
            }
        }
    }
}

impl<'a> From<&weekdays::Symbols<'a>> for LinearNames<'a> {
    fn from(other: &weekdays::Symbols<'a>) -> Self {
        // Input is a cow array of length 7. Need to make it a VarZeroVec.
        let vec: alloc::vec::Vec<&str> = other.0.iter().map(|x| &**x).collect();
        LinearNames {
            names: (&vec).into(),
        }
    }
}

impl<'a> From<&day_periods::Symbols<'a>> for LinearNames<'a> {
    fn from(other: &day_periods::Symbols<'a>) -> Self {
        // Input is a struct with four fields. Need to make it a VarZeroVec.
        let vec: alloc::vec::Vec<&str> = match (other.noon.as_ref(), other.midnight.as_ref()) {
            (Some(noon), Some(midnight)) => vec![&other.am, &other.pm, &noon, &midnight],
            (Some(noon), None) => vec![&other.am, &other.pm, &noon],
            (None, Some(midnight)) => vec![&other.am, &other.pm, "", &midnight],
            (None, None) => vec![&other.am, &other.pm],
        };
        LinearNames {
            names: (&vec).into(),
        }
    }
}

impl cldr_serde::ca::MonthSymbols {
    fn get_unaliased(&self, other: &Self) -> Option<Self> {
        if self == other {
            None
        } else {
            Some(self.clone())
        }
    }
}
impl ca::Contexts<cldr_serde::ca::MonthSymbols> {
    fn get(&self, ctx: &(&'static [TinyStr4], &str)) -> months::Contexts<'static> {
        months::Contexts {
            format: self.format.get(ctx),
            stand_alone: self
                .stand_alone
                .as_ref()
                .and_then(|stand_alone| {
                    let abbreviated = stand_alone
                        .abbreviated
                        .as_ref()
                        .and_then(|v| v.get_unaliased(&self.format.abbreviated));
                    let narrow = stand_alone
                        .narrow
                        .as_ref()
                        .and_then(|v| v.get_unaliased(&self.format.narrow));
                    let short = if stand_alone.short == self.format.short {
                        None
                    } else {
                        stand_alone.short.clone()
                    };
                    let wide = stand_alone
                        .wide
                        .as_ref()
                        .and_then(|v| v.get_unaliased(&self.format.wide));
                    if abbreviated.is_none()
                        && narrow.is_none()
                        && wide.is_none()
                        && short.is_none()
                    {
                        None
                    } else {
                        Some(ca::StandAloneWidths {
                            abbreviated,
                            narrow,
                            short,
                            wide,
                        })
                    }
                })
                .map(|ref stand_alone| months::StandAloneWidths {
                    abbreviated: stand_alone.abbreviated.as_ref().map(|width| width.get(ctx)),
                    narrow: stand_alone.narrow.as_ref().map(|width| width.get(ctx)),
                    short: stand_alone.short.as_ref().map(|width| width.get(ctx)),
                    wide: stand_alone.wide.as_ref().map(|width| width.get(ctx)),
                }),
        }
    }
}
impl ca::StandAloneWidths<cldr_serde::ca::MonthSymbols> {}
impl ca::FormatWidths<cldr_serde::ca::MonthSymbols> {
    fn get(&self, ctx: &(&'static [TinyStr4], &str)) -> months::FormatWidths<'static> {
        months::FormatWidths {
            wide: self.wide.get(ctx),
        }
    }
}

impl cldr_serde::ca::MonthSymbols {
    fn get(&self, ctx: &(&'static [TinyStr4], &str)) -> months::Symbols<'static> {
        if ctx.0.len() == 12 && self.0.len() == 12 {
            let mut arr: [Cow<'static, str>; 12] = Default::default();
            for (k, v) in self.0.iter() {
                let index: usize = k
                    .parse()
                    .expect("CLDR month indices must parse as numbers!");
                if index == 0 {
                    panic!("CLDR month indices cannot be zero");
                }

                arr[index - 1] = Cow::Owned(v.into());
            }

            for (i, val) in arr.iter().enumerate() {
                if val.is_empty() {
                    panic!("Solar calendar does not have data for month {i}");
                }
            }
            months::Symbols::SolarTwelve(arr)
        } else {
            let mut map = BTreeMap::new();
            for (k, v) in self.0.iter() {
                let code = if k == "7-yeartype-leap" && ctx.1 == "hebrew" {
                    tinystr!(4, "M06L")
                } else {
                    let index: usize = k
                        .parse()
                        .expect("CLDR month indices must parse as numbers!");

                    if index == 0 {
                        panic!("CLDR month indices cannot be zero");
                    }
                    *ctx.0
                        .get(index - 1)
                        .expect("Found out of bounds month index for calendar")
                };

                map.insert(MonthCode(code), v.as_ref());
            }
            months::Symbols::Other(map.into_iter().collect())
        }
    }
}

impl cldr_serde::ca::DaySymbols {
    fn get(&self, _ctx: &()) -> weekdays::Symbols<'static> {
        weekdays::Symbols([
            Cow::Owned(self.sun.clone()),
            Cow::Owned(self.mon.clone()),
            Cow::Owned(self.tue.clone()),
            Cow::Owned(self.wed.clone()),
            Cow::Owned(self.thu.clone()),
            Cow::Owned(self.fri.clone()),
            Cow::Owned(self.sat.clone()),
        ])
    }
}
impl cldr_serde::ca::DaySymbols {
    fn get_unaliased(&self, other: &Self) -> Option<Self> {
        if self == other {
            None
        } else {
            Some(self.clone())
        }
    }
}
impl ca::Contexts<cldr_serde::ca::DaySymbols> {
    fn get(&self, ctx: &()) -> weekdays::Contexts<'static> {
        weekdays::Contexts {
            format: weekdays::FormatWidths {
                short: self.format.short.as_ref().map(|width| width.get(ctx)),
            },
            stand_alone: self
                .stand_alone
                .as_ref()
                .map(|stand_alone| {
                    let abbreviated = stand_alone
                        .abbreviated
                        .as_ref()
                        .and_then(|v| v.get_unaliased(&self.format.abbreviated));
                    let narrow = stand_alone
                        .narrow
                        .as_ref()
                        .and_then(|v| v.get_unaliased(&self.format.narrow));
                    let short = if stand_alone.short == self.format.short {
                        None
                    } else {
                        stand_alone.short.clone()
                    };
                    let wide = stand_alone
                        .wide
                        .as_ref()
                        .and_then(|v| v.get_unaliased(&self.format.wide));
                    abbreviated.is_some() || narrow.is_some() || wide.is_some() || short.is_some()
                })
                .unwrap_or(false),
        }
    }
}

fn convert_dates(other: &cldr_serde::ca::Dates, calendar: DatagenCalendar) -> DateSymbols<'static> {
    DateSymbols {
        months: other.months.get(&(
            {
                // This will need to be more complicated to handle lunar calendars
                // https://github.com/unicode-org/icu4x/issues/2066
                static SOLAR_MONTH_CODES: &[TinyStr4] = &[
                    tinystr!(4, "M01"),
                    tinystr!(4, "M02"),
                    tinystr!(4, "M03"),
                    tinystr!(4, "M04"),
                    tinystr!(4, "M05"),
                    tinystr!(4, "M06"),
                    tinystr!(4, "M07"),
                    tinystr!(4, "M08"),
                    tinystr!(4, "M09"),
                    tinystr!(4, "M10"),
                    tinystr!(4, "M11"),
                    tinystr!(4, "M12"),
                    tinystr!(4, "M13"),
                ];
                // CLDR labels the regular months and M05L by their ordinals
                // whereas M06L is stored as 7-yeartype-leap
                static HEBREW_MONTH_CODES: &[TinyStr4] = &[
                    tinystr!(4, "M01"),
                    tinystr!(4, "M02"),
                    tinystr!(4, "M03"),
                    tinystr!(4, "M04"),
                    tinystr!(4, "M05"),
                    tinystr!(4, "M05L"),
                    tinystr!(4, "M06"),
                    tinystr!(4, "M07"),
                    tinystr!(4, "M08"),
                    tinystr!(4, "M09"),
                    tinystr!(4, "M10"),
                    tinystr!(4, "M11"),
                    tinystr!(4, "M12"),
                    // M06L is handled separately in MonthSymbols code
                ];
                match calendar {
                    DatagenCalendar::Buddhist
                    | DatagenCalendar::Chinese
                    | DatagenCalendar::Dangi
                    | DatagenCalendar::Gregorian
                    | DatagenCalendar::Indian
                    | DatagenCalendar::Hijri
                    | DatagenCalendar::JapaneseExtended
                    | DatagenCalendar::JapaneseModern
                    | DatagenCalendar::Persian
                    | DatagenCalendar::Roc => &SOLAR_MONTH_CODES[0..12],
                    DatagenCalendar::Coptic | DatagenCalendar::Ethiopic => SOLAR_MONTH_CODES,
                    DatagenCalendar::Hebrew => HEBREW_MONTH_CODES,
                }
            },
            calendar.cldr_name(),
        )),
        weekdays: other.days.get(&()),
    }
}

#[test]
fn test_basic_symbols() {
    use icu::calendar::types::MonthCode;
    use tinystr::tinystr;

    let provider = SourceDataProvider::new_testing();

    let data = provider
        .get_dates_resource(&langid!("cs").into(), Some(DatagenCalendar::Gregorian))
        .unwrap();

    let cs_dates = convert_dates(data, DatagenCalendar::Gregorian);

    assert_eq!(
        "srpna",
        cs_dates
            .months
            .format
            .wide
            .get(MonthCode(tinystr!(4, "M08")))
            .unwrap()
    );

    assert_eq!("po", cs_dates.weekdays.format.short.as_ref().unwrap().0[1]);
}

#[test]
fn unalias_contexts() {
    let provider = SourceDataProvider::new_testing();

    let data = provider
        .get_dates_resource(&langid!("cs").into(), Some(DatagenCalendar::Gregorian))
        .unwrap();

    let cs_dates = convert_dates(data, DatagenCalendar::Gregorian);

    // Czech months are not unaliased because `wide` differs.
    assert!(cs_dates.months.stand_alone.is_some());

    // Czech months are not unaliased because `wide` differs.
    assert!(cs_dates
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .abbreviated
        .is_none());
    assert!(cs_dates
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .short
        .is_none());
    assert!(cs_dates
        .months
        .stand_alone
        .as_ref()
        .unwrap()
        .narrow
        .is_none());
    assert!(cs_dates.months.stand_alone.as_ref().unwrap().wide.is_some());

    // Czech weekdays are unaliased because they completely overlap.
    assert!(!cs_dates.weekdays.stand_alone);
}

// TODO(#586) - Append items support needs to be added.
#[test]
#[should_panic]
fn test_missing_append_items_support() {
    let mut components = components::Bag::empty();
    components.year = Some(components::Year::Numeric);
    components.month = Some(components::Month::Long);
    components.day = Some(components::Day::NumericDayOfMonth);
    // This will be appended.
    components.time_zone_name = Some(components::TimeZoneName::LongSpecific);
    let requested_fields = components.to_vec_fields(HourCycle::H23);

    let provider = SourceDataProvider::new_testing();
    let data = provider
        .get_dates_resource(&langid!("en").into(), Some(DatagenCalendar::Gregorian))
        .unwrap();
    let patterns = DateLengths::from(data);
    let skeletons = DateSkeletonPatterns::from(&data.datetime_formats.available_formats);

    match create_best_pattern_for_fields(
        &skeletons,
        &patterns.length_combinations,
        &requested_fields,
        &Default::default(),
        false,
    ) {
        BestSkeleton::AllFieldsMatch(available_format_pattern, _) => {
            // TODO - Append items are needed here.
            assert_eq!(
                available_format_pattern
                    .expect_pattern("pattern should not have plural variants")
                    .to_string()
                    .as_str(),
                "MMMM d, y vvvv"
            )
        }
        best => panic!("Unexpected {best:?}"),
    };
}

#[test]
fn test_basic_patterns() {
    let provider = SourceDataProvider::new_testing();
    let data = provider
        .get_dates_resource(&langid!("cs").into(), Some(DatagenCalendar::Gregorian))
        .unwrap();

    let cs_dates = DateLengths::from(data);

    assert_eq!("yMd", cs_dates.date.medium.to_string());
}

#[test]
fn test_with_numbering_system() {
    let provider = SourceDataProvider::new_testing();
    let data = provider
        .get_dates_resource(&langid!("haw").into(), Some(DatagenCalendar::Gregorian))
        .unwrap();

    let haw_dates = DateLengths::from(data);

    assert_eq!("yMMMd", haw_dates.date.medium.to_string());
    // TODO(#308): Support numbering system variations. We currently throw them away.
    assert_eq!("yyMd", haw_dates.date.short.to_string());
}
