// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::bin_uniset::BinaryPropertyUnicodeSetDataProvider;
use crate::enum_uniset::EnumeratedPropertyUnicodeSetDataProvider;
use crate::uprops_helpers::get_last_component_no_version;
use icu_properties::provider::UnicodePropertyV1Marker;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;

use std::path::Path;

/// This data provider returns `UnicodeSet` data inside a `UnicodeProperty`
/// data struct. The source data is in the form of a directory of TOML file(s)
/// of data for the property(-ies) desired, as given by the ICU4C property data
/// exporter tool.
pub struct PropertiesDataProvider {
    binary: BinaryPropertyUnicodeSetDataProvider,
    enumerated: EnumeratedPropertyUnicodeSetDataProvider,
}

impl PropertiesDataProvider {
    /// Construct a new data provider instance. `root_dir` is the path to the
    /// root directory containing the property data TOML files.
    pub fn try_new(root_dir: &Path) -> eyre::Result<Self> {
        let binary = BinaryPropertyUnicodeSetDataProvider::try_new(root_dir)?;
        let enumerated = EnumeratedPropertyUnicodeSetDataProvider::try_new(root_dir)?;
        Ok(Self { binary, enumerated })
    }
}

impl DynProvider<UnicodePropertyV1Marker> for PropertiesDataProvider {
    fn load_payload(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<UnicodePropertyV1Marker>, DataError> {
        if get_last_component_no_version(&key)
            .contains('=')
        {
            self.enumerated.load_payload(key, req)
        } else {
            self.binary.load_payload(key, req)
        }
    }
}

icu_provider::impl_dyn_provider!(PropertiesDataProvider, {
    _ => UnicodePropertyV1Marker,
}, SERDE_SE);

impl IterableProvider for PropertiesDataProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}
