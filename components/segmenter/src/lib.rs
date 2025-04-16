// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Segment strings by lines, graphemes, words, and sentences.
//!
//! This module is published as its own crate ([`icu_segmenter`](https://docs.rs/icu_segmenter/latest/icu_segmenter/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! This module contains segmenter implementation for the following rules.
//!
//! - Line segmenter that is compatible with [Unicode Standard Annex #14][UAX14], _Unicode Line
//!   Breaking Algorithm_, with options to tailor line-breaking behavior for CSS [`line-break`] and
//!   [`word-break`] properties.
//! - Grapheme cluster segmenter, word segmenter, and sentence segmenter that are compatible with
//!   [Unicode Standard Annex #29][UAX29], _Unicode Text Segmentation_.
//!
//! [UAX14]: https://www.unicode.org/reports/tr14/
//! [UAX29]: https://www.unicode.org/reports/tr29/
//! [`line-break`]: https://drafts.csswg.org/css-text-3/#line-break-property
//! [`word-break`]: https://drafts.csswg.org/css-text-3/#word-break-property
//!
//! # Examples
//!
//! ## Line Break
//!
//! Find line break opportunities:
//!
//!```rust
//! use icu::segmenter::LineSegmenter;
//!
//! let segmenter = LineSegmenter::new_auto(Default::default());
//!
//! let breakpoints: Vec<usize> = segmenter
//!     .segment_str("Hello World. Xin chào thế giới!")
//!     .collect();
//! assert_eq!(&breakpoints, &[0, 6, 13, 17, 23, 29, 36]);
//! ```
//!
//! See [`LineSegmenter`] for more examples.
//!
//! ## Grapheme Cluster Break
//!
//! Find all grapheme cluster boundaries:
//!
//!```rust
//! use icu::segmenter::GraphemeClusterSegmenter;
//!
//! let segmenter = GraphemeClusterSegmenter::new();
//!
//! let breakpoints: Vec<usize> = segmenter
//!     .segment_str("Hello World. Xin chào thế giới!")
//!     .collect();
//! assert_eq!(
//!     &breakpoints,
//!     &[
//!         0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
//!         19, 21, 22, 23, 24, 25, 28, 29, 30, 31, 34, 35, 36
//!     ]
//! );
//! ```
//!
//! See [`GraphemeClusterSegmenter`] for more examples.
//!
//! ## Word Break
//!
//! Find all word boundaries:
//!
//!```rust
//! use icu::segmenter::{options::WordBreakInvariantOptions, WordSegmenter};
//!
//! let segmenter =
//!     WordSegmenter::new_auto(WordBreakInvariantOptions::default());
//!
//! let breakpoints: Vec<usize> = segmenter
//!     .segment_str("Hello World. Xin chào thế giới!")
//!     .collect();
//! assert_eq!(
//!     &breakpoints,
//!     &[0, 5, 6, 11, 12, 13, 16, 17, 22, 23, 28, 29, 35, 36]
//! );
//! ```
//!
//! See [`WordSegmenter`] for more examples.
//!
//! ## Sentence Break
//!
//! Segment the string into sentences:
//!
//!```rust
//! use icu::segmenter::{
//!     options::SentenceBreakInvariantOptions, SentenceSegmenter,
//! };
//!
//! let segmenter =
//!     SentenceSegmenter::new(SentenceBreakInvariantOptions::default());
//!
//! let breakpoints: Vec<usize> = segmenter
//!     .segment_str("Hello World. Xin chào thế giới!")
//!     .collect();
//! assert_eq!(&breakpoints, &[0, 13, 36]);
//! ```
//!
//! See [`SentenceSegmenter`] for more examples.

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        clippy::trivially_copy_pass_by_ref,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod complex;
mod indices;
mod iterator_helpers;
mod rule_segmenter;

/// [`GraphemeClusterSegmenter`] and its related iterators, borrowed types, and options.
mod grapheme;
/// [`LineSegmenter`] and its related iterators, borrowed types, and options.
mod line;
/// [`SentenceSegmenter`] and its related iterators, borrowed types, and options.
mod sentence;
/// [`WordSegmenter`] and its related iterators, borrowed types, and options.
mod word;

pub mod provider;

// Main Segmenter and BreakIterator public types
pub use crate::grapheme::GraphemeClusterSegmenter;
pub use crate::grapheme::GraphemeClusterSegmenterBorrowed;
pub use crate::line::LineSegmenter;
pub use crate::line::LineSegmenterBorrowed;
pub use crate::sentence::SentenceSegmenter;
pub use crate::sentence::SentenceSegmenterBorrowed;
pub use crate::word::WordSegmenter;
pub use crate::word::WordSegmenterBorrowed;

/// Options structs and enums
pub mod options {
    pub use crate::line::LineBreakOptions;
    pub use crate::line::LineBreakStrictness;
    pub use crate::line::LineBreakWordOption;
    pub use crate::sentence::SentenceBreakInvariantOptions;
    pub use crate::sentence::SentenceBreakOptions;
    pub use crate::word::WordBreakInvariantOptions;
    pub use crate::word::WordBreakOptions;
    pub use crate::word::WordType;
}

/// Largely-internal scaffolding types (You should very rarely need to reference these directly)
pub mod scaffold {
    pub use crate::line::LineBreakType;
    pub use crate::rule_segmenter::{Latin1, PotentiallyIllFormedUtf8, RuleBreakType, Utf16, Utf8};
    pub use crate::word::WordBreakType;
}

/// Types supporting iteration over segments. Obtained from the segmenter types.
pub mod iterators {
    pub use crate::grapheme::GraphemeClusterBreakIterator;
    pub use crate::line::LineBreakIterator;
    pub use crate::sentence::SentenceBreakIterator;
    pub use crate::word::{WordBreakIterator, WordBreakIteratorWithWordType};
}

pub(crate) mod private {
    /// Trait marking other traits that are considered unstable and should not generally be
    /// implemented outside of the segmenter crate.
    pub trait Sealed {}
}
