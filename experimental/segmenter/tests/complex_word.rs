// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::WordBreakSegmenter;

// Additional word segmenter tests with complex string.

#[test]
fn word_break_th() {
    let provider = icu_testdata::get_provider();
    let segmenter = WordBreakSegmenter::try_new(&provider).expect("Data exists");

    // http://wpt.live/css/css-text/word-break/word-break-normal-th-000.html
    let s = "ภาษาไทยภาษาไทย";
    let utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = segmenter.segment_utf16(&utf16);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 4, 7, 11, 14],
        "word segmenter with Thai"
    );
    let iter = segmenter.segment_str(s);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 12, 21, 33, 42],
        "word segmenter with Thai"
    );

    // Combine non-Thai and Thai.
    let s = "aภาษาไทยภาษาไทยb";
    let utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = segmenter.segment_utf16(&utf16);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 1, 5, 8, 12, 15, 16],
        "word segmenter with Thai and ascii"
    );
}

#[ignore = "testdata doesn't have Burmese data"]
#[test]
fn word_break_my() {
    let provider = icu_testdata::get_provider();
    let segmenter = WordBreakSegmenter::try_new(&provider).expect("Data exists");

    let s = "မြန်မာစာမြန်မာစာမြန်မာစာ";
    let utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = segmenter.segment_utf16(&utf16);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 8, 16, 22, 24],
        "word segmenter with Burmese"
    );
}

#[test]
fn word_break_hiragana() {
    let provider = icu_testdata::get_provider();
    let segmenter = WordBreakSegmenter::try_new(&provider).expect("Data exists");

    let s = "うなぎうなじ";
    let iter = segmenter.segment_str(s);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 9, 18],
        "word segmenter with Hiragana"
    );
}

#[test]
fn word_break_mixed_han() {
    let provider = icu_testdata::get_provider();
    let segmenter = WordBreakSegmenter::try_new(&provider).expect("Data exists");

    let s = "Welcome龟山岛龟山岛Welcome";
    let iter = segmenter.segment_str(s);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 7, 16, 25, 32],
        "word segmenter with Chinese and letter"
    );
}
