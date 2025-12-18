// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Data for calendar arithmetic
pub(crate) mod eras;

#[test]
fn test_calendar_resolution() {
    use std::collections::BTreeMap;

    use icu::{
        calendar::preferences::{CalendarAlgorithm, CalendarPreferences},
        locale::{
            extensions::unicode::Value,
            subtags::{Language, Region},
            LanguageIdentifier,
        },
    };

    #[derive(serde::Deserialize)]
    struct Resource {
        supplemental: Supplemental,
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Supplemental {
        calendar_preference_data: BTreeMap<Region, Vec<String>>,
    }

    for (&region, algorithms) in &crate::SourceDataProvider::new_testing()
        .cldr()
        .unwrap()
        .core()
        .read_and_parse::<Resource>("supplemental/calendarPreferenceData.json")
        .unwrap()
        .supplemental
        .calendar_preference_data
    {
        let mut region_preferences = CalendarPreferences::from(&LanguageIdentifier::from((
            Language::UNKNOWN,
            None,
            Some(region),
        )));

        let algorithms = algorithms
            .iter()
            .map(|a| match a.as_str() {
                "gregorian" => CalendarAlgorithm::Gregory,
                a => CalendarAlgorithm::try_from(&Value::try_from_str(a).unwrap()).unwrap(),
            })
            .collect::<Vec<_>>();

        assert_eq!(
            region_preferences.resolved_algorithm(),
            algorithms[0],
            "{region:?}",
        );

        if let Some(&preferred_islamic_algorithm) = algorithms
            .iter()
            .find(|&&a| matches!(a, CalendarAlgorithm::Hijri(Some(_))))
        {
            region_preferences.calendar_algorithm = Some(CalendarAlgorithm::Hijri(None));
            assert_eq!(
                region_preferences.resolved_algorithm(),
                preferred_islamic_algorithm,
                "{region:?}",
            );
        }
    }
}
