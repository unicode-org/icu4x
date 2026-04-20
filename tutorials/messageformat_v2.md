# MessageFormat 2 Tutorial

_Author: Alexey Lyakhov_
_Crate: `icu_experimental::messageformat` (experimental, subject to change)_

This tutorial walks through the Rust implementation of Unicode **LDML
MessageFormat** in ICU4X, with normative text in
[TR35 Part 9 — LDML 46.1](https://www.unicode.org/reports/tr35/tr35-73/tr35-messageFormat.html)
and the current edition in
[TR35 Part 9 — LDML 48](https://www.unicode.org/reports/tr35/tr35-76/tr35-messageFormat.html)
(aligned with the JavaScript [`messageformat`](https://www.npmjs.com/package/messageformat) 4.x
stack for default-function naming). The implementation lives in the `icu_experimental` crate while
the API stabilizes; it will graduate to a top-level `icu_messageformat`
crate once the surface settles.

For **TR35 edition differences**, **known implementation gaps** (`:unit`, `:offset`,
`currencySign=accounting` / [icu4x#4677](https://github.com/unicode-org/icu4x/issues/4677)), and
**scope vs JS tooling** (CST, visitor APIs, multi-error validation), see
[`messageformat-tr35-spec-tracking.md`](../messageformat-tr35-spec-tracking.md)
at the repository root.

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

## Conformance vs JavaScript tooling

Passing the vendored **Unicode MessageFormat WG** JSON fixtures (synced via
`cargo make sync-mf2-tests`, run with
`cargo test -p icu_experimental --test messageformat_conformance --all-features`)
is ICU4X’s bar for **spec processor** behavior. The npm
[`messageformat`](https://www.npmjs.com/package/messageformat) package also adds
**tooling** layers (CST with source trivia, rich `visit`, multi-error validation,
`Intl.MessageFormat`-shaped APIs). Those are **optional** parity targets; ICU4X
documents what is in scope in
[`messageformat-tr35-spec-tracking.md` §5](../messageformat-tr35-spec-tracking.md).

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
- `.locale(Locale)` — **required**. `.build()` returns `BuildError::MissingLocale`
  if omitted. Call `.locale_undetermined()` to opt into root (`und`) behavior explicitly.
- `.direction(Direction)` — base directionality for bidi isolation; defaults to LTR.
- `.bidi_isolation(…)` — accepts a `bool`, a `BidiIsolation` value
  (including `BidiIsolation::Custom(Arc<dyn BidiStrategy>)` for BYO strategies), or
  the unit structs `DefaultBidiStrategy` / `NoneBidiStrategy`. Defaults to
  `BidiIsolation::Default`.
- `.functions(FunctionRegistry)` — replace the default registry entirely.
- `.function(name, handler)` — register or override a single function handler.

## 3. Built-in functions

Default registry by feature (same table as the `messageformat` module rustdoc):

| Feature set | Registered functions |
|---|---|
| (default) | `:string` |
| `compiled_data` (default for `icu_experimental`) | `:string`, `:number`, `:integer`, `:percent`, `:currency`, `:offset`, `:math` (alias of `:offset`) |
| `unstable` + `compiled_data` | all of the above plus draft `:unit`, `:date`, `:time`, `:datetime` |

`:string` performs NFC-normalized exact-match selection. `:number` and
`:integer` use CLDR plural rules for the active locale (`select=plural`
default, with `ordinal` and `exact` alternatives). Draft `:date` / `:time` /
`:datetime` use ICU4X datetime formatting when `unstable` is enabled; `:unit`
uses the units formatter.

### Spec nuances and custom handlers

- **`:math`** is the same handler as **`:offset`** (`add` / `subtract` digit-size
  options). For other transforms, register a custom
  [`FunctionHandler`](https://docs.rs/icu_experimental/latest/icu_experimental/messageformat/trait.FunctionHandler.html).
- **`:number` / `:integer` / `:percent`**: `notation`, `compactDisplay`, and
  `numberingSystem` are implemented. Scientific/engineering mantissas omit
  trailing fractional zeros unless **`minimumFractionDigits`** is set; the
  separator is **`E`** by default (or **`e`** with **`scientificNotation=e`**);
  **`scientificNotation=timesSuperscript`** renders **`×10`** with Unicode
  superscript exponents. Non-`standard`
  **`notation` on `:percent`** uses `%` prefix vs suffix inferred from the
  locale’s standard percent sample.
- **`:currency`**: `currencyDisplay=name` / `symbol` / `narrowSymbol` honor
  **`notation`** and **`compactDisplay`** (compact short uses ICU4X compact
  currency formatters; compact long on **name** uses long compact currency;
  compact long on **symbol** stitches a long compact decimal into a sample
  currency layout; scientific/engineering stitch like ECMA-402). `code` /
  `never` still format the amount with the same numeric options. **`currencySign=accounting`**
  uses a locale heuristic: some languages get parentheses around the formatted
  magnitude; others keep a normal signed currency string (see
  `messageformat-tr35-spec-tracking.md` §3). Full CLDR accounting subpatterns are
  still tracked under [icu4x#4677](https://github.com/unicode-org/icu4x/issues/4677).
- **Parity with `Intl.MessageFormat` / npm** (`@formatjs/intl-messageformat`, etc.):
  ICU4X targets the same MF2 option names and ECMA-402 number shapes where data
  exists; differences may remain for edge locales, markup, or options not yet
  modeled in `icu_decimal` / `icu_experimental::dimension`.

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
wrapping). **`u:locale`** (Draft) is parsed and applied as a per-expression
locale override for built-in handlers via [`FunctionContext::locale`].
Invalid `u:locale` values emit a function error and fall back per the spec.

```rust
use icu::experimental::messageformat::{Direction, FormattedPart, MessageFormatter};
use icu::locale::locale;

let formatter = MessageFormatter::builder()
    .source("Begin {$t :string u:dir=rtl} end.")
    .locale(locale!("en"))
    .direction(Direction::Ltr)
    .build()
    .unwrap();
let inputs: &[(&str, &str)] = &[("t", "שלום")];
let (text, _) = formatter.format_to_string(&inputs);
// The placeholder is wrapped with RLI...PDI because its direction differs
// from the LTR base.
assert_eq!(text, "Begin \u{2067}שלום\u{2069} end.");
```

Disable isolation globally with `.bidi_isolation(false)`, or plug in a
custom strategy:

```rust
use std::borrow::Cow;
use std::sync::Arc;
use icu::experimental::messageformat::{
    BidiIsolation, BidiStrategy, Direction, MessageFormatter,
};
use icu::locale::locale;

#[derive(Debug)]
struct Bracket;
impl BidiStrategy for Bracket {
    fn isolate<'a>(
        &'a self,
        _base: Direction,
        _placeholder: Option<Direction>,
        _explicit: bool,
    ) -> (Cow<'a, str>, Cow<'a, str>) {
        (Cow::Borrowed("["), Cow::Borrowed("]"))
    }
}

let formatter = MessageFormatter::builder()
    .source("Hi, {$u :string}!")
    .locale(locale!("en"))
    .bidi_isolation(BidiIsolation::Custom(Arc::new(Bracket)))
    .build()
    .unwrap();
let inputs: &[(&str, &str)] = &[("u", "Ada")];
assert_eq!(formatter.format_to_string(&inputs).0, "Hi, [Ada]!");
```

## 8. Pre-parsing

Parsing is pure; validated messages are reusable across formatters:

```rust
use icu::experimental::messageformat::{ast::Message, MessageFormatter, ValidatedMessage};
use icu::locale::locale;

let validated: ValidatedMessage =
    Message::parse_and_validate("Hello, {$user}!").unwrap();

let formatter = MessageFormatter::builder()
    .message(validated)
    .locale(locale!("en"))
    .build()
    .unwrap();
```

## 9. Conformance

Phases 0–10 of the implementation have been delivered with unit tests
inside each module plus integration tests in
`components/experimental/tests/messageformat/`. A dedicated conformance
runner consumes a pinned snapshot of the
[working-group JSON fixtures](https://github.com/unicode-org/message-format-wg/tree/main/test)
under `components/experimental/tests/messageformat/fixtures/tests/` for
string-output and error-category coverage. The pinned upstream git revision
is recorded in `fixtures/UPSTREAM_SHA` and described in
`fixtures/README.md`.

```sh
cargo test -p icu_experimental --test messageformat_conformance --all-features
```

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
