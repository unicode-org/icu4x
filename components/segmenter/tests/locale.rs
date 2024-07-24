// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::locale;
use icu_segmenter::{SentenceSegmenter, WordSegmenter};

// Additional segmenter tests with locale.

#[test]
fn word_break_with_locale() {
    // MidLetter is different because U+0x3A isn't MidLetter on Swedish.
    let s = "hello:world";
    let segmenter =
        WordSegmenter::try_new_auto(&locale!("sv").into()).expect("Loading should succeed!");
    let iter = segmenter.segment_str(s);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 11],
        "word segmenter with Swedish"
    );

    let segmenter =
        WordSegmenter::try_new_auto(&locale!("en").into()).expect("Loading should succeed!");
    let iter = segmenter.segment_str(s);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 5, 6, 11],
        "word segmenter with English"
    );
}

#[test]
fn sentence_break_with_locale() {
    // SB11 is different because U+0x3B is STerm on Greek.
    let s = "hello; world";
    let segmenter =
        SentenceSegmenter::try_new(&locale!("el").into()).expect("Loading should succeed!");
    let iter = segmenter.segment_str(s);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 7, 12],
        "sentence segmenter with Greek"
    );

    let segmenter =
        SentenceSegmenter::try_new(&locale!("en").into()).expect("Loading should succeed!");
    let iter = segmenter.segment_str(s);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 12],
        "sentence segmenter with English"
    );
}
