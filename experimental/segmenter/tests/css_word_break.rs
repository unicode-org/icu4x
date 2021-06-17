// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::LineBreakIterator;
use icu_segmenter::LineBreakIteratorUtf16;
use icu_segmenter::LineBreakRule;
use icu_segmenter::WordBreakRule;

fn break_all(s: &str, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Strict,
        WordBreakRule::BreakAll,
        false,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = LineBreakIteratorUtf16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Strict,
        WordBreakRule::BreakAll,
        false,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

fn keep_all(s: &str, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Strict,
        WordBreakRule::KeepAll,
        false,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = LineBreakIteratorUtf16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Strict,
        WordBreakRule::KeepAll,
        false,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

fn normal(s: &str, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = LineBreakIteratorUtf16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

#[test]
fn wordbreak_breakall() {
    // from css/css-text/word-break/word-break-break-all-000.html
    let s = "\u{65e5}\u{672c}\u{8a9e}";
    break_all(s, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/word-break/word-break-break-all-001.html
    let s = "latin";
    break_all(s, vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);

    // from css/css-text/word-break/word-break-break-all-002.html
    let s = "\u{d55c}\u{ae00}\u{c77e}";
    break_all(s, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/word-break/word-break-break-all-003.html
    let s = "ภาษาไทยภาษาไทย";
    break_all(
        s,
        vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
    );

    // from css/css-text/word-break/word-break-break-all-004.html
    let s = "التدويل نشاط التدويل";
    break_all(
        s,
        vec![
            2, 4, 6, 8, 10, 12, 15, 17, 19, 21, 24, 26, 28, 30, 32, 34, 36, 38,
        ],
        vec![
            1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 20,
        ],
    );

    // from css/css-text/word-break/word-break-break-all-008.html
    let s = "हिन्दी हिन्दी हिन्दी";
    break_all(
        s,
        vec![6, 12, 19, 25, 31, 38, 44, 50, 56],
        vec![2, 4, 7, 9, 11, 14, 16, 18, 20],
    );

    // from css/css-text/word-break/word-break-break-all-014.html
    let s = "\u{1f496}\u{1f494}";
    break_all(s, vec![4, 8], vec![2, 4]);

    // from css/css-text/word-break/word-break-break-all-018.html
    //let s = "XXXX\u{00a0}X";
    //break_all(s, vec![1, 2, 3, 5, 6], vec![1, 2, 3, 5, 6]);

    // from css/css-text/word-break/word-break-break-all-022.html
    //let s = "XX\u{00a0}X";
    //break_all(s, vec![1, 2, 4, 5], vec![1, 2, 3, 4]);

    // from css/css-text/word-break/word-break-break-all-023.html
    let s = "XX XX\u{005C}\u{005C}\u{005C}";
    break_all(s, vec![1, 3, 4, 5, 6, 7, 8], vec![1, 3, 4, 5, 6, 7, 8]);

    // from css/css-text/word-break/word-break-break-all-026.html
    let s = "XX XXX///";
    break_all(s, vec![1, 3, 4, 5, 9], vec![1, 3, 4, 5, 9]);

    // css/css-text/word-break/word-break-break-all-inline-008.html
    let s = "X.";
    break_all(s, vec![2], vec![2]);

    // ID and CJ
    let s = "フォ";
    break_all(s, vec![3, 6], vec![1, 2]);
}

#[test]
fn wordbreak_keepall() {
    // from css/css-text/word-break/word-break-keep-all-000.html
    let s = "latin";
    keep_all(s, vec![5], vec![5]);

    // from css/css-text/word-break/word-break-keep-all-001.html
    let s = "\u{65e5}\u{672c}\u{8a9e}";
    keep_all(s, vec![9], vec![3]);

    // from css/css-text/word-break/word-break-keep-all-002.html
    let s = "한글이";
    keep_all(s, vec![9], vec![3]);

    // from css/css-text/word-break/word-break-keep-all-003.html
    let s = "และและ";
    keep_all(s, vec![9, 18], vec![3, 6]);

    // from css/css-text/word-break/word-break-keep-all-005.html
    let s = "字\u{3000}字";
    keep_all(s, vec![6, 9], vec![2, 3]);

    // from css/css-text/word-break/word-break-keep-all-006.html
    let s = "字\u{3001}字";
    keep_all(s, vec![6, 9], vec![2, 3]);

    // from css/css-text/word-boundary/word-boundary-107.html
    let s = "しょう。";
    keep_all(s, vec![12], vec![4]);

    // failed test. JL, JV and JT
    let s = "\u{110B}\u{1162}\u{1100}\u{1175}\u{1111}\u{1161}\u{11AB}\u{1103}\u{1161}";
    keep_all(s, vec![27], vec![9]);
}

#[test]
fn wordbreak_normal() {
    {
        let s = "\u{0e20}\u{0e32}\u{0e29}\u{0e32}\u{0e44}\u{0e17}\u{0e22}\u{0e20}\u{0e32}\u{0e29}\u{0e32}\u{0e44}\u{0e17}\u{0e22}";
        normal(s, vec![12, 21, 33, 42], vec![4, 7, 11, 14]);
    }
}

#[test]
#[ignore = "We don't have km breaking rule yet"]
fn wordbreak_normal_km() {
    // from css/css-text/word-break/word-break-normal-km-000.html
    let _s = "ភាសាខ្មែរភាសាខ្មែរភាសាខ្មែរ";
    normal(_s, vec![27, 54, 81], vec![9, 18, 27]);
}

#[test]
#[ignore = "We don't have lo breaking rule yet"]
fn wordbreak_normal_lo() {
    // from css/css-text/word-break/word-break-normal-lo-000.html
    let _s = "ພາສາລາວພາສາລາວພາສາລາວ";
    normal(_s, vec![12, 21, 33, 42, 54, 63], vec![4, 7, 11, 14, 18, 21]);
}
