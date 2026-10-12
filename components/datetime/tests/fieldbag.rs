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
use icu_datetime::options::{Alignment, Length, TimePrecision, YearStyle};
use icu_locale_core::locale;
use writeable::Writeable;

/// Returns an iterator yielding `None` followed by `Some(v)` for every variant in `values`.
fn with_none<T: Copy>(values: &'static [T]) -> impl Iterator<Item = Option<T>> + Clone {
    [None].into_iter().chain(values.iter().copied().map(Some))
}

/// Yields all 864 combinations of date fields (`era`, `year`, `month`, `day`, `weekday`).
fn all_date_field_bags() -> impl Iterator<Item = DateTimeFieldBag> {
    with_none(Era::VALUES).flat_map(|era| {
        with_none(Year::VALUES).flat_map(move |year| {
            with_none(Month::VALUES).flat_map(move |month| {
                with_none(Day::VALUES).flat_map(move |day| {
                    with_none(Weekday::VALUES).map(move |weekday| {
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

/// Yields all 1,296 combinations of time fields (`day_period`, `hour_kind`, `hour`,
/// `minute`, `second`, `fractional_second_digits`).
fn all_time_field_bags() -> impl Iterator<Item = DateTimeFieldBag> {
    with_none(DayPeriod::VALUES).flat_map(|day_period| {
        with_none(HourKind::VALUES).flat_map(move |hour_kind| {
            with_none(Hour::VALUES).flat_map(move |hour| {
                with_none(Minute::VALUES).flat_map(move |minute| {
                    with_none(Second::VALUES).flat_map(move |second| {
                        with_none(FractionalSecondDigits::VALUES).map(
                            move |fractional_second_digits| {
                                let mut bag = DateTimeFieldBag::default();
                                bag.day_period = day_period;
                                bag.hour_kind = hour_kind;
                                bag.hour = hour;
                                bag.minute = minute;
                                bag.second = second;
                                bag.fractional_second_digits = fractional_second_digits;
                                bag
                            },
                        )
                    })
                })
            })
        })
    })
}

/// Yields a comprehensive set of `DateTimeFieldBag` combinations without taking the full
/// 864 x 1,296 x 7 = 7,838,208 Cartesian product in debug builds:
/// - All 864 date combinations x all 7 zone combinations (6,048 bags)
/// - All 1,296 time combinations x all 7 zone combinations (9,072 bags)
/// - All 864 date combinations x 14 representative time combinations x all 7 zone combinations
fn representative_field_bags() -> impl Iterator<Item = DateTimeFieldBag> {
    let date_and_zone = all_date_field_bags().flat_map(|date_bag| {
        with_none(TimeZoneName::VALUES).map(move |time_zone_name| {
            let mut bag = date_bag;
            bag.time_zone_name = time_zone_name;
            bag
        })
    });

    let time_and_zone = all_time_field_bags().flat_map(|time_bag| {
        with_none(TimeZoneName::VALUES).map(move |time_zone_name| {
            let mut bag = time_bag;
            bag.time_zone_name = time_zone_name;
            bag
        })
    });

    // Cross-product of all 864 date bags with a subset of time bags covering all time precisions,
    // alignments, hour kinds, and day periods, across all 7 zone styles.
    let sample_time_bags = [
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
    .map(|s| DateTimeFieldBag::try_from_skeleton(s).unwrap());

    let date_time_zone = all_date_field_bags().flat_map(move |date_bag| {
        sample_time_bags.into_iter().flat_map(move |time_bag| {
            with_none(TimeZoneName::VALUES).map(move |time_zone_name| {
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

/// Yields all 83,160 combinations of `FieldSetBuilder` options (`date_fields`, `zone_style`,
/// `length`, `time_precision`, `alignment`, `year_style`).
fn all_field_set_builders() -> impl Iterator<Item = FieldSetBuilder> {
    with_none(DateFields::VALUES).flat_map(move |date_fields| {
        with_none(ZoneStyle::VALUES).flat_map(move |zone_style| {
            with_none(Length::VALUES).flat_map(move |length| {
                with_none(TimePrecision::VALUES).flat_map(move |time_precision| {
                    with_none(Alignment::VALUES).flat_map(move |alignment| {
                        with_none(YearStyle::VALUES).map(move |year_style| {
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

/// Returns the expected `DateTimeFieldBag` after round-tripping `bag` (produced by
/// `from_field_set_builder`) through `FieldSetBuilder` (`bag -> builder -> expected_bag`).
fn expected_bag_after_builder_roundtrip(bag: &DateTimeFieldBag) -> DateTimeFieldBag {
    let mut expected = *bag;
    // When both `month` and `weekday` are present, `Weekday::Short` promotes a numeric month's
    // `Length::Short` to `Length::Medium`, which materializes as `Month::Short`.
    if expected.weekday.is_some()
        && matches!(expected.month, Some(Month::Numeric | Month::TwoDigit))
    {
        expected.month = Some(Month::Short);
    }
    // When `bag` came from an invalid `FieldSetBuilder` (e.g. `YearStyle::WithEra` without a
    // year field, an empty builder, or `CalendarPeriod` + `Time`/`Zone`), `to_field_set_builder`
    // fills in missing date fields on the resulting `DateFields`:
    let builder = bag.to_field_set_builder();
    if matches!(
        builder.date_fields,
        Some(DateFields::Y | DateFields::YM | DateFields::YMD | DateFields::YMDE)
    ) {
        expected.year.get_or_insert(Year::Numeric);
    }
    if matches!(
        builder.date_fields,
        Some(DateFields::MD | DateFields::YMD | DateFields::MDE | DateFields::YMDE)
    ) {
        let default_month = if expected.weekday == Some(Weekday::Long) {
            Month::Long
        } else {
            Month::Short
        };
        expected.month.get_or_insert(default_month);
        let default_day =
            if expected.month == Some(Month::TwoDigit) || expected.hour == Some(Hour::TwoDigit) {
                Day::TwoDigit
            } else {
                Day::Numeric
            };
        expected.day.get_or_insert(default_day);
    }
    expected
}

/// Returns the expected `FieldSetBuilder` after round-tripping `builder` through
/// `DateTimeFieldBag` (`builder -> bag -> expected_builder`).
fn expected_builder_after_bag_roundtrip(builder: &FieldSetBuilder) -> FieldSetBuilder {
    let mut expected = builder.clone();
    // When `builder` filled in missing `YMD` fields (e.g. from `""`, `"yd"`, or `"Mz"`),
    // converting to `DateTimeFieldBag` materializes `Month::Short` (`Length::Medium`) and
    // `Year::Numeric` (`YearStyle::Full`) where `builder` had `None`.
    if expected.date_fields == Some(DateFields::YMD) {
        expected.length.get_or_insert(Length::Medium);
        expected.year_style.get_or_insert(YearStyle::Full);
    }
    // When `builder` has neither a month nor a weekday, `DateTimeFieldBag` has no field that
    // carries `Length` (since `day_period` is dropped in `from_field_set_builder`, #487).
    if matches!(
        expected.date_fields,
        None | Some(DateFields::D | DateFields::Y)
    ) {
        expected.length = None;
    }
    expected
}

/// Returns the expected `FieldSetBuilder` after building `fieldset` and converting back via
/// `FixedCalendarDateTimeFormatter::to_field_set_builder()`.
fn expected_builder_after_composite_roundtrip(
    builder: &FieldSetBuilder,
    fieldset: CompositeFieldSet,
) -> FieldSetBuilder {
    let mut expected = builder.clone();
    // All `CompositeFieldSet` variants except `Zone` carry a concrete `Length`, defaulting
    // `length: None` to `Length::Medium`.
    if !matches!(fieldset, CompositeFieldSet::Zone(_)) {
        expected.length.get_or_insert(Length::Medium);
    }
    expected
}

/// Tests UTS 35 skeleton string serialization (`Writeable` / `Display`) and parsing
/// (`try_from_skeleton` / `from_skeleton`) across `representative_field_bags()`, verifying
/// that every `DateTimeFieldBag` round-trips through a skeleton string (with standalone
/// `hour_kind` dropped when `hour` is `None`) and re-serializes to the identical string.
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

/// Tests conversions between `DateTimeFieldBag` and `FieldSetBuilder` in both directions:
/// 1. `Bag -> Builder -> Bag -> Builder`: verifies that every bag produces a valid
///    `CompositeFieldSet` and reaches a fixed point on the first round-trip.
/// 2. `Builder -> Bag -> Builder`: verifies across all 83,160 `FieldSetBuilder` combinations
///    that valid builders stabilize immediately and invalid builders stabilize after missing
///    fields are filled in (`bag2 == bag3` and `builder2 == builder3`).
#[test]
fn test_fieldset_builder_and_fieldbag() {
    // 1. Bag -> Builder -> Bag -> Builder
    for bag in representative_field_bags() {
        let builder1 = bag.to_field_set_builder();
        let composite1 = builder1.clone().build_composite().unwrap_or_else(|err| {
            panic!("bag {bag:?} produced invalid builder {builder1:?}: {err:?}")
        });
        assert_eq!(bag.to_composite_field_set(), composite1);

        let bag1 = DateTimeFieldBag::from_field_set_builder(&builder1);
        let builder2 = bag1.to_field_set_builder();
        let bag2 = DateTimeFieldBag::from_field_set_builder(&builder2);
        let builder3 = bag2.to_field_set_builder();

        assert_eq!(bag1, bag2, "bag: {bag:?}");
        assert_eq!(
            expected_builder_after_bag_roundtrip(&builder1),
            builder2,
            "bag: {bag:?}"
        );
        assert_eq!(builder2, builder3, "bag: {bag:?}");
    }

    // 2. Builder -> Bag -> Builder across all 83,160 FieldSetBuilder combinations
    for builder in all_field_set_builders() {
        let bag1 = DateTimeFieldBag::from_field_set_builder(&builder);
        let builder1 = bag1.to_field_set_builder();
        let _composite1 = builder1.clone().build_composite().unwrap_or_else(|err| {
            panic!("builder {builder:?} -> bag {bag1:?} -> invalid builder {builder1:?}: {err:?}")
        });

        let bag2 = DateTimeFieldBag::from_field_set_builder(&builder1);
        let builder2 = bag2.to_field_set_builder();
        let bag3 = DateTimeFieldBag::from_field_set_builder(&builder2);
        let builder3 = bag3.to_field_set_builder();

        assert_eq!(
            expected_bag_after_builder_roundtrip(&bag1),
            bag2,
            "builder: {builder:?}"
        );
        assert_eq!(
            expected_builder_after_bag_roundtrip(&builder1),
            builder2,
            "builder: {builder:?}"
        );
        assert_eq!(bag2, bag3, "builder: {builder:?}");
        assert_eq!(builder2, builder3, "builder: {builder:?}");
    }
}

/// Tests round-tripping between `FieldSetBuilder`, `CompositeFieldSet`, and
/// `FixedCalendarDateTimeFormatter::to_field_set_builder()` across all valid builders.
#[test]
fn test_fieldset_formatter_and_builder() {
    let prefs = locale!("en").into();
    let mut valid_builder_count = 0;

    for builder in all_field_set_builders() {
        // `all_field_set_builders()` is an exhaustive Cartesian product of all builder
        // options, including invalid combinations (e.g. `year_style` without a year field,
        // or `CalendarPeriod` + `TimePrecision`) that `build_composite()` rightly rejects.
        let Ok(fieldset) = builder.clone().build_composite() else {
            continue;
        };
        valid_builder_count += 1;

        let formatter =
            FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(prefs, fieldset).unwrap();
        let recovered_builder = formatter.to_field_set_builder();

        assert_eq!(
            expected_builder_after_composite_roundtrip(&builder, fieldset),
            recovered_builder,
            "fieldset: {fieldset:?}"
        );
        assert_eq!(
            recovered_builder.build_composite().unwrap(),
            fieldset,
            "builder: {builder:?}"
        );
    }

    assert_eq!(valid_builder_count, 24152);
}
