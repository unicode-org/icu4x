// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::cmp::Ordering;

use atoi::FromRadix16;
use icu_collator::*;
use icu_locid::{langid, Locale};

type StackString = arraystring::ArrayString<arraystring::typenum::U32>;

/// Parse a string of space-separated hexadecimal code points (ending in end of input or semicolon)
fn parse_hex(mut hexes: &[u8]) -> Option<StackString> {
    let mut buf = StackString::new();
    loop {
        let (scalar, mut offset) = u32::from_radix_16(hexes);
        if let Some(c) = core::char::from_u32(scalar) {
            buf.try_push(c).unwrap();
        } else {
            return None;
        }
        if offset == hexes.len() {
            return Some(buf);
        }
        match hexes[offset] {
            // '\r' is for git on Windows touching the line ends
            b';' | b'\r' => {
                return Some(buf);
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
fn test_parse_hex() {
    assert_eq!(
        &parse_hex(b"1F926 1F3FC 200D 2642 FE0F").unwrap(),
        "\u{1F926}\u{1F3FC}\u{200D}\u{2642}\u{FE0F}"
    );
    assert_eq!(
        &parse_hex(b"1F926 1F3FC 200D 2642 FE0F; whatever").unwrap(),
        "\u{1F926}\u{1F3FC}\u{200D}\u{2642}\u{FE0F}"
    );
}

fn check_expectations(
    collator: &Collator,
    left: &[&str],
    right: &[&str],
    expectations: &[Ordering],
) {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut expect_iter = expectations.iter();
    while let (Some(left_str), Some(right_str), Some(expectation)) =
        (left_iter.next(), right_iter.next(), expect_iter.next())
    {
        assert_eq!(collator.compare(left_str, right_str), *expectation);
    }
}

#[test]
fn test_implicit_unihan() {
    // Adapted from `CollationTest::TestImplicits()` in collationtest.cpp of ICU4C.
    // The radical-stroke order of the characters tested agrees with their code point
    // order.

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);

    let collator = Collator::try_new(&Default::default(), options).unwrap();
    assert_eq!(collator.compare("\u{4E00}", "\u{4E00}"), Ordering::Equal);
    assert_eq!(collator.compare("\u{4E00}", "\u{4E01}"), Ordering::Less);
    assert_eq!(collator.compare("\u{4E01}", "\u{4E00}"), Ordering::Greater);

    assert_eq!(collator.compare("\u{4E18}", "\u{4E42}"), Ordering::Less);
    assert_eq!(collator.compare("\u{4E94}", "\u{50F6}"), Ordering::Less);
}

#[test]
fn test_currency() {
    // Adapted from `CollationCurrencyTest::currencyTest` in currcoll.cpp of ICU4C.
    // All the currency symbols, in collation order.
    let currencies = "\u{00A4}\u{00A2}\u{FFE0}\u{0024}\u{FF04}\u{FE69}\u{00A3}\u{FFE1}\u{00A5}\u{FFE5}\u{09F2}\u{09F3}\u{0E3F}\u{17DB}\u{20A0}\u{20A1}\u{20A2}\u{20A3}\u{20A4}\u{20A5}\u{20A6}\u{20A7}\u{20A9}\u{FFE6}\u{20AA}\u{20AB}\u{20AC}\u{20AD}\u{20AE}\u{20AF}";

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);

    let collator = Collator::try_new(&Default::default(), options).unwrap();
    // Iterating as chars and re-encoding due to
    // https://github.com/rust-lang/rust/issues/83871 being nightly-only. :-(
    let mut lower_buf = [0u8; 4];
    let mut higher_buf = [0u8; 4];
    let mut chars = currencies.chars();
    while let Some(lower) = chars.next() {
        let tail = chars.clone();
        for higher in tail {
            let lower_str = lower.encode_utf8(&mut lower_buf);
            let higher_str = higher.encode_utf8(&mut higher_buf);
            assert_eq!(collator.compare(lower_str, higher_str), Ordering::Less);
        }
    }
}

#[test]
fn test_de() {
    // Adapted from `CollationGermanTest` in decoll.cpp of ICU4C.
    let left = [
        "Größe", "abc", "Töne", "Töne", "Töne", "äbc", "äbc", "äbc", "Straße", "efg", "äbc",
        "Straße",
    ];

    let right = [
        "Grossist",
        "a\u{0308}bc",
        "Ton",
        "Tod",
        "Tofu",
        "A\u{0308}bc",
        "a\u{0308}bc",
        "aebc",
        "Strasse",
        "efg",
        "aebc",
        "Strasse",
    ];

    let expect_primary = [
        Ordering::Less,
        Ordering::Equal,
        Ordering::Greater,
        Ordering::Greater,
        Ordering::Greater,
        Ordering::Equal,
        Ordering::Equal,
        Ordering::Less,
        Ordering::Equal,
        Ordering::Equal,
        Ordering::Less,
        Ordering::Equal,
    ];

    let expect_tertiary = [
        Ordering::Less,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Greater,
        Ordering::Greater,
        Ordering::Less,
        Ordering::Equal,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Equal,
        Ordering::Less,
        Ordering::Greater,
    ];

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Primary);

    {
        // Note: German uses the root collation
        let collator = Collator::try_new(&Default::default(), options).unwrap();
        check_expectations(&collator, &left, &right, &expect_primary);
    }

    options.strength = Some(Strength::Tertiary);

    {
        // Note: German uses the root collation
        let collator = Collator::try_new(&Default::default(), options).unwrap();
        check_expectations(&collator, &left, &right, &expect_tertiary);
    }
}

#[test]
fn test_en() {
    // Adapted from encoll.cpp in ICU4C
    let left = [
        "ab",
        "black-bird",
        "black bird",
        "black-bird",
        "Hello",
        "ABC",
        "abc",
        "blackbird",
        "black-bird",
        "black-bird",
        "pêche",
        "péché",
        "ÄB̈C̈",
        "äbc",
        "pécher",
        "roles",
        "abc",
        "A",
        "A",
        "ab",
        "tcompareplain",
        "ab",
        "a#b",
        "a#b",
        "abc",
        "Abcda",
        "abcda",
        "abcda",
        "æbcda",
        "äbcda",
        "abc",
        "abc",
        "abc",
        "abc",
        "abc",
        "acHc",
        "äbc",
        "thîs",
        "pêche",
        "abc",
        "abc",
        "abc",
        "aæc",
        "abc",
        "abc",
        "aæc",
        "abc",
        "abc",
        "péché",
    ]; // 49

    let right = [
        "abc",
        "blackbird",
        "black-bird",
        "black",
        "hello",
        "ABC",
        "ABC",
        "blackbirds",
        "blackbirds",
        "blackbird",
        "péché",
        "pécher",
        "ÄB̈C̈",
        "Äbc",
        "péche",
        "rôle",
        "Aácd",
        "Aácd",
        "abc",
        "abc",
        "TComparePlain",
        "aBc",
        "a#B",
        "a&b",
        "a#c",
        "abcda",
        "Äbcda",
        "äbcda",
        "Äbcda",
        "Äbcda",
        "ab#c",
        "abc",
        "ab=c",
        "abd",
        "äbc",
        "aCHc",
        "äbc",
        "thîs",
        "péché",
        "aBC",
        "abd",
        "äbc",
        "aÆc",
        "aBd",
        "äbc",
        "aÆc",
        "aBd",
        "äbc",
        "pêche",
    ]; // 49

    let expectations = [
        Ordering::Less,
        Ordering::Less, /* Ordering::Greater, */
        Ordering::Less,
        Ordering::Greater,
        Ordering::Greater,
        Ordering::Equal,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less, /* Ordering::Greater, */
        /* 10 */
        Ordering::Greater,
        Ordering::Less,
        Ordering::Equal,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Greater,
        Ordering::Greater,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less, /* 20 */
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Greater,
        Ordering::Greater,
        /* Test Tertiary  > 26 */
        Ordering::Less,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Less, /* 30 */
        Ordering::Greater,
        Ordering::Equal,
        Ordering::Greater,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        /* test identical > 36 */
        Ordering::Equal,
        Ordering::Equal,
        /* test primary > 38 */
        Ordering::Equal,
        Ordering::Equal, /* 40 */
        Ordering::Less,
        Ordering::Equal,
        Ordering::Equal,
        /* test secondary > 43 */
        Ordering::Less,
        Ordering::Less,
        Ordering::Equal,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less, // 49
    ];

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Tertiary);

    {
        // Note: English uses the root collation
        let collator = Collator::try_new(&Default::default(), options).unwrap();
        check_expectations(&collator, &left[..38], &right[..38], &expectations[..38]);
    }

    options.strength = Some(Strength::Primary);

    {
        // Note: English uses the root collation
        let collator = Collator::try_new(&Default::default(), options).unwrap();
        check_expectations(
            &collator,
            &left[38..43],
            &right[38..43],
            &expectations[38..43],
        );
    }

    options.strength = Some(Strength::Secondary);

    {
        // Note: English uses the root collation
        let collator = Collator::try_new(&Default::default(), options).unwrap();
        check_expectations(&collator, &left[43..], &right[43..], &expectations[43..]);
    }
}

#[test]
fn test_en_bugs() {
    // Adapted from encoll.cpp in ICU4C
    let bugs = ["a", "A", "e", "E", "é", "è", "ê", "ë", "ea", "x"];
    //        let locale: Locale = langid!("en").into();
    let locale: Locale = Locale::default(); // English uses the root collation

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Tertiary);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        let mut outer = bugs.iter();
        while let Some(left) = outer.next() {
            let inner = outer.clone();
            for right in inner {
                assert_eq!(collator.compare(left, right), Ordering::Less);
            }
        }
    }
}

#[test]
fn test_ja_tertiary() {
    // Adapted from `CollationKanaTest::TestTertiary` in jacoll.cpp in ICU4C
    let left = [
        "ﾞ", // half-width
        "あ",
        "ア",
        "ああ",
        "アー",
        "アート",
    ];
    let right = [
        "ﾟ", // half-width
        "ア",
        "ああ",
        "アー",
        "アート",
        "ああと",
    ];
    let expectations = [
        Ordering::Less,
        Ordering::Equal, // Katakanas and Hiraganas are equal on tertiary level
        Ordering::Less,
        Ordering::Greater, // Prolonged sound mark sorts BEFORE equivalent vowel
        Ordering::Less,
        Ordering::Less, // Prolonged sound mark sorts BEFORE equivalent vowel
    ];
    let locale: Locale = langid!("ja").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Tertiary);
    options.case_level = Some(CaseLevel::On);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[test]
fn test_ja_base() {
    // Adapted from `CollationKanaTest::TestBase` in jacoll.cpp of ICU4C.
    let cases = ["カ", "カキ", "キ", "キキ"];

    let locale: Locale = langid!("ja").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Primary);

    let collator = Collator::try_new(&locale.into(), options).unwrap();
    let mut case_iter = cases.iter();
    while let Some(lower) = case_iter.next() {
        let tail = case_iter.clone();
        for higher in tail {
            assert_eq!(collator.compare(lower, higher), Ordering::Less);
        }
    }
}

#[test]
fn test_ja_plain_dakuten_handakuten() {
    // Adapted from `CollationKanaTest::TestPlainDakutenHandakuten` in jacoll.cpp of ICU4C.
    let cases = ["ハカ", "バカ", "ハキ", "バキ"];

    let locale: Locale = langid!("ja").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Secondary);

    let collator = Collator::try_new(&locale.into(), options).unwrap();
    let mut case_iter = cases.iter();
    while let Some(lower) = case_iter.next() {
        let tail = case_iter.clone();
        for higher in tail {
            assert_eq!(collator.compare(lower, higher), Ordering::Less);
        }
    }
}

#[test]
fn test_ja_small_large() {
    // Adapted from `CollationKanaTest::TestSmallLarge` in jacoll.cpp of ICU4C.
    let cases = ["ッハ", "ツハ", "ッバ", "ツバ"];

    let locale: Locale = langid!("ja").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Tertiary);
    options.case_level = Some(CaseLevel::On);

    let collator = Collator::try_new(&locale.into(), options).unwrap();
    let mut case_iter = cases.iter();
    while let Some(lower) = case_iter.next() {
        let tail = case_iter.clone();
        for higher in tail {
            assert_eq!(collator.compare(lower, higher), Ordering::Less);
        }
    }
}

#[test]
fn test_ja_hiragana_katakana() {
    // Adapted from `CollationKanaTest::TestKatakanaHiragana` in jacoll.cpp of ICU4C.
    let cases = ["あッ", "アッ", "あツ", "アツ"];

    let locale: Locale = langid!("ja").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);
    options.case_level = Some(CaseLevel::On);

    let collator = Collator::try_new(&locale.into(), options).unwrap();
    let mut case_iter = cases.iter();
    while let Some(lower) = case_iter.next() {
        let tail = case_iter.clone();
        for higher in tail {
            assert_eq!(collator.compare(lower, higher), Ordering::Less);
        }
    }
}

#[test]
fn test_ja_hiragana_katakana_utf16() {
    // Adapted from `CollationKanaTest::TestKatakanaHiragana` in jacoll.cpp of ICU4C.
    let cases = [
        &[0x3042u16, 0x30C3u16],
        &[0x30A2u16, 0x30C3u16],
        &[0x3042u16, 0x30C4u16],
        &[0x30A2u16, 0x30C4u16],
    ];

    let locale: Locale = langid!("ja").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);
    options.case_level = Some(CaseLevel::On);

    let collator = Collator::try_new(&locale.into(), options).unwrap();
    let mut case_iter = cases.iter();
    while let Some(lower) = case_iter.next() {
        let tail = case_iter.clone();
        for higher in tail {
            assert_eq!(
                collator.compare_utf16(&lower[..], &higher[..]),
                Ordering::Less
            );
        }
    }
}

#[test]
fn test_ja_chooon_kigoo() {
    // Adapted from `CollationKanaTest::TestChooonKigoo` in jacoll.cpp of ICU4C.
    let cases = [
        "カーあ",
        "カーア",
        "カイあ",
        "カイア",
        "キーあ", /* Prolonged sound mark sorts BEFORE equivalent vowel (ICU 2.0) */
        "キーア", /* Prolonged sound mark sorts BEFORE equivalent vowel (ICU 2.0) */
        "キイあ",
        "キイア",
    ];

    let locale: Locale = langid!("ja").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);
    options.case_level = Some(CaseLevel::On);

    let collator = Collator::try_new(&locale.into(), options).unwrap();
    let mut case_iter = cases.iter();
    while let Some(lower) = case_iter.next() {
        let tail = case_iter.clone();
        for higher in tail {
            assert_eq!(collator.compare(lower, higher), Ordering::Less);
        }
    }
}

// TODO: This test should eventually test fallback
// TODO: Test Swedish and Chinese also, since they have unusual
// variant defaults. (But are currently not part of the test data.)
#[ignore]
#[test]
fn test_region_fallback() {
    // There's no explicit fi-FI data.
    let locale: Locale = "fi-u-co-standard".parse().unwrap();

    // let locale: Locale = langid!("fi-FI").into();

    let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
    assert_eq!(collator.compare("ä", "z"), Ordering::Greater);
}

#[test]
fn test_reordering() {
    let locale: Locale = langid!("bn").into();

    // অ is Bangla
    // ऄ is Devanagari

    {
        let collator: Collator =
            Collator::try_new(&Default::default(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("অ", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ऄ", "a"), Ordering::Greater);
        assert_eq!(collator.compare("অ", "ऄ"), Ordering::Greater);
    }

    {
        let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("অ", "a"), Ordering::Less);
        assert_eq!(collator.compare("ऄ", "a"), Ordering::Less);
        assert_eq!(collator.compare("অ", "ऄ"), Ordering::Less);
    }
}

#[ignore]
#[test]
fn test_zh() {
    // Note: ㄅ is Bopomofo.

    {
        let collator: Collator =
            Collator::try_new(&Default::default(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Greater);
        assert_eq!(collator.compare("佰", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Greater);
        assert_eq!(collator.compare("艾", "佰"), Ordering::Greater);
        assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Greater);
        assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Greater);
        assert_eq!(collator.compare("不", "把"), Ordering::Less);
    }
    {
        let locale: Locale = langid!("zh").into(); // Defaults to -u-co-pinyin
        let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Less);
        assert_eq!(collator.compare("佰", "a"), Ordering::Less);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Greater);
        assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
        assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("不", "把"), Ordering::Greater);
    }
    {
        let locale: Locale = "zh-u-co-pinyin".parse().unwrap();
        let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Less);
        assert_eq!(collator.compare("佰", "a"), Ordering::Less);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Greater);
        assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
        assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("不", "把"), Ordering::Greater);
    }
    {
        let locale: Locale = "zh-u-co-gb2312".parse().unwrap();
        let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Greater);
        assert_eq!(collator.compare("佰", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Greater);
        assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
        // In GB2312 proper, Bopomofo comes before Han, but the
        // collation leaves Bopomofo unreordered, so it comes after.
        assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("不", "把"), Ordering::Greater);
    }
    {
        let locale: Locale = "zh-u-co-stroke".parse().unwrap();
        let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Less);
        assert_eq!(collator.compare("佰", "a"), Ordering::Less);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Less);
        assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Less);
        assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
        assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("不", "把"), Ordering::Less);
    }
    {
        let locale: Locale = "zh-u-co-zhuyin".parse().unwrap();
        let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Less);
        assert_eq!(collator.compare("佰", "a"), Ordering::Less);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Less);
        assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Less);
        assert_eq!(collator.compare("艾", "佰"), Ordering::Greater);
        assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("不", "把"), Ordering::Greater);
    }
    {
        let locale: Locale = "zh-u-co-unihan".parse().unwrap();
        let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Less);
        assert_eq!(collator.compare("佰", "a"), Ordering::Less);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Less);
        assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Less);
        assert_eq!(collator.compare("艾", "佰"), Ordering::Greater);
        assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("不", "把"), Ordering::Less);
    }
    {
        let locale: Locale = "zh-u-co-big5han".parse().unwrap();
        let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Greater);
        assert_eq!(collator.compare("佰", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Less);
        assert_eq!(collator.compare("艾", "佰"), Ordering::Less);
        assert_eq!(collator.compare("艾", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("佰", "ㄅ"), Ordering::Less);
        assert_eq!(collator.compare("不", "把"), Ordering::Less);
    }
    // TODO: Test script and region aliases
}

// TODO: frcoll requires support for fr-CA

// TODO: Write a test for Bangla

#[test]
fn test_es_tertiary() {
    // Adapted from `CollationSpanishTest::TestTertiary` in escoll.cpp in ICU4C
    let left = ["alias", "Elliot", "Hello", "acHc", "acc"];
    let right = ["allias", "Emiot", "hellO", "aCHc", "aCHc"];
    let expectations = [
        Ordering::Less,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Less,
        Ordering::Less,
    ];
    let locale: Locale = langid!("es").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Tertiary);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[test]
fn test_es_primary() {
    // Adapted from `CollationSpanishTest::TestPrimary` in escoll.cpp in ICU4C
    let left = ["alias", "acHc", "acc", "Hello"];
    let right = ["allias", "aCHc", "aCHc", "hellO"];
    let expectations = [
        Ordering::Less,
        Ordering::Equal,
        Ordering::Less,
        Ordering::Equal,
    ];
    let locale: Locale = langid!("es").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Primary);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[test]
fn test_el_secondary() {
    // Adapted from `CollationRegressionTest::Test4095316` in regcoll.cpp of ICU4C.
    let locale: Locale = Locale::default(); // Greek uses the root collation

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Secondary);

    let collator = Collator::try_new(&locale.into(), options).unwrap();
    assert_eq!(collator.compare("ϔ", "Ϋ"), Ordering::Equal);
}

#[test]
fn test_th_dictionary() {
    // Adapted from `CollationThaiTest::TestDictionary` of thcoll.cpp in ICU4C.
    let dict = include_str!("data/riwords.txt")
        .strip_prefix('\u{FEFF}')
        .unwrap();
    let locale: Locale = langid!("th").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);

    let collator = Collator::try_new(&locale.into(), options).unwrap();
    let mut lines = dict.lines();
    let mut prev = loop {
        if let Some(line) = lines.next() {
            if line.starts_with('#') {
                continue;
            }
            break line;
        } else {
            panic!("Malformed dictionary");
        }
    };

    for line in lines {
        assert_eq!(collator.compare(prev, line), Ordering::Less);
        prev = line;
    }
}

#[test]
fn test_th_corner_cases() {
    // Adapted from `CollationThaiTest::TestCornerCases` in thcoll.cpp in ICU4C
    let left = [
        // Shorter words precede longer
        "\u{0E01}",
        // Tone marks are considered after letters (i.e. are primary ignorable)
        "\u{0E01}\u{0E32}",
        // ditto for other over-marks
        "\u{0E01}\u{0E32}",
        // commonly used mark-in-context order.
        // In effect, marks are sorted after each syllable.
        "\u{0E01}\u{0E32}\u{0E01}\u{0E49}\u{0E32}",
        // Hyphens and other punctuation follow whitespace but come before letters
        //            "\u{0E01}\u{0E32}",
        "\u{0E01}\u{0E32}-",
        // Doubler follows an identical word without the doubler
        // "\u{0E01}\u{0E32}",
        "\u{0E01}\u{0E32}\u{0E46}",
        // \u{0E45} after either \u{0E24} or \{u0E26} is treated as a single
        // combining character, similar to "c < ch" in traditional spanish.
        "\u{0E24}\u{0E29}\u{0E35}",
        "\u{0E26}\u{0E29}\u{0E35}",
        // Vowels reorder, should compare \u{0E2D} and \u{0E34}
        // (Should the middle code point differ between the two strings?)
        "\u{0E40}\u{0E01}\u{0E2D}",
        // Tones are compared after the rest of the word (e.g. primary ignorable)
        "\u{0E01}\u{0E32}\u{0E01}\u{0E48}\u{0E32}",
        // Periods are ignored entirely
        "\u{0E01}.\u{0E01}.",
    ];
    let right = [
        "\u{0E01}\u{0E01}",
        "\u{0E01}\u{0E49}\u{0E32}",
        "\u{0E01}\u{0E32}\u{0E4C}",
        "\u{0E01}\u{0E48}\u{0E32}\u{0E01}\u{0E49}\u{0E32}",
        //            "\u{0E01}\u{0E32}-",
        "\u{0E01}\u{0E32}\u{0E01}\u{0E32}",
        // "\u{0E01}\u{0E32}\u{0E46}",
        "\u{0E01}\u{0E32}\u{0E01}\u{0E32}",
        "\u{0E24}\u{0E45}\u{0E29}\u{0E35}",
        "\u{0E26}\u{0E45}\u{0E29}\u{0E35}",
        "\u{0E40}\u{0E01}\u{0E34}",
        "\u{0E01}\u{0E49}\u{0E32}\u{0E01}\u{0E32}",
        "\u{0E01}\u{0E32}",
    ];
    let expectations = [
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        //            Ordering::Equal,
        Ordering::Less,
        // Ordering::Equal,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
    ];
    let locale: Locale = langid!("th").into();
    {
        // TODO(#2013): Check why the commented-out cases fail
        let collator = Collator::try_new(&locale.into(), CollatorOptions::new()).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[test]
fn test_th_reordering() {
    // Adapted from `CollationThaiTest::TestReordering` in thcoll.cpp in ICU4C
    let left = [
        // composition
        "\u{0E41}c\u{0301}",
        // supplementaries
        // TODO(#2013): Why does this fail?
        // "\u{0E41}\u{1D7CE}",
        // supplementary composition decomps to supplementary
        "\u{0E41}\u{1D15F}",
        // supplementary composition decomps to BMP
        "\u{0E41}\u{2F802}", /* omit bacward iteration tests
                              * contraction bug
                              * "\u{0E24}\u{0E41}",
                              * TODO: Support contracting starters, then add more here */
    ];
    let right = [
        "\u{0E41}\u{0107}",
        // "\u{0E41}\u{1D7CF}",
        "\u{0E41}\u{1D158}\u{1D165}",
        "\u{0E41}\u{4E41}", // "\u{0E41}\u{0E24}",
    ];
    let expectations = [
        Ordering::Equal,
        // Ordering::Less,
        Ordering::Equal,
        Ordering::Equal,
        // Ordering::Equal,
    ];
    let locale: Locale = langid!("th").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Secondary);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[test]
fn test_tr_tertiary() {
    // Adapted from `CollationTurkishTest::TestTertiary` in trcoll.cpp in ICU4C
    let left = ["ş", "vät", "old", "üoid", "hĞalt", "stresŞ", "voıd", "idea"];
    let right = ["ü", "vbt", "Öay", "void", "halt", "ŞtreŞs", "void", "Idea"];
    let expectations = [
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Less,
        Ordering::Less,
        Ordering::Greater,
    ];
    let locale: Locale = langid!("tr").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Tertiary);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[test]
fn test_tr_primary() {
    // Adapted from `CollationTurkishTest::TestPrimary` in trcoll.cpp in ICU4C
    let left = ["üoid", "voıd", "idea"];
    let right = ["void", "void", "Idea"];
    let expectations = [Ordering::Less, Ordering::Less, Ordering::Greater];
    let locale: Locale = langid!("tr").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Tertiary);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[ignore]
#[test]
fn test_lt_tertiary() {
    let left = [
        "a\u{0307}\u{0300}a",
        "a\u{0307}\u{0301}a",
        "a\u{0307}\u{0302}a",
        "a\u{0307}\u{0303}a",
        "ž",
    ];
    let right = [
        "a\u{0300}a",
        "a\u{0301}a",
        "a\u{0302}a",
        "a\u{0303}a",
        "z\u{033F}",
    ];
    let expectations = [
        Ordering::Equal,
        Ordering::Equal,
        Ordering::Greater,
        Ordering::Equal,
        Ordering::Greater,
    ];
    let locale: Locale = langid!("lt").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Tertiary);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[ignore]
#[test]
fn test_lt_primary() {
    let left = ["ž"];
    let right = ["z"];
    let expectations = [Ordering::Greater];
    let locale: Locale = langid!("lt").into();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Primary);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[test]
fn test_basics() {
    // Adapted from `CollationAPITest::TestProperty` in apicoll.cpp in ICU4C
    let left = ["ab", "ab", "blackbird", "black bird", "Hello", "", "abä"];
    let right = ["abc", "AB", "black-bird", "black-bird", "hello", "", "abß"];
    let expectations = [
        Ordering::Less,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Equal,
    ];

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Tertiary);

    {
        let collator = Collator::try_new(&Default::default(), options).unwrap();
        check_expectations(&collator, &left, &right, &expectations);
    }
}

#[test]
fn test_numeric_long() {
    let mut options = CollatorOptions::new();
    options.numeric = Some(Numeric::On);

    let collator = Collator::try_new(&Default::default(), options).unwrap();
    let mut left = String::new();
    let mut right = String::new();
    // We'll make left larger than right numerically. However, first, let's use
    // a leading zero to make left look less than right non-numerically.
    left.push('0');
    left.push('1');
    right.push('1');
    // Now, let's make sure we end up with segments longer than
    // 254 digits
    for _ in 0..256 {
        left.push('2');
        right.push('2');
    }
    // Now the difference:
    left.push('4');
    right.push('3');
    // Again making segments long
    for _ in 0..256 {
        left.push('5');
        right.push('5');
    }
    // And some trailing zeros
    for _ in 0..7 {
        left.push('0');
        right.push('0');
    }
    assert_eq!(collator.compare(&left, &right), Ordering::Greater);
}

#[test]
fn test_numeric_after() {
    let mut options = CollatorOptions::new();
    options.numeric = Some(Numeric::On);

    let collator = Collator::try_new(&Default::default(), options).unwrap();
    assert_eq!(collator.compare("0001000b", "1000a"), Ordering::Greater);
}

#[test]
fn test_unpaired_surrogates() {
    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);

    let collator = Collator::try_new(&Default::default(), options).unwrap();
    assert_eq!(
        collator.compare_utf16(&[0xD801u16], &[0xD802u16]),
        Ordering::Equal
    );
}

#[test]
fn test_backward_second_level() {
    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Secondary);

    {
        let collator = Collator::try_new(&Default::default(), options).unwrap();

        let cases = ["cote", "coté", "côte", "côté"];
        let mut case_iter = cases.iter();
        while let Some(lower) = case_iter.next() {
            let tail = case_iter.clone();
            for higher in tail {
                assert_eq!(collator.compare(lower, higher), Ordering::Less);
            }
        }
    }

    options.backward_second_level = Some(BackwardSecondLevel::On);

    {
        let collator = Collator::try_new(&Default::default(), options).unwrap();

        {
            let cases = ["cote", "côte", "coté", "côté"];
            let mut case_iter = cases.iter();
            while let Some(lower) = case_iter.next() {
                let tail = case_iter.clone();
                for higher in tail {
                    assert_eq!(collator.compare(lower, higher), Ordering::Less);
                }
            }
        }
        {
            let cases = ["cote\u{FFFE}coté", "côte\u{FFFE}cote"];
            let mut case_iter = cases.iter();
            while let Some(lower) = case_iter.next() {
                let tail = case_iter.clone();
                for higher in tail {
                    assert_eq!(collator.compare(lower, higher), Ordering::Less);
                }
            }
        }
    }
}

#[test]
fn test_cantillation() {
    let locale: Locale = Locale::default();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);

    {
        let collator = Collator::try_new(&(&locale).into(), options).unwrap();
        assert_eq!(
            collator.compare(
                "\u{05D3}\u{05D7}\u{05D9}\u{05AD}",
                "\u{05D3}\u{05D7}\u{05D9}"
            ),
            Ordering::Equal
        );
    }

    options.strength = Some(Strength::Identical);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        assert_eq!(
            collator.compare(
                "\u{05D3}\u{05D7}\u{05D9}\u{05AD}",
                "\u{05D3}\u{05D7}\u{05D9}"
            ),
            Ordering::Greater
        );
    }
}

#[test]
fn test_cantillation_utf8() {
    let locale: Locale = Locale::default();

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);

    {
        let collator = Collator::try_new(&(&locale).into(), options).unwrap();
        assert_eq!(
            collator.compare_utf8(
                "\u{05D3}\u{05D7}\u{05D9}\u{05AD}".as_bytes(),
                "\u{05D3}\u{05D7}\u{05D9}".as_bytes()
            ),
            Ordering::Equal
        );
    }

    options.strength = Some(Strength::Identical);

    {
        let collator = Collator::try_new(&locale.into(), options).unwrap();
        assert_eq!(
            collator.compare(
                "\u{05D3}\u{05D7}\u{05D9}\u{05AD}",
                "\u{05D3}\u{05D7}\u{05D9}"
            ),
            Ordering::Greater
        );
    }
}

#[test]
fn test_conformance_shifted() {
    // Adapted from `UCAConformanceTest::TestTableShifted` of ucaconf.cpp in ICU4C.
    let bugs = [];
    let dict = include_bytes!("data/CollationTest_CLDR_SHIFTED.txt");

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);
    options.alternate_handling = Some(AlternateHandling::Shifted);

    let collator = Collator::try_new(&Default::default(), options).unwrap();
    let mut lines = dict.split(|b| b == &b'\n');
    let mut prev = loop {
        if let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }
            if line.starts_with(&[b'#']) {
                continue;
            }
            if let Some(parsed) = parse_hex(line) {
                break parsed;
            }
        } else {
            panic!("Malformed dictionary");
        }
    };

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some(parsed) = parse_hex(line) {
            if !bugs.contains(&parsed.as_str())
                && collator.compare(&prev, &parsed) == Ordering::Greater
            {
                assert_eq!(&prev[..], &parsed[..]);
            }
            prev = parsed;
        }
    }
}

#[test]
fn test_conformance_non_ignorable() {
    // Adapted from `UCAConformanceTest::TestTableNonIgnorable` of ucaconf.cpp in ICU4C.
    let bugs = [];
    let dict = include_bytes!("data/CollationTest_CLDR_NON_IGNORABLE.txt");

    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Quaternary);
    options.alternate_handling = Some(AlternateHandling::NonIgnorable);

    let collator = Collator::try_new(&Default::default(), options).unwrap();
    let mut lines = dict.split(|b| b == &b'\n');
    let mut prev = loop {
        if let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }
            if line.starts_with(&[b'#']) {
                continue;
            }
            if let Some(parsed) = parse_hex(line) {
                break parsed;
            }
        } else {
            panic!("Malformed dictionary");
        }
    };

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some(parsed) = parse_hex(line) {
            if !bugs.contains(&parsed.as_str())
                && collator.compare(&prev, &parsed) == Ordering::Greater
            {
                assert_eq!(&prev[..], &parsed[..]);
            }
            prev = parsed;
        }
    }
}

#[test]
fn test_case_level() {
    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Primary);
    options.case_level = Some(CaseLevel::On);
    let collator_with_case = Collator::try_new(&Default::default(), options).unwrap();
    assert_eq!(
        collator_with_case.compare("aA", "Aa"),
        core::cmp::Ordering::Less
    );
}

// TODO: Test languages that map to the root.
// The languages that map to root without script reordering are:
// ca (at least for now)
// de
// en
// fr
// ga
// id
// it
// lb
// ms
// nl
// pt
// sw
// xh
// zu
//
// A bit unclear: ff in Latin script?
//
// These are root plus script reordering:
// am
// be
// bg
// chr
// el
// ka
// lo
// mn
// ne
// ru

// TODO: Test imports. These aren't aliases but should get deduplicated
// in the provider:
// bo: dz-u-co-standard (draft: unconfirmed)
// bs: hr
// bs-Cyrl: sr
// fa-AF: ps
// sr-Latn: hr

// TODO: Test that nn and nb are aliases for no

// TODO: Consider testing ff-Adlm for supplementary-plane tailoring, including contractions

// TODO: Test Tibetan

// TODO: Test de-AT-u-co-phonebk vs de-DE-u-co-phonebk

// TODO: Test da defaulting to [caseFirst upper]
// TODO: Test fr-CA defaulting to backward second level
