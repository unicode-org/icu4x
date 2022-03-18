// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::map::MapValue;
use crate::core::{ParseInput, ParseResult, Parsed};

impl<T, Item, Source> MaybeParse<Item, Source> for T
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
    T: ParseInput<Item, Vec<Item>, Source>,
{
}

/// Extended behavior that is automatically implemented for implementors of [`ParseInput`].
pub trait MaybeParse<Item, Source>: ParseInput<Item, Vec<Item>, Source>
where
    Item: std::fmt::Debug + std::fmt::Display,
    Source: Clone + ParseInput<Item, Vec<Item>, Source>,
{
    /// Attempts to parse a value given a `parse_fn`.
    ///
    /// Upon success, returns a [`Parsed`] with `Some(value)` with the source input
    /// updated to the location after invoking the `parse_fn`.
    ///
    /// Upon failure, retuns a [`Parsed`] with `None` and the source input at the
    /// same location as prior to invoking the `parse_fn`.
    fn maybe_parse<Value, F: Fn(&mut Source) -> ParseResult<Value, Source>>(
        &mut self,
        parse_fn: F,
    ) -> ParseResult<Option<Value>, Source> {
        let mut source = self.source()?;
        match parse_fn(&mut source) {
            Err(_) => Ok(Parsed::new(None, source)),
            success => success.map_value(|value| Some(value)),
        }
    }
}
