// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::DatagenProvider;
use icu::properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker;
use icu_provider::prelude::*;

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
impl DatagenProvider {
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
            .ok_or_else(|| DataErrorKind::MissingDataMarker.into_error())
    }
}

// implement data provider 2 different ways, based on whether or not
// features exist that enable the use of CPT Builder (ex: `use_wasm` or `use_icu4c`)
impl DataProvider<BidiAuxiliaryPropertiesV1Marker> for DatagenProvider {
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<BidiAuxiliaryPropertiesV1Marker>, DataError> {
        use icu::collections::codepointinvlist::CodePointInversionListBuilder;
        use icu::collections::codepointtrie::CodePointTrie;
        use icu::collections::codepointtrie::TrieType;
        use icu::properties::provider::bidi_data::{
            BidiAuxiliaryPropertiesV1, MirroredPairedBracketData,
        };
        use icu_codepointtrie_builder::{CodePointTrieBuilder, CodePointTrieBuilderData};

        self.check_req::<BidiAuxiliaryPropertiesV1Marker>(req)?;

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

        // Create the equivalent of CPT<MirroredPairedBracketData>, but since the
        // trie's value type's ULE serializes to 24 bits, which CPT builder cannot handle, widen
        // to 32 bits using u32.
        let trie_vals_structured_iter =
            (0..=(char::MAX as u32)).map(|cp| MirroredPairedBracketData {
                mirroring_glyph: bmg_trie.get32(cp),
                mirrored: bidi_m_cpinvlist.contains32(cp),
                paired_bracket_type: bpt_trie.get32(cp),
            });
        let trie_vals_ule_iter = trie_vals_structured_iter.map(u32::from);
        let trie_vals_vec: Vec<u32> = trie_vals_ule_iter.collect();
        let trie_data = CodePointTrieBuilderData::ValuesByCodePoint(&trie_vals_vec);
        let default_val: u32 = MirroredPairedBracketData::default().into();

        // Create CPT<u32> using the builder, then use CPT method to map the CPT's
        // values from u32 to MirroredPairedBracketData
        let trie: CodePointTrie<u32> = CodePointTrieBuilder {
            data: trie_data,
            default_value: default_val,
            error_value: default_val,
            trie_type: TrieType::Small,
        }
        .build();
        let trie_mpbd = trie
            .try_alloc_map_value(MirroredPairedBracketData::try_from)
            .map_err(|_| DataError::custom("Cannot parse MirroredPairedBracketData from u32"))?;

        let data_struct = BidiAuxiliaryPropertiesV1 { trie: trie_mpbd };
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data_struct),
        })
    }

    #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<BidiAuxiliaryPropertiesV1Marker>, DataError> {
        self.check_req::<BidiAuxiliaryPropertiesV1Marker>(req)?;
        return Err(DataError::custom(
            "icu_datagen_bikeshed must be built with use_icu4c or use_wasm to build Bidi auxiliary properties data",
        ));
    }
}

impl crate::IterableDataProviderCached<BidiAuxiliaryPropertiesV1Marker> for DatagenProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu::properties::bidi_data::BidiPairingProperties;

    #[test]
    fn test_bidi_data_provider() {
        let provider = DatagenProvider::new_testing();

        let bidi_data =
            icu::properties::bidi_data::load_bidi_auxiliary_properties_unstable(&provider).unwrap();
        let bidi_data = bidi_data.as_borrowed();

        let close_paren = bidi_data.get32_mirroring_props(')' as u32);
        assert_eq!(close_paren.mirroring_glyph, Some('('));
        assert!(close_paren.mirrored);
        let close_angle_bracket = bidi_data.get32_mirroring_props('>' as u32);
        assert_eq!(close_angle_bracket.mirroring_glyph, Some('<'));
        assert!(close_angle_bracket.mirrored);

        let open_paren = bidi_data.get32_pairing_props('(' as u32);
        assert_eq!(open_paren, BidiPairingProperties::Open(')'));
        let open_angle_bracket = bidi_data.get32_pairing_props('<' as u32);
        assert_eq!(open_angle_bracket, BidiPairingProperties::None);
    }
}
