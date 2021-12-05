// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops_helpers;
use crate::uprops_serde::script::ScriptExtensionsProperty;
use icu_codepointtrie::CodePointTrie;
use icu_properties::provider::{ScriptExtensionsPropertyV1, ScriptExtensionsPropertyV1Marker};
use icu_properties::script::{ScriptExtensions, ScriptWithExt};
use icu_properties::Script;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::path::Path;
use tinystr::tinystr16;
use zerovec::ule::{AsULE, PlainOldULE};
use zerovec::zerovec::ZeroVecULE;
use zerovec::VarZeroVec;

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

// source data to ICU4X plain/raw/utility data structure
impl TryFrom<&ScriptExtensionsProperty> for ScriptExtensions<'static> {
    type Error = DataError;

    fn try_from(
        scx_data: &ScriptExtensionsProperty,
    ) -> Result<ScriptExtensions<'static>, DataError> {
        let cpt_data = &scx_data.code_point_trie;
        let scx_array_data = &scx_data.script_code_array;

        let trie = CodePointTrie::<ScriptWithExt>::try_from(cpt_data)?;

        // Convert the input from Vec<Vec<u16>> to Vec<Vec<PlainOldULE<2>>> so that
        // we can go through the VarZeroVec construction process for a desired result
        // type of VZV<ZeroVecULE<T::ULE>>
        let ule_scx_array_data: Vec<Vec<PlainOldULE<2>>> = scx_array_data
            .iter()
            .map(|v| {
                v.iter()
                    .map(|i| ScriptWithExt(*i).as_unaligned())
                    .collect::<Vec<PlainOldULE<2>>>()
            })
            .collect::<Vec<Vec<PlainOldULE<2>>>>();
        let bytes =
            VarZeroVec::<ZeroVecULE<Script>>::get_serializable_bytes(&ule_scx_array_data).unwrap();
        let scx_vzv: VarZeroVec<ZeroVecULE<Script>> = VarZeroVec::parse_byte_slice(&bytes)
            .map_err(DataError::new_resc_error)?
            .into_owned();

        ScriptExtensions::try_new(trie, scx_vzv).map_err(DataError::new_resc_error)
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
        debug_assert_eq!(&tinystr16!("scx"), &req.resource_path.key.sub_category);

        let source_scx_data = &self.data;

        let data_struct = ScriptExtensionsPropertyV1::try_from(source_scx_data)?;

        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
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

        // TODO: why does this test pass? U+1DFA is in ScriptExtensions.txt, but where is it in Scripts.txt?

        assert_eq!(scx.get_script_val(0x1DFA), Script::Syriac); // U+1DFA COMBINING DOT BELOW LEFT

        // TODO: why doesn't this test pass? U+30A0 is listed as Common in ScriptExtensions.txt and Scripts.txt

        assert_eq!(scx.get_script_val(0x30A0), Script::Common); // U+30A0 KATAKANA-HIRAGANA DOUBLE HYPHEN
        
    }
}
