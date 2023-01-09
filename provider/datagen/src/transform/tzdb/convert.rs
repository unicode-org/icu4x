// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use icu_datetime::provider::tzdb::{LocalTimeRecordV1, TransitionDateV1, TransitionDayV1};
use tzif::data::{
    posix::{TransitionDate, TransitionDay},
    tzif::{TzifData, UtLocalIndicator},
};

fn create_transition_day(day: TransitionDay) -> TransitionDayV1 {
    match day {
        TransitionDay::NoLeap(value) => TransitionDayV1::NoLeap(value),
        TransitionDay::WithLeap(value) => TransitionDayV1::WithLeap(value),
        TransitionDay::Mwd(m, w, d) => TransitionDayV1::Mwd(m as u8, w as u8, d as u8),
    }
}

pub(super) fn create_transition_date(date: TransitionDate) -> TransitionDateV1 {
    TransitionDateV1 {
        day_of_year: create_transition_day(date.day),
        time_of_day: date.time.0 as i32,
    }
}

pub(super) fn try_create_time_zone_transition_list(
    tzif_data: &TzifData,
) -> Result<BTreeMap<LocalTimeRecordV1, Vec<i64>>, icu_provider::DataError> {
    let data_block = tzif_data
        .data_block2
        .as_ref()
        .unwrap_or(&tzif_data.data_block1);
    let local_time_records = &data_block.local_time_type_records;
    let transition_times = &data_block.transition_times;
    let transition_types = &data_block.transition_types;
    let ut_local_indicators = &data_block.ut_local_indicators;

    let mut transition_list: BTreeMap<LocalTimeRecordV1, Vec<i64>> = BTreeMap::new();

    if transition_times.len() != transition_types.len() {
        return Err(icu_provider::DataError::custom(
            "TZif transition_times and transition_types must have the same length",
        ));
    }

    for (&transition_time, &transition_type) in transition_times.iter().zip(transition_types) {
        let local_time_record = LocalTimeRecordV1 {
            offset: local_time_records[transition_type].utoff.0 as i32,
            is_dst: local_time_records[transition_type].is_dst,
        };
        // We want all of the values to be in local time.
        // According to the TZif spec, if there are no UT/local indicators, then every time
        // is assumed to be in local time. However, if there are UT/local indicators,
        // and the type is UT (rather than Local), then we need to convert it to local time
        // by applying the `utoff` value.
        // https://datatracker.ietf.org/doc/rfc8536/
        let local_time = if !ut_local_indicators.is_empty()
            && matches!(ut_local_indicators[transition_type], UtLocalIndicator::Ut)
        {
            transition_time + local_time_records[transition_type].utoff
        } else {
            transition_time
        };
        transition_list
            .entry(local_time_record)
            .or_default()
            .push(local_time.0);
    }

    Ok(transition_list)
}
