// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::collections::HashSet;

use icu::experimental::dimension::provider::currency_compact::*;
use icu_provider::prelude::*;
use icu_provider::DataProvider;
use zerovec::ZeroMap;

impl DataProvider<CurrencyCompactV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyCompactV1Marker>, DataError> {
        todo!()
    }
}
