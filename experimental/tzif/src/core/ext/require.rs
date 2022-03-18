// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::core::{ParseInput, ParseResult};

impl<T, Item, Source> Require<Item, Source> for T
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
    T: ParseInput<Item, Vec<Item>, Source>,
{
}

/// Extended behavior that is automatically implemented for implementors of [`ParseInput`].
pub trait Require<Item, Source>: ParseInput<Item, Vec<Item>, Source>
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
{
    /// Parses a value using the given parse function and ensures that it matches the given predicate.
    /// Returns the parsed value.
    fn require_next<F: FnOnce(&Item) -> bool>(
        &mut self,
        predicate: F,
    ) -> ParseResult<Item, Source> {
        match self.next() {
            Ok(parsed) if predicate(parsed.value()) => Ok(parsed),
            Ok(parsed) => eyre::bail!(
                "require_next(): false predicate, found `{}`",
                parsed.value()
            ),
            err => err,
        }
    }

    /// Takes `n` values and ensures that they match the given predicate.
    /// Returns a vector of the parsed values.
    fn require_take<F: FnOnce(&Vec<Item>) -> bool>(
        &mut self,
        n: usize,
        predicate: F,
    ) -> ParseResult<Vec<Item>, Source> {
        match self.take(n) {
            Ok(parsed) if predicate(&parsed.value()) => Ok(parsed),
            Ok(parsed) => eyre::bail!(
                "require_take(): false predicate, found {:?}",
                parsed.value()
            ),
            err => err,
        }
    }
}
