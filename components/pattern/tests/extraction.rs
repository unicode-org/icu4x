// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_pattern::{
    DoublePlaceholderKey, DoublePlaceholderPattern, SinglePlaceholderKey, SinglePlaceholderPattern,
};

#[test]
fn test_single_extraction() {
    // 1. Basic match
    let pattern = SinglePlaceholderPattern::try_from_str("a{0}b", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("axyzb").unwrap();
    assert_eq!(matches.get(SinglePlaceholderKey::Singleton), Some("xyz"));

    // 2. No match
    assert!(pattern.extract_placeholders("axyz").is_none());
    assert!(pattern.extract_placeholders("xyzb").is_none());

    // 3. Empty prefix/suffix
    let pattern = SinglePlaceholderPattern::try_from_str("{0}", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("hello").unwrap();
    assert_eq!(matches.get(SinglePlaceholderKey::Singleton), Some("hello"));

    // 4. No placeholder, matches
    let pattern = SinglePlaceholderPattern::try_from_str("literal", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("literal").unwrap();
    assert_eq!(matches.get(SinglePlaceholderKey::Singleton), None);

    // 5. No placeholder, no match
    assert!(pattern.extract_placeholders("other").is_none());

    // 6. Empty pattern, empty input
    let pattern = SinglePlaceholderPattern::try_from_str("", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("").unwrap();
    assert_eq!(matches.get(SinglePlaceholderKey::Singleton), None);

    // 7. Empty pattern, non-empty input
    assert!(pattern.extract_placeholders("non-empty").is_none());

    // 8. Only placeholder, empty input
    let pattern = SinglePlaceholderPattern::try_from_str("{0}", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("").unwrap();
    assert_eq!(matches.get(SinglePlaceholderKey::Singleton), Some(""));

    // 9. Prefix and placeholder, empty match
    let pattern = SinglePlaceholderPattern::try_from_str("a{0}", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("a").unwrap();
    assert_eq!(matches.get(SinglePlaceholderKey::Singleton), Some(""));

    // 10. Placeholder and suffix, empty match
    let pattern = SinglePlaceholderPattern::try_from_str("{0}b", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("b").unwrap();
    assert_eq!(matches.get(SinglePlaceholderKey::Singleton), Some(""));
}

#[test]
fn test_double_extraction() {
    // 1. Basic match
    let pattern = DoublePlaceholderPattern::try_from_str("a{0}b{1}c", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("axxbyyc").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), Some("xx"));
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), Some("yy"));

    // 2. Ambiguity (first match / lazy)
    let pattern = DoublePlaceholderPattern::try_from_str("a{0}b{1}b", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("axbybb").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), Some("x"));
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), Some("yb"));

    // 3. Reversed order in pattern
    let pattern = DoublePlaceholderPattern::try_from_str("a{1}b{0}c", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("axxbyyc").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), Some("yy"));
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), Some("xx"));

    // 4. Adjacent placeholders at the end
    let pattern = DoublePlaceholderPattern::try_from_str("a{0}{1}", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("axyz").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), Some(""));
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), Some("xyz"));

    // 5. Adjacent placeholders at the start
    let pattern = DoublePlaceholderPattern::try_from_str("{0}{1}b", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("axyzb").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), Some(""));
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), Some("axyz"));

    // 6. 0 placeholders, matches
    let pattern = DoublePlaceholderPattern::try_from_str("literal", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("literal").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), None);
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), None);

    // 7. 0 placeholders, no match
    assert!(pattern.extract_placeholders("other").is_none());

    // 8. 1 placeholder (Place0)
    let pattern = DoublePlaceholderPattern::try_from_str("a{0}b", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("axyzb").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), Some("xyz"));
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), None);

    // 9. 1 placeholder (Place1)
    let pattern = DoublePlaceholderPattern::try_from_str("{1}b", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("xyzb").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), None);
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), Some("xyz"));

    // 10. Backtracking recovery (first split fails later, second succeeds)
    let pattern = DoublePlaceholderPattern::try_from_str("{0}b{1}c", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("xbybczc").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), Some("x"));
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), Some("ybcz"));
}
