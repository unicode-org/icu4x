// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use icu::calendar::cal::Hijri;
use icu::calendar::provider::hijri::*;
use icu_provider::prelude::*;

impl DataProvider<CalendarHijriSimulatedMeccaV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CalendarHijriSimulatedMeccaV1>, DataError> {
        self.check_req::<CalendarHijriSimulatedMeccaV1>(req)?;
        let cache = Hijri::new_simulated_mecca_always_calculating().build_cache(1317..1567);
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(cache),
        })
    }
}

impl crate::IterableDataProviderCached<CalendarHijriSimulatedMeccaV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}
