use super::error::Error;
use super::{Pattern, PatternItem};
use crate::fields::{Field, FieldLength, FieldSymbol};
use std::convert::TryFrom;

enum State {
    Symbol {
        symbol: FieldSymbol,
        byte: u8,
        start_idx: usize,
    },
    Literal {
        start_idx: usize,
    },
}

pub struct Parser<'p> {
    source: &'p [u8],
    state: State,
    items: Vec<PatternItem>,
    ptr: usize,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p [u8]) -> Self {
        Self {
            source,
            state: State::Literal { start_idx: 0 },
            items: Vec::with_capacity(source.len()),
            ptr: 0,
        }
    }

    fn collect_item(&mut self) -> Result<(), Error> {
        match self.state {
            State::Symbol {
                symbol, start_idx, ..
            } => self.collect_symbol(symbol, start_idx)?,
            State::Literal { start_idx } => self.collect_literal(start_idx),
        }
        Ok(())
    }

    fn collect_symbol(&mut self, symbol: FieldSymbol, start_idx: usize) -> Result<(), Error> {
        let len = self.ptr - start_idx;
        let length = FieldLength::try_from(len)?;
        self.items.push(Field { symbol, length }.into());
        Ok(())
    }

    fn collect_literal(&mut self, start_idx: usize) {
        if start_idx < self.ptr {
            let slice = &self.source[start_idx..self.ptr];
            let item =
                PatternItem::Literal(unsafe { String::from_utf8_unchecked(slice.to_owned()) });
            self.items.push(item);
        }
    }

    pub fn parse(mut self) -> Result<Vec<PatternItem>, Error> {
        while let Some(b) = self.source.get(self.ptr) {
            if let State::Symbol { byte, .. } = &self.state {
                if byte == b {
                    self.ptr += 1;
                    continue;
                }
            }

            if let Ok(symbol) = FieldSymbol::try_from(*b) {
                self.collect_item()?;
                self.state = State::Symbol {
                    symbol,
                    byte: *b,
                    start_idx: self.ptr,
                };
            } else if let State::Symbol {
                symbol, start_idx, ..
            } = self.state
            {
                self.collect_symbol(symbol, start_idx)?;
                self.state = State::Literal {
                    start_idx: self.ptr,
                };
            }
            self.ptr += 1;
        }
        self.collect_item()?;
        Ok(self.items)
    }

    pub fn parse_placeholders(
        mut self,
        mut replacements: Vec<Pattern>,
    ) -> Result<Vec<PatternItem>, Error> {
        while let Some(b) = self.source.get(self.ptr) {
            if *b == b'{' {
                if let State::Literal { start_idx } = self.state {
                    self.collect_literal(start_idx);
                }
                self.ptr += 1;
                let b = self.source.get(self.ptr).ok_or(Error::Unknown)?;
                let idx: usize = *b as usize - 48;
                let replacement = replacements
                    .get_mut(idx)
                    .ok_or(Error::UnknownSubstitution(*b))?;
                self.items.append(&mut replacement.0);
                self.ptr += 2;
                self.state = State::Literal {
                    start_idx: self.ptr,
                };
            } else {
                self.ptr += 1;
            }
        }
        self.collect_item()?;
        Ok(self.items)
    }
}
