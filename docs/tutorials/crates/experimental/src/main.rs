// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using test data with experimental features.
//! 
//! For more information, see the tutorial [cargo.md](../../cargo.md).

use icu::segmenter::GraphemeClusterSegmenter;

fn main() {
    let segmenter = GraphemeClusterSegmenter::try_new_unstable(
        &icu_testdata::unstable(),
    )
    .expect("Grapheme break data should be present in testdata");
    let breakpoints = segmenter.segment_str("🐻🏴‍☠️🐯").collect::<Vec<usize>>();
    assert_eq!(breakpoints, &[0, 4, 17, 21]);
    println!("{:?}", breakpoints);
}
