// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(all(test, feature = "unstable", feature = "compiled_data"))]

use icu_calendar::Gregorian;
use icu_datetime::FixedCalendarDateTimeFormatter;
use icu_datetime::fieldbag::DateTimeFieldBag;
use icu_datetime::fieldbag::field::*;
use icu_datetime::fieldsets::builder::{DateFields, FieldSetBuilder, ZoneStyle};
use icu_datetime::fieldsets::enums::CompositeFieldSet;
use icu_datetime::options::{Alignment, Length, SubsecondDigits, TimePrecision, YearStyle};
use icu_locale_core::locale;
use writeable::Writeable;

const ERAS: &[Option<Era>] = &[None, Some(Era::Short), Some(Era::Long), Some(Era::Narrow)];
const YEARS: &[Option<Year>] = &[None, Some(Year::Numeric), Some(Year::TwoDigit)];
const MONTHS: &[Option<Month>] = &[
    None,
    Some(Month::Numeric),
    Some(Month::TwoDigit),
    Some(Month::Short),
    Some(Month::Long),
    Some(Month::Narrow),
];
const DAYS: &[Option<Day>] = &[None, Some(Day::Numeric), Some(Day::TwoDigit)];
const WEEKDAYS: &[Option<Weekday>] = &[
    None,
    Some(Weekday::Short),
    Some(Weekday::Long),
    Some(Weekday::Narrow),
];
const DAY_PERIODS: &[Option<DayPeriod>] = &[
    None,
    Some(DayPeriod::FlexibleShort),
    Some(DayPeriod::FlexibleLong),
    Some(DayPeriod::FlexibleNarrow),
];
const HOUR_KINDS: &[Option<HourKind>] = &[None, Some(HourKind::Clock12), Some(HourKind::Clock24)];
const HOURS: &[Option<Hour>] = &[None, Some(Hour::Numeric), Some(Hour::TwoDigit)];
const MINUTES: &[Option<Minute>] = &[None, Some(Minute::Numeric), Some(Minute::TwoDigit)];
const SECONDS: &[Option<Second>] = &[None, Some(Second::Numeric), Some(Second::TwoDigit)];
const FRACTIONAL_SECOND_DIGITS: &[Option<FractionalSecondDigits>] = &[
    None,
    Some(FractionalSecondDigits::F1),
    Some(FractionalSecondDigits::F2),
    Some(FractionalSecondDigits::F3),
];
const TIME_ZONE_NAMES: &[Option<TimeZoneName>] = &[
    None,
    Some(TimeZoneName::ShortSpecific),
    Some(TimeZoneName::LongSpecific),
    Some(TimeZoneName::ShortOffset),
    Some(TimeZoneName::LongOffset),
    Some(TimeZoneName::ShortGeneric),
    Some(TimeZoneName::LongGeneric),
];

const LENGTHS: &[Option<Length>] = &[
    None,
    Some(Length::Long),
    Some(Length::Medium),
    Some(Length::Short),
];
const TIME_PRECISIONS: &[Option<TimePrecision>] = &[
    None,
    Some(TimePrecision::Hour),
    Some(TimePrecision::Minute),
    Some(TimePrecision::MinuteOptional),
    Some(TimePrecision::Second),
    Some(TimePrecision::Subsecond(SubsecondDigits::S1)),
    Some(TimePrecision::Subsecond(SubsecondDigits::S2)),
    Some(TimePrecision::Subsecond(SubsecondDigits::S3)),
    Some(TimePrecision::Subsecond(SubsecondDigits::S4)),
    Some(TimePrecision::Subsecond(SubsecondDigits::S5)),
    Some(TimePrecision::Subsecond(SubsecondDigits::S6)),
    Some(TimePrecision::Subsecond(SubsecondDigits::S7)),
    Some(TimePrecision::Subsecond(SubsecondDigits::S8)),
    Some(TimePrecision::Subsecond(SubsecondDigits::S9)),
];
const ALIGNMENTS: &[Option<Alignment>] = &[None, Some(Alignment::Auto), Some(Alignment::Column)];
const YEAR_STYLES: &[Option<YearStyle>] = &[
    None,
    Some(YearStyle::Auto),
    Some(YearStyle::Full),
    Some(YearStyle::WithEra),
    Some(YearStyle::NoEra),
];

fn all_date_field_bags() -> impl Iterator<Item = DateTimeFieldBag> {
    ERAS.iter().flat_map(|&era| {
        YEARS.iter().flat_map(move |&year| {
            MONTHS.iter().flat_map(move |&month| {
                DAYS.iter().flat_map(move |&day| {
                    WEEKDAYS.iter().map(move |&weekday| {
                        let mut bag = DateTimeFieldBag::default();
                        bag.era = era;
                        bag.year = year;
                        bag.month = month;
                        bag.day = day;
                        bag.weekday = weekday;
                        bag
                    })
                })
            })
        })
    })
}

fn all_time_field_bags() -> impl Iterator<Item = DateTimeFieldBag> {
    DAY_PERIODS.iter().flat_map(|&day_period| {
        HOUR_KINDS.iter().flat_map(move |&hour_kind| {
            HOURS.iter().flat_map(move |&hour| {
                MINUTES.iter().flat_map(move |&minute| {
                    SECONDS.iter().flat_map(move |&second| {
                        FRACTIONAL_SECOND_DIGITS
                            .iter()
                            .map(move |&fractional_second_digits| {
                                let mut bag = DateTimeFieldBag::default();
                                bag.day_period = day_period;
                                bag.hour_kind = hour_kind;
                                bag.hour = hour;
                                bag.minute = minute;
                                bag.second = second;
                                bag.fractional_second_digits = fractional_second_digits;
                                bag
                            })
                    })
                })
            })
        })
    })
}

/// Returns a comprehensive set of `DateTimeFieldBag` combinations without taking the full
/// 864 x 1,296 x 7 = 7,838,208 Cartesian product in debug builds:
/// - All 864 date combinations x all 7 zone combinations (6,048 bags)
/// - All 1,296 time combinations x all 7 zone combinations (9,072 bags)
/// - Representative date combinations x representative time combinations x all 7 zone combinations
fn representative_field_bags() -> impl Iterator<Item = DateTimeFieldBag> {
    let date_and_zone = all_date_field_bags().flat_map(|date_bag| {
        TIME_ZONE_NAMES.iter().map(move |&time_zone_name| {
            let mut bag = date_bag;
            bag.time_zone_name = time_zone_name;
            bag
        })
    });

    let time_and_zone = all_time_field_bags().flat_map(|time_bag| {
        TIME_ZONE_NAMES.iter().map(move |&time_zone_name| {
            let mut bag = time_bag;
            bag.time_zone_name = time_zone_name;
            bag
        })
    });

    // Cross-product of all 864 date bags with a subset of time bags covering all time precisions,
    // alignments, hour kinds, and day periods, across all 7 zone styles.
    let sample_time_bags: Vec<DateTimeFieldBag> = [
        "",
        "B",
        "h",
        "H",
        "j",
        "jj",
        "jm",
        "jjmm",
        "jms",
        "jjmmss",
        "jmsS",
        "jjmmssSSS",
        "ms",
        "S",
    ]
    .into_iter()
    .map(|s| DateTimeFieldBag::try_from_skeleton(s).unwrap())
    .collect();

    let date_time_zone = all_date_field_bags().flat_map(move |date_bag| {
        sample_time_bags
            .clone()
            .into_iter()
            .flat_map(move |time_bag| {
                TIME_ZONE_NAMES.iter().map(move |&time_zone_name| {
                    let mut bag = date_bag;
                    bag.day_period = time_bag.day_period;
                    bag.hour_kind = time_bag.hour_kind;
                    bag.hour = time_bag.hour;
                    bag.minute = time_bag.minute;
                    bag.second = time_bag.second;
                    bag.fractional_second_digits = time_bag.fractional_second_digits;
                    bag.time_zone_name = time_zone_name;
                    bag
                })
            })
    });

    date_and_zone.chain(time_and_zone).chain(date_time_zone)
}

fn all_field_set_builders() -> impl Iterator<Item = FieldSetBuilder> {
    let date_fields_iter = [None]
        .into_iter()
        .chain(DateFields::VALUES.iter().copied().map(Some));
    let zone_styles_iter = [None]
        .into_iter()
        .chain(ZoneStyle::VALUES.iter().copied().map(Some));

    date_fields_iter.flat_map(move |date_fields| {
        zone_styles_iter.clone().flat_map(move |zone_style| {
            LENGTHS.iter().flat_map(move |&length| {
                TIME_PRECISIONS.iter().flat_map(move |&time_precision| {
                    ALIGNMENTS.iter().flat_map(move |&alignment| {
                        YEAR_STYLES.iter().map(move |&year_style| {
                            let mut builder = FieldSetBuilder::new();
                            builder.length = length;
                            builder.date_fields = date_fields;
                            builder.time_precision = time_precision;
                            builder.zone_style = zone_style;
                            builder.alignment = alignment;
                            builder.year_style = year_style;
                            builder
                        })
                    })
                })
            })
        })
    })
}

#[test]
fn test_skeleton_and_fieldbag() {
    let mut skeleton = String::new();
    let mut roundtrip_skeleton = String::new();

    for bag in representative_field_bags() {
        skeleton.clear();
        bag.write_to(&mut skeleton).unwrap();

        let parsed = DateTimeFieldBag::try_from_skeleton(&skeleton).unwrap_or_else(|err| {
            panic!("failed to parse generated skeleton {skeleton:?} from {bag:?}: {err:?}")
        });
        assert_eq!(parsed, DateTimeFieldBag::from_skeleton(&skeleton));

        // In UTS 35 skeletons, `hour_kind` (`h` vs `H` vs `j`) is encoded on the hour symbol.
        // When `hour` is `None`, no hour symbol is emitted, so `hour_kind` is dropped; all other
        // fields round-trip losslessly.
        let mut expected = bag;
        if expected.hour.is_none() {
            expected.hour_kind = None;
        }
        assert_eq!(parsed, expected, "skeleton: {skeleton:?}");

        // The parsed bag must always re-serialize to the exact same skeleton string.
        roundtrip_skeleton.clear();
        parsed.write_to(&mut roundtrip_skeleton).unwrap();
        assert_eq!(skeleton, roundtrip_skeleton);
    }
}

#[test]
fn test_fieldset_builder_and_fieldbag() {
    // 1. Bag -> Builder -> CompositeFieldSet -> Bag -> Builder
    for bag in representative_field_bags() {
        let builder1 = bag.to_field_set_builder();
        let composite1 = builder1.clone().build_composite().unwrap_or_else(|err| {
            panic!("bag {bag:?} produced invalid builder {builder1:?}: {err:?}")
        });
        assert_eq!(bag.to_composite_field_set(), composite1);

        // Round-tripping through DateTimeFieldBag must preserve the selected fields
        // (`date_fields`, `time_precision`, `zone_style`) and reach a strict fixed point.
        // Note: It can take up to 2 round-trips to stabilize when both `month` and `weekday`
        // are present (e.g. `Month::Numeric` + `Weekday::Narrow` -> `MDE::short()` ->
        // `Month::Numeric` + `Weekday::Short` -> `MDE::medium()` -> `Month::Short` + `Weekday::Short`).
        let bag1 = DateTimeFieldBag::from_field_set_builder(&builder1);
        let builder2 = bag1.to_field_set_builder();
        let composite2 = builder2.clone().build_composite().unwrap();
        let bag2 = DateTimeFieldBag::from_field_set_builder(&builder2);
        let builder3 = bag2.to_field_set_builder();
        let composite3 = builder3.clone().build_composite().unwrap();
        let bag3 = DateTimeFieldBag::from_field_set_builder(&builder3);

        assert_eq!(builder1.date_fields, builder2.date_fields, "bag: {bag:?}");
        assert_eq!(
            builder1.time_precision, builder2.time_precision,
            "bag: {bag:?}"
        );
        assert_eq!(builder1.zone_style, builder2.zone_style, "bag: {bag:?}");
        // Except when `builder1` is `MDE`/`YMDE` with `Length::Short` (where `bag` had
        // numeric month + `Weekday::Narrow`, which `from_field_set_builder` turns into
        // numeric month + `Weekday::Short`, causing `to_field_set_builder` to promote
        // length to `Length::Medium`), `bag1` is already a fixed point on the first step.
        if !(matches!(
            builder1.date_fields,
            Some(DateFields::MDE | DateFields::YMDE)
        ) && builder1.length == Some(Length::Short))
        {
            assert_eq!(bag1, bag2, "bag: {bag:?}");
        }
        assert_eq!(bag2, bag3, "bag: {bag:?}");
        assert_eq!(builder2, builder3, "bag: {bag:?}");
        assert_eq!(composite2, composite3, "bag: {bag:?}");
    }

    // 2. Builder -> Bag -> Builder across all 83,160 FieldSetBuilder combinations
    for builder in all_field_set_builders() {
        let bag1 = DateTimeFieldBag::from_field_set_builder(&builder);
        let builder1 = bag1.to_field_set_builder();
        let composite1 = builder1.clone().build_composite().unwrap_or_else(|err| {
            panic!("builder {builder:?} -> bag {bag1:?} -> invalid builder {builder1:?}: {err:?}")
        });

        let bag2 = DateTimeFieldBag::from_field_set_builder(&builder1);
        let builder2 = bag2.to_field_set_builder();
        let composite2 = builder2.clone().build_composite().unwrap();

        let bag3 = DateTimeFieldBag::from_field_set_builder(&builder2);
        let builder3 = bag3.to_field_set_builder();
        let composite3 = builder3.clone().build_composite().unwrap();

        assert_eq!(
            builder1.date_fields, builder2.date_fields,
            "builder: {builder:?}"
        );
        assert_eq!(
            builder1.time_precision, builder2.time_precision,
            "builder: {builder:?}"
        );
        assert_eq!(
            builder1.zone_style, builder2.zone_style,
            "builder: {builder:?}"
        );
        // For any valid `FieldSetBuilder` (except `MDE`/`YMDE` with `Length::Short`, where
        // `from_field_set_builder` emits `Month::Numeric` + `Weekday::Short`, which
        // `to_field_set_builder` maps to `Length::Medium`), `bag1`/`builder1`/`composite1`
        // are already at a fixed point on the first step.
        // (Invalid builders, such as an empty builder or `CalendarPeriod` + `Zone`, have
        // missing fields filled in when `bag1` converts to `builder1`, so they stabilize
        // on the second step `bag2`/`builder2`/`composite2`.)
        if builder.clone().build_composite().is_ok()
            && !(matches!(
                builder.date_fields,
                Some(DateFields::MDE | DateFields::YMDE)
            ) && builder.length == Some(Length::Short))
        {
            assert_eq!(bag1, bag2, "builder: {builder:?}");
            assert_eq!(builder1, builder2, "builder: {builder:?}");
            assert_eq!(composite1, composite2, "builder: {builder:?}");
        }
        assert_eq!(bag2, bag3, "builder: {builder:?}");
        assert_eq!(builder2, builder3, "builder: {builder:?}");
        assert_eq!(composite2, composite3, "builder: {builder:?}");
    }
}

#[test]
fn test_fieldset_formatter_and_builder() {
    let prefs = locale!("en").into();

    for builder in all_field_set_builders() {
        let Ok(fieldset) = builder.clone().build_composite() else {
            continue;
        };

        // Skip duplicate builders where `length: None` defaulted to `Length::Medium`
        // for field sets that carry a length option.
        if builder.length.is_none() && !matches!(fieldset, CompositeFieldSet::Zone(_)) {
            continue;
        }

        let formatter =
            FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(prefs, fieldset).unwrap();
        let recovered_builder = formatter.to_field_set_builder();

        assert_eq!(recovered_builder, builder, "fieldset: {fieldset:?}");
        assert_eq!(
            recovered_builder.build_composite().unwrap(),
            fieldset,
            "builder: {builder:?}"
        );
    }
}
