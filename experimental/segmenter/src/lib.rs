// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Experimental\] Segment strings by lines, graphemes, word, and sentences.
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
//! <div class="stab unstable">
//! 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
//! of the icu meta-crate. Use with caution.
//! <a href="https://github.com/unicode-org/icu4x/issues/2259">#2259</a>
//! </div>
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
//! use icu::segmenter::LineSegmenter;
//!
//! let segmenter = LineSegmenter::try_new_auto_unstable(&icu_testdata::unstable())
//!     .expect("Data exists");
//!
//! let breakpoints: Vec<usize> =
//!     segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[6, 11]);
//! ```
//!
//! See [`LineSegmenter`] for more examples.
//!
//! ## Grapheme Cluster Break
//!
//! See [`GraphemeClusterSegmenter`] for examples.
//!
//! ## Word Break
//!
//! Segment a string:
//!
//!```rust
//! use icu::segmenter::WordSegmenter;
//!
//! let segmenter = WordSegmenter::try_new_auto_unstable(&icu_testdata::unstable())
//!     .expect("Data exists");
//!
//! let breakpoints: Vec<usize> =
//!     segmenter.segment_str("Hello World").collect();
//! assert_eq!(&breakpoints, &[0, 5, 6, 11]);
//! ```
//!
//! See [`WordSegmenter`] for more examples.
//!
//! ## Sentence Break
//!
//! See [`SentenceSegmenter`] for examples.

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod complex;
mod dictionary;
mod error;
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

#[cfg(feature = "lstm")]
mod lstm;
#[cfg(feature = "lstm")]
mod math_helper;

pub use crate::grapheme::{
    GraphemeClusterBreakIteratorLatin1, GraphemeClusterBreakIteratorPotentiallyIllFormedUtf8,
    GraphemeClusterBreakIteratorUtf16, GraphemeClusterBreakIteratorUtf8, GraphemeClusterSegmenter,
};
pub use crate::line::{
    LineBreakIterator, LineBreakIteratorLatin1, LineBreakIteratorPotentiallyIllFormedUtf8,
    LineBreakIteratorUtf16, LineBreakIteratorUtf8, LineBreakOptions, LineBreakRule, LineSegmenter,
    WordBreakRule,
};
pub use crate::sentence::{
    SentenceBreakIteratorLatin1, SentenceBreakIteratorPotentiallyIllFormedUtf8,
    SentenceBreakIteratorUtf16, SentenceBreakIteratorUtf8, SentenceSegmenter,
};
pub use crate::word::{
    WordBreakIteratorLatin1, WordBreakIteratorPotentiallyIllFormedUtf8, WordBreakIteratorUtf16,
    WordBreakIteratorUtf8, WordSegmenter,
};

pub use crate::rule_segmenter::RuleBreakIterator;

pub use crate::rule_segmenter::RuleStatusType;

pub use error::SegmenterError;

#[doc(inline)]
pub use SegmenterError as Error;
