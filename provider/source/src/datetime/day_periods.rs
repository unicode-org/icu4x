// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::datetime::provider::day_periods::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

fn parse_hour(s: &str) -> u8 {
    s.split(':').next().unwrap().parse().unwrap()
}

fn period_idx(name: &str) -> Option<u8> {
    match name {
        "morning1" => Some(0),
        "morning2" => Some(1),
        "afternoon1" => Some(2),
        "afternoon2" => Some(3),
        "evening1" => Some(4),
        "evening2" => Some(5),
        "night1" => Some(6),
        "night2" => Some(7),
        _ => None,
    }
}

impl DataProvider<DayPeriodRulesDesign1V1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesDesign1V1>, DataError> {
        self.check_req::<DayPeriodRulesDesign1V1>(req)?;
        
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
            .day_period_rules
            .0
            .get(&langid)
            .or_else(|| {
                let mut minimized = langid.clone();
                minimized.script = None;
                minimized.region = None;
                day_periods.supplemental.day_period_rules.0.get(&minimized)
            });

        let mut entries = Vec::new();
        if let Some(rules) = rules {
            for (period, rule) in rules {
                if let Some(idx) = period_idx(period) {
                    if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                        let start = parse_hour(from);
                        let end = parse_hour(before);
                        let val = ((idx as u16) << 10) | ((start as u16) << 5) | (end as u16);
                        entries.push(DayPeriodRule1(val));
                    }
                }
            }
            // Sort by period index to be consistent
            entries.sort_by_key(|e| e.0 >> 10);
        }

        let data = DayPeriodRulesV1Design1 {
            entries: entries.into_iter().collect(),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<DayPeriodRulesDesign1V1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rules
            .0
            .keys()
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l.clone())))
            .collect())
    }
}

impl DataProvider<DayPeriodRulesDesign2V1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesDesign2V1>, DataError> {
        self.check_req::<DayPeriodRulesDesign2V1>(req)?;
        
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
            .day_period_rules
            .0
            .get(&langid)
            .or_else(|| {
                let mut minimized = langid.clone();
                minimized.script = None;
                minimized.region = None;
                day_periods.supplemental.day_period_rules.0.get(&minimized)
            });

        let mut entries = Vec::new();
        let mut presence = 0u8;
        if let Some(rules) = rules {
            for (period, rule) in rules {
                if let Some(idx) = period_idx(period) {
                    if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                        let start = parse_hour(from);
                        let end = parse_hour(before);
                        let delta = if end >= start { end - start } else { end + 24 - start };
                        let val = (start << 3) | (delta - 1); // assuming delta >= 1
                        entries.push((idx, DayPeriodRule2(val)));
                        presence |= 1 << idx;
                    }
                }
            }
            // Sort by period index to match presence bitmask
            entries.sort_by_key(|e| e.0);
        }
        let entries = entries.into_iter().map(|e| e.1).collect::<Vec<_>>();

        let data = DayPeriodRulesV1Design2 {
            presence,
            entries: entries.into_iter().collect(),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<DayPeriodRulesDesign2V1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rules
            .0
            .keys()
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l.clone())))
            .collect())
    }
}

impl DataProvider<DayPeriodRulesDesign3V1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesDesign3V1>, DataError> {
        self.check_req::<DayPeriodRulesDesign3V1>(req)?;
        
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
            .day_period_rules
            .0
            .get(&langid)
            .or_else(|| {
                let mut minimized = langid.clone();
                minimized.script = None;
                minimized.region = None;
                day_periods.supplemental.day_period_rules.0.get(&minimized)
            });

        let mut entries = Vec::new();
        let mut presence = 0u8;
        if let Some(rules) = rules {
            for (period, rule) in rules {
                if let Some(idx) = period_idx(period) {
                    if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                        let start = parse_hour(from);
                        let end = parse_hour(before);
                        entries.push((idx, start, end));
                        presence |= 1 << idx;
                    }
                }
            }
            // Sort by start time to construct diffs
            entries.sort_by_key(|e| e.1);
        }

        let mut diffs = Vec::new();
        let mut start_time = 0;
        if let Some(first) = entries.first() {
            start_time = first.1;
            let mut last_start = start_time;
            for entry in entries.iter().skip(1) {
                let diff = if entry.1 >= last_start { entry.1 - last_start } else { entry.1 + 24 - last_start };
                diffs.push(diff);
                last_start = entry.1;
            }
        }

        let data = DayPeriodRulesV1Design3 {
            presence,
            start_time,
            diffs: diffs.into_iter().collect(),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<DayPeriodRulesDesign3V1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rules
            .0
            .keys()
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l.clone())))
            .collect())
    }
}

impl DataProvider<DayPeriodRulesDesign4V1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesDesign4V1>, DataError> {
        self.check_req::<DayPeriodRulesDesign4V1>(req)?;
        
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
            .day_period_rules
            .0
            .get(&langid)
            .or_else(|| {
                let mut minimized = langid.clone();
                minimized.script = None;
                minimized.region = None;
                day_periods.supplemental.day_period_rules.0.get(&minimized)
            });

        let mut bitmap = [0u8; 9];
        if let Some(rules) = rules {
            for (period, rule) in rules {
                if let Some(idx) = period_idx(period) {
                    if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                        let start = parse_hour(from);
                        let end = parse_hour(before);
                        // Fill bitmap for hours from start to end
                        let mut h = start;
                        loop {
                            let byte_idx = (h as usize * 3) / 8;
                            let bit_offset = (h as usize * 3) % 8;
                            
                            bitmap[byte_idx] &= !(0x7 << bit_offset);
                            bitmap[byte_idx] |= (idx & 0x7) << bit_offset;
                            
                            if bit_offset > 5 {
                                let next_byte_idx = byte_idx + 1;
                                let bits_written = 8 - bit_offset;
                                bitmap[next_byte_idx] &= !(0x7 >> bits_written);
                                bitmap[next_byte_idx] |= (idx & 0x7) >> bits_written;
                            }
                            
                            h = (h + 1) % 24;
                            if h == end || (h == 0 && end == 24) {
                                break;
                            }
                        }
                    }
                }
            }
        }

        let data = DayPeriodRulesV1Design4 { bitmap };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<DayPeriodRulesDesign4V1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rules
            .0
            .keys()
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l.clone())))
            .collect())
    }
}

impl DataProvider<DayPeriodRulesDesign5V1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DayPeriodRulesDesign5V1>, DataError> {
        self.check_req::<DayPeriodRulesDesign5V1>(req)?;
        
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
            .day_period_rules
            .0
            .get(&langid)
            .or_else(|| {
                let mut minimized = langid.clone();
                minimized.script = None;
                minimized.region = None;
                day_periods.supplemental.day_period_rules.0.get(&minimized)
            });

        let mut presence = 0u8;
        let mut occupancy = 0u32; // 24 bits used

        if let Some(rules) = rules {
            let mut entries = Vec::new();
            for (period, rule) in rules {
                if let Some(idx) = period_idx(period) {
                    if let (Some(from), Some(before)) = (&rule.from, &rule.before) {
                        let start = parse_hour(from);
                        let end = parse_hour(before);
                        entries.push((idx, start, end));
                        presence |= 1 << idx;
                    }
                }
            }
            
            // Sort by start time to find transitions
            entries.sort_by_key(|e| e.1);

            if let Some(first) = entries.first() {
                let mut period_at_0 = None;
                for entry in &entries {
                    if entry.1 <= 0 && entry.2 > 0 {
                         period_at_0 = Some(entry.0);
                         break;
                    }
                    if entry.1 > entry.2 {
                         if 0 >= entry.1 || 0 < entry.2 {
                              period_at_0 = Some(entry.0);
                              break;
                         }
                    }
                }
                
                let mut starts_with_last = false;
                if let Some(p_idx) = period_at_0 {
                     if let Some(last) = entries.last() {
                          if p_idx == last.0 {
                               starts_with_last = true;
                          }
                     }
                }

                let mut current_state = if starts_with_last { 0 } else { 1 };
                
                let mut transitions = HashSet::new();
                for entry in &entries {
                     transitions.insert(entry.1);
                }

                for h in 0..24 {
                     if transitions.contains(&(h as u8)) {
                          current_state = 1 - current_state;
                     }
                     if current_state == 1 {
                          occupancy |= 1 << h;
                     }
                }
            }
        }

        let data = DayPeriodRulesV1Design5((occupancy << 8) | presence as u32);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<DayPeriodRulesDesign5V1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let day_periods: &cldr_serde::day_periods::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/dayPeriods.json")?;
        Ok(day_periods
            .supplemental
            .day_period_rules
            .0
            .keys()
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l.clone())))
            .collect())
    }
}
