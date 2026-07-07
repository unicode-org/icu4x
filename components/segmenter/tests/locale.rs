// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::langid;
use icu_segmenter::options::{SentenceBreakOptions, WordBreakOptions};
use icu_segmenter::*;

include!("helpers.rs.raw");

// Additional segmenter tests with locale.

#[test]
fn word_break_with_locale() {
    let mut options = WordBreakOptions::default();

    let langid = langid!("sv");
    options.content_locale = Some(&langid);
    check_word(
        "hello:world",
        &["hello:world"],
        WordSegmenter::try_new_auto(options).unwrap().as_borrowed(),
    );
    check_word(
        "hello:world",
        &["hello:world"],
        WordSegmenter::try_new_neo_auto(options)
            .unwrap()
            .as_borrowed(),
    );

    let langid = langid!("en");
    options.content_locale = Some(&langid);
    check_word(
        "hello:world",
        &["hello:world"],
        WordSegmenter::try_new_neo_auto(options)
            .unwrap()
            .as_borrowed(),
    );
}

#[test]
fn sentence_break_with_locale() {
    // SB11 is different because U+0x3B is STerm on Greek.
    let mut options = SentenceBreakOptions::default();

    let langid = langid!("el");
    options.content_locale = Some(&langid);
    check_sentence(
        "hello; world",
        &["hello; ", "world"],
        SentenceSegmenter::try_new(options).unwrap().as_borrowed(),
    );
    check_sentence(
        "hello; world",
        &["hello; ", "world"],
        SentenceSegmenter::try_new_neo(options)
            .unwrap()
            .as_borrowed(),
    );

    let langid = langid!("en");
    options.content_locale = Some(&langid);
    check_sentence(
        "hello; world",
        &["hello; world"],
        SentenceSegmenter::try_new(options).unwrap().as_borrowed(),
    );
    check_sentence(
        "hello; world",
        &["hello; world"],
        SentenceSegmenter::try_new_neo(options)
            .unwrap()
            .as_borrowed(),
    );
}
