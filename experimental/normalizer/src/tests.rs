// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::DecomposingNormalizer;

#[test]
fn test_basic() {
    let data_provider = icu_testdata::get_provider();

    let normalizer: DecomposingNormalizer = DecomposingNormalizer::try_new(&data_provider).unwrap();
    assert_eq!(normalizer.normalize("ä"), "a\u{0308}");
}
