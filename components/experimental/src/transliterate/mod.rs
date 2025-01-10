// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Transliteration
//!
//! See [`Transliterator`].

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations


pub mod provider;

mod compile;
#[allow(clippy::indexing_slicing, clippy::unwrap_used)] // TODO(#3958): Remove.
mod transliterator;

pub use transliterator::{CustomTransliterator, Transliterator};

pub use compile::RuleCollection;
pub use compile::RuleCollectionProvider;
