// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::FieldSymbol;
use crate::pattern;
use crate::skeleton::SkeletonError;
use icu_provider::prelude::DataError;
use thiserror::Error;

/// A list of possible error outcomes for the [`DateTimeFormat`](crate::DateTimeFormat) struct.
#[derive(Error, Debug)]
pub enum DateTimeFormatError {
    /// An error originating from parsing a pattern.
    #[error(transparent)]
    Pattern(#[from] pattern::Error),
    /// An error originating from the [`Write`](std::fmt::Write) trait.
    #[error(transparent)]
    Format(#[from] std::fmt::Error),
    /// An error originating inside of the [`DataProvider`](icu_provider::DataProvider).
    #[error(transparent)]
    DataProvider(#[from] DataError),
    /// An error originating from a missing field in datetime input.
    /// TODO: How can we return which field was missing?
    #[error("Missing input field")]
    MissingInputField,
    /// An error originating from skeleton matching.
    #[error(transparent)]
    Skeleton(#[from] SkeletonError),
    /// An error originating from an unsupported field in a datetime format.
    #[error("Unsupported field: {0:?}")]
    UnsupportedField(FieldSymbol),
}
