// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(
    not(any(feature = "use_wasm", feature = "use_icu4c")),
    allow(unused_imports, dead_code)
)]

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

    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    pub(super) fn build_enumerated_prop<T: EnumeratedProperty>(
        &self,
    ) -> Result<CodePointTrie<'static, T>, DataError> {
        let name = core::str::from_utf8(T::NAME).unwrap();
        let short_name = core::str::from_utf8(T::SHORT_NAME).unwrap();

        self.validate_property_name(name, short_name)?;

        let discriminants = self.enumerated_prop_names(name, short_name)?.0;

        let mut builder = icu_codepointtrie_builder::CodePointTrieBuilder::new(
            T::default(),
            T::default(),
            self.trie_type().into(),
        );

        let file = match name {
            "Indic_Conjunct_Break" => "ucd/DerivedCoreProperties.txt".into(),
            "Canonical_Combining_Class"
            | "General_Category"
            | "Bidi_Class"
            | "Numeric_Type"
            | "East_Asian_Width"
            | "Joining_Type"
            | "Joining_Group" => {
                format!(
                    "ucd/extracted/Derived{}.txt",
                    name.replace('_', "").replace("Canonical", "")
                )
            }
            "Grapheme_Cluster_Break" | "Word_Break" | "Sentence_Break" => {
                format!(
                    "ucd/auxiliary/{}Property.txt",
                    name.replace('_', "").replace("Cluster", "")
                )
            }
            _ => format!(
                "ucd/{}.txt",
                name.replace('_', "").replace("Script", "Scripts")
            ),
        };

        for line in self.unicode()?.read_to_string(&file)?.lines() {
            let line = line.strip_prefix("# @missing: ").unwrap_or(line);
            let line = line.split('#').next().unwrap().trim();
            if line.is_empty() {
                continue;
            }
            let mut fields = line.split(';');
            let cp_range = fields.next().unwrap().trim();
            if &file == "ucd/DerivedCoreProperties.txt" {
                // This is a file containing multiple properties, so we need to check
                // the second column for the property name
                if fields.next().unwrap().trim() != short_name {
                    continue;
                }
            }
            let value = fields.next().unwrap().trim();
            let value = discriminants.get(value).copied().expect(value);
            let value = TrieValue::try_from_u32(value as u32).ok().unwrap();

            if let Some((start, end)) = cp_range.split_once("..") {
                let start = u32::from_str_radix(start, 16).unwrap();
                let end = u32::from_str_radix(end, 16).unwrap();
                builder.set_range_value(start..=end, value);
            } else {
                let cp = u32::from_str_radix(cp_range, 16).unwrap();
                builder.set_value(cp, value);
            }
        }

        Ok(builder.build())
    }

    #[allow(clippy::type_complexity)] // just a tuple
    fn enumerated_prop_names<'a>(
        &'a self,
        name: &str,
        mut short_name: &str,
    ) -> Result<
        (
            BTreeMap<&'a str, u16>,
            BTreeMap<u16, &'a str>,
            BTreeMap<u16, &'a str>,
        ),
        DataError,
    > {
        let mut short_names = self
            .get_enumerated_prop(name, short_name)?
            .values
            .iter()
            .map(|value| (value.discr, value.short.as_str()))
            .collect::<BTreeMap<_, _>>();

        if short_name == "InCB" {
            // https://unicode-org.atlassian.net/browse/ICU-23383
            short_names.extend([(1, "Consonant"), (2, "Extend"), (3, "Linker")]);
        }

        // For the gcm property we want to look up gc names.
        if short_name == "gcm" {
            short_name = "gc";
        }

        let mut discriminants = short_names
            .iter()
            .map(|(&d, &n)| (n, d))
            .collect::<BTreeMap<_, _>>();
        let mut long_names = BTreeMap::new();

        if short_name == "sc" {
            // ICU adds a bunch of scripts that don't appear in Unicode,
            // and hence don't have long names
            for short in [
                "Afak", "Aran", "Blis", "Cirt", "Cyrs", "Egyd", "Egyh", "Geok", "Hanb", "Hans",
                "Hant", "Hntl", "Inds", "Jamo", "Jpan", "Jurc", "Kore", "Kpel", "Latf", "Latg",
                "Loma", "Maya", "Moon", "Nkgb", "Phlv", "Roro", "Sara", "Syre", "Syrj", "Syrn",
                "Teng", "Visp", "Wole", "Zmth", "Zsye", "Zsym", "Zxxx",
            ] {
                long_names.entry(discriminants[short]).or_insert(short);
            }
        }

        for line in self
            .unicode()?
            .read_to_string("ucd/PropertyValueAliases.txt")?
            .lines()
        {
            let line = line.split('#').next().unwrap().trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split(';').map(str::trim);
            if parts.next().unwrap().trim() != short_name {
                continue;
            }
            let numeric_name = (short_name == "ccc").then(|| parts.next().unwrap());
            let short = parts.next().unwrap();
            let Some(discriminant) = discriminants.get(short).copied() else {
                continue;
            };
            let long = parts.next().unwrap();
            long_names.insert(discriminant, long);
            discriminants.insert(long, discriminant);
            for alias in parts {
                discriminants.insert(alias, discriminant);
            }
            if let Some(numeric_name) = numeric_name {
                discriminants.insert(numeric_name, discriminant);
            }
        }

        Ok((discriminants, short_names, long_names))
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

                    #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
                    return Err(DataError::custom(
                        "icu_provider_source must be built with use_icu4c or use_wasm to build properties data",
                    )
                    .with_req($marker::INFO, req));
                    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
                    {
                        let trie = if let Some(t) = self.unicode()?.cpt_cache.get(core::str::from_utf8(<$prop as EnumeratedProperty>::SHORT_NAME).unwrap()).
                            and_then(|t| t.downcast_ref::<CodePointTrie<'static, $prop>>().cloned()) {
                            t
                        } else {
                            let trie = self.build_enumerated_prop::<$prop>()?;

                            self.unicode()?.cpt_cache
                                .insert(core::str::from_utf8(<$prop as EnumeratedProperty>::SHORT_NAME).unwrap(), Box::new(trie.clone()));

                            trie
                        };

                        Ok(DataResponse {
                            metadata: Default::default(),
                            payload: DataPayload::from_owned(PropertyCodePointMap::CodePointTrie(trie)),
                        })
                    }
                }
            }

            impl DataProvider<$parse_marker> for SourceDataProvider
            {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$parse_marker>, DataError> {
                    self.check_req::<$parse_marker>(req)?;
                    let data = self.enumerated_prop_names(
                        core::str::from_utf8(<$prop as EnumeratedProperty>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as EnumeratedProperty>::SHORT_NAME).unwrap()
                    )?;
                    let map = data.0;
                    for name in map.keys() {
                        if name.contains('-') || name.bytes().any(|b| b.is_ascii_whitespace()) {
                            return Err(
                                DataError::custom("Property name contains '-' or whitespace")
                                    .with_display_context(name),
                            );
                        }
                    }
                    let trie = map
                        .into_iter()
                        // Filter CCC's numeric names.
                        // TODO: Don't
                        .filter(|(k, _)| k.parse::<usize>().is_err())
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
                    let data = self.enumerated_prop_names(
                        core::str::from_utf8(<$prop as EnumeratedProperty>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as EnumeratedProperty>::SHORT_NAME).unwrap()
                    )?;
                    let map = ($short_convert)(data.1)?;

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
                    let data = self.enumerated_prop_names(
                        core::str::from_utf8(<$prop as EnumeratedProperty>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as EnumeratedProperty>::SHORT_NAME).unwrap()
                    )?;
                    let map = ($long_convert)(data.2)?;

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

        let mut discriminants = self
            .get_mask_prop("General_Category_Mask", "gcm", "General_Category")?
            .values
            .iter()
            .map(|value| {
                (
                    value.short.as_str(),
                    GeneralCategoryGroup::from(value.discr).to_u32() as usize,
                )
            })
            .collect::<BTreeMap<_, _>>();

        for line in self
            .unicode()?
            .read_to_string("ucd/PropertyValueAliases.txt")?
            .lines()
        {
            let line = line.split('#').next().unwrap().trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split(';').map(str::trim);
            if parts.next().unwrap() != "gc" {
                continue;
            }
            let short = parts.next().unwrap();
            let Some(discriminant) = discriminants.get(short).copied() else {
                continue;
            };
            let long = parts.next().unwrap();
            discriminants.insert(long, discriminant);
            for alias in parts {
                discriminants.insert(alias, discriminant);
            }
        }

        let trie = discriminants
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
