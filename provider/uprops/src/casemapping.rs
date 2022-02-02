// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops_serde;
use icu_casemapping::provider::{CaseMappingV1, CaseMappingV1Marker};
use icu_casemapping::CaseMappingInternals;
use icu_codepointtrie::CodePointTrieHeader;
use icu_provider::prelude::*;

use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

pub struct CaseMappingDataProvider {
    case_mapping: CaseMappingInternals<'static>,
}

/// A data provider reading from .toml files produced by the ICU4C icuwriteuprops tool.
impl CaseMappingDataProvider {
    pub fn try_new(path: PathBuf) -> Result<Self, DataError> {
        let toml_str = fs::read_to_string(&path).unwrap();
        let toml: uprops_serde::case::Main = toml::from_str(&toml_str).unwrap();

        let trie_data = &toml.ucase.code_point_trie;
        let trie_header = CodePointTrieHeader::try_from(trie_data)?;
        let trie_index = &trie_data.index;
        let trie_data = &trie_data.data_16.as_ref().ok_or_else(|| {
            DataError::custom("Did not find 16-bit data array for case mapping in TOML")
        })?;
        let exceptions = &toml.ucase.exceptions.exceptions;
        let unfold = &toml.ucase.unfold.unfold;

        let case_mapping = CaseMappingInternals::try_from_icu(
            trie_header,
            trie_index,
            trie_data,
            exceptions,
            unfold,
        )
        .map_err(|e| {
            DataError::custom("Could not create CaseMappingInternals").with_display_context(&e)
        })?;

        Ok(Self { case_mapping })
    }
}

impl DataProvider<CaseMappingV1Marker> for CaseMappingDataProvider {
    fn load_payload(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<CaseMappingV1Marker>, DataError> {
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(CaseMappingV1 {
                casemap: self.case_mapping.clone(),
            })),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::casemapping::CaseMappingDataProvider;
    use icu_casemapping::CaseMapping;

    #[test]
    fn test_upper() {
        let root_dir = icu_testdata::paths::data_root()
            .join("uprops")
            .join("ucase.toml");
        let provider = CaseMappingDataProvider::try_new(root_dir).expect("Loading was successful");
        let case_mapping = CaseMapping::new(&provider).expect("Loading was successful");
        assert_eq!(case_mapping.to_uppercase('a'), 'A');
        assert_eq!(case_mapping.to_uppercase('\u{1c4}'), '\u{1c4}');
        assert_eq!(case_mapping.to_titlecase('\u{1c4}'), '\u{1c5}');
        assert_eq!(case_mapping.to_lowercase('\u{1c4}'), '\u{1c6}');
        assert_eq!(case_mapping.to_uppercase('\u{1c5}'), '\u{1c4}');
        assert_eq!(case_mapping.to_titlecase('\u{1c5}'), '\u{1c5}');
        assert_eq!(case_mapping.to_lowercase('\u{1c5}'), '\u{1c6}');
        assert_eq!(case_mapping.to_uppercase('\u{1c6}'), '\u{1c4}');
        assert_eq!(case_mapping.to_titlecase('\u{1c6}'), '\u{1c5}');
        assert_eq!(case_mapping.to_lowercase('\u{1c6}'), '\u{1c6}');
    }

    #[test]
    fn test_full_upper() {
        let root_dir = icu_testdata::paths::data_root()
            .join("uprops")
            .join("ucase.toml");
        let provider = CaseMappingDataProvider::try_new(root_dir).expect("Loading was successful");
        let case_mapping = CaseMapping::new(&provider).expect("Loading was successful");
        assert_eq!(&case_mapping.to_full_uppercase(""), "");
        assert_eq!(&case_mapping.to_full_uppercase("ABCDEFG"), "ABCDEFG");
        assert_eq!(&case_mapping.to_full_uppercase("abcdefg"), "ABCDEFG");
    }
}
