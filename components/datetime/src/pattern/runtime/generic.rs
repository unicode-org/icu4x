// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::{reference, PatternError},
    super::{GenericPatternItem, PatternItem},
    Pattern,
};
use alloc::vec::Vec;
use core::{fmt, str::FromStr};
use icu_provider::yoke::{self, Yokeable, ZeroCopyFrom};
use zerovec::{ZeroVec, ZeroVecIter};

#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom, serde::Serialize, serde::Deserialize)]
pub struct GenericPattern<'data> {
    #[serde(borrow)]
    pub items: ZeroVec<'data, GenericPatternItem>,
}

impl<'data> GenericPattern<'data> {
    /// The function allows for creation of new DTF pattern from a generic pattern
    /// and replacement patterns.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_datetime::pattern::runtime::{CombinedPattern, Pattern};
    ///
    /// let date: Pattern = "Y-m-d".parse()
    ///         .expect("Failed to parse pattern");
    /// let time: Pattern = "HH:mm".parse()
    ///         .expect("Failed to parse pattern");
    ///
    /// let glue: CombinedPattern = "{1} 'at' {0}".parse()
    ///         .expect("Failed to parse generic pattern");
    /// assert_eq!(
    ///     glue.combined(date, time)
    ///         .expect("Failed to combine patterns")
    ///         .to_string(),
    ///     "Y-m-d 'at' HH:mm"
    /// );
    /// ```
    pub fn combine(self, date: Pattern<'data>, time: Pattern<'data>) -> CombinedPattern<'data> {
        CombinedPattern {
            combined: self,
            date,
            time,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
pub struct CombinedPattern<'data> {
    combined: GenericPattern<'data>,
    date: Pattern<'data>,
    time: Pattern<'data>,
}

impl<'l> IntoIterator for &'l CombinedPattern<'_> {
    type Item = PatternItem;
    type IntoIter = CombinedPatternIterator<'l>;

    fn into_iter(self) -> Self::IntoIter {
        CombinedPatternIterator {
            date: self.date.items.iter(),
            time: self.time.items.iter(),
            items: self.combined.items.iter(),
            state: State::Combined,
        }
    }
}

enum State {
    Combined,
    Date,
    Time,
}

pub struct CombinedPatternIterator<'l> {
    date: ZeroVecIter<'l, PatternItem>,
    time: ZeroVecIter<'l, PatternItem>,
    items: ZeroVecIter<'l, GenericPatternItem>,
    state: State,
}

impl<'l> Iterator for CombinedPatternIterator<'l> {
    type Item = PatternItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::Combined => match self.items.next()? {
                    GenericPatternItem::Date => {
                        self.state = State::Date;
                    }
                    GenericPatternItem::Time => {
                        self.state = State::Time;
                    }
                    GenericPatternItem::Literal(ch) => {
                        return Some(PatternItem::Literal(ch));
                    }
                },
                State::Date => {
                    let item = self.date.next();
                    if item.is_some() {
                        return item;
                    } else {
                        self.state = State::Combined;
                    }
                }
                State::Time => {
                    let item = self.time.next();
                    if item.is_some() {
                        return item;
                    } else {
                        self.state = State::Combined;
                    }
                }
            }
        }
    }
}

impl Default for GenericPattern<'_> {
    fn default() -> Self {
        Self {
            items: ZeroVec::clone_from_slice(&[]),
        }
    }
}

impl From<&reference::GenericPattern> for GenericPattern<'_> {
    fn from(input: &reference::GenericPattern) -> Self {
        Self {
            items: ZeroVec::clone_from_slice(&input.items),
        }
    }
}

impl From<&GenericPattern<'_>> for reference::GenericPattern {
    fn from(input: &GenericPattern<'_>) -> Self {
        Self {
            items: input.items.to_vec(),
        }
    }
}

impl fmt::Display for GenericPattern<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let reference = crate::pattern::reference::GenericPattern::from(self);
        reference.fmt(formatter)
    }
}

impl fmt::Display for CombinedPattern<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use crate::pattern::reference::display::write_buffer;
        use core::fmt::Write;

        let mut buffer = alloc::string::String::new();
        for pattern_item in self.into_iter() {
            match pattern_item {
                PatternItem::Field(field) => {
                    write_buffer(&buffer, formatter)?;
                    buffer.clear();
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        formatter.write_char(ch)?;
                    }
                }
                PatternItem::Literal(ch) => {
                    buffer.push(ch);
                }
            }
        }
        write_buffer(&buffer, formatter)?;
        buffer.clear();
        Ok(())
    }
}

impl FromStr for GenericPattern<'_> {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reference = crate::pattern::reference::GenericPattern::from_str(s)?;
        Ok(Self::from(&reference))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_runtime_generic_pattern_combine() {
        let pattern: GenericPattern = "{0} 'at' {1}"
            .parse()
            .expect("Failed to parse a generic pattern.");

        let date = "Y/m/d".parse().expect("Failed to parse a date pattern.");

        let time = "HH:mm".parse().expect("Failed to parse a time pattern.");

        let combined_pattern = pattern.combine(date, time);

        assert_eq!(combined_pattern.to_string(), "Y/m/d 'at' HH:mm");
    }
}
