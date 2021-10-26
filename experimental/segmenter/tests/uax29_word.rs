// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::WordBreakIterator;
use icu_segmenter::WordBreakIteratorUtf16;

// Additional word segmenter tests with complex string.

#[test]
fn word_break_th() {
    // http://wpt.live/css/css-text/word-break/word-break-normal-th-000.html
    let s = "ภาษาไทยภาษาไทย";
    let utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = WordBreakIteratorUtf16::new(&utf16);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 4, 7, 11, 14],
        "word segmenter with Thai"
    );
    let iter = WordBreakIterator::new(s);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 12, 21, 33, 42],
        "word segmenter with Thai"
    );

    // Combine non-Thai and Thai.
    let s = "aภาษาไทยภาษาไทยb";
    let utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = WordBreakIteratorUtf16::new(&utf16);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 1, 5, 8, 12, 15, 16],
        "word segmenter with Thai and ascii"
    );
}

#[test]
fn word_break_my() {
    let s = "မြန်မာစာမြန်မာစာမြန်မာစာ";
    let utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = WordBreakIteratorUtf16::new(&utf16);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 8, 16, 22, 24],
        "word segmenter with Burmese"
    );
}
