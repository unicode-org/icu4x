// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use icu::properties::provider::PropertyEnumBidiMirroringGlyphV1;
use icu_provider::prelude::*;

// implement data provider 2 different ways, based on whether or not
// features exist that enable the use of CPT Builder (ex: `use_wasm` or `use_icu4c`)
impl DataProvider<PropertyEnumBidiMirroringGlyphV1> for SourceDataProvider {
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<PropertyEnumBidiMirroringGlyphV1>, DataError> {
        use icu::collections::codepointtrie::TrieType;
        use icu::collections::codepointtrie::TrieValue;
        use icu::properties::props::BidiMirroringGlyph;
        use icu::properties::props::BidiPairedBracketType;
        use icu::properties::props::EnumeratedProperty;
        use icu_codepointtrie_builder::{CodePointTrieBuilder, CodePointTrieBuilderData};

        self.check_req::<PropertyEnumBidiMirroringGlyphV1>(req)?;

        let bidi_m_cpinvlist = self
            .get_binary_prop_for_code_point_set("Bidi_Mirrored", "Bidi_M")?
            .build_inversion_list();

        let bmg_trie = self
            .get_enumerated_prop(
                core::str::from_utf8(BidiMirroringGlyph::NAME).unwrap(),
                core::str::from_utf8(BidiMirroringGlyph::SHORT_NAME).unwrap(),
            )?
            .build_codepointtrie()?;

        let bpt = self.get_enumerated_prop("Bidi_Paired_Bracket_Type", "bpt")?;
        let bpt_trie = bpt.build_codepointtrie::<u16>()?;
        let bpt_lookup = bpt.values_to_names_long();

        let code_point_trie = CodePointTrieBuilder {
            data: CodePointTrieBuilderData::ByCodePoint(
                0..=(char::MAX as u32),
                Box::new(|cp| {
                    if !bidi_m_cpinvlist.contains32(cp) {
                        return None;
                    }
                    Some(BidiMirroringGlyph {
                        mirrored: true,
                        mirroring_glyph: {
                            let m = bmg_trie.get32(cp);
                            if m as u32 == 0 {
                                log::trace!(
                                    "Missing mirroring glyph: U+{cp:X}: {}",
                                    char::try_from_u32(cp).unwrap()
                                );
                                None
                            } else {
                                Some(m)
                            }
                        },
                        paired_bracket_type: match bpt_lookup.get(&(bpt_trie.get32(cp))).copied() {
                            Some("Open") => BidiPairedBracketType::Open,
                            Some("Close") => BidiPairedBracketType::Close,
                            Some("None") => BidiPairedBracketType::None,
                            _ => {
                                log::trace!(
                                    "Missing paired-bracket-type: U+{cp:X}: {}",
                                    char::try_from_u32(cp).unwrap()
                                );
                                BidiPairedBracketType::None
                            }
                        },
                    })
                }),
            ),
            default_value: BidiMirroringGlyph::default(),
            error_value: BidiMirroringGlyph::default(),
            trie_type: TrieType::Small,
        }
        .build();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                icu::properties::provider::PropertyCodePointMap::CodePointTrie(code_point_trie),
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
