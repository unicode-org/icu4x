// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::langid;
use icu_segmenter::neo::*;
use icu_segmenter::options::{SentenceBreakOptions, WordBreakOptions};

include!("helpers.rs.raw");

// Additional segmenter tests with locale.

#[test]
fn word_break_with_locale() {
    let mut options = WordBreakOptions::default();

    let langid = langid!("sv");
    options.content_locale = Some(&langid);
    let segmenter = WordSegmenter::try_new_auto(options).unwrap();

    check_word("hello:world", &["hello:world"], segmenter.as_borrowed());

    let langid = langid!("en");
    options.content_locale = Some(&langid);
    let segmenter = WordSegmenter::try_new_auto(options).unwrap();

    check_word("hello:world", &["hello:world"], segmenter.as_borrowed());
}

#[test]
fn sentence_break_with_locale() {
    // SB11 is different because U+0x3B is STerm on Greek.
    let mut options = SentenceBreakOptions::default();

    let langid = langid!("el");
    options.content_locale = Some(&langid);
    let segmenter = SentenceSegmenter::try_new(options).unwrap();
    check_sentence(
        "hello; world",
        &["hello; ", "world"],
        segmenter.as_borrowed(),
    );

    let langid = langid!("en");
    options.content_locale = Some(&langid);
    let segmenter = SentenceSegmenter::try_new(options).unwrap();
    check_sentence("hello; world", &["hello; world"], segmenter.as_borrowed());
}
