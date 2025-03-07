// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::calendar::japanese::JAPANEXT_FILE;
use crate::cldr_serde::{self, ca};
use icu::calendar::provider::JapaneseEras;
use icu::calendar::types::MonthCode;
use icu::datetime::provider::calendar::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use tinystr::{tinystr, TinyStr16, TinyStr4};

pub(crate) fn convert_dates(other: &cldr_serde::ca::Dates, calendar: &str) -> DateSymbols<'static> {
    let eras = if let Some(ref eras) = other.eras {
        convert_eras(eras, calendar)
    } else {
        Default::default()
    };
    DateSymbols {
        months: other.months.get(&(get_month_code_map(calendar), calendar)),
        weekdays: other.days.get(&()),
        eras,
    }
}

pub(crate) fn convert_times(other: &cldr_serde::ca::Dates) -> TimeSymbols<'static> {
    TimeSymbols {
        day_periods: other.day_periods.get(&()),
    }
}

fn convert_eras(eras: &cldr_serde::ca::Eras, calendar: &str) -> Eras<'static> {
    let mut out_eras = Eras::default();

    for &(cldr, code) in &get_era_code_map()[calendar] {
        if let Some(name) = eras.names.get(&cldr.to_string()) {
            out_eras.names.insert(code.as_str().into(), name);
        }
        if let Some(abbr) = eras.abbr.get(&cldr.to_string()) {
            out_eras.abbr.insert(code.as_str().into(), abbr);
        }
        if let Some(narrow) = eras.narrow.get(&cldr.to_string()) {
            out_eras.narrow.insert(code.as_str().into(), narrow);
        }
    }
    out_eras
}
/// Returns a month code map and whether the map has leap months
pub(crate) fn get_month_code_map(calendar: &str) -> &'static [TinyStr4] {
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
        "gregory" | "buddhist" | "japanese" | "japanext" | "indian" | "persian" | "roc"
        | "islamic" | "islamic-civil" | "islamic-umalqura" | "islamic-tbla" | "chinese"
        | "dangi" => &SOLAR_MONTH_CODES[0..12],
        "coptic" | "ethiopic" => SOLAR_MONTH_CODES,
        "hebrew" => HEBREW_MONTH_CODES,
        _ => panic!("Month map unknown for {calendar}"),
    }
}

/// Produces a map from the CLDR era index, to the era code and the ICU4X era index.
///
/// Eras are returned in chronological order; which is what ICU4X uses for indexing eras.
/// Therefore the first era returned by this function is FormattableEra::Index(0), etc, for
/// calendars which use FormattableEra.
///
/// See FormattableEra for a definition of what chronological order is in this context.
///
/// In 2.0 the era codes are only used for formatting Japanese, and this code can be simplified
// TODO: This should be replaced by loading calendarData.json
pub(crate) fn get_era_code_map() -> &'static BTreeMap<&'static str, Vec<(usize, TinyStr16)>> {
    static MAP: OnceLock<BTreeMap<&'static str, Vec<(usize, TinyStr16)>>> = OnceLock::new();

    MAP.get_or_init(|| {
        let japanese = &serde_json::from_str::<JapaneseEras>(JAPANEXT_FILE)
            .expect("Failed to parse the precached golden. This is a bug.")
            .dates_to_eras;

        let gregory = vec![
            (0, tinystr!(16, "gregory-inverse")),
            (1, tinystr!(16, "gregory")),
        ];

        [
            ("gregory", gregory.clone()),
            ("buddhist", vec![(0, tinystr!(16, "buddhist"))]),
            (
                "japanese",
                gregory
                    .clone()
                    .into_iter()
                    .chain(japanese.iter().enumerate().filter_map(|(i, (start, era))| {
                        if start.year < 1868 {
                            return None;
                        }
                        Some((i + 2, era))
                    }))
                    .collect(),
            ),
            (
                "japanext",
                gregory
                    .clone()
                    .into_iter()
                    .chain(
                        japanese
                            .iter()
                            .enumerate()
                            .map(|(i, (_, era))| (i + 2, era)),
                    )
                    .collect(),
            ),
            (
                "coptic",
                vec![
                    (0, tinystr!(16, "coptic-inverse")),
                    (1, tinystr!(16, "coptic")),
                ],
            ),
            ("dangi", vec![(0, tinystr!(16, "dangi"))]),
            ("chinese", vec![(0, tinystr!(16, "chinese"))]),
            ("indian", vec![(0, tinystr!(16, "indian"))]),
            ("islamic", vec![(0, tinystr!(16, "islamic"))]),
            ("islamic-civil", vec![(0, tinystr!(16, "islamic-civil"))]),
            (
                "islamic-umalqura",
                vec![(0, tinystr!(16, "islamic-umalqura"))],
            ),
            ("islamic-tbla", vec![(0, tinystr!(16, "islamic-tbla"))]),
            ("persian", vec![(0, tinystr!(16, "persian"))]),
            ("hebrew", vec![(0, tinystr!(16, "hebrew"))]),
            (
                "ethiopic",
                vec![(0, tinystr!(16, "ethioaa")), (1, tinystr!(16, "ethiopic"))],
            ),
            ("ethioaa", vec![(0, tinystr!(16, "ethioaa"))]),
            (
                "roc",
                vec![(0, tinystr!(16, "roc-inverse")), (1, tinystr!(16, "roc"))],
            ),
        ]
        .into_iter()
        .collect()
    })
}

// this test is trivial now, but in the future it should test the data in CLDR
#[test]
pub fn ethiopic_and_ethioaa_are_compatible() {
    let ethiopic = &get_era_code_map()["ethiopic"];
    let ethioaa = &get_era_code_map()["ethioaa"];
    assert_eq!(ethioaa.iter().zip(ethiopic).find(|(e, a)| e != a), None);
}

// this test is trivial now, but in the future it should test the data in CLDR
#[test]
pub fn japanese_and_japanext_are_compatible() {
    let japanese = &get_era_code_map()["japanese"];
    let japanext = &get_era_code_map()["japanext"];
    assert_eq!(
        japanext
            .iter()
            .take(2)
            .zip(japanese.iter().take(2))
            .find(|(e, a)| e != a),
        None
    );
    assert_eq!(
        japanext
            .iter()
            .skip(2)
            .rev()
            .zip(japanese.iter().skip(2).rev())
            .find(|(e, a)| e != a),
        None
    );
}

#[test]
pub fn test_era_code_map_consistency() {
    let provider = crate::SourceDataProvider::new_testing();

    let cldr_data = &provider
        .cldr()
        .unwrap()
        .core()
        .read_and_parse::<cldr_serde::japanese::Resource>("supplemental/calendarData.json")
        .unwrap()
        .supplemental
        .calendar_data;

    let hardcoded_data = get_era_code_map();

    for (calendar, data) in cldr_data {
        let calendar = match calendar.as_str() {
            "generic" => continue,
            "islamic-rgsa" => continue,
            "gregorian" => "gregory",
            "ethiopic-amete-alem" => "ethioaa",
            "japanese" => "japanext",
            c => c,
        };

        let hardcoded_era_codes = hardcoded_data[calendar]
            .iter()
            .map(|(i, name)| (*i, name.as_str()))
            .collect::<BTreeMap<_, _>>();

        let cldr_era_codes = data
            .eras
            .iter()
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.code.as_deref()))
            .map(|(k, v)| (if calendar == "japanext" { k + 2 } else { k }, v))
            .chain(data.inherit_eras.as_ref().into_iter().flat_map(|i| {
                cldr_data[&i.calendar]
                    .eras
                    .iter()
                    .map(|(a, b)| (a.parse::<usize>().unwrap(), b.code.as_deref()))
            }))
            .collect::<BTreeMap<_, _>>();

        // Compare, allowing ICU to have values where CLDR does not (pre-Meiji)
        assert!(
            hardcoded_era_codes
                .iter()
                .zip(&cldr_era_codes)
                .all(|((hk, hv), (ck, cv))| hk == ck && cv.map(|v| v == *hv).unwrap_or(true)),
            "{calendar}: {hardcoded_era_codes:?} != {cldr_era_codes:?}"
        );
    }
}

#[test]
fn test_calendar_eras() {
    use icu::calendar::preferences::CalendarAlgorithm;
    use icu::calendar::provider::EraStartDate;
    use icu::calendar::Iso;
    use icu::calendar::{AnyCalendar, AnyCalendarKind, Date};
    use icu::locale::extensions::unicode::Value;

    let provider = crate::SourceDataProvider::new_testing();

    let mut cldr_data = provider
        .cldr()
        .unwrap()
        .core()
        .read_and_parse::<cldr_serde::japanese::Resource>("supplemental/calendarData.json")
        .unwrap()
        .supplemental
        .calendar_data
        .clone();

    cldr_data
        .get_mut("ethiopic")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .end = None;

    cldr_data
        .get_mut("ethiopic")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: -5492,
        month: 7,
        day: 19,
    });

    cldr_data
        .get_mut("ethiopic-amete-alem")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .end = None;
    cldr_data
        .get_mut("ethiopic-amete-alem")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: -5492,
        month: 7,
        day: 19,
    });

    cldr_data
        .get_mut("chinese")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: -2636,
        month: 1,
        day: 1,
    });

    cldr_data
        .get_mut("dangi")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: -2331,
        month: 1,
        day: 1,
    });

    cldr_data
        .get_mut("hebrew")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: -3760,
        month: 9,
        day: 7,
    });

    cldr_data
        .get_mut("islamic-civil")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: 622,
        month: 7,
        day: 19,
    });

    cldr_data
        .get_mut("islamic")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: 622,
        month: 7,
        day: 19,
    });

    cldr_data
        .get_mut("islamic-tbla")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: 622,
        month: 7,
        day: 18,
    });

    cldr_data
        .get_mut("islamic-umalqura")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: 622,
        month: 7,
        day: 18,
    });

    cldr_data
        .get_mut("persian")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: 622,
        month: 3,
        day: 21,
    });

    cldr_data
        .get_mut("indian")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: 79,
        month: 3,
        day: 22,
    });

    cldr_data
        .get_mut("japanese")
        .unwrap()
        .eras
        .iter_mut()
        .for_each(|(_, era)| {
            if let Some(start) = era.start.as_mut() {
                if start.month == 2 && start.day > 28 {
                    start.day = if calendrical_calculations::iso::is_leap_year(start.year) {
                        29
                    } else {
                        28
                    };
                }
            }
        });

    for (calendar, data) in cldr_data {
        let kind = match calendar.as_str() {
            "generic" => continue,
            "islamic-rgsa" => continue,
            "gregorian" => AnyCalendarKind::Gregorian,
            "ethiopic-amete-alem" => AnyCalendarKind::EthiopianAmeteAlem,
            "japanese" => AnyCalendarKind::JapaneseExtended,
            c => icu::calendar::AnyCalendarKind::try_from(
                CalendarAlgorithm::try_from(&Value::try_from_str(c).unwrap()).unwrap(),
            )
            .unwrap(),
        };

        let cal = AnyCalendar::try_new_for_kind_unstable(&provider, kind).unwrap();
        let cal = icu::calendar::Ref(&cal);

        for (idx, era) in data.eras.values().cloned().enumerate() {
            let (in_era_iso, not_in_era_iso) = match (era.start, era.end) {
                (Some(start), None) => {
                    let start = Date::try_new_iso(start.year, start.month, start.day).unwrap();
                    (start, Iso::from_fixed(Iso::to_fixed(start) - 1))
                }
                (None, Some(end)) => {
                    let end = Date::try_new_iso(end.year, end.month, end.day).unwrap();
                    (end, Iso::from_fixed(Iso::to_fixed(end) + 1))
                }
                _ => unreachable!(),
            };

            let in_era = in_era_iso.to_calendar(cal);
            let not_in_era = not_in_era_iso.to_calendar(cal);

            // Check that code and aliases produce identical results
            for era in era
                .aliases
                .as_deref()
                .into_iter()
                .flat_map(|s| s.split(' '))
                .chain(era.code.as_deref())
            {
                assert_eq!(
                    Date::try_new_from_codes(
                        Some(era),
                        in_era.year().era_year_or_extended(),
                        in_era.month().standard_code,
                        in_era.day_of_month().0,
                        cal,
                    ),
                    Ok(in_era)
                );
            }

            // Unless this is the first era and it's not an inverse era, check that the
            // not_in_era date is in a different era
            if idx > 0 || era.end.is_some() {
                assert_ne!(
                    not_in_era.year().standard_era(),
                    in_era.year().standard_era()
                );
            }

            // The remaining tests don't work for cyclic calendars
            if calendar == "dangi" || calendar == "chinese" {
                continue;
            }

            // Check that the correct era code is returned
            if let Some(code) = era.code.as_deref() {
                assert_eq!(in_era.year().standard_era().unwrap().0, code);
            }

            // Check that the start/end date uses year 1, and minimal/maximal month/day
            assert_eq!(in_era.year().era_year_or_extended(), 1);
            if calendar == "japanese" {
                // Japanese is the only calendar that doesn't have its own months
            } else if era.start.is_some() {
                assert_eq!(in_era.month().ordinal, 1);
                assert_eq!(in_era.day_of_month().0, 1);
            } else {
                assert_eq!(in_era.month().ordinal, in_era.months_in_year());
                assert_eq!(in_era.day_of_month().0, in_era.days_in_month());
            }
        }
    }
}

macro_rules! symbols_from {
    ([$symbols: path, $name2: ident $(,)?], $ctx:ty, [ $($element: ident),+ $(,)? ] $(,)?) => {
        impl $symbols {
            fn get(&self, _ctx: &$ctx) -> $name2::Symbols<'static> {
                $name2::Symbols([
                    $(
                        Cow::Owned(self.$element.clone()),
                    )*
                ])
            }
        }
        symbols_from!([$symbols, $name2], $ctx);
    };
    ([$symbols: path, $name2: ident $(,)?], $ctx:ty, { $($element: ident),+ $(,)? } $(,)?) => {
        impl $symbols {
            fn get(&self, _ctx: &$ctx) -> $name2::Symbols<'static> {
                $name2::Symbols {
                    $(
                        $element: self.$element.clone(),
                    )*
                }
            }
        }
        symbols_from!([$symbols, $name2], $ctx);
    };
    ([$symbols: path, $name2: ident], $ctx:ty) => {
        impl $symbols {
            // Helper function which returns `None` if the two groups of symbols overlap.
            pub(crate) fn get_unaliased(&self, other: &Self) -> Option<Self> {
                if self == other {
                    None
                } else {
                    Some(self.clone())
                }
            }
        }

        impl ca::Contexts<$symbols> {
            fn get(&self, ctx: &$ctx) -> $name2::Contexts<'static> {
                $name2::Contexts {
                    format: self.format.get(ctx),
                    stand_alone: self.stand_alone.as_ref().and_then(|stand_alone| {
                        stand_alone.get_unaliased(&self.format)
                    }).map(|ref stand_alone| stand_alone.get(ctx))
                }
            }
        }

        impl ca::StandAloneWidths<$symbols> {
            // Helper function which returns `None` if the two groups of symbols overlap.
            pub(crate) fn get_unaliased(&self, other: &ca::FormatWidths<$symbols>) -> Option<Self> {
                let abbreviated = self.abbreviated.as_ref().and_then(|v| v.get_unaliased(&other.abbreviated));
                let narrow = self.narrow.as_ref().and_then(|v| v.get_unaliased(&other.narrow));
                let short = if self.short == other.short {
                    None
                } else {
                    self.short.clone()
                };
                let wide = self.wide.as_ref().and_then(|v| v.get_unaliased(&other.wide));

                if abbreviated.is_none() && narrow.is_none() && wide.is_none() && short.is_none() {
                    None
                } else {
                    Some(Self {
                        abbreviated,
                        narrow,
                        short,
                        wide,
                    })
                }
            }
        }

        impl ca::FormatWidths<$symbols> {
            fn get(&self, ctx: &$ctx) -> $name2::FormatWidths<'static> {
                $name2::FormatWidths {
                    abbreviated: self.abbreviated.get(ctx),
                    narrow: self.narrow.get(ctx),
                    short: self.short.as_ref().map(|width| width.get(ctx)),
                    wide: self.wide.get(ctx),
                }
            }
        }

        impl ca::StandAloneWidths<$symbols> {
            fn get(&self, ctx: &$ctx) -> $name2::StandAloneWidths<'static> {
                $name2::StandAloneWidths {
                    abbreviated: self.abbreviated.as_ref().map(|width| width.get(ctx)),
                    narrow: self.narrow.as_ref().map(|width| width.get(ctx)),
                    short: self.short.as_ref().map(|width| width.get(ctx)),
                    wide: self.wide.as_ref().map(|width| width.get(ctx)),
                }
            }
        }
    };
}
symbols_from!(
    [cldr_serde::ca::MonthSymbols, months],
    (&'static [TinyStr4], &str)
);

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

symbols_from!(
    [cldr_serde::ca::DaySymbols, weekdays],
    (),
    [sun, mon, tue, wed, thu, fri, sat]
);

symbols_from!(
    [
        cldr_serde::ca::DayPeriodSymbols,
        day_periods,
    ],
    (),
    {
        am,
        pm,
        noon,
        midnight,
    },
);

#[cfg(test)]
mod tests {
    use crate::SourceDataProvider;
    use icu::datetime::provider::calendar::*;
    use icu::datetime::provider::neo::*;
    use icu::locale::langid;
    use icu_provider::prelude::*;

    #[test]
    fn test_adapter_months_numeric() {
        let symbols: DataPayload<GregorianDateSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_month_abbreviated: DataPayload<GregorianMonthNamesV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("3"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_month_abbreviated:?}"),
            "Linear([\"Jan\", \"Feb\", \"Mar\", \"Apr\", \"May\", \"Jun\", \"Jul\", \"Aug\", \"Sep\", \"Oct\", \"Nov\", \"Dec\"])"
        );
    }

    #[test]
    fn test_adapter_months_map() {
        let symbols: DataPayload<HebrewDateSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_month_abbreviated: DataPayload<HebrewMonthNamesV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("3"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_month_abbreviated:?}"),
            "LeapLinear([\"Tishri\", \"Heshvan\", \"Kislev\", \"Tevet\", \"Shevat\", \"Adar\", \"Nisan\", \"Iyar\", \"Sivan\", \"Tamuz\", \"Av\", \"Elul\", \"\", \"\", \"\", \"\", \"Adar I\", \"Adar II\", \"\", \"\", \"\", \"\", \"\", \"\"])"
        );
    }

    #[test]
    fn test_adapter_weekdays_abbreviated() {
        let symbols: DataPayload<HebrewDateSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_weekdays_abbreviated: DataPayload<WeekdayNamesV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("3"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_weekdays_abbreviated:?}"),
            "LinearNames { names: [\"Sun\", \"Mon\", \"Tue\", \"Wed\", \"Thu\", \"Fri\", \"Sat\"] }"
        );
    }

    #[test]
    fn test_adapter_weekdays_short() {
        let symbols: DataPayload<HebrewDateSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_weekdays_short: DataPayload<WeekdayNamesV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("6s"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_weekdays_short:?}"),
            "LinearNames { names: [\"Su\", \"Mo\", \"Tu\", \"We\", \"Th\", \"Fr\", \"Sa\"] }"
        );
    }

    #[test]
    fn test_adapter_dayperiods() {
        let symbols: DataPayload<TimeSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_dayperiods_abbreviated: DataPayload<DayPeriodNamesV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("3s"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_dayperiods_abbreviated:?}"),
            "LinearNames { names: [\"AM\", \"PM\", \"noon\", \"midnight\"] }"
        );
    }
}
