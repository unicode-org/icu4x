# Number Formatting Design Documents

This directory contains design documents, architectural specifications, and technical proposals for numeric, dimension, and measurement formatters in ICU4X.

---

## Scope & Components

The number formatting ecosystem in ICU4X provides modular, zero-copy, and `#[no_std]` compliant implementations of the Unicode Locale Data Markup Language ([UTS #35](https://unicode.org/reports/tr35/tr35-numbers.html)) and [ECMA-402](https://tc39.es/ecma402/#sec-intl.numberformat) specifications:

* **Decimal Formatter (`icu::decimal`)**:
  Core localized numeric formatting, grouping separators, decimal symbols, numbering system glyphs, and precision/rounding rules.
* **Currency Formatter (`icu::currency` / `icu_experimental::dimension::currency`)**:
  Monetary formatting supporting standard symbols, narrow symbols, ISO 4217 codes, pluralized display names, accounting parenthetical formats, no-currency numeric formatting, and compact currency notations.
* **Units & Measurement Formatter (`icu::units` / `icu_experimental::dimension::units`)**:
  Dimensioned unit formatting across categories (length, mass, volume, area, temperature, speed, etc.) with compound units and SI prefixes.
* **Duration Formatter (`icu::duration` / `icu_experimental::dimension::duration`)**:
  Human-readable elapsed duration formatting across hours, minutes, seconds, and fractional sub-seconds per ECMA-402 `Intl.DurationFormat`.
* **Percent & Permille Formatter (`icu_experimental::dimension::percent`)**:
  Localized percentage (`%`) and permille (`‰`) formatting.
* **Compact Decimal Formatter (`icu::decimal::compact`)**:
  Abbreviated and expanded compact scale notations (e.g. `1.2M`, `1.2 million`).

---

## Design Documents

| Document | Topic | Status | Description |
| :--- | :--- | :--- | :--- |
| [**Currency Formatter**](currency_formatter.md) | Monetary & Currency Formatting | `In Implementation` | Architecture, data markers, and UTS #35 / ECMA-402 compliance. |

---

## Key Standards & Specifications

* [Unicode Technical Standard #35: Part 3 (Numbers)](https://unicode.org/reports/tr35/tr35-numbers.html)
* [ECMA-402: `Intl.NumberFormat` Specification](https://tc39.es/ecma402/#sec-intl.numberformat)
* [ECMA-402: `Intl.DurationFormat` Specification](https://tc39.es/ecma402/#sec-intl.durationformat)
* [ISO 4217 Currency Codes](https://www.iso.org/iso-4217-currency-codes.html)
