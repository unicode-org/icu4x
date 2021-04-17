// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::interpolator::InterpolatorError;
use std::{fmt::Debug, str::FromStr};
use thiserror::Error;

/// An error returned from a pattern.
///
/// # Type parameters
///
/// - `K`: A key for the replacement provider.
#[derive(Error, Debug, PartialEq)]
pub enum PatternError<K>
where
    K: Debug + FromStr + PartialEq,
    K::Err: Debug + PartialEq,
{
    #[error("Interpolator error: {0:?}")]
    Interpolator(InterpolatorError<K>),
    #[error("Format error: {0:?}")]
    Format(std::fmt::Error),
}

impl<K> From<InterpolatorError<K>> for PatternError<K>
where
    K: Debug + FromStr + PartialEq,
    K::Err: Debug + PartialEq,
{
    fn from(err: InterpolatorError<K>) -> Self {
        Self::Interpolator(err)
    }
}

impl<K> From<std::fmt::Error> for PatternError<K>
where
    K: Debug + FromStr + PartialEq,
    K::Err: Debug + PartialEq,
{
    fn from(err: std::fmt::Error) -> Self {
        Self::Format(err)
    }
}
