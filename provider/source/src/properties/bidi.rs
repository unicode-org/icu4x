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
        use super::ucd_helpers;
        use icu::collections::codepointtrie::TrieType;
        use icu::properties::props::BidiMirroringGlyph;
        use icu::properties::props::BidiPairedBracketType;
        use icu::properties::props::EnumeratedProperty;
        use icu_codepointtrie_builder::CodePointTrieBuilder;
        use std::collections::HashMap;

        self.check_req::<PropertyEnumBidiMirroringGlyphV1>(req)?;

        if let Some(t) = self
            .unicode()?
            .cpt_cache
            .get(core::str::from_utf8(BidiMirroringGlyph::SHORT_NAME).unwrap())
        {
            let trie = t.downcast_ref::<icu::collections::codepointtrie::CodePointTrie<'static, BidiMirroringGlyph>>().unwrap().clone();

            return Ok(DataResponse {
                metadata: Default::default(),
                payload: DataPayload::from_owned(
                    icu::properties::provider::PropertyCodePointMap::CodePointTrie(trie),
                ),
            });
        }

        let bidi_m_cpinvlist = self.get_binary_prop("Bidi_Mirrored", "Bidi_M")?;

        let bidi_mirroring = self
            .parse_ucd_lines("ucd/BidiMirroring.txt")?
            .filter_map(|line| {
                let mut fields = line.skip_missing_rule()?.fields();
                let cp_range = fields.next().unwrap().trim();
                let prop_value = fields.next().unwrap().trim();
                let value = ucd_helpers::parse_cp(prop_value);
                let cp = ucd_helpers::parse_cp(cp_range);
                Some((cp, char::from_u32(value).unwrap()))
            })
            .collect::<HashMap<_, _>>();

        let paired_brackets = self.parse_ucd_lines("ucd/BidiBrackets.txt")?.filter_map(|line| {
                let mut parts = line.skip_missing_rule()?.fields();
                let cp = ucd_helpers::parse_cp(parts.next().unwrap().trim());
                let mirror = ucd_helpers::parse_cp(parts.next().unwrap().trim());

                if bidi_mirroring[&cp] as u32 != mirror {
                    log::warn!(
                        "BidiMirroring.txt and BidiBrackets.txt disagree for U+{cp:X}: {:?} vs U+{mirror:X}", 
                        bidi_mirroring[&cp]
                    );
                }

                let typ = match parts.next().unwrap().trim() {
                    "o" => BidiPairedBracketType::Open,
                    "c" => BidiPairedBracketType::Close,
                    "n" => BidiPairedBracketType::None,
                    _ => unreachable!(),
                };
                Some((cp, typ))

        }).collect::<HashMap<_, _>>();

        let mut builder = CodePointTrieBuilder::new(
            BidiMirroringGlyph::default(),
            BidiMirroringGlyph::default(),
            TrieType::Small,
        );

        for cp in 0..=(char::MAX as u32) {
            if !bidi_m_cpinvlist.contains32(cp) {
                continue;
            }
            let mirroring_glyph = bidi_mirroring.get(&cp).copied();
            if mirroring_glyph.is_none() {
                log::trace!(
                    "Missing mirroring glyph: U+{cp:X}: {}",
                    char::from_u32(cp).unwrap()
                );
            };
            let paired_bracket_type = paired_brackets
                .get(&cp)
                .copied()
                .unwrap_or(BidiPairedBracketType::None);
            builder.set_value(
                cp,
                BidiMirroringGlyph {
                    mirrored: true,
                    mirroring_glyph,
                    paired_bracket_type,
                },
            );
        }

        let trie = builder.build();

        self.unicode()?.cpt_cache.insert(
            core::str::from_utf8(BidiMirroringGlyph::SHORT_NAME).unwrap(),
            Box::new(trie.clone()),
        );

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                icu::properties::provider::PropertyCodePointMap::CodePointTrie(trie),
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
            "icu_provider_source must be built with `use_icu4c` or `use_wasm` to build enumerated properties data",
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
