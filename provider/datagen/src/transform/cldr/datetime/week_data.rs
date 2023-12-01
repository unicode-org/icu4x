// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde::{
    self,
    week_data::{Territory, Weekday, DEFAULT_TERRITORY},
};
use icu_calendar::provider::{
    WeekDataV1, WeekDataV1Marker, WeekDataV2, WeekDataV2Marker, WeekendSet,
};
use icu_locid::LanguageIdentifier;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::HashSet;

impl IterableDataProvider<WeekDataV1Marker> for crate::DatagenProvider {
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

impl DataProvider<WeekDataV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<WeekDataV1Marker>, DataError> {
        self.check_req::<WeekDataV1Marker>(req)?;
        let territory = req
            .locale
            .region()
            .map(|v| -> Result<Territory, DataError> { Ok(Territory::Region(v)) })
            .transpose()?
            .unwrap_or_else(|| DEFAULT_TERRITORY.clone());

        let week_data: &cldr_serde::week_data::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/weekData.json")?;
        let week_data = &week_data.supplemental.week_data;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(WeekDataV1 {
                first_weekday: week_data
                    .first_day
                    .get(&territory)
                    .or_else(|| week_data.first_day.get(&DEFAULT_TERRITORY))
                    .ok_or(DataError::custom(
                        "Missing default entry for firstDay in weekData.json",
                    ))?
                    .into(),
                min_week_days: week_data
                    .min_days
                    .get(&territory)
                    .or_else(|| week_data.min_days.get(&DEFAULT_TERRITORY))
                    .ok_or(DataError::custom(
                        "Missing default entry for minDays in weekData.json",
                    ))?
                    .0,
            })),
        })
    }
}

#[test]
fn basic_cldr_week_data() {
    use icu_calendar::types::IsoWeekday;
    use icu_locid::langid;

    let provider = crate::DatagenProvider::new_testing();

    let default_week_data: DataPayload<WeekDataV1Marker> = provider
        .load(Default::default())
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(1, default_week_data.get().min_week_days);
    assert_eq!(IsoWeekday::Monday, default_week_data.get().first_weekday);

    let fr_week_data: DataPayload<WeekDataV1Marker> = provider
        .load(DataRequest {
            locale: &DataLocale::from(langid!("und-FR")),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(4, fr_week_data.get().min_week_days);
    assert_eq!(IsoWeekday::Monday, fr_week_data.get().first_weekday);

    let iq_week_data: DataPayload<WeekDataV1Marker> = provider
        .load(DataRequest {
            locale: &DataLocale::from(langid!("und-IQ")),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    // Only first_weekday is defined for IQ, min_week_days uses the default.
    assert_eq!(
        default_week_data.get().min_week_days,
        iq_week_data.get().min_week_days
    );
    assert_eq!(IsoWeekday::Saturday, iq_week_data.get().first_weekday);

    let gg_week_data: DataPayload<WeekDataV1Marker> = provider
        .load(DataRequest {
            locale: &DataLocale::from(langid!("und-GG")),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(4, gg_week_data.get().min_week_days);
    // Only min_week_days is defined for GG, first_weekday uses the default.
    assert_eq!(
        default_week_data.get().first_weekday,
        gg_week_data.get().first_weekday
    );
}

impl DataProvider<WeekDataV2Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<WeekDataV2Marker>, DataError> {
        self.check_req::<WeekDataV2Marker>(req)?;
        let territory = req
            .locale
            .region()
            .map(Territory::Region)
            .unwrap_or_else(|| DEFAULT_TERRITORY.clone());

        let week_data: &cldr_serde::week_data::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/weekData.json")?;
        let week_data = &week_data.supplemental.week_data;

        let first_weekday: icu_calendar::types::IsoWeekday = week_data
            .first_day
            .get(&territory)
            .or_else(|| week_data.first_day.get(&DEFAULT_TERRITORY))
            .ok_or(DataError::custom(
                "Missing default entry for firstDay in weekData.json",
            ))?
            .into();

        let min_week_days: u8 = week_data
            .min_days
            .get(&territory)
            .or_else(|| week_data.min_days.get(&DEFAULT_TERRITORY))
            .ok_or(DataError::custom(
                "Missing default entry for minDays in weekData.json",
            ))?
            .0;

        let weekend: WeekendSet = {
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

            WeekendSet(weekend_start.bit_value() | weekend_end.bit_value())
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
            .chain(week_data.weekend_start.keys())
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

impl Weekday {
    fn bit_value(&self) -> u8 {
        match self {
            Weekday::Mon => 1 << 6,
            Weekday::Tue => 1 << 5,
            Weekday::Wed => 1 << 4,
            Weekday::Thu => 1 << 3,
            Weekday::Fri => 1 << 2,
            Weekday::Sat => 1 << 1,
            Weekday::Sun => 1 << 0,
        }
    }
}

#[test]
fn basic_cldr_week_data_v2() {
    use icu_calendar::types::IsoWeekday;
    use icu_locid::langid;

    let provider = crate::DatagenProvider::new_testing();

    let default_week_data: DataPayload<WeekDataV2Marker> = provider
        .load(Default::default())
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(1, default_week_data.get().min_week_days);
    assert_eq!(IsoWeekday::Monday, default_week_data.get().first_weekday);
    assert_eq!(
        Weekday::Sat.bit_value() | Weekday::Sun.bit_value(),
        default_week_data.get().weekend.0
    );

    let fr_week_data: DataPayload<WeekDataV2Marker> = provider
        .load(DataRequest {
            locale: &DataLocale::from(langid!("und-FR")),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(4, fr_week_data.get().min_week_days);
    assert_eq!(IsoWeekday::Monday, fr_week_data.get().first_weekday);
    assert_eq!(
        Weekday::Sat.bit_value() | Weekday::Sun.bit_value(),
        fr_week_data.get().weekend.0
    );

    let iq_week_data: DataPayload<WeekDataV2Marker> = provider
        .load(DataRequest {
            locale: &DataLocale::from(langid!("und-IQ")),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    // Only first_weekday is defined for IQ, min_week_days uses the default.
    assert_eq!(
        default_week_data.get().min_week_days,
        iq_week_data.get().min_week_days
    );
    assert_eq!(IsoWeekday::Saturday, iq_week_data.get().first_weekday);
    assert_eq!(
        Weekday::Fri.bit_value() | Weekday::Sat.bit_value(),
        iq_week_data.get().weekend.0
    );

    let gg_week_data: DataPayload<WeekDataV2Marker> = provider
        .load(DataRequest {
            locale: &DataLocale::from(langid!("und-GG")),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(4, gg_week_data.get().min_week_days);
    // Only min_week_days is defined for GG, first_weekday uses the default.
    assert_eq!(
        default_week_data.get().first_weekday,
        gg_week_data.get().first_weekday
    );
    assert_eq!(
        Weekday::Sat.bit_value() | Weekday::Sun.bit_value(),
        gg_week_data.get().weekend.0
    );

    let ir_week_data: DataPayload<WeekDataV2Marker> = provider
        .load(DataRequest {
            locale: &DataLocale::from(langid!("und-IR")),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(
        default_week_data.get().min_week_days,
        ir_week_data.get().min_week_days
    );
    assert_eq!(IsoWeekday::Saturday, ir_week_data.get().first_weekday);
    assert_eq!(Weekday::Fri.bit_value(), ir_week_data.get().weekend.0);
}
