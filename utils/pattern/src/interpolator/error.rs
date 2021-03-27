// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::ParserError;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum InterpolatorError<R>
where
    R: Debug,
{
    #[error("Invalid placeholder: {0:?}")]
    InvalidPlaceholder(R),
    #[error("Missing placeholder: {0}")]
    MissingPlaceholder(String),
    #[error("Unclosed placeholder")]
    UnclosedPlaceholder,
    #[error("Unclosed quoted literal")]
    UnclosedQuotedLiteral,
    #[error("Parser error: {0}")]
    Parser(ParserError<R>),
}

impl<R> From<R> for InterpolatorError<R>
where
    R: Debug,
{
    fn from(input: R) -> Self {
        Self::InvalidPlaceholder(input)
    }
}

impl<R> From<ParserError<R>> for InterpolatorError<R>
where
    R: Debug,
{
    fn from(err: ParserError<R>) -> Self {
        Self::Parser(err)
    }
}
