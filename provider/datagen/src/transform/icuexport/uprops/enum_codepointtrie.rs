// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceData;
use icu_collections::codepointtrie::CodePointTrie;
use icu_properties::provider::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::convert::TryFrom;

fn get_enumerated_prop<'a>(
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
    ($(($marker:ident, $names_marker:ident, $prop_name:literal)),+,) => {
        $(
            impl DataProvider<$marker> for crate::DatagenProvider
            {
                fn load(&self, _: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    let source_cpt_data = &get_enumerated_prop(&self.source, $prop_name)?.code_point_trie;

                    let code_point_trie = CodePointTrie::try_from(source_cpt_data).map_err(|e| {
                        DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
                    })?;
                    let data_struct = PropertyCodePointMapV1::CodePointTrie(code_point_trie);
                    Ok(DataResponse {
                        metadata: DataResponseMetadata::default(),
                        payload: Some(DataPayload::from_owned(data_struct)),
                    })
                }
            }

            impl IterableDataProvider<$marker> for crate::DatagenProvider {
                fn supported_locales(
                    &self,
                ) -> Result<Vec<DataLocale>, DataError> {
                    get_enumerated_prop(&self.source, $prop_name)?;
                    Ok(vec![Default::default()])
                }
            }

            impl DataProvider<$names_marker> for crate::DatagenProvider
            {
                fn load(&self, _: DataRequest) -> Result<DataResponse<$names_marker>, DataError> {
                    let data = get_enumerated_prop(&self.source, $prop_name)
                        .map_err(|_| DataError::custom("Loading icuexport property data failed: \
                                                        Are you using a sufficiently recent icuexport?"))?;
                    let mut map = BTreeMap::new();
                    for value in &data.values {
                        let discr = value.discr;
                        map.insert(NormalizedPropertyNameStr::boxed_from_bytes(value.long.as_bytes()), discr);
                        if let Some(ref short) = value.short {
                            map.insert(NormalizedPropertyNameStr::boxed_from_bytes(short.as_bytes()), discr);
                        }
                        for alias in &value.aliases {
                            map.insert(NormalizedPropertyNameStr::boxed_from_bytes(alias.as_bytes()), discr);
                        }
                    }
                    let data_struct = PropertyValueNameToEnumMapV1 { map: map.into_iter().collect() };
                    Ok(DataResponse {
                        metadata: DataResponseMetadata::default(),
                        payload: Some(DataPayload::from_owned(data_struct)),
                    })
                }
            }

            impl IterableDataProvider<$names_marker> for crate::DatagenProvider {
                fn supported_locales(
                    &self,
                ) -> Result<Vec<DataLocale>, DataError> {
                    get_enumerated_prop(&self.source, $prop_name)?;
                    Ok(vec![Default::default()])
                }
            }
        )+
    };
}

expand!(
    (
        CanonicalCombiningClassV1Marker,
        CanonicalCombiningClassNameToValueV1Marker,
        "ccc"
    ),
    (
        GeneralCategoryV1Marker,
        GeneralCategoryNameToValueV1Marker,
        "gc"
    ),
    (BidiClassV1Marker, BidiClassNameToValueV1Marker, "bc"),
    (ScriptV1Marker, ScriptNameToValueV1Marker, "sc"),
    (
        EastAsianWidthV1Marker,
        EastAsianWidthNameToValueV1Marker,
        "ea"
    ),
    (LineBreakV1Marker, LineBreakNameToValueV1Marker, "lb"),
    (
        GraphemeClusterBreakV1Marker,
        GraphemeClusterBreakNameToValueV1Marker,
        "GCB"
    ),
    (WordBreakV1Marker, WordBreakNameToValueV1Marker, "WB"),
    (
        SentenceBreakV1Marker,
        SentenceBreakNameToValueV1Marker,
        "SB"
    ),
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu_collections::codepointtrie::CodePointTrie;
    use icu_properties::provider::{
        GeneralCategoryV1Marker, PropertyCodePointMapV1, ScriptV1Marker,
    };
    use icu_properties::{GeneralCategory, Script};

    // A test of the UnicodeProperty General_Category is truly a test of the
    // `GeneralCategory` Rust enum, not the `GeneralCategoryGroup` Rust enum,
    // since we must match the representation and value width of the data from
    // the ICU CodePointTrie that ICU4X is reading from.
    #[test]
    fn test_general_category() {
        let provider = crate::DatagenProvider::for_test();

        let payload: DataPayload<GeneralCategoryV1Marker> = provider
            .load(Default::default())
            .and_then(DataResponse::take_payload)
            .expect("Loading was successful");

        let trie: &CodePointTrie<GeneralCategory> = match payload.get() {
            PropertyCodePointMapV1::CodePointTrie(ref t) => t,
            _ => unreachable!("Should have serialized to a code point trie"),
        };

        assert_eq!(trie.get32('꣓' as u32), GeneralCategory::DecimalNumber);
        assert_eq!(trie.get32('≈' as u32), GeneralCategory::MathSymbol);
    }

    #[test]
    fn test_script() {
        let provider = crate::DatagenProvider::for_test();

        let payload: DataPayload<ScriptV1Marker> = provider
            .load(Default::default())
            .and_then(DataResponse::take_payload)
            .expect("Loading was successful");

        let trie: &CodePointTrie<Script> = match payload.get() {
            PropertyCodePointMapV1::CodePointTrie(ref t) => t,
            _ => unreachable!("Should have serialized to a code point trie"),
        };
        assert_eq!(trie.get32('꣓' as u32), Script::Saurashtra);
        assert_eq!(trie.get32('≈' as u32), Script::Common);
    }
}
