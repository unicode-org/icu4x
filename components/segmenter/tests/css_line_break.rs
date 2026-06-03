// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::{langid, LanguageIdentifier};
use icu_segmenter::options::LineBreakOptions;
use icu_segmenter::options::LineBreakStrictness;
use icu_segmenter::options::LineBreakWordOption;
use icu_segmenter::*;

include!("helpers.rs.raw");

static JA: LanguageIdentifier = langid!("ja");

#[track_caller]
fn strict(s: &str, ja_zh: bool, expected: &[&str]) {
    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Strict);
    options.word_option = Some(LineBreakWordOption::Normal);
    options.content_locale = ja_zh.then_some(&JA);
    check_line(s, expected, LineSegmenter::new_dictionary(options));
}

#[track_caller]
fn normal(s: &str, ja_zh: bool, expected: &[&str]) {
    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Normal);
    options.word_option = Some(LineBreakWordOption::Normal);
    options.content_locale = ja_zh.then_some(&JA);
    check_line(s, expected, LineSegmenter::new_dictionary(options));
}

#[track_caller]
fn loose(s: &str, ja_zh: bool, expected: &[&str]) {
    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Loose);
    options.word_option = Some(LineBreakWordOption::Normal);
    options.content_locale = ja_zh.then_some(&JA);
    check_line(s, expected, LineSegmenter::new_dictionary(options));
}

#[track_caller]
fn anywhere(s: &str, ja_zh: bool, expected: &[&str]) {
    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Anywhere);
    options.word_option = Some(LineBreakWordOption::Normal);
    options.content_locale = ja_zh.then_some(&JA);
    check_line(s, expected, LineSegmenter::new_dictionary(options));
}

#[test]
fn linebreak_strict() {
    // from css/css-text/line-break/line-break-*-011.xht
    strict("サぁサ", false, &["サぁ", "サ"]);

    // from css/css-text/line-break/line-break-*-012.xht
    strict("サーサ", false, &["サー", "サ"]);

    // from css/css-text/line-break/line-break-*-013.xht
    strict("サ〜サ", false, &["サ〜", "サ"]);

    // from css/css-text/line-break/line-break-*-014.xht
    strict("サ々サ", false, &["サ々", "サ"]);

    // from css/css-text/line-break/line-break-*-015a.xht
    strict("‥‥サ", false, &["‥‥", "サ"]);

    // from css/css-text/line-break/line-break-*-016a.xht
    strict("サ・サ", false, &["サ・", "サ"]);

    // from css/css-text/line-break/line-break-*-017a.xht
    strict("サ°サ", false, &["サ°", "サ"]);

    // from css/css-text/line-break/line-break-*-018.xht
    strict("サ€サ", false, &["サ", "€サ"]);

    // from css/css-text/i18n/ja/css-text-line-break-ja-pr-strict.html
    // TODO: Why ID ÷ ID × PR × ID ÷ ID ?
    // strict("文文±字字", true, &["文", "文±字", "字"]);
    // strict("文文€字字", true, &["文", "文€字", "字"]);
    // strict("文文＄字字", true, &["文", "文＄字", "字"]);
}

#[test]
fn linebreak_normal() {
    // from css/css-text/line-break/line-break-*-011.xht
    normal("サぁサ", false, &["サ", "ぁ", "サ"]);

    // from css/css-text/line-break/line-break-*-012.xht
    normal("サーサ", false, &["サ", "ー", "サ"]);

    // from css/css-text/line-break/line-break-*-013.xht
    normal("サ〜サ", true, &["サ", "〜", "サ"]);

    // from css/css-text/line-break/line-break-*-014.xht
    normal("サ々サ", true, &["サ々", "サ"]);

    // from css/css-text/line-break/line-break-*-015.xht
    normal("‥‥サ", true, &["‥‥", "サ"]);

    // from css/css-text/line-break/line-break-*-016a.xht
    normal("サ・サ", true, &["サ・", "サ"]);

    // from css/css-text/line-break/line-break-*-017a.xht
    normal("サ°サ", true, &["サ°", "サ"]);

    // from css/css-text/line-break/line-break-*-018.xht
    normal("サ€サ", true, &["サ", "€サ"]);

    // from css/css-text/i18n/unknown-lang/css-text-line-break-pr-normal.html
    normal("文文±字字", false, &["文", "文", "±字", "字"]);
    normal("文文€字字", false, &["文", "文", "€字", "字"]);
    normal("文文№字字", false, &["文", "文", "№字", "字"]);
}

#[test]
fn linebreak_loose() {
    // from css/css-text/line-break/line-break-*-011.xht
    loose("サぁサ", true, &["サ", "ぁ", "サ"]);

    // from css/css-text/line-break/line-break-*-012.xht
    loose("サーサ", true, &["サ", "ー", "サ"]);

    // from css/css-text/line-break/line-break-loose-013.xht
    loose("サ〜サ", true, &["サ", "〜", "サ"]);

    // from css/css-text/line-break/line-break-*-014.xht
    loose("サ々サ", true, &["サ", "々", "サ"]);

    // from css/css-text/line-break/line-break-*-015.xht
    loose("‥‥サ", true, &["‥", "‥", "サ"]);

    // from css/css-text/line-break/line-break-*-016a.xht
    loose("サ・サ", true, &["サ", "・", "サ"]);

    // from css/css-text/line-break/line-break-*-017a.xht
    loose("サ°サ", true, &["サ", "°", "サ"]);

    // from css/css-text/line-break/line-break-*-018.xht
    loose("文€文", true, &["文", "€", "文"]);
    loose("文№文", true, &["文", "№", "文"]);
    loose("文＄文", true, &["文", "＄", "文"]);
    loose("文￡文", true, &["文", "￡", "文"]);
    loose("文￥文", true, &["文", "￥", "文"]);

    // from css/css-text/i18n/ja/css-text-line-break-ja-pr-loose.html
    loose("文±文", true, &["文", "±", "文"]);
    loose("文€文", true, &["文", "€", "文"]);
    loose("文＄文", true, &["文", "＄", "文"]);

    // from css/css-text/i18n/unknown-lang/css-text-line-break-in-loose.html
    loose("文․文", false, &["文", "․", "文"]);
    loose("文‥文", false, &["文", "‥", "文"]);
    loose("文…文", false, &["文", "…", "文"]);
    loose("文⋯文", false, &["文", "⋯", "文"]);
    loose("文︙文", false, &["文", "︙", "文"]);

    // from css/css-text/i18n/unknown-lang/css-text-line-break-pr-loose.html
    loose("文±文", false, &["文", "±文"]);
    loose("文€文", false, &["文", "€文"]);
    loose("文№文", false, &["文", "№文"]);
    loose("文＄文", false, &["文", "＄文"]);

    // from css/css-text/i18n/zh/css-text-line-break-zh-in-loose.xht
    loose("文․文", true, &["文", "․", "文"]);
    loose("文‥文", true, &["文", "‥", "文"]);
    loose("文…文", true, &["文", "…", "文"]);
    loose("文⋯文", true, &["文", "⋯", "文"]);
    loose("文︙文", true, &["文", "︙", "文"]);

    // css/css-text/line-break/line-break-loose-hyphens-001.html
    loose("文‐文", true, &["文", "‐", "文"]);
    loose("文–文", true, &["文", "–", "文"]);

    // css/css-text/line-break/line-break-loose-hyphens-003.html
    loose("aa‐", false, &["aa‐"]);
    loose("aa–", false, &["aa–"]);
}

#[test]
fn linebreak_anywhere() {
    anywhere(
        "الخيل والليل",
        false,
        &["ا", "ل", "خ", "ي", "ل", " ", "و", "ا", "ل", "ل", "ي", "ل"],
    );

    // css/css-text/line-break/line-break-anywhere-001.html
    anywhere(
        "aa-a.a)a,a) a aa\u{2060}aa･a",
        true,
        &[
            "a", "a", "-", "a", ".", "a", ")", "a", ",", "a", ")", " ", "a", " ", "a", "a",
            "\u{2060}", "a", "a", "･", "a",
        ],
    );

    // css/css-text/line-break/line-break-anywhere-002.html
    anywhere(
        "no hyphenation",
        false,
        &[
            "n", "o", " ", "h", "y", "p", "h", "e", "n", "a", "t", "i", "o", "n",
        ],
    );

    // css/css-text/line-break/line-break-anywhere-003.html
    anywhere("latin", false, &["l", "a", "t", "i", "n"]);

    // css/css-text/line-break/line-break-anywhere-004.html
    anywhere("XX XXX", false, &["X", "X", " ", "X", "X", "X"]);

    // css/css-text/line-break/line-break-anywhere-005.html
    anywhere("X X", false, &["X", " ", "X"]);

    // css/css-text/line-break/line-break-anywhere-006.html
    anywhere(
        "XXXX XXXX",
        false,
        &["X", "X", "X", "X", " ", "X", "X", "X", "X"],
    );

    // css/css-text/line-break/line-break-anywhere-007.html
    anywhere("X XX...", true, &["X", " ", "X", "X", ".", ".", "."]);

    // css/css-text/line-break/line-break-anywhere-008.html
    anywhere("X XX...", true, &["X", " ", "X", "X", ".", ".", "."]);

    // css/css-text/line-break/line-break-anywhere-009.html
    anywhere("X X", true, &["X", " ", "X"]);

    // css/css-text/line-break/line-break-anywhere-010.html
    anywhere(
        "XXXX XXXX",
        true,
        &["X", "X", "X", "X", " ", "X", "X", "X", "X"],
    );

    // css/css-text/line-break/line-break-anywhere-011.html
    anywhere("XX///", true, &["X", "X", "/", "/", "/"]);

    // css/css-text/line-break/line-break-anywhere-012.html
    anywhere(r#"X XX\\\"#, true, &["X", " ", "X", "X", "\\", "\\", "\\"]);

    // css/css-text/line-break/line-break-anywhere-013.html
    anywhere("XXX/X", true, &["X", "X", "X", "/", "X"]);

    // css/css-text/line-break/line-break-anywhere-014.html
    anywhere(r#"XXX\X"#, false, &["X", "X", "X", "\\", "X"]);

    // css/css-text/line-break/line-break-anywhere-015.html
    anywhere(r#"XXX\X"#, false, &["X", "X", "X", "\\", "X"]);

    // css/css-text/line-break/line-break-anywhere-016.html
    anywhere("XXX/X", false, &["X", "X", "X", "/", "X"]);

    // css/css-text/line-break/line-break-anywhere-017.html
    anywhere("XXXX X", false, &["X", "X", "X", "X", " ", "X"]);

    // line-break-anywhere-overrides-uax-behavior-001.htm
    anywhere("XX\u{2060}XX", false, &["X", "X", "\u{2060}", "X", "X"]);

    // line-break-anywhere-overrides-uax-behavior-004.htm
    anywhere(
        "..\u{200B}...X",
        false,
        &[".", ".", "\u{200B}", ".", ".", ".", "X"],
    );
}
