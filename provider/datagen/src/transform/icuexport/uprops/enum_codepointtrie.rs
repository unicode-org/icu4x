// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceData;
use icu_codepointtrie::CodePointTrie;
use icu_properties::provider::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::convert::TryFrom;

/// A data provider reading from TOML files produced by the ICU4C icuexportdata tool.
///
/// This data provider returns `CodePointTrie` data inside a `UnicodePropertyMap` data struct.
pub struct EnumeratedPropertyCodePointTrieProvider {
    source: SourceData,
}

impl From<&SourceData> for EnumeratedPropertyCodePointTrieProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

fn get_enumerated<'a>(
    source: &'a SourceData,
    key: &str,
) -> Result<&'a super::uprops_serde::enumerated::EnumeratedPropertyMap, DataError> {
    source
        .icuexport()?
        .read_and_parse_toml::<super::uprops_serde::enumerated::Main>(&format!(
            "uprops/{}/{}.toml",
            source.trie_type(),
            key
        ))?
        .enum_property
        .get(0)
        .ok_or_else(|| DataErrorKind::MissingDataKey.into_error())
}

macro_rules! expand {
    ($(($marker:ident, $prop_name:literal)),+,) => {
        $(
            impl DataProvider<$marker> for EnumeratedPropertyCodePointTrieProvider
            {
                fn load(&self, _: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    let source_cpt_data = &get_enumerated(&self.source, $prop_name)?.code_point_trie;

                    let code_point_trie = CodePointTrie::try_from(source_cpt_data).map_err(|e| {
                        DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
                    })?;
                    let data_struct = UnicodePropertyMapV1::CodePointTrie(code_point_trie);
                    Ok(DataResponse {
                        metadata: DataResponseMetadata::default(),
                        payload: Some(DataPayload::from_owned(data_struct)),
                    })
                }
            }

            impl IterableDataProvider<$marker> for EnumeratedPropertyCodePointTrieProvider {
                fn supported_locales(
                    &self,
                ) -> Result<Vec<DataLocale>, DataError> {
                    get_enumerated(&self.source, $prop_name)?;
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
    use icu_properties::provider::{GeneralCategoryV1Marker, ScriptV1Marker, UnicodePropertyMapV1};
    use icu_properties::{GeneralCategory, Script};

    // A test of the UnicodeProperty General_Category is truly a test of the
    // `GeneralCategory` Rust enum, not the `GeneralCategoryGroup` Rust enum,
    // since we must match the representation and value width of the data from
    // the ICU CodePointTrie that ICU4X is reading from.
    #[test]
    fn test_general_category() {
        let provider = EnumeratedPropertyCodePointTrieProvider::from(&SourceData::for_test());

        let payload: DataPayload<GeneralCategoryV1Marker> = provider
            .load(Default::default())
            .and_then(DataResponse::take_payload)
            .expect("Loading was successful");

        let trie: &CodePointTrie<GeneralCategory> = match payload.get() {
            UnicodePropertyMapV1::CodePointTrie(ref t) => t,
            _ => unreachable!("Should have serialized to a code point trie"),
        };

        assert_eq!(trie.get('꣓' as u32), GeneralCategory::DecimalNumber);
        assert_eq!(trie.get('≈' as u32), GeneralCategory::MathSymbol);
    }

    #[test]
    fn test_script() {
        let provider = EnumeratedPropertyCodePointTrieProvider::from(&SourceData::for_test());

        let payload: DataPayload<ScriptV1Marker> = provider
            .load(Default::default())
            .and_then(DataResponse::take_payload)
            .expect("Loading was successful");

        let trie: &CodePointTrie<Script> = match payload.get() {
            UnicodePropertyMapV1::CodePointTrie(ref t) => t,
            _ => unreachable!("Should have serialized to a code point trie"),
        };
        assert_eq!(trie.get('꣓' as u32), Script::Saurashtra);
        assert_eq!(trie.get('≈' as u32), Script::Common);
    }
}
