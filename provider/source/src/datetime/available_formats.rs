// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use icu::datetime::provider::pattern::runtime::Pattern;
use icu::datetime::provider::skeleton::reference::Skeleton;
use icu::plurals::{PluralCategory, PluralElements};

use crate::AltVariantKind;
use crate::SourceDataProvider;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum DatetimeAsciiPreference {
    PreferAscii,
    Default,
}

impl SourceDataProvider {
    pub(crate) fn datetime_ascii_preference(&self) -> DatetimeAsciiPreference {
        if self.alt_variants.contains(&AltVariantKind::DatetimeAscii) {
            DatetimeAsciiPreference::PreferAscii
        } else {
            DatetimeAsciiPreference::Default
        }
    }
}

impl cldr_serde::ca::AvailableFormats {
    pub fn parse_skeletons(
        &self,
        datetime_ascii_preference: DatetimeAsciiPreference,
    ) -> BTreeMap<Skeleton, PluralElements<Pattern<'static>>> {
        let mut patterns: BTreeMap<String, BTreeMap<PluralCategory, String>> = BTreeMap::new();

        // The CLDR keys for available_formats can have duplicate skeletons with either
        // an additional variant, or with multiple variants for different plurals.
        for (skeleton_str, pattern_str) in self.0.iter() {
            let (skeleton_str, is_alt_ascii) = match skeleton_str.strip_suffix("-alt-ascii") {
                Some(stripped) => (stripped, true),
                None => (skeleton_str.as_str(), false),
            };
            if datetime_ascii_preference == DatetimeAsciiPreference::Default && is_alt_ascii {
                continue;
            }

            let (skeleton, plural_category) = match skeleton_str.split_once("-count-") {
                Some((s, v)) => (s, PluralCategory::get_for_cldr_string(v).unwrap()),
                None => (skeleton_str, PluralCategory::Other),
            };

            patterns
                .entry(skeleton.to_string())
                .or_default()
                .insert(plural_category, pattern_str.to_string());
        }

        // TODO(#308): Support numbering system variations. We currently throw them away.
        super::parse_cldr_skeletons(&patterns, |_skeleton, patterns| {
            let patterns = PluralElements::new(&patterns[&PluralCategory::Other])
                .with_zero_value(patterns.get(&PluralCategory::Zero))
                .with_one_value(patterns.get(&PluralCategory::One))
                .with_two_value(patterns.get(&PluralCategory::Two))
                .with_few_value(patterns.get(&PluralCategory::Few))
                .with_many_value(patterns.get(&PluralCategory::Many))
                .map(|s| s.parse().expect(s));
            Some(patterns)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use icu::datetime::provider::skeleton::{
        BestSkeleton, SkeletonError, get_best_available_format_pattern,
    };

    use core::convert::TryFrom;
    use core::str::FromStr;
    use icu::datetime::provider::fields::components;
    use icu::datetime::provider::skeleton::reference::Skeleton;
    use icu::datetime::{
        provider::fields::{Day, Field, FieldLength, Month, Weekday},
        provider::pattern::reference,
    };
    use icu::locale::locale;
    use icu::locale::preferences::extensions::unicode::keywords::HourCycle;
    use std::collections::BTreeMap;

    use crate::SourceDataProvider;
    use crate::datetime::DatagenCalendar;

    fn get_data_payload() -> BTreeMap<Skeleton, PluralElements<Pattern<'static>>> {
        let locale = locale!("en").into();

        let provider = SourceDataProvider::new_testing();
        let data = provider
            .get_dates_resource(&locale, Some(DatagenCalendar::Gregorian))
            .unwrap();
        data.datetime_formats
            .available_formats
            .parse_skeletons(DatetimeAsciiPreference::Default)
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
        let skeletons = get_data_payload();

        match get_best_available_format_pattern(&skeletons, &requested_fields, false) {
            BestSkeleton::AllFieldsMatch(available_format_pattern, _)
            | BestSkeleton::MissingOrExtraFields(available_format_pattern, _) => {
                assert_eq!(
                    available_format_pattern
                        .try_into_other()
                        .expect("pattern should not have plural variants")
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
        let skeletons = get_data_payload();

        match get_best_available_format_pattern(&skeletons, &requested_fields, false) {
            BestSkeleton::MissingOrExtraFields(available_format_pattern, _) => {
                assert_eq!(
                    available_format_pattern
                        .try_into_other()
                        .expect("pattern should not have plural variants")
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

    #[test]
    fn test_skeleton_empty_bag() {
        let components: components::Bag = Default::default();
        let requested_fields = components.to_vec_fields(HourCycle::H23);
        let skeletons = get_data_payload();

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
        let mut skeletons = BTreeMap::new();
        skeletons.insert(
            Skeleton::try_from("EEEE").unwrap(),
            PluralElements::new(Pattern::from_str("weekday EEEE").unwrap()),
        );

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
        // Flexible day periods
        "Bh", "Bhm", "Bhms", "EBhm", "EBhms",
    ];

    // NOTE: If you are moving this to the SUPPORTED section, make sure to remove the match
    //       on your symbol from impl From<fields::SymbolError> for SkeletonError
    //       and then regenerate data.
    #[rustfmt::skip]
    const UNSUPPORTED_STRING_SKELETONS: &[&str] = &[
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
        let skeletons = get_data_payload();

        match get_best_available_format_pattern(&skeletons, &requested_fields, false) {
            BestSkeleton::AllFieldsMatch(available_format_pattern, _) => {
                assert_eq!(
                    available_format_pattern
                        .try_into_other()
                        .expect("pattern should not have plural variants")
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
        let skeletons = get_data_payload();

        match get_best_available_format_pattern(&skeletons, &requested_fields, false) {
            BestSkeleton::AllFieldsMatch(available_format_pattern, _) => {
                assert_eq!(
                    available_format_pattern
                        .try_into_other()
                        .expect("pattern should not have plural variants")
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

    #[test]
    fn test_alt_ascii_parsing() {
        let locale = locale!("en").into();
        let provider = SourceDataProvider::new_testing();
        let data = provider
            .get_dates_resource(&locale, Some(DatagenCalendar::Gregorian))
            .unwrap();

        let skeletons_no_alt = data
            .datetime_formats
            .available_formats
            .parse_skeletons(DatetimeAsciiPreference::Default);
        let skeletons_alt = data
            .datetime_formats
            .available_formats
            .parse_skeletons(DatetimeAsciiPreference::PreferAscii);

        let h_skeleton = Skeleton::try_from("h").unwrap();

        let pattern_no_alt = skeletons_no_alt
            .get(&h_skeleton)
            .unwrap()
            .clone()
            .try_into_other()
            .unwrap()
            .to_string();
        let pattern_alt = skeletons_alt
            .get(&h_skeleton)
            .unwrap()
            .clone()
            .try_into_other()
            .unwrap()
            .to_string();

        assert!(
            pattern_alt.is_ascii(),
            "Alt-ASCII pattern should be all ASCII"
        );
        assert_ne!(
            pattern_alt, pattern_no_alt,
            "Alt-ASCII had an impact on the result"
        );
    }
}
