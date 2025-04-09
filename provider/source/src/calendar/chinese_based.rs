// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use calendrical_calculations::chinese_based::{Chinese, Dangi};
use icu::calendar::provider::chinese_based::*;
use icu_provider::prelude::*;

const YEARS: i32 = 250;
const ISO_START: i32 = 1900;

impl DataProvider<CalendarChineseV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarChineseV1>, DataError> {
        self.check_req::<CalendarChineseV1>(req)?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ChineseBasedCache::compute_for::<Chinese>(
                ISO_START..(ISO_START + YEARS),
            )),
        })
    }
}

impl DataProvider<CalendarDangiV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarDangiV1>, DataError> {
        self.check_req::<CalendarDangiV1>(req)?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ChineseBasedCache::compute_for::<Dangi>(
                ISO_START..(ISO_START + YEARS),
            )),
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
