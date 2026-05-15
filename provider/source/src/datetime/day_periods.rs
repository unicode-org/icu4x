// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use icu::datetime::provider::day_periods::*;
use icu_provider::prelude::*;
use std::borrow::Cow;

/// Computes `DayPeriodRules` from CLDR supplemental day period rules.
///
/// Returns `None` if the rules are empty or do not contain any flexible day periods.
pub(crate) fn compute_day_periods<'a>(
    rules: &std::collections::BTreeMap<String, cldr_serde::day_periods::DayPeriodRule>,
    names: &'a std::collections::BTreeMap<String, Cow<'a, str>>,
    locale: DataLocale,
) -> Result<(DayPeriodRules, impl Iterator<Item = &'a str>), DataError> {
    /// Parses a "HH:MM" time string and returns the hour as a u8.
    /// Logs a warning if the minute value is non-zero, as precision will be lost.
    fn parse_hour(s: &str) -> u8 {
        let mut parts = s.split(':');
        let hour = parts.next().unwrap().parse().unwrap();
        if let Some(min_str) = parts.next() {
            let min: u32 = min_str.parse().unwrap();
            if min != 0 {
                log::warn!(
                    "Non-zero minute found in day period time: {}, precision will be lost",
                    s
                );
            }
        }
        hour
    }

    let mut entries = std::collections::BTreeMap::new();

    for (period, rule) in rules {
        if rule.at.is_some() {
            assert!(
                period == "noon" || period == "midnight",
                "Found 'at' rule for non-noon/midnight period: {} in locale {}",
                period,
                locale
            );
        }
        if let Some(name) = names.get(period) {
            if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                let start = parse_hour(from);
                let end = parse_hour(before);
                entries.insert((start, end), &**name);
            } else {
                log::warn!("Did not have from/before values for rule {period} in locale {locale}")
            }
        } else if period != "morning" && period != "afternoon" {
            log::warn!("missing name for range {period} in locale {locale}");
        }
    }

    DayPeriodRules::from_periods(entries).map_err(|e| {
        DataError::custom("day period rules")
            .with_debug_context(&locale)
            .with_display_context(e)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cldr_serde::day_periods::DayPeriodRule;
    use std::collections::BTreeMap;

    #[test]
    fn test_compute_day_periods() {
        let mut rules = BTreeMap::new();
        let mut names = BTreeMap::new();

        // Simulate rules where night extends into morning:
        // morning1: 06:00 - 12:00
        // afternoon1: 12:00 - 18:00
        // evening1: 18:00 - 21:00
        // night1: 21:00 - 06:00

        rules.insert(
            "morning1".to_string(),
            DayPeriodRule {
                from: Some("06:00".to_string()),
                before: Some("12:00".to_string()),
                at: None,
            },
        );
        names.insert(String::from("morning1"), Cow::Borrowed("foo"));
        rules.insert(
            "afternoon1".to_string(),
            DayPeriodRule {
                from: Some("12:00".to_string()),
                before: Some("18:00".to_string()),
                at: None,
            },
        );
        names.insert(String::from("afternoon1"), Cow::Borrowed("bar"));
        rules.insert(
            "evening1".to_string(),
            DayPeriodRule {
                from: Some("18:00".to_string()),
                before: Some("21:00".to_string()),
                at: None,
            },
        );
        names.insert(String::from("evening1"), Cow::Borrowed("baz"));
        rules.insert(
            "night1".to_string(),
            DayPeriodRule {
                from: Some("21:00".to_string()),
                before: Some("06:00".to_string()),
                at: None,
            },
        );
        names.insert(String::from("night1"), Cow::Borrowed("qux"));

        let actual = compute_day_periods(&rules, &names, Default::default()).unwrap();

        let expected = DayPeriodRules::from_periods(
            [
                ((6, 12), "foo"),
                ((12, 18), "bar"),
                ((18, 21), "baz"),
                ((21, 6), "qux"),
            ]
            .into_iter()
            .collect(),
        )
        .unwrap();

        assert_eq!(actual.0, expected.0);

        assert_eq!(actual.1.collect::<Vec<_>>(), expected.1.collect::<Vec<_>>());
    }
}
