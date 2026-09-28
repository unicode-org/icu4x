// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::*;
use icu_segmenter::{WordSegmenter, options::WordBreakInvariantOptions};

include!("helpers.rs.raw");

// Additional word segmenter tests with complex string.

#[test]
fn word_break_th() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        WordSegmenter::new_lstm(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
    ] {
        // http://wpt.live/css/css-text/word-break/word-break-normal-th-000.html
        let s = "ภาษาไทยภาษาไทย";
        let expected = ["ภาษา", "ไทย", "ภาษา", "ไทย"];

        check_word(s, &expected, segmenter);

        // Combine non-Thai and Thai.
        let s = "aภาษาไทยภาษาไทยb";
        let expected = ["a", "ภาษา", "ไทย", "ภาษา", "ไทย", "b"];

        check_word(s, &expected, segmenter);
    }
}

fn check_word_file(file: &'static str, segmenter: WordSegmenterBorrowed<'_>) {
    for line in file.lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let expected = line.split('|').collect::<Vec<_>>();
        let s = expected.concat();
        check_word(&s, &expected, segmenter);
    }
}

// The expected segmentations in the test data file match ICU4C (ICU 78).
// The file is intended to also live in the ICU repository, where the
// dictionary data lives, and to be tested against ICU4C there.
#[test]
fn word_break_th_dictionary() {
    for segmenter in [
        WordSegmenter::new_dictionary(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_dictionary();
            s
        },
    ] {
        check_word_file(include_str!("testdata/WordBreakThaiTest.txt"), segmenter);
    }
}

#[test]
fn word_break_my() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
    ] {
        let s = "မြန်မာစာမြန်မာစာမြန်မာစာ";
        let expected = ["မြန်မာစာ", "မြန်မာစာ", "မြန်မာ", "စာ"];
        check_word(s, &expected, segmenter);
    }
}

#[test]
fn word_break_hiragana() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        WordSegmenter::new_dictionary(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_dictionary();
            s
        },
    ] {
        let s = "うなぎうなじ";
        let expected = ["うなぎ", "うなじ"];
        check_word(s, &expected, segmenter);
    }
}

#[test]
fn word_break_mixed_han() {
    for segmenter in [
        WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
        WordSegmenter::new_dictionary(WordBreakInvariantOptions::default()),
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_auto();
            s
        },
        {
            let mut s = WordSegmenter::new_neo_for_non_complex_scripts(
                WordBreakInvariantOptions::default(),
            );
            s.load_dictionary();
            s
        },
    ] {
        let s = "Welcome龟山岛龟山岛Welcome";
        let expected = ["Welcome", "龟山岛", "龟山岛", "Welcome"];
        check_word(s, &expected, segmenter);
    }
}

#[test]
fn word_line_th_wikipedia_auto() {
    use icu_segmenter::LineSegmenter;

    let text = "แพนด้าแดง (อังกฤษ: Red panda, Shining cat; จีน: 小熊貓; พินอิน: Xiǎo xióngmāo) สัตว์เลี้ยงลูกด้วยนมชนิดหนึ่ง มีชื่อวิทยาศาสตร์ว่า Ailurus fulgens";

    for segmenter in [WordSegmenter::new_auto(Default::default()), {
        let mut s =
            WordSegmenter::new_neo_for_non_complex_scripts(WordBreakInvariantOptions::default());
        s.load_auto();
        s
    }] {
        check_word(
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
            segmenter,
        );
    }

    check_line(
        text,
        &[
            "แพน",
            "ด้า",
            "แดง",
            " ",
            "(อัง",
            "กฤษ",
            ": ",
            "Red ",
            "panda, ",
            "Shining ",
            "cat; ",
            "จีน",
            ": ",
            "小",
            "熊",
            "貓; ",
            "พิน",
            "อิน",
            ": ",
            "Xiǎo ",
            "xióngmāo) ",
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
            "Ailurus ",
            "fulgens",
        ],
        LineSegmenter::new_auto(Default::default()),
    );

    check_line(
        text,
        &[
            "แพน",
            "ด้า",
            "แดง",
            " ",
            "(อัง",
            "กฤษ",
            ": ",
            "Red ",
            "panda, ",
            "Shining ",
            "cat; ",
            "จีน",
            ": ",
            "小",
            "熊",
            "貓; ",
            "พิน",
            "อิน",
            ": ",
            "Xiǎo ",
            "xióngmāo) ",
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
            "Ailurus ",
            "fulgens",
        ],
        {
            let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
            s.load_lstm();
            s
        },
    );

    check_line(
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
        {
            let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
            s.load_lstm();
            s
        },
    );
}
