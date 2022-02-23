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

pub const fn get_subtag_iterator(t: &[u8]) -> SubtagIterator {
    let mut l_cursor = 0;
    while l_cursor < t.len() && (t[l_cursor] == b'_' || t[l_cursor] == b'-') {
        l_cursor += 1;
    }
    let mut r_cursor = l_cursor;
    while r_cursor < t.len() && t[r_cursor] != b'_' && t[r_cursor] != b'-' {
        r_cursor += 1;
    }
    SubtagIterator {
        t,
        l_cursor,
        r_cursor,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SubtagIterator<'a> {
    t: &'a [u8],
    l_cursor: usize,
    r_cursor: usize,
}

pub type ManualSlice<'a> = (&'a [u8], usize, usize);

impl<'a> SubtagIterator<'a> {
    pub const fn next_manual(mut self) -> (Self, Option<ManualSlice<'a>>) {
        if self.l_cursor == self.r_cursor {
            (self, None)
        } else {
            let r = (self.t, self.l_cursor, self.r_cursor);
            self.l_cursor = self.r_cursor;
            while self.l_cursor < self.t.len()
                && (self.t[self.l_cursor] == b'_' || self.t[self.l_cursor] == b'-')
            {
                self.l_cursor += 1;
            }
            self.r_cursor = self.l_cursor;
            while self.r_cursor < self.t.len()
                && self.t[self.r_cursor] != b'_'
                && self.t[self.r_cursor] != b'-'
            {
                self.r_cursor += 1;
            }
            (self, Some(r))
        }
    }

    pub const fn peek_manual(&self) -> Option<ManualSlice<'a>> {
        if self.l_cursor == self.r_cursor {
            None
        } else {
            Some((self.t, self.l_cursor, self.r_cursor))
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
