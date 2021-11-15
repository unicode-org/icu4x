// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops_helpers::{self, TomlEnumerated};
use crate::uprops_serde::enumerated::EnumeratedPropertyCodePointTrie;

use icu_codepointtrie::{CodePointTrie, CodePointTrieHeader, TrieType, TrieValue};
use icu_properties::provider::*;
use icu_properties::provider::{UnicodePropertyMapV1, UnicodePropertyMapV1Marker};
use icu_properties::{
    EastAsianWidth, GeneralSubcategory, GraphemeClusterBreak, LineBreak, Script, SentenceBreak,
    WordBreak,
};
use icu_provider::iter::IterableDataProviderCore;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::path::Path;
use zerovec::ZeroVec;

/// This data provider returns `CodePointTrie` data inside a
/// `UnicodePropertyMap` data struct. The source data is the same as that of
/// [crate::provider::PropertiesDataProvider], which is a TOML file of data
/// for the property(-ies) desired, as given by the ICU4C property data
/// exporter tool.
pub struct EnumeratedPropertyCodePointTrieProvider {
    data: TomlEnumerated,
}

impl EnumeratedPropertyCodePointTrieProvider {
    pub fn try_new(root_dir: &Path) -> eyre::Result<Self> {
        let data = uprops_helpers::load_enumerated_from_dir(root_dir)?;
        Ok(Self { data })
    }
}

impl<T: TrieValue> TryFrom<&EnumeratedPropertyCodePointTrie> for UnicodePropertyMapV1<'static, T> {
    type Error = DataError;

    fn try_from(
        cpt_data: &EnumeratedPropertyCodePointTrie,
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
            if let Some(data_8) = &cpt_data.data_8 {
                data_8.iter().map(|i| T::try_from_u32(*i as u32)).collect()
            } else if let Some(data_16) = &cpt_data.data_16 {
                data_16.iter().map(|i| T::try_from_u32(*i as u32)).collect()
            } else if let Some(data_32) = &cpt_data.data_32 {
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
        trie.map(|t| UnicodePropertyMapV1 { code_point_trie: t })
    }
}

impl<T: TrieValue> DataProvider<UnicodePropertyMapV1Marker<T>>
    for EnumeratedPropertyCodePointTrieProvider
{
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<UnicodePropertyMapV1Marker<T>>, DataError> {
        // For data resource keys that represent the CodePointTrie data for an enumerated
        // property, the ResourceKey sub-category string will just be the short alias
        // for the property.
        let prop_name = &req.resource_path.key.sub_category;
        let source_cpt_data = &self
            .data
            .get(prop_name)
            .ok_or(DataError::MissingResourceKey(req.resource_path.key))?
            .code_point_trie;

        let data_struct = UnicodePropertyMapV1::<T>::try_from(source_cpt_data)?;

        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(data_struct)),
        })
    }
}

icu_provider::impl_dyn_provider!(EnumeratedPropertyCodePointTrieProvider, {
    key::GENERAL_CATEGORY_V1 => UnicodePropertyMapV1Marker<GeneralSubcategory>,
    key::SCRIPT_V1 => UnicodePropertyMapV1Marker<Script>,
    key::EAST_ASIAN_WIDTH_V1 => UnicodePropertyMapV1Marker<EastAsianWidth>,
    key::LINE_BREAK_V1 => UnicodePropertyMapV1Marker<LineBreak>,
    key::GRAPHEME_CLUSTER_BREAK_V1 => UnicodePropertyMapV1Marker<GraphemeClusterBreak>,
    key::WORD_BREAK_V1 => UnicodePropertyMapV1Marker<WordBreak>,
    key::SENTENCE_BREAK_V1 => UnicodePropertyMapV1Marker<SentenceBreak>,
}, SERDE_SE);

impl IterableDataProviderCore for EnumeratedPropertyCodePointTrieProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_codepointtrie::CodePointTrie;
    use icu_properties::provider::key;
    use icu_properties::{GeneralSubcategory, Script};

    // A test of the UnicodeProperty General_Category is truly a test of the
    // `GeneralSubcategory` Rust enum, not the `GeneralCategory` Rust enum,
    // since we must match the representation and value width of the data from
    // the ICU CodePointTrie that ICU4X is reading from.
    #[test]
    fn test_general_category() {
        let root_dir = icu_testdata::paths::data_root().join("uprops");
        let provider = EnumeratedPropertyCodePointTrieProvider::try_new(&root_dir)
            .expect("TOML should load successfully");

        let payload: DataPayload<UnicodePropertyMapV1Marker<GeneralSubcategory>> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::GENERAL_CATEGORY_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let trie: &CodePointTrie<GeneralSubcategory> = &payload.get().code_point_trie;

        assert_eq!(trie.get('꣓' as u32), GeneralSubcategory::Digit);
        assert_eq!(trie.get('≈' as u32), GeneralSubcategory::MathSymbol);
    }

    #[test]
    fn test_script() {
        let root_dir = icu_testdata::paths::data_root().join("uprops");
        let provider = EnumeratedPropertyCodePointTrieProvider::try_new(&root_dir)
            .expect("TOML should load successfully");

        let payload: DataPayload<UnicodePropertyMapV1Marker<Script>> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::SCRIPT_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let trie: &CodePointTrie<Script> = &payload.get().code_point_trie;

        assert_eq!(trie.get('꣓' as u32), Script::Saurashtra);
        assert_eq!(trie.get('≈' as u32), Script::Common);
    }
}
