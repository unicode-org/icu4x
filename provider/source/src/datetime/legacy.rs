// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO(#5613): Even though these markers are no longer exported, we need them in order to export
//! semantic skeleton data markers. This should be refactored to skip the intermediate data struct.

use alloc::borrow::Cow;
use icu::datetime::provider::pattern::runtime;
use icu::datetime::provider::skeleton::*;
use icu::{calendar::types::MonthCode, datetime::provider::pattern::CoarseHourCycle};
use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use tinystr::{tinystr, TinyStr4};
use zerovec::ZeroMap;

/// Data struct for date/time patterns broken down by pattern length.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct LengthPatterns<'data> {
    /// A full length date/time pattern.
    pub full: runtime::Pattern<'data>,
    /// A long length date/time pattern.
    pub long: runtime::Pattern<'data>,
    /// A medium length date/time pattern.
    pub medium: runtime::Pattern<'data>,
    /// A short length date/time pattern.
    pub short: runtime::Pattern<'data>,
}

/// Pattern data for dates.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, zerofrom::ZeroFrom, yoke::Yokeable)]
pub struct DateLengths<'data> {
    /// Date pattern data, broken down by pattern length.
    pub date: LengthPatterns<'data>,

    /// Patterns used to combine date and time length patterns into full date_time patterns.
    pub length_combinations: GenericLengthPatterns<'data>,
}

/// Pattern data for times.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct TimeLengths<'data> {
    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h11 or h12.
    pub time_h11_h12: LengthPatterns<'data>,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h23 or h24.
    pub time_h23_h24: LengthPatterns<'data>,

    /// By default a locale will prefer one hour cycle type over another.
    pub preferred_hour_cycle: CoarseHourCycle,
}

/// Symbol data for the months, weekdays, and eras needed to format a date.
///
/// For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct DateSymbols<'data> {
    /// Symbol data for months.
    pub months: months::Contexts<'data>,
    /// Symbol data for weekdays.
    pub weekdays: weekdays::Contexts<'data>,
    /// Symbol data for eras.
    pub eras: Eras<'data>,
}

/// Symbol data for the day periods needed to format a time.
///
/// For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct TimeSymbols<'data> {
    /// Symbol data for day periods.
    pub day_periods: day_periods::Contexts<'data>,
}

/// String data for the name, abbreviation, and narrow form of a date's era.
///
/// Keys of the map represent era codes, and the values are the display names.
///
/// Era codes are derived from CLDR data, and are calendar specific.
/// Some examples include: `"be"`, `"0"` / `"1"`, `"bce"` / `"ce"`,
/// `"heisei"` / `"meiji"` / `"reiwa"` / ...  Not all era codes are inherited as-is,
/// such as for the extended Japanese calendar.
///
/// For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct Eras<'data> {
    /// Symbol data for era names.
    ///
    /// Keys are era codes, and values are display names. See [`Eras`].
    pub names: ZeroMap<'data, PotentialUtf8, str>,
    /// Symbol data for era abbreviations.
    ///
    /// Keys are era codes, and values are display names. See [`Eras`].
    pub abbr: ZeroMap<'data, PotentialUtf8, str>,
    /// Symbol data for era narrow forms.
    ///
    /// Keys are era codes, and values are display names. See [`Eras`].
    pub narrow: ZeroMap<'data, PotentialUtf8, str>,
}

// Note: the SymbolsV* struct doc strings metadata are attached to `$name` in the macro invocation to
// avoid macro parsing ambiguity caused by other metadata already attached to `$symbols`.
macro_rules! symbols {
    ($(#[$symbols_attr:meta])*  $name: ident, $field_id: ident, $symbols: item) => {

        $(#[$symbols_attr])*
        #[doc = concat!("Formatting symbols for [`",
                stringify!($field_id),
                "`](crate::provider::fields::FieldSymbol::",
                stringify!($field_id),
                ").\n\n",
                "For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).")]
        pub mod $name {
            use super::*;

            #[derive(Debug, PartialEq, Clone, zerofrom::ZeroFrom, yoke::Yokeable)]
            #[yoke(prove_covariance_manually)]
            #[doc = concat!("Locale data for ", stringify!($field_id), " corresponding to the symbols.")]
            ///
            /// <div class="stab unstable">
            /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
            /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
            /// to be stable, their Rust representation might not be. Use with caution.
            /// </div>
            $symbols

            // UTS 35 specifies that `format` widths are mandatory,
            // except for `short`.
            #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
            #[yoke(prove_covariance_manually)]
            #[doc = concat!("Symbol data for the \"format\" style formatting of ", stringify!($field_id),
                ".\n\nThe format style is used in contexts where it is different from the stand-alone form, ex: ",
                "a case inflected form where the stand-alone form is the nominative case.")]
            ///
            /// <div class="stab unstable">
            /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
            /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
            /// to be stable, their Rust representation might not be. Use with caution.
            /// </div>
            pub struct FormatWidths<'data> {
                #[doc = concat!("Abbreviated length symbol for \"format\" style symbol for ", stringify!($name), ".")]
                pub abbreviated: Symbols<'data>,
                #[doc = concat!("Narrow length symbol for \"format\" style symbol for ", stringify!($name), ".")]
                pub narrow: Symbols<'data>,
                #[doc = concat!("Short length symbol for \"format\" style symbol for ", stringify!($name), ", if present.")]
                pub short: Option<Symbols<'data>>,
                #[doc = concat!("Wide length symbol for \"format\" style symbol for ", stringify!($name), ".")]
                pub wide: Symbols<'data>,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
            #[yoke(prove_covariance_manually)]
            #[doc = concat!("Symbol data for the \"stand-alone\" style formatting of ", stringify!($field_id),
                ".\n\nThe stand-alone style is used in contexts where the field is displayed by itself.")]
            ///
            /// <div class="stab unstable">
            /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
            /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
            /// to be stable, their Rust representation might not be. Use with caution.
            /// </div>
            pub struct StandAloneWidths<'data> {
                #[doc = concat!("Abbreviated length symbol for \"stand-alone\" style symbol for ", stringify!($name), ".")]
                pub abbreviated: Option<Symbols<'data>>,
                #[doc = concat!("Narrow length symbol for \"stand-alone\" style symbol for ", stringify!($name), ".")]
                pub narrow: Option<Symbols<'data>>,
                #[doc = concat!("Short length symbol for \"stand-alone\" style symbol for ", stringify!($name), ".")]
                pub short: Option<Symbols<'data>>,
                #[doc = concat!("Wide length symbol for \"stand-alone\" style symbol for ", stringify!($name), ".")]
                pub wide: Option<Symbols<'data>>,
            }

            #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
            #[yoke(prove_covariance_manually)]
            #[doc = concat!("The struct containing the symbol data for ", stringify!($field_id),
                " that contains the \"format\" style symbol data ([`FormatWidths`]) and \"stand-alone\" style symbol data ([`StandAloneWidths`]).")]
            ///
            /// <div class="stab unstable">
            /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
            /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
            /// to be stable, their Rust representation might not be. Use with caution.
            /// </div>
            pub struct Contexts<'data> {
                /// The symbol data for "format" style symbols.
                pub format: FormatWidths<'data>,
                /// The symbol data for "stand-alone" style symbols.
                pub stand_alone: Option<StandAloneWidths<'data>>,
            }
        }
    };
}

symbols!(
    months,
    Month,
    #[expect(clippy::large_enum_variant)]
    pub enum Symbols<'data> {
        /// Twelve symbols for a solar calendar
        ///
        /// This is an optimization to reduce data size.
        SolarTwelve([Cow<'data, str>; 12]),
        /// A calendar with an arbitrary number of months, potentially including leap months
        Other(ZeroMap<'data, MonthCode, str>),
    }
);

impl months::Symbols<'_> {
    /// Get the symbol for the given month code
    pub fn get(&self, code: MonthCode) -> Option<&str> {
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

symbols!(
    weekdays,
    Weekday,
    #[derive(Default)]
    pub struct Symbols<'data>(pub [Cow<'data, str>; 7]);
);

symbols!(
    day_periods,
    DayPeriod,
    #[derive(Default)]
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
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::cldr_serde::{ca, eras::EraData};
    use crate::datetime::DatagenCalendar;
    use crate::{cldr_serde, SourceDataProvider};
    use icu::calendar::types::MonthCode;
    use icu::locale::langid;
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use tinystr::{tinystr, TinyStr4};
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
                abbreviated: self.abbreviated.get(ctx),
                narrow: self.narrow.get(ctx),
                short: self.short.as_ref().map(|width| width.get(ctx)),
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
                    abbreviated: self.format.abbreviated.get(ctx),
                    narrow: self.format.narrow.get(ctx),
                    short: self.format.short.as_ref().map(|width| width.get(ctx)),
                    wide: self.format.wide.get(ctx),
                },
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
                    .map(|ref stand_alone| weekdays::StandAloneWidths {
                        abbreviated: stand_alone.abbreviated.as_ref().map(|width| width.get(ctx)),
                        narrow: stand_alone.narrow.as_ref().map(|width| width.get(ctx)),
                        short: stand_alone.short.as_ref().map(|width| width.get(ctx)),
                        wide: stand_alone.wide.as_ref().map(|width| width.get(ctx)),
                    }),
            }
        }
    }

    fn convert_dates(
        other: &cldr_serde::ca::Dates,
        calendar: DatagenCalendar,
        all_eras: &[(usize, EraData)],
    ) -> DateSymbols<'static> {
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
            eras: other
                .eras
                .as_ref()
                .map(|in_eras| {
                    let mut out_eras = Eras::default();

                    for (index, era) in all_eras {
                        if let Some(name) = in_eras.names.get(&index.to_string()) {
                            out_eras
                                .names
                                .insert(era.code.as_deref().unwrap().into(), name);
                        }
                        if let Some(abbr) = in_eras.abbr.get(&index.to_string()) {
                            out_eras
                                .abbr
                                .insert(era.code.as_deref().unwrap().into(), abbr);
                        }
                        if let Some(narrow) = in_eras.narrow.get(&index.to_string()) {
                            out_eras
                                .narrow
                                .insert(era.code.as_deref().unwrap().into(), narrow);
                        }
                    }
                    out_eras
                })
                .unwrap_or_default(),
        }
    }
    #[test]
    fn test_basic_symbols() {
        use icu::calendar::types::MonthCode;
        use tinystr::tinystr;

        let provider = SourceDataProvider::new_testing();

        let data = provider
            .get_datetime_resources(&langid!("cs").into(), Some(DatagenCalendar::Gregorian))
            .unwrap();

        let all_eras = &provider.all_eras().unwrap()[&DatagenCalendar::Gregorian];

        let cs_dates = convert_dates(&data, DatagenCalendar::Gregorian, all_eras);

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
            .get_datetime_resources(&langid!("cs").into(), Some(DatagenCalendar::Gregorian))
            .unwrap();

        let all_eras = &provider.all_eras().unwrap()[&DatagenCalendar::Gregorian];

        let cs_dates = convert_dates(&data, DatagenCalendar::Gregorian, all_eras);

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
        assert!(cs_dates.weekdays.stand_alone.is_none());
    }
}
