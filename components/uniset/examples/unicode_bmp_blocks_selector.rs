// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// An example application which uses icu_uniset to test what blocks of
// Basic Multilingual Plane a character belongs to.
//
// In this example we use `UnicodeSetBuilder` to construct just the first
// two blocks of the first plane, and use an instance of a `BMPBlockSelector`
// to retrieve which of those blocks each character of a string belongs to.
//
// This is a simple example of the API use and is severely oversimplified
// compared to real Unicode block selection.
use icu_uniset::{UnicodeSet, UnicodeSetBuilder};

fn get_basic_latin_block() -> UnicodeSet {
    let mut builder = UnicodeSetBuilder::new();
    builder.add_range(&('\u{0000}'..='\u{007F}'));
    builder.build()
}

fn get_latin1_supplement_block() -> UnicodeSet {
    let mut builder = UnicodeSetBuilder::new();
    builder.add_range(&('\u{0080}'..='\u{00FF}'));
    builder.build()
}

#[derive(Copy, Clone, Debug)]
enum BMPBlock {
    Basic,
    Latin1Supplement,
    Unknown,
}

struct BMPBlockSelector {
    blocks: Vec<(BMPBlock, UnicodeSet)>,
}

impl BMPBlockSelector {
    pub fn new() -> Self {
        let blocks = vec![
            (BMPBlock::Basic, get_basic_latin_block()),
            (BMPBlock::Latin1Supplement, get_latin1_supplement_block()),
        ];
        Self { blocks }
    }

    pub fn select(&self, input: char) -> BMPBlock {
        for (block, set) in &self.blocks {
            if set.contains(input) {
                return *block;
            }
        }
        BMPBlock::Unknown
    }
}

fn print(_input: &str) {
    #[cfg(debug_assertions)]
    println!("{}", _input);
}

fn main() {
    let selector = BMPBlockSelector::new();

    let sample = "Welcome to MyName©®, Алексей!";

    let mut result = vec![];

    for ch in sample.chars() {
        result.push((ch, selector.select(ch)));
    }

    print("\n====== Unicode BMP Block Selector example ============");
    for (ch, block) in result {
        print(&format!("{}: {:#?}", ch, block));
    }
}
