// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::LineBreakIterator;
use icu_segmenter::LineBreakIteratorUtf16;
use icu_segmenter::LineBreakRule;
use icu_segmenter::WordBreakRule;

fn strict(s: &str, ja_zh: bool, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = LineBreakIteratorUtf16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

fn normal(s: &str, ja_zh: bool, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = LineBreakIteratorUtf16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

fn loose(s: &str, ja_zh: bool, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = LineBreakIteratorUtf16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

fn anywhere(s: &str, ja_zh: bool, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Anywhere,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = LineBreakIteratorUtf16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Anywhere,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

#[test]
fn linebreak_strict() {
    // from css/css-text/line-break/line-break-*-011.xht
    strict("サ\u{3041}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-012.xht
    strict("サ\u{30FC}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-013.xht
    strict("サ\u{301C}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-014.xht
    strict("サ\u{3005}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-015a.xht
    // XXX ID x IN in UAX14. But why?
    strict("サ\u{2025}\u{2025}サ", false, vec![9, 12], vec![3, 4]);

    // from css/css-text/line-break/line-break-*-016a.xht
    strict("サ\u{30FB}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-017a.xht
    strict("サ\u{00B0}サ", false, vec![5, 8], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-018.xht
    //strict("サ\u{20AC}サ", false, vec![9], vec![3]);

    // from css/css-text/i18n/ja/css-text-line-break-ja-pr-strict.html
    // TODO: Why ID ÷ ID × PR × ID ÷ ID ?
    //strict("文文\u{00b1}字字", true, vec![3, 11, 14], vec![1, 4, 5]);
    //strict("文文\u{20AC}字字", true, vec![3, 11, 14], vec![1, 4, 5]);
    //strict("文文\u{FF04}字字", true, vec![3, 11, 14], vec![1, 4, 5]);
}

#[test]
fn linebreak_normal() {
    // from css/css-text/line-break/line-break-*-011.xht
    normal("サ\u{3041}サ", false, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-012.xht
    normal("サ\u{30FC}サ", false, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-013.xht
    normal("サ\u{301C}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-014.xht
    normal("サ\u{3005}サ", true, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-015.xht
    normal("サ\u{2025}\u{2025}サ", true, vec![9, 12], vec![3, 4]);

    // from css/css-text/line-break/line-break-*-016a.xht
    normal("サ\u{30FB}サ", true, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-017a.xht
    normal("サ\u{00B0}サ", true, vec![5, 8], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-018.xht
    normal("サ\u{20AC}サ", true, vec![3, 9], vec![1, 3]);

    // from css/css-text/i18n/unknown-lang/css-text-line-break-pr-normal.html
    // TODO: Why ID ÷ ID × PR × ID ÷ ID ?
    //normal("文文\u{00b1}字字", false, vec![3, 11, 14], vec![1, 4, 5]);
    //normal("文文\u{20AC}字字", false, vec![3, 11, 14], vec![1, 4, 5]);
    //normal("文文\u{2116}字字", false, vec![3, 11, 14], vec![1, 4, 5]);
}

#[test]
fn linebreak_loose() {
    // from css/css-text/line-break/line-break-*-011.xht
    loose("サ\u{3041}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-012.xht
    loose("サ\u{30FC}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-loose-013.xht
    loose("サ\u{301C}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-014.xht
    loose("サ\u{3005}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-015.xht
    loose(
        "サ\u{2025}\u{2025}サ",
        true,
        vec![3, 6, 9, 12],
        vec![1, 2, 3, 4],
    );

    // from css/css-text/line-break/line-break-*-016a.xht
    loose("サ\u{30FB}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-017a.xht
    loose("サ\u{00B0}サ", true, vec![3, 5, 8], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-018.xht
    loose("文\u{20AC}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{2116}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{ff04}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{ffe1}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{ffe5}文", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/i18n/ja/css-text-line-break-ja-pr-loose.html
    loose("文\u{00b1}文", true, vec![3, 5, 8], vec![1, 2, 3]);
    loose("文\u{20ac}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{ff04}文", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/i18n/unknown-lang/css-text-line-break-in-loose.html
    loose("文\u{2024}文", false, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{2025}文", false, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{2026}文", false, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{22ef}文", false, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{fe19}文", false, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/i18n/unknown-lang/css-text-line-break-pr-loose.html
    //loose("文\u{00b1}文", false, vec![8], vec![3]);
    //loose("文\u{20ac}文", false, vec![9], vec![3]);
    //loose("文\u{2116}文", false, vec![9], vec![3]);
    //loose("文\u{ff04}文", false, vec![9], vec![3]);

    // from css/css-text/i18n/zh/css-text-line-break-zh-in-loose.xht
    loose("文\u{2024}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{2025}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{2026}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{22ef}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{fe19}文", true, vec![3, 6, 9], vec![1, 2, 3]);

    // css/css-text/line-break/line-break-loose-hyphens-001.html
    loose("文\u{2010}文", true, vec![3, 6, 9], vec![1, 2, 3]);
    loose("文\u{2013}文", true, vec![3, 6, 9], vec![1, 2, 3]);

    // css/css-text/line-break/line-break-loose-hyphens-003.html
    loose("aa\u{2010}", false, vec![5], vec![3]);
    loose("aa\u{2013}", false, vec![5], vec![3]);
}

#[test]
fn linebreak_anywhere() {
    // css/css-text/line-break/line-break-anywhere-001.html
    anywhere(
        "aa-a.a)a,a) a\u{00A0}aa\u{2060}a\u{200D}a･a",
        true,
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 20, 21, 24, 25, 28, 29,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
        ],
    );

    // css/css-text/line-break/line-break-anywhere-002.html
    anywhere(
        "no hyphenation",
        false,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    );

    // css/css-text/line-break/line-break-anywhere-003.html
    anywhere("latin", false, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);

    // css/css-text/line-break/line-break-anywhere-004.html
    anywhere(
        "XX XXX",
        false,
        vec![1, 2, 3, 4, 5, 6],
        vec![1, 2, 3, 4, 5, 6],
    );

    // css/css-text/line-break/line-break-anywhere-005.html
    anywhere("X X", false, vec![1, 2, 3], vec![1, 2, 3]);

    // css/css-text/line-break/line-break-anywhere-006.html
    anywhere(
        "XXXX\u{00A0}XXXX",
        false,
        vec![1, 2, 3, 4, 6, 7, 8, 9, 10],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    );

    // css/css-text/line-break/line-break-anywhere-007.html
    anywhere(
        "X XX...",
        true,
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![1, 2, 3, 4, 5, 6, 7],
    );

    // css/css-text/line-break/line-break-anywhere-008.html
    anywhere(
        "X XX...",
        true,
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![1, 2, 3, 4, 5, 6, 7],
    );

    // css/css-text/line-break/line-break-anywhere-009.html
    anywhere("X\u{00A0}X", true, vec![1, 3, 4], vec![1, 2, 3]);

    // css/css-text/line-break/line-break-anywhere-010.html
    anywhere(
        "XXXX\u{00A0}XXXX",
        true,
        vec![1, 2, 3, 4, 6, 7, 8, 9, 10],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    );

    // css/css-text/line-break/line-break-anywhere-011.html
    anywhere("XX///", true, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);

    // css/css-text/line-break/line-break-anywhere-012.html
    anywhere(
        "X XX\\\\\\",
        true,
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![1, 2, 3, 4, 5, 6, 7],
    );

    // css/css-text/line-break/line-break-anywhere-013.html
    anywhere("XXX/X", true, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);

    // css/css-text/line-break/line-break-anywhere-014.html
    anywhere("XXX\\X", false, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);

    // css/css-text/line-break/line-break-anywhere-015.html
    anywhere("XXX\\X", false, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);

    // css/css-text/line-break/line-break-anywhere-016.html
    anywhere("XXX/X", false, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);

    // css/css-text/line-break/line-break-anywhere-017.html
    anywhere(
        "XXXX X",
        false,
        vec![1, 2, 3, 4, 5, 6],
        vec![1, 2, 3, 4, 5, 6],
    );

    // line-break-anywhere-overrides-uax-behavior-001.htm
    anywhere(
        "XX\u{2060}XX",
        false,
        vec![1, 2, 5, 6, 7],
        vec![1, 2, 3, 4, 5],
    );

    // line-break-anywhere-overrides-uax-behavior-004.htm
    anywhere(
        "..\u{200B}...X",
        false,
        vec![1, 2, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7],
    );
}
