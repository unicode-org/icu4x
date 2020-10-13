// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::error::Error;
use super::{Pattern, PatternItem};
use crate::fields::{Field, FieldSymbol};
use std::convert::{TryFrom, TryInto};

enum Segment {
    Symbol { symbol: FieldSymbol, byte: u8 },
    Literal,
}

struct State {
    segment: Segment,
    start_idx: usize,
}

pub struct Parser<'p> {
    source: &'p str,
    state: State,
    items: Vec<PatternItem>,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Self {
            source,
            state: State {
                segment: Segment::Literal,
                start_idx: 0,
            },
            items: Vec::with_capacity(source.len()),
        }
    }

    fn collect_item(&mut self, idx: usize) -> Result<(), Error> {
        match self.state.segment {
            Segment::Symbol { symbol, .. } => self.collect_symbol(symbol, idx)?,
            Segment::Literal => self.collect_literal(idx),
        }
        Ok(())
    }

    fn collect_symbol(&mut self, symbol: FieldSymbol, idx: usize) -> Result<(), Error> {
        let field: Field = (symbol, idx - self.state.start_idx).try_into()?;
        self.items.push(field.into());
        Ok(())
    }

    fn collect_literal(&mut self, idx: usize) {
        if idx > self.state.start_idx {
            let slice = &self.source[self.state.start_idx..idx];
            let item = PatternItem::Literal(slice.to_string());
            self.items.push(item);
        }
    }

    pub fn parse(mut self) -> Result<Vec<PatternItem>, Error> {
        for (idx, b) in self.source.bytes().enumerate() {
            if let Segment::Symbol { byte, .. } = self.state.segment {
                if b == byte {
                    continue;
                }
            }

            if let Ok(symbol) = FieldSymbol::try_from(b) {
                self.collect_item(idx)?;
                self.state = State {
                    segment: Segment::Symbol { symbol, byte: b },
                    start_idx: idx,
                };
            } else if let Segment::Symbol { symbol, .. } = self.state.segment {
                self.collect_symbol(symbol, idx)?;
                self.state = State {
                    segment: Segment::Literal,
                    start_idx: idx,
                };
            }
        }
        self.collect_item(self.source.len())?;
        Ok(self.items)
    }

    pub fn parse_placeholders(
        mut self,
        mut replacements: Vec<Pattern>,
    ) -> Result<Vec<PatternItem>, Error> {
        let mut bytes = self.source.bytes().enumerate();
        while let Some((idx, b)) = bytes.next() {
            if b == b'{' {
                if let Segment::Literal = self.state.segment {
                    self.collect_literal(idx);
                }
                let (_, b) = bytes.next().ok_or(Error::Unknown)?;
                let replacement = replacements
                    .get_mut(b as usize - 48)
                    .ok_or(Error::UnknownSubstitution(b))?;
                self.items.append(&mut replacement.0);
                bytes.next().ok_or(Error::Unknown)?;
                self.state = State {
                    segment: Segment::Literal,
                    start_idx: idx + 3,
                };
            }
        }
        self.collect_item(self.source.len())?;
        Ok(self.items)
    }
}
