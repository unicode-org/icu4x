// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) struct Uts35DateTimePatternTokenizer<'a>(pub &'a str);

pub(crate) enum Token<'a> {
    Symbol(&'a str),
    Literal(&'a str),
}

impl<'a> Uts35DateTimePatternTokenizer<'a> {
    pub fn step(&mut self) -> Option<Token<'a>> {
        todo!()
    }
}
