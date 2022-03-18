// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use eyre::Context;

use crate::core::{ParseInput, ParseResult};

use super::map::MapSource;

impl<T, Item, Source> Inspect<Item, Source> for T
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
    T: ParseInput<Item, Vec<Item>, Source>,
{
}

/// Extended behavior that is automatically implemented for implementors of [`ParseInput`].
pub trait Inspect<Item, Source>: ParseInput<Item, Vec<Item>, Source>
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
{
    /// Retrieves the next `Item` if possible but does not advance the `Source`.
    fn inspect_next(&mut self) -> ParseResult<Item, Source> {
        let source = self.source()?;
        match self.next() {
            Ok(parsed) => Ok(parsed.map_source(|_| source)),
            err => err.wrap_err("inspect_next(): failed to inspect next"),
        }
    }
}
