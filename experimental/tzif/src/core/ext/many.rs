// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use eyre::Context;

use crate::core::{ParseInput, ParseResult, Parsed};

use super::{expect::Expect, map::MapValue, require::Require};

impl<T, Item, Source> ParseMany<Item, Source> for T
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
    T: ParseInput<Item, Vec<Item>, Source>,
{
}

/// Extended behavior that is automatically implemented for implementors of [`ParseInput`].
pub trait ParseMany<Item, Source>: ParseInput<Item, Vec<Item>, Source>
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
{
    /// Parses zero or more of the given parse function. Returns a vector of the parsed values.
    fn parse_zero_or_more<Value, F: Fn(&mut Source) -> ParseResult<Value, Source>>(
        &mut self,
        parse_fn: F,
    ) -> ParseResult<Vec<Value>, Source> {
        let mut vec = Vec::new();
        let mut source = self.source()?;
        while let Ok(mut parsed) = parse_fn(&mut source) {
            source = parsed.source();
            vec.push(parsed.into_value());
        }
        Ok(Parsed::new(vec, source))
    }

    /// Parses one or more of the given parse function. Returns vector of the parsed values.
    fn parse_one_or_more<Value, F: Fn(&mut Source) -> ParseResult<Value, Source>>(
        &mut self,
        parse_fn: F,
    ) -> ParseResult<Vec<Value>, Source> {
        let mut vec = Vec::new();
        let mut source = self.source()?;
        let mut result = parse_fn(&mut source);
        while let Ok(mut parsed) = result {
            source = parsed.source();
            vec.push(parsed.into_value());
            result = parse_fn(&mut source);
        }
        if vec.is_empty() {
            return result
                .wrap_err("parse_one_or_more(): expected one or more, but parsed none")
                .map_value(|_| Vec::new());
        }
        Ok(Parsed::new(vec, source))
    }

    /// Parses exactly `n` of the given parse function. Returns a vector of the parsed values.
    fn parse_exactly<Value, F: Fn(&mut Source) -> ParseResult<Value, Source>>(
        &mut self,
        n: usize,
        parse_fn: F,
    ) -> ParseResult<Vec<Value>, Source> {
        let mut vec = Vec::with_capacity(n);
        let mut source = self.source()?;
        for i in 0..n {
            let mut parsed = parse_fn(&mut source).wrap_err_with(|| {
                format!("parse_exactly(): expected `{n}` items, but parsed `{i}` items")
            })?;
            source = parsed.source();
            vec.push(parsed.into_value());
        }
        if vec.len() != n {
            eyre::bail!(
                "parse_exactly(): epxected `{}` items, but parsed `{}` items",
                n,
                vec.len()
            );
        }
        Ok(Parsed::new(vec, source))
    }

    /// Takes items while they match the given predicate.
    /// Returns a vector of the parsed items.
    fn take_while<F: Fn(&Item) -> bool>(&mut self, predicate: F) -> ParseResult<Vec<Item>, Source> {
        let mut source = self.source()?;
        let mut vec = Vec::new();
        while let Ok(parsed) = source.require_next(&predicate) {
            let (value, rest) = parsed.split();
            vec.push(value);
            source = rest;
        }
        Ok(Parsed::new(vec, source))
    }

    /// Skips `n` items ahead in the parse input, dropping them and returning the unit type.
    fn skip(&mut self, n: usize) -> ParseResult<(), Source> {
        let mut source = self.source()?;
        for _ in 0..n {
            source = source.next().source()?;
        }
        Ok(Parsed::new((), source))
    }

    /// Skips the next items in the parse input while each item matches the given predicate.
    /// Drops the skipped items, returning the number of items that were skipped.
    fn skip_while<F: Fn(&Item) -> bool>(&mut self, predicate: F) -> ParseResult<usize, Source> {
        let mut source = self.source()?;
        let mut skipped = 0;
        while let Ok(mut parsed) = source.expect_next(&predicate) {
            skipped += 1;
            source = parsed.source();
        }
        Ok(Parsed::new(skipped, source))
    }
}
