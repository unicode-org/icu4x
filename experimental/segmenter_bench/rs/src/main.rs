// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::io::BufRead;

fn main() {
    let provider = icu_testdata::get_static_provider();

    eprintln!("Welcome! Please send text to stdin:");

    let mut text = Vec::<u16>::new();
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        text.extend(line.unwrap().encode_utf16());
    }

    eprintln!("Segmenting string with {} UTF-16 code units", text.len());

    let segmenter = icu_segmenter::WordBreakSegmenter::try_new(&provider).unwrap();

    for _ in 0..10000 {
        let mut brkitr = segmenter.segment_utf16(&text);
        while brkitr.next().is_some() {}
    }
}
