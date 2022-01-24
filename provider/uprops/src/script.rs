// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops_helpers;
use crate::uprops_serde::script_extensions::ScriptExtensionsProperty;
use icu_codepointtrie::CodePointTrie;
use icu_properties::provider::{ScriptExtensionsPropertyV1, ScriptExtensionsPropertyV1Marker};
use icu_properties::script::{ScriptExtensions, ScriptWithExt};
use icu_properties::Script;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::path::Path;
use zerovec::{VarZeroVec, ZeroSlice, ZeroVec};

/// This data provider returns a [`ScriptExtensions`] instance,
/// which efficiently represents data for the Script and Script_Extensions
/// properties. The source data is the same as that of
/// [crate::provider::PropertiesDataProvider], which is a TOML file of data
/// for the property(-ies) desired, as given by the ICU4C property data
/// exporter tool.
pub struct ScriptExtensionsPropertyProvider {
    data: ScriptExtensionsProperty,
}

/// A data provider reading from .toml files produced by the ICU4C icuexportdata tool.
/// In particular, it expects a file `scx.toml` for the specially-exported data
/// structure that represents the combined data for Script / Script_Extensions.
impl ScriptExtensionsPropertyProvider {
    pub fn try_new(root_dir: &Path) -> eyre::Result<Self> {
        let data = uprops_helpers::load_script_extensions_from_dir(root_dir)?;
        Ok(Self { data })
    }
}

// source data to ICU4X plain/raw/utility data structure
impl TryFrom<&ScriptExtensionsProperty> for ScriptExtensions<'static> {
    type Error = DataError;

    fn try_from(
        scx_data: &ScriptExtensionsProperty,
    ) -> Result<ScriptExtensions<'static>, DataError> {
        let cpt_data = &scx_data.code_point_trie;
        let scx_array_data = &scx_data.script_code_array;

        let trie = CodePointTrie::<ScriptWithExt>::try_from(cpt_data)?;

        // Convert the input from Vec<Vec<u16>> to Vec<ZeroVec<Script>> so that
        // we can go through the VarZeroVec construction process for a desired result
        // type of VZV<ZeroSlice<Script>>
        let ule_scx_array_data: Vec<ZeroVec<Script>> = scx_array_data
            .iter()
            .map(|v| v.iter().map(|i| Script(*i)).collect::<ZeroVec<Script>>())
            .collect::<Vec<ZeroVec<Script>>>();
        let scx_vzv: VarZeroVec<ZeroSlice<Script>> =
            VarZeroVec::from(ule_scx_array_data.as_slice());

        ScriptExtensions::try_new(trie, scx_vzv).map_err(|e| {
            DataError::custom("Could not create ScriptExtensions from a trie and scx_vzv")
                .with_error_context(&e)
        })
    }
}

// source data to ICU4X provider data struct conversion
impl TryFrom<&ScriptExtensionsProperty> for ScriptExtensionsPropertyV1<'static> {
    type Error = DataError;

    fn try_from(
        scx_data: &ScriptExtensionsProperty,
    ) -> Result<ScriptExtensionsPropertyV1<'static>, DataError> {
        let scx = ScriptExtensions::try_from(scx_data);
        scx.map(|s| ScriptExtensionsPropertyV1 { data: s })
    }
}

// implement data provider
impl DataProvider<ScriptExtensionsPropertyV1Marker> for ScriptExtensionsPropertyProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<ScriptExtensionsPropertyV1Marker>, DataError> {
        if req.resource_path.key.get_last_component_no_version() != "scx" {
            return Err(DataErrorKind::MissingResourceKey.with_req(req));
        }

        let source_scx_data = &self.data;

        let data_struct = ScriptExtensionsPropertyV1::try_from(source_scx_data)?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(data_struct)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_properties::provider::key;

    #[test]
    fn test_script_val_from_script_extensions() {
        let root_dir = icu_testdata::paths::uprops_toml_root();
        let provider = ScriptExtensionsPropertyProvider::try_new(&root_dir)
            .expect("TOML should load successfully");

        let payload: DataPayload<ScriptExtensionsPropertyV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::SCRIPT_EXTENSIONS_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let scx: &ScriptExtensions = &payload.get().data;

        assert_eq!(scx.get_script_val('𐓐' as u32), Script::Osage); // U+104D0 OSAGE CAPITAL LETTER KHA
        assert_eq!(scx.get_script_val('🥳' as u32), Script::Common); // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
        assert_eq!(scx.get_script_val(0x200D), Script::Inherited); // ZERO WIDTH JOINER
        assert_eq!(scx.get_script_val('௫' as u32), Script::Tamil); // U+0BEB TAMIL DIGIT FIVE
        assert_eq!(scx.get_script_val(0x11303), Script::Grantha); // GRANTHA SIGN VISARGA
        assert_eq!(scx.get_script_val(0x30A0), Script::Common); // U+30A0 KATAKANA-HIRAGANA DOUBLE HYPHEN
    }

    #[test]
    fn test_scx_array_from_script_extensions() {
        use zerovec::ZeroVec;

        let root_dir = icu_testdata::paths::uprops_toml_root();
        let provider = ScriptExtensionsPropertyProvider::try_new(&root_dir)
            .expect("TOML should load successfully");

        let payload: DataPayload<ScriptExtensionsPropertyV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::SCRIPT_EXTENSIONS_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let scx: &ScriptExtensions = &payload.get().data;

        assert_eq!(
            scx.get_script_extensions_val('𐓐' as u32).as_zerovec(), // U+104D0 OSAGE CAPITAL LETTER KHA
            ZeroVec::<Script>::alloc_from_slice(&[Script::Osage])
        );
        assert_eq!(
            scx.get_script_extensions_val('🥳' as u32).as_zerovec(), // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
            ZeroVec::<Script>::alloc_from_slice(&[Script::Common])
        );
        assert_eq!(
            scx.get_script_extensions_val(0x200D).as_zerovec(), // ZERO WIDTH JOINER
            ZeroVec::<Script>::alloc_from_slice(&[Script::Inherited])
        );
        assert_eq!(
            scx.get_script_extensions_val('௫' as u32).as_zerovec(), // U+0BEB TAMIL DIGIT FIVE
            ZeroVec::<Script>::alloc_from_slice(&[Script::Tamil, Script::Grantha])
        );
        assert_eq!(
            scx.get_script_extensions_val(0x11303).as_zerovec(), // GRANTHA SIGN VISARGA
            ZeroVec::<Script>::alloc_from_slice(&[Script::Tamil, Script::Grantha])
        );
        assert_eq!(
            scx.get_script_extensions_val(0x30A0).as_zerovec(), // KATAKANA-HIRAGANA DOUBLE HYPHEN
            ZeroVec::<Script>::alloc_from_slice(&[Script::Hiragana, Script::Katakana])
        );
    }
}
