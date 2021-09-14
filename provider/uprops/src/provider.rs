// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::binary::BinaryPropertiesDataProvider;
use crate::enumerated::EnumeratedPropertiesDataProvider;
use icu_provider::iter::IterableDataProviderCore;
use icu_provider::prelude::*;
use icu_uniset::provider::UnicodePropertyV1Marker;

use std::path::PathBuf;

pub struct PropertiesDataProvider {
    binary: BinaryPropertiesDataProvider,
    enumerated: EnumeratedPropertiesDataProvider,
}

impl PropertiesDataProvider {
    pub fn new(root_dir: PathBuf) -> Self {
        let binary = BinaryPropertiesDataProvider::new(root_dir.clone());
        let enumerated = EnumeratedPropertiesDataProvider::new(root_dir);
        Self { binary, enumerated }
    }
}

impl<'data> DataProvider<'data, UnicodePropertyV1Marker> for PropertiesDataProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, UnicodePropertyV1Marker>, DataError> {
        if req.resource_path.key.sub_category.contains('=') {
            self.enumerated.load_payload(req)
        } else {
            self.binary.load_payload(req)
        }
    }
}

icu_provider::impl_dyn_provider!(PropertiesDataProvider, {
    _ => UnicodePropertyV1Marker,
}, SERDE_SE, 'data);

impl IterableDataProviderCore for PropertiesDataProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}
