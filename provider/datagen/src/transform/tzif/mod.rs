// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde::time_zones as cldr_time_zones;
use crate::transform::cldr::time_zones::compute_bcp47_tzids_hashmap;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_timezone::provider::tzif::{
    TimeZoneHistoricTransitionsV1, TimeZoneHistoricTransitionsV1Marker, TimeZoneTransitionRuleV1,
    TimeZoneTransitionRulesV1, TimeZoneTransitionRulesV1Marker,
};
use itertools::Itertools;
use zerovec::ZeroVec;

mod convert;
pub(crate) mod source;

impl DataProvider<TimeZoneHistoricTransitionsV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        _req: DataRequest,
    ) -> Result<DataResponse<TimeZoneHistoricTransitionsV1Marker>, DataError> {
        let tzif_paths = self.tzif()?;

        let bcp47_tzid_resource: &cldr_time_zones::bcp47_tzid::Resource =
            self.cldr()?.bcp47().read_and_parse("timezone.json")?;

        let bcp47_tzids =
            compute_bcp47_tzids_hashmap(&bcp47_tzid_resource.keyword.u.time_zones.values);

        let tzif_data = tzif_paths.read_and_parse()?;

        let raw_transitions = bcp47_tzids
            .into_iter()
            .filter_map(|(tzid, bcp47_tzid)| tzif_data.get(&tzid).map(|data| (bcp47_tzid, data)))
            .sorted_by(|(lhs_tzid, _), (rhs_tzid, _)| lhs_tzid.cmp(rhs_tzid))
            // If we are generating data from IANA files that were generated with backward-compatible aliases
            // there may be duplicate data for each BCP47 TZID, so we need to ensure we only take the data once.
            .dedup_by(|(lhs_tzid, _), (rhs_tzid, _)| lhs_tzid == rhs_tzid)
            .flat_map(|(bcp47_tzid, data)| {
                match convert::try_create_time_zone_transition_list(data) {
                    Ok(list) => list
                        .into_iter()
                        .map(move |(local_time_record, transition_times)| {
                            Ok((
                                bcp47_tzid,
                                local_time_record,
                                ZeroVec::alloc_from_slice(&transition_times),
                            ))
                        })
                        .collect(),
                    Err(e) => vec![Err(e)],
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let transitions_v1 = TimeZoneHistoricTransitionsV1 {
            historic_transitions_indices: raw_transitions
                .iter()
                .enumerate()
                .map(|(index, &(tzid, record, _))| (tzid, record, index))
                .collect(),
            historic_transitions: raw_transitions
                .into_iter()
                .map(|(_, _, transitions)| transitions)
                .collect::<Vec<_>>()
                .as_slice()
                .into(),
        };

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(transitions_v1)),
        })
    }
}

impl IterableDataProvider<TimeZoneHistoricTransitionsV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl DataProvider<TimeZoneTransitionRulesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        _req: DataRequest,
    ) -> Result<DataResponse<TimeZoneTransitionRulesV1Marker>, DataError> {
        let tzif_paths = self.tzif()?;
        let bcp47_tzid_resource: &cldr_time_zones::bcp47_tzid::Resource =
            self.cldr()?.bcp47().read_and_parse("timezone.json")?;

        let bcp47_tzids =
            compute_bcp47_tzids_hashmap(&bcp47_tzid_resource.keyword.u.time_zones.values);

        let tzif_data = tzif_paths.read_and_parse()?;

        let raw_rules = bcp47_tzids
            .into_iter()
            .filter_map(|(tzid, bcp47_tzid)| tzif_data.get(&tzid).map(|data| (bcp47_tzid, data)))
            .sorted_by(|(lhs_tzid, _), (rhs_tzid, _)| lhs_tzid.cmp(rhs_tzid))
            // If we are generating data from IANA files that were generated with backward-compatible aliases
            // there may be duplicate data for each BCP47 TZID, so we need to ensure we only take the data once.
            .dedup_by(|(lhs_tzid, _), (rhs_tzid, _)| lhs_tzid == rhs_tzid)
            .filter_map(|(bcp47_tzid, data)| {
                data.footer.as_ref().map(|tz_string| {
                    (
                        bcp47_tzid,
                        TimeZoneTransitionRuleV1 {
                            std_offset: tz_string.std_info.offset.0 as i32,
                            dst_offset: tz_string
                                .dst_info
                                .as_ref()
                                .map(|info| info.variant_info.offset.0 as i32),
                            dst_start: tz_string
                                .dst_info
                                .as_ref()
                                .map(|info| convert::create_transition_date(info.start_date)),
                            dst_end: tz_string
                                .dst_info
                                .as_ref()
                                .map(|info| convert::create_transition_date(info.end_date)),
                        },
                    )
                })
            });

        let rules_v1 = TimeZoneTransitionRulesV1 {
            transition_rules: raw_rules.collect(),
        };

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(rules_v1)),
        })
    }
}

impl IterableDataProvider<TimeZoneTransitionRulesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatagenProvider;
    use icu_timezone::provider::tzif::{LocalTimeRecordV1, TransitionDayV1};
    use icu_timezone::TimeZoneBcp47Id;
    use tinystr::TinyAsciiStr;
    use zerovec::ule::AsULE;

    #[test]
    fn load_tzif_historic_transitions() {
        let provider = DatagenProvider::new_testing();
        let payload: DataPayload<TimeZoneHistoricTransitionsV1Marker> = provider
            .load(DataRequest::default())
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let data = payload.get();
        assert!(
            !data.historic_transitions_indices.is_empty(),
            "TZif historic transitions indices should be populated"
        );
        assert!(
            !data.historic_transitions.is_empty(),
            "TZif historic transitions should be populated"
        );
        for cursor in data.historic_transitions_indices.iter0() {
            assert!(
                cursor.iter1_copied().count() > 0,
                "Each TZID should have at least one index entry"
            );
            for (_, index) in cursor.iter1_copied() {
                assert!(
                    data.historic_transitions.get(index).is_some(),
                    "Each index should lead to a populated transition list"
                );
                assert!(
                    !data.historic_transitions[index].is_empty(),
                    "Each historic transition list should have at least one entry"
                );
            }
        }
    }

    #[test]
    fn test_locate_transition() {
        let provider = DatagenProvider::new_testing();
        let payload: DataPayload<TimeZoneHistoricTransitionsV1Marker> = provider
            .load(DataRequest::default())
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let data = payload.get();
        let tzid = TimeZoneBcp47Id(TinyAsciiStr::<8>::from_str("uslax").unwrap());
        let timestamp = -179134000;
        let expected = LocalTimeRecordV1 {
            offset: -25200,
            is_dst: true,
        };
        let cursor = data.historic_transitions_indices.get0(&tzid);
        assert!(
            cursor.is_some(),
            "Transitions should exist for time zone ID {tzid:?}, but none were found"
        );
        if let Some(cursor) = cursor {
            let record = cursor
                .iter1_copied()
                .min_by_key(|&(_, index)| {
                    data.historic_transitions[index]
                        .iter()
                        .take_while(|&seconds| seconds <= timestamp)
                        .map(|seconds| seconds.abs_diff(timestamp))
                        .min()
                })
                .map(|(record, _)| record)
                .copied();
            assert!(record.is_some(), "A valid local time record for {tzid:?} with the given timestamp {timestamp:?} should be found, but none was found");
            if let Some(record_ule) = record {
                let record = LocalTimeRecordV1::from_unaligned(record_ule);
                assert_eq!(
                    record,
                    expected,
                    "Given {tzid:?} and {timestamp}, expected the following record {expected:?}, but found {record:?}",
                );
            }
        }
    }

    #[test]
    fn load_tzif_transition_rules() {
        let provider = DatagenProvider::new_testing();
        let payload: DataPayload<TimeZoneTransitionRulesV1Marker> = provider
            .load(DataRequest::default())
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let data = payload.get();
        assert!(
            !data.transition_rules.is_empty(),
            "Transition rules data should contain at least one rule"
        );
        for (_, &rule_ule) in data.transition_rules.iter() {
            let rule = TimeZoneTransitionRuleV1::from_unaligned(rule_ule);
            if let Some(value) = rule.dst_offset {
                assert!(
                    -90_000 < value && value < 90_000,
                    "The TransitionRule DST offset should be between -90000 seconds and 90000 seconds, but it is {}",
                    value
                );
            }
            if let Some(date) = rule.dst_start {
                assert!(
                    -601200 <= date.time_of_day && date.time_of_day <= 601200,
                    "The time of day should be between [-601200, 601200] seconds, but it is {}",
                    date.time_of_day
                );
                match date.day_of_year {
                    TransitionDayV1::NoLeap(day) => assert!(
                        (1..=365).contains(&day),
                        "The NoLeap transition day should be between [1, 365], but it is {}",
                        day
                    ),
                    TransitionDayV1::WithLeap(day) => assert!(
                        day <= 365,
                        "The WithLeap transition day should be between [0, 365], but it is {}",
                        day
                    ),
                    TransitionDayV1::Mwd(m, w, d) => {
                        assert!(
                            (1..=12).contains(&m),
                            "The Mwd month value should be between [1, 12], but it is {}",
                            m
                        );
                        assert!(
                            (1..=5).contains(&w),
                            "The Mwd wonth value should be between [1,  5], but it is {}",
                            w
                        );
                        assert!(
                            (0..=6).contains(&d),
                            "The Mwd donth value should be between [0, 6], but it is {}",
                            d
                        );
                    }
                }
            }
        }
    }
}
