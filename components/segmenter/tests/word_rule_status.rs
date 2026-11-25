// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::WordSegmenter;
use icu_segmenter::options::WordBreakInvariantOptions;
use icu_segmenter::options::WordType;

#[test]
fn rule_status() {
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
        let mut iter = segmenter.segment_str("hello world 123");

        assert_eq!(iter.next(), Some(0), "SOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "SOT is false");

        assert_eq!(iter.next(), Some(5), "after hello");
        assert_eq!(iter.word_type(), WordType::Letter, "letter");
        assert!(iter.is_word_like(), "Letter is true");

        assert_eq!(iter.next(), Some(6), "after space");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "None is false");

        assert_eq!(iter.next(), Some(11), "after world");
        assert_eq!(iter.word_type(), WordType::Letter, "letter");
        assert!(iter.is_word_like(), "Letter is true");

        assert_eq!(iter.next(), Some(12), "after space");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "None is false");

        assert_eq!(iter.next(), Some(15), "after number");
        assert_eq!(iter.word_type(), WordType::Number, "number");
        assert!(iter.is_word_like(), "Number is true");

        assert_eq!(iter.next(), None, "EOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "None is false");
    }
}

#[test]
fn rule_status_letter_eof() {
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
        let mut iter = segmenter.segment_str("one.");

        assert_eq!(iter.next(), Some(0), "SOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "SOT is false");

        assert_eq!(iter.next(), Some(3), "after one");
        assert_eq!(iter.word_type(), WordType::Letter, "letter");
        assert!(iter.is_word_like(), "Letter is true");

        assert_eq!(iter.next(), Some(4), "after full stop");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "None is false");

        assert_eq!(iter.next(), None, "EOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "None is false");
    }
}

#[test]
fn rule_status_numeric_eof() {
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
        let mut iter = segmenter.segment_str("42.");

        assert_eq!(iter.next(), Some(0), "SOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "SOT is false");

        assert_eq!(iter.next(), Some(2), "after 42");
        assert_eq!(iter.word_type(), WordType::Number, "Number");
        assert!(iter.is_word_like(), "Number is true");

        assert_eq!(iter.next(), Some(3), "after full stop");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "None is false");

        assert_eq!(iter.next(), None, "EOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "None is false");
    }
}

#[test]
fn rule_status_th() {
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
        let mut iter = segmenter.segment_str("ภาษาไทยภาษาไทย");

        assert_eq!(iter.next(), Some(0), "SOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "SOT is false");

        assert_eq!(iter.next(), Some(12), "after 1st word");
        assert_eq!(iter.word_type(), WordType::Letter, "letter");
        assert!(iter.is_word_like(), "Letter(Thai) is true");

        assert_eq!(iter.next(), Some(21), "after 2nd word");
        assert_eq!(iter.word_type(), WordType::Letter, "letter");
        assert!(iter.is_word_like(), "Letter(Thai) is true");

        assert_eq!(iter.next(), Some(33), "after 3rd word");
        assert_eq!(iter.word_type(), WordType::Letter, "letter");
        assert!(iter.is_word_like(), "Letter(Thai) is true");

        assert_eq!(iter.next(), Some(42), "after 4th word and next is EOT");
        assert_eq!(iter.word_type(), WordType::Letter, "letter");
        assert!(iter.is_word_like(), "Letter(Thai) is true");

        assert_eq!(iter.next(), None, "EOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "None is false");
    }
}

#[test]
fn rule_status_letter_followed_by_zwj_and_extend() {
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
        // WB4: a trailing ZWJ or Extend does not change the type of the word before it.
        // "a" is ALetter and U+200D is ZWJ, so the word must still report Letter instead
        // of the status of the `ALetter_ZWJ` state that follows it.
        let mut iter = segmenter.segment_str("a\u{200d}");

        assert_eq!(iter.next(), Some(0), "SOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "SOT is false");

        assert_eq!(iter.next(), Some(4), "after a and ZWJ");
        assert_eq!(iter.word_type(), WordType::Letter, "letter");
        assert!(iter.is_word_like(), "Letter is true");

        assert_eq!(iter.next(), None, "EOT");
        assert_eq!(iter.word_type(), WordType::None, "none");
        assert!(!iter.is_word_like(), "None is false");
    }
}

#[test]
fn rule_status_katakana_followed_by_zwj() {
    // Only the UAX #29 rule-based segmenter is exercised here: its `rule_status_table` is
    // generated from `provider/source/data/segmenter/word.toml`. The neo segmenter reads
    // its statuses from the ICU state machine data instead, and currently breaks "ヱ‍"
    // into two segments, which is a separate question from the reported status values.
    let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());

    // "ヱ" (U+30F1) is Katakana and the trailing U+200D is ZWJ. The status used to come
    // from the `Katakana_ZWJ` state's index, which read an unrelated rule's value.
    let mut iter = segmenter.segment_str("\u{30f1}\u{200d}");

    assert_eq!(iter.next(), Some(0), "SOT");
    assert_eq!(iter.word_type(), WordType::None, "none");

    assert_eq!(iter.next(), Some(6), "after katakana and ZWJ");
    assert_eq!(iter.word_type(), WordType::Letter, "letter");
    assert!(iter.is_word_like(), "Letter is true");

    assert_eq!(iter.next(), None, "EOT");
    assert_eq!(iter.word_type(), WordType::None, "none");
}
