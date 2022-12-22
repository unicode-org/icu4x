// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use icu_datetime::provider::tzdb::LocalTimeRecordV1;
use tzif::data::tzif::{TzifData, UtLocalIndicator};

pub(super) fn create_time_zone_transition_list_v1(
    tzif_data: &TzifData,
) -> BTreeMap<LocalTimeRecordV1, Vec<i64>> {
    let data_block = tzif_data
        .data_block2
        .as_ref()
        .unwrap_or(&tzif_data.data_block1);
    let local_time_records = &data_block.local_time_type_records;
    let transition_times = &data_block.transition_times;
    let transition_types = &data_block.transition_types;
    let ut_local_indicators = &data_block.ut_local_indicators;

    let mut transition_list: BTreeMap<LocalTimeRecordV1, Vec<i64>> = BTreeMap::new();

    for (&transition_time, &transition_type) in transition_times.iter().zip(transition_types) {
        let local_time_record = LocalTimeRecordV1 {
            offset: local_time_records[transition_type].utoff.0 as i32,
            is_dst: local_time_records[transition_type].is_dst,
        };
        let local_time = if !ut_local_indicators.is_empty()
            && matches!(ut_local_indicators[transition_type], UtLocalIndicator::Ut)
        {
            transition_time + local_time_records[transition_type].utoff
        } else {
            transition_time
        };
        match transition_list.get_mut(&local_time_record) {
            Some(v) => v.push(local_time.0),
            None => {
                transition_list.insert(local_time_record, vec![local_time.0]);
            }
        }
    }

    transition_list
}
