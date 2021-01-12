// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod patterns;

use super::{format_number, write_pattern};
use crate::{date::MockDateTime, fields::FieldLength, pattern::Pattern, provider::{gregory::DatesV1, key::GREGORY_V1}};
use icu_locid::LanguageIdentifier;
use icu_provider::{
    DataProvider, DataRequest, ResourceOptions, ResourcePath,
};
use std::borrow::Cow;

#[test]
fn test_format_number() {
    let values = &[2, 20, 201, 2017, 20173];
    let samples = &[
        (FieldLength::One, ["2", "20", "201", "2017", "20173"]),
        (FieldLength::TwoDigit, ["02", "20", "01", "17", "73"]),
        (
            FieldLength::Abbreviated,
            ["002", "020", "201", "2017", "20173"],
        ),
        (FieldLength::Wide, ["0002", "0020", "0201", "2017", "20173"]),
    ];
    for (length, expected) in samples {
        for (value, expected) in values.iter().zip(expected) {
            let mut s = String::new();
            format_number(&mut s, *value, *length).unwrap();
            assert_eq!(s, *expected);
        }
    }
}

#[test]
fn test_format_dayperiods() {
    use patterns::structs::Expectation;
    let provider = icu_testdata::get_provider();
    for test in patterns::get_tests("dayperiods").unwrap().0 {
        let langid: LanguageIdentifier = test.locale.parse().unwrap();
        let data: Cow<DatesV1> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: GREGORY_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid.clone()),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        for test_case in &test.test_cases {
            for dt_input in &test_case.date_times {
                let date_time: MockDateTime = dt_input.parse().unwrap();
                for Expectation { patterns, expected } in &test_case.expectations {
                    for pattern_input in patterns {
                        let pattern = Pattern::from_bytes(pattern_input).unwrap();
                        let mut actual = String::with_capacity(expected.len());
                        write_pattern(&pattern, &data, &date_time, &mut actual).unwrap();
                        assert_eq!(
                            actual, *expected,
                            "\n\
                            locale:   `{}`,\n\
                            datetime: `{}`,\n\
                            pattern:  `{}`",
                            langid, dt_input, pattern_input,
                        );
                    }
                }
            }
        }
    }
}
