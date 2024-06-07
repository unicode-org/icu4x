// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::provider::DatagenProvider;
use calendrical_calculations::islamic::{
    IslamicBasedMarker, ObservationalIslamicMarker, SaudiIslamicMarker,
};
use calendrical_calculations::iso;
use icu_calendar::provider::islamic::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;

const YEARS: i32 = 250;
const ISO_START: i32 = 1900;

fn load<IB: IslamicBasedMarker>() -> IslamicCacheV1<'static> {
    let extended_start = IB::approximate_islamic_from_fixed(iso::fixed_from_iso(ISO_START, 1, 1));
    let extended_end = extended_start + YEARS;
    IslamicCacheV1::compute_for::<IB>(extended_start..extended_end)
}

impl DataProvider<IslamicObservationalCacheV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<IslamicObservationalCacheV1Marker>, DataError> {
        self.check_req::<IslamicObservationalCacheV1Marker>(req)?;
        let cache = load::<ObservationalIslamicMarker>();
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(cache)),
        })
    }
}

impl IterableDataProvider<IslamicObservationalCacheV1Marker> for DatagenProvider {
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<IslamicUmmAlQuraCacheV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<IslamicUmmAlQuraCacheV1Marker>, DataError> {
        self.check_req::<IslamicUmmAlQuraCacheV1Marker>(req)?;
        let cache = load::<SaudiIslamicMarker>();
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(cache)),
        })
    }
}

impl IterableDataProvider<IslamicUmmAlQuraCacheV1Marker> for DatagenProvider {
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}
