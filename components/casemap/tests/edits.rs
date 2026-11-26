// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "compiled_data")]

use core::ops::Range;
use icu_casemap::options::{LeadingAdjustment, TitlecaseOptions, TrailingCase};
use icu_casemap::{CaseMapWriteable, CaseMapper, TitlecaseMapper};
use icu_locale_core::langid;

type EditRanges = (Range<usize>, Range<usize>);

fn check(mapping: impl CaseMapWriteable, expected: &str, ranges: &[EditRanges]) {
    let (output, edits) = mapping.to_string_with_edits();
    assert_eq!(output, expected);
    assert_eq!(output, mapping.write_to_string());
    let actual: Vec<_> = edits
        .into_iter()
        .map(|e| (e.source, e.destination))
        .collect();
    assert_eq!(actual, ranges);
}

#[test]
fn changed_ranges() {
    let cm = CaseMapper::new();
    let root = langid!("und");
    check(cm.uppercase("", &root), "", &[]);
    check(cm.uppercase("A😀!", &root), "A😀!", &[]);
    check(
        cm.uppercase("cß!ﬃz", &root),
        "CSS!FFIZ",
        &[(0..1, 0..1), (1..3, 1..3), (4..7, 4..7), (7..8, 7..8)],
    );
    check(
        cm.lowercase("İ Ⱥ𐐀!", &root),
        "i\u{307} ⱥ𐐨!",
        &[(0..2, 0..3), (3..5, 4..7), (5..9, 7..11)],
    );
    check(cm.fold("ẞİ!"), "ssi\u{307}!", &[(0..3, 0..2), (3..5, 2..5)]);
    check(cm.fold_turkic("Iİ!"), "ıi!", &[(0..1, 0..2), (1..3, 2..3)]);
}

#[test]
fn contextual_changes_and_deletions() {
    let cm = CaseMapper::new();
    check(
        cm.lowercase("I\u{307}X", &langid!("tr")),
        "ix",
        &[(0..1, 0..1), (1..3, 1..1), (3..4, 1..2)],
    );
    check(
        cm.lowercase("I\u{301}", &langid!("lt")),
        "i\u{307}\u{301}",
        &[(0..1, 0..3)],
    );
    check(
        cm.lowercase("ΟΣ ΟΣΑ", &langid!("und")),
        "ος οσα",
        &[
            (0..2, 0..2),
            (2..4, 2..4),
            (5..7, 5..7),
            (7..9, 7..9),
            (9..11, 9..11),
        ],
    );
    check(
        cm.uppercase("α\u{301}ι!", &langid!("el")),
        "ΑΙ\u{308}!",
        &[(0..2, 0..2), (2..4, 2..2), (4..6, 2..6)],
    );
    // The eta writes two characters; the following tonos is deleted.
    check(
        cm.uppercase("η\u{301}", &langid!("el")),
        "Η\u{301}",
        &[(0..2, 0..4), (2..4, 4..4)],
    );
}

#[test]
fn greek_computed_replacements() {
    let cm = CaseMapper::new();
    let el = langid!("el");
    // Taking the Greek special-case path does not necessarily change the text.
    for source in ["Α", "Η", "Ι", "Ϊ", "Υ", "Ϋ", "Ρ", "Ή"] {
        check(cm.uppercase(source, &el), source, &[]);
    }
    // The replacement begins with the original uppercase base, but expands it.
    // The whole string is unchanged even though the individual mappings change.
    check(
        cm.uppercase("Η\u{301}", &el),
        "Η\u{301}",
        &[(0..2, 0..4), (2..4, 4..4)],
    );
    // Preserve the order of the base, combining dialytika, and capital iota.
    check(
        cm.uppercase("ᾳ\u{308}z", &el),
        "Α\u{308}ΙZ",
        &[(0..3, 0..6), (3..5, 6..6), (5..6, 6..7)],
    );
    check(
        cm.uppercase("η\u{301}\u{308}", &el),
        "Η\u{301}\u{308}",
        &[(0..2, 0..6), (2..4, 6..6), (4..6, 6..6)],
    );
}

#[test]
fn titlecase_prefix_tail_and_dutch_ij() {
    let cm = CaseMapper::new();
    let tm = TitlecaseMapper::new();
    let root = langid!("und");
    let options = TitlecaseOptions::default();
    check(
        tm.titlecase_segment("«ßABC»,", &root, options),
        "«Ssabc»,",
        &[(2..4, 2..4), (4..5, 4..5), (5..6, 5..6), (6..7, 6..7)],
    );
    check(
        cm.titlecase_segment_with_only_case_data("«ßABC»,", &root, options),
        "«Ssabc»,",
        &[(2..4, 2..4), (4..5, 4..5), (5..6, 5..6), (6..7, 6..7)],
    );
    check(
        tm.titlecase_segment("«😀!»,", &root, options),
        "«😀!»,",
        &[],
    );
    let mut preserve = options;
    preserve.trailing_case = Some(TrailingCase::Unchanged);
    check(
        tm.titlecase_segment("«iJkD»,", &langid!("nl"), preserve),
        "«IJkD»,",
        &[(2..3, 2..3)],
    );
    check(
        tm.titlecase_segment("«ijKD»,", &langid!("nl"), options),
        "«IJkd»,",
        &[(2..3, 2..3), (3..4, 3..4), (4..5, 4..5), (5..6, 5..6)],
    );
    check(
        tm.titlecase_segment("«iABC", &langid!("tr"), preserve),
        "«İABC",
        &[(2..3, 2..4)],
    );
    let mut no_adjust = options;
    no_adjust.leading_adjustment = Some(LeadingAdjustment::None);
    check(
        tm.titlecase_segment("«ABC»,", &root, no_adjust),
        "«abc»,",
        &[(2..3, 2..3), (3..4, 3..4), (4..5, 4..5)],
    );
}

// Validate the edit contract independently of the mapping: unchanged gaps must
// match and replacing the source ranges must reconstruct the entire output.
fn check_reconstruction(source: &str, mapping: impl CaseMapWriteable) {
    let (output, edits) = mapping.to_string_with_edits();
    assert_eq!(output, mapping.write_to_string());
    let mut reconstructed = String::new();
    let (mut source_end, mut destination_end) = (0, 0);
    for edit in edits {
        assert!(edit.source.start >= source_end);
        assert!(edit.destination.start >= destination_end);
        assert_eq!(
            &source[source_end..edit.source.start],
            &output[destination_end..edit.destination.start]
        );
        assert_eq!(source[edit.source.clone()].chars().count(), 1);
        assert_ne!(
            &source[edit.source.clone()],
            &output[edit.destination.clone()]
        );
        reconstructed.push_str(&source[source_end..edit.source.start]);
        reconstructed.push_str(&output[edit.destination.clone()]);
        source_end = edit.source.end;
        destination_end = edit.destination.end;
    }
    assert_eq!(&source[source_end..], &output[destination_end..]);
    reconstructed.push_str(&source[source_end..]);
    assert_eq!(reconstructed, output);
}

#[test]
fn reconstruct_all_mapping_modes() {
    let cm = CaseMapper::new();
    let tm = TitlecaseMapper::new();
    let inputs = [
        "",
        "abc",
        "«123 ijkD»,",
        "«i\u{301}j\u{301}ssel»,",
        "Aß ﬃİẞȺ𐐀😀",
        "I\u{307} I\u{301} ΟΣ ΟΣΑ α\u{301}ι η\u{301}",
        "և Երևանի",
    ];
    for source in inputs {
        check_reconstruction(source, cm.fold(source));
        check_reconstruction(source, cm.fold_turkic(source));
        for locale in [
            langid!("und"),
            langid!("tr"),
            langid!("lt"),
            langid!("el"),
            langid!("nl"),
            langid!("hy"),
        ] {
            check_reconstruction(source, cm.lowercase(source, &locale));
            check_reconstruction(source, cm.uppercase(source, &locale));
            for leading in [
                LeadingAdjustment::Auto,
                LeadingAdjustment::None,
                LeadingAdjustment::ToCased,
            ] {
                for trailing in [TrailingCase::Lower, TrailingCase::Unchanged] {
                    let mut options = TitlecaseOptions::default();
                    options.leading_adjustment = Some(leading);
                    options.trailing_case = Some(trailing);
                    check_reconstruction(source, tm.titlecase_segment(source, &locale, options));
                    check_reconstruction(
                        source,
                        cm.titlecase_segment_with_only_case_data(source, &locale, options),
                    );
                }
            }
        }
    }
}
