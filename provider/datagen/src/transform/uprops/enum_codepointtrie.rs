// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::uprops::uprops_helpers::{self, TomlEnumerated};

use crate::SourceData;
use icu_codepointtrie::CodePointTrie;
use icu_properties::provider::*;
use icu_provider::datagen::*;
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

macro_rules! expand {
    ($(($marker:ident, $prop_name:literal)),+,) => {
        $(
            impl ResourceProvider<$marker> for EnumeratedPropertyCodePointTrieProvider
            {
                fn load_resource(&self, _: &DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    if self.data.read().unwrap().is_none() {
                        let data = uprops_helpers::load_enumerated_from_dir(self.source.get_uprops_root()?)?;
                        *self.data.write().unwrap() = Some(data);
                    }

                    let guard = self.data.read().unwrap();

                    let source_cpt_data = &guard
                        .as_ref()
                        .unwrap()
                        .get($prop_name)
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

            impl IterableResourceProvider<$marker> for EnumeratedPropertyCodePointTrieProvider {
                fn supported_options(
                    &self,
                ) -> Result<Vec<ResourceOptions>, DataError> {
                    Ok(vec![Default::default()])
                }
            }
        )+

        icu_provider::make_exportable_provider!(EnumeratedPropertyCodePointTrieProvider, [$($marker),+,]);
    };
}

expand!(
    (CanonicalCombiningClassV1Marker, "ccc"),
    (GeneralCategoryV1Marker, "gc"),
    (BidiClassV1Marker, "bc"),
    (ScriptV1Marker, "sc"),
    (EastAsianWidthV1Marker, "ea"),
    (LineBreakV1Marker, "lb"),
    (GraphemeClusterBreakV1Marker, "GCB"),
    (WordBreakV1Marker, "WB"),
    (SentenceBreakV1Marker, "SB"),
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu_codepointtrie::CodePointTrie;
    use icu_properties::provider::{GeneralCategoryV1Marker, ScriptV1Marker};
    use icu_properties::{GeneralCategory, Script};

    // A test of the UnicodeProperty General_Category is truly a test of the
    // `GeneralCategory` Rust enum, not the `GeneralCategoryGroup` Rust enum,
    // since we must match the representation and value width of the data from
    // the ICU CodePointTrie that ICU4X is reading from.
    #[test]
    fn test_general_category() {
        let provider = EnumeratedPropertyCodePointTrieProvider::from(&SourceData::for_test());

        let payload: DataPayload<GeneralCategoryV1Marker> = provider
            .load_resource(&DataRequest::default())
            .and_then(DataResponse::take_payload)
            .expect("Loading was successful");

        let trie: &CodePointTrie<GeneralCategory> = &payload.get().code_point_trie;

        assert_eq!(trie.get('꣓' as u32), GeneralCategory::DecimalNumber);
        assert_eq!(trie.get('≈' as u32), GeneralCategory::MathSymbol);
    }

    #[test]
    fn test_script() {
        let provider = EnumeratedPropertyCodePointTrieProvider::from(&SourceData::for_test());

        let payload: DataPayload<ScriptV1Marker> = provider
            .load_resource(&DataRequest::default())
            .and_then(DataResponse::take_payload)
            .expect("Loading was successful");

        let trie: &CodePointTrie<Script> = &payload.get().code_point_trie;

        assert_eq!(trie.get('꣓' as u32), Script::Saurashtra);
        assert_eq!(trie.get('≈' as u32), Script::Common);
    }
}
