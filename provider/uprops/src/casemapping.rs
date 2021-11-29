// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops_serde;
use icu_casemapping::provider::{CaseMappingV1, CaseMappingV1Marker};
use icu_casemapping::CaseMapping;
use icu_codepointtrie::{CodePointTrieHeader, TrieType};
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
            trie_type,
        };
        let trie_index = &trie_data.index;
        let trie_data = &trie_data.data_16.as_ref().ok_or_else(|| {
            DataError::new_resc_error(icu_codepointtrie::error::Error::FromDeserialized {
                reason: "Did not find 16-bit data array for case mapping in TOML",
            })
        })?;
        let exceptions = &toml.ucase.exceptions.exceptions;
        let unfold = &toml.ucase.unfold.unfold;

        let case_mapping =
            CaseMapping::try_from_icu(trie_header, trie_index, trie_data, exceptions, unfold)
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
            payload: Some(DataPayload::from_owned(CaseMappingV1 {
                casemap: self.case_mapping.clone(),
            })),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::casemapping::CaseMappingDataProvider;
    use icu_casemapping::{internals::*, provider::*};
    use icu_provider::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn test_upper() {
        let root_dir = icu_testdata::paths::data_root()
            .join("uprops")
            .join("ucase.toml");
        let provider = CaseMappingDataProvider::try_new(root_dir).expect("Loading was successful");
        let payload: DataPayload<CaseMappingV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::CASE_MAPPING_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was succesful");
        let case_mapping = &payload.get().casemap;
        assert_eq!(case_mapping.to_upper('a'), 'A');
        assert_eq!(case_mapping.to_upper('\u{1c4}'), '\u{1c4}');
        assert_eq!(case_mapping.to_title('\u{1c4}'), '\u{1c5}');
        assert_eq!(case_mapping.to_lower('\u{1c4}'), '\u{1c6}');
        assert_eq!(case_mapping.to_upper('\u{1c5}'), '\u{1c4}');
        assert_eq!(case_mapping.to_title('\u{1c5}'), '\u{1c5}');
        assert_eq!(case_mapping.to_lower('\u{1c5}'), '\u{1c6}');
        assert_eq!(case_mapping.to_upper('\u{1c6}'), '\u{1c4}');
        assert_eq!(case_mapping.to_title('\u{1c6}'), '\u{1c5}');
        assert_eq!(case_mapping.to_lower('\u{1c6}'), '\u{1c6}');
    }

    #[test]
    fn test_softdotted() {
        let root_dir = icu_testdata::paths::data_root()
            .join("uprops")
            .join("ucase.toml");
        let provider = CaseMappingDataProvider::try_new(root_dir).expect("Loading was successful");
        let payload: DataPayload<CaseMappingV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::CASE_MAPPING_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was succesful");
        let case_mapping = &payload.get().casemap;
        assert_eq!(case_mapping.is_soft_dotted('a'), false);
        assert_eq!(case_mapping.is_soft_dotted('i'), true);
        assert_eq!(case_mapping.is_soft_dotted('j'), true);
    }

    #[derive(Eq, PartialEq, Default)]
    struct SimpleSet {
        chars: HashSet<char>,
        strings: HashSet<String>,
    }

    impl SimpleSet {
        pub fn chars(&self) -> Vec<char> {
            let mut result: Vec<char> = self.chars.iter().map(|&c| c).collect();
            result.sort();
            result
        }
        pub fn strings(&self) -> Vec<String> {
            let mut result: Vec<String> = self.strings.iter().map(|c| c.clone()).collect();
            result.sort();
            result
        }
        pub fn clear(&mut self) {
            self.chars.clear();
            self.strings.clear();
        }
    }

    impl SetAdder for SimpleSet {
        fn add_char(&mut self, c: char) {
            self.chars.insert(c);
        }
        fn add_string(&mut self, s: &str) {
            self.strings.insert(String::from(s));
        }
    }

    #[test]
    fn test_closure() {
        let root_dir = icu_testdata::paths::data_root()
            .join("uprops")
            .join("ucase.toml");
        let provider = CaseMappingDataProvider::try_new(root_dir).expect("Loading was successful");
        let payload: DataPayload<CaseMappingV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::CASE_MAPPING_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was succesful");
        let case_mapping = &payload.get().casemap;
        let mut closure = SimpleSet::default();

        case_mapping.add_case_closure('i', &mut closure);
        assert_eq!(closure.chars(), vec!['I']);
        assert!(closure.strings().is_empty());
        closure.clear();

        case_mapping.add_case_closure('k', &mut closure);
        assert_eq!(closure.chars(), vec!['K', '\u{212a}']); // Kelvin sign
        assert!(closure.strings().is_empty());
        closure.clear();

        case_mapping.add_case_closure('s', &mut closure);
        assert_eq!(closure.chars(), vec!['S', '\u{17f}']); // long S
        assert!(closure.strings().is_empty());
        closure.clear();

        case_mapping.add_case_closure('\u{df}', &mut closure); // lowercase sharp s
        assert_eq!(closure.chars(), vec!['\u{1e9e}']); // uppercase sharp s
        assert_eq!(closure.strings(), vec![String::from("ss")]);
        closure.clear();
    }
}
