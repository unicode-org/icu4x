// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The constructor must reject normalization data whose trie type differs from
//! the one that the comparison code assumes instead of panicking later.

use icu_collator::Collator;
use icu_collator::provider::*;
use icu_collections::codepointtrie::{CodePointTrie, CodePointTrieHeader, TrieType};
use icu_normalizer::provider::{DecompositionData, NormalizerNfdDataV1, NormalizerNfdTablesV1};
use icu_provider::prelude::*;
use zerovec::ZeroVec;

/// Serves baked data except for replacing the NFD trie with a small-type trie.
struct SmallTrieProvider;

macro_rules! baked_impl {
    ($baked:path, $($marker:ty),+) => {
        $(
            impl DataProvider<$marker> for SmallTrieProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    $baked.load(req)
                }
            }
        )+
    };
}

baked_impl!(
    Baked,
    CollationSpecialPrimariesV1,
    CollationRootV1,
    CollationTailoringV1,
    CollationDiacriticsV1,
    CollationJamoV1,
    CollationMetadataV1,
    CollationReorderingV1
);
baked_impl!(icu_normalizer::provider::Baked, NormalizerNfdTablesV1);

impl DataProvider<NormalizerNfdDataV1> for SmallTrieProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<NormalizerNfdDataV1>, DataError> {
        // A structurally valid small-type trie that maps everything to
        // `NON_ROUND_TRIP_MARKER`, i.e. every character decomposes to itself.
        let header = CodePointTrieHeader {
            high_start: 0,
            shifted12_high_start: 0,
            index3_null_offset: 0,
            data_null_offset: 0,
            null_value: 1 << 30,
            trie_type: TrieType::Small,
        };
        let mut arr = [0u16; 1024];
        arr[1] = 64;
        let index = ZeroVec::alloc_from_slice(&arr);
        let data = ZeroVec::alloc_from_slice(&[1u32 << 30; 128]);
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(DecompositionData {
                trie: CodePointTrie::try_new(header, index, data).unwrap(),
                passthrough_cap: 0xC0,
            }),
        })
    }
}

#[test]
fn test_reject_small_decomposition_trie() {
    // With `serde`, the collator works with either trie type.
    assert_eq!(
        Collator::try_new_unstable(&SmallTrieProvider, Default::default(), Default::default())
            .is_ok(),
        cfg!(feature = "serde")
    );
}
