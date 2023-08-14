// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use icu_casemap::provider::{CaseMapUnfoldV1, CaseMapUnfoldV1Marker, CaseMapV1, CaseMapV1Marker};
use icu_collections::codepointtrie::toml::CodePointDataSlice;
use icu_collections::codepointtrie::CodePointTrieHeader;
use icu_provider::prelude::*;
use std::convert::TryFrom;

mod ucase_serde;

impl DataProvider<CaseMapV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CaseMapV1Marker>, DataError> {
        self.check_req::<CaseMapV1Marker>(req)?;
        let toml = &self
            .icuexport()?
            .read_and_parse_toml::<ucase_serde::Main>(&format!(
                "ucase/{}/ucase.toml",
                self.trie_type()
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

        let case_mapping = CaseMapV1::try_from_icu(trie_header, trie_index, trie_data, exceptions)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(case_mapping)),
        })
    }
}

impl icu_provider::datagen::IterableDataProvider<CaseMapV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl DataProvider<CaseMapUnfoldV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CaseMapUnfoldV1Marker>, DataError> {
        self.check_req::<CaseMapUnfoldV1Marker>(req)?;
        let toml = &self
            .icuexport()?
            .read_and_parse_toml::<ucase_serde::Main>(&format!(
                "ucase/{}/ucase.toml",
                self.trie_type()
            ))?
            .ucase;

        let unfold = &toml.unfold.unfold;

        let unfold = CaseMapUnfoldV1::try_from_icu(unfold)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(unfold)),
        })
    }
}

impl icu_provider::datagen::IterableDataProvider<CaseMapUnfoldV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}
