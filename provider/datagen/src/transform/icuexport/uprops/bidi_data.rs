// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::icuexport::uprops::{bin_cp_set, enum_codepointtrie};
use crate::SourceData;

use icu_codepointtrie_builder::{CodePointTrieBuilder, CodePointTrieBuilderData};
use icu_collections::codepointinvlist::CodePointInversionListBuilder;
use icu_collections::codepointtrie::{CodePointTrie, TrieType};
use icu_properties::provider::bidi_data::{
    BidiAuxiliaryPropertiesV1, BidiAuxiliaryPropertiesV1Marker, MirroredPairedBracketData,
};
use icu_properties::BidiPairedBracketType;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use zerovec::ule::{AsULE, ULE};

fn get_code_point_prop_map<'a>(
    source: &'a SourceData,
    key: &str,
) -> Result<&'a super::uprops_serde::code_point_prop::CodePointPropertyMap, DataError> {
    source
        .icuexport()?
        .read_and_parse_toml::<super::uprops_serde::code_point_prop::Main>(&format!(
            "uprops/{}/{}.toml",
            source.trie_type(),
            key
        ))?
        .enum_property
        .get(0)
        .ok_or_else(|| DataErrorKind::MissingDataKey.into_error())
}

// implement data provider
impl DataProvider<BidiAuxiliaryPropertiesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        _: DataRequest,
    ) -> Result<DataResponse<BidiAuxiliaryPropertiesV1Marker>, DataError> {
        // Bidi_M / Bidi_Mirrored
        let bidi_m_data = bin_cp_set::get_binary_prop_for_code_point_set(&self.source, "Bidi_M")?;
        let mut bidi_m_builder = CodePointInversionListBuilder::new();
        for (start, end) in &bidi_m_data.ranges {
            bidi_m_builder.add_range_u32(&(start..=end));
        }
        let bidi_m_cpinvlist = bidi_m_builder.build();

        // bmg / Bidi_Mirroring_Glyph
        let bmg_data = &get_code_point_prop_map(&self.source, "bmg")?.code_point_trie;
        let bmg_trie = CodePointTrie::try_from(bmg_data).map_err(|e| {
            DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
        })?;

        // bpt / Bidi_Paired_Bracket_Type
        let bpt_data =
            &enum_codepointtrie::get_enumerated_prop(&self.source, "bpt")?.code_point_trie;
        let bpt_trie = CodePointTrie::try_from(bpt_data).map_err(|e| {
            DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
        })?;

        let trie_vals_structured_vec: Vec<MirroredPairedBracketData> = (0..=(char::MAX as u32))
            .map(|cp| MirroredPairedBracketData {
                mirroring_glyph: bmg_trie.get32(cp),
                mirrored: bidi_m_cpinvlist.contains32(cp),
                paired_bracket_type: bpt_trie.get32(cp),
            })
            .collect();
        let trie_data = CodePointTrieBuilderData::ValuesByCodePoint(&trie_vals_structured_vec);

        let trie_builder = CodePointTrieBuilder {
            data: trie_data,
            default_value: MirroredPairedBracketData::default(),
            error_value: MirroredPairedBracketData::default(),
            trie_type: TrieType::Small,
        };
        let trie = trie_builder.build();

        let data_struct = BidiAuxiliaryPropertiesV1::new(trie);

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(data_struct)),
        })
    }
}

impl IterableDataProvider<BidiAuxiliaryPropertiesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}
