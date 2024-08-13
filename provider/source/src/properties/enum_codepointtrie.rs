// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::collections::codepointtrie::CodePointTrie;
use icu::properties::provider::{names::*, *};
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use tinystr::TinyStr4;

impl SourceDataProvider {
    pub(super) fn get_enumerated_prop<'a>(
        &'a self,
        key: &str,
    ) -> Result<&'a super::uprops_serde::enumerated::EnumeratedPropertyMap, DataError> {
        self.icuexport()?
            .read_and_parse_toml::<super::uprops_serde::enumerated::Main>(&format!(
                "uprops/{}/{}.toml",
                self.trie_type(),
                key
            ))?
            .enum_property
            .first()
            .ok_or_else(|| DataErrorKind::MarkerNotFound.into_error())
    }
    fn get_mask_prop<'a>(
        &'a self,
        key: &str,
    ) -> Result<&'a super::uprops_serde::mask::MaskPropertyMap, DataError> {
        self.icuexport()?
            .read_and_parse_toml::<super::uprops_serde::mask::Main>(&format!(
                "uprops/{}/{}.toml",
                self.trie_type(),
                key
            ))?
            .mask_property
            .first()
            .ok_or(DataError::custom(
                "Loading icuexport property data failed: \
                 Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)",
            ))
    }
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

/// Convert a map from property values to their names into
/// a linear map where each index represents a property value
fn map_to_vec<'a>(
    map: &'a BTreeMap<u16, &'a str>,
    prop_name: &str,
) -> Result<Vec<&'a str>, DataError> {
    // Use .first_key_value() and .last_key_value() after bumping MSRV
    let first = if let Some((&first, _)) = map.iter().next() {
        if first > 0 {
            return Err(DataError::custom(
                "Property has nonzero starting discriminant, perhaps consider \
                 storing its names as a sparse map or by specializing this error",
            )
            .with_display_context(&format!("Property: {prop_name}, discr: {first}")));
        }

        first
    } else {
        return Err(DataError::custom("Property has no values!").with_display_context(prop_name));
    };
    let last = if let Some((&last, _)) = map.iter().next_back() {
        let range = usize::from(1 + last - first);
        let count = map.len();
        let gaps = range - count;
        if gaps > 0 {
            return Err(DataError::custom("Property has more than 0 gaps, \
                perhaps consider storing its names in a sparse map or by specializing this error")
                .with_display_context(&format!("Property: {prop_name}, discriminant range: {first}..{last}, discriminant count: {count}")));
        }

        last
    } else {
        return Err(DataError::custom("Property has no values!").with_display_context(prop_name));
    };

    let mut v = Vec::new();
    for i in 0..=last {
        if let Some(&val) = map.get(&i) {
            v.push(val)
        } else {
            v.push("")
        }
    }
    Ok(v)
}

/// Load the mapping from property values to their names
fn load_values_to_names(
    data: &super::uprops_serde::enumerated::EnumeratedPropertyMap,
    is_short: bool,
) -> Result<BTreeMap<u16, &str>, DataError> {
    let mut map: BTreeMap<_, &str> = BTreeMap::new();

    for value in &data.values {
        let discr = u16::try_from(value.discr)
            .map_err(|_| DataError::custom("Found value larger than u16 for property"))?;
        if is_short {
            if let Some(ref short) = value.short {
                map.insert(discr, short);
            }
        } else {
            map.insert(discr, &value.long);
        }
    }

    Ok(map)
}

/// Load the mapping from property values to their names as a sparse map
fn load_values_to_names_sparse<M>(
    p: &SourceDataProvider,
    prop_name: &str,
    is_short: bool,
) -> Result<DataResponse<M>, DataError>
where
    M: DynamicDataMarker<DataStruct = PropertyEnumToValueNameSparseMapV1<'static>>,
{
    let data = p.get_enumerated_prop(prop_name)
        .map_err(|_| DataError::custom("Loading icuexport property data failed: \
                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)"))?;
    let map = load_values_to_names(data, is_short)?;
    let map = map.into_iter().collect();
    let data_struct = PropertyEnumToValueNameSparseMapV1 { map };
    Ok(DataResponse {
        metadata: Default::default(),
        payload: DataPayload::from_owned(data_struct),
    })
}

/// Load the mapping from property values to their names as a linear map
fn load_values_to_names_linear<M>(
    p: &SourceDataProvider,
    prop_name: &str,
    is_short: bool,
) -> Result<DataResponse<M>, DataError>
where
    M: DynamicDataMarker<DataStruct = PropertyEnumToValueNameLinearMapV1<'static>>,
{
    let data = p.get_enumerated_prop(prop_name)
        .map_err(|_| DataError::custom("Loading icuexport property data failed: \
                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)"))?;
    let map = load_values_to_names(data, is_short)?;
    let vec = map_to_vec(&map, prop_name)?;
    let varzerovec = (&vec).into();
    let data_struct = PropertyEnumToValueNameLinearMapV1 { map: varzerovec };
    Ok(DataResponse {
        metadata: Default::default(),
        payload: DataPayload::from_owned(data_struct),
    })
}

/// Load the mapping from property values to their names as a linear map of TinyStr4s
fn load_values_to_names_linear4<M>(
    p: &SourceDataProvider,
    prop_name: &str,
    is_short: bool,
) -> Result<DataResponse<M>, DataError>
where
    M: DynamicDataMarker<DataStruct = PropertyEnumToValueNameLinearTiny4MapV1<'static>>,
{
    let data = p.get_enumerated_prop(prop_name)
        .map_err(|_| DataError::custom("Loading icuexport property data failed: \
                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)"))?;
    let map = load_values_to_names(data, is_short)?;
    let vec = map_to_vec(&map, prop_name)?;
    let vec: Result<Vec<_>, _> = vec.into_iter().map(TinyStr4::try_from_str).collect();

    let vec = vec.map_err(|_| {
        DataError::custom("Found property value longer than 4 characters for linear4 property")
    })?;
    let zerovec = vec.into_iter().collect();
    let data_struct = PropertyEnumToValueNameLinearTiny4MapV1 { map: zerovec };
    Ok(DataResponse {
        metadata: Default::default(),
        payload: DataPayload::from_owned(data_struct),
    })
}
macro_rules! expand {
    ($(($marker:ident, $marker_n2e:ident,
        // marker_e2sns is short for marker_enum_to_short_name_sparse, etc
        // We only support selecting one of these at a time right now, but we need
        // different variable names for the macro matcher to work
        $((sparse: $marker_e2sns:ident, $marker_e2lns:ident),)?
        $((linear: $marker_e2snl:ident, $marker_e2lnl:ident),)?
        $((linear4: $marker_e2snl4:ident, $marker_e2lnl4:ident),)?


        $prop_name:literal)),+,) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider
            {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let source_cpt_data = &self.get_enumerated_prop($prop_name)?.code_point_trie;

                    let code_point_trie = CodePointTrie::try_from(source_cpt_data).map_err(|e| {
                        DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
                    })?;
                    let data_struct = PropertyCodePointMapV1::CodePointTrie(code_point_trie);
                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(data_struct),
                    })
                }
            }

            impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                    self.get_enumerated_prop($prop_name)?;
                    Ok(HashSet::from_iter([Default::default()]))
                }
            }

            impl DataProvider<$marker_n2e> for SourceDataProvider
            {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker_n2e>, DataError> {
                    self.check_req::<$marker_n2e>(req)?;
                    let data = self.get_enumerated_prop($prop_name)
                        .map_err(|_| DataError::custom("Loading icuexport property data failed: \
                                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)"))?;

                    let data_struct = get_prop_values_map(&data.values, |v| u16::try_from(v).map_err(|_| DataError::custom(concat!("Found value larger than u16 for property ", $prop_name))))?;
                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(data_struct),
                    })
                }
            }

            impl crate::IterableDataProviderCached<$marker_n2e> for SourceDataProvider {
                                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                    self.get_enumerated_prop($prop_name)?;
                    Ok(HashSet::from_iter([Default::default()]))
                }
            }

            $(
                impl DataProvider<$marker_e2sns> for SourceDataProvider
                {
                    fn load(&self, req: DataRequest) -> Result<DataResponse<$marker_e2sns>, DataError> {
                        self.check_req::<$marker_e2sns>(req)?;
                        load_values_to_names_sparse(self, $prop_name, true)
                    }
                }

                impl crate::IterableDataProviderCached<$marker_e2sns> for SourceDataProvider {
                    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                        self.get_enumerated_prop($prop_name)?;
                        Ok(HashSet::from_iter([Default::default()]))
                    }
                }

                impl DataProvider<$marker_e2lns> for SourceDataProvider
                {
                    fn load(&self, req: DataRequest) -> Result<DataResponse<$marker_e2lns>, DataError> {
                        self.check_req::<$marker_e2lns>(req)?;
                        load_values_to_names_sparse(self, $prop_name, false)
                    }
                }

                impl crate::IterableDataProviderCached<$marker_e2lns> for SourceDataProvider {
                    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                        self.get_enumerated_prop($prop_name)?;
                        Ok(HashSet::from_iter([Default::default()]))
                    }
                }
            )?

            $(
                impl DataProvider<$marker_e2snl> for SourceDataProvider
                {
                    fn load(&self, req: DataRequest) -> Result<DataResponse<$marker_e2snl>, DataError> {
                        self.check_req::<$marker_e2snl>(req)?;
                        load_values_to_names_linear(self, $prop_name, true)
                    }
                }

                impl crate::IterableDataProviderCached<$marker_e2snl> for SourceDataProvider {
                    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                        self.get_enumerated_prop($prop_name)?;
                        Ok(HashSet::from_iter([Default::default()]))
                    }
                }

                impl DataProvider<$marker_e2lnl> for SourceDataProvider
                {
                    fn load(&self, req: DataRequest) -> Result<DataResponse<$marker_e2lnl>, DataError> {
                        self.check_req::<$marker_e2lnl>(req)?;
                        load_values_to_names_linear(self, $prop_name, false)
                    }
                }

                impl crate::IterableDataProviderCached<$marker_e2lnl> for SourceDataProvider {
                    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                        self.get_enumerated_prop($prop_name)?;
                        Ok(HashSet::from_iter([Default::default()]))
                    }
                }
            )?

            $(
                impl DataProvider<$marker_e2snl4> for SourceDataProvider
                {
                    fn load(&self, req: DataRequest) -> Result<DataResponse<$marker_e2snl4>, DataError> {
                        self.check_req::<$marker_e2snl4>(req)?;
                        load_values_to_names_linear4(self, $prop_name, true)
                    }
                }

                impl crate::IterableDataProviderCached<$marker_e2snl4> for SourceDataProvider {
                    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                        self.get_enumerated_prop($prop_name)?;
                        Ok(HashSet::from_iter([Default::default()]))
                    }
                }

                impl DataProvider<$marker_e2lnl4> for SourceDataProvider
                {
                    fn load(&self, req: DataRequest) -> Result<DataResponse<$marker_e2lnl4>, DataError> {
                        self.check_req::<$marker_e2lnl4>(req)?;
                        // Tiny4 is only for short names
                        load_values_to_names_linear(self, $prop_name, false)
                    }
                }

                impl crate::IterableDataProviderCached<$marker_e2lnl4> for SourceDataProvider {
                    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>  {
                        self.get_enumerated_prop($prop_name)?;
                        Ok(HashSet::from_iter([Default::default()]))
                    }
                }
            )?
        )+
    };
}

// Special handling for GeneralCategoryMask
impl DataProvider<GeneralCategoryMaskNameToValueV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<GeneralCategoryMaskNameToValueV1Marker>, DataError> {
        use icu::properties::GeneralCategoryGroup;
        use zerovec::ule::AsULE;

        self.check_req::<GeneralCategoryMaskNameToValueV1Marker>(req)?;

        let data = self.get_mask_prop("gcm")?;
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
            metadata: Default::default(),
            payload: DataPayload::from_owned(data_struct),
        })
    }
}

impl crate::IterableDataProviderCached<GeneralCategoryMaskNameToValueV1Marker>
    for SourceDataProvider
{
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.get_mask_prop("gcm")?;
        Ok(HashSet::from_iter([Default::default()]))
    }
}

expand!(
    (
        CanonicalCombiningClassV1Marker,
        CanonicalCombiningClassNameToValueV1Marker,
        (
            sparse: CanonicalCombiningClassValueToShortNameV1Marker,
            CanonicalCombiningClassValueToLongNameV1Marker
        ),
        "ccc"
    ),
    (
        GeneralCategoryV1Marker,
        GeneralCategoryNameToValueV1Marker,
        (
            linear: GeneralCategoryValueToShortNameV1Marker,
            GeneralCategoryValueToLongNameV1Marker
        ),
        "gc"
    ),
    (
        BidiClassV1Marker,
        BidiClassNameToValueV1Marker,
        (
            linear: BidiClassValueToShortNameV1Marker,
            BidiClassValueToLongNameV1Marker
        ),
        "bc"
    ),
    (
        ScriptV1Marker,
        ScriptNameToValueV1Marker,
        (
            linear4: ScriptValueToShortNameV1Marker,
            ScriptValueToLongNameV1Marker
        ),
        "sc"
    ),
    (
        HangulSyllableTypeV1Marker,
        HangulSyllableTypeNameToValueV1Marker,
        (
            linear: HangulSyllableTypeValueToShortNameV1Marker,
            HangulSyllableTypeValueToLongNameV1Marker
        ),
        "hst"
    ),
    (
        EastAsianWidthV1Marker,
        EastAsianWidthNameToValueV1Marker,
        (
            linear: EastAsianWidthValueToShortNameV1Marker,
            EastAsianWidthValueToLongNameV1Marker
        ),
        "ea"
    ),
    (
        IndicSyllabicCategoryV1Marker,
        IndicSyllabicCategoryNameToValueV1Marker,
        (
            linear: IndicSyllabicCategoryValueToShortNameV1Marker,
            IndicSyllabicCategoryValueToLongNameV1Marker
        ),
        "InSC"
    ),
    (
        LineBreakV1Marker,
        LineBreakNameToValueV1Marker,
        (
            linear: LineBreakValueToShortNameV1Marker,
            LineBreakValueToLongNameV1Marker
        ),
        "lb"
    ),
    (
        GraphemeClusterBreakV1Marker,
        GraphemeClusterBreakNameToValueV1Marker,
        (
            linear: GraphemeClusterBreakValueToShortNameV1Marker,
            GraphemeClusterBreakValueToLongNameV1Marker
        ),
        "GCB"
    ),
    (
        WordBreakV1Marker,
        WordBreakNameToValueV1Marker,
        (
            linear: WordBreakValueToShortNameV1Marker,
            WordBreakValueToLongNameV1Marker
        ),
        "WB"
    ),
    (
        SentenceBreakV1Marker,
        SentenceBreakNameToValueV1Marker,
        (
            linear: SentenceBreakValueToShortNameV1Marker,
            SentenceBreakValueToLongNameV1Marker
        ),
        "SB"
    ),
    (
        JoiningTypeV1Marker,
        JoiningTypeNameToValueV1Marker,
        (
            linear: JoiningTypeValueToShortNameV1Marker,
            JoiningTypeValueToLongNameV1Marker
        ),
        "jt"
    ),
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu::properties::{GeneralCategory, Script};

    // A test of the UnicodeProperty General_Category is truly a test of the
    // `GeneralCategory` Rust enum, not the `GeneralCategoryGroup` Rust enum,
    // since we must match the representation and value width of the data from
    // the ICU CodePointTrie that ICU4X is reading from.
    #[test]
    fn test_general_category() {
        let provider = SourceDataProvider::new_testing();

        let trie = icu::properties::maps::load_general_category(&provider).unwrap();
        let trie = trie.as_code_point_trie().unwrap();

        assert_eq!(trie.get32('꣓' as u32), GeneralCategory::DecimalNumber);
        assert_eq!(trie.get32('≈' as u32), GeneralCategory::MathSymbol);
    }

    #[test]
    fn test_script() {
        let provider = SourceDataProvider::new_testing();

        let trie = icu::properties::maps::load_script(&provider).unwrap();
        let trie = trie.as_code_point_trie().unwrap();

        assert_eq!(trie.get32('꣓' as u32), Script::Saurashtra);
        assert_eq!(trie.get32('≈' as u32), Script::Common);
    }
}
