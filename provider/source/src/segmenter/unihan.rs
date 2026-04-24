// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations for Unihan radicals.

use crate::AbstractFs;
use crate::{IterableDataProviderCached, SourceDataProvider};
use icu::collections::codepointinvlist::CodePointInversionListBuilder;
use icu::segmenter::provider::radical::{SegmenterUnihanRadicalV1, UnihanRadicalsData};
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
use icu_codepointtrie_builder::CodePointTrieBuilder;
use icu_provider::prelude::*;
use std::collections::HashSet;

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
fn build_unihan_radicals_data(
    unihan: &AbstractFs,
    ucd: &AbstractFs,
    trie_type: crate::TrieType,
) -> Result<UnihanRadicalsData<'static>, DataError> {
    let identifier_status = ucd.read_to_string("security/IdentifierStatus.txt")?;
    let mut id_builder = CodePointInversionListBuilder::new();
    for line in identifier_status.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let field = line.split(';').next().unwrap().trim();
        let (start, end) = field.split_once("..").unwrap_or((field, field));

        let start =
            u32::from_str_radix(start, 16).expect("Invalid IdentifierStatus codepoint format");
        let end = u32::from_str_radix(end, 16).expect("Invalid IdentifierStatus codepoint format");

        id_builder.add_range32(start..=end);
    }
    let identifier_status = id_builder.build();

    let raw_content = unihan.read_to_string("Unihan_IRGSources.txt")?;
    let mut builder = CodePointTrieBuilder::new(0u8, 0u8, trie_type.into());

    for line in raw_content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.trim().split('\t').collect();
        if parts[1] != "kRSUnicode" {
            continue;
        }
        let codepoint = parts[0]
            .strip_prefix("U+")
            .and_then(|hex| u32::from_str_radix(hex, 16).ok())
            .expect("Invalid Unihan codepoint format");

        if !identifier_status.contains32(codepoint) {
            continue;
        }

        let mut candidate = parts[2].trim();
        if let Some(first_part) = candidate.split_whitespace().next() {
            candidate = first_part;
        }
        let radical_str = candidate.split('.').next().unwrap_or(candidate);
        let clean_str = radical_str.replace('\'', "");
        if let Ok(value) = clean_str.parse::<u8>() {
            builder.set_value(codepoint, value);
        }
    }

    let trie = builder.build();

    Ok(UnihanRadicalsData { trie })
}

impl DataProvider<SegmenterUnihanRadicalV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterUnihanRadicalV1>, DataError> {
        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(
            DataError::custom("Unihan data generation requires use_wasm or use_icu4c")
                .with_req(SegmenterUnihanRadicalV1::INFO, req),
        );

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        {
            self.check_req::<SegmenterUnihanRadicalV1>(req)?;

            let unihan = self.unihan()?;
            let ucd = self.ucd()?;
            let data = build_unihan_radicals_data(unihan, ucd, self.trie_type())?;

            Ok(DataResponse {
                metadata: Default::default(),
                payload: DataPayload::from_owned(data),
            })
        }
    }
}

impl IterableDataProviderCached<SegmenterUnihanRadicalV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_radical_values_trie() {
        let provider = SourceDataProvider::new_testing();

        let response: DataResponse<SegmenterUnihanRadicalV1> = provider
            .load(DataRequest::default())
            .expect("Failed to build CodePointTrie from Unihan data");

        let trie = &response.payload.get().trie;

        assert_eq!(trie.get('我'), 62);
        assert_eq!(trie.get('爱'), 87);
        assert_eq!(trie.get('中'), 2);
        assert_eq!(trie.get('文'), 67);

        assert_eq!(trie.get('A'), 0);
    }
}
