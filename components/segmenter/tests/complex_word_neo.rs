// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::{neo::LineSegmenterBorrowed, neo::WordSegmenterBorrowed};
use icu_segmenter::{neo::WordSegmenter, options::WordBreakInvariantOptions};
use itertools::Itertools;

// Additional word segmenter tests with complex string.

#[track_caller]
fn test_word(segmenter: WordSegmenterBorrowed, s: &str, expected: &[&str]) {
    let segments = segmenter
        .segment_str(s)
        .tuple_windows()
        .map(|(a, b)| &s[a..b])
        .collect::<Vec<_>>();
    assert_eq!(segments, expected);

    let utf16: Vec<u16> = s.encode_utf16().collect();
    let expected = expected
        .iter()
        .copied()
        .map(|s| s.encode_utf16().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let iter = segmenter
        .segment_utf16(&utf16)
        .tuple_windows()
        .map(|(a, b)| &utf16[a..b])
        .collect::<Vec<_>>();
    assert_eq!(iter, expected);
}

#[track_caller]
fn test_line(segmenter: LineSegmenterBorrowed, s: &str, expected: &[&str]) {
    let segments = segmenter
        .segment_str(s)
        .tuple_windows()
        .map(|(a, b)| &s[a..b])
        .collect::<Vec<_>>();
    assert_eq!(segments, expected);

    let utf16: Vec<u16> = s.encode_utf16().collect();
    let expected = expected
        .iter()
        .copied()
        .map(|s| s.encode_utf16().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let iter = segmenter
        .segment_utf16(&utf16)
        .tuple_windows()
        .map(|(a, b)| &utf16[a..b])
        .collect::<Vec<_>>();
    assert_eq!(iter, expected);
}

#[test]
fn word_break_th() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        WordSegmenter::new_lstm(WordBreakInvariantOptions::default()),
    ] {
        // http://wpt.live/css/css-text/word-break/word-break-normal-th-000.html
        let s = "ภาษาไทยภาษาไทย";
        let expected = ["ภาษา", "ไทย", "ภาษา", "ไทย"];

        test_word(segmenter, s, &expected);

        // Combine non-Thai and Thai.
        let s = "aภาษาไทยภาษาไทยb";
        let expected = ["a", "ภาษา", "ไทย", "ภาษา", "ไทย", "b"];

        test_word(segmenter, s, &expected);
    }
}

#[test]
fn word_break_my() {
    let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());

    let s = "မြန်မာစာမြန်မာစာမြန်မာစာ";
    let expected = ["မြန်မာစာ", "မြန်မာစာ", "မြန်မာ", "စာ"];
    test_word(segmenter, s, &expected);
}

#[test]
fn word_break_hiragana() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        WordSegmenter::new_dictionary(WordBreakInvariantOptions::default()),
    ] {
        let s = "うなぎうなじ";
        let expected = ["うなぎ", "うなじ"];
        test_word(segmenter, s, &expected);
    }
}

#[test]
fn word_break_mixed_han() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        WordSegmenter::new_dictionary(WordBreakInvariantOptions::default()),
    ] {
        let s = "Welcome龟山岛龟山岛Welcome";
        let expected = ["Welcome", "龟山岛", "龟山岛", "Welcome"];
        test_word(segmenter, s, &expected);
    }
}

#[test]
fn word_line_th_wikipedia_auto() {
    use icu_segmenter::neo::LineSegmenter;

    let text = "แพนด้าแดง (อังกฤษ: Red panda, Shining cat; จีน: 小熊貓; พินอิน: Xiǎo xióngmāo) สัตว์เลี้ยงลูกด้วยนมชนิดหนึ่ง มีชื่อวิทยาศาสตร์ว่า Ailurus fulgens";

    let segmenter_word_auto = WordSegmenter::new_auto(Default::default());
    let segmenter_line_auto = LineSegmenter::new_auto(Default::default());

    test_word(
        segmenter_word_auto,
        text,
        &[
            "แพน",
            "ด้า",
            "แดง",
            " ",
            "(",
            "อัง",
            "กฤษ",
            ":",
            " ",
            "Red",
            " ",
            "panda",
            ",",
            " ",
            "Shining",
            " ",
            "cat",
            ";",
            " ",
            "จีน",
            ":",
            " ",
            "小熊",
            "貓",
            ";",
            " ",
            "พิน",
            "อิน",
            ":",
            " ",
            "Xiǎo",
            " ",
            "xióngmāo",
            ")",
            " ",
            "สัตว์",
            "เลี้ยง",
            "ลูก",
            "ด้วย",
            "นม",
            "ชนิด",
            "หนึ่ง",
            " ",
            "มี",
            "ชื่อ",
            "วิทยาศาสตร์",
            "ว่า",
            " ",
            "Ailurus",
            " ",
            "fulgens",
        ],
    );
    test_line(
        segmenter_line_auto,
        text,
        &[
            "แพน",
            "ด้า",
            "แดง ",
            "(อัง",
            "กฤษ: ",
            "Red ",
            "panda, ",
            "Shining ",
            "cat; ",
            "จีน: ",
            "小",
            "熊",
            "貓; ",
            "พิน",
            "อิน: ",
            "Xiǎo ",
            "xióngmāo) ",
            "สัตว์",
            "เลี้ยง",
            "ลูก",
            "ด้วย",
            "นม",
            "ชนิด",
            "หนึ่ง ",
            "มี",
            "ชื่อ",
            "วิทยาศาสตร์",
            "ว่า ",
            "Ailurus ",
            "fulgens",
        ],
    );
}
