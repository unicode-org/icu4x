# MessageFormat 2 — TR35 spec tracking and implementation gaps

This document tracks **LDML MessageFormat** evolution relative to ICU4X’s
[`icu_experimental::messageformat`](components/experimental/src/messageformat/mod.rs)
implementation, maps work to source files, and records **scope decisions** for
parity with the JavaScript [`messageformat`](https://www.npmjs.com/package/messageformat)
ecosystem (not normative for Unicode conformance).

Normative references:

- **LDML 46.1** — [tr35-messageFormat, tr35-73](https://www.unicode.org/reports/tr35/tr35-73/tr35-messageFormat.html)  
  ICU4X module docs currently cite this as the primary alignment baseline.
- **LDML 48** — [tr35-messageFormat, tr35-76](https://www.unicode.org/reports/tr35/tr35-76/tr35-messageFormat.html)  
  The `messageformat` 4.x npm package documents alignment with this edition.

---

## 1. LDML 46.1 (tr35-73) vs 48 (tr35-76) — structural deltas

Compared HTML TOCs and body structure (Unicode-published reports):

| Topic | LDML 46.1 (tr35-73) | LDML 48 (tr35-76) | ICU4X note |
| --- | --- | --- | --- |
| **Default function layout** | Numeric formatting emphasizes `:number` / `:integer` with options such as `style=percent` and `style=currency` (see “Percent Style”, currency option trees under `:number`). | Dedicated sections for **`:offset`**, **`:currency`**, **`:percent`**, and **`:unit`** under “Default Functions”, each with operands / options / selection / formatting. | Handlers match the **48-style split** (`:percent`, `:currency`, `:offset` are separate in [`function.rs`](components/experimental/src/messageformat/function.rs)); `:math` is registered as an alias for `:offset` per older naming. |
| **Message function errors** | Bad Operand, Bad Option, Bad Variant Key (no separate “Unsupported Operation” in the same form). | Adds **Unsupported Operation** as a normative error category for handlers that cannot support a valid option/operand combination. | [`FunctionError::UnsupportedOperation`](components/experimental/src/messageformat/error.rs) is used throughout draft and edge paths. |
| **Document size / detail** | Shorter single-file narrative. | Larger spec (more examples, option tables, cross-links). | Re-audit **REQUIRED** vs **RECOMMENDED** options when bumping the cited LDML version. |

**Action for maintainers:** When intentionally targeting LDML 48, diff the “Default
Functions” sections for each of `:string`, `:number`, `:integer`, `:offset`,
`:currency`, `:percent`, `:unit`, `:datetime`, `:date`, `:time` and reconcile
option names, defaults, and error rules with
[`components/experimental/src/messageformat/function.rs`](components/experimental/src/messageformat/function.rs).

### 1a. LDML 48 default functions — checklist vs `function.rs` (maintenance audit)

Snapshot of **where** each TR35 default function lives and what to re-check when
bumping the cited LDML edition. This is not a substitute for reading the
current spec text; it maps **handlers** to the primary implementation entry points.

| Spec function | Rust handler (`function.rs`) | Options / behavior to re-diff against TR35 |
| --- | --- | --- |
| `:string` | `StringHandler` | Operand typing; NFC string selection; no extra function options. |
| `:number` | `NumberHandler` `{ kind: Number }` | `NumberOptions::parse` (notation, rounding, `numberingSystem`, `select`, …); plural / ordinal / exact selection via `NumberSelector`. |
| `:integer` | `NumberHandler` `{ kind: Integer }` | Same option bag as `:number` with integer digit defaults. |
| `:percent` | `NumberHandler` `{ kind: Percent }` | Scaled percent value; non-standard `notation` uses `format_percent_styled`. |
| `:currency` | `CurrencyHandler` | `currency`, `currencyDisplay`, `currencySign`, `fractionDigits`, plus shared number options; accounting sign: see §3 and [#4677](https://github.com/unicode-org/icu4x/issues/4677). |
| `:offset` | `OffsetHandler` | Exactly one of `add` / `subtract` (`digit-size-option`); composes with `NumberOptions` on operand. |
| `:math` | Same as `:offset` | Registered alias for older messages. |
| `:unit` | `UnitHandler` (`unstable`) | `unit`, `unitDisplay`, digit options; `usage` → `UnsupportedOperation`; conversion not implemented (§2). |
| `:date` | `DateTimeHandler` `{ kind: "date" }` (`unstable`) | `parse_datetime_options`, IXDTF operands — draft options per spec. |
| `:time` | `DateTimeHandler` `{ kind: "time" }` (`unstable`) | Same. |
| `:datetime` | `DateTimeHandler` `{ kind: "datetime" }` (`unstable`) | Same. |

### 1b. LDML 48 (tr35-76) vs `function.rs` — code-derived option inventory

Maintenance snapshot: cross-check these **parsed / recognized option names** against
[LDML Part 9 — MessageFormat, tr35-76](https://www.unicode.org/reports/tr35/tr35-76/tr35-messageFormat.html)
whenever the cited TR35 edition changes. Unknown keys still surface as
[`FunctionError::BadOption`](components/experimental/src/messageformat/error.rs)
from the match arms in `parse_datetime_options` / `NumberOptions::parse` /
handler-specific validation.

**Shared numeric bag** ([`NumberOptions::parse`](components/experimental/src/messageformat/function.rs)):
`signDisplay`, `useGrouping`, `minimumFractionDigits`, `maximumFractionDigits`,
`minimumIntegerDigits`, `minimumSignificantDigits`, `maximumSignificantDigits`,
`roundingPriority`, `trailingZeroDisplay`, `roundingIncrement`, `roundingMode`,
`notation`, `compactDisplay`, `scientificNotation`, `numberingSystem`.

**`:integer` / `:percent`:** Same parser as `:number` after handler-specific
merging skips (`:integer` clears fraction + significant digit options; `:percent`
skips `minimumIntegerDigits`, `roundingIncrement`, `select`). `:percent` forces
`select` behavior to plural internally.

**`:number` / `:integer` selection:** `select` ∈ {`plural`, `ordinal`, `exact`}
([`parse_select_option`](components/experimental/src/messageformat/function.rs)).

**`:currency`:** `currency` (required for plain numeric operands), `currencyDisplay`
(`narrowSymbol` \| `symbol` \| `name` \| `code` \| `never`), `currencySign`
(`standard` \| `accounting`), `fractionDigits` (`auto` or digit count), plus
the shared numeric bag above. Accounting uses a **locale heuristic**
(`currency_accounting_style`) for parentheses vs locale-standard negative
rendering until [#4677](https://github.com/unicode-org/icu4x/issues/4677) exposes
dedicated CLDR patterns in [`CurrencyEssentials`](components/experimental/src/dimension/provider/currency/essentials.rs).

**`:offset` / `:math`:** `add` or `subtract` (exactly one), each a
`digit-size-option` string.

**`:unit` (`unstable`):** `unit` (required), `unitDisplay` (`long` \| `short` \| `narrow`),
`usage` (well-formed identifier → resolved or `UnsupportedOperation`), plus the
shared numeric bag for digit rounding on the operand.

**`:date` / `:time` / `:datetime` (`unstable`):** `u:*` keys skipped by
`parse_datetime_options`; others depend on `kind`:
`date` → `fields`, `length`, `timeZone`, `calendar`, `numberingSystem`;
`time` → `precision`, `timeZoneStyle`, `hour12`, `timeZone`, `calendar`, `numberingSystem`;
`datetime` → `dateFields`, `dateLength`, `timePrecision`, `timeZoneStyle`, `hour12`,
`timeZone`, `calendar`, `numberingSystem`. Zoned output requires `timeZoneStyle`
to be set (otherwise [`UnsupportedOperation`](components/experimental/src/messageformat/error.rs)).

### 1c. LDML 48 “Default Functions” diff procedure (vs `function.rs`)

Use this when adopting **LDML 48** as the normative baseline or after any TR35 Part 9 bump.

1. Open [LDML Part 9 — MessageFormat, tr35-76](https://www.unicode.org/reports/tr35/tr35-76/tr35-messageFormat.html) and the **Default Functions** table of contents (stable and draft entries).
2. For each row in §1a above, read the spec’s **Options**, **Operands**, **Selection**, and **Errors** subsections for that function.
3. In ICU4X, walk the matching handler and helpers in [`function.rs`](components/experimental/src/messageformat/function.rs) (`NumberOptions::parse`, `parse_datetime_options`, handler-specific option maps, `FunctionError` arms).
4. Record deltas in a PR description (or a short issue comment): **new option names**, **default changes**, **REQUIRED vs RECOMMENDED** wording moves, and any new **Unsupported Operation** cases the spec now requires.
5. After edits, run `cargo test -p icu_experimental --test messageformat_conformance --all-features` and refresh vendored WG JSON only when ready (see [`fixtures/README.md`](components/experimental/tests/messageformat/fixtures/README.md)).

---

## 2. `:unit` — `usage`, conversion, and tests

**Current behavior** ([`UnitHandler`](components/experimental/src/messageformat/function.rs)):

- Requires `unit` option and a numeric operand (including `InputValue::Unit`).
- **`usage`:** If present and a well-formed identifier →
  [`FunctionError::UnsupportedOperation`](components/experimental/src/messageformat/error.rs).
  If malformed → `BadOption`.
- **Conversion:** Not implemented; TR35 *Unit Conversion* is out of scope until
  ICU4X dimension APIs support it.

**WG conformance fixtures:** At upstream SHA pinned in
[`fixtures/README.md`](components/experimental/tests/messageformat/fixtures/README.md),
`unicode-org/message-format-wg` had **no** `test/tests/functions/unit.json`.
Upstream JSON therefore does not exercise `:unit`.

**Plan:**

1. **Short term:** ~~Add ICU4X-only integration tests~~ **Done:** see
   `messageformatter_end_to_end_unit_without_usage` and
   `messageformatter_unit_usage_yields_unsupported_operation` in
   [`tests.rs`](components/experimental/tests/messageformat/tests.rs)
   (`unstable` + `compiled_data`).
2. **When WG adds `unit.json`:** Run `cargo make sync-mf2-tests`, merge, and keep
   [`KNOWN_FAILURES`](components/experimental/tests/messageformat/conformance.rs) empty or list only unavoidable gaps
   (step-by-step: [`fixtures/README.md`](components/experimental/tests/messageformat/fixtures/README.md) § *When upstream adds `functions/unit.json`*).
3. **Long term:** Implement `usage` + conversion against the dimension stack;
   remove `UnsupportedOperation` for supported `usage` values.

---

## 3. `:currency` — `currencySign=accounting` and CLDR

**Deep dive (CLDR → datagen → formatters → MessageFormat):** see [`documents/design/messageformat_currency_accounting.md`](documents/design/messageformat_currency_accounting.md).

**Tracking issue:** [icu4x#4677](https://github.com/unicode-org/icu4x/issues/4677) (open) — add CLDR
negative / accounting pattern slots to dimension data, then replace the
`currency_accounting_style` heuristic in [`CurrencyHandler`](components/experimental/src/messageformat/function.rs).

**Current behavior** ([`CurrencyHandler`](components/experimental/src/messageformat/function.rs)):

- For **`notation=standard`** with symbol / narrow symbol display, `currencySign=accounting`
  (and optional `;` negatives for `currencySign=standard`) are driven by CLDR
  patterns in [`CurrencyEssentials`](components/experimental/src/dimension/provider/currency/essentials.rs)
  via [`CurrencyFormatter`](components/experimental/src/dimension/currency/formatter.rs)
  (`CurrencyDisplaySign` in [`options.rs`](components/experimental/src/dimension/currency/options.rs)).
- For **long name, compact, scientific/engineering, ISO code, and hidden** paths,
  [`currency_accounting_style`](components/experimental/src/messageformat/function.rs)
  and `stitch_*` helpers may still apply the older **locale heuristic** and outer
  parenthesis wrapping where those formatters do not yet consume accounting payloads.
- Follow-up under **#4677**: extend the same CLDR-driven behavior to the remaining
  branches and remove residual heuristics; see
  [`documents/design/messageformat_currency_accounting.md`](documents/design/messageformat_currency_accounting.md).

**Regression coverage:** ICU4X-only cases in
[`functions/currency.json`](components/experimental/tests/messageformat/fixtures/tests/functions/currency.json)
lock `currencySign=accounting` for `en-US` (parentheses) and `nl-NL` (standard
sign path). Extend this file when #4677 lands and output shifts.

**Plan:**

1. **Blocked on** [icu4x#4677](https://github.com/unicode-org/icu4x/issues/4677):
   add negative / accounting pattern payloads to `CurrencyEssentials`, thread
   them through `CurrencyFormatter` / compact variants, then simplify or remove
   the `currency_accounting_style` heuristic in favor of data-driven choice.
2. After (1), refresh the ICU4X-only rows in `currency.json` (and add more
   locales) to match CLDR-backed output.
3. Crate docs in [`mod.rs`](components/experimental/src/messageformat/mod.rs)
   and this section stay authoritative for limitations until (1) ships.

[`CurrencyEssentials`]: components/experimental/src/dimension/provider/currency/essentials.rs

---

## 4. `:offset` / `:math` — operand range and `UnsupportedOperation`

**Spec:** `add` / `subtract` must satisfy the `digit-size-option` ABNF
(non-negative integer, effectively `0`..=`99` in ICU4X). Invalid shapes →
`BadOption` (not `UnsupportedOperation`).

**Implementation** ([`add_decimals`](components/experimental/src/messageformat/function.rs)):

- Converts **base** and **delta** to `i64` via the decimal string form, then
  `checked_add` / `checked_mul`.
- **Yields `UnsupportedOperation` when:**
  - **Base** is not an integer in the `i64` range (e.g. fractional `3.14`, or
    magnitude overflow when string-parsed as integer fails).
  - **Sum** overflows `i64`.
- **Delta** outside `digit-size-option` is rejected earlier as `BadOption`.

This matches a deliberate **“small integer adjustment”** use case (plural
offsets). Messages that need arbitrary-precision or fractional offsets should use
a different strategy (e.g. precompute in application code or a custom function).

**Plan:** No code change required unless TR35 adds examples that require broader
numeric domains; if so, consider `num-bigint` / decimal arithmetic instead of
`i64` fast path.

---

## 5. Ecosystem parity (`messageformat` npm) — scope decision

**Unicode conformance vs JavaScript ecosystem:** Passing the vendored
`message-format-wg` JSON fixtures (see [`conformance.rs`](components/experimental/tests/messageformat/conformance.rs))
is the bar for **spec processor** behavior. Features below are **npm / tooling
ergonomics**; omitting them does **not** by itself imply non-conformance with
the LDML MessageFormat specification tracked by the Unicode WG.

| Capability | `messageformat` (JS) | ICU4X decision |
| --- | --- | --- |
| **CST** (`parseCST`, lossless source trivia) | Yes | **Out of scope** for `icu_experimental::messageformat` unless a dedicated contributor owns a parallel CST layer. Parser targets owned **AST** + [`Message::to_source`](components/experimental/src/messageformat/ast.rs) for round-trip source. |
| **`validate` / `visit` utilities** | Rich JS API | **In scope partially:** [`validate`](components/experimental/src/messageformat/validator.rs) / [`ValidatedMessage`](components/experimental/src/messageformat/validator.rs); **no** separate visitor API planned unless tooling demand appears. |
| **`Intl.MessageFormat` polyfill shape** | Yes | **Out of scope:** Rust API remains [`MessageFormatter`](components/experimental/src/messageformat/formatter.rs) / builder. |
| **Multi-error validation** | Common in tooling | **Deferred:** validator returns the **first** data-model error; multi-error reporting can be a follow-up if editor integrations need it. |

**Optional future workstreams** (only if npm-class *tooling* parity is required; not for Unicode
processor conformance): (1) a lossless **CST** alongside the owned AST, (2) a **visitor** /
transform API over the message tree, (3) **multi-error** validation for editor diagnostics.
Each should be scoped as its own design + ownership; see also
[`messageformat-v2-implementation-details.md`](messageformat-v2-implementation-details.md)
for the existing conformance-first test strategy.

### 5.1 Deferred workstreams (npm / editor parity)

If product requirements go beyond Unicode processor conformance, treat each item as its **own**
design + tracking issue (do not fold into the MF2 formatter crate without an owner):

1. **CST layer** — lossless parse with comments/whitespace; `parseCST`-class workflows for
   formatters and pretty-printers. Today: owned AST + [`Message::to_source`](components/experimental/src/messageformat/ast.rs).
2. **Visitor / transform API** — walk and rewrite `Message` trees (lints, migrations, codegen).
   Today: consume [`ValidatedMessage`](components/experimental/src/messageformat/validator.rs) and pattern-match on [`ast::Message`](components/experimental/src/messageformat/ast.rs).
3. **Multi-error validation** — collect every data-model violation in one pass for IDE
   diagnostics. Today: [`validate`](components/experimental/src/messageformat/validator.rs) returns the **first** [`ValidationError`](components/experimental/src/messageformat/error.rs).

---

## Related design docs

- [`messageformat-v2-research.md`](messageformat-v2-research.md)
- [`messageformat-v2-architecture.md`](messageformat-v2-architecture.md)
- [`messageformat-v2-implementation-details.md`](messageformat-v2-implementation-details.md)
- Tutorial: [`tutorials/messageformat_v2.md`](tutorials/messageformat_v2.md)
