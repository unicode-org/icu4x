// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde::{
    self,
    week_data::{Territory, DEFAULT_TERRITORY},
};

use icu_locid::LanguageIdentifier;
use icu_locid_transform::provider::*;

use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;

use std::collections::HashSet;
use zerovec::ZeroVec;

impl DataProvider<WeekDataV2Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<WeekDataV2Marker>, DataError> {
        self.check_req::<WeekDataV2Marker>(req)?;
        let territory = req
            .locale
            .region()
            .map(|r| -> Result<Territory, DataError> { Ok(Territory::Region(r)) })
            .transpose()?
            .unwrap_or_else(|| DEFAULT_TERRITORY.clone());

        let week_data: &cldr_serde::week_data::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/weekData.json")?;

        let week_data = &week_data.supplemental.week_data;

        let first_weekday: IsoWeekday = week_data
            .first_day
            .get(&territory)
            .or_else(|| week_data.first_day.get(&DEFAULT_TERRITORY))
            .ok_or(DataError::custom(
                "Missing default entry for firstDay in weekData.json",
            ))?
            .into();

        let min_week_days = week_data
            .min_days
            .get(&territory)
            .or_else(|| week_data.min_days.get(&DEFAULT_TERRITORY))
            .ok_or(DataError::custom(
                "Missing default entry for minDays in weekData.json",
            ))?
            .0;

        // Transform weekend_start and weekend_end to be a vector of days.
        // If a weekend has more than two days (more than start and end),
        // we will generate the days vector without assuming the ordering of the days numbers.

        let weekend_start: IsoWeekday = week_data
            .weekend_start
            .get(&territory)
            .or_else(|| week_data.weekend_start.get(&DEFAULT_TERRITORY))
            .ok_or(DataError::custom(
                "Missing default entry for weekendStart in weekData.json",
            ))?
            .into();

        let weekend_end: IsoWeekday = week_data
            .weekend_end
            .get(&territory)
            .or_else(|| week_data.weekend_end.get(&DEFAULT_TERRITORY))
            .ok_or(DataError::custom(
                "Missing default entry for weekendStart in weekData.json",
            ))?
            .into();

        let weekend_distance = (weekend_start as u8).abs_diff(weekend_end as u8);
        let weekend: ZeroVec<IsoWeekday> = if weekend_distance <= 1 {
            ZeroVec::alloc_from_slice(&[weekend_start, weekend_end])
        } else {
            let mut vec_weekend: Vec<IsoWeekday> = Vec::new();
            vec_weekend.reserve(weekend_distance.into());

            vec_weekend.push(weekend_start);
            for num_days in 1..weekend_distance {
                let next_weekend_day = add_to_weekday(weekend_start, num_days);
                vec_weekend.push(next_weekend_day);
            }
            vec_weekend.push(weekend_end);
            ZeroVec::alloc_from_slice(&vec_weekend)
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(WeekDataV2 {
                first_weekday,
                min_week_days,
                weekend,
            })),
        })
    }
}

impl IterableDataProvider<WeekDataV2Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        let week_data: &cldr_serde::week_data::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/weekData.json")?;

        let week_data = &week_data.supplemental.week_data;

        let regions: HashSet<DataLocale> = week_data
            .min_days
            .keys()
            .chain(week_data.first_day.keys())
            .chain(week_data.weekend_start.keys())
            .chain(week_data.weekend_end.keys())
            .filter_map(|t| match t {
                &DEFAULT_TERRITORY => Some(None),
                Territory::Region(r) => Some(Some(*r)),
                _ => None,
            })
            .map(LanguageIdentifier::from)
            .map(DataLocale::from)
            .collect();

        Ok(regions.into_iter().collect())
    }
}

/// Returns the weekday that's `num_days` after `weekday`.
fn add_to_weekday(weekday: IsoWeekday, num_days: u8) -> IsoWeekday {
    let new_weekday = (7 + (weekday as u8) + (num_days % 7)) % 7;
    IsoWeekday::from(new_weekday as usize)
}
