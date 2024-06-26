// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::DatagenProvider;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

impl DataProvider<HelloWorldV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        self.check_req::<HelloWorldV1Marker>(req)?;
        HelloWorldProvider.load(req)
    }
}

impl crate::IterableDataProviderCached<HelloWorldV1Marker> for DatagenProvider {
    fn iter_requests_cached(
        &self,
    ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        HelloWorldProvider.iter_requests()
    }
}
