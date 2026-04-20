// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Unicode `MessageFormat` 2 (MF2) — construction, parsing, and formatting of
//! localizable messages per Unicode **LDML MessageFormat** ([LDML 46.1 tr35-messageFormat],
//! [LDML 48 tr35-messageFormat]). ICU4X documents **46.1** for continuity with earlier
//! TR35 anchors and **48** for the current default-function layout and interoperability
//! with the JavaScript [`messageformat`](https://www.npmjs.com/package/messageformat) 4.x
//! line (which targets LDML 48). Default handlers follow the **48-style split** (`:percent`,
//! `:currency`, `:offset` as separate functions). See `messageformat-tr35-spec-tracking.md`
//! at the repository root for **46.1 vs 48 deltas**, **tracked gaps** (`:unit`, `:offset`
//! range, `currencySign=accounting` vs [icu4x#4677](https://github.com/unicode-org/icu4x/issues/4677)),
//! and **ecosystem scope** vs JS tooling.
//!
//! 🚧 This module is under active development. The public API is not yet
//! stable and will undergo breaking changes. See the design documents at
//! the repository root (`messageformat-v2-research.md`,
//! `messageformat-v2-architecture.md`,
//! `messageformat-v2-implementation-details.md`,
//! `messageformat-tr35-spec-tracking.md`) for background.
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
//! - Bidi isolation and `u:id` / `u:dir` option handling, including
//!   pluggable [`BidiStrategy`] implementations via
//!   [`BidiIsolation::Custom`].
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
//! | `compiled_data` | `:string`, `:number`, `:integer`, `:percent`, `:currency`, `:offset`, `:math` |
//! | `unstable` + `compiled_data` | all of the above plus draft `:unit`, `:date`, `:time`, `:datetime` |
//!
//! **Conformance configuration:** the vendored Unicode WG JSON suite is run with
//! `--all-features` (see `components/experimental/tests/messageformat/conformance.rs`).
//! Treat **`compiled_data`** (and **`unstable`** for draft functions) as the supported
//! way to obtain a **full** built-in registry; minimal feature sets are for slim
//! dependencies, not a claim of standalone Unicode “processor” completeness.
//!
//! `:offset`, `:math`, `:percent`, `:currency`, and the `:number` / `:integer` family
//! are spec default functions but depend on baked decimal and plural-rule
//! data, so they are gated on `compiled_data`. Builds with
//! `default-features = false` will emit [`FormatError::UnknownFunction`] for
//! them unless the caller registers alternatives via
//! [`MessageFormatterBuilder::function`].
//!
//! # Spec coverage (incremental)
//!
//! **`:math`** is registered as an alias for **`:offset`** (same `add` /
//! `subtract` options), matching the older LDML name before `:offset` became
//! stable.
//!
//! ECMA-402-style **shared options** on `:number`, `:integer`, `:percent`, and
//! `:currency` include **`notation`**, **`compactDisplay`**, and
//! **`numberingSystem`**: compact uses [`icu_decimal::CompactDecimalFormatter`];
//! scientific and engineering default to an ASCII **`E`** between mantissa and
//! exponent (or **`e`** when **`scientificNotation=e`**), with the exponent
//! formatted via `DecimalFormatter` (locale digits and minus sign). The
//! mantissa omits trailing fractional zeros unless **`minimumFractionDigits`**
//! requires them. Optional **`scientificNotation=timesSuperscript`** uses
//! **`×10`** with Unicode superscript digits (and U+207B for negative exponents).
//! `numberingSystem`
//! is merged into the format locale as `-u-nu-`. Non-`standard` **`notation`**
//! on `:percent` uses the same percent-mark **prefix vs suffix** as a sample
//! `notation=standard` format, reusing the locale’s percent character when
//! found (`%`, `٪`, etc.).
//!
//! **`:currency`**: **`currencyDisplay=name`** via
//! [`LongCurrencyFormatter`](crate::dimension::currency::long_formatter::LongCurrencyFormatter);
//! **`symbol`** / **`narrowSymbol`** via [`CurrencyFormatter`](crate::dimension::currency::formatter::CurrencyFormatter);
//! **`code`** / **`never`** as `CODE amount` or digits only. Non-`standard`
//! **`notation`** on **symbol**, **narrowSymbol**, and **name** follows ECMA-402
//! shape (compact short uses [`CompactCurrencyFormatter`](crate::dimension::currency::compact_formatter::CompactCurrencyFormatter);
//! compact long on **name** uses [`LongCompactCurrencyFormatter`](crate::dimension::currency::long_compact_formatter::LongCompactCurrencyFormatter);
//! compact long on **symbol** and scientific/engineering stitch the styled
//! amount into a sample standard currency string). **`currencySign=accounting`**
//! (design doc: `documents/design/messageformat_currency_accounting.md` at repo root)
//! uses **CLDR accounting patterns** from [`CurrencyEssentials`](crate::dimension::provider::currency::essentials::CurrencyEssentials)
//! on the branches documented by the **`cldr_handles_accounting_shell`** inline comment in
//! [`function.rs`](crate::messageformat::function); other display / notation combinations may still
//! use an ASCII **`(...)`** fallback shell until [icu4x#4677](https://github.com/unicode-org/icu4x/issues/4677)
//! extends the same data path everywhere.
//!
//! Draft options such as **`u:locale`** may still evolve with the TR35 text; the
//! resolver applies overrides per the current editor’s draft.
//!
//! # Tracked spec-depth gaps
//!
//! The following behaviors are intentionally incomplete relative to LDML Part 9
//! ([46.1](https://www.unicode.org/reports/tr35/tr35-73/tr35-messageFormat.html),
//! [48](https://www.unicode.org/reports/tr35/tr35-76/tr35-messageFormat.html))
//! until the underlying ICU4X dimension or unit stack grows matching capabilities.
//! When implementing, re-read the current **Default functions** sections in Part 9
//! (stable and draft) — option names and requirements can evolve between TR35 releases.
//!
//! - **`:unit` — mixed-unit `usage` outputs** — Single-unit usage preferences and
//!   conversion are wired for the draft `:unit` handler, but preferences that
//!   select mixed units such as `foot-and-inch` still surface
//!   [`FunctionError::UnsupportedOperation`](error::FunctionError).
//! - **`:offset` arithmetic** — Offset math uses a small-integer fast path; operands
//!   that do not fit that path may surface [`FunctionError::UnsupportedOperation`](error::FunctionError).
//!
//! # Locale (required)
//!
//! [`MessageFormatterBuilder::locale`] must be called — [`build`] returns
//! [`BuildError::MissingLocale`] otherwise. This prevents callers from
//! silently shipping root-locale output from locale-sensitive functions
//! (`:number`, `:integer`, `:percent`, `:currency`, `:date` / `:time` /
//! `:datetime`). If root-locale (`und`) behavior is genuinely desired, opt
//! in explicitly with [`MessageFormatterBuilder::locale_undetermined`].
//!
//! [`build`]: MessageFormatterBuilder::build
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
//! [LDML 48 tr35-messageFormat]: https://www.unicode.org/reports/tr35/tr35-76/tr35-messageFormat.html

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

pub use bidi::{BidiIsolation, BidiStrategy, DefaultBidiStrategy, Direction, NoneBidiStrategy};
pub use error::{BuildError, FormatError, FunctionError, ParseError, ValidationError};
pub use formatter::{MessageFormatter, MessageFormatterBuilder};
pub use function::{FunctionContext, FunctionHandler, FunctionOptions, FunctionRegistry};
pub use input::{InputValue, InputValues, OwnedInputs};
pub use parts::FormattedPart;
pub use selector::SelectorImpl;
pub use validator::{validate, ValidatedMessage};
pub use value::ResolvedValue;
