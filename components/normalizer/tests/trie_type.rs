// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Constructors must reject data whose trie type differs from the one that
//! the normalization code assumes instead of panicking later.

use icu_collections::codepointtrie::{CodePointTrie, CodePointTrieHeader, TrieType};
use icu_normalizer::properties::CanonicalComposition;
use icu_normalizer::provider::{
    DecompositionData, NormalizerNfcV1, NormalizerNfdDataV1, NormalizerNfdTablesV1,
    NormalizerNfkdDataV1, NormalizerNfkdTablesV1, NormalizerUts46DataV1,
};
use icu_normalizer::uts46::Uts46Mapper;
use icu_normalizer::{ComposingNormalizer, DecomposingNormalizer};
use icu_provider::prelude::*;
use zerovec::ZeroVec;

/// A structurally valid small-type trie that maps everything to `value`.
fn small_trie(value: u32) -> CodePointTrie<'static, u32> {
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
    let data = ZeroVec::alloc_from_slice(&[value; 128]);
    CodePointTrie::try_new(header, index, data).unwrap()
}

/// Serves baked data except for replacing the NFD and NFKD tries with
/// small-type tries.
struct SmallTrieProvider;

impl DataProvider<NormalizerNfcV1> for SmallTrieProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<NormalizerNfcV1>, DataError> {
        icu_normalizer::provider::Baked.load(req)
    }
}

macro_rules! decomposition_data_impl {
    ($marker:ty) => {
        impl DataProvider<$marker> for SmallTrieProvider {
            fn load(&self, _req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
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
fn test_reject_small_decomposition_trie() {
    let provider = SmallTrieProvider;
    // With `serde`, a small-type trie is rebuilt as a fast-type trie.
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

/// Serves baked data except for rebuilding the NFD and NFKD tries with the
/// small type.
#[cfg(feature = "serde")]
struct RebuiltSmallTrieProvider;

#[cfg(feature = "serde")]
fn rebuild_as_small(trie: &CodePointTrie<'_, u32>) -> CodePointTrie<'static, u32> {
    let mut builder = icu_collections::codepointtrie::CodePointTrieBuilder::new(
        trie.null_value(),
        trie.error_value(),
        TrieType::Small,
    );
    for range in trie.iter_ranges() {
        builder.set_range_value(range.range, range.value);
    }
    let rebuilt = builder.build();
    assert!(matches!(
        rebuilt.as_typed_ref(),
        icu_collections::codepointtrie::Typed::Small(_)
    ));
    rebuilt
}

#[cfg(feature = "serde")]
macro_rules! rebuilt_decomposition_data_impl {
    ($marker:ty) => {
        impl DataProvider<$marker> for RebuiltSmallTrieProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                let baked: DataResponse<$marker> = icu_normalizer::provider::Baked.load(req)?;
                Ok(DataResponse {
                    metadata: baked.metadata,
                    payload: DataPayload::from_owned(DecompositionData {
                        trie: rebuild_as_small(&baked.payload.get().trie),
                        passthrough_cap: baked.payload.get().passthrough_cap,
                    }),
                })
            }
        }
    };
}

#[cfg(feature = "serde")]
rebuilt_decomposition_data_impl!(NormalizerNfdDataV1);
#[cfg(feature = "serde")]
rebuilt_decomposition_data_impl!(NormalizerNfkdDataV1);

#[cfg(feature = "serde")]
macro_rules! baked_impl {
    ($marker:ty) => {
        impl DataProvider<$marker> for RebuiltSmallTrieProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                icu_normalizer::provider::Baked.load(req)
            }
        }
    };
}

#[cfg(feature = "serde")]
baked_impl!(NormalizerNfdTablesV1);
#[cfg(feature = "serde")]
baked_impl!(NormalizerNfkdTablesV1);
#[cfg(feature = "serde")]
baked_impl!(NormalizerNfcV1);

#[cfg(feature = "serde")]
#[test]
fn test_small_decomposition_trie_rebuilt_as_fast() {
    let provider = RebuiltSmallTrieProvider;
    let inputs = [
        "ascii",
        "\u{00E4}\u{00C5}\u{1E0B}\u{0323}",
        "a\u{0301}\u{0323}\u{0308}",
        "\u{D55C}\u{AE00}\u{1100}\u{1161}\u{11A8}",
        "\u{FDFA}\u{FB01}\u{2460}\u{3392}",
        "\u{1D15E}\u{1D160}\u{2F800}\u{1F600}",
        "\u{0344}\u{0F73}\u{0F75}\u{0F81}",
        "\u{FFFD}\u{10FFFF}",
    ];
    let pairs = [
        (
            DecomposingNormalizer::try_new_nfd_unstable(&provider).unwrap(),
            DecomposingNormalizer::new_nfd(),
        ),
        (
            DecomposingNormalizer::try_new_nfkd_unstable(&provider).unwrap(),
            DecomposingNormalizer::new_nfkd(),
        ),
    ];
    for (rebuilt, baked) in &pairs {
        let rebuilt = rebuilt.as_borrowed();
        for input in inputs {
            assert_eq!(rebuilt.normalize(input), baked.normalize(input));
            let utf16: Vec<u16> = input.encode_utf16().collect();
            assert_eq!(
                rebuilt.normalize_utf16(&utf16),
                baked.normalize_utf16(&utf16)
            );
        }
    }
    let composing = [
        (
            ComposingNormalizer::try_new_nfc_unstable(&provider).unwrap(),
            ComposingNormalizer::new_nfc(),
        ),
        (
            ComposingNormalizer::try_new_nfkc_unstable(&provider).unwrap(),
            ComposingNormalizer::new_nfkc(),
        ),
    ];
    for (rebuilt, baked) in &composing {
        let rebuilt = rebuilt.as_borrowed();
        for input in inputs {
            assert_eq!(rebuilt.normalize(input), baked.normalize(input));
        }
    }
}
