// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use icu::properties::provider::PropertyEnumBidiMirroringGlyphV1;
use icu_provider::prelude::*;

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
impl SourceDataProvider {
    fn get_code_point_prop_map<'a>(
        &'a self,
        key: &str,
    ) -> Result<&'a super::uprops_serde::code_point_prop::CodePointPropertyMap, DataError> {
        self.icuexport()?
            .read_and_parse_toml::<super::uprops_serde::code_point_prop::Main>(&format!(
                "uprops/{}/{}.toml",
                self.trie_type(),
                key
            ))?
            .enum_property
            .first()
            .ok_or_else(|| DataErrorKind::MarkerNotFound.into_error())
    }
}

// implement data provider 2 different ways, based on whether or not
// features exist that enable the use of CPT Builder (ex: `use_wasm` or `use_icu4c`)
impl DataProvider<PropertyEnumBidiMirroringGlyphV1> for SourceDataProvider {
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<PropertyEnumBidiMirroringGlyphV1>, DataError> {
        use icu::collections::codepointinvlist::CodePointInversionListBuilder;
        use icu::collections::codepointtrie::CodePointTrie;
        use icu::collections::codepointtrie::TrieType;
        use icu::collections::codepointtrie::TrieValue;
        use icu::properties::props::BidiMirroringGlyph;
        use icu::properties::props::BidiPairedBracketType;
        use icu_codepointtrie_builder::{CodePointTrieBuilder, CodePointTrieBuilderData};

        self.check_req::<PropertyEnumBidiMirroringGlyphV1>(req)?;

        // Bidi_M / Bidi_Mirrored
        let bidi_m_data = self.get_binary_prop_for_code_point_set("Bidi_M")?;
        let mut bidi_m_builder = CodePointInversionListBuilder::new();
        for (start, end) in &bidi_m_data.ranges {
            bidi_m_builder.add_range32(start..=end);
        }
        let bidi_m_cpinvlist = bidi_m_builder.build();

        // bmg / Bidi_Mirroring_Glyph
        let bmg_data = &self.get_code_point_prop_map("bmg")?.code_point_trie;
        let bmg_trie = CodePointTrie::try_from(bmg_data).map_err(|e| {
            DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
        })?;

        // bpt / Bidi_Paired_Bracket_Type
        let bpt_data = &self.get_enumerated_prop("bpt")?.code_point_trie;
        let bpt_trie = CodePointTrie::try_from(bpt_data).map_err(|e| {
            DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
        })?;

        let trie_vals = (0..=(char::MAX as u32)).map(|cp| {
            let mut r = BidiMirroringGlyph::default();
            r.mirrored = bidi_m_cpinvlist.contains32(cp);
            r.mirroring_glyph = r
                .mirrored
                .then_some(bmg_trie.get32(cp))
                .filter(|&cp| cp as u32 != 0);
            r.paired_bracket_type = match bpt_trie.get32(cp) {
                1 => BidiPairedBracketType::Open,
                2 => BidiPairedBracketType::Close,
                _ => BidiPairedBracketType::None,
            };
            if r.mirrored && r.mirroring_glyph.is_none() {
                log::trace!(
                    "Missing mirroring glyph: U+{cp:X}: {}",
                    char::try_from_u32(cp).unwrap()
                );
            }
            r
        });

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                icu::properties::provider::PropertyCodePointMap::CodePointTrie(
                    CodePointTrieBuilder {
                        data: CodePointTrieBuilderData::ValuesByCodePoint(
                            &trie_vals.collect::<Vec<_>>(),
                        ),
                        default_value: BidiMirroringGlyph::default(),
                        error_value: BidiMirroringGlyph::default(),
                        trie_type: TrieType::Small,
                    }
                    .build(),
                ),
            ),
        })
    }

    #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<PropertyEnumBidiMirroringGlyphV1>, DataError> {
        self.check_req::<PropertyEnumBidiMirroringGlyphV1>(req)?;
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build Bidi auxiliary properties data",
        ));
    }
}

impl crate::IterableDataProviderCached<PropertyEnumBidiMirroringGlyphV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu::properties::props::{BidiMirroringGlyph, BidiPairedBracketType};

    #[test]
    fn test_bidi_data_provider() {
        let provider = SourceDataProvider::new_testing();

        let bidi_data =
            icu::properties::CodePointMapData::<BidiMirroringGlyph>::try_new_unstable(&provider)
                .unwrap();
        let bidi_data = bidi_data.as_borrowed();

        let close_paren = bidi_data.get(')');
        assert_eq!(close_paren.mirroring_glyph, Some('('));
        assert!(close_paren.mirrored);
        let close_angle_bracket = bidi_data.get('>');
        assert_eq!(close_angle_bracket.mirroring_glyph, Some('<'));
        assert!(close_angle_bracket.mirrored);

        let open_paren = bidi_data.get('(');
        assert_eq!(open_paren.paired_bracket_type, BidiPairedBracketType::Open);
        let open_angle_bracket = bidi_data.get('<');
        assert_eq!(
            open_angle_bracket.paired_bracket_type,
            BidiPairedBracketType::None
        );
    }
}
