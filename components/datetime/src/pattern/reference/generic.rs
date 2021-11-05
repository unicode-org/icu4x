// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::PatternError,
    super::{GenericPatternItem, PatternItem},
    Parser, Pattern,
};
use alloc::vec::Vec;
use core::str::FromStr;

pub struct GenericPattern {
    pub items: Vec<GenericPatternItem>,
}

impl GenericPattern {
    pub fn combine<'l>(
        &'l self,
        date: &'l Pattern,
        time: &'l Pattern,
    ) -> GenericPatternIterator<'l> {
        GenericPatternIterator {
            date: date.items.iter(),
            time: time.items.iter(),
            items: self.items.iter(),
            state: State::Generic,
        }
    }
}

enum State {
    Generic,
    Date,
    Time,
}

pub struct GenericPatternIterator<'l> {
    date: core::slice::Iter<'l, PatternItem>,
    time: core::slice::Iter<'l, PatternItem>,
    items: core::slice::Iter<'l, GenericPatternItem>,
    state: State,
}

impl<'l> Iterator for GenericPatternIterator<'l> {
    type Item = PatternItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::Generic => match self.items.next()? {
                    GenericPatternItem::Date => {
                        self.state = State::Date;
                    }
                    GenericPatternItem::Time => {
                        self.state = State::Time;
                    }
                    GenericPatternItem::Literal(ch) => {
                        return Some(PatternItem::Literal(*ch));
                    }
                },
                State::Date => {
                    let item = self.date.next();
                    if item.is_some() {
                        return item.copied();
                    } else {
                        self.state = State::Generic;
                    }
                }
                State::Time => {
                    let item = self.time.next();
                    if item.is_some() {
                        return item.copied();
                    } else {
                        self.state = State::Generic;
                    }
                }
            }
        }
    }
}

impl From<Vec<GenericPatternItem>> for GenericPattern {
    fn from(items: Vec<GenericPatternItem>) -> Self {
        Self { items }
    }
}

impl FromStr for GenericPattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s).parse_generic().map(Self::from)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reference_generic_pattern_combine() {
        let pattern: GenericPattern = "{0} 'at' {1}"
            .parse()
            .expect("Failed to parse a generic pattern.");

        let date = "Y/m/d".parse().expect("Failed to parse a date pattern.");
        let time = "HH:mm".parse().expect("Failed to parse a time pattern.");

        let pattern = pattern.combine(&date, &time);
        assert_eq!(pattern.into_string(), "Y/m/d 'at' HH:mm");
    }
}
