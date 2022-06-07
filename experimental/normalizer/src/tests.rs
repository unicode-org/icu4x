// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::DecomposingNormalizer;

#[test]
fn test_nfd_basic() {
    let data_provider = icu_testdata::get_provider();

    let normalizer: DecomposingNormalizer =
        DecomposingNormalizer::try_new_nfd(&data_provider).unwrap();
    assert_eq!(normalizer.normalize("ä"), "a\u{0308}");
    assert_eq!(normalizer.normalize("Ä"), "A\u{0308}");
    assert_eq!(normalizer.normalize("ệ"), "e\u{0323}\u{0302}");
    assert_eq!(normalizer.normalize("Ệ"), "E\u{0323}\u{0302}");
    assert_eq!(normalizer.normalize("𝅗𝅥"), "𝅗\u{1D165}");
    assert_eq!(normalizer.normalize("\u{2126}"), "Ω"); // ohm sign
    assert_eq!(normalizer.normalize("ﾍﾞ"), "ﾍﾞ"); // half-width unchanged
    assert_eq!(normalizer.normalize("ﬁ"), "ﬁ"); // ligature unchanged
    assert_eq!(normalizer.normalize("\u{FDFA}"), "\u{FDFA}"); // ligature unchanged
}

#[test]
fn test_nfkd_basic() {
    let data_provider = icu_testdata::get_provider();

    let normalizer: DecomposingNormalizer =
        DecomposingNormalizer::try_new_nfkd(&data_provider).unwrap();
    assert_eq!(normalizer.normalize("ä"), "a\u{0308}");
    assert_eq!(normalizer.normalize("Ä"), "A\u{0308}");
    assert_eq!(normalizer.normalize("ệ"), "e\u{0323}\u{0302}");
    assert_eq!(normalizer.normalize("Ệ"), "E\u{0323}\u{0302}");
    assert_eq!(normalizer.normalize("𝅗𝅥"), "𝅗\u{1D165}");
    assert_eq!(normalizer.normalize("\u{2126}"), "Ω"); // ohm sign
    assert_eq!(normalizer.normalize("ﾍﾞ"), "ヘ\u{3099}"); // half-width to full-width
    assert_eq!(normalizer.normalize("ﬁ"), "fi"); // ligature expanded
    assert_eq!(normalizer.normalize("\u{FDFA}"), "\u{635}\u{644}\u{649} \u{627}\u{644}\u{644}\u{647} \u{639}\u{644}\u{64A}\u{647} \u{648}\u{633}\u{644}\u{645}");
    // ligature expanded
}

#[test]
fn test_uts46d_basic() {
    let data_provider = icu_testdata::get_provider();

    let normalizer: DecomposingNormalizer =
        DecomposingNormalizer::try_new_uts46_decomposed_without_ignored_and_disallowed(
            &data_provider,
        )
        .unwrap();
    assert_eq!(normalizer.normalize("ä"), "a\u{0308}");
    assert_eq!(normalizer.normalize("Ä"), "a\u{0308}");
    assert_eq!(normalizer.normalize("ệ"), "e\u{0323}\u{0302}");
    assert_eq!(normalizer.normalize("Ệ"), "e\u{0323}\u{0302}");
    assert_eq!(normalizer.normalize("𝅗𝅥"), "𝅗\u{1D165}");
    assert_eq!(normalizer.normalize("\u{2126}"), "ω"); // ohm sign
    assert_eq!(normalizer.normalize("ﾍﾞ"), "ヘ\u{3099}"); // half-width to full-width
    assert_eq!(normalizer.normalize("ﬁ"), "fi"); // ligature expanded
    assert_eq!(normalizer.normalize("\u{FDFA}"), "\u{635}\u{644}\u{649} \u{627}\u{644}\u{644}\u{647} \u{639}\u{644}\u{64A}\u{647} \u{648}\u{633}\u{644}\u{645}");
    // ligature expanded
}
