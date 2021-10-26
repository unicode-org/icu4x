// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![warn(missing_docs)]

//! A segmenter implementation for the following rules.
//!
//! - Line breaker that is compatible with [Unicode Standard Annex #14][UAX14] and CSS properties.
//! - Word breaker that is compatible with [Unicode Standard Annex #29][UAX29].
//!
//! [UAX14]: http://www.unicode.org/reports/tr14/
//! [UAX29]: http://www.unicode.org/reports/tr29/
//!
//! # Line breaker
//!
//!```rust
//! use icu_segmenter::LineBreakIterator;
//!
//! let mut iter = LineBreakIterator::new("Hello World");
//! let result: Vec<usize> = iter.collect();
//! println!("{:?}", result);
//! ```
//!
//! With CSS property.
//! ```rust
//! use icu_segmenter::{LineBreakIterator, LineBreakRule, WordBreakRule};
//!
//! let iter = LineBreakIterator::new_with_break_rule(
//!     "Hello World",
//!     LineBreakRule::Strict,
//!     WordBreakRule::BreakAll,
//!     false,
//! );
//! let result: Vec<usize> = iter.collect();
//! println!("{:?}", result);
//! ```
//!
//! Use Latin 1 string for C binding and etc
//!
//! ```rust
//! use icu_segmenter::LineBreakIteratorLatin1;
//!
//! let s = "Hello World";
//! let iter = LineBreakIteratorLatin1::new(s.as_bytes());
//! let result: Vec<usize> = iter.collect();
//! println!("{:?}", result);
//! ```
//!
//! # word breaker
//!
//!```rust
//! use icu_segmenter::WordBreakIterator;
//!
//! let mut iter = WordBreakIterator::new("Hello World");
//! let result: Vec<usize> = iter.collect();
//! println!("{:?}", result);
//! ```
//!
//! Use Latin 1 string for C binding and etc.
//!
//! ```rust
//! use icu_segmenter::WordBreakIteratorLatin1;
//!
//! let s = "Hello World";
//! let iter = WordBreakIteratorLatin1::new(s.as_bytes());
//! let result: Vec<usize> = iter.collect();
//! println!("{:?}", result);
//! ```
//!
//! # Generating property table
//!
//! Copy the following files to `tools` directory. Then run `./generate_properties.py` in `tools` directory (requires Python 3.8+). Machine generated files are moved to `src` directory.
//! - <https://www.unicode.org/Public/UCD/latest/ucd/LineBreak.txt>
//! - <https://www.unicode.org/Public/UCD/latest/ucd/EastAsianWidth.txt>

mod indices;
mod language;
mod lb_define;
mod line_breaker;
mod lstm;
mod properties_defines;
mod properties_other;
mod property_table;
mod rule_segmenter;
mod rule_table;

mod word;

#[macro_use]
extern crate lazy_static;

pub use crate::line_breaker::*;
pub use crate::word::{WordBreakIterator, WordBreakIteratorLatin1, WordBreakIteratorUtf16};
