// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceData;
use icu_collections::codepointtrie::CodePointTrie;
use icu_properties::provider::{names::*, *};
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::convert::TryFrom;

pub(crate) fn get_enumerated_prop<'a>(
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

fn get_prop_values_map<F>(
    values: &[super::uprops_serde::PropertyValue],
    transform_u32: F,
) -> Result<PropertyValueNameToEnumMapV1<'static>, DataError>
where
    F: Fn(u32) -> Result<u16, DataError>,
{
    let mut map = BTreeMap::new();
    for value in values {
        let discr = transform_u32(value.discr)?;
        map.insert(
            NormalizedPropertyNameStr::boxed_from_bytes(value.long.as_bytes()),
            discr,
        );
        if let Some(ref short) = value.short {
            map.insert(
                NormalizedPropertyNameStr::boxed_from_bytes(short.as_bytes()),
                discr,
            );
        }
        for alias in &value.aliases {
            map.insert(
                NormalizedPropertyNameStr::boxed_from_bytes(alias.as_bytes()),
                discr,
            );
        }
    }
    Ok(PropertyValueNameToEnumMapV1 {
        map: map.into_iter().collect(),
    })
}

fn load_values_to_names<M>(
    p: &crate::DatagenProvider,
    prop_name: &str,
    is_short: bool,
) -> Result<DataResponse<M>, DataError>
where
    M: DataMarker<Yokeable = PropertyEnumToValueNameMapV1<'static>>,
{
    let data = get_enumerated_prop(&p.source, prop_name)
        .map_err(|_| DataError::custom("Loading icuexport property data failed: \
                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)"))?;
    let mut map: BTreeMap<_, &str> = BTreeMap::new();

    for value in &data.values {
        let discr = u16::try_from(value.discr)
            .map_err(|_| DataError::custom("Found value larger than u16 for property".into()))?;
        if is_short {
            if let Some(ref short) = value.short {
                map.insert(discr, short);
            }
        } else {
            map.insert(discr, &value.long);
        }
    }

    let data_struct = if prop_name == "ccc" {
        // CCC has lots of gaps
        PropertyEnumToValueNameMapV1::Map(map.into_iter().collect())
    } else {
        let first = if let Some((&first, _)) = map.first_key_value() {
            if first > 0 {
                return Err(DataError::custom(
                    "Property has nonzero starting discriminant, perhaps consider \
                     storing its names as a map or by specializing this error",
                )
                .with_display_context(&format!("Property: {prop_name}, discr: {first}")));
            }

            first
        } else {
            return Err(
                DataError::custom("Property has no values!").with_display_context(prop_name)
            );
        };
        let last = if let Some((&last, _)) = map.last_key_value() {
            let range = usize::from(1 + last - first);
            let count = map.len();
            let gaps = range - count;
            if gaps > 0 {
                return Err(DataError::custom("Property has more than 0 gaps, \
                    perhaps consider storing its names in a map or by specializing this error")
                    .with_display_context(&format!("Property: {prop_name}, discriminant range: {first}..{last}, discriminant count: {count}")));
            }

            last
        } else {
            return Err(
                DataError::custom("Property has no values!").with_display_context(prop_name)
            );
        };

        let mut v = Vec::new();
        for i in 0..last {
            if let Some(&val) = map.get(&i) {
                v.push(val)
            } else {
                v.push("")
            }
        }

        PropertyEnumToValueNameMapV1::Linear((&v).into())
    };

    Ok(DataResponse {
        metadata: DataResponseMetadata::default(),
        payload: Some(DataPayload::from_owned(data_struct)),
    })
}

macro_rules! expand {
    ($(($marker:ident, $marker_n2e:ident, $marker_e2sn:ident, $marker_e2ln:ident, $prop_name:literal)),+,) => {
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

            impl DataProvider<$marker_n2e> for crate::DatagenProvider
            {
                fn load(&self, _: DataRequest) -> Result<DataResponse<$marker_n2e>, DataError> {
                    let data = get_enumerated_prop(&self.source, $prop_name)
                        .map_err(|_| DataError::custom("Loading icuexport property data failed: \
                                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)"))?;

                    let data_struct = get_prop_values_map(&data.values, |v| u16::try_from(v).map_err(|_| DataError::custom(concat!("Found value larger than u16 for property ", $prop_name))))?;
                    Ok(DataResponse {
                        metadata: DataResponseMetadata::default(),
                        payload: Some(DataPayload::from_owned(data_struct)),
                    })
                }
            }

            impl IterableDataProvider<$marker_n2e> for crate::DatagenProvider {
                fn supported_locales(
                    &self,
                ) -> Result<Vec<DataLocale>, DataError> {
                    get_enumerated_prop(&self.source, $prop_name)?;
                    Ok(vec![Default::default()])
                }
            }

            impl DataProvider<$marker_e2sn> for crate::DatagenProvider
            {
                fn load(&self, _: DataRequest) -> Result<DataResponse<$marker_e2sn>, DataError> {
                    load_values_to_names(self, $prop_name, true)
                }
            }

            impl IterableDataProvider<$marker_e2sn> for crate::DatagenProvider {
                fn supported_locales(
                    &self,
                ) -> Result<Vec<DataLocale>, DataError> {
                    get_enumerated_prop(&self.source, $prop_name)?;
                    Ok(vec![Default::default()])
                }
            }

            impl DataProvider<$marker_e2ln> for crate::DatagenProvider
            {
                fn load(&self, _: DataRequest) -> Result<DataResponse<$marker_e2ln>, DataError> {
                    load_values_to_names(self, $prop_name, false)
                }
            }

            impl IterableDataProvider<$marker_e2ln> for crate::DatagenProvider {
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

fn get_mask_prop<'a>(
    source: &'a SourceData,
    key: &str,
) -> Result<&'a super::uprops_serde::mask::MaskPropertyMap, DataError> {
    source
        .icuexport()?
        .read_and_parse_toml::<super::uprops_serde::mask::Main>(&format!(
            "uprops/{}/{}.toml",
            source.trie_type(),
            key
        ))?
        .mask_property
        .get(0)
        .ok_or(DataError::custom("Loading icuexport property data failed: \
                                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)"))
}
// Special handling for GeneralCategoryMask
impl DataProvider<GeneralCategoryMaskNameToValueV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        _: DataRequest,
    ) -> Result<DataResponse<GeneralCategoryMaskNameToValueV1Marker>, DataError> {
        use icu_properties::GeneralCategoryGroup;
        use zerovec::ule::AsULE;

        let data = get_mask_prop(&self.source, "gcm")?;
        let data_struct = get_prop_values_map(&data.values, |v| {
            let value: GeneralCategoryGroup = v.into();
            let ule = value.to_unaligned();
            let packed = u16::from_unaligned(ule);

            // sentinel value
            if packed == 0xFF00 {
                return Err(DataError::custom("Found unknown general category mask value, properties code may need to be updated."));
            }
            Ok(packed)
        })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(data_struct)),
        })
    }
}

impl IterableDataProvider<GeneralCategoryMaskNameToValueV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        get_mask_prop(&self.source, "gcm")?;
        Ok(vec![Default::default()])
    }
}

expand!(
    (
        CanonicalCombiningClassV1Marker,
        CanonicalCombiningClassNameToValueV1Marker,
        CanonicalCombiningClassValueToShortNameV1Marker,
        CanonicalCombiningClassValueToLongNameV1Marker,
        "ccc"
    ),
    (
        GeneralCategoryV1Marker,
        GeneralCategoryNameToValueV1Marker,
        GeneralCategoryValueToShortNameV1Marker,
        GeneralCategoryValueToLongNameV1Marker,
        "gc"
    ),
    (
        BidiClassV1Marker,
        BidiClassNameToValueV1Marker,
        BidiClassValueToShortNameV1Marker,
        BidiClassValueToLongNameV1Marker,
        "bc"
    ),
    (
        ScriptV1Marker,
        ScriptNameToValueV1Marker,
        ScriptValueToShortNameV1Marker,
        ScriptValueToLongNameV1Marker,
        "sc"
    ),
    (
        EastAsianWidthV1Marker,
        EastAsianWidthNameToValueV1Marker,
        EastAsianWidthValueToShortNameV1Marker,
        EastAsianWidthValueToLongNameV1Marker,
        "ea"
    ),
    (
        LineBreakV1Marker,
        LineBreakNameToValueV1Marker,
        LineBreakValueToShortNameV1Marker,
        LineBreakValueToLongNameV1Marker,
        "lb"
    ),
    (
        GraphemeClusterBreakV1Marker,
        GraphemeClusterBreakNameToValueV1Marker,
        GraphemeClusterBreakValueToShortNameV1Marker,
        GraphemeClusterBreakValueToLongNameV1Marker,
        "GCB"
    ),
    (
        WordBreakV1Marker,
        WordBreakNameToValueV1Marker,
        WordBreakValueToShortNameV1Marker,
        WordBreakValueToLongNameV1Marker,
        "WB"
    ),
    (
        SentenceBreakV1Marker,
        SentenceBreakNameToValueV1Marker,
        SentenceBreakValueToShortNameV1Marker,
        SentenceBreakValueToLongNameV1Marker,
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
