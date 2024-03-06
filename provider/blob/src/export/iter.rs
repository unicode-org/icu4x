// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::BlobDataProvider;
use icu_provider::datagen::*;
use icu_provider::prelude::*;

impl DynamicDataProvider<BufferMarker> for BlobDataProvider {
    fn load_data(
        &self,
        key: DataKey,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        self.load_buffer(key, req)
    }
}

impl IterableDynamicDataProvider<BufferMarker> for BlobDataProvider {
    fn supported_locales_for_key(&self, key: DataKey) -> Result<Vec<DataLocale>, DataError> {
        self.data.get().list_locales(key)
    }
}
