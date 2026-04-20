# MessageFormat `:currency` and CLDR accounting / negative patterns

This document traces how **LDML MessageFormat 2** `:currency` with `currencySign=accounting` is implemented in ICU4X, how **CLDR** `currencyFormats` data flows into [`CurrencyEssentials`](../../components/experimental/src/dimension/provider/currency/essentials.rs), and what remains for **full** parity (long name, compact, scientific paths, numbering-system–aware datagen).

**Implemented:** CLDR `standard` / `accounting` / `standard-alphaNextToNumber` / `accounting-alphaNextToNumber` patterns (including optional `;` negative subpatterns) are baked into `CurrencyEssentials`; the dimension currency formatters select them via [`CurrencyDisplaySign`](../../components/experimental/src/dimension/currency/options.rs). MessageFormat `:currency` routes **CLDR-native** accounting for the matrix described in §1.2 (symbol / narrow **standard** and **compact short**, long name **standard** and **compact long**, etc.) and uses an ASCII **`(...)`** fallback shell only when **`cldr_handles_accounting_shell`** is false.

Normative MF2 behavior: [LDML Part 9 — MessageFormat](https://www.unicode.org/reports/tr35/tr35-76/tr35-messageFormat.html) (`:currency` default function). Cross-repo tracking: [icu4x#4677](https://github.com/unicode-org/icu4x/issues/4677).

---

## 1. User-visible behavior

### 1.1 Spec options

- **`currencySign=standard`** (default): formatting follows the locale’s usual presentation for negative monetary amounts (sign placement, spacing, etc., as implemented).
- **`currencySign=accounting`**: presentation intended for accounting contexts (often parentheses around the amount, or other locale-specific conventions). Exact rules are defined in TR35 / ECMA-402 alignment; ICU4X aims to match **CLDR** and common engine behavior once data is complete.

### 1.2 What ICU4X does today (summary)

In [`CurrencyHandler::format`](../../components/experimental/src/messageformat/function.rs):

1. When **`cldr_handles_accounting_shell`** is true (see inline comment in that function), CLDR **standard** / **accounting** patterns from [`CurrencyEssentials`](../../components/experimental/src/dimension/provider/currency/essentials.rs) are applied via the dimension formatters (`CurrencyFormatter`, `CompactCurrencyFormatter`, `LongCurrencyFormatter`, or `LongCompactCurrencyFormatter`) with [`CurrencyFormatterOptions`](../../components/experimental/src/dimension/currency/options.rs) / long / compact options carrying `currency_display_sign` from MF2 `currencySign`. No ASCII outer **`(...)`** wrap is used on these paths.
2. **Other combinations** (e.g. ISO code, `currencyDisplay=never`, scientific/engineering, some compact + display pairings): the handler may still wrap the formatted string in ASCII **`(...)`** when `currencySign=accounting`, the amount is negative, and the short-currency resolver reports that the negative sign is encoded in the pattern — a fallback until **#4677** threads the same CLDR payloads through every branch.

Regression expectations include ICU4X-only rows under [`components/experimental/tests/messageformat/fixtures/tests/functions/currency.json`](../../components/experimental/tests/messageformat/fixtures/tests/functions/currency.json) (e.g. `en-US` parentheses vs `de-DE` minus presentation).

---

## 2. End-to-end data flow (current)

```mermaid
flowchart LR
  subgraph cldr [CLDR_JSON]
    numbers[numbers.json_currencyFormats]
    currencies[currencies.json_per_ISO]
  end
  subgraph datagen [provider_source]
    extract[extract_currency_essentials]
  end
  subgraph payload [CurrencyEssentialsV1]
    stdPat[standard_pattern]
    alphaPat[standard_alpha_next_to_number_pattern]
    map[pattern_config_map]
  end
  subgraph runtime [dimension]
    CF[CurrencyFormatter]
    dec[DecimalFormatter_format_sign]
  end
  subgraph mf2 [messageformat]
    CH[CurrencyHandler]
    stitch[stitch_standard_currency_number]
    wrap[ASCII_outer_wrap_fallback]
  end
  numbers --> extract
  currencies --> extract
  extract --> stdPat
  extract --> alphaPat
  extract --> map
  stdPat --> CF
  alphaPat --> CF
  map --> CF
  CF --> dec
  CH --> stitch
  CF --> stitch
  CH --> wrap
```

**Note:** `LongCurrencyFormatter`, `CompactCurrencyFormatter`, and `LongCompactCurrencyFormatter` use **additional** data markers (`CurrencyExtendedDataV1`, `CurrencyPatternsDataV1`, compact payloads). MessageFormat’s `CurrencyHandler` branches into those formatters for some `currencyDisplay` / `notation` combinations; any future accounting work must keep **all** branches consistent (see section 5).

---

## 3. CLDR representation (available in serde, underused in currency essentials)

### 3.1 `NumberPattern` positive / negative

In [`provider/source/src/cldr_serde/numbers.rs`](../../provider/source/src/cldr_serde/numbers.rs), [`NumberPattern`](../../provider/source/src/cldr_serde/numbers.rs) stores:

- **`positive`**: required subpattern (vector of `NumberPatternItem`).
- **`negative`**: optional second subpattern parsed from the substring after **`;`** in the CLDR pattern string (UTS #35 style).

So a single CLDR currency format string like `¤#,##0.00;(¤#,##0.00)` becomes one `NumberPattern` with both arms once deserialized.

### 3.2 `CurrencyFormattingPatterns`

[`CurrencyFormattingPatterns`](../../provider/source/src/cldr_serde/numbers.rs) includes at least:

- `standard: NumberPattern`
- optional `standard-alphaNextToNumber` (`standard-alphaNextToNumber` in JSON)

Both are **`NumberPattern`** values and therefore **may carry `negative`** when CLDR supplies it.

---

## 4. Datagen today (still rough edges)

[`CurrencyEssentials`](../../components/experimental/src/dimension/provider/currency/essentials.rs) **does** serialize CLDR **standard** / **accounting** / **standard-alphaNextToNumber** / **accounting-alphaNextToNumber** patterns and optional **`;` negative** subpatterns from [`provider/source/src/currency/essentials.rs`](../../provider/source/src/currency/essentials.rs) (`create_pattern` on `pattern.positive`, `create_optional_negative_subpattern` on `pattern.negative`).

Remaining datagen limitations called out in source (see **#4677**, **#6064**, **#3838**):

| Topic | Location | Behavior |
| --- | --- | --- |
| Primary pattern body | `create_pattern` | Still maps **`pattern.positive`** only into each main `DoublePlaceholderPattern` (correct for the positive arm; negative is a sibling field today). |
| `PatternSelection` (standard vs alpha-next-to-number) | `currency_pattern_selection` | Uses **`pattern.positive`** only when inspecting currency vs digit placement (**#6064**). |
| Currency format numbering system | `extract_currency_essentials` | Reads `currency_patterns` under **`"latn"`** only (**#3838**). |
| Compact short currency patterns | [`provider/source/src/currency/compact.rs`](../../provider/source/src/currency/compact.rs) | May still log when **`pattern.negative`** is present and build from **positive** only. |

---

## 5. Runtime payload and formatters

[`CurrencyEssentials`](../../components/experimental/src/dimension/provider/currency/essentials.rs) holds the pattern fields above plus `pattern_config_map`, `placeholders`, and `default_pattern_config`. [`CurrencyFormatter`](../../components/experimental/src/dimension/currency/formatter.rs) resolves the correct pattern (including accounting vs standard) via [`CurrencyDisplaySign`](../../components/experimental/src/dimension/currency/options.rs) and applies CLDR structure where the interpolated pattern encodes sign/parentheses.

[`CurrencyHandler`](../../components/experimental/src/messageformat/function.rs) also uses [`LongCurrencyFormatter`](../../components/experimental/src/dimension/currency/long_formatter.rs), [`CompactCurrencyFormatter`](../../components/experimental/src/dimension/currency/compact_formatter.rs), and [`LongCompactCurrencyFormatter`](../../components/experimental/src/dimension/currency/long_compact_formatter.rs) with the same `currency_display_sign` option for branches where CLDR is wired through.

---

## 6. MessageFormat layer — current algorithm (reference)

Implemented in [`CurrencyHandler::format`](../../components/experimental/src/messageformat/function.rs):

1. Parse **`currencySign`** into [`CurrencyDisplaySign`](../../components/experimental/src/dimension/currency/options.rs) (**standard** vs **accounting**).
2. For **accounting** + negative amounts, optionally rewrite the numeric operand to **absolute** magnitude when the short `CurrencyFormatter` reports that the **negative sign is encoded inside the CLDR pattern** (`negative_sign_encoded_in_pattern`), so the formatter does not double-apply a decimal minus.
3. Dispatch to the dimension formatters per `currencyDisplay` / `notation` / `compactDisplay`. When **`cldr_handles_accounting_shell`** is true, CLDR owns the accounting presentation for that branch.
4. If **`cldr_handles_accounting_shell`** is false but step 2 applied, wrap the final string in ASCII **`(...)`** as a **fallback accounting shell** (see §1.2).

### 6.1 Why “stitch” exists

Helpers such as [`stitch_standard_currency_number`](../../components/experimental/src/messageformat/function.rs) probe a sample magnitude and split around the plain decimal to recover **prefix/suffix** around styled inner numbers when composing **scientific** / **engineering** / some **compact** shapes with currency literals. That remains a **layout** tool; it is not a full CLDR accounting substitute on its own.

---

## 7. Target state (design directions — #4677)

Pick one approach in **#4677** / an RFC before large changes.

### Option A — Full CLDR parity on every branch

- Thread the same **accounting / negative** pattern selection used by `CurrencyFormatter` through **ISO code**, **`currencyDisplay=never`**, **scientific/engineering**, and any **compact + display** pair that still hits the ASCII **`(...)`** fallback today.
- Tighten **compact** datagen when CLDR supplies negative subpatterns.

### Option B — Phased

- Land compact / long-marker accounting field-by-field, shrinking the MessageFormat fallback matrix after each milestone.

### Validation targets

- Compare against **ICU4C / ICU4J** and **ECMA-402** where applicable.
- Expand ICU4X-only [`currency.json`](../../components/experimental/tests/messageformat/fixtures/tests/functions/currency.json) rows as branches become data-driven.

---

## 8. Testing and verification matrix

| Layer | What to run / extend |
| --- | --- |
| MessageFormat | `cargo test -p icu_experimental --test messageformat_conformance --all-features`; ICU4X-only [`currency.json`](../../components/experimental/tests/messageformat/fixtures/tests/functions/currency.json) |
| Datagen | Unit tests in [`provider/source/src/currency/essentials.rs`](../../provider/source/src/currency/essentials.rs) (`test_basic` ~305+); today includes `TODO(#6064)` expectations for placeholders (e.g. EGP) — **expect these to evolve** when negative subpatterns affect selection |
| Dimension | Formatter tests under `components/experimental/src/dimension/currency/` and any provider JSON snapshots |

---

## 9. Issue cross-reference

| Tracker | Role |
| --- | --- |
| [icu4x#4677](https://github.com/unicode-org/icu4x/issues/4677) | Umbrella: extend CLDR accounting / negative pattern consumption to **all** branches (compact datagen, ISO/hidden/scientific MessageFormat paths, etc.); remove the MessageFormat ASCII **`(...)`** fallback where redundant. |
| **#6064** (see TODOs in source) | Negative **subpattern** in currency pattern parsing / `PatternSelection` and tests ([`provider/source/src/currency/essentials.rs`](../../provider/source/src/currency/essentials.rs), [`components/experimental/src/dimension/currency/format.rs`](../../components/experimental/src/dimension/currency/format.rs), compact format). |
| **#3838** (see TODOs in source) | Currency patterns should follow **resolved numbering system**, not hard-coded `"latn"` in datagen ([`provider/source/src/currency/essentials.rs`](../../provider/source/src/currency/essentials.rs)). |

---

## Related documents

- [`messageformat-tr35-spec-tracking.md`](../../messageformat-tr35-spec-tracking.md) §3 — high-level gap summary and plan bullets.
- [`documents/design/data_pipeline.md`](data_pipeline.md) — general ICU4X locale data concepts.
