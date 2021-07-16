// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use std::{fmt::Debug, str::FromStr};

/// An error returned when interpolating a pattern.
///
/// # Type parameters
///
/// - `K`: A key for the replacement provider.
#[derive(Display, Debug, PartialEq)]
pub enum InterpolatorError<K>
where
    K: Debug + FromStr + PartialEq,
    K::Err: Debug + PartialEq,
{
    #[displaydoc("Invalid placeholder: {0:?}")]
    InvalidPlaceholder(K::Err),
    #[displaydoc("Missing placeholder: {0:?}")]
    MissingPlaceholder(K),
    #[displaydoc("Unclosed placeholder")]
    UnclosedPlaceholder,
    #[displaydoc("Unclosed quoted literal")]
    UnclosedQuotedLiteral,
}

impl<K> std::error::Error for InterpolatorError<K>
where
    K: Debug + FromStr + PartialEq,
    K::Err: Debug + PartialEq,
{
}
