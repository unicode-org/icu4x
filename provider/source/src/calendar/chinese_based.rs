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

fn load<CB: ChineseBased>() -> ChineseBasedCache<'static> {
    let extended_start = CB::extended_from_iso(ISO_START);
    let extended_end = extended_start + YEARS;
    ChineseBasedCache::compute_for::<CB>(extended_start..extended_end)
}

impl DataProvider<CalendarChineseV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarChineseV1>, DataError> {
        self.check_req::<CalendarChineseV1>(req)?;
        let cache = load::<Chinese>();
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(cache),
        })
    }
}

impl DataProvider<CalendarDangiV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarDangiV1>, DataError> {
        self.check_req::<CalendarDangiV1>(req)?;
        let cache = load::<Dangi>();
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(cache),
        })
    }
}

impl crate::IterableDataProviderCached<CalendarChineseV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl crate::IterableDataProviderCached<CalendarDangiV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}
