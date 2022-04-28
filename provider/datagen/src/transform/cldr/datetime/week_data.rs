// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
use crate::transform::cldr::cldr_serde::{
    self,
    week_data::{Territory, DEFAULT_TERRITORY},
};
use crate::transform::cldr::reader::open_reader;
use crate::SourceData;
use icu_calendar::arithmetic::week_of::CalendarInfo;
use icu_datetime::provider::week_data::*;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::sync::RwLock;

/// A data provider reading from CLDR JSON weekData files.
#[derive(Debug)]
pub struct WeekDataProvider {
    source: SourceData,
    data: RwLock<Option<(CalendarInfo, cldr_serde::week_data::WeekData)>>,
}

impl From<&SourceData> for WeekDataProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
            data: RwLock::new(None),
        }
    }
}

impl WeekDataProvider {
    fn init(&self) -> Result<(), DataError> {
        if self.data.read().unwrap().is_none() {
            let path = self
                .source
                .get_cldr_paths()?
                .cldr_core()
                .join("supplemental/weekData.json");
            let resource: cldr_serde::week_data::Resource =
                serde_json::from_reader(open_reader(&path)?)
                    .map_err(|e| DatagenError::from((e, path)))?;
            let week_data = resource.supplemental.week_data;
            *self.data.write().unwrap() = Some((
                CalendarInfo {
                    first_weekday: week_data
                        .first_day
                        .get(&DEFAULT_TERRITORY)
                        .ok_or(DataError::custom(
                            "Missing default entry for firstDay in weekData.json",
                        ))?
                        .into(),
                    min_week_days: week_data
                        .min_days
                        .get(&DEFAULT_TERRITORY)
                        .ok_or(DataError::custom(
                            "Missing default entry for minDays in weekData.json",
                        ))?
                        .0,
                },
                week_data,
            ));
        }
        Ok(())
    }
}

impl IterableResourceProvider<WeekDataV1Marker> for WeekDataProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    fn supported_options(&self) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        self.init()?;
        let guard = self.data.read().unwrap();
        let week_data = &guard.as_ref().unwrap().1;
        let regions: HashSet<ResourceOptions> = week_data
            .min_days
            .keys()
            .chain(week_data.first_day.keys())
            .filter_map(|t| match t {
                &DEFAULT_TERRITORY => Some(None),
                Territory::Region(r) => Some(Some(*r)),
                _ => None,
            })
            .map(ResourceOptions::temp_for_region)
            .collect();
        Ok(Box::new(regions.into_iter()))
    }
}

impl ResourceProvider<WeekDataV1Marker> for WeekDataProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<WeekDataV1Marker>, DataError> {
        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        let territory = req
            .options
            .region()
            .map(|v| -> Result<Territory, DataError> { Ok(Territory::Region(v)) })
            .transpose()?
            .unwrap_or_else(|| DEFAULT_TERRITORY.clone());

        self.init()?;

        let guard = self.data.read().unwrap();
        let (default, week_data) = &guard.as_ref().unwrap();

        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(WeekDataV1(CalendarInfo {
                first_weekday: week_data
                    .first_day
                    .get(&territory)
                    .map(|w| w.into())
                    .unwrap_or(default.first_weekday),
                min_week_days: week_data
                    .min_days
                    .get(&territory)
                    .map(|c| c.0)
                    .unwrap_or(default.min_week_days),
            }))),
        })
    }
}

icu_provider::impl_dyn_provider!(
    WeekDataProvider,
    [WeekDataV1Marker,],
    SERDE_SE,
    ITERABLE_SERDE_SE,
    DATA_CONVERTER
);

#[test]
fn basic_cldr_week_data() {
    use icu_calendar::types::IsoWeekday;
    use icu_locid::region;

    let provider = WeekDataProvider::from(&SourceData::for_test());

    let default_week_data: DataPayload<WeekDataV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions::default(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(1, default_week_data.get().0.min_week_days);
    assert_eq!(IsoWeekday::Monday, default_week_data.get().0.first_weekday);

    let fr_week_data: DataPayload<WeekDataV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions::temp_for_region(Some(region!("FR"))),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(4, fr_week_data.get().0.min_week_days);
    assert_eq!(IsoWeekday::Monday, fr_week_data.get().0.first_weekday);

    let iq_week_data: DataPayload<WeekDataV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions::temp_for_region(Some(region!("IQ"))),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    // Only first_weekday is defined for IQ, min_week_days uses the default.
    assert_eq!(
        default_week_data.get().0.min_week_days,
        iq_week_data.get().0.min_week_days
    );
    assert_eq!(IsoWeekday::Saturday, iq_week_data.get().0.first_weekday);

    let gg_week_data: DataPayload<WeekDataV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions::temp_for_region(Some(region!("GG"))),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(4, gg_week_data.get().0.min_week_days);
    // Only min_week_days is defined for GG, first_weekday uses the default.
    assert_eq!(
        default_week_data.get().0.first_weekday,
        gg_week_data.get().0.first_weekday
    );
}
