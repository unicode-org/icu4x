// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "serde")]

mod fixtures;
mod patterns;

use icu_datetime::{mock::MockDateTime, DateTimeFormatOptions};
use icu_datetime::{
    provider::{gregory::DatesV1, key::GREGORY_V1},
    DateTimeFormat,
};
use icu_locid::{LanguageIdentifier, Locale};
use icu_provider::{
    struct_provider::StructProvider, DataProvider, DataRequest, ResourceOptions, ResourcePath,
};
use std::{borrow::Cow, fmt::Write};

fn test_fixture(fixture_name: &str) {
    let provider = icu_testdata::get_provider();

    for fx in fixtures::get_fixture(fixture_name)
        .expect("Unable to get fixture.")
        .0
    {
        let locale: Locale = fx.input.locale.parse().unwrap();
        let options = fixtures::get_options(&fx.input.options);
        let dtf = DateTimeFormat::try_new(locale, &provider, &options).unwrap();

        let value: MockDateTime = fx.input.value.parse().unwrap();

        let result = dtf.format_to_string(&value);
        match fx.description {
            Some(description) => assert_eq!(
                result, fx.output.value,
                "expected {:?} to equal {:?} – {}",
                result, fx.output.value, description
            ),
            None => assert_eq!(result, fx.output.value),
        }

        let mut s = String::new();
        dtf.format_to_write(&mut s, &value).unwrap();
        assert_eq!(s, fx.output.value);

        let fdt = dtf.format(&value);
        assert_eq!(fdt.to_string(), fx.output.value);

        let mut s = String::new();
        write!(s, "{}", fdt).unwrap();
        assert_eq!(s, fx.output.value);
    }
}

#[test]
fn test_dayperiod_patterns() {
    use patterns::structs::Expectation;
    let provider = icu_testdata::get_provider();
    let format_options = DateTimeFormatOptions::default();
    for test in patterns::get_tests("dayperiods").unwrap().0 {
        let langid: LanguageIdentifier = test.locale.parse().unwrap();
        let mut data: Cow<DatesV1> = provider
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
            .payload
            .take()
            .unwrap();
        *data
            .to_mut()
            .patterns
            .date_time
            .length_patterns
            .long
            .to_mut() = String::from("{0}");
        for test_case in &test.test_cases {
            for dt_input in &test_case.date_times {
                let date_time: MockDateTime = dt_input.parse().unwrap();
                for Expectation { patterns, expected } in &test_case.expectations {
                    for pattern_input in patterns {
                        *data.to_mut().patterns.time.long.to_mut() = String::from(pattern_input);
                        let provider = StructProvider {
                            key: GREGORY_V1,
                            data: data.as_ref(),
                        };
                        let dtf =
                            DateTimeFormat::try_new(langid.clone(), &provider, &format_options)
                                .unwrap();
                        assert_eq!(
                            dtf.format(&date_time).to_string(),
                            *expected,
                            "\n\
                            locale:   `{}`,\n\
                            datetime: `{}`,\n\
                            pattern:  `{}`",
                            langid,
                            dt_input,
                            pattern_input,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn test_length_fixtures() {
    // components/datetime/tests/fixtures/tests/lengths.json
    test_fixture("lengths");
}

/// Tests component::Bag configurations that have exact matches to CLDR skeletons.
#[test]
fn test_components_exact_matches() {
    // components/datetime/tests/fixtures/tests/components-exact-matches.json
    test_fixture("components-exact-matches");
}

/// Tests that component::Bags can adjust for width differences in the final pattern.
/// TODO(584) - This is unimplemented and will panic.
#[test]
#[should_panic]
fn test_components_width_differences() {
    // components/datetime/tests/fixtures/tests/components-exact-matches.json
    test_fixture("components-width-differences");
}

/// Tests that component::Bags can combine a date skeleton, and a time skeleton.
#[test]
fn test_components_combine_date_time() {
    // components/datetime/tests/fixtures/tests/components-combine-date-time.json
    test_fixture("components-combine-date-time");
}
