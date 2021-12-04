// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops_helpers;
use crate::uprops_serde::script::ScriptExtensionsProperty;
use icu_codepointtrie::CodePointTrie;
use icu_properties::provider::{ScriptExtensionsPropertyV1, ScriptExtensionsPropertyV1Marker};
use icu_properties::script::ScriptWithExt;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::path::Path;
use tinystr::tinystr16;

/// This data provider returns a [`crate::script::ScriptExtensions`] instance,
/// which efficiently represents data for the Script and Script_Extensions
/// properties. The source data is the same as that of
/// [crate::provider::PropertiesDataProvider], which is a TOML file of data
/// for the property(-ies) desired, as given by the ICU4C property data
/// exporter tool.
pub struct ScriptExtensionsPropertyProvider {
    data: ScriptExtensionsProperty,
}

/// A data provider reading from .toml files produced by the ICU4C icuwriteuprops tool.
/// In particular, it expects a file `scx.toml` for the specially-exported data
/// structure that represents the combined data for Script / Script_Extensions.
impl ScriptExtensionsPropertyProvider {
    pub fn try_new(root_dir: &Path) -> eyre::Result<Self> {
        let data = uprops_helpers::load_script_extensions_from_dir(root_dir)?;
        Ok(Self { data })
    }
}

// source data to ICU4X data struct conversion
impl TryFrom<&ScriptExtensionsProperty> for ScriptExtensionsPropertyV1<'static> {
    type Error = DataError;

    fn try_from(
        scx_data: &ScriptExtensionsProperty,
    ) -> Result<ScriptExtensionsPropertyV1<'static>, DataError> {
        let toml_data = scx_data;
        let cpt_data = &toml_data.code_point_trie;
        let _scx_array_data = &toml_data.script_code_array;

        let _trie = CodePointTrie::<ScriptWithExt>::try_from(cpt_data)?;

        Err(DataError::Resource("unimplemented".to_string()))
    }
}

// implement data provider
impl DataProvider<ScriptExtensionsPropertyV1Marker> for ScriptExtensionsPropertyProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<ScriptExtensionsPropertyV1Marker>, DataError> {
        debug_assert_eq!(&tinystr16!("scx"), &req.resource_path.key.sub_category);

        Err(DataError::Resource("unimplemented".to_string()))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_() {
        unimplemented!();
    }
}
