// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::neo::*;
use icu_segmenter::options::LineBreakOptions;
use icu_segmenter::options::LineBreakStrictness;
use icu_segmenter::options::LineBreakWordOption;

include!("helpers.rs.raw");

#[track_caller]
fn break_all(s: &str, expected: &[&str]) {
    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Strict);
    options.word_option = Some(LineBreakWordOption::BreakAll);
    options.content_locale = None;
    check_line(s, expected, LineSegmenter::new_dictionary(options));
}

#[track_caller]
fn keep_all(s: &str, expected: &[&str]) {
    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Strict);
    options.word_option = Some(LineBreakWordOption::KeepAll);
    options.content_locale = None;
    check_line(s, expected, LineSegmenter::new_dictionary(options));
}

#[track_caller]
fn normal(s: &str, expected: &[&str]) {
    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Strict);
    options.word_option = Some(LineBreakWordOption::Normal);
    options.content_locale = None;
    check_line(s, expected, LineSegmenter::new_dictionary(options));
}

#[test]
fn wordbreak_breakall() {
    // from css/css-text/word-break/word-break-break-all-000.html
    break_all("日本語", &["日", "本", "語"]);

    // from css/css-text/word-break/word-break-break-all-001.html
    break_all("latin", &["l", "a", "t", "i", "n"]);

    // from css/css-text/word-break/word-break-break-all-002.html
    break_all("한글읾", &["한", "글", "읾"]);

    // from css/css-text/word-break/word-break-break-all-003.html
    break_all(
        "ภาษาไทยภาษาไทย",
        &[
            "ภ", "า", "ษ", "า", "ไ", "ท", "ย", "ภ", "า", "ษ", "า", "ไ", "ท", "ย",
        ],
    );

    // from css/css-text/word-break/word-break-break-all-004.html
    break_all(
        "التدويل نشاط التدويل",
        &[
            "ا", "ل", "ت", "د", "و", "ي", "ل ", "ن", "ش", "ا", "ط ", "ا", "ل", "ت", "د", "و", "ي",
            "ل",
        ],
    );

    // from css/css-text/word-break/word-break-break-all-008.html
    break_all(
        "हिन्दी हिन्दी हिन्दी",
        &["हि", "न्", "दी ", "हि", "न्", "दी ", "हि", "न्", "दी"],
    );

    // from css/css-text/word-break/word-break-break-all-014.html
    break_all("💖💔", &["💖", "💔"]);

    // from css/css-text/word-break/word-break-break-all-018.html
    // break_all("XXXX X", &["X", "X", "X", "X\u{a0}", "X"]);

    // from css/css-text/word-break/word-break-break-all-022.html
    // break_all("XX X", &["X", "X", " ", "X"]);

    // from css/css-text/word-break/word-break-break-all-023.html
    break_all(r#"XX XX\\\"#, &["X", "X ", "X", "X", "\\", "\\", "\\"]);

    // from css/css-text/word-break/word-break-break-all-026.html
    break_all("XX XXX///", &["X", "X ", "X", "X", "X///"]);

    // css/css-text/word-break/word-break-break-all-inline-008.html
    break_all("X.", &["X."]);

    // ID and CJ
    break_all("フォ", &["フ", "ォ"]);
}

#[test]
fn wordbreak_keepall() {
    // from css/css-text/word-break/word-break-keep-all-000.html
    keep_all("latin", &["latin"]);

    // from css/css-text/word-break/word-break-keep-all-001.html
    keep_all("日本語", &["日本語"]);

    // from css/css-text/word-break/word-break-keep-all-002.html
    keep_all("한글이", &["한글이"]);

    // from css/css-text/word-break/word-break-keep-all-005.html
    keep_all("字　字", &["字　", "字"]);

    // from css/css-text/word-break/word-break-keep-all-006.html
    keep_all("字、字", &["字、", "字"]);

    // from css/css-text/word-boundary/word-boundary-107.html
    keep_all("しょう。", &["しょう。"]);

    // failed test. JL, JV and JT
    keep_all("애기판다", &["애기판다"]);

    // from css/css-text/word-break/word-break-keep-all-003.html
    keep_all("และและ", &["และ", "และ"]);
}

#[test]
fn wordbreak_normal_th() {
    // from css/css-text/word-break/word-break-normal-th-000.html
    normal("ภาษาไทยภาษาไทย", &["ภาษา", "ไทย", "ภาษา", "ไทย"]);
}

#[test]
fn wordbreak_normal_km() {
    // from css/css-text/word-break/word-break-normal-km-000.html
    normal("ភាសាខ្មែរភាសាខ្មែរភាសាខ្មែរ", &["ភាសាខ្មែរ", "ភាសាខ្មែរ", "ភាសាខ្មែរ"]);
}

#[test]
fn wordbreak_normal_lo() {
    // from css/css-text/word-break/word-break-normal-lo-000.html
    normal(
        "ພາສາລາວພາສາລາວພາສາລາວ",
        &["ພາສາ", "ລາວ", "ພາສາ", "ລາວ", "ພາສາ", "ລາວ"],
    );
}
