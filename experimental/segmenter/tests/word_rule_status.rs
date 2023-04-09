// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::RuleStatusType;
use icu_segmenter::WordSegmenter;

#[test]
fn rule_status() {
    let segmenter =
        WordSegmenter::try_new_auto_unstable(&icu_testdata::unstable()).expect("Data exists");
    let mut iter = segmenter.segment_str("hello world 123");

    assert_eq!(iter.next(), Some(0), "SOT");
    assert_eq!(iter.rule_status(), RuleStatusType::None, "none");
    assert!(!iter.is_word_like(), "SOT is false");

    assert_eq!(iter.next(), Some(5), "after hello");
    assert_eq!(iter.rule_status(), RuleStatusType::Letter, "letter");
    assert!(iter.is_word_like(), "Letter is true");

    assert_eq!(iter.next(), Some(6), "after space");
    assert_eq!(iter.rule_status(), RuleStatusType::None, "none");
    assert!(!iter.is_word_like(), "None is false");

    assert_eq!(iter.next(), Some(11), "after world");
    assert_eq!(iter.rule_status(), RuleStatusType::Letter, "letter");
    assert!(iter.is_word_like(), "Letter is true");

    assert_eq!(iter.next(), Some(12), "after space");
    assert_eq!(iter.rule_status(), RuleStatusType::None, "none");
    assert!(!iter.is_word_like(), "None is false");

    assert_eq!(iter.next(), Some(15), "after number");
    assert_eq!(iter.rule_status(), RuleStatusType::Number, "number");
    assert!(iter.is_word_like(), "Number is true");
}

#[test]
fn rule_status_th() {
    let segmenter =
        WordSegmenter::try_new_auto_unstable(&icu_testdata::unstable()).expect("Data exists");
    let mut iter = segmenter.segment_str("ภาษาไทยภาษาไทย");

    assert_eq!(iter.next(), Some(0), "SOT");
    assert_eq!(iter.rule_status(), RuleStatusType::None, "none");
    assert!(!iter.is_word_like(), "SOT is false");

    assert_eq!(iter.next(), Some(12), "after 1st word");
    assert_eq!(iter.rule_status(), RuleStatusType::Letter, "letter");
    assert!(iter.is_word_like(), "Letter(Thai) is true");

    assert_eq!(iter.next(), Some(21), "after 2nd word");
    assert_eq!(iter.rule_status(), RuleStatusType::Letter, "letter");
    assert!(iter.is_word_like(), "Letter(Thai) is true");
}

/* The rule status functions are no longer public to non word break iterators.
#[test]
fn rule_status_no_word() {
    let segmenter =
        SentenceSegmenter::try_new_unstable(&icu_testdata::unstable()).expect("Data exists");
    let mut iter = segmenter.segment_str("hello");

    assert_eq!(iter.next(), Some(0), "SOT");
    assert_eq!(iter.rule_status(), RuleStatusType::None, "none");
    assert!(!iter.is_word_like(), "always false");

    assert_eq!(iter.next(), Some(5), "1st sentence");
    assert_eq!(iter.rule_status(), RuleStatusType::None, "none");
    assert!(!iter.is_word_like(), "always false");
}
*/
