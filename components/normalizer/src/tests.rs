// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CanonicalComposition;
use crate::CanonicalDecomposition;
use crate::ComposingNormalizer;
use crate::Decomposed;
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
    assert_eq!(normalizer.normalize("㈎"), "㈎"); // parenthetical unchanged
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
    assert_eq!(normalizer.normalize("㈎"), "(\u{1100}\u{1161})"); // parenthetical expanded
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
    assert_eq!(normalizer.normalize("㈎"), "(\u{1100}\u{1161})"); // parenthetical expanded

    // Deviations (UTS 46, 6 Mapping Table Derivation, Step 4)
    assert_eq!(normalizer.normalize("\u{200C}"), "\u{200C}");
    assert_eq!(normalizer.normalize("\u{200D}"), "\u{200D}");
    assert_eq!(normalizer.normalize("ß"), "ß");
    assert_eq!(normalizer.normalize("ς"), "ς");
}

#[test]
fn test_nfc_basic() {
    let data_provider = icu_testdata::get_provider();

    let normalizer: ComposingNormalizer = ComposingNormalizer::try_new_nfc(&data_provider).unwrap();
    assert_eq!(normalizer.normalize("a\u{0308}"), "ä");
    assert_eq!(normalizer.normalize("A\u{0308}"), "Ä");
    assert_eq!(normalizer.normalize("e\u{0323}\u{0302}"), "ệ");
    assert_eq!(normalizer.normalize("E\u{0323}\u{0302}"), "Ệ");
    assert_eq!(normalizer.normalize("𝅗𝅥"), "𝅗\u{1D165}"); // Composition exclusion

    assert_eq!(normalizer.normalize("\u{2126}"), "Ω"); // ohm sign
    assert_eq!(normalizer.normalize("ﾍﾞ"), "ﾍﾞ"); // half-width unchanged
    assert_eq!(normalizer.normalize("ﬁ"), "ﬁ"); // ligature unchanged
    assert_eq!(normalizer.normalize("\u{FDFA}"), "\u{FDFA}"); // ligature unchanged
    assert_eq!(normalizer.normalize("㈎"), "㈎"); // parenthetical unchanged
}

#[test]
fn test_nfkc_basic() {
    let data_provider = icu_testdata::get_provider();

    let normalizer: ComposingNormalizer =
        ComposingNormalizer::try_new_nfkc(&data_provider).unwrap();
    assert_eq!(normalizer.normalize("a\u{0308}"), "ä");
    assert_eq!(normalizer.normalize("A\u{0308}"), "Ä");
    assert_eq!(normalizer.normalize("e\u{0323}\u{0302}"), "ệ");
    assert_eq!(normalizer.normalize("E\u{0323}\u{0302}"), "Ệ");
    assert_eq!(normalizer.normalize("𝅗𝅥"), "𝅗\u{1D165}"); // Composition exclusion

    assert_eq!(normalizer.normalize("\u{2126}"), "Ω"); // ohm sign
    assert_eq!(normalizer.normalize("ﾍﾞ"), "ベ"); // half-width to full-width, the compose
    assert_eq!(normalizer.normalize("ﬁ"), "fi"); // ligature expanded
    assert_eq!(normalizer.normalize("\u{FDFA}"), "\u{0635}\u{0644}\u{0649} \u{0627}\u{0644}\u{0644}\u{0647} \u{0639}\u{0644}\u{064A}\u{0647} \u{0648}\u{0633}\u{0644}\u{0645}");
    // ligature expanded
    assert_eq!(normalizer.normalize("㈎"), "(가)"); // parenthetical expanded and partially recomposed
}

#[test]
fn test_uts46_basic() {
    let data_provider = icu_testdata::get_provider();

    let normalizer: ComposingNormalizer =
        ComposingNormalizer::try_new_uts46_without_ignored_and_disallowed(&data_provider).unwrap();
    assert_eq!(normalizer.normalize("a\u{0308}"), "ä");
    assert_eq!(normalizer.normalize("A\u{0308}"), "ä");
    assert_eq!(normalizer.normalize("e\u{0323}\u{0302}"), "ệ");
    assert_eq!(normalizer.normalize("E\u{0323}\u{0302}"), "ệ");
    assert_eq!(normalizer.normalize("𝅗𝅥"), "𝅗\u{1D165}"); // Composition exclusion

    assert_eq!(normalizer.normalize("\u{2126}"), "ω"); // ohm sign
    assert_eq!(normalizer.normalize("ﾍﾞ"), "ベ"); // half-width to full-width, the compose
    assert_eq!(normalizer.normalize("ﬁ"), "fi"); // ligature expanded
    assert_eq!(normalizer.normalize("\u{FDFA}"), "\u{0635}\u{0644}\u{0649} \u{0627}\u{0644}\u{0644}\u{0647} \u{0639}\u{0644}\u{064A}\u{0647} \u{0648}\u{0633}\u{0644}\u{0645}");
    // ligature expanded
    assert_eq!(normalizer.normalize("㈎"), "(가)"); // parenthetical expanded and partially recomposed

    // Deviations (UTS 46, 6 Mapping Table Derivation, Step 4)
    assert_eq!(normalizer.normalize("\u{200C}"), "\u{200C}");
    assert_eq!(normalizer.normalize("\u{200D}"), "\u{200D}");
    assert_eq!(normalizer.normalize("ß"), "ß");
    assert_eq!(normalizer.normalize("ς"), "ς");
}

use atoi::FromRadix16;
type StackString = arraystring::ArrayString<arraystring::typenum::U48>;

/// Parse five semicolon-terminated strings consisting of space-separated hexadecimal scalar values
fn parse_hex(mut hexes: &[u8]) -> [StackString; 5] {
    let mut strings = [
        StackString::new(),
        StackString::new(),
        StackString::new(),
        StackString::new(),
        StackString::new(),
    ];
    let mut current = 0;
    loop {
        let (scalar, mut offset) = u32::from_radix_16(hexes);
        let c = core::char::from_u32(scalar).unwrap();
        strings[current].try_push(c).unwrap();
        match hexes[offset] {
            b';' => {
                current += 1;
                if current == strings.len() {
                    return strings;
                }
                offset += 1;
            }
            b' ' => {
                offset += 1;
            }
            _ => {
                panic!("Bad format: Garbage");
            }
        }
        hexes = &hexes[offset..];
    }
}

#[test]
fn test_conformance() {
    let data_provider = icu_testdata::get_provider();

    let nfd: DecomposingNormalizer = DecomposingNormalizer::try_new_nfd(&data_provider).unwrap();
    let nfkd: DecomposingNormalizer = DecomposingNormalizer::try_new_nfkd(&data_provider).unwrap();
    let nfc: ComposingNormalizer = ComposingNormalizer::try_new_nfc(&data_provider).unwrap();
    let nfkc: ComposingNormalizer = ComposingNormalizer::try_new_nfkc(&data_provider).unwrap();

    let mut prev = 0u32;
    let mut part = 0u8;
    let data = include_bytes!("../testdata/NormalizationTest.txt");
    let lines = data.split(|b| b == &b'\n');
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.starts_with(b"#") {
            continue;
        }
        if line.starts_with(&b"@Part"[..]) {
            part = line[5] - b'0';
            if part == 2 {
                for u in prev + 1..=0x10FFFF {
                    if let Some(c) = char::from_u32(u) {
                        assert!(nfd
                            .normalize_iter(core::iter::once(c))
                            .eq(core::iter::once(c)));
                        assert!(nfkd
                            .normalize_iter(core::iter::once(c))
                            .eq(core::iter::once(c)));
                        assert!(nfc
                            .normalize_iter(core::iter::once(c))
                            .eq(core::iter::once(c)));
                        assert!(nfkc
                            .normalize_iter(core::iter::once(c))
                            .eq(core::iter::once(c)));
                    }
                }
            }
            continue;
        }
        let strings = parse_hex(line);
        // 0: source
        // 1: NFC
        // 2: NFD
        // 3: NFKC
        // 4: NFKD
        if part == 1 {
            let mut iter = strings[0].chars();
            let current = iter.next().unwrap();
            assert_eq!(iter.next(), None);
            let current_u = u32::from(current);
            for u in prev + 1..current_u {
                if let Some(c) = char::from_u32(u) {
                    assert!(nfd
                        .normalize_iter(core::iter::once(c))
                        .eq(core::iter::once(c)));
                    assert!(nfkd
                        .normalize_iter(core::iter::once(c))
                        .eq(core::iter::once(c)));
                    assert!(nfc
                        .normalize_iter(core::iter::once(c))
                        .eq(core::iter::once(c)));
                    assert!(nfkc
                        .normalize_iter(core::iter::once(c))
                        .eq(core::iter::once(c)));
                }
            }
            prev = current_u;
        }
        // NFC
        assert!(nfc
            .normalize_iter(strings[0].chars())
            .eq(strings[1].chars()));
        assert!(nfc
            .normalize_iter(strings[1].chars())
            .eq(strings[1].chars()));
        assert!(nfc
            .normalize_iter(strings[2].chars())
            .eq(strings[1].chars()));

        assert!(nfc
            .normalize_iter(strings[3].chars())
            .eq(strings[3].chars()));
        assert!(nfc
            .normalize_iter(strings[4].chars())
            .eq(strings[3].chars()));

        // NFD
        assert!(nfd
            .normalize_iter(strings[0].chars())
            .eq(strings[2].chars()));
        assert!(nfd
            .normalize_iter(strings[1].chars())
            .eq(strings[2].chars()));
        assert!(nfd
            .normalize_iter(strings[2].chars())
            .eq(strings[2].chars()));

        assert!(nfd
            .normalize_iter(strings[3].chars())
            .eq(strings[4].chars()));
        assert!(nfd
            .normalize_iter(strings[4].chars())
            .eq(strings[4].chars()));

        // NFKC
        assert!(nfkc
            .normalize_iter(strings[0].chars())
            .eq(strings[3].chars()));
        assert!(nfkc
            .normalize_iter(strings[1].chars())
            .eq(strings[3].chars()));
        assert!(nfkc
            .normalize_iter(strings[2].chars())
            .eq(strings[3].chars()));
        assert!(nfkc
            .normalize_iter(strings[3].chars())
            .eq(strings[3].chars()));
        assert!(nfkc
            .normalize_iter(strings[4].chars())
            .eq(strings[3].chars()));

        // NFKD
        assert!(nfkd
            .normalize_iter(strings[0].chars())
            .eq(strings[4].chars()));
        assert!(nfkd
            .normalize_iter(strings[1].chars())
            .eq(strings[4].chars()));
        assert!(nfkd
            .normalize_iter(strings[2].chars())
            .eq(strings[4].chars()));
        assert!(nfkd
            .normalize_iter(strings[3].chars())
            .eq(strings[4].chars()));
        assert!(nfkd
            .normalize_iter(strings[4].chars())
            .eq(strings[4].chars()));
    }
}

#[test]
fn test_hangul() {
    use icu_uniset::{CodePointSet, CodePointSetBuilder};
    use zerofrom::ZeroFrom;
    let builder = CodePointSetBuilder::new();
    let set: CodePointSet = builder.build();

    let data_provider = icu_testdata::get_provider();

    let normalizer: ComposingNormalizer = ComposingNormalizer::try_new_nfc(&data_provider).unwrap();
    {
        let mut norm_iter = normalizer.normalize_iter("A\u{AC00}\u{11A7}".chars());
        // Pessimize passthrough to avoid hiding bugs.
        norm_iter
            .decomposition
            .potential_passthrough_and_not_backward_combining = Some(ZeroFrom::zero_from(&set));
        assert!(norm_iter.eq("A\u{AC00}\u{11A7}".chars()));
    }
    {
        let mut norm_iter = normalizer.normalize_iter("A\u{AC00}\u{11C2}".chars());
        // Pessimize passthrough to avoid hiding bugs.
        norm_iter
            .decomposition
            .potential_passthrough_and_not_backward_combining = Some(ZeroFrom::zero_from(&set));
        assert!(norm_iter.eq("A\u{AC1B}".chars()));
    }
}

#[test]
fn test_canonical_composition() {
    let data_provider = icu_testdata::get_provider();
    let comp = CanonicalComposition::try_new(&data_provider).unwrap();

    assert_eq!(comp.compose('a', 'b'), None); // Just two starters

    assert_eq!(comp.compose('a', '\u{0308}'), Some('ä'));
    assert_eq!(comp.compose('A', '\u{0308}'), Some('Ä'));
    assert_eq!(comp.compose('ẹ', '\u{0302}'), Some('ệ'));
    assert_eq!(comp.compose('Ẹ', '\u{0302}'), Some('Ệ'));
    assert_eq!(comp.compose('\u{1D157}', '\u{1D165}'), None); // Composition exclusion

    assert_eq!(comp.compose('ে', 'া'), Some('ো')); // Second is starter; BMP
    assert_eq!(comp.compose('𑄱', '𑄧'), Some('𑄮')); // Second is starter; non-BMP

    assert_eq!(comp.compose('ᄀ', 'ᅡ'), Some('가')); // Hangul LV
    assert_eq!(comp.compose('가', 'ᆨ'), Some('각')); // Hangul LVT
}

#[test]
fn test_canonical_decomposition() {
    let data_provider = icu_testdata::get_provider();
    let decomp = CanonicalDecomposition::try_new(&data_provider).unwrap();

    assert_eq!(
        decomp.decompose('ä'),
        Decomposed::Expansion('a', '\u{0308}')
    );
    assert_eq!(
        decomp.decompose('Ä'),
        Decomposed::Expansion('A', '\u{0308}')
    );
    assert_eq!(
        decomp.decompose('ệ'),
        Decomposed::Expansion('ẹ', '\u{0302}')
    );
    assert_eq!(
        decomp.decompose('Ệ'),
        Decomposed::Expansion('Ẹ', '\u{0302}')
    );
    assert_eq!(
        decomp.decompose('\u{1D15E}'),
        Decomposed::Expansion('\u{1D157}', '\u{1D165}')
    );
    assert_eq!(decomp.decompose('ো'), Decomposed::Expansion('ে', 'া'));
    assert_eq!(decomp.decompose('𑄮'), Decomposed::Expansion('𑄱', '𑄧'));
    assert_eq!(decomp.decompose('가'), Decomposed::Expansion('ᄀ', 'ᅡ'));
    assert_eq!(decomp.decompose('각'), Decomposed::Expansion('가', 'ᆨ'));

    assert_eq!(decomp.decompose('\u{212B}'), Decomposed::Singleton('Å')); // ANGSTROM SIGN
    assert_eq!(decomp.decompose('\u{2126}'), Decomposed::Singleton('Ω')); // OHM SIGN

    assert_eq!(decomp.decompose('\u{1F71}'), Decomposed::Singleton('ά')); // oxia
    assert_eq!(
        decomp.decompose('\u{1F72}'),
        Decomposed::Expansion('ε', '\u{0300}')
    ); // not oxia but in the oxia range
    assert_eq!(
        decomp.decompose('ά'),
        Decomposed::Expansion('α', '\u{0301}')
    ); // tonos
}
