// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops_serde;
use icu_casemapping::CaseMapping;
use icu_codepointtrie::{CodePointTrieHeader, TrieType};
use icu_casemapping::provider::{CaseMappingV1, CaseMappingV1Marker};
use icu_provider::prelude::*;

use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

pub struct CaseMappingDataProvider {
    case_mapping: CaseMapping<'static>,
}

/// A data provider reading from .toml files produced by the ICU4C icuwriteuprops tool.
impl CaseMappingDataProvider {
    pub fn try_new(path: PathBuf) -> Result<Self, DataError> {
        let toml_str = fs::read_to_string(&path).unwrap();
        let toml: uprops_serde::case::Main = toml::from_str(&toml_str).unwrap();

	let trie_data = &toml.ucase.code_point_trie;
        let trie_type: TrieType =
            TrieType::try_from(trie_data.trie_type_enum_val).map_err(DataError::new_resc_error)?;
        let trie_header = CodePointTrieHeader {
            high_start: trie_data.high_start,
            shifted12_high_start: trie_data.shifted12_high_start,
            index3_null_offset: trie_data.index3_null_offset,
            data_null_offset: trie_data.data_null_offset,
            null_value: trie_data.null_value,
            trie_type: trie_type,
        };
	let trie_index = &trie_data.index;
	let trie_data = &trie_data.data_16.as_ref().unwrap(); // TODO: return error
	let exceptions = &toml.ucase.exceptions.exceptions;
 	let unfold = &toml.ucase.unfold.unfold;

	let case_mapping = CaseMapping::try_from_icu(trie_header, trie_index, trie_data,
						     exceptions, unfold)
	    .map_err(DataError::new_resc_error)?;

	Ok(Self { case_mapping })
    }
}

impl DataProvider<CaseMappingV1Marker> for CaseMappingDataProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<CaseMappingV1Marker>, DataError> {
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(
                CaseMappingV1 { casemap: self.case_mapping.clone() }
            )),
        })
    }
}

// icu_provider::impl_dyn_provider!(BinaryPropertyUnicodeSetDataProvider, {
//     _ => UnicodePropertyV1Marker,
// }, SERDE_SE, 'data);

// impl IterableDataProviderCore for BinaryPropertyUnicodeSetDataProvider {
//     fn supported_options_for_key(
//         &self,
//         _resc_key: &ResourceKey,
//     ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
//         let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
//         Ok(Box::new(list.into_iter()))
//     }
// }

#[test]
fn test_basic() {
    let root_dir = icu_testdata::paths::data_root().join("uprops").join("ucase.toml");
    CaseMappingDataProvider::try_new(root_dir).expect("Loading was successful");
}
