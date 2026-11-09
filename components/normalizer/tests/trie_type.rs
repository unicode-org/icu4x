// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Constructors must reject data whose trie type differs from the one that
//! the normalization code assumes instead of panicking later.

use icu_collections::codepointtrie::{CodePointTrie, CodePointTrieHeader, TrieType, TrieValue};
use icu_normalizer::properties::CanonicalComposition;
use icu_normalizer::provider::{
    CanonicalCompositionsNew, DecompositionData, NormalizerNfcV2, NormalizerNfdDataV1,
    NormalizerNfdTablesV1, NormalizerNfkdDataV1, NormalizerNfkdTablesV1, NormalizerUts46DataV1,
};
use icu_normalizer::uts46::Uts46Mapper;
use icu_normalizer::{ComposingNormalizer, DecomposingNormalizer};
use icu_provider::prelude::*;
use zerovec::ZeroVec;

/// A structurally valid small-type trie that maps everything to `value`.
fn small_trie<T: TrieValue>(value: u32) -> CodePointTrie<'static, T> {
    let header = CodePointTrieHeader {
        high_start: 0,
        shifted12_high_start: 0,
        index3_null_offset: 0,
        data_null_offset: 0,
        null_value: value,
        trie_type: TrieType::Small,
    };
    let mut arr = [0u16; 1024];
    arr[1] = 64;
    let index = ZeroVec::alloc_from_slice(&arr);
    let data = ZeroVec::alloc_from_slice(&[T::try_from_u32(value).ok().unwrap(); 128]);
    CodePointTrie::try_new(header, index, data).unwrap()
}

/// Serves baked data except for replacing the tries of the selected markers
/// with small-type tries.
struct SmallTrieProvider {
    small_compositions: bool,
    small_decompositions: bool,
}

impl DataProvider<NormalizerNfcV2> for SmallTrieProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<NormalizerNfcV2>, DataError> {
        if !self.small_compositions {
            return icu_normalizer::provider::Baked.load(req);
        }
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CanonicalCompositionsNew {
                trie: small_trie(0),
                linear16: ZeroVec::new(),
                linear24: ZeroVec::new(),
            }),
        })
    }
}

macro_rules! decomposition_data_impl {
    ($marker:ty) => {
        impl DataProvider<$marker> for SmallTrieProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                if !self.small_decompositions {
                    return icu_normalizer::provider::Baked.load(req);
                }
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(DecompositionData {
                        // Every character decomposes to itself, and U+FFFD has
                        // `NON_ROUND_TRIP_MARKER` set as required.
                        trie: small_trie(1 << 30),
                        passthrough_cap: 0xC0,
                    }),
                })
            }
        }
    };
}

decomposition_data_impl!(NormalizerNfdDataV1);
decomposition_data_impl!(NormalizerNfkdDataV1);

impl DataProvider<NormalizerUts46DataV1> for SmallTrieProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<NormalizerUts46DataV1>, DataError> {
        // UTS 46 does not assume a trie type for its decomposition data.
        icu_normalizer::provider::Baked.load(req)
    }
}

impl DataProvider<NormalizerNfdTablesV1> for SmallTrieProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<NormalizerNfdTablesV1>, DataError> {
        icu_normalizer::provider::Baked.load(req)
    }
}

impl DataProvider<NormalizerNfkdTablesV1> for SmallTrieProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<NormalizerNfkdTablesV1>, DataError> {
        icu_normalizer::provider::Baked.load(req)
    }
}

#[test]
fn test_reject_small_composition_trie() {
    let provider = SmallTrieProvider {
        small_compositions: true,
        small_decompositions: false,
    };
    assert!(DecomposingNormalizer::try_new_nfd_unstable(&provider).is_ok());
    assert!(DecomposingNormalizer::try_new_nfkd_unstable(&provider).is_ok());
    assert!(ComposingNormalizer::try_new_nfc_unstable(&provider).is_err());
    assert!(ComposingNormalizer::try_new_nfkc_unstable(&provider).is_err());
    assert!(Uts46Mapper::try_new(&provider).is_err());
    assert!(CanonicalComposition::try_new_unstable(&provider).is_err());
}

#[test]
fn test_reject_small_decomposition_trie() {
    let provider = SmallTrieProvider {
        small_compositions: false,
        small_decompositions: true,
    };
    // With `serde`, the normalizer works with either trie type.
    let accepted = cfg!(feature = "serde");
    assert_eq!(
        DecomposingNormalizer::try_new_nfd_unstable(&provider).is_ok(),
        accepted
    );
    assert_eq!(
        DecomposingNormalizer::try_new_nfkd_unstable(&provider).is_ok(),
        accepted
    );
    assert_eq!(
        ComposingNormalizer::try_new_nfc_unstable(&provider).is_ok(),
        accepted
    );
    assert_eq!(
        ComposingNormalizer::try_new_nfkc_unstable(&provider).is_ok(),
        accepted
    );
    assert!(Uts46Mapper::try_new(&provider).is_ok());
    assert!(CanonicalComposition::try_new_unstable(&provider).is_ok());
}
