// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! \[Experimental\] Segment strings by lines, graphemes, word, and sentences.
//!
//! This module is published as its own crate ([`icu_segmenter`](https://docs.rs/icu_segmenter/latest/icu_segmenter/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! This module contains segmenter implementation for the following rules.
//!
//! - Line breaker that is compatible with [Unicode Standard Annex #14][UAX14] and CSS properties.
//! - Grapheme cluster breaker, word breaker, and sentence breaker that are compatible with
//!   [Unicode Standard Annex #29][UAX29].
//!
//! [UAX14]: https://www.unicode.org/reports/tr14/
//! [UAX29]: https://www.unicode.org/reports/tr29/
//!
//! # Examples
//!
//! ## Line Break
//!
//! Segment a string with default options:
//!
//!```rust
//! use icu_segmenter::LineBreakSegmenter;
//!
//! let segmenter = LineBreakSegmenter::try_new(&icu_testdata::unstable())
//!     .expect("Data exists");
//!
//! let breakpoints: Vec<usize> =
//!     segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[6, 11]);
//! ```
//!
//! See [`LineBreakSegmenter`] for more examples.
//!
//! ## Grapheme Cluster Break
//!
//! See [`GraphemeClusterBreakSegmenter`] for examples.
//!
//! ## Word Break
//!
//! Segment a string:
//!
//!```rust
//! use icu_segmenter::WordBreakSegmenter;
//!
//! let segmenter = WordBreakSegmenter::try_new(&icu_testdata::unstable())
//!     .expect("Data exists");
//!
//! let breakpoints: Vec<usize> =
//!     segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 5, 6, 11]);
//! ```
//!
//! See [`WordBreakSegmenter`] for more examples.
//!
//! ## Sentence Break
//!
//! See [`SentenceBreakSegmenter`] for examples.

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![warn(missing_docs)]

extern crate alloc;

mod complex;
mod dictionary;
mod indices;
mod language;
mod rule_segmenter;

mod grapheme;
mod line;
mod sentence;
mod word;

pub mod provider;

// icu_datagen uses symbols, but we don't want to expose this implementation detail to the users.
#[doc(hidden)]
pub mod symbols;

// Use the LSTM when the feature is enabled.
#[cfg(feature = "lstm")]
mod lstm;
#[cfg(feature = "lstm")]
mod lstm_bies;
#[cfg(feature = "lstm")]
mod lstm_error;
#[cfg(feature = "lstm")]
mod math_helper;

pub use crate::grapheme::{
    GraphemeClusterBreakIteratorLatin1, GraphemeClusterBreakIteratorPotentiallyInvalidUtf8,
    GraphemeClusterBreakIteratorUtf16, GraphemeClusterBreakIteratorUtf8,
    GraphemeClusterBreakSegmenter,
};
pub use crate::line::{
    LineBreakIterator, LineBreakIteratorLatin1, LineBreakIteratorPotentiallyInvalidUtf8,
    LineBreakIteratorUtf16, LineBreakIteratorUtf8, LineBreakOptions, LineBreakRule,
    LineBreakSegmenter, WordBreakRule,
};
pub use crate::sentence::{
    SentenceBreakIteratorLatin1, SentenceBreakIteratorPotentiallyInvalidUtf8,
    SentenceBreakIteratorUtf16, SentenceBreakIteratorUtf8, SentenceBreakSegmenter,
};
pub use crate::word::{
    WordBreakIteratorLatin1, WordBreakIteratorPotentiallyInvalidUtf8, WordBreakIteratorUtf16,
    WordBreakIteratorUtf8, WordBreakSegmenter,
};

pub use crate::rule_segmenter::RuleBreakIterator;
