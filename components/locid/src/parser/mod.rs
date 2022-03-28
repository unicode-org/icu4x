// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod errors;
mod langid;
mod locale;

pub use errors::ParserError;
pub use langid::{
    parse_language_identifier, parse_language_identifier_from_iter,
    parse_language_identifier_without_variants, ParserMode,
};
pub use locale::parse_locale;

pub const fn get_subtag_iterator(slice: &[u8]) -> SubtagIterator {
    let mut current_start = 0;
    while current_start < slice.len()
        && (slice[current_start] == b'-' || slice[current_start] == b'_')
    {
        current_start += 1;
    }
    let mut current_end = current_start;
    while current_end < slice.len() && slice[current_end] != b'-' && slice[current_end] != b'_' {
        current_end += 1;
    }
    SubtagIterator {
        slice,
        current_start,
        current_end,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SubtagIterator<'a> {
    slice: &'a [u8],
    current_start: usize,
    current_end: usize,
}

pub type ManualSlice<'a> = (&'a [u8], usize, usize);

impl<'a> SubtagIterator<'a> {
    pub const fn next_manual(mut self) -> (Self, Option<ManualSlice<'a>>) {
        if self.current_start == self.current_end {
            (self, None)
        } else {
            let r = (self.slice, self.current_start, self.current_end);
            self.current_start = self.current_end;
            while self.current_start < self.slice.len()
                && (self.slice[self.current_start] == b'-'
                    || self.slice[self.current_start] == b'_')
            {
                self.current_start += 1;
            }
            self.current_end = self.current_start;
            while self.current_end < self.slice.len()
                && self.slice[self.current_end] != b'-'
                && self.slice[self.current_end] != b'_'
            {
                self.current_end += 1;
            }
            (self, Some(r))
        }
    }

    pub const fn peek_manual(&self) -> Option<ManualSlice<'a>> {
        if self.current_start == self.current_end {
            None
        } else {
            Some((self.slice, self.current_start, self.current_end))
        }
    }

    pub fn peek(&self) -> Option<&'a [u8]> {
        self.peek_manual().map(|(t, s, e)| &t[s..e])
    }
}

impl<'a> Iterator for SubtagIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let (s, res) = self.next_manual();
        self.clone_from(&s);
        res.map(|(t, s, e)| &t[s..e])
    }
}
