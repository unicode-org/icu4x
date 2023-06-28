// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Skeletons are used for pattern matching. See the [`Skeleton`](reference::Skeleton) struct for more information.

mod error;
mod helpers;
pub mod reference;
pub mod runtime;
#[cfg(feature = "serde")]
mod serde;
pub use error::*;
pub use helpers::*;

#[cfg(all(test, feature = "datagen", feature = "experimental"))]
mod test {
    use super::reference::Skeleton;
    use super::*;
    use icu_locid::Locale;
    use icu_provider::prelude::*;

    use crate::{
        fields::{Day, Field, FieldLength, Month, Weekday},
        options::components,
        pattern::runtime,
        provider::calendar::{
            DateSkeletonPatternsV1, DateSkeletonPatternsV1Marker, GregorianDateLengthsV1Marker,
            SkeletonV1,
        },
    };
    use core::convert::TryFrom;
    use core::str::FromStr;
    use litemap::LiteMap;

    use ::serde::{
        ser::{self, SerializeSeq},
        Serialize,
    };

    fn get_data_payload() -> (
        DataPayload<GregorianDateLengthsV1Marker>,
        DataPayload<DateSkeletonPatternsV1Marker>,
    ) {
        let locale = "en-u-ca-gregory".parse::<Locale>().unwrap().into();
        let req = DataRequest {
            locale: &locale,
            metadata: Default::default(),
        };
        let patterns = icu_testdata::buffer()
            .as_deserializing()
            .load(req)
            .expect("Failed to load payload")
            .take_payload()
            .expect("Failed to retrieve payload");
        let skeletons = icu_testdata::buffer()
            .as_deserializing()
            .load(req)
            .expect("Failed to load payload")
            .take_payload()
            .expect("Failed to retrieve payload");
        (patterns, skeletons)
    }

    /// This is an initial smoke test to verify the skeleton machinery is working. For more in-depth
    /// testing see components/datetime/tests/fixtures/tests/components-*.json
    #[test]
    fn test_skeleton_matching() {
        let components = components::Bag {
            year: Some(components::Year::Numeric),
            month: Some(components::Month::Long),
            day: Some(components::Day::NumericDayOfMonth),

            hour: Some(components::Numeric::Numeric),
            minute: Some(components::Numeric::Numeric),
            second: Some(components::Numeric::Numeric),

            ..Default::default()
        };
        let requested_fields = components.to_vec_fields();
        let (_, skeletons) = get_data_payload();

        match get_best_available_format_pattern(skeletons.get(), &requested_fields, false) {
            BestSkeleton::AllFieldsMatch(available_format_pattern)
            | BestSkeleton::MissingOrExtraFields(available_format_pattern) => {
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
        let components = components::Bag {
            week: Some(components::Week::TwoDigitWeekOfYear),
            weekday: Some(components::Text::Short),
            ..Default::default()
        };
        let requested_fields = components.to_vec_fields();
        let (_, skeletons) = get_data_payload();

        match get_best_available_format_pattern(skeletons.get(), &requested_fields, false) {
            BestSkeleton::MissingOrExtraFields(available_format_pattern) => {
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
        let components = components::Bag {
            year: Some(components::Year::Numeric),
            month: Some(components::Month::Long),
            day: Some(components::Day::NumericDayOfMonth),
            // This will be appended.
            time_zone_name: Some(components::TimeZoneName::LongSpecific),
            ..Default::default()
        };
        let requested_fields = components.to_vec_fields();
        let (patterns, skeletons) = get_data_payload();

        match create_best_pattern_for_fields(
            skeletons.get(),
            &patterns.get().length_combinations,
            &requested_fields,
            &Default::default(),
            false,
        ) {
            BestSkeleton::AllFieldsMatch(available_format_pattern) => {
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
        let requested_fields = components.to_vec_fields();
        let (_, skeletons) = get_data_payload();

        assert_eq!(
            get_best_available_format_pattern(skeletons.get(), &requested_fields, false),
            BestSkeleton::NoMatch,
            "No match was found"
        );
    }

    #[test]
    fn test_skeleton_no_match() {
        let components = components::Bag {
            hour: Some(components::Numeric::Numeric),
            time_zone_name: Some(components::TimeZoneName::LongSpecific),
            ..Default::default()
        };
        let requested_fields = components.to_vec_fields();
        // Construct a set of skeletons that do not use the hour nor time zone symbols.
        let mut skeletons = LiteMap::new();
        skeletons.insert(
            SkeletonV1::try_from("EEEE").unwrap(),
            runtime::Pattern::from_str("weekday EEEE").unwrap().into(),
        );
        let skeletons = DateSkeletonPatternsV1(skeletons);

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
        "MdEEEE", "MdE", "MMM", "MMMdEEEE", "MMMdE", "MMMM", "MMMMW",
        "MMMMdEEEE", "MMMMdE", "MMMMd",
        "MMMMdd", "MMMd", "MMMdd", "MMd", "MMdd", "Md", "Mdd", "d", "h", "hm", "hms", "mmss", "ms",
        "y", "yM", "yMdEEEE", "yMdE", "yMM", "yMMM", "yMMMdEEEE", "yMMMdE", "yMMMM", "yMMMMdEEEE",
        "yMMMMdE", "yMMMMdcccc", "yMMMMd", "yMMMd", "yMMdd", "yMd", "yw",
        "Gy", "GyM", "GyMMM", "GyMMMdEEEE", "GyMMMdE", "GyMMMM", "GyMMMMdE", "GyMMMMd", "GyMMMd",
        // Time zones
        "HHmmZ", "Hmsv", "Hmsvvvv", "Hmv", "Hmvvvv", "hmsv", "hmsvvvv", "hmv", "hmvvvv",
    ];

    // NOTE: If you are moving this to the SUPPORTED section, make sure to remove the match
    //       on your symbol from impl From<fields::SymbolError> for SkeletonError
    //       and then regenerate the test data.
    //       https://github.com/unicode-org/icu4x/blob/main/provider/testdata/README.md
    #[rustfmt::skip]
    const UNSUPPORTED_STRING_SKELETONS: &[&str] = &[
        // TODO(#487) - Flexible day periods
        "Bh", "Bhm", "Bhms", "EBhm", "EBhms",
        // TODO(#501) - Quarters
        "yQ", "yQQQ", "yQQQQ",
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
                    length: FieldLength::Wide
                },
                Field {
                    symbol: Day::DayOfMonth.into(),
                    length: FieldLength::One
                },
                Field {
                    symbol: Weekday::Format.into(),
                    length: FieldLength::Wide
                },
            ])
        );
    }

    #[test]
    fn test_skeleton_tuple_ordering() {
        let skeletons_strings = Vec::from([
            "y", "yM", "yMdE", "yMdEEEE", "yMMM", "yw", "M", "Md", "Mdd", "MMd", "MMdd", "d", "h",
            "hm", "hms", "Hm", "Hms", "ms", "mmss",
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

    /// Skeletons are represented in bincode as a vec of field, but bincode shouldn't be completely
    /// trusted, test that the bincode gets validated correctly.
    struct TestInvalidSkeleton(Vec<Field>);

    #[cfg(feature = "serde")]
    impl Serialize for TestInvalidSkeleton {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            let fields = &self.0;
            let mut seq = serializer.serialize_seq(Some(fields.len()))?;
            for item in fields.iter() {
                seq.serialize_element(item)?;
            }
            seq.end()
        }
    }

    #[cfg(feature = "datagen")]
    fn assert_pattern_to_skeleton(pattern: &str, skeleton: &str, message: &str) {
        assert_eq!(
            serde_json::to_string(skeleton).expect("Failed to transform skeleton to string."),
            serde_json::to_string(&Skeleton::from(
                &pattern
                    .parse::<crate::pattern::reference::Pattern>()
                    .expect("Failed to create pattern from bytes.")
            ))
            .expect("Failed to transform skeleton to string."),
            "{message}"
        );
    }

    #[test]
    #[cfg(feature = "datagen")]
    fn test_pattern_to_skeleton() {
        assert_pattern_to_skeleton("H:mm:ss v", "Hmmssv", "Test a complicated time pattern");
        assert_pattern_to_skeleton(
            "v ss:mm:H",
            "Hmmssv",
            "Test the skeleton ordering is consistent",
        );

        assert_pattern_to_skeleton("K:mm", "hmm", "H11 maps to H12");
        assert_pattern_to_skeleton("k:mm", "Hmm", "H23 maps to H24");

        assert_pattern_to_skeleton("ha mm", "hmm", "Day periods get removed");
        assert_pattern_to_skeleton("h 'at' b mm", "hmm", "Day periods get removed");

        assert_pattern_to_skeleton("y", "y", "The year is passed through");
        assert_pattern_to_skeleton("Y", "Y", "The year is passed through");

        assert_pattern_to_skeleton("LLL", "MMM", "Remove standalone months.");

        assert_pattern_to_skeleton("s", "s", "Seconds pass through");
        assert_pattern_to_skeleton("S", "S", "Seconds pass through");
        assert_pattern_to_skeleton("A", "A", "Seconds pass through");

        assert_pattern_to_skeleton("z", "z", "Time zones get passed through");
        assert_pattern_to_skeleton("Z", "Z", "Time zones get passed through");
        assert_pattern_to_skeleton("O", "O", "Time zones get passed through");
        assert_pattern_to_skeleton("v", "v", "Time zones get passed through");
        assert_pattern_to_skeleton("V", "V", "Time zones get passed through");
        assert_pattern_to_skeleton("X", "X", "Time zones get passed through");
        assert_pattern_to_skeleton("x", "x", "Time zones get passed through");
    }
}
