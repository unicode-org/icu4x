// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_personnames::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;

impl DataProvider<PersonNamesFormattingDefinitionV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        _req: DataRequest,
    ) -> Result<DataResponse<PersonNamesFormattingDefinitionV1Marker>, DataError> {
        Err(DataError::custom("Not Implemented"))
    }
}

impl IterableDataProvider<PersonNamesFormattingDefinitionV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Err(DataError::custom("Not Implemented"))
    }
}
