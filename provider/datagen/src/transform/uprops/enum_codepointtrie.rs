// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::uprops::uprops_helpers::{
    self, get_last_component_no_version, TomlEnumerated,
};

use crate::SourceData;
use icu_codepointtrie::{CodePointTrie, TrieValue};
use icu_properties::provider::*;
use icu_properties::provider::{UnicodePropertyMapV1, UnicodePropertyMapV1Marker};
use icu_properties::{
    BidiClass, CanonicalCombiningClass, EastAsianWidth, GeneralCategory, GraphemeClusterBreak,
    LineBreak, Script, SentenceBreak, WordBreak,
};
use icu_provider::datagen::IterableDynProvider;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::sync::RwLock;

/// A data provider reading from TOML files produced by the ICU4C icuexportdata tool.
///
/// This data provider returns `CodePointTrie` data inside a `UnicodePropertyMap` data struct.
pub struct EnumeratedPropertyCodePointTrieProvider {
    source: SourceData,
    data: RwLock<Option<TomlEnumerated>>,
}

impl From<&SourceData> for EnumeratedPropertyCodePointTrieProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
            data: RwLock::new(None),
        }
    }
}

// implement data provider
impl<T: TrieValue> DynProvider<UnicodePropertyMapV1Marker<T>>
    for EnumeratedPropertyCodePointTrieProvider
{
    fn load_payload(
        &self,
        key: ResourceKey,
        _: &DataRequest,
    ) -> Result<DataResponse<UnicodePropertyMapV1Marker<T>>, DataError> {
        // For data resource keys that represent the CodePointTrie data for an enumerated
        // property, the ResourceKey sub-category string will just be the short alias
        // for the property.
        let prop_name = get_last_component_no_version(key);

        if self.data.read().unwrap().is_none() {
            let data = uprops_helpers::load_enumerated_from_dir(self.source.get_uprops_root()?)?;
            *self.data.write().unwrap() = Some(data);
        }

        let guard = self.data.read().unwrap();

        let source_cpt_data = &guard
            .as_ref()
            .unwrap()
            .get(prop_name)
            .ok_or(DataErrorKind::MissingResourceKey.into_error())?
            .code_point_trie;

        let code_point_trie = CodePointTrie::try_from(source_cpt_data).map_err(|e| {
            DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
        })?;
        let data_struct = UnicodePropertyMapV1 { code_point_trie };

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(data_struct)),
        })
    }
}

icu_provider::impl_dyn_provider!(EnumeratedPropertyCodePointTrieProvider, {
    key::CANONICAL_COMBINING_CLASS_V1 => UnicodePropertyMapV1Marker<CanonicalCombiningClass>,
    key::GENERAL_CATEGORY_V1 => UnicodePropertyMapV1Marker<GeneralCategory>,
    key::BIDI_CLASS => UnicodePropertyMapV1Marker<BidiClass>,
    key::SCRIPT_V1 => UnicodePropertyMapV1Marker<Script>,
    key::EAST_ASIAN_WIDTH_V1 => UnicodePropertyMapV1Marker<EastAsianWidth>,
    key::LINE_BREAK_V1 => UnicodePropertyMapV1Marker<LineBreak>,
    key::GRAPHEME_CLUSTER_BREAK_V1 => UnicodePropertyMapV1Marker<GraphemeClusterBreak>,
    key::WORD_BREAK_V1 => UnicodePropertyMapV1Marker<WordBreak>,
    key::SENTENCE_BREAK_V1 => UnicodePropertyMapV1Marker<SentenceBreak>,
}, SERDE_SE, ITERABLE_SERDE_SE, DATA_CONVERTER);

impl<T: TrieValue> IterableDynProvider<UnicodePropertyMapV1Marker<T>>
    for EnumeratedPropertyCodePointTrieProvider
{
    fn supported_options_for_key(
        &self,
        _: ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_codepointtrie::CodePointTrie;
    use icu_properties::provider::key;
    use icu_properties::{GeneralCategory, Script};

    // A test of the UnicodeProperty General_Category is truly a test of the
    // `GeneralCategory` Rust enum, not the `GeneralCategoryGroup` Rust enum,
    // since we must match the representation and value width of the data from
    // the ICU CodePointTrie that ICU4X is reading from.
    #[test]
    fn test_general_category() {
        let provider = EnumeratedPropertyCodePointTrieProvider::from(&SourceData::for_test());

        let payload: DataPayload<UnicodePropertyMapV1Marker<GeneralCategory>> = provider
            .load_payload(key::GENERAL_CATEGORY_V1, &DataRequest::default())
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let trie: &CodePointTrie<GeneralCategory> = &payload.get().code_point_trie;

        assert_eq!(trie.get('꣓' as u32), GeneralCategory::DecimalNumber);
        assert_eq!(trie.get('≈' as u32), GeneralCategory::MathSymbol);
    }

    #[test]
    fn test_script() {
        let provider = EnumeratedPropertyCodePointTrieProvider::from(&SourceData::for_test());

        let payload: DataPayload<UnicodePropertyMapV1Marker<Script>> = provider
            .load_payload(key::SCRIPT_V1, &DataRequest::default())
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let trie: &CodePointTrie<Script> = &payload.get().code_point_trie;

        assert_eq!(trie.get('꣓' as u32), Script::Saurashtra);
        assert_eq!(trie.get('≈' as u32), Script::Common);
    }
}
