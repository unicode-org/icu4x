// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Buffer constructors must fall back to `NormalizerNfcV1` data when
//! `NormalizerNfcV2` data is absent.

#![cfg(all(feature = "serde", feature = "compiled_data"))]

use icu_normalizer::properties::{
    CanonicalComposition, CanonicalCompositionBorrowed, CanonicalDecompositionBorrowed, Decomposed,
};
use icu_normalizer::uts46::{Uts46Mapper, Uts46MapperBorrowed};
use icu_provider::buf::AsDeserializingBufferProvider;
use icu_provider_blob::BlobDataProvider;

/// Contains only `NormalizerNfcV1`. Generated with:
///
/// ```text
/// cargo run -p icu4x-datagen --all-features -- \
///     --icuexport-root provider/source/tests/data/icuexport \
///     --markers NormalizerNfcV1 --format blob --overwrite \
///     --out components/normalizer/tests/data/normalizer_nfc_v1.postcard
/// ```
fn v1_provider() -> BlobDataProvider {
    BlobDataProvider::try_new_from_static_blob(include_bytes!("data/normalizer_nfc_v1.postcard"))
        .unwrap()
}

/// Contains `NormalizerNfcV1` and the other markers needed by `Uts46Mapper`,
/// but not `NormalizerNfcV2`. Generated with:
///
/// ```text
/// cargo run -p icu4x-datagen --all-features -- \
///     --icuexport-root provider/source/tests/data/icuexport \
///     --markers NormalizerNfcV1 NormalizerUts46DataV1 NormalizerNfdTablesV1 NormalizerNfkdTablesV1 \
///     --format blob --overwrite \
///     --out components/normalizer/tests/data/normalizer_uts46_nfc_v1.postcard
/// ```
fn uts46_v1_provider() -> BlobDataProvider {
    BlobDataProvider::try_new_from_static_blob(include_bytes!(
        "data/normalizer_uts46_nfc_v1.postcard"
    ))
    .unwrap()
}

#[test]
fn test_canonical_composition_v1_matches_baked() {
    let v1 = CanonicalComposition::try_new_with_buffer_provider(&v1_provider()).unwrap();
    let v1 = v1.as_borrowed();
    let baked = CanonicalCompositionBorrowed::new();
    let decomposition = CanonicalDecompositionBorrowed::new();

    // Every composition is the inverse of a canonical decomposition into a
    // pair, so this covers all compositions (and composition exclusions).
    let mut pairs = 0;
    for c in '\u{0}'..='\u{10FFFF}' {
        if let Decomposed::Expansion(starter, second) = decomposition.decompose(c) {
            assert_eq!(
                v1.compose(starter, second),
                baked.compose(starter, second),
                "U+{:04X}",
                u32::from(c)
            );
            pairs += 1;
        }
    }
    assert!(pairs > 1000);

    // Hangul, astral, and non-composing pairs.
    assert_eq!(v1.compose('\u{1100}', '\u{1161}'), Some('가'));
    assert_eq!(v1.compose('가', '\u{11A8}'), Some('각'));
    assert_eq!(v1.compose('\u{11099}', '\u{110BA}'), Some('\u{1109A}'));
    assert_eq!(v1.compose('a', 'b'), None);
    assert_eq!(v1.compose('a', '\u{0300}'), Some('à'));
}

#[test]
fn test_unstable_constructor_rejects_v1() {
    assert!(CanonicalComposition::try_new_unstable(&v1_provider().as_deserializing()).is_err());
}

#[test]
fn test_uts46_v1_matches_baked() {
    let v1 = Uts46Mapper::try_new_with_buffer_provider(&uts46_v1_provider()).unwrap();
    let v1 = v1.as_borrowed();
    let baked = Uts46MapperBorrowed::new();

    for c in '\u{0}'..='\u{10FFFF}' {
        let input = [c, '\u{0301}'];
        assert!(
            v1.map_normalize(input.iter().copied())
                .eq(baked.map_normalize(input.iter().copied())),
            "U+{:04X}",
            u32::from(c)
        );
        assert!(
            v1.normalize_validate(input.iter().copied())
                .eq(baked.normalize_validate(input.iter().copied())),
            "U+{:04X}",
            u32::from(c)
        );
    }
}

#[test]
fn test_uts46_unstable_constructor_rejects_v1() {
    assert!(Uts46Mapper::try_new(&uts46_v1_provider().as_deserializing()).is_err());
}
