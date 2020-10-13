// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::error::Error;
use super::{Pattern, PatternItem};
use crate::fields::{Field, FieldSymbol};
use smallstr::SmallString;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
enum Segment {
    Symbol { symbol: FieldSymbol, byte: u8 },
    Literal,
    QuotedLiteral,
}

#[derive(Debug, PartialEq)]
struct State {
    segment: Segment,
    start_idx: usize,
}

#[derive(Debug, PartialEq)]
pub struct Parser<'p> {
    source: &'p str,
    state: State,
    items: Vec<PatternItem>,
    literal_buffer: SmallString<[u8; 2]>,
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
            literal_buffer: SmallString::new(),
        }
    }

    fn collect_item(&mut self, idx: usize) -> Result<(), Error> {
        match self.state.segment {
            Segment::Symbol { symbol, .. } => self.collect_symbol(symbol, idx),
            Segment::Literal => self.collect_literal(idx),
            Segment::QuotedLiteral => Err(Error::UnclosedLiteral),
        }
    }

    fn collect_symbol(&mut self, symbol: FieldSymbol, idx: usize) -> Result<(), Error> {
        let field: Field = (symbol, idx - self.state.start_idx).try_into()?;
        self.items.push(field.into());
        Ok(())
    }

    fn collect_literal(&mut self, idx: usize) -> Result<(), Error> {
        self.dump_buffer(idx);
        if !self.literal_buffer.is_empty() {
            let item = PatternItem::Literal(self.literal_buffer.drain().collect());
            self.items.push(item);
        }
        Ok(())
    }

    fn dump_buffer(&mut self, idx: usize) {
        if idx > self.state.start_idx {
            let slice = &self.source[self.state.start_idx..idx];
            self.literal_buffer.push_str(slice);
        }
        self.state.start_idx = idx;
    }

    fn quoted_literal<I: Iterator<Item = u8>>(
        &mut self,
        idx: usize,
        bytes: &mut std::iter::Peekable<std::iter::Enumerate<I>>,
    ) {
        self.dump_buffer(idx);
        if let Some(&(_, b'\'')) = bytes.peek() {
            self.literal_buffer.push('\'');
            self.state.start_idx = idx + 2;
            bytes.next();
            return;
        }
        if self.state.segment == Segment::QuotedLiteral {
            self.state.segment = Segment::Literal;
        } else {
            self.state.segment = Segment::QuotedLiteral;
        }
        self.state.start_idx = idx + 1;
    }

    pub fn parse(mut self) -> Result<Vec<PatternItem>, Error> {
        let mut bytes = self.source.bytes().enumerate().peekable();
        while let Some((idx, b)) = bytes.next() {
            match self.state.segment {
                Segment::QuotedLiteral if b != b'\'' => continue,
                Segment::Symbol { byte, .. } if byte == b => continue,
                _ => {}
            }

            if let Ok(symbol) = FieldSymbol::try_from(b) {
                self.collect_item(idx)?;
                self.state = State {
                    segment: Segment::Symbol { symbol, byte: b },
                    start_idx: idx,
                };
            } else if let Segment::Symbol { symbol, .. } = self.state.segment {
                self.collect_symbol(symbol, idx)?;
                self.state = if b == b'\'' {
                    State {
                        segment: Segment::QuotedLiteral,
                        start_idx: idx + 1,
                    }
                } else {
                    State {
                        segment: Segment::Literal,
                        start_idx: idx,
                    }
                };
            } else if b == b'\'' {
                self.quoted_literal(idx, &mut bytes);
            }
        }
        self.collect_item(self.source.len())?;
        Ok(self.items)
    }

    pub fn parse_placeholders(
        mut self,
        mut replacements: Vec<Pattern>,
    ) -> Result<Vec<PatternItem>, Error> {
        let mut bytes = self.source.bytes().enumerate().peekable();
        while let Some((idx, b)) = bytes.next() {
            if b == b'{' {
                match self.state.segment {
                    Segment::Literal => {
                        self.collect_literal(idx)?;
                    }
                    Segment::QuotedLiteral => {
                        return Err(Error::UnclosedLiteral);
                    }
                    _ => {}
                }
                let (_, b) = bytes.next().ok_or(Error::UnclosedPlaceholder)?;
                let replacement = replacements
                    .get_mut(b as usize - 48)
                    .ok_or(Error::UnknownSubstitution(b))?;
                self.items.append(&mut replacement.0);
                let (_, b) = bytes.next().ok_or(Error::UnclosedPlaceholder)?;
                if b != b'}' {
                    return Err(Error::UnclosedPlaceholder);
                }
                self.state = State {
                    segment: Segment::Literal,
                    start_idx: idx + 3,
                };
            } else if b == b'\'' {
                self.quoted_literal(idx, &mut bytes);
            }
        }
        self.collect_item(self.source.len())?;
        Ok(self.items)
    }
}
