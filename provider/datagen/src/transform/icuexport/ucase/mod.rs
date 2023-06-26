// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use icu_casemapping::provider::{CaseMappingV1, CaseMappingV1Marker};
use icu_collections::codepointtrie::toml::CodePointDataSlice;
use icu_collections::codepointtrie::CodePointTrieHeader;
use icu_provider::prelude::*;
use std::convert::TryFrom;

mod ucase_serde;

impl DataProvider<CaseMappingV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CaseMappingV1Marker>, DataError> {
        self.check_req::<CaseMappingV1Marker>(req)?;
        let toml = &self
            .source
            .icuexport()?
            .read_and_parse_toml::<ucase_serde::Main>(&format!(
                "ucase/{}/ucase.toml",
                self.source.options.trie_type
            ))?
            .ucase;

        let trie_data = &toml.code_point_trie;
        let trie_header = CodePointTrieHeader::try_from(trie_data).map_err(|e| {
            DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
        })?;
        let trie_index = trie_data.index_slice();
        let trie_data = if let Ok(CodePointDataSlice::U16(s)) = trie_data.data_slice() {
            s
        } else {
            return Err(DataError::custom(
                "Did not find 16-bit data array for case mapping in TOML",
            ));
        };
        let exceptions = &toml.exceptions.exceptions;
        let unfold = &toml.unfold.unfold;

        let case_mapping =
            CaseMappingV1::try_from_icu(trie_header, trie_index, trie_data, exceptions, unfold)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(case_mapping)),
        })
    }
}

impl icu_provider::datagen::IterableDataProvider<CaseMappingV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}
