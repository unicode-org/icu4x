// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: Document all exported types in this module
// #![warn(missing_docs)]

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
//! let provider = icu_testdata::get_provider();
//! let segmenter = LineBreakSegmenter::try_new(&provider).expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[6, 11]);
//! ```
//!
//! Segment a string with CSS option overrides:
//!
//! ```rust
//! use icu_segmenter::{LineBreakOptions, LineBreakRule, LineBreakSegmenter, WordBreakRule};
//!
//! let mut options = LineBreakOptions::default();
//! options.line_break_rule = LineBreakRule::Strict;
//! options.word_break_rule = WordBreakRule::BreakAll;
//! options.ja_zh = false;
//! let provider = icu_testdata::get_provider();
//! let segmenter =
//!     LineBreakSegmenter::try_new_with_options(&provider, options).expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[1, 2, 3, 4, 6, 7, 8, 9, 10, 11]);
//! ```
//!
//! Segment a Latin1 byte string:
//!
//! ```rust
//! use icu_segmenter::LineBreakSegmenter;
//!
//! let provider = icu_testdata::get_provider();
//! let segmenter = LineBreakSegmenter::try_new(&provider).expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_latin1(b"Hello World").collect();
//! assert_eq!(&breakpoints, &[6, 11]);
//! ```
//!
//! ## Grapheme Cluster Break
//!
//! Segment a string:
//!
//!```rust
//! use icu_segmenter::GraphemeClusterBreakSegmenter;
//! let provider = icu_testdata::get_provider();
//! let segmenter = GraphemeClusterBreakSegmenter::try_new(&provider).expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_str("Hello 🗺").collect();
//! // World Map (U+1F5FA) is encoded in four bytes in UTF-8.
//! assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 5, 6, 10]);
//! ```
//!
//! Segment a Latin1 byte string:
//!
//! ```rust
//! use icu_segmenter::GraphemeClusterBreakSegmenter;
//! let provider = icu_testdata::get_provider();
//! let segmenter = GraphemeClusterBreakSegmenter::try_new(&provider).expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_latin1(b"Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
//! ```
//!
//! ## Word Break
//!
//! Segment a string:
//!
//!```rust
//! use icu_segmenter::WordBreakSegmenter;
//! let provider = icu_testdata::get_provider();
//! let segmenter = WordBreakSegmenter::try_new(&provider).expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 5, 6, 11]);
//! ```
//!
//! Segment a Latin1 byte string:
//!
//! ```rust
//! use icu_segmenter::WordBreakSegmenter;
//! let provider = icu_testdata::get_provider();
//! let segmenter = WordBreakSegmenter::try_new(&provider).expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_latin1(b"Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 5, 6, 11]);
//! ```
//!
//! ## Sentence Break
//!
//! Segment a string:
//!
//!```rust
//! use icu_segmenter::SentenceBreakSegmenter;
//! let provider = icu_testdata::get_provider();
//! let segmenter = SentenceBreakSegmenter::try_new(&provider).expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 11]);
//! ```
//!
//! Segment a Latin1 byte string:
//!
//! ```rust
//! use icu_segmenter::SentenceBreakSegmenter;
//! let provider = icu_testdata::get_provider();
//! let segmenter = SentenceBreakSegmenter::try_new(&provider).expect("Data exists");
//!
//! let breakpoints: Vec<usize> = segmenter.segment_latin1(b"Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 11]);
//! ```

#![cfg_attr(not(any(test, feature = "std")), no_std)]

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
    GraphemeClusterBreakIteratorLatin1, GraphemeClusterBreakIteratorUtf16,
    GraphemeClusterBreakIteratorUtf8, GraphemeClusterBreakSegmenter,
};
pub use crate::line::{
    LineBreakIterator, LineBreakIteratorLatin1, LineBreakIteratorUtf16, LineBreakIteratorUtf8,
    LineBreakOptions, LineBreakRule, LineBreakSegmenter, WordBreakRule,
};
pub use crate::sentence::{
    SentenceBreakIteratorLatin1, SentenceBreakIteratorUtf16, SentenceBreakIteratorUtf8,
    SentenceBreakSegmenter,
};
pub use crate::word::{
    WordBreakIteratorLatin1, WordBreakIteratorUtf16, WordBreakIteratorUtf8, WordBreakSegmenter,
};
