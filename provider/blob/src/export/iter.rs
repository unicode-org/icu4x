// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::BlobDataProvider;
use icu_provider::datagen::*;
use icu_provider::prelude::*;

impl IterableDynamicDataProvider<BufferMarker> for BlobDataProvider {
    fn supported_requests_for_marker(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        self.data.get().list_requests(marker)
    }
}
