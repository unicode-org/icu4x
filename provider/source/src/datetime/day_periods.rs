// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::datetime::provider::day_periods::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

impl DataProvider<DayPeriodRulesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesV1>, DataError> {
        self.check_req::<DayPeriodRulesV1>(req)?;

        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;

        let langid = icu::locale::LanguageIdentifier::from((
            req.id.locale.language,
            req.id.locale.script,
            req.id.locale.region,
        ));

        let rules = day_periods
            .supplemental
            .day_period_rule_set
            .0
            .get(&langid.to_string())
            .ok_or_else(|| {
                DataErrorKind::IdentifierNotFound
                    .with_req(<DayPeriodRulesV1 as DataMarker>::INFO, req)
            })?;

        let data = compute_day_periods(rules, &req.id.locale.to_string())?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

/// Computes `DayPeriodRules` from CLDR supplemental day period rules.
///
/// Returns `None` if the rules are empty or do not contain any flexible day periods.
pub(crate) fn compute_day_periods(
    rules: &std::collections::BTreeMap<String, cldr_serde::day_periods::DayPeriodRule>,
    locale_str: &str,
) -> Result<DayPeriodRules, DataError> {
    let mut entries = std::collections::BTreeMap::new();

    for (period, rule) in rules {
        if rule.at.is_some() {
            assert!(
                period == "noon" || period == "midnight",
                "Found 'at' rule for non-noon/midnight period: {} in locale {}",
                period,
                locale_str
            );
        }
        if let Some(period_enum) = DayPeriod::from_cldr_name(period) {
            if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                let start = parse_hour(from);
                let end = parse_hour(before);
                entries.insert((start, end), period_enum);
            } else {
                log::warn!(
                    "Did not have from/before values for rule {period} in locale {locale_str}"
                )
            }
        } else if period != "morning" && period != "afternoon" {
            log::warn!("Unknown range period found {period} in locale {locale_str}");
        }
    }

    DayPeriodRules::from_periods(&entries)
        .map_err(|e| DataError::custom("rules").with_display_context(e))
}

impl IterableDataProviderCached<DayPeriodRulesV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rule_set
            .0
            .iter()
            .filter_map(|(l, rules)| {
                let langid: icu::locale::LanguageIdentifier = l.parse().unwrap();
                if compute_day_periods(rules, l).is_ok() {
                    Some(DataIdentifierCow::from_locale(DataLocale::from(langid)))
                } else {
                    None
                }
            })
            .collect())
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cldr_serde::day_periods::DayPeriodRule;
    use std::collections::BTreeMap;

    #[test]
    fn test_compute_day_periods() {
        let mut rules = BTreeMap::new();

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
        rules.insert(
            "afternoon1".to_string(),
            DayPeriodRule {
                from: Some("12:00".to_string()),
                before: Some("18:00".to_string()),
                at: None,
            },
        );
        rules.insert(
            "evening1".to_string(),
            DayPeriodRule {
                from: Some("18:00".to_string()),
                before: Some("21:00".to_string()),
                at: None,
            },
        );
        rules.insert(
            "night1".to_string(),
            DayPeriodRule {
                from: Some("21:00".to_string()),
                before: Some("06:00".to_string()),
                at: None,
            },
        );

        let rules = compute_day_periods(&rules, "test").unwrap();

        assert_eq!(
            rules,
            DayPeriodRules::from_periods(
                &[
                    ((6, 12), DayPeriod::Morning1),
                    ((12, 18), DayPeriod::Afternoon1),
                    ((18, 21), DayPeriod::Evening1),
                    ((21, 6), DayPeriod::Night1),
                ]
                .into_iter()
                .collect()
            )
            .unwrap()
        )
    }
}
