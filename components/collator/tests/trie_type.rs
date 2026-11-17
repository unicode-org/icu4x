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
    ($provider:ty, $baked:path, $($marker:ty),+) => {
        $(
            impl DataProvider<$marker> for $provider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    $baked.load(req)
                }
            }
        )+
    };
}

baked_impl!(
    SmallTrieProvider,
    Baked,
    CollationSpecialPrimariesV1,
    CollationRootV1,
    CollationTailoringV1,
    CollationDiacriticsV1,
    CollationJamoV1,
    CollationMetadataV1,
    CollationReorderingV1
);
baked_impl!(
    SmallTrieProvider,
    icu_normalizer::provider::Baked,
    NormalizerNfdTablesV1
);

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
    // With `serde`, a small-type trie is rebuilt as a fast-type trie.
    assert_eq!(
        Collator::try_new_unstable(&SmallTrieProvider, Default::default(), Default::default())
            .is_ok(),
        cfg!(feature = "serde")
    );
}

/// Serves baked data except for rebuilding the NFD trie with the small type.
#[cfg(feature = "serde")]
struct RebuiltSmallTrieProvider;

#[cfg(feature = "serde")]
baked_impl!(
    RebuiltSmallTrieProvider,
    Baked,
    CollationSpecialPrimariesV1,
    CollationRootV1,
    CollationTailoringV1,
    CollationDiacriticsV1,
    CollationJamoV1,
    CollationMetadataV1,
    CollationReorderingV1
);
#[cfg(feature = "serde")]
baked_impl!(
    RebuiltSmallTrieProvider,
    icu_normalizer::provider::Baked,
    NormalizerNfdTablesV1
);

#[cfg(feature = "serde")]
impl DataProvider<NormalizerNfdDataV1> for RebuiltSmallTrieProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<NormalizerNfdDataV1>, DataError> {
        let baked: DataResponse<NormalizerNfdDataV1> = icu_normalizer::provider::Baked.load(req)?;
        let trie = &baked.payload.get().trie;
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
        Ok(DataResponse {
            metadata: baked.metadata,
            payload: DataPayload::from_owned(DecompositionData {
                trie: rebuilt,
                passthrough_cap: baked.payload.get().passthrough_cap,
            }),
        })
    }
}

#[cfg(feature = "serde")]
#[test]
fn test_small_decomposition_trie_rebuilt_as_fast() {
    let rebuilt = Collator::try_new_unstable(
        &RebuiltSmallTrieProvider,
        Default::default(),
        Default::default(),
    )
    .unwrap();
    let rebuilt = rebuilt.as_borrowed();
    let baked =
        icu_collator::CollatorBorrowed::try_new(Default::default(), Default::default()).unwrap();
    let inputs = [
        "",
        "a",
        "A",
        "\u{00E4}",
        "a\u{0308}",
        "a\u{0323}\u{0308}",
        "a\u{0308}\u{0323}",
        "\u{1E0B}\u{0323}",
        "\u{D55C}\u{AE00}",
        "\u{1100}\u{1161}\u{11A8}",
        "\u{0344}",
        "\u{0F73}\u{0F75}",
        "\u{1D15E}",
        "\u{2F800}",
        "\u{FFFD}",
        "\u{10FFFF}",
    ];
    for left in inputs {
        for right in inputs {
            assert_eq!(
                rebuilt.compare(left, right),
                baked.compare(left, right),
                "{left:?} vs {right:?}"
            );
            let left16: Vec<u16> = left.encode_utf16().collect();
            let right16: Vec<u16> = right.encode_utf16().collect();
            assert_eq!(
                rebuilt.compare_utf16(&left16, &right16),
                baked.compare_utf16(&left16, &right16),
                "{left:?} vs {right:?}"
            );
        }
    }
}
