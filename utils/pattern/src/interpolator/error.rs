// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{fmt::Debug, str::FromStr};
use thiserror::Error;

/// An error returned when interpolating a pattern.
///
/// # Type parameters
///
/// - `K`: A key for the replacement provider.
#[derive(Error, Debug, PartialEq)]
pub enum InterpolatorError<K>
where
    K: Debug + FromStr + PartialEq,
    K::Err: Debug + PartialEq,
{
    #[error("Invalid placeholder: {0:?}")]
    InvalidPlaceholder(K::Err),
    #[error("Missing placeholder: {0:?}")]
    MissingPlaceholder(K),
    #[error("Unclosed placeholder")]
    UnclosedPlaceholder,
    #[error("Unclosed quoted literal")]
    UnclosedQuotedLiteral,
}
