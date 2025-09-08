// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use icu::datetime::provider::skeleton::reference::Skeleton;
use icu::datetime::provider::skeleton::*;
use icu::plurals::PluralCategory;
use std::collections::HashMap;
use std::convert::TryFrom;

impl From<&cldr_serde::ca::AvailableFormats> for DateSkeletonPatterns<'_> {
    fn from(other: &cldr_serde::ca::AvailableFormats) -> Self {
        let mut patterns: HashMap<String, HashMap<String, String>> = HashMap::new();

        // The CLDR keys for available_formats can have duplicate skeletons with either
        // an additional variant, or with multiple variants for different plurals.
        for (skeleton_str, pattern_str) in other.0.iter() {
            let (skeleton_str, plural_form_str) = match skeleton_str.split_once("-count-") {
                Some((s, v)) => (s, v),
                None => (skeleton_str.as_ref(), "other"),
            };

            patterns
                .entry(skeleton_str.to_string())
                .or_default()
                .insert(plural_form_str.to_string(), pattern_str.to_string());
        }

        let skeletons = patterns
            .iter()
            .filter_map(|(skeleton_str, patterns)| {
                let skeleton = match Skeleton::try_from(skeleton_str.as_str()) {
                    Ok(s) => s,
                    Err(SkeletonError::SymbolUnimplemented(_)) => return None,
                    Err(SkeletonError::SkeletonHasVariant) => return None,
                    Err(err) => panic!(
                        "Unexpected skeleton error while parsing skeleton {skeleton_str:?} {err}"
                    ),
                };
                let pattern_str = patterns.get("other").expect("Other variant must exist");
                let pattern = pattern_str.parse().expect("Unable to parse a pattern");

                #[allow(unreachable_code, unused_variables, unused_mut)] // TODO(#5643)
                let mut pattern_plurals = if patterns.len() == 1 {
                    PatternPlurals::SinglePattern(pattern)
                } else {
                    let mut plural_pattern =
                        PluralPattern::new(pattern).expect("Unable to construct PluralPattern");

                    for (key, pattern_str) in patterns {
                        if key == "other" {
                            continue;
                        }
                        let cat = PluralCategory::get_for_cldr_string(key)
                            .expect("Failed to retrieve plural category");
                        let pattern = pattern_str.parse().expect("Unable to parse a pattern");
                        plural_pattern.maybe_set_variant(cat, pattern);
                    }
                    PatternPlurals::MultipleVariants(plural_pattern)
                };
                // In some cases we may encounter duplicated patterns, which will
                // get deduplicated and result in a single-variant `MultiVariant` branch
                // here. The following `normalize` will turn those cases to `SingleVariant`.
                pattern_plurals.normalize();

                Some((SkeletonData(skeleton), pattern_plurals))
            })
            .collect();

        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self(skeletons)
    }
}

#[cfg(test)]
mod test {
    use super::super::legacy::DateLengths;
    use super::*;
    use core::convert::TryFrom;
    use core::str::FromStr;
    use icu::datetime::provider::fields::components;
    use icu::datetime::provider::skeleton::reference::Skeleton;
    use icu::datetime::{
        provider::fields::{Day, Field, FieldLength, Month, Weekday},
        provider::pattern::{reference, runtime},
    };
    use icu::locale::locale;
    use icu::locale::preferences::extensions::unicode::keywords::HourCycle;
    use litemap::LiteMap;

    use crate::datetime::DatagenCalendar;
    use crate::SourceDataProvider;

    fn get_data_payload() -> (DateLengths<'static>, DateSkeletonPatterns<'static>) {
        let locale = locale!("en").into();

        let data = SourceDataProvider::new_testing()
            .get_datetime_resources(&locale, Some(DatagenCalendar::Gregorian))
            .unwrap();
        let patterns = DateLengths::from(&data);
        let skeletons = DateSkeletonPatterns::from(&data.datetime_formats.available_formats);
        (patterns, skeletons)
    }

    /// This is an initial smoke test to verify the skeleton machinery is working. For more in-depth
    /// testing see components/datetime/tests/fixtures/tests/components-*.json
    #[test]
    fn test_skeleton_matching() {
        let mut components = components::Bag::empty();
        components.year = Some(components::Year::Numeric);
        components.month = Some(components::Month::Long);
        components.day = Some(components::Day::NumericDayOfMonth);

        components.hour = Some(components::Numeric::Numeric);
        components.minute = Some(components::Numeric::Numeric);
        components.second = Some(components::Numeric::Numeric);

        let requested_fields = components.to_vec_fields(HourCycle::H23);
        let (_, skeletons) = get_data_payload();

        match get_best_available_format_pattern(&skeletons, &requested_fields, false) {
            BestSkeleton::AllFieldsMatch(available_format_pattern, _)
            | BestSkeleton::MissingOrExtraFields(available_format_pattern, _) => {
                assert_eq!(
                    available_format_pattern
                        .expect_pattern("pattern should not have plural variants")
                        .to_string()
                        .as_str(),
                    "H:m:s"
                )
            }
            BestSkeleton::NoMatch => {
                panic!("No skeleton was found.")
            }
        };
    }

    #[test]
    fn test_skeleton_matching_missing_fields() {
        let mut components = components::Bag::empty();
        components.time_zone_name = Some(components::TimeZoneName::LongOffset);
        components.weekday = Some(components::Text::Short);
        let requested_fields = components.to_vec_fields(HourCycle::H23);
        let (_, skeletons) = get_data_payload();

        match get_best_available_format_pattern(&skeletons, &requested_fields, false) {
            BestSkeleton::MissingOrExtraFields(available_format_pattern, _) => {
                assert_eq!(
                    available_format_pattern
                        .expect_pattern("pattern should not have plural variants")
                        .to_string()
                        .as_str(),
                    // CLDR has ("yw", "MMMMW", "ccc"). The first two result in 1 missing & 1 extra symbol vs just
                    // 1 missing symbol for "ccc".
                    "ccc"
                )
            }
            best => panic!("Unexpected {best:?}"),
        };
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
        let (patterns, skeletons) = get_data_payload();

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
    fn test_skeleton_empty_bag() {
        let components: components::Bag = Default::default();
        let requested_fields = components.to_vec_fields(HourCycle::H23);
        let (_, skeletons) = get_data_payload();

        assert_eq!(
            get_best_available_format_pattern(&skeletons, &requested_fields, false),
            BestSkeleton::NoMatch,
            "No match was found"
        );
    }

    #[test]
    fn test_skeleton_no_match() {
        let mut components = components::Bag::empty();
        components.hour = Some(components::Numeric::Numeric);
        components.time_zone_name = Some(components::TimeZoneName::LongSpecific);
        let requested_fields = components.to_vec_fields(HourCycle::H23);
        // Construct a set of skeletons that do not use the hour nor time zone symbols.
        let mut skeletons = LiteMap::new();
        skeletons.insert(
            SkeletonData::try_from("EEEE").unwrap(),
            runtime::Pattern::from_str("weekday EEEE").unwrap().into(),
        );
        let skeletons = DateSkeletonPatterns(skeletons);

        assert_eq!(
            get_best_available_format_pattern(&skeletons, &requested_fields, false),
            BestSkeleton::NoMatch,
            "No match was found"
        );
    }

    // These were all of the skeletons from the "available formats" in the CLDR as of 2021-01
    // Generated with:
    // https://gist.github.com/gregtatum/1d76bbdb87132f71a969a10f0c1d2d9c

    #[rustfmt::skip]
    const SUPPORTED_STRING_SKELETONS: &[&str] = &[
        "E", "dEEEE", "EHm", "EHms", "dE", "Ehm", "Ehms", "H", "HHmm", "HHmmss", "Hm", "Hms", "M",
        "MdEEEE", "MdE", "MMM", "MMMdEEEE", "MMMdE", "MMMM",
        "MMMMdEEEE", "MMMMdE", "MMMMd",
        "MMMMdd", "MMMd", "MMMdd", "MMd", "MMdd", "Md", "Mdd", "d", "h", "hm", "hms", "mmss", "ms",
        "y", "yM", "yMdEEEE", "yMdE", "yMM", "yMMM", "yMMMdEEEE", "yMMMdE", "yMMMM", "yMMMMdEEEE",
        "yMMMMdE", "yMMMMdcccc", "yMMMMd", "yMMMd", "yMMdd", "yMd", 
        "Gy", "GyM", "GyMMM", "GyMMMdEEEE", "GyMMMdE", "GyMMMM", "GyMMMMdE", "GyMMMMd", "GyMMMd",
        // Time zones
        "HHmmZ", "Hmsv", "Hmsvvvv", "Hmv", "Hmvvvv", "hmsv", "hmsvvvv", "hmv", "hmvvvv",
    ];

    // NOTE: If you are moving this to the SUPPORTED section, make sure to remove the match
    //       on your symbol from impl From<fields::SymbolError> for SkeletonError
    //       and then regenerate baked data (`cargo make bakeddata components/datetime`)
    #[rustfmt::skip]
    const UNSUPPORTED_STRING_SKELETONS: &[&str] = &[
        // TODO(#487) - Flexible day periods
        "Bh", "Bhm", "Bhms", "EBhm", "EBhms",
        // TODO(#501) - Quarters
        "yQ", "yQQQ", "yQQQQ",
        // TODO(#5643) - Weeks
        "MMMMW", "yw",
    ];

    #[test]
    fn test_known_skeletons_ok() {
        for string_skeleton in SUPPORTED_STRING_SKELETONS {
            match Skeleton::try_from(*string_skeleton) {
                Ok(_) => {}
                Err(err) => {
                    panic!(
                        "Unable to parse string_skeleton {string_skeleton:?} with error, {err:?}"
                    )
                }
            }
        }
    }

    #[test]
    fn test_unsupported_skeletons_skeletons_err() {
        for string_skeleton in UNSUPPORTED_STRING_SKELETONS {
            match Skeleton::try_from(*string_skeleton) {
                Ok(_) => {
                    panic!(
                        "An unsupported field is now supported, consider moving {string_skeleton:?} to the \
                         supported skeletons, and ensure the skeleton is properly implemented."
                    )
                }
                Err(err) => match err {
                    SkeletonError::SymbolUnimplemented(_) => {
                        // Every skeleton should return this error.
                    }
                    _ => panic!("{err}"),
                },
            }
        }
    }

    #[test]
    fn test_skeleton_deserialization() {
        assert_eq!(
            Skeleton::try_from("MMMMdEEEE").unwrap(),
            Skeleton::from(vec![
                Field {
                    symbol: Month::Format.into(),
                    length: FieldLength::Four
                },
                Field {
                    symbol: Day::DayOfMonth.into(),
                    length: FieldLength::One
                },
                Field {
                    symbol: Weekday::Format.into(),
                    length: FieldLength::Four
                },
            ])
        );
    }

    #[test]
    fn test_skeleton_tuple_ordering() {
        let skeletons_strings = Vec::from([
            "y", "yM", "yMdE", "yMdEEEE", "yMMM", "M", "Md", "Mdd", "MMd", "MMdd", "d", "h", "hm",
            "hms", "Hm", "Hms", "ms", "mmss",
        ]);

        let skeleton_fields: Vec<Skeleton> = skeletons_strings
            .iter()
            .map(|skeleton_string| Skeleton::try_from(*skeleton_string).unwrap())
            .collect();

        for (strings, fields) in skeletons_strings.windows(2).zip(skeleton_fields.windows(2)) {
            if fields[0].cmp(&fields[1]) != core::cmp::Ordering::Less {
                panic!("Expected {:?} < {:?}", strings[0], strings[1]);
            }
        }
    }

    #[test]
    fn test_skeleton_json_reordering() {
        let unordered_skeleton = "EEEEyMd";
        let ordered_skeleton = "yMdEEEE";

        // Wrap the string in quotes so it's a JSON string.
        let json: String = serde_json::to_string(unordered_skeleton).unwrap();

        // Wrap the string in quotes so it's a JSON string.
        let skeleton = serde_json::from_str::<Skeleton>(&json)
            .expect("Unable to parse an unordered skeletons.");

        assert_eq!(
            serde_json::to_string(&skeleton).unwrap(),
            serde_json::to_string(ordered_skeleton).unwrap()
        );
    }

    /// This test handles a branch in the skeleton serialization code that takes into account
    /// duplicate field errors when deserializing from string.
    #[test]
    fn test_skeleton_json_duplicate_fields() {
        // Wrap the string in quotes so it's a JSON string.
        let json: String = serde_json::to_string("EEEEyMdEEEE").unwrap();
        let err =
            serde_json::from_str::<Skeleton>(&json).expect_err("Expected a duplicate field error.");

        assert_eq!(
            format!("{err}"),
            "invalid value: \"EEEEyMdEEEE\" duplicate field in skeleton, expected field symbols representing a skeleton at line 1 column 13"
        );
    }

    #[test]
    fn test_skeleton_matching_weekday_short() {
        let mut components = components::Bag::empty();
        components.weekday = Some(components::Text::Short);
        let default_hour_cycle = HourCycle::H23;
        let requested_fields = components.to_vec_fields(default_hour_cycle);
        let (_, skeletons) = get_data_payload();

        match get_best_available_format_pattern(&skeletons, &requested_fields, false) {
            BestSkeleton::AllFieldsMatch(available_format_pattern, _) => {
                assert_eq!(
                    available_format_pattern
                        .expect_pattern("pattern should not have plural variants")
                        .to_string()
                        .as_str(),
                    // Requesting E, CLDR has ccc, should not be shortened to c
                    "ccc"
                )
            }
            best => panic!("Unexpected {best:?}"),
        };
    }

    #[test]
    fn test_skeleton_matching_weekday_long() {
        let mut components = components::Bag::empty();
        components.weekday = Some(components::Text::Long);
        let default_hour_cycle = HourCycle::H23;
        let requested_fields = components.to_vec_fields(default_hour_cycle);
        let (_, skeletons) = get_data_payload();

        match get_best_available_format_pattern(&skeletons, &requested_fields, false) {
            BestSkeleton::AllFieldsMatch(available_format_pattern, _) => {
                assert_eq!(
                    available_format_pattern
                        .expect_pattern("pattern should not have plural variants")
                        .to_string()
                        .as_str(),
                    // Requesting EEEE, CLDR has ccc, should be lengthened to cccc
                    "cccc"
                )
            }
            best => panic!("Unexpected {best:?}"),
        };
    }

    fn assert_pattern_to_skeleton(pattern: &str, skeleton: &str, message: &str) {
        assert_eq!(
            serde_json::to_string(skeleton).expect("Failed to transform skeleton to string."),
            serde_json::to_string(&Skeleton::from(
                &pattern
                    .parse::<reference::Pattern>()
                    .expect("Failed to create pattern from bytes.")
            ))
            .expect("Failed to transform skeleton to string."),
            "{message}"
        );
    }

    #[test]
    fn test_pattern_to_skeleton() {
        assert_pattern_to_skeleton("H:mm:ss v", "Hmmssv", "Test a complicated time pattern");
        assert_pattern_to_skeleton(
            "v ss:mm:H",
            "Hmmssv",
            "Test the skeleton ordering is consistent",
        );

        assert_pattern_to_skeleton("K:mm", "hmm", "H11 maps to H12");

        assert_pattern_to_skeleton("ha mm", "hmm", "Day periods get removed");
        assert_pattern_to_skeleton("h 'at' b mm", "hmm", "Day periods get removed");

        assert_pattern_to_skeleton("y", "y", "The year is passed through");
        assert_pattern_to_skeleton("U", "U", "The year is passed through");

        assert_pattern_to_skeleton("LLL", "MMM", "Remove standalone months.");

        assert_pattern_to_skeleton("s", "s", "Seconds pass through");
        assert_pattern_to_skeleton("A", "A", "Seconds pass through");

        assert_pattern_to_skeleton("z", "z", "Time zones get passed through");
        assert_pattern_to_skeleton("O", "O", "Time zones get passed through");
        assert_pattern_to_skeleton("v", "v", "Time zones get passed through");
        assert_pattern_to_skeleton("V", "V", "Time zones get passed through");
        assert_pattern_to_skeleton("X", "X", "Time zones get passed through");
        assert_pattern_to_skeleton("x", "x", "Time zones get passed through");

        assert_pattern_to_skeleton("Z", "xxxx", "Z gets resolved");
    }
}
