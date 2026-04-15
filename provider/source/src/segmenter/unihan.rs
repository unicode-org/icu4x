// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations for Unihan radicals.

use crate::{IterableDataProviderCached, SourceDataProvider};
use icu::collections::codepointtrie;
use icu::segmenter::provider::radical::{SegmenterUnihanRadicalV1, UnihanIrgData};
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
use icu_codepointtrie_builder::CodePointTrieBuilder;
use icu_provider::prelude::*;
use std::collections::HashSet;

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

            let unihan_cache = self.unihan()?;
            let ucd = self.ucd()?;
            let irg_map = unihan_cache.irg_sources(ucd)?;

            let mut builder = CodePointTrieBuilder::new(
                0u8,
                0u8,
                match self.trie_type() {
                    crate::TrieType::Fast => codepointtrie::TrieType::Fast,
                    crate::TrieType::Small => codepointtrie::TrieType::Small,
                },
            );

            for (ch, irg_val) in irg_map.iter() {
                builder.set_value(*ch as u32, irg_val.value);
            }

            let trie = builder.build();

            Ok(DataResponse {
                metadata: Default::default(),
                payload: DataPayload::from_owned(UnihanIrgData { trie }),
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
    use crate::SourceDataProvider;
    use icu::segmenter::provider::radical::SegmenterUnihanRadicalV1;
    use icu_provider::prelude::*;

    #[test]
    fn test_chinese_irg_values() {
        let provider = SourceDataProvider::new_testing();

        let unihan_cache = provider.unihan().expect("Unihan data should be available");
        let ucd = provider.ucd().expect("UCD data should be available");

        let irg_map = unihan_cache
            .irg_sources(ucd)
            .expect("Should be able to parse Unihan_IRGSources.txt");

        assert_eq!(irg_map.get(&'我').map(|v| v.value), Some(62));
        assert_eq!(irg_map.get(&'爱').map(|v| v.value), Some(87));
        assert_eq!(irg_map.get(&'中').map(|v| v.value), Some(2));
        assert_eq!(irg_map.get(&'文').map(|v| v.value), Some(67));
    }

    #[test]
    fn test_chinese_irg_values_trie() {
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
