// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::core::{ParseResult, Parsed};

use super::map::MapValue;

/// A trait to map only the source of a [`Parsed`] or [`ParseResult`], propagating the value.
pub trait OrDefaultTo {
    /// The type of the inner value within an [`Option`]
    type Inner;
    /// The type of the default value to output.
    type Output;

    /// Default to a given value if the current [`Option`] is [`None`].
    fn or_default_to(self, default: Self::Inner) -> Self::Output;
}

impl<Inner, Source: Clone> OrDefaultTo for Parsed<Option<Inner>, Source> {
    type Inner = Inner;
    type Output = Parsed<Inner, Source>;
    fn or_default_to(self, default: Self::Inner) -> Self::Output {
        self.map_value(|value| match value {
            Some(inner) => inner,
            None => default,
        })
    }
}

impl<Inner, Source: Clone> OrDefaultTo for ParseResult<Option<Inner>, Source> {
    type Inner = Inner;
    type Output = ParseResult<Inner, Source>;
    fn or_default_to(self, default: Self::Inner) -> Self::Output {
        self.map(|parsed| parsed.or_default_to(default))
    }
}
