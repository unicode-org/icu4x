// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Unicode `MessageFormat` 2 (MF2) — construction, parsing, and formatting of
//! localizable messages per [LDML 46.1 tr35-messageFormat].
//!
//! 🚧 This module is under active development. The public API is not yet
//! stable and will undergo breaking changes. See the design documents at
//! the repository root (`messageformat-v2-research.md`,
//! `messageformat-v2-architecture.md`,
//! `messageformat-v2-implementation-details.md`) for background.
//!
//! # Scope
//!
//! `MessageFormat` 2 is a specification for building dynamic messages that
//! interpolate values, adapt to grammatical number / gender, and can be
//! extended with user-defined formatters. This crate provides:
//!
//! - A parser that turns MF2 syntax into an owned [`ast::Message`].
//! - A validator that enforces data-model invariants and promotes a
//!   [`Message`](ast::Message) to a [`ValidatedMessage`].
//! - A formatter that renders a validated message against an input value map,
//!   applying built-in functions plus caller-provided custom functions.
//! - Structured output via [`MessageFormatter::format_to_parts`].
//! - Bidi isolation and `u:id` / `u:dir` option handling.
//!
//! # Built-in function registry by feature
//!
//! The function names ICU4X registers by default depend on which Cargo
//! features are enabled — [`FunctionRegistry::default_registry`] populates
//! a different set per configuration:
//!
//! | Feature set | Registered functions |
//! | ----------- | -------------------- |
//! | (default) | `:string` |
//! | `compiled_data` | `:string`, `:number`, `:integer`, `:percent`, `:currency`, `:offset` |
//! | `unstable` + `compiled_data` | all of the above plus draft `:unit`, `:date`, `:time`, `:datetime` |
//!
//! `:offset`, `:percent`, `:currency`, and the `:number` / `:integer` family
//! are spec default functions but depend on baked decimal and plural-rule
//! data, so they are gated on `compiled_data`. Builds with
//! `default-features = false` will emit [`FormatError::UnknownFunction`] for
//! them unless the caller registers alternatives via
//! [`MessageFormatterBuilder::function`].
//!
//! # Locale defaults
//!
//! [`MessageFormatterBuilder::locale`] is optional and defaults to `und`
//! (undetermined). Locale-sensitive functions (`:number`, `:integer`,
//! `:percent`, `:currency`, `:date` / `:time` / `:datetime`) will silently
//! produce root-locale output under `und`. Set an explicit locale whenever
//! a localized result is expected.
//!
//! # Quickstart
//!
//! ```
//! use icu::experimental::messageformat::{MessageFormatter, OwnedInputs};
//! use icu::locale::locale;
//!
//! let formatter = MessageFormatter::builder()
//!     .source(".input {$count :integer}\n\
//!              .match $count\n\
//!              0   {{You have no new messages.}}\n\
//!              one {{You have {$count} new message.}}\n\
//!              *   {{You have {$count} new messages.}}")
//!     .locale(locale!("en"))
//!     .build()
//!     .expect("valid MF2 source");
//!
//! let inputs = OwnedInputs::new().with_number("count", 3_i64);
//! let (text, errors) = formatter.format_to_string(&inputs);
//! assert_eq!(text, "You have 3 new messages.");
//! assert!(errors.is_empty());
//! ```
//!
//! [LDML 46.1 tr35-messageFormat]: https://www.unicode.org/reports/tr35/tr35-73/tr35-messageFormat.html

// Sub-module relaxation: matches the policy used by the other experimental
// submodules (see e.g. `personnames/mod.rs`). These allows are dropped when
// the submodule graduates to a top-level `components/messageformat/` crate.
#![allow(
    clippy::panic,
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::expect_used,
    missing_docs
)]

pub mod ast;
pub mod bidi;
pub mod error;
pub mod formatter;
pub mod function;
pub mod input;
pub mod parser;
pub mod parts;
mod resolver;
pub mod selector;
pub mod validator;
pub mod value;

pub use bidi::{BidiIsolation, Direction};
pub use error::{BuildError, FormatError, FunctionError, ParseError, ValidationError};
pub use formatter::{MessageFormatter, MessageFormatterBuilder};
pub use function::{FunctionContext, FunctionHandler, FunctionOptions, FunctionRegistry};
pub use input::{InputValue, InputValues, OwnedInputs};
pub use parts::FormattedPart;
pub use selector::SelectorImpl;
pub use validator::{validate, ValidatedMessage};
pub use value::ResolvedValue;
