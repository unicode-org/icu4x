// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations for Unihan radicals.

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
use crate::AbstractFs;
use crate::{IterableDataProviderCached, SourceDataProvider};
use icu::collections::codepointtrie;
use icu::segmenter::provider::radical::{SegmenterUnihanRadicalV1, UnihanRadicalsData};
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
use icu_codepointtrie_builder::CodePointTrieBuilder;
use icu_provider::prelude::*;
use std::collections::HashSet;

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
fn build_unihan_radicals_data(
    unihan: &AbstractFs,
    identifier_status: &AbstractFs,
    trie_type: crate::TrieType,
) -> Result<UnihanRadicalsData<'static>, DataError> {
    let identifier_status = identifier_status.read_to_string("security/IdentifierStatus.txt")?;
    let identifier_status = identifier_status
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            let field = line.split(';').next().unwrap().trim();
            let (start, end) = field.split_once("..").unwrap_or((field, field));
            (
                u32::from_str_radix(start, 16).expect("Invalid IdentifierStatus codepoint format"),
                u32::from_str_radix(end, 16).expect("Invalid IdentifierStatus codepoint format"),
            )
        })
        .collect::<Vec<_>>();

    let raw_content = unihan.read_to_string("Unihan_IRGSources.txt")?;
    let mut builder = CodePointTrieBuilder::new(
        0u8,
        0u8,
        match trie_type {
            crate::TrieType::Fast => codepointtrie::TrieType::Fast,
            crate::TrieType::Small => codepointtrie::TrieType::Small,
        },
    );

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

        let codepoint_idx = identifier_status.partition_point(|(start, _)| *start <= codepoint);
        if codepoint_idx == 0 || identifier_status[codepoint_idx - 1].1 < codepoint {
            continue;
        }

        let mut candidate = parts[2].trim();
        if let Some(first_part) = candidate.split_whitespace().next() {
            candidate = first_part;
        }
        let radical_str = if let Some(idx) = candidate.find('.') {
            &candidate[..idx]
        } else {
            candidate
        };
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

#[cfg(all(test, any(feature = "use_wasm", feature = "use_icu4c")))]
mod tests {
    use super::build_unihan_radicals_data;
    use crate::SourceDataProvider;
    use icu::segmenter::provider::radical::SegmenterUnihanRadicalV1;
    use icu_provider::prelude::*;

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
