// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use calendrical_calculations::islamic::{IslamicBased, ObservationalIslamic, SaudiIslamic};
use calendrical_calculations::iso;
use icu::calendar::provider::hijri::*;
use icu_provider::prelude::*;

const YEARS: i32 = 250;
const ISO_START: i32 = 1900;

fn load<IB: IslamicBased>(model: IB) -> HijriCache<'static> {
    let extended_start = IB::approximate_islamic_from_fixed(iso::fixed_from_iso(ISO_START, 1, 1));
    let extended_end = extended_start + YEARS;
    HijriCache::compute_for(extended_start..extended_end, model)
}

impl DataProvider<CalendarHijriObservationalMeccaV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CalendarHijriObservationalMeccaV1>, DataError> {
        self.check_req::<CalendarHijriObservationalMeccaV1>(req)?;
        let cache = load(ObservationalIslamic::mecca());
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
        let cache = load(SaudiIslamic);
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
