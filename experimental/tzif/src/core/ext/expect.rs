// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use eyre::Context;

use crate::core::{ParseInput, ParseResult};

use super::map::{MapSource, MapValue};

impl<T, Item, Source> Expect<Item, Source> for T
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
    T: ParseInput<Item, Vec<Item>, Source>,
{
}

/// Extended behavior that is automatically implemented for implementors of [`ParseInput`].
pub trait Expect<Item, Source>: ParseInput<Item, Vec<Item>, Source>
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
{
    /// Parses a value using the given parse function and ensures that it matches the given predicate.
    /// Then drops the parsed value, returning the unit type.
    fn expect_next<F: FnOnce(&Item) -> bool>(&mut self, predicate: F) -> ParseResult<(), Source> {
        match self.next() {
            Ok(parsed) if predicate(parsed.value()) => Ok(parsed.map_value(|_| ())),
            Ok(parsed) => eyre::bail!("expect_next(): false predicate, found `{}`", parsed.value()),
            Err(err) => Err(err),
        }
    }

    /// Retrieves the next `Item` if possible but does not advance the `Source`.
    fn inspect_next(&mut self) -> ParseResult<Item, Source> {
        let source = self.source()?;
        match self.next() {
            Ok(parsed) => Ok(parsed.map_source(|_| source)),
            err => err.wrap_err("inspect_next(): failed to inspect next"),
        }
    }
}
