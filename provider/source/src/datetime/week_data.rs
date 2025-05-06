// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde::{
    self,
    week_data::{Territory, DEFAULT_TERRITORY},
};
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::calendar::provider::{CalendarWeekV1, WeekData, WeekdaySet};
use icu_provider::prelude::*;
use std::collections::HashSet;

impl DataProvider<CalendarWeekV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarWeekV1>, DataError> {
        self.check_req::<CalendarWeekV1>(req)?;
        let territory = req
            .id
            .locale
            .region
            .map(Territory::Region)
            .unwrap_or_else(|| DEFAULT_TERRITORY.clone());

        let week_data: &cldr_serde::week_data::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/weekData.json")?;
        let week_data = &week_data.supplemental.week_data;

        let first_weekday: icu::calendar::types::Weekday = week_data
            .first_day
            .get(&territory)
            .or_else(|| week_data.first_day.get(&DEFAULT_TERRITORY))
            .ok_or(DataError::custom(
                "Missing default entry for firstDay in weekData.json",
            ))?
            .into();

        let weekend = {
            let weekend_start = week_data
                .weekend_start
                .get(&territory)
                .or_else(|| week_data.weekend_start.get(&DEFAULT_TERRITORY))
                .ok_or(DataError::custom(
                    "Missing default entry for weekendStart in weekData.json",
                ))?;

            let weekend_end = week_data
                .weekend_end
                .get(&territory)
                .or_else(|| week_data.weekend_end.get(&DEFAULT_TERRITORY))
                .ok_or(DataError::custom(
                    "Missing default entry for weekendEnd in weekData.json",
                ))?;
            WeekdaySet::new(&[weekend_start.into(), weekend_end.into()])
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(WeekData {
                first_weekday,
                weekend,
            }),
        })
    }
}

impl IterableDataProviderCached<CalendarWeekV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let week_data: &cldr_serde::week_data::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/weekData.json")?;
        let week_data = &week_data.supplemental.week_data;
        Ok(week_data
            .min_days
            .keys()
            .chain(week_data.first_day.keys())
            .chain(week_data.weekend_end.keys())
            .chain(week_data.weekend_start.keys())
            .filter_map(|t| match t {
                &DEFAULT_TERRITORY => Some(None),
                Territory::Region(r) => Some(Some(*r)),
                _ => None,
            })
            .map(|region| {
                let mut locale = DataLocale::default();
                locale.region = region;
                DataIdentifierCow::from_locale(locale)
            })
            .collect())
    }
}

#[test]
fn test_basic_cldr_week_data_v2() {
    use icu::calendar::provider::WeekdaySet;
    use icu::calendar::types::Weekday::*;
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    let default_week_data: DataResponse<CalendarWeekV1> =
        provider.load(Default::default()).unwrap();
    assert_eq!(Monday, default_week_data.payload.get().first_weekday);
    assert_eq!(
        WeekdaySet::new(&[Saturday, Sunday]),
        default_week_data.payload.get().weekend
    );

    let fr_week_data: DataResponse<CalendarWeekV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("und-FR").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(Monday, fr_week_data.payload.get().first_weekday);
    assert_eq!(
        WeekdaySet::new(&[Saturday, Sunday]),
        fr_week_data.payload.get().weekend
    );

    let iq_week_data: DataResponse<CalendarWeekV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("und-IQ").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();
    // Only first_weekday is defined for IQ, min_week_days uses the default.
    assert_eq!(Saturday, iq_week_data.payload.get().first_weekday);
    assert_eq!(
        WeekdaySet::new(&[Friday, Saturday]),
        iq_week_data.payload.get().weekend
    );

    let gg_week_data: DataResponse<CalendarWeekV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("und-GG").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();
    // Only min_week_days is defined for GG, first_weekday uses the default.
    assert_eq!(
        default_week_data.payload.get().first_weekday,
        gg_week_data.payload.get().first_weekday
    );
    assert_eq!(
        WeekdaySet::new(&[Saturday, Sunday]),
        gg_week_data.payload.get().weekend
    );

    let ir_week_data: DataResponse<CalendarWeekV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("und-IR").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(Saturday, ir_week_data.payload.get().first_weekday);
    assert_eq!(
        WeekdaySet::new(&[Friday]),
        ir_week_data.payload.get().weekend
    );
}
