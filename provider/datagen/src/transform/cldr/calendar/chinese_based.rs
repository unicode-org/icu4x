// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use calendrical_calculations::chinese_based::{Chinese, ChineseBased, Dangi};
use icu_calendar::provider::chinese_based::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;

const YEARS: i32 = 200;
const ISO_START: i32 = 1900;

fn load<CB: ChineseBased>() -> ChineseBasedCacheV1<'static> {
    let extended_start = CB::extended_from_iso(ISO_START);
    let extended_end = extended_start + YEARS;
    ChineseBasedCacheV1::compute_for::<CB>(extended_start..extended_end)
}

impl DataProvider<ChineseCacheV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ChineseCacheV1Marker>, DataError> {
        self.check_req::<ChineseCacheV1Marker>(req)?;
        let cache = load::<Chinese>();
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(cache)),
        })
    }
}

impl DataProvider<DangiCacheV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DangiCacheV1Marker>, DataError> {
        self.check_req::<DangiCacheV1Marker>(req)?;
        let cache = load::<Dangi>();
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(cache)),
        })
    }
}

impl IterableDataProvider<ChineseCacheV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl IterableDataProvider<DangiCacheV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}
