// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use icu::calendar::cal::{HijriObservational, HijriUmmAlQura};
use icu::calendar::provider::hijri::*;
use icu_provider::prelude::*;

impl DataProvider<CalendarHijriObservationalMeccaV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CalendarHijriObservationalMeccaV1>, DataError> {
        self.check_req::<CalendarHijriObservationalMeccaV1>(req)?;
        let cache = HijriObservational::new_mecca_always_calculating().build_cache(1317..1567);
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(cache),
        })
    }
}

impl crate::IterableDataProviderCached<CalendarHijriObservationalMeccaV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<CalendarHijriUmmalquraV1> for crate::SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarHijriUmmalquraV1>, DataError> {
        self.check_req::<CalendarHijriUmmalquraV1>(req)?;
        let cache = HijriUmmAlQura::new_always_calculating().build_cache(1317..1567);
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(cache),
        })
    }
}

impl crate::IterableDataProviderCached<CalendarHijriUmmalquraV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}
