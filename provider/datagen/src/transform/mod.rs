// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod cldr;
pub mod icuexport;
#[cfg(feature = "experimental")]
pub mod segmenter;

#[cfg(feature = "experimental")]
pub mod tzdb;

use icu_provider::datagen::*;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;

impl DataProvider<HelloWorldV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        HelloWorldProvider.load(req)
    }
}

impl IterableDataProvider<HelloWorldV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        HelloWorldProvider.supported_locales()
    }
}
