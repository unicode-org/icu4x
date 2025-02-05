// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::decimal::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

impl DataProvider<DecimalDigitsV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DecimalDigitsV1Marker>, DataError> {
        self.check_req::<DecimalDigitsV1Marker>(req)?;

        let nsname = req.id.marker_attributes.as_str();

        if nsname.is_empty() {
            panic!("Found empty numbering system")
        }

        let result = DecimalDigits {
            digits: self.get_digits_for_numbering_system(nsname)?,
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl IterableDataProviderCached<DecimalDigitsV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.iter_ids_for_used_numbers()
    }
}
