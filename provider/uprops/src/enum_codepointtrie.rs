// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::uprops_serde;
use crate::uprops_serde::enumerated::EnumeratedPropertyCodePointTrie;

use icu_codepointtrie::codepointtrie::{CodePointTrie, TrieValue};
use icu_codepointtrie::provider::{UnicodePropertyMapV1, UnicodePropertyMapV1Marker};
use icu_provider::prelude::*;
use icu_uniset::enum_props::EnumeratedProperty;  // TODO(#1160) - Refactor property definitions out of UnicodeSet

use std::fs;
use std::path::PathBuf;

pub struct EnumeratedPropertyCodePointTrieProvider {
    root_dir: PathBuf,
}

impl EnumeratedPropertyCodePointTrieProvider {
    pub fn new(root_dir: PathBuf) -> Self {
        EnumeratedPropertyCodePointTrieProvider { root_dir }
    }

    fn get_toml_data(&self, name: &str) -> Result<uprops_serde::enumerated::Main, Error> {
        let mut path: PathBuf = self.root_dir.clone().join(name);
        path.set_extension("toml");
        let toml_str = fs::read_to_string(&path).map_err(|e| Error::Io(e, path.clone()))?;
        toml::from_str(&toml_str).map_err(|e| Error::Toml(e, path))
    }
}

impl<T: TrieValue> From<uprops_serde::enumerated::EnumeratedPropertyCodePointTrie> for UnicodePropertyMapV1<'static, T> {
    fn from(cpt_data: EnumeratedPropertyCodePointTrie) -> UnicodePropertyMapV1<'static, T> {
        let trie = CodePointTrie::<T>::try_new(
            // TODO
        );
    }
}

impl<'data, T: TrieValue> DataProvider<'data, UnicodePropertyMapV1Marker<T>> 
    for EnumeratedPropertyCodePointTrieProvider
{
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, UnicodePropertyMapV1Marker<T>>, DataError> {
        // For data resource keys that represent the CodePointTrie data for an enumerated
        // property, the ResourceKey sub-category string will just be the short alias
        // for the property.
        let prop_name = &req.resource_path.key.sub_category;

        let toml_data: uprops_serde::enumerated::Main = self
        .get_toml_data(prop_name)
        .map_err(DataError::new_resc_error)?;

        let prop_enum: EnumeratedProperty = EnumeratedProperty::from(prop_name);

        let source_cpt_data: uprops_serde::enumerated::EnumeratedPropertyCodePointTrie = 
            toml_data.enum_property.data.code_point_trie;

    }
}