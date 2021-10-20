// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::uprops_serde;
use crate::uprops_serde::enumerated::EnumeratedPropertyCodePointTrie;

use icu_codepointtrie::codepointtrie::{CodePointTrie, CodePointTrieHeader, TrieType, TrieValue};
use icu_properties::provider::{UnicodePropertyMapV1, UnicodePropertyMapV1Marker};
use icu_provider::prelude::*;
use zerovec::ZeroVec;

use core::convert::TryFrom;

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

impl<T: TrieValue> TryFrom<uprops_serde::enumerated::EnumeratedPropertyCodePointTrie>
    for UnicodePropertyMapV1<'static, T>
{
    type Error = DataError;

    fn try_from(
        cpt_data: EnumeratedPropertyCodePointTrie,
    ) -> Result<UnicodePropertyMapV1<'static, T>, DataError> {
        let trie_type_enum: TrieType =
            TrieType::try_from(cpt_data.trie_type_enum_val).map_err(DataError::new_resc_error)?;
        let header = CodePointTrieHeader {
            high_start: cpt_data.high_start,
            shifted12_high_start: cpt_data.shifted12_high_start,
            index3_null_offset: cpt_data.index3_null_offset,
            data_null_offset: cpt_data.data_null_offset,
            null_value: cpt_data.null_value,
            trie_type: trie_type_enum,
        };
        let index: ZeroVec<u16> = ZeroVec::clone_from_slice(&cpt_data.index);
        let data: Result<ZeroVec<'static, T>, T::TryFromU32Error> =
            if let Some(data_8) = cpt_data.data_8 {
                data_8.iter().map(|i| T::try_from_u32(*i as u32)).collect()
            } else if let Some(data_16) = cpt_data.data_16 {
                data_16.iter().map(|i| T::try_from_u32(*i as u32)).collect()
            } else if let Some(data_32) = cpt_data.data_32 {
                data_32.iter().map(|i| T::try_from_u32(*i as u32)).collect()
            } else {
                return Err(DataError::new_resc_error(
                    icu_codepointtrie::error::Error::FromDeserialized {
                        reason: "Did not find data array for CodePointTrie in TOML",
                    },
                ));
            };

        let data = data.map_err(DataError::new_resc_error)?;
        let trie =
            CodePointTrie::<T>::try_new(header, index, data).map_err(DataError::new_resc_error);
        trie.map(|t| UnicodePropertyMapV1 { codepoint_trie: t })
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

        let source_cpt_data: uprops_serde::enumerated::EnumeratedPropertyCodePointTrie =
            toml_data.enum_property.data.code_point_trie;

        let data_struct = UnicodePropertyMapV1::<T>::try_from(source_cpt_data)?;

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
    use icu_codepointtrie::codepointtrie::CodePointTrie;
    use icu_properties::provider::key;
    use icu_properties::GeneralSubcategory;

    // A test of the UnicodeProperty General_Category is truly a test of the
    // `GeneralSubcategory` Rust enum, not the `GeneralCategory` Rust enum,
    // since we must match the representation and value width of the data from
    // the ICU CodePointTrie that ICU4X is reading from.
    #[test]
    fn test_general_category() {
        let root_dir = icu_testdata::paths::data_root().join("uprops");
        let provider = EnumeratedPropertyCodePointTrieProvider::new(root_dir);

        let payload: DataPayload<'_, UnicodePropertyMapV1Marker<GeneralSubcategory>> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::GENERAL_CATEGORY_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let trie: &CodePointTrie<GeneralSubcategory> = &payload.get().codepoint_trie;

        assert_eq!(trie.get('꣓' as u32), GeneralSubcategory::Digit);
        assert_eq!(trie.get('≈' as u32), GeneralSubcategory::MathSymbol);
    }
}
