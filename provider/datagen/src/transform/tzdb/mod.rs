// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::provider::tzdb::{
    TimeZoneHistoricTransitionsV1, TimeZoneHistoricTransitionsV1Marker,
};
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataPayload, DataProvider, DataRequest, DataResponse,
    DataResponseMetadata,
};
use itertools::Itertools;
use zerovec::ZeroVec;

use crate::transform::cldr::cldr_serde::time_zones as cldr_time_zones;
use crate::transform::cldr::time_zones::compute_bcp47_tzids_hashmap;

mod convert;
pub(crate) mod source;

impl DataProvider<TimeZoneHistoricTransitionsV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        _req: DataRequest,
    ) -> Result<
        icu_provider::DataResponse<TimeZoneHistoricTransitionsV1Marker>,
        icu_provider::DataError,
    > {
        let tzdb_cache = self.source.tzdb()?;
        let bcp47_tzid_resource: &cldr_time_zones::bcp47_tzid::Resource = self
            .source
            .cldr()?
            .bcp47()
            .read_and_parse("timezone.json")?;

        let bcp47_tzids =
            compute_bcp47_tzids_hashmap(&bcp47_tzid_resource.keyword.u.time_zones.values);

        let tzif_data = tzdb_cache.tzif().read_and_parse()?;

        let raw_transitions = bcp47_tzids
            .into_iter()
            .filter_map(|(tzid, bcp47_tzid)| {
                tzif_data
                    .iter()
                    .find_map(|(tzif_tzid, data)| (&tzid == tzif_tzid).then(|| data))
                    .map(|data| (bcp47_tzid, data))
            })
            .sorted_by(|(lhs_tzid, _), (rhs_tzid, _)| lhs_tzid.cmp(rhs_tzid))
            // If we are generating data from IANA files that were generated with backward-compatible aliases
            // there may be duplicate data for each BCP47 TZID, so we need to ensure we only take the data once.
            .dedup_by(|(lhs_tzid, _), (rhs_tzid, _)| lhs_tzid == rhs_tzid)
            .flat_map(|(bcp47_tzid, data)| {
                let list = convert::create_time_zone_transition_list_v1(data);
                std::iter::repeat(bcp47_tzid)
                    .zip(list.into_iter())
                    .map(|(a, (b, c))| {
                        let zv = ZeroVec::alloc_from_slice(&c);
                        (a, b, zv)
                    })
            })
            .collect::<Vec<_>>();

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
    fn supported_locales(&self) -> Result<Vec<icu_provider::DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DatagenProvider;
    use icu_datetime::provider::tzdb::LocalTimeRecordV1;
    use icu_timezone::TimeZoneBcp47Id;
    use tinystr::TinyAsciiStr;
    use zerovec::ule::AsULE;

    #[test]
    fn load_tzif_historic_transitions() {
        let provider = DatagenProvider::for_test();
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
        let provider = DatagenProvider::for_test();
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
}
