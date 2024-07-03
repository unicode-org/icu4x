// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use calendrical_calculations::chinese_based::{Chinese, ChineseBased, Dangi};
use icu::calendar::provider::chinese_based::*;
use icu_provider::prelude::*;

const YEARS: i32 = 250;
const ISO_START: i32 = 1900;

fn load<CB: ChineseBased>() -> ChineseBasedCacheV1<'static> {
    let extended_start = CB::extended_from_iso(ISO_START);
    let extended_end = extended_start + YEARS;
    ChineseBasedCacheV1::compute_for::<CB>(extended_start..extended_end)
}

impl DataProvider<ChineseCacheV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ChineseCacheV1Marker>, DataError> {
        self.check_req::<ChineseCacheV1Marker>(req)?;
        let cache = load::<Chinese>();
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(cache),
        })
    }
}

impl DataProvider<DangiCacheV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DangiCacheV1Marker>, DataError> {
        self.check_req::<DangiCacheV1Marker>(req)?;
        let cache = load::<Dangi>();
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(cache),
        })
    }
}

impl crate::IterableDataProviderCached<ChineseCacheV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl crate::IterableDataProviderCached<DangiCacheV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}
