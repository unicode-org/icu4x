// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_pattern::{
    DoublePlaceholderKey, DoublePlaceholderPattern, MultiNamedPlaceholderKey,
    MultiNamedPlaceholderPattern, SinglePlaceholderKey, SinglePlaceholderPattern,
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

    // 4. Adjacent placeholders
    let pattern = DoublePlaceholderPattern::try_from_str("a{0}{1}b", Default::default()).unwrap();
    let matches = pattern.extract_placeholders("axyzb").unwrap();
    assert_eq!(matches.get(DoublePlaceholderKey::Place0), Some(""));
    assert_eq!(matches.get(DoublePlaceholderKey::Place1), Some("xyz"));
}

#[test]
#[cfg(feature = "alloc")]
fn test_multi_named_extraction() {
    let pattern = MultiNamedPlaceholderPattern::try_from_str(
        "Hello, {person0} and {person1}!",
        Default::default(),
    )
    .unwrap();
    let matches = pattern
        .extract_placeholders("Hello, Alice and Bob!")
        .unwrap();
    assert_eq!(
        matches.get(MultiNamedPlaceholderKey("person0")),
        Some("Alice")
    );
    assert_eq!(
        matches.get(MultiNamedPlaceholderKey("person1")),
        Some("Bob")
    );

    assert_eq!(matches.get(MultiNamedPlaceholderKey("person2")), None);
}
