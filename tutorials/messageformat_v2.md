# MessageFormat 2 Tutorial

_Author: Alexey Lyakhov_
_Crate: `icu_experimental::messageformat` (experimental, subject to change)_

This tutorial walks through the Rust implementation of the Unicode
[MessageFormat 2 specification](https://www.unicode.org/reports/tr35/tr35-73/tr35-messageFormat.html)
in ICU4X. The implementation lives in the `icu_experimental` crate while
the API stabilizes; it will graduate to a top-level `icu_messageformat`
crate once the surface settles.

## 1. What is MessageFormat 2?

MF2 is a canonical syntax and data model for localizable messages that
interpolate values, pluralize, and adapt to grammatical gender. It is the
successor to ICU MessageFormat 1 and the basis of the proposed ECMA-402
`Intl.MessageFormat` API.

A trivial message:

```text
Hello, {$user}!
```

A message that pluralizes:

```text
.input {$count :integer}
.match $count
0   {{You have no items.}}
one {{You have one item.}}
*   {{You have {$count} items.}}
```

## 2. Building a formatter

```rust
use icu::experimental::messageformat::{MessageFormatter, OwnedInputs};
use icu::locale::locale;

let formatter = MessageFormatter::builder()
    .source("Hello, {$user}!")
    .locale(locale!("en"))
    .build()
    .expect("valid MF2 source");

let inputs = OwnedInputs::new().with_str("user", "Ada");
let (text, _errors) = formatter.format_to_string(&inputs);
assert_eq!(text, "Hello, Ada!");
```

The builder accepts:

- `.source(&str)` — MF2 source text, parsed and validated at `.build()`.
- `.message(ValidatedMessage)` — a pre-parsed, pre-validated message.
- `.locale(Locale)` — defaults to `und`.
- `.direction(Direction)` — base directionality for bidi isolation; defaults to LTR.
- `.bidi_isolation(bool)` — toggle isolate wrapping; defaults to `true`.
- `.functions(FunctionRegistry)` — replace the default registry entirely.
- `.function(name, handler)` — register or override a single function handler.

## 3. Built-in functions

| Function | Status | Feature gate |
|---|---|---|
| `:string` | Stable | none |
| `:number`, `:integer` | Stable | `compiled_data` (default) |
| `:date`, `:time`, `:datetime` | Draft (scaffold only) | `unstable + compiled_data` |

`:string` performs NFC-normalized exact-match selection. `:number` and
`:integer` use CLDR plural rules for the active locale (`select=plural`
default, with `ordinal` and `exact` alternatives).

## 4. Error handling

MF2 distinguishes four error classes. The crate reports them via distinct types:

| Category | Type | Aborts format? |
|---|---|---|
| Syntax | [`ParseError::Syntax`] | yes |
| Data Model | [`ValidationError`] | yes (at build time) |
| Resolution | [`FormatError::UnresolvedVariable`], [`UnknownFunction`], [`BadSelector`] | no |
| Message Function | [`FormatError::FunctionError { error: FunctionError::* }`] | no |

Resolution and function errors trigger fallback substitution per the spec
(`{$name}`, `{|literal|}`, `{:fn}`, `{}`) and are returned in the second
tuple element of `format_to_string` / `format_to_parts`.

## 5. Structured output (`formatToParts`)

```rust
use icu::experimental::messageformat::{FormattedPart, MessageFormatter, OwnedInputs};
use icu::locale::locale;

let formatter = MessageFormatter::builder()
    .source("You have {$count :integer} items.")
    .locale(locale!("en"))
    .build()
    .unwrap();
let (parts, _) = formatter.format_to_parts(&OwnedInputs::new().with_number("count", 42_i64));

// Expect three parts: Text, Expression, Text.
assert!(matches!(&parts[0], FormattedPart::Text { value } if value == "You have "));
assert!(matches!(
    &parts[1],
    FormattedPart::Expression { kind, value, .. } if kind.as_ref() == "integer" && value == "42"
));
```

## 6. Custom functions

```rust
use icu::experimental::messageformat::{
    FunctionContext, FunctionError, FunctionHandler, FunctionOptions, MessageFormatter,
    ResolvedValue,
};

#[derive(Debug)]
struct Shout;

impl FunctionHandler for Shout {
    fn format(
        &self,
        _ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        _options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        let input = operand.ok_or(FunctionError::BadOperand)?;
        Ok(ResolvedValue::new(input.text().to_uppercase()))
    }
}

let formatter = MessageFormatter::builder()
    .source("Hey, {$name :shout}!")
    .function("shout", Shout)
    .build()
    .unwrap();
let (text, _) = formatter.format_to_string(&[("name", "ada")].as_slice());
assert_eq!(text, "Hey, ADA!");
```

Custom functions may also attach a [`SelectorImpl`] to enable `.match`
selection on their return value.

## 7. Bidi isolation and `u:` options

MF2 reserves the `u:` namespace for universal options. This crate honors
`u:id` (propagates into `FormattedPart::Expression.id`) and `u:dir`
(overrides the placeholder's direction, driving LRI / RLI / FSI + PDI
wrapping). `u:locale` is part of the Draft spec and is accepted but
ignored for now.

```rust
use icu::experimental::messageformat::{Direction, FormattedPart, MessageFormatter};

let formatter = MessageFormatter::builder()
    .source("Begin {$t :string u:dir=rtl} end.")
    .direction(Direction::Ltr)
    .build()
    .unwrap();
let inputs: &[(&str, &str)] = &[("t", "שלום")];
let (text, _) = formatter.format_to_string(&inputs);
// The placeholder is wrapped with RLI...PDI because its direction differs
// from the LTR base.
assert_eq!(text, "Begin \u{2067}שלום\u{2069} end.");
```

Disable isolation globally with `.bidi_isolation(false)`.

## 8. Pre-parsing

Parsing is pure; validated messages are reusable across formatters:

```rust
use icu::experimental::messageformat::{ast::Message, MessageFormatter, ValidatedMessage};

let validated: ValidatedMessage =
    Message::parse_and_validate("Hello, {$user}!").unwrap();

let formatter = MessageFormatter::builder()
    .message(validated)
    .build()
    .unwrap();
```

## 9. Conformance

Phases 0–10 of the implementation have been delivered with unit tests
inside each module plus integration tests in
`components/experimental/tests/messageformat/`. A dedicated conformance
runner now consumes a pinned snapshot of the
[working-group JSON fixtures](https://github.com/unicode-org/message-format-wg/tree/main/test)
under `components/experimental/tests/messageformat/fixtures/` for
string-output and error-category coverage.

## 10. Further reading

- Repository-level design notes:
  - `messageformat-v2-research.md`
  - `messageformat-v2-architecture.md`
  - `messageformat-v2-implementation-details.md`
- [Unicode MessageFormat 2 spec](https://www.unicode.org/reports/tr35/tr35-73/tr35-messageFormat.html)
- [Working-group repo](https://github.com/unicode-org/message-format-wg)
- [JavaScript reference: `messageformat` v4](https://www.npmjs.com/package/messageformat)

[`ParseError::Syntax`]: https://docs.rs/icu_experimental/latest/icu_experimental/messageformat/enum.ParseError.html
[`ValidationError`]: https://docs.rs/icu_experimental/latest/icu_experimental/messageformat/enum.ValidationError.html
[`FormatError::UnresolvedVariable`]: https://docs.rs/icu_experimental/latest/icu_experimental/messageformat/enum.FormatError.html
[`UnknownFunction`]: https://docs.rs/icu_experimental/latest/icu_experimental/messageformat/enum.FormatError.html
[`BadSelector`]: https://docs.rs/icu_experimental/latest/icu_experimental/messageformat/enum.FormatError.html
[`FormatError::FunctionError { error: FunctionError::* }`]: https://docs.rs/icu_experimental/latest/icu_experimental/messageformat/enum.FormatError.html
[`SelectorImpl`]: https://docs.rs/icu_experimental/latest/icu_experimental/messageformat/trait.SelectorImpl.html
