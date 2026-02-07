// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::collections::codepointtrie::{CodePointTrie, TrieValue};
use icu::properties::props::EnumeratedProperty;
use icu::properties::provider::{names::*, *};
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::ule::NichedOption;

impl SourceDataProvider {
    pub(super) fn get_enumerated_prop<'a>(
        &'a self,
        name: &str,
        short_name: &str,
    ) -> Result<&'a super::uprops_serde::enumerated::EnumeratedPropertyMap, DataError> {
        let data = self.icuexport()?
            .read_and_parse_toml::<super::uprops_serde::enumerated::Main>(&format!(
                "uprops/{}/{}.toml",
                self.trie_type(),
                short_name
            ))?
            .enum_property
            .first()
            .ok_or_else(|| DataError::custom("Loading icuexport property data failed: \
                                            Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)"))?;

        if name != data.long_name || short_name != data.short_name {
            return Err(DataError::custom("Property name mismatch").with_display_context(name));
        }

        Ok(data)
    }

    fn get_mask_prop<'a>(
        &'a self,
        name: &str,
        short_name: &str,
        mask_for: &str,
    ) -> Result<&'a super::uprops_serde::mask::MaskPropertyMap, DataError> {
        let data = self
            .icuexport()?
            .read_and_parse_toml::<super::uprops_serde::mask::Main>(&format!(
                "uprops/{}/{}.toml",
                self.trie_type(),
                short_name
            ))?
            .mask_property
            .first()
            .ok_or(DataError::custom(
                "Loading icuexport property data failed: \
                 Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)",
            ))?;

        if data.long_name != name || data.short_name != short_name || data.mask_for != mask_for {
            return Err(DataError::custom("Property name mismatch")
                .with_marker(PropertyNameParseGeneralCategoryMaskV1::INFO));
        }

        Ok(data)
    }
}

impl super::uprops_serde::enumerated::EnumeratedPropertyMap {
    #[cfg_attr(
        any(feature = "use_wasm", feature = "use_icu4c"),
        allow(clippy::unnecessary_wraps)
    )]
    pub(crate) fn build_codepointtrie<T: TrieValue + Copy>(
        &self,
    ) -> Result<CodePointTrie<'static, T>, DataError> {
        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
            return Err(DataError::custom(
                "icu_provider_source must be built with `use_icu4c` or `use_wasm` to build enumerated properties data",
            ));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        {
            use icu::collections::codepointtrie::TrieType;
            use icu_codepointtrie_builder::CodePointTrieBuilder;

            let mut builder = CodePointTrieBuilder::new(
                T::try_from_u32(0).ok().unwrap(),
                T::try_from_u32(0).ok().unwrap(),
                TrieType::Small,
            );

            for range in &self.ranges {
                builder.set_range_value(
                    range.a..=range.b,
                    T::try_from_u32(range.v as u32).ok().unwrap(),
                );
            }

            Ok(builder.build())
        }
    }

    pub(crate) fn names_to_values(&self) -> BTreeMap<&str, u16> {
        let mut map = BTreeMap::new();

        for range in &self.ranges {
            if let Some(name) = range.name.as_deref() {
                map.insert(name, range.v);
            }
        }

        for value in &self.values {
            map.insert(value.long.as_str(), value.discr);
            if let Some(ref short) = value.short {
                map.insert(short.as_str(), value.discr);
            }
            for alias in &value.aliases {
                map.insert(alias.as_str(), value.discr);
            }
        }

        map
    }

    pub(crate) fn values_to_names_long(&self) -> BTreeMap<u16, &str> {
        let mut map: BTreeMap<_, &str> = BTreeMap::new();

        for range in &self.ranges {
            if let Some(name) = range.name.as_deref() {
                map.insert(range.v, name);
            }
        }

        for value in &self.values {
            map.insert(value.discr, &value.long);
        }

        map
    }

    pub(crate) fn values_to_names_short(&self) -> BTreeMap<u16, &str> {
        let mut map: BTreeMap<_, &str> = BTreeMap::new();

        for range in &self.ranges {
            if let Some(name) = range.name.as_deref() {
                map.insert(range.v, name);
            }
        }

        for value in &self.values {
            if let Some(ref short) = value.short {
                map.insert(value.discr, short);
            }
        }

        map
    }
}

fn validate_dense(map: &BTreeMap<u16, &str>) -> Result<(), DataError> {
    if let Some((&first, _)) = map.first_key_value() {
        if first > 0 {
            return Err(DataError::custom(
                "Property has nonzero starting discriminant, perhaps consider \
                 storing its names as a sparse map or by specializing this error",
            )
            .with_display_context(&first));
        }
    } else {
        return Err(DataError::custom("Property has no values!"));
    };
    if let Some((&last, _)) = map.last_key_value() {
        let range = usize::from(1 + last);
        let count = map.len();
        let gaps = range - count;
        if gaps > 0 {
            return Err(DataError::custom(
                "Property has more than 0 gaps, \
                perhaps consider storing its names in a sparse map or by specializing this error",
            )
            .with_display_context(&gaps));
        }
    } else {
        return Err(DataError::custom("Property has no values!"));
    };
    Ok(())
}

#[allow(clippy::unnecessary_wraps)] // signature required by macro
fn convert_sparse(
    map: BTreeMap<u16, &str>,
) -> Result<PropertyEnumToValueNameSparseMap<'static>, DataError> {
    Ok(PropertyEnumToValueNameSparseMap {
        map: map.into_iter().collect(),
    })
}

fn convert_linear(
    map: BTreeMap<u16, &str>,
) -> Result<PropertyEnumToValueNameLinearMap<'static>, DataError> {
    validate_dense(&map)?;

    Ok(PropertyEnumToValueNameLinearMap {
        map: (&map.into_values().collect::<Vec<_>>()).into(),
    })
}

fn convert_script(
    map: BTreeMap<u16, &str>,
) -> Result<PropertyScriptToIcuScriptMap<'static>, DataError> {
    validate_dense(&map)?;

    Ok(PropertyScriptToIcuScriptMap {
        map: map
            .into_values()
            .map(|s| {
                if s.is_empty() {
                    Ok(NichedOption(None))
                } else {
                    icu::locale::subtags::Script::try_from_str(s)
                        .map(Some)
                        .map(NichedOption)
                }
            })
            .collect::<Result<_, _>>()
            .map_err(|_| DataError::custom("Found invalid script tag"))?,
    })
}

macro_rules! expand {
    ($(
        (
            $prop:ty,
            $marker:ident,
            $parse_marker:ident,
            $short_marker:ident[$short_convert:ident],
            $long_marker:ident[$long_convert:ident]
        )
    ),+,) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider
            {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let data = self.get_enumerated_prop(
                        core::str::from_utf8(<$prop as EnumeratedProperty>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as EnumeratedProperty>::SHORT_NAME).unwrap()
                    )?;
                    let trie = data.build_codepointtrie()?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(PropertyCodePointMap::CodePointTrie(trie)),
                    })
                }
            }

            impl DataProvider<$parse_marker> for SourceDataProvider
            {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$parse_marker>, DataError> {
                    self.check_req::<$parse_marker>(req)?;
                    let data = self.get_enumerated_prop(
                        core::str::from_utf8(<$prop as EnumeratedProperty>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as EnumeratedProperty>::SHORT_NAME).unwrap()
                    )?;
                    let trie = data.names_to_values()
                        .into_iter()
                        .map(|(k, v)| (k, v as usize))
                        .collect::<ZeroTrieSimpleAscii<_>>()
                        .convert_store();

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(PropertyValueNameToEnumMap { map: trie }),
                    })
                }
            }

            impl DataProvider<$short_marker> for SourceDataProvider
            {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$short_marker>, DataError> {
                    self.check_req::<$short_marker>(req)?;
                    let data = self.get_enumerated_prop(
                        core::str::from_utf8(<$prop as EnumeratedProperty>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as EnumeratedProperty>::SHORT_NAME).unwrap()
                    )?;
                    let map = ($short_convert)(data.values_to_names_short())?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(map),
                    })
                }
            }

            impl DataProvider<$long_marker> for SourceDataProvider
            {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$long_marker>, DataError> {
                    self.check_req::<$long_marker>(req)?;
                    let data = self.get_enumerated_prop(
                        core::str::from_utf8(<$prop as EnumeratedProperty>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as EnumeratedProperty>::SHORT_NAME).unwrap()
                    )?;
                    let map = ($long_convert)(data.values_to_names_long())?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(map),
                    })
                }
            }

            impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                    Ok(HashSet::from_iter([Default::default()]))
                }
            }

            impl crate::IterableDataProviderCached<$parse_marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                    Ok(HashSet::from_iter([Default::default()]))
                }
            }

            impl crate::IterableDataProviderCached<$short_marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                    Ok(HashSet::from_iter([Default::default()]))
                }
            }

            impl crate::IterableDataProviderCached<$long_marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                    Ok(HashSet::from_iter([Default::default()]))
                }
            }

        )+
    }
}

// Special handling for GeneralCategoryMask
impl DataProvider<PropertyNameParseGeneralCategoryMaskV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<PropertyNameParseGeneralCategoryMaskV1>, DataError> {
        use icu::properties::props::GeneralCategoryGroup;

        self.check_req::<PropertyNameParseGeneralCategoryMaskV1>(req)?;

        let data = self.get_mask_prop("General_Category_Mask", "gcm", "General_Category")?;

        let mut map = BTreeMap::new();

        for value in &data.values {
            let packed = TrieValue::to_u32(GeneralCategoryGroup::from(value.discr)) as usize;

            // sentinel value
            if packed == 0xFF00 {
                return Err(DataError::custom("Found unknown general category mask value, properties code may need to be updated."));
            }

            map.insert(value.long.as_str(), packed);
            if let Some(ref short) = value.short {
                map.insert(short.as_str(), packed);
            }
            for alias in &value.aliases {
                map.insert(alias.as_str(), packed);
            }
        }

        let trie = map
            .into_iter()
            .collect::<ZeroTrieSimpleAscii<_>>()
            .convert_store();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(PropertyValueNameToEnumMap { map: trie }),
        })
    }
}

impl crate::IterableDataProviderCached<PropertyNameParseGeneralCategoryMaskV1>
    for SourceDataProvider
{
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

expand!(
    (
        icu::properties::props::CanonicalCombiningClass,
        PropertyEnumCanonicalCombiningClassV1,
        PropertyNameParseCanonicalCombiningClassV1,
        PropertyNameShortCanonicalCombiningClassV1[convert_sparse],
        PropertyNameLongCanonicalCombiningClassV1[convert_sparse]
    ),
    (
        icu::properties::props::GeneralCategory,
        PropertyEnumGeneralCategoryV1,
        PropertyNameParseGeneralCategoryV1,
        PropertyNameShortGeneralCategoryV1[convert_linear],
        PropertyNameLongGeneralCategoryV1[convert_linear]
    ),
    (
        icu::properties::props::BidiClass,
        PropertyEnumBidiClassV1,
        PropertyNameParseBidiClassV1,
        PropertyNameShortBidiClassV1[convert_linear],
        PropertyNameLongBidiClassV1[convert_linear]
    ),
    (
        icu::properties::props::NumericType,
        PropertyEnumNumericTypeV1,
        PropertyNameParseNumericTypeV1,
        PropertyNameShortNumericTypeV1[convert_linear],
        PropertyNameLongNumericTypeV1[convert_linear]
    ),
    (
        icu::properties::props::Script,
        PropertyEnumScriptV1,
        PropertyNameParseScriptV1,
        PropertyNameShortScriptV1[convert_script],
        PropertyNameLongScriptV1[convert_linear]
    ),
    (
        icu::properties::props::HangulSyllableType,
        PropertyEnumHangulSyllableTypeV1,
        PropertyNameParseHangulSyllableTypeV1,
        PropertyNameShortHangulSyllableTypeV1[convert_linear],
        PropertyNameLongHangulSyllableTypeV1[convert_linear]
    ),
    (
        icu::properties::props::EastAsianWidth,
        PropertyEnumEastAsianWidthV1,
        PropertyNameParseEastAsianWidthV1,
        PropertyNameShortEastAsianWidthV1[convert_linear],
        PropertyNameLongEastAsianWidthV1[convert_linear]
    ),
    (
        icu::properties::props::IndicSyllabicCategory,
        PropertyEnumIndicSyllabicCategoryV1,
        PropertyNameParseIndicSyllabicCategoryV1,
        PropertyNameShortIndicSyllabicCategoryV1[convert_linear],
        PropertyNameLongIndicSyllabicCategoryV1[convert_linear]
    ),
    (
        icu::properties::props::IndicConjunctBreak,
        PropertyEnumIndicConjunctBreakV1,
        PropertyNameParseIndicConjunctBreakV1,
        PropertyNameShortIndicConjunctBreakV1[convert_linear],
        PropertyNameLongIndicConjunctBreakV1[convert_linear]
    ),
    (
        icu::properties::props::LineBreak,
        PropertyEnumLineBreakV1,
        PropertyNameParseLineBreakV1,
        PropertyNameShortLineBreakV1[convert_linear],
        PropertyNameLongLineBreakV1[convert_linear]
    ),
    (
        icu::properties::props::GraphemeClusterBreak,
        PropertyEnumGraphemeClusterBreakV1,
        PropertyNameParseGraphemeClusterBreakV1,
        PropertyNameShortGraphemeClusterBreakV1[convert_linear],
        PropertyNameLongGraphemeClusterBreakV1[convert_linear]
    ),
    (
        icu::properties::props::WordBreak,
        PropertyEnumWordBreakV1,
        PropertyNameParseWordBreakV1,
        PropertyNameShortWordBreakV1[convert_linear],
        PropertyNameLongWordBreakV1[convert_linear]
    ),
    (
        icu::properties::props::SentenceBreak,
        PropertyEnumSentenceBreakV1,
        PropertyNameParseSentenceBreakV1,
        PropertyNameShortSentenceBreakV1[convert_linear],
        PropertyNameLongSentenceBreakV1[convert_linear]
    ),
    (
        icu::properties::props::JoiningType,
        PropertyEnumJoiningTypeV1,
        PropertyNameParseJoiningTypeV1,
        PropertyNameShortJoiningTypeV1[convert_linear],
        PropertyNameLongJoiningTypeV1[convert_linear]
    ),
    (
        icu::properties::props::JoiningGroup,
        PropertyEnumJoiningGroupV1,
        PropertyNameParseJoiningGroupV1,
        PropertyNameShortJoiningGroupV1[convert_linear],
        PropertyNameLongJoiningGroupV1[convert_linear]
    ),
    (
        icu::properties::props::VerticalOrientation,
        PropertyEnumVerticalOrientationV1,
        PropertyNameParseVerticalOrientationV1,
        PropertyNameShortVerticalOrientationV1[convert_linear],
        PropertyNameLongVerticalOrientationV1[convert_linear]
    ),
);

#[cfg(test)]
mod tests {
    use super::*;

    // A test of the UnicodeProperty General_Category is truly a test of the
    // `GeneralCategory` Rust enum, not the `GeneralCategoryGroup` Rust enum,
    // since we must match the representation and value width of the data from
    // the ICU CodePointTrie that ICU4X is reading from.
    #[test]
    fn test_general_category() {
        use icu::properties::{props::GeneralCategory, CodePointMapData};
        let provider = SourceDataProvider::new_testing();

        let trie = CodePointMapData::<GeneralCategory>::try_new_unstable(&provider).unwrap();
        let trie = trie.as_code_point_trie().unwrap();

        assert_eq!(trie.get32('꣓' as u32), GeneralCategory::DecimalNumber);
        assert_eq!(trie.get32('≈' as u32), GeneralCategory::MathSymbol);
    }

    #[test]
    fn test_script() {
        use icu::properties::{props::Script, CodePointMapData};
        let provider = SourceDataProvider::new_testing();

        let trie = CodePointMapData::<Script>::try_new_unstable(&provider).unwrap();
        let trie = trie.as_code_point_trie().unwrap();

        assert_eq!(trie.get32('꣓' as u32), Script::Saurashtra);
        assert_eq!(trie.get32('≈' as u32), Script::Common);
    }
}
